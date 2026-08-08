use crate::{Entry, Filter, Role, Source};
use chrono::{DateTime, Utc};
use serde_json::Value;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;
use walkdir::WalkDir;

/// Collect prompts and outputs from `~/.codex/sessions/YYYY/MM/DD/rollout-*.jsonl`.
///
/// `event_msg` lines are the canonical clean source: `user_message` carries the
/// human-typed prompt and `agent_message` the user-visible agent text, while
/// `response_item` duplicates them plus injected context (AGENTS.md, skills).
pub fn collect(root: &Path, filter: &Filter) -> Vec<Entry> {
    let mut entries = Vec::new();
    for file in WalkDir::new(root).into_iter().flatten() {
        let name = file.file_name().to_string_lossy();
        if !file.file_type().is_file() || !name.starts_with("rollout-") || !name.ends_with(".jsonl")
        {
            continue;
        }
        // A file last written before the window opens has no lines in range.
        if let (Some(since), Ok(metadata)) = (filter.date_range.start(), file.metadata()) {
            if let Ok(modified) = metadata.modified() {
                if DateTime::<Utc>::from(modified) < since {
                    continue;
                }
            }
        }
        collect_file(file.path(), filter, &mut entries);
    }
    entries
}

fn collect_file(path: &Path, filter: &Filter, entries: &mut Vec<Entry>) {
    let Ok(file) = fs::File::open(path) else {
        return;
    };
    let mut session_id = path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or("unknown")
        .to_string();
    let mut project = String::new();

    for line in BufReader::new(file).lines() {
        let Ok(line) = line else {
            continue;
        };
        let Ok(value) = serde_json::from_str::<Value>(&line) else {
            continue;
        };
        let Some(payload) = value.get("payload") else {
            continue;
        };
        match value.get("type").and_then(Value::as_str) {
            // First line of every rollout file: session id and project cwd.
            Some("session_meta") => {
                if let Some(id) = payload.get("id").and_then(Value::as_str) {
                    session_id = id.to_string();
                }
                if let Some(cwd) = payload.get("cwd").and_then(Value::as_str) {
                    project = cwd.to_string();
                }
                if !filter.accepts_project(&project) {
                    return;
                }
            }
            Some("event_msg") => {
                let role = match payload.get("type").and_then(Value::as_str) {
                    Some("user_message") => Role::User,
                    Some("agent_message") => Role::Assistant,
                    _ => continue,
                };
                if !filter.accepts_role(role) {
                    continue;
                }
                let Some(text) = payload.get("message").and_then(Value::as_str) else {
                    continue;
                };
                if text.trim().is_empty() {
                    continue;
                }
                let Some(timestamp) = value
                    .get("timestamp")
                    .and_then(Value::as_str)
                    .and_then(|raw| DateTime::parse_from_rfc3339(raw).ok())
                    .map(|parsed| parsed.with_timezone(&Utc))
                else {
                    continue;
                };
                if !filter.accepts_time(timestamp) || !filter.accepts_project(&project) {
                    continue;
                }
                entries.push(Entry {
                    source: Source::Codex,
                    role,
                    timestamp,
                    project: project.clone(),
                    session_id: session_id.clone(),
                    text: text.to_string(),
                });
            }
            _ => continue,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn collect_lines(lines: &[&str], filter: &Filter) -> Vec<Entry> {
        static NEXT_DIR: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
        let dir = std::env::temp_dir().join(format!(
            "prompt-export-test-{}-{}",
            std::process::id(),
            NEXT_DIR.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
        ));
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("rollout-2026-08-01T23-47-16-test.jsonl");
        let mut file = fs::File::create(&path).unwrap();
        for line in lines {
            writeln!(file, "{line}").unwrap();
        }
        let mut entries = Vec::new();
        collect_file(&path, filter, &mut entries);
        fs::remove_dir_all(&dir).ok();
        entries
    }

    fn all_filter() -> Filter {
        Filter {
            date_range: cli_core::date_range::DateRange::default(),
            include_user: true,
            include_assistant: true,
            project: None,
        }
    }

    #[test]
    fn extracts_user_and_agent_messages_only() {
        let entries = collect_lines(
            &[
                r#"{"timestamp":"2026-08-01T14:47:16.049Z","type":"session_meta","payload":{"id":"019fbdcb-293d","cwd":"/workspace/demo"}}"#,
                r#"{"timestamp":"2026-08-01T14:48:00.000Z","type":"event_msg","payload":{"type":"user_message","message":"dmg downloads 빌드해주세요."}}"#,
                r#"{"timestamp":"2026-08-01T14:48:01.000Z","type":"event_msg","payload":{"type":"token_count","info":{}}}"#,
                r##"{"timestamp":"2026-08-01T14:49:00.000Z","type":"response_item","payload":{"type":"message","role":"user","content":[{"type":"input_text","text":"# AGENTS.md instructions"}]}}"##,
                r#"{"timestamp":"2026-08-01T14:50:00.000Z","type":"event_msg","payload":{"type":"agent_message","message":"빌드를 시작합니다.","phase":"final_answer"}}"#,
            ],
            &all_filter(),
        );

        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].role, Role::User);
        assert_eq!(entries[0].text, "dmg downloads 빌드해주세요.");
        assert_eq!(entries[0].session_id, "019fbdcb-293d");
        assert_eq!(entries[0].project, "/workspace/demo");
        assert_eq!(entries[1].role, Role::Assistant);
        assert_eq!(entries[1].text, "빌드를 시작합니다.");
    }

    #[test]
    fn project_filter_skips_whole_session() {
        let mut filter = all_filter();
        filter.project = Some("other-project".to_string());
        let entries = collect_lines(
            &[
                r#"{"timestamp":"2026-08-01T14:47:16.049Z","type":"session_meta","payload":{"id":"019fbdcb-293d","cwd":"/workspace/demo"}}"#,
                r#"{"timestamp":"2026-08-01T14:48:00.000Z","type":"event_msg","payload":{"type":"user_message","message":"프롬프트"}}"#,
            ],
            &filter,
        );
        assert!(entries.is_empty());
    }
}
