use anyhow::{bail, Context, Result};
use chrono::NaiveDateTime;
use std::env;
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

pub(crate) fn command_name_from_command(command: &str) -> String {
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

pub(crate) fn command_log_path(
    home_dir: &Path,
    timestamp: NaiveDateTime,
    command: &str,
) -> PathBuf {
    home_dir
        .join(".commands")
        .join(timestamp.format("%y%m%d").to_string())
        .join(format!(
            "{}-{}.log",
            timestamp.format("%H%M%S"),
            command_name_from_command(command)
        ))
}

pub(crate) fn run_with_home_and_timestamp(
    command: &str,
    args: &[String],
    home_dir: &Path,
    timestamp: NaiveDateTime,
) -> Result<PathBuf> {
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

    let status = Command::new(command)
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::from(stdout))
        .stderr(Stdio::null())
        .status()
        .with_context(|| format!("Failed to run command '{}'", command))?;

    if !status.success() {
        bail!(
            "Command '{}' exited with status {}. stdout was saved to '{}'",
            command,
            status,
            log_path.display()
        );
    }

    Ok(log_path)
}

pub fn run(command: &str, args: Vec<String>) -> Result<()> {
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
    fn run_captures_stdout_to_log_without_printing_it() {
        let home = unique_temp_home();
        let timestamp = NaiveDate::from_ymd_opt(2026, 6, 5)
            .unwrap()
            .and_hms_opt(14, 22, 33)
            .unwrap();
        let args = vec!["captured output".to_string()];

        let log_path = run_with_home_and_timestamp("echo", &args, &home, timestamp).unwrap();

        assert_eq!(log_path, home.join(".commands/260605/142233-echo.log"));
        assert_eq!(fs::read_to_string(log_path).unwrap(), "captured output\n");

        fs::remove_dir_all(home).unwrap();
    }

    fn unique_temp_home() -> PathBuf {
        let mut path = std::env::temp_dir();
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        path.push(format!(
            "dev-tools-silent-test-{}-{}",
            std::process::id(),
            nanos
        ));
        path
    }
}
