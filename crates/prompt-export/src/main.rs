mod claude;
mod codex;
mod session_log;

use anyhow::{Context, Result};
use chrono::{DateTime, Duration, Local, Utc};
use clap::Parser;
use cli_core::date_range::{DateRange, DateRangeArgs};
use cli_core::ui::Theme;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "prompt-export", version)]
#[command(
    about = "Export Claude Code / Codex prompts and agent outputs as Markdown",
    long_about = "Collects user prompts and agent outputs from Claude Code (~/.claude/projects)\n\
                  and Codex (~/.codex/sessions) session logs, filters them by period, role,\n\
                  and project, and renders Markdown suitable for later LLM analysis.\n\n\
                  Without a period option the full history is exported."
)]
struct Cli {
    #[arg(long, default_value = "all", help = "Log source: claude, codex, all")]
    source: String,

    #[arg(
        long,
        default_value = "user",
        help = "Roles to include: user, assistant, all"
    )]
    role: String,

    #[command(flatten)]
    date_range: DateRangeArgs,

    #[arg(
        long,
        help = "Only sessions whose project path contains this substring"
    )]
    project: Option<String>,

    #[arg(
        short,
        long,
        value_name = "FILE",
        help = "Export Markdown to a file instead of stdout"
    )]
    export: Option<PathBuf>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Source {
    Claude,
    Codex,
}

impl fmt::Display for Source {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Source::Claude => write!(f, "Claude Code"),
            Source::Codex => write!(f, "Codex"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Role {
    User,
    Assistant,
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Role::User => write!(f, "User"),
            Role::Assistant => write!(f, "Assistant"),
        }
    }
}

pub struct Entry {
    pub source: Source,
    pub role: Role,
    pub timestamp: DateTime<Utc>,
    pub project: String,
    pub session_id: String,
    pub text: String,
}

pub struct Filter {
    pub date_range: DateRange,
    pub include_user: bool,
    pub include_assistant: bool,
    pub project: Option<String>,
}

impl Filter {
    pub fn accepts_role(&self, role: Role) -> bool {
        match role {
            Role::User => self.include_user,
            Role::Assistant => self.include_assistant,
        }
    }

    pub fn accepts_time(&self, timestamp: DateTime<Utc>) -> bool {
        self.date_range.contains(timestamp)
    }

    pub fn accepts_project(&self, project: &str) -> bool {
        self.project
            .as_ref()
            .is_none_or(|needle| project.contains(needle.as_str()))
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let (include_user, include_assistant) = match cli.role.to_lowercase().as_str() {
        "user" => (true, false),
        "assistant" => (false, true),
        "all" => (true, true),
        other => anyhow::bail!("Unknown role: {other}. Use user, assistant, or all"),
    };
    let (use_claude, use_codex) = match cli.source.to_lowercase().as_str() {
        "claude" => (true, false),
        "codex" => (false, true),
        "all" => (true, true),
        other => anyhow::bail!("Unknown source: {other}. Use claude, codex, or all"),
    };

    let filter = Filter {
        date_range: cli.date_range.resolve()?,
        include_user,
        include_assistant,
        project: cli.project.clone(),
    };

    let home = cli_core::command_log::home_dir()?;
    let mut entries = Vec::new();
    if use_claude {
        entries.extend(claude::collect(
            &home.join(".claude").join("projects"),
            &filter,
        ));
    }
    if use_codex {
        entries.extend(codex::collect(
            &home.join(".codex").join("sessions"),
            &filter,
        ));
    }

    let (markdown, total) = render_markdown(&entries, &filter);
    if total == 0 {
        eprintln!("{}", Theme::warning("No entries matched the filters"));
    }

    match &cli.export {
        Some(path) => {
            std::fs::write(path, &markdown)
                .with_context(|| format!("Failed to write {}", path.display()))?;
            println!(
                "{}",
                Theme::success(format!("Exported {total} entries to {}", path.display()))
            );
        }
        None => print!("{markdown}"),
    }

    Ok(())
}

fn render_markdown(entries: &[Entry], filter: &Filter) -> (String, usize) {
    struct Session<'a> {
        source: Source,
        session_id: &'a str,
        project: &'a str,
        entries: Vec<&'a Entry>,
    }

    let mut sessions: Vec<Session> = Vec::new();
    let mut index: HashMap<(Source, &str), usize> = HashMap::new();
    // Same instant + same content in the same project = the same event logged
    // twice: codex exec double-logs, and forked/resumed Claude sessions copy
    // history into a new session file. Repeated prompts at other times stay.
    let mut seen: HashSet<(Source, Role, DateTime<Utc>, &str, &str)> = HashSet::new();
    for entry in entries {
        if !seen.insert((
            entry.source,
            entry.role,
            entry.timestamp,
            entry.project.as_str(),
            entry.text.as_str(),
        )) {
            continue;
        }
        let key = (entry.source, entry.session_id.as_str());
        let position = *index.entry(key).or_insert_with(|| {
            sessions.push(Session {
                source: entry.source,
                session_id: &entry.session_id,
                project: &entry.project,
                entries: Vec::new(),
            });
            sessions.len() - 1
        });
        sessions[position].entries.push(entry);
    }
    for session in &mut sessions {
        session.entries.sort_by_key(|entry| entry.timestamp);
    }
    sessions.sort_by_key(|session| session.entries[0].timestamp);
    let total = seen.len();

    let describe = |bound: Option<DateTime<Utc>>, fallback: &str| {
        bound.map_or_else(
            || fallback.to_string(),
            |value| {
                value
                    .with_timezone(&Local)
                    .format("%Y-%m-%d %H:%M")
                    .to_string()
            },
        )
    };
    // The exclusive end bound reads better as the last included second.
    let displayed_until = filter
        .date_range
        .end()
        .map(|until| until - Duration::seconds(1));

    let mut markdown = String::new();
    markdown.push_str("# Prompt Export\n\n");
    markdown.push_str(&format!(
        "- Generated: {}\n",
        Local::now().format("%Y-%m-%d %H:%M:%S %z")
    ));
    markdown.push_str(&format!(
        "- Period: {} ~ {}\n",
        describe(filter.date_range.start(), "beginning"),
        describe(displayed_until, "now")
    ));
    markdown.push_str(&format!("- Entries: {total}\n"));

    for session in sessions {
        let short_id: String = session.session_id.chars().take(8).collect();
        let started = session.entries[0]
            .timestamp
            .with_timezone(&Local)
            .format("%Y-%m-%d");
        markdown.push_str(&format!(
            "\n## {} · {} ({}, session {})\n",
            session.source, session.project, started, short_id
        ));
        for entry in session.entries {
            markdown.push_str(&format!(
                "\n### {} · {}\n\n{}\n",
                entry.role,
                entry
                    .timestamp
                    .with_timezone(&Local)
                    .format("%Y-%m-%d %H:%M:%S"),
                entry.text.trim()
            ));
        }
    }

    (markdown, total)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDateTime;

    fn at(raw: &str) -> DateTime<Utc> {
        NaiveDateTime::parse_from_str(raw, "%Y-%m-%d %H:%M:%S")
            .unwrap()
            .and_utc()
    }

    fn filter_between(since: Option<&str>, until: Option<&str>) -> Filter {
        Filter {
            date_range: DateRange::new(since.map(at), until.map(at)),
            include_user: true,
            include_assistant: false,
            project: None,
        }
    }

    #[test]
    fn time_filter_is_inclusive_since_exclusive_until() {
        let filter = filter_between(Some("2026-08-01 00:00:00"), Some("2026-08-08 00:00:00"));

        assert!(filter.accepts_time(at("2026-08-01 00:00:00")));
        assert!(filter.accepts_time(at("2026-08-07 23:59:59")));
        assert!(!filter.accepts_time(at("2026-07-31 23:59:59")));
        assert!(!filter.accepts_time(at("2026-08-08 00:00:00")));
    }

    #[test]
    fn dedups_same_instant_across_sessions_and_counts_after_dedup() {
        let entry = |timestamp: &str, text: &str, session: &str| Entry {
            source: Source::Codex,
            role: Role::User,
            timestamp: at(timestamp),
            project: "/workspace/demo".to_string(),
            session_id: session.to_string(),
            text: text.to_string(),
        };
        let entries = vec![
            entry("2026-08-03 00:40:49", "계속", "session-1"),
            // Double-logged event in the same session: dropped.
            entry("2026-08-03 00:40:49", "계속", "session-1"),
            // Forked-session copy of the same event: dropped.
            entry("2026-08-03 00:40:49", "계속", "session-2"),
            // Repeated prompt at a later time: kept.
            entry("2026-08-03 00:41:26", "계속", "session-1"),
        ];
        let filter = filter_between(None, None);

        let (markdown, total) = render_markdown(&entries, &filter);
        assert_eq!(total, 2);
        assert_eq!(markdown.matches("계속").count(), 2);
        assert!(markdown.contains("- Entries: 2\n"));
        assert!(!markdown.contains("session-2"));
    }

    #[test]
    fn project_filter_matches_substring() {
        let mut filter = filter_between(None, None);
        assert!(filter.accepts_project("/Volumes/990EVO+/workspace/chann/cli-tools"));

        filter.project = Some("cli-tools".to_string());
        assert!(filter.accepts_project("/Volumes/990EVO+/workspace/chann/cli-tools"));
        assert!(!filter.accepts_project("/Users/channprj/other"));
    }
}
