use crate::{Entry, Filter, Role, Source};
use chrono::{DateTime, Utc};
use serde_json::Value;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// Non-human user lines are wrapped in these markers by the Claude Code CLI.
const INJECTED_PREFIXES: &[&str] = &[
    "<command-name>",
    "<command-message>",
    "<local-command-stdout>",
    "<local-command-caveat>",
    "<system-reminder>",
    "<teammate-message",
    "<bash-input>",
    "<bash-stdout>",
    "<bash-stderr>",
    "Caveat: ",
    "[Request interrupted",
    "Another Claude session sent a message",
];

/// promptSource values written by a human (recent CLI versions only).
const HUMAN_PROMPT_SOURCES: &[&str] = &["typed", "queued", "suggestion_accepted"];

/// Collect prompts and outputs from `~/.claude/projects/*/*.jsonl` session logs.
///
/// Only files directly inside a project directory are sessions; subdirectories
/// hold subagent transcripts and are skipped.
pub fn collect(root: &Path, filter: &Filter) -> Vec<Entry> {
    let mut entries = Vec::new();
    let Ok(projects) = fs::read_dir(root) else {
        return entries;
    };
    for project in projects.flatten() {
        let Ok(files) = fs::read_dir(project.path()) else {
            continue;
        };
        for file in files.flatten() {
            let path = file.path();
            if path.extension().and_then(|ext| ext.to_str()) != Some("jsonl") {
                continue;
            }
            // A file last written before the window opens has no lines in range.
            if let (Some(since), Ok(metadata)) = (filter.since, file.metadata()) {
                if let Ok(modified) = metadata.modified() {
                    if DateTime::<Utc>::from(modified) < since {
                        continue;
                    }
                }
            }
            collect_file(&path, filter, &mut entries);
        }
    }
    entries
}

fn collect_file(path: &Path, filter: &Filter, entries: &mut Vec<Entry>) {
    let Ok(file) = fs::File::open(path) else {
        return;
    };
    let fallback_session = path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or("unknown")
        .to_string();

    // One assistant API response is split across consecutive lines that share
    // a message id; merge those text blocks into a single entry.
    let mut last_message_id = String::new();

    for line in BufReader::new(file).lines() {
        let Ok(line) = line else {
            continue;
        };
        let Ok(value) = serde_json::from_str::<Value>(&line) else {
            continue;
        };
        let Some((role, text)) = parse_line(&value) else {
            continue;
        };
        if !filter.accepts_role(role) {
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
        if !filter.accepts_time(timestamp) {
            continue;
        }
        let project = value.get("cwd").and_then(Value::as_str).unwrap_or("");
        if !filter.accepts_project(project) {
            continue;
        }

        let message_id = value
            .get("message")
            .and_then(|message| message.get("id"))
            .and_then(Value::as_str)
            .unwrap_or("");
        if role == Role::Assistant && !message_id.is_empty() && message_id == last_message_id {
            if let Some(last) = entries.last_mut() {
                last.text.push_str("\n\n");
                last.text.push_str(&text);
                continue;
            }
        }
        last_message_id = message_id.to_string();

        entries.push(Entry {
            source: Source::Claude,
            role,
            timestamp,
            project: project.to_string(),
            session_id: value
                .get("sessionId")
                .and_then(Value::as_str)
                .unwrap_or(&fallback_session)
                .to_string(),
            text,
        });
    }
}

/// Extract (role, text) from one session line, or None for lines that are not
/// a human prompt or user-visible assistant text.
fn parse_line(value: &Value) -> Option<(Role, String)> {
    if value.get("isSidechain").and_then(Value::as_bool) == Some(true)
        || value.get("isApiErrorMessage").and_then(Value::as_bool) == Some(true)
    {
        return None;
    }
    match value.get("type")?.as_str()? {
        "user" => {
            if value.get("isMeta").and_then(Value::as_bool) == Some(true)
                || value.get("isCompactSummary").and_then(Value::as_bool) == Some(true)
            {
                return None;
            }
            // Recent CLI versions label who wrote the prompt; anything not
            // human-written (sdk, system) is injected traffic.
            if let Some(source) = value.get("promptSource").and_then(Value::as_str) {
                if !HUMAN_PROMPT_SOURCES.contains(&source) {
                    return None;
                }
            }
            let text = match value.get("message")?.get("content")? {
                Value::String(text) => text.clone(),
                Value::Array(items) => {
                    // Arrays with tool_result items are tool output, not prompts.
                    let is_tool_result = items.iter().any(|item| {
                        item.get("type").and_then(Value::as_str) == Some("tool_result")
                    });
                    if is_tool_result {
                        return None;
                    }
                    items
                        .iter()
                        .filter(|item| item.get("type").and_then(Value::as_str) == Some("text"))
                        .filter_map(|item| item.get("text").and_then(Value::as_str))
                        .collect::<Vec<_>>()
                        .join("\n")
                }
                _ => return None,
            };
            if !is_human_prompt(&text) {
                return None;
            }
            Some((Role::User, text))
        }
        "assistant" => {
            let text = value
                .get("message")?
                .get("content")?
                .as_array()?
                .iter()
                .filter(|block| block.get("type").and_then(Value::as_str) == Some("text"))
                .filter_map(|block| block.get("text").and_then(Value::as_str))
                .collect::<Vec<_>>()
                .join("\n\n");
            if text.trim().is_empty() {
                return None;
            }
            Some((Role::Assistant, text))
        }
        _ => None,
    }
}

fn is_human_prompt(text: &str) -> bool {
    let trimmed = text.trim_start();
    !trimmed.is_empty()
        && !INJECTED_PREFIXES
            .iter()
            .any(|prefix| trimmed.starts_with(prefix))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn keeps_typed_user_prompt() {
        let line = json!({
            "type": "user",
            "isSidechain": false,
            "message": {"role": "user", "content": "서버 재시작 설정을 알려줘"}
        });
        let (role, text) = parse_line(&line).unwrap();
        assert_eq!(role, Role::User);
        assert_eq!(text, "서버 재시작 설정을 알려줘");
    }

    #[test]
    fn drops_meta_command_and_tool_result_lines() {
        let meta = json!({
            "type": "user",
            "isMeta": true,
            "message": {"role": "user", "content": "skill load"}
        });
        let command = json!({
            "type": "user",
            "message": {"role": "user", "content": "<command-name>/model</command-name>"}
        });
        let command_message_first = json!({
            "type": "user",
            "message": {"role": "user", "content":
                "<command-message>git-commit-push</command-message>\n<command-name>/git-commit-push</command-name>"}
        });
        let bash_input = json!({
            "type": "user",
            "message": {"role": "user", "content": "<bash-input>ls -la</bash-input>"}
        });
        let compact = json!({
            "type": "user",
            "isCompactSummary": true,
            "message": {"role": "user", "content": "This session is being continued..."}
        });
        let sidechain = json!({
            "type": "user",
            "isSidechain": true,
            "message": {"role": "user", "content": "subagent prompt"}
        });
        let tool_result = json!({
            "type": "user",
            "message": {"role": "user", "content": [
                {"type": "tool_result", "tool_use_id": "toolu_01", "content": "output"}
            ]}
        });
        for line in [
            meta,
            command,
            command_message_first,
            bash_input,
            compact,
            sidechain,
            tool_result,
        ] {
            assert!(parse_line(&line).is_none(), "should drop: {line}");
        }
    }

    #[test]
    fn drops_non_human_prompt_sources_but_keeps_typed() {
        let system = json!({
            "type": "user",
            "promptSource": "system",
            "message": {"role": "user", "content": "Another Claude session sent a message: ..."}
        });
        let sdk = json!({
            "type": "user",
            "promptSource": "sdk",
            "message": {"role": "user", "content": "programmatic prompt"}
        });
        let teammate_no_metadata = json!({
            "type": "user",
            "message": {"role": "user", "content": "Another Claude session sent a message:\n<teammate-message ...>"}
        });
        for line in [system, sdk, teammate_no_metadata] {
            assert!(parse_line(&line).is_none(), "should drop: {line}");
        }

        let typed = json!({
            "type": "user",
            "promptSource": "typed",
            "origin": {"kind": "human"},
            "message": {"role": "user", "content": "진짜 사용자 프롬프트"}
        });
        assert_eq!(parse_line(&typed).unwrap().1, "진짜 사용자 프롬프트");
    }

    #[test]
    fn extracts_assistant_text_and_skips_tool_use() {
        let text_block = json!({
            "type": "assistant",
            "message": {"role": "assistant", "content": [{"type": "text", "text": "답변입니다."}]}
        });
        let (role, text) = parse_line(&text_block).unwrap();
        assert_eq!(role, Role::Assistant);
        assert_eq!(text, "답변입니다.");

        let tool_use = json!({
            "type": "assistant",
            "message": {"role": "assistant", "content": [
                {"type": "tool_use", "id": "toolu_01", "name": "Bash", "input": {}}
            ]}
        });
        assert!(parse_line(&tool_use).is_none());

        let thinking = json!({
            "type": "assistant",
            "message": {"role": "assistant", "content": [{"type": "thinking", "thinking": "..."}]}
        });
        assert!(parse_line(&thinking).is_none());

        let api_error = json!({
            "type": "assistant",
            "isApiErrorMessage": true,
            "message": {"role": "assistant", "content": [
                {"type": "text", "text": "You've hit your session limit"}
            ]}
        });
        assert!(parse_line(&api_error).is_none());
    }
}
