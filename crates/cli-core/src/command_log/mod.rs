use anyhow::{Context, Result};
use chrono::NaiveDateTime;
use std::env;
use std::fs::{self, File};
#[cfg(unix)]
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};

/// A command launched in the background. The process keeps running after this
/// handle is dropped; callers that need to wait for completion (e.g. tests)
/// can do so through `child`.
pub struct Spawned {
    pub log_path: PathBuf,
    pub child: Child,
}

pub fn command_name_from_command(command: &str) -> String {
    let name = Path::new(command)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or(command);

    let safe_name: String = name
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '.') {
                ch
            } else {
                '_'
            }
        })
        .collect();

    let safe_name = safe_name.trim_matches('_');
    if safe_name.is_empty() {
        "command".to_string()
    } else {
        safe_name.to_string()
    }
}

pub fn command_log_path(home_dir: &Path, timestamp: NaiveDateTime, command: &str) -> PathBuf {
    home_dir
        .join(".commands")
        .join(timestamp.format("%y%m%d").to_string())
        .join(format!(
            "{}-{}.log",
            timestamp.format("%H%M%S"),
            command_name_from_command(command)
        ))
}

pub fn run_with_home_and_timestamp(
    command: &str,
    args: &[String],
    home_dir: &Path,
    timestamp: NaiveDateTime,
) -> Result<Spawned> {
    let log_path = command_log_path(home_dir, timestamp, command);
    if let Some(log_dir) = log_path.parent() {
        fs::create_dir_all(log_dir).with_context(|| {
            format!(
                "Failed to create command log directory '{}'",
                log_dir.display()
            )
        })?;
    }

    let stdout = File::create(&log_path)
        .with_context(|| format!("Failed to create command log '{}'", log_path.display()))?;

    let child = spawn_command(command, args, &log_path, stdout)?;

    Ok(Spawned { log_path, child })
}

#[cfg(unix)]
fn spawn_command(
    command: &str,
    args: &[String],
    log_path: &Path,
    stdout: File,
) -> Result<Child> {
    let shell = user_shell();
    let script = shell_script(command, args, log_path);

    drop(stdout);

    Command::new(&shell)
        .arg("-i")
        .arg("-c")
        .arg(&script)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        // Detach into a new process group so the command keeps running in the
        // background and is unaffected by job-control signals (Ctrl+C/Ctrl+Z)
        // sent to the shell that launched it.
        .process_group(0)
        .spawn()
        .with_context(|| {
            format!(
                "Failed to start shell '{}' for command '{}'",
                shell, command
            )
        })
}

#[cfg(not(unix))]
fn spawn_command(
    command: &str,
    args: &[String],
    _log_path: &Path,
    stdout: File,
) -> Result<Child> {
    Command::new(command)
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::from(stdout))
        .stderr(Stdio::null())
        .spawn()
        .with_context(|| format!("Failed to run command '{}'", command))
}

#[cfg(unix)]
fn shell_script(command: &str, args: &[String], log_path: &Path) -> String {
    format!(
        "exec > {}\n{}",
        shell_quote(&log_path.to_string_lossy()),
        shell_command_line(command, args)
    )
}

#[cfg(unix)]
fn user_shell() -> String {
    env::var("SHELL")
        .ok()
        .filter(|shell| !shell.trim().is_empty())
        .unwrap_or_else(|| "/bin/sh".to_string())
}

#[cfg(unix)]
fn shell_command_line(command: &str, args: &[String]) -> String {
    std::iter::once(shell_command_word(command))
        .chain(args.iter().map(|arg| shell_quote(arg)))
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(unix)]
fn shell_command_word(command: &str) -> String {
    if is_shell_bareword(command) {
        command.to_string()
    } else {
        shell_quote(command)
    }
}

#[cfg(unix)]
fn is_shell_bareword(value: &str) -> bool {
    !value.is_empty()
        && value.chars().all(|ch| {
            ch.is_ascii_alphanumeric()
                || matches!(
                    ch,
                    '_' | '-' | '.' | '/' | ':' | '+' | '=' | '@' | '%' | '~'
                )
        })
}

#[cfg(unix)]
fn shell_quote(value: &str) -> String {
    if value.is_empty() {
        return "''".to_string();
    }

    format!("'{}'", value.replace('\'', "'\\''"))
}

pub fn run(command: &str, args: Vec<String>) -> Result<()> {
    // Launch the command in the background and return to the shell immediately.
    // Output is still captured to the log file; dropping the handle leaves the
    // detached process running.
    let _ = run_with_home_and_timestamp(
        command,
        &args,
        &home_dir()?,
        chrono::Local::now().naive_local(),
    )?;
    Ok(())
}

fn home_dir() -> Result<PathBuf> {
    env::var_os("HOME")
        .or_else(|| env::var_os("USERPROFILE"))
        .map(PathBuf::from)
        .context("Could not determine home directory from HOME or USERPROFILE")
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn command_log_path_uses_yymmdd_hhmmss_and_command_name() {
        let timestamp = NaiveDate::from_ymd_opt(2026, 6, 5)
            .unwrap()
            .and_hms_opt(14, 22, 33)
            .unwrap();

        let path = command_log_path(Path::new("/tmp/home"), timestamp, "git");

        assert_eq!(
            path,
            PathBuf::from("/tmp/home/.commands/260605/142233-git.log")
        );
    }

    #[test]
    fn command_name_uses_first_executable_basename_and_safe_filename_chars() {
        assert_eq!(command_name_from_command("echo"), "echo");
        assert_eq!(
            command_name_from_command("/usr/local/bin/python3"),
            "python3"
        );
        assert_eq!(
            command_name_from_command("../weird command"),
            "weird_command"
        );
    }

    #[test]
    #[cfg(unix)]
    fn shell_command_line_leaves_safe_command_unquoted_for_alias_expansion() {
        let args = vec![
            "hello world".to_string(),
            "it's quoted".to_string(),
            "--flag=value".to_string(),
        ];

        assert_eq!(
            shell_command_line("update-agents", &args),
            "update-agents 'hello world' 'it'\\''s quoted' '--flag=value'"
        );
    }

    #[test]
    fn run_captures_stdout_to_log_without_printing_it() {
        let home = unique_temp_home();
        let timestamp = NaiveDate::from_ymd_opt(2026, 6, 5)
            .unwrap()
            .and_hms_opt(14, 22, 33)
            .unwrap();
        let args = vec!["captured output".to_string()];

        let mut spawned = run_with_home_and_timestamp("echo", &args, &home, timestamp).unwrap();
        // The command runs in the background, so wait for it before reading the log.
        spawned.child.wait().unwrap();

        assert_eq!(spawned.log_path, home.join(".commands/260605/142233-echo.log"));
        assert_eq!(
            fs::read_to_string(&spawned.log_path).unwrap(),
            "captured output\n"
        );

        fs::remove_dir_all(home).unwrap();
    }

    fn unique_temp_home() -> PathBuf {
        let mut path = std::env::temp_dir();
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        path.push(format!(
            "cli-tools-command-log-test-{}-{}",
            std::process::id(),
            nanos
        ));
        path
    }
}
