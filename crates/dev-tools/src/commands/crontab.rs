use anyhow::{bail, Context, Result};
use chrono::{DateTime, Local};
use cli_core::output::TableFormatter;
use cli_core::ui::Theme;
use cron::Schedule;
use cron_descriptor::cronparser::cron_expression_descriptor;
use owo_colors::OwoColorize;
use std::io::Write;
use std::process::{Command, Stdio};
use std::str::FromStr;

/// One line of the user crontab. Non-entry lines (comments, env vars,
/// blanks, unparsable lines) are preserved verbatim on write-back.
#[derive(Debug, Clone, PartialEq)]
enum Line {
    Entry {
        raw: String,
        schedule: String,
        command: String,
    },
    Other(String),
}

pub fn list() -> Result<()> {
    let lines = parse(&read_crontab()?);
    let entries: Vec<&Line> = lines
        .iter()
        .filter(|l| matches!(l, Line::Entry { .. }))
        .collect();

    if entries.is_empty() {
        println!("{}", Theme::info("Crontab is empty. Add an entry with: dev-tools crontab add \"0 9 * * *\" \"<command>\""));
        return Ok(());
    }

    println!("\n{}", Theme::header(format!("Crontab ({} entries)", entries.len())));
    let mut table = TableFormatter::create_table();
    table.set_header(vec![
        TableFormatter::header_cell("#"),
        TableFormatter::header_cell("Schedule"),
        TableFormatter::header_cell("Command"),
        TableFormatter::header_cell("Description"),
        TableFormatter::header_cell("Next Run"),
    ]);

    for (i, line) in entries.iter().enumerate() {
        if let Line::Entry { schedule, command, .. } = line {
            table.add_row(vec![
                TableFormatter::value_cell(i + 1),
                TableFormatter::highlight_cell(schedule),
                TableFormatter::value_cell(command),
                TableFormatter::value_cell(describe(schedule)),
                TableFormatter::value_cell(
                    next_run(schedule).map_or("-".to_string(), |t| {
                        t.format("%Y-%m-%d %H:%M:%S").to_string()
                    }),
                ),
            ]);
        }
    }
    println!("{}", table);
    Ok(())
}

pub fn add(schedule: &str, command: &str, comment: Option<&str>) -> Result<()> {
    validate_schedule(schedule)?;
    let mut lines = parse(&read_crontab()?);
    add_entry(&mut lines, schedule, command, comment);
    write_crontab(&render(&lines))?;

    println!("{} {}", Theme::success("Added:"), Theme::value(format!("{} {}", schedule, command)));
    print_next_run(schedule);
    Ok(())
}

pub fn remove(index: usize) -> Result<()> {
    let mut lines = parse(&read_crontab()?);
    let removed = remove_entry(&mut lines, index)?;
    write_crontab(&render(&lines))?;

    println!("{} {}", Theme::success("Removed:"), Theme::value(&removed));
    Ok(())
}

pub fn edit(index: usize, schedule: Option<&str>, command: Option<&str>) -> Result<()> {
    if schedule.is_none() && command.is_none() {
        bail!("Nothing to change. Pass --schedule and/or --command");
    }
    if let Some(s) = schedule {
        validate_schedule(s)?;
    }
    let mut lines = parse(&read_crontab()?);
    let (old, new) = edit_entry(&mut lines, index, schedule, command)?;
    write_crontab(&render(&lines))?;

    println!("{} {}", "Before:".dimmed(), Theme::value(&old));
    println!("{} {}", Theme::success("After: "), Theme::highlight(&new));
    if let Some(s) = schedule {
        print_next_run(s);
    }
    Ok(())
}

fn print_next_run(schedule: &str) {
    if let Some(t) = next_run(schedule) {
        println!(
            "  {} {} ({})",
            "Next run:".dimmed(),
            t.format("%Y-%m-%d %H:%M:%S"),
            describe(schedule)
        );
    }
}

// --- pure core (unit tested) ---

fn parse(content: &str) -> Vec<Line> {
    content
        .lines()
        .map(|line| {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') || is_env_assignment(trimmed) {
                return Line::Other(line.to_string());
            }
            match split_entry(trimmed) {
                Some((schedule, command)) if validate_schedule(&schedule).is_ok() => Line::Entry {
                    raw: line.to_string(),
                    schedule,
                    command,
                },
                _ => Line::Other(line.to_string()),
            }
        })
        .collect()
}

/// Split a crontab entry line into (schedule, command).
fn split_entry(line: &str) -> Option<(String, String)> {
    let fields: Vec<&str> = line.split_whitespace().collect();
    let schedule_len = if line.starts_with('@') { 1 } else { 5 };
    if fields.len() <= schedule_len {
        return None;
    }
    Some((
        fields[..schedule_len].join(" "),
        fields[schedule_len..].join(" "),
    ))
}

fn is_env_assignment(line: &str) -> bool {
    line.split_whitespace()
        .next()
        .is_some_and(|first| first.contains('='))
}

fn render(lines: &[Line]) -> String {
    if lines.is_empty() {
        return String::new();
    }
    let mut out = lines
        .iter()
        .map(|l| match l {
            Line::Entry { raw, .. } => raw.as_str(),
            Line::Other(raw) => raw.as_str(),
        })
        .collect::<Vec<_>>()
        .join("\n");
    out.push('\n');
    out
}

fn entry_positions(lines: &[Line]) -> Vec<usize> {
    lines
        .iter()
        .enumerate()
        .filter_map(|(i, l)| matches!(l, Line::Entry { .. }).then_some(i))
        .collect()
}

fn position_of(lines: &[Line], index: usize) -> Result<usize> {
    let positions = entry_positions(lines);
    if index == 0 || index > positions.len() {
        bail!(
            "No entry #{}. Crontab has {} entries (see: dev-tools crontab list)",
            index,
            positions.len()
        );
    }
    Ok(positions[index - 1])
}

fn add_entry(lines: &mut Vec<Line>, schedule: &str, command: &str, comment: Option<&str>) {
    if let Some(c) = comment {
        lines.push(Line::Other(format!("# {}", c)));
    }
    let raw = format!("{} {}", schedule, command);
    lines.push(Line::Entry {
        raw,
        schedule: schedule.to_string(),
        command: command.to_string(),
    });
}

fn remove_entry(lines: &mut Vec<Line>, index: usize) -> Result<String> {
    let pos = position_of(lines, index)?;
    match lines.remove(pos) {
        Line::Entry { raw, .. } => Ok(raw),
        Line::Other(_) => unreachable!("position_of returns entry positions only"),
    }
}

fn edit_entry(
    lines: &mut [Line],
    index: usize,
    new_schedule: Option<&str>,
    new_command: Option<&str>,
) -> Result<(String, String)> {
    let pos = position_of(lines, index)?;
    if let Line::Entry { raw, schedule, command } = &mut lines[pos] {
        let old = raw.clone();
        if let Some(s) = new_schedule {
            *schedule = s.to_string();
        }
        if let Some(c) = new_command {
            *command = c.to_string();
        }
        *raw = format!("{} {}", schedule, command);
        Ok((old, raw.clone()))
    } else {
        unreachable!("position_of returns entry positions only")
    }
}

/// Accept standard 5-field expressions and @-specials (incl. @reboot).
fn validate_schedule(schedule: &str) -> Result<()> {
    if schedule == "@reboot" {
        return Ok(());
    }
    let expr = to_six_field(schedule)
        .ok_or_else(|| anyhow::anyhow!("Expected 5 fields (min hour day month weekday) or an @-special, got: {:?}", schedule))?;
    Schedule::from_str(&expr)
        .map(|_| ())
        .map_err(|e| anyhow::anyhow!("Invalid cron schedule {:?}: {}", schedule, e))
}

/// Normalize to the 6-field (with seconds) form the `cron` crate parses.
/// Returns None for field counts crontab does not accept.
fn to_six_field(schedule: &str) -> Option<String> {
    let expanded = match schedule {
        "@yearly" | "@annually" => "0 0 1 1 *",
        "@monthly" => "0 0 1 * *",
        "@weekly" => "0 0 * * 0",
        "@daily" | "@midnight" => "0 0 * * *",
        "@hourly" => "0 * * * *",
        other => other,
    };
    (expanded.split_whitespace().count() == 5).then(|| format!("0 {}", expanded))
}

fn describe(schedule: &str) -> String {
    if schedule == "@reboot" {
        return "At system startup".to_string();
    }
    let expanded = match schedule {
        "@yearly" | "@annually" => "0 0 1 1 *",
        "@monthly" => "0 0 1 * *",
        "@weekly" => "0 0 * * 0",
        "@daily" | "@midnight" => "0 0 * * *",
        "@hourly" => "0 * * * *",
        other => other,
    };
    cron_expression_descriptor::get_description_cron(expanded).unwrap_or_else(|_| "-".to_string())
}

fn next_run(schedule: &str) -> Option<DateTime<Local>> {
    let expr = to_six_field(schedule)?;
    Schedule::from_str(&expr)
        .ok()?
        .upcoming(Local)
        .next()
}

// --- crontab I/O ---

fn read_crontab() -> Result<String> {
    let output = Command::new("crontab")
        .arg("-l")
        .output()
        .context("Failed to run `crontab -l`")?;
    if output.status.success() {
        return Ok(String::from_utf8_lossy(&output.stdout).into_owned());
    }
    let stderr = String::from_utf8_lossy(&output.stderr);
    if stderr.contains("no crontab for") {
        return Ok(String::new());
    }
    bail!("`crontab -l` failed: {}", stderr.trim());
}

fn write_crontab(content: &str) -> Result<()> {
    let mut child = Command::new("crontab")
        .arg("-")
        .stdin(Stdio::piped())
        .spawn()
        .context("Failed to run `crontab -`")?;
    child
        .stdin
        .take()
        .expect("stdin was piped")
        .write_all(content.as_bytes())
        .context("Failed to write to `crontab -`")?;
    let status = child.wait()?;
    if !status.success() {
        bail!("crontab rejected the new table (exit: {})", status);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "# backups\nMAILTO=me@example.com\n0 9 * * 1-5 /usr/local/bin/backup.sh\n\n@reboot /usr/local/bin/warmup.sh\n";

    #[test]
    fn parse_classifies_entries_comments_and_env() {
        let lines = parse(SAMPLE);
        assert!(matches!(lines[0], Line::Other(_))); // comment
        assert!(matches!(lines[1], Line::Other(_))); // env
        assert!(matches!(lines[2], Line::Entry { .. }));
        assert!(matches!(lines[3], Line::Other(_))); // blank
        assert!(matches!(lines[4], Line::Entry { .. }));
    }

    #[test]
    fn render_round_trips_verbatim() {
        assert_eq!(render(&parse(SAMPLE)), SAMPLE);
        assert_eq!(render(&parse("")), "");
    }

    #[test]
    fn add_appends_entry_with_comment() {
        let mut lines = parse(SAMPLE);
        add_entry(&mut lines, "*/5 * * * *", "echo hi", Some("say hi"));
        assert!(render(&lines).ends_with("# say hi\n*/5 * * * * echo hi\n"));
        assert_eq!(entry_positions(&lines).len(), 3);
    }

    #[test]
    fn remove_targets_nth_entry_ignoring_other_lines() {
        let mut lines = parse(SAMPLE);
        let removed = remove_entry(&mut lines, 2).unwrap();
        assert_eq!(removed, "@reboot /usr/local/bin/warmup.sh");
        assert_eq!(entry_positions(&lines).len(), 1);
        assert!(remove_entry(&mut lines, 2).is_err());
        assert!(remove_entry(&mut lines, 0).is_err());
    }

    #[test]
    fn edit_replaces_schedule_and_command() {
        let mut lines = parse(SAMPLE);
        let (old, new) = edit_entry(&mut lines, 1, Some("30 8 * * *"), None).unwrap();
        assert_eq!(old, "0 9 * * 1-5 /usr/local/bin/backup.sh");
        assert_eq!(new, "30 8 * * * /usr/local/bin/backup.sh");
        let (_, new) = edit_entry(&mut lines, 1, None, Some("echo done")).unwrap();
        assert_eq!(new, "30 8 * * * echo done");
    }

    #[test]
    fn validate_accepts_five_field_and_specials_rejects_junk() {
        assert!(validate_schedule("*/5 * * * *").is_ok());
        assert!(validate_schedule("@daily").is_ok());
        assert!(validate_schedule("@reboot").is_ok());
        assert!(validate_schedule("0 0 * *").is_err()); // 4 fields
        assert!(validate_schedule("0 0 * * * *").is_err()); // 6 fields
        assert!(validate_schedule("99 * * * *").is_err()); // bad minute
    }

    #[test]
    fn describe_and_next_run_handle_specials() {
        assert_eq!(describe("@reboot"), "At system startup");
        assert!(next_run("@reboot").is_none());
        assert!(next_run("@daily").is_some());
        assert!(!describe("0 9 * * 1-5").is_empty());
    }

    #[test]
    fn unparsable_lines_are_preserved_not_dropped() {
        let weird = "this is not a cron line\n";
        assert_eq!(render(&parse(weird)), weird);
        assert!(entry_positions(&parse(weird)).is_empty());
    }
}
