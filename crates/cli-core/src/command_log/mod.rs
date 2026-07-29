use anyhow::{Context, Result};
use chrono::NaiveDateTime;
use std::env;
use std::fs::{self, File};
#[cfg(unix)]
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};

#[cfg(target_os = "macos")]
mod macos_notification;

/// A command launched in the background. The process keeps running after this
/// handle is dropped; callers that need to wait for completion (e.g. tests)
/// can do so through `child`.
pub struct Spawned {
    pub log_path: PathBuf,
    pub child: Child,
}

#[derive(Clone, Copy)]
enum CompletionNotification {
    Disabled,
    System,
}

pub const TERMINAL_FOCUS_FLAG: &str = "--__zzz-focus-terminal";

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
    run_with_home_timestamp_and_notification(
        command,
        args,
        home_dir,
        timestamp,
        CompletionNotification::Disabled,
    )
}

fn run_with_home_timestamp_and_notification(
    command: &str,
    args: &[String],
    home_dir: &Path,
    timestamp: NaiveDateTime,
    completion_notification: CompletionNotification,
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

    let child = spawn_command(command, args, &log_path, stdout, completion_notification)?;

    Ok(Spawned { log_path, child })
}

#[cfg(unix)]
fn spawn_command(
    command: &str,
    args: &[String],
    log_path: &Path,
    stdout: File,
    completion_notification: CompletionNotification,
) -> Result<Child> {
    let shell = user_shell();
    let script = match completion_notification {
        CompletionNotification::Disabled => shell_script(command, args, log_path),
        CompletionNotification::System => {
            shell_script_with_system_notification(command, args, log_path)
        }
    };

    drop(stdout);

    let mut process = Command::new(&shell);
    process
        .arg("-i")
        .arg("-c")
        .arg(&script)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());

    // Start a new session instead of only a new process group. An interactive
    // shell in a background process group can be stopped by terminal job
    // control before it runs the command; a session without a controlling
    // terminal remains independent after zzz exits.
    unsafe {
        process.pre_exec(|| {
            if libc::setsid() == -1 {
                Err(std::io::Error::last_os_error())
            } else {
                Ok(())
            }
        });
    }

    process.spawn().with_context(|| {
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
    _completion_notification: CompletionNotification,
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

#[cfg(target_os = "macos")]
fn shell_script_with_system_notification(
    command: &str,
    args: &[String],
    log_path: &Path,
) -> String {
    let command_name = command_name_from_command(command);
    let notifications = macos_notification::completion_commands(&command_name);
    shell_script_with_notification_commands(
        command,
        args,
        log_path,
        &notifications.succeeded,
        &notifications.failed,
    )
}

#[cfg(target_os = "macos")]
fn shell_script_with_notification_commands(
    command: &str,
    args: &[String],
    log_path: &Path,
    success_command: &str,
    failure_command: &str,
) -> String {
    format!(
        "set +e\n{}\nzzz_command_status=$?\n\
         if [ \"$zzz_command_status\" -eq 0 ]; then\n  ( {} )\n\
         else\n  ( {} )\n\
         fi\nexit \"$zzz_command_status\"",
        shell_script(command, args, log_path),
        success_command,
        failure_command,
    )
}

#[cfg(all(unix, not(target_os = "macos")))]
fn shell_script_with_system_notification(
    command: &str,
    args: &[String],
    log_path: &Path,
) -> String {
    shell_script(command, args, log_path)
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
    run_with_completion_notification(command, args, CompletionNotification::Disabled)
}

pub fn run_with_system_notification(command: &str, args: Vec<String>) -> Result<()> {
    run_with_completion_notification(command, args, CompletionNotification::System)
}

pub fn spawn(command: &str, args: Vec<String>) -> Result<Spawned> {
    spawn_with_completion_notification(command, args, CompletionNotification::Disabled)
}

pub fn spawn_with_system_notification(command: &str, args: Vec<String>) -> Result<Spawned> {
    spawn_with_completion_notification(command, args, CompletionNotification::System)
}

#[cfg(target_os = "macos")]
pub fn focus_terminal(kind: &str, locator: &str) -> Result<()> {
    macos_notification::focus_terminal(kind, locator)
}

#[cfg(not(target_os = "macos"))]
pub fn focus_terminal(_kind: &str, _locator: &str) -> Result<()> {
    anyhow::bail!("Terminal notification focus is only supported on macOS")
}

fn run_with_completion_notification(
    command: &str,
    args: Vec<String>,
    completion_notification: CompletionNotification,
) -> Result<()> {
    let _ = spawn_with_completion_notification(command, args, completion_notification)?;
    Ok(())
}

fn spawn_with_completion_notification(
    command: &str,
    args: Vec<String>,
    completion_notification: CompletionNotification,
) -> Result<Spawned> {
    // Launch the command in the background and return to the shell immediately.
    // Output is still captured to the log file; dropping the handle leaves the
    // detached process running.
    run_with_home_timestamp_and_notification(
        command,
        &args,
        &home_dir()?,
        chrono::Local::now().naive_local(),
        completion_notification,
    )
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
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    static UNIQUE_COUNTER: AtomicU64 = AtomicU64::new(0);

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

        assert_eq!(
            spawned.log_path,
            home.join(".commands/260605/142233-echo.log")
        );
        assert_eq!(
            fs::read_to_string(&spawned.log_path).unwrap(),
            "captured output\n"
        );

        fs::remove_dir_all(home).unwrap();
    }

    #[test]
    #[cfg(target_os = "macos")]
    fn interactive_shell_runs_in_a_detached_session_from_a_real_terminal() {
        const CHILD_ENV: &str = "ZZZ_INTERACTIVE_SHELL_PTY_TEST_CHILD";
        if env::var_os(CHILD_ENV).is_some() {
            let home = unique_temp_home();
            let timestamp = NaiveDate::from_ymd_opt(2026, 7, 30)
                .unwrap()
                .and_hms_opt(3, 0, 0)
                .unwrap();
            let args = vec!["-c".to_string(), "sleep 0.25; printf detached".to_string()];
            let mut spawned =
                run_with_home_and_timestamp("/bin/sh", &args, &home, timestamp).unwrap();

            let session_id = unsafe { libc::getsid(spawned.child.id() as libc::pid_t) };
            assert_ne!(session_id, -1, "could not read command shell session");
            assert_eq!(
                session_id as u32,
                spawned.child.id(),
                "command shell did not start in its own session"
            );

            for _ in 0..1_000 {
                if let Some(status) = spawned.child.try_wait().unwrap() {
                    assert!(status.success(), "shell exited with {status}");
                    assert_eq!(fs::read_to_string(&spawned.log_path).unwrap(), "detached");
                    fs::remove_dir_all(home).unwrap();
                    return;
                }
                std::thread::sleep(Duration::from_millis(10));
            }

            let process = Command::new("/bin/ps")
                .args([
                    "-o",
                    "pid=,ppid=,pgid=,tpgid=,stat=,command=",
                    "-p",
                    &spawned.child.id().to_string(),
                ])
                .output()
                .unwrap();
            spawned.child.kill().unwrap();
            spawned.child.wait().unwrap();
            fs::remove_dir_all(home).unwrap();
            panic!(
                "interactive shell stopped instead of running the command:\n{}",
                String::from_utf8_lossy(&process.stdout)
            );
        }

        let output = Command::new("/usr/bin/script")
            .args(["-q", "/dev/null"])
            .arg(env::current_exe().unwrap())
            .args([
                "--exact",
                "command_log::tests::interactive_shell_runs_in_a_detached_session_from_a_real_terminal",
                "--nocapture",
            ])
            .env(CHILD_ENV, "1")
            .output()
            .unwrap();

        assert!(
            output.status.success(),
            "PTY child failed:\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }

    #[test]
    #[cfg(target_os = "macos")]
    fn completion_wrapper_notifies_when_command_succeeds() {
        let fixture = notification_fixture();
        let args = vec!["completed".to_string()];
        let script = shell_script_with_notification_commands(
            "printf",
            &args,
            &fixture.log_path,
            &notification_test_command(&fixture.notification_path, "Succeeded"),
            &notification_test_command(&fixture.notification_path, "Failed"),
        );

        let status = run_test_shell(&script);

        assert!(status.success());
        assert_eq!(fs::read_to_string(&fixture.log_path).unwrap(), "completed");
        assert_eq!(
            fs::read_to_string(&fixture.notification_path).unwrap(),
            "Succeeded\n"
        );

        fs::remove_dir_all(fixture.home).unwrap();
    }

    #[test]
    #[cfg(target_os = "macos")]
    fn completion_wrapper_notifies_and_preserves_command_failure() {
        let fixture = notification_fixture();
        let args = vec!["-c".to_string(), "exit 7".to_string()];
        let script = shell_script_with_notification_commands(
            "sh",
            &args,
            &fixture.log_path,
            &notification_test_command(&fixture.notification_path, "Succeeded"),
            &notification_test_command(&fixture.notification_path, "Failed"),
        );

        let status = run_test_shell(&script);

        assert_eq!(status.code(), Some(7));
        assert_eq!(
            fs::read_to_string(&fixture.notification_path).unwrap(),
            "Failed\n"
        );

        fs::remove_dir_all(fixture.home).unwrap();
    }

    #[test]
    #[cfg(target_os = "macos")]
    fn completion_wrapper_preserves_status_when_notification_fails() {
        let fixture = notification_fixture();
        let args = vec!["-c".to_string(), "exit 7".to_string()];
        let script = shell_script_with_notification_commands(
            "sh",
            &args,
            &fixture.log_path,
            "exit 9",
            "exit 9",
        );

        let status = run_test_shell(&script);

        assert_eq!(status.code(), Some(7));
        fs::remove_dir_all(fixture.home).unwrap();
    }

    #[cfg(target_os = "macos")]
    struct NotificationFixture {
        home: PathBuf,
        log_path: PathBuf,
        notification_path: PathBuf,
    }

    #[cfg(target_os = "macos")]
    fn notification_fixture() -> NotificationFixture {
        let home = unique_temp_home();
        fs::create_dir_all(&home).unwrap();

        let log_path = home.join("command.log");
        let notification_path = home.join("notification.log");

        NotificationFixture {
            home,
            log_path,
            notification_path,
        }
    }

    #[cfg(target_os = "macos")]
    fn notification_test_command(path: &Path, outcome: &str) -> String {
        format!(
            "printf '%s\\n' {} > {}",
            shell_quote(outcome),
            shell_quote(&path.to_string_lossy())
        )
    }

    #[cfg(target_os = "macos")]
    fn run_test_shell(script: &str) -> std::process::ExitStatus {
        Command::new("/bin/sh")
            .arg("-c")
            .arg(script)
            .status()
            .unwrap()
    }

    fn unique_temp_home() -> PathBuf {
        let mut path = std::env::temp_dir();
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let counter = UNIQUE_COUNTER.fetch_add(1, Ordering::Relaxed);
        path.push(format!(
            "cli-tools-command-log-test-{}-{}-{}",
            std::process::id(),
            nanos,
            counter
        ));
        path
    }
}
