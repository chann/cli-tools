use std::fs;
#[cfg(target_os = "macos")]
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

static UNIQUE_COUNTER: AtomicU64 = AtomicU64::new(0);

#[test]
fn help_is_readable_and_documents_convenience_options() {
    let output = Command::new(zzz_bin())
        .args(["--color", "never", "--help"])
        .output()
        .unwrap();

    assert!(output.status.success(), "zzz failed: {output:?}");
    assert_eq!(output.stderr, b"");

    let help = String::from_utf8(output.stdout).unwrap();
    assert!(help.contains("Run a command quietly in the background"));
    assert!(help.contains("Usage: zzz [OPTIONS] <COMMAND>..."));
    assert!(help.contains("Options:"));
    assert!(help.contains("--no-notify"));
    assert!(help.contains("--wait"));
    assert!(help.contains("--print-log"));
    assert!(help.contains("--color <COLOR>"));
    assert!(help.contains("Examples:"));
    assert!(help.contains("zzz --wait cargo test"));
    assert!(!help.contains("\u{1b}["));
}

#[test]
fn help_can_force_color_for_non_interactive_output() {
    let output = Command::new(zzz_bin())
        .args(["--color", "always", "--help"])
        .output()
        .unwrap();

    assert!(output.status.success(), "zzz failed: {output:?}");
    assert!(
        String::from_utf8(output.stdout)
            .unwrap()
            .contains("\u{1b}["),
        "forced-color help did not contain ANSI styling"
    );
}

#[test]
#[cfg(unix)]
fn wait_returns_the_background_command_exit_status() {
    let home = unique_temp_home();
    fs::create_dir_all(&home).unwrap();

    let output = Command::new(zzz_bin())
        .args(["--no-notify", "--wait", "sh", "-c", "exit 7"])
        .env("HOME", &home)
        .env("ZDOTDIR", &home)
        .env_remove("USERPROFILE")
        .output()
        .unwrap();

    assert_eq!(
        output.status.code(),
        Some(7),
        "unexpected result: {output:?}"
    );
    fs::remove_dir_all(home).unwrap();
}

#[test]
#[cfg(unix)]
fn print_log_reports_the_completed_commands_log_path() {
    let home = unique_temp_home();
    fs::create_dir_all(&home).unwrap();

    let output = Command::new(zzz_bin())
        .args([
            "--no-notify",
            "--wait",
            "--print-log",
            "printf",
            "%s",
            "captured",
        ])
        .env("HOME", &home)
        .env("ZDOTDIR", &home)
        .env_remove("USERPROFILE")
        .output()
        .unwrap();

    assert!(output.status.success(), "zzz failed: {output:?}");
    assert_eq!(output.stderr, b"");

    let log_path = PathBuf::from(String::from_utf8(output.stdout).unwrap().trim());
    assert!(log_path.starts_with(home.join(".commands")));
    assert_eq!(fs::read_to_string(log_path).unwrap(), "captured");

    fs::remove_dir_all(home).unwrap();
}

#[test]
#[cfg(unix)]
fn command_arguments_that_look_like_zzz_options_are_forwarded() {
    let home = unique_temp_home();
    fs::create_dir_all(&home).unwrap();

    let output = Command::new(zzz_bin())
        .args(["--no-notify", "--wait", "printf", "%s", "--print-log"])
        .env("HOME", &home)
        .env("ZDOTDIR", &home)
        .env_remove("USERPROFILE")
        .output()
        .unwrap();

    assert!(output.status.success(), "zzz failed: {output:?}");
    let log = wait_for_single_log(&home, "--print-log");
    assert_eq!(fs::read_to_string(log).unwrap(), "--print-log");

    fs::remove_dir_all(home).unwrap();
}

#[test]
fn zzz_runs_command_silently_and_saves_stdout() {
    let home = unique_temp_home();
    fs::create_dir_all(&home).unwrap();

    let output = Command::new(zzz_bin())
        .arg("echo")
        .arg("hello")
        .env("HOME", &home)
        .env("ZDOTDIR", &home)
        .env_remove("USERPROFILE")
        .output()
        .unwrap();

    assert!(output.status.success(), "zzz failed: {output:?}");
    assert_eq!(output.stdout, b"");
    assert_eq!(output.stderr, b"");

    // zzz returns immediately; the command finishes writing the log in the background.
    let log = wait_for_single_log(&home, "hello\n");
    let log_name = log.file_name().unwrap().to_string_lossy();
    assert!(
        log_name.ends_with("-echo.log"),
        "unexpected log name: {log_name}"
    );

    fs::remove_dir_all(home).unwrap();
}

#[test]
#[cfg(unix)]
fn zzz_runs_zsh_alias_from_user_rc_file() {
    let Some(zsh) = zsh_path() else {
        return;
    };
    let home = unique_temp_home();
    fs::create_dir_all(&home).unwrap();
    fs::write(
        home.join(".zshrc"),
        "printf \"%s\\n\" startup-output\nalias update-agents='printf \"%s\\n\" alias-output'\n",
    )
    .unwrap();

    let output = Command::new(zzz_bin())
        .arg("update-agents")
        .env("HOME", &home)
        .env("ZDOTDIR", &home)
        .env("SHELL", zsh)
        .env_remove("USERPROFILE")
        .output()
        .unwrap();

    assert!(output.status.success(), "zzz failed: {output:?}");
    assert_eq!(output.stdout, b"");
    assert_eq!(output.stderr, b"");

    let log = wait_for_single_log(&home, "alias-output\n");
    let log_name = log.file_name().unwrap().to_string_lossy();
    assert!(
        log_name.ends_with("-update-agents.log"),
        "unexpected log name: {log_name}"
    );

    fs::remove_dir_all(home).unwrap();
}

#[test]
#[cfg(target_os = "macos")]
fn completion_notification_launches_alerter_in_a_detached_worker() {
    let home = unique_temp_home();
    let bin_dir = home.join("bin");
    let capture_path = home.join("alerter-arguments");
    fs::create_dir_all(&bin_dir).unwrap();

    let alerter_path = bin_dir.join("alerter");
    fs::write(
        &alerter_path,
        "#!/bin/sh\nprintf '%s\\n' \"$@\" > \"$ZZZ_ALERTER_CAPTURE\"\nprintf '@TIMEOUT\\n'\n",
    )
    .unwrap();
    let mut permissions = fs::metadata(&alerter_path).unwrap().permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(&alerter_path, permissions).unwrap();

    let output = Command::new(zzz_bin())
        .args([
            "--__zzz-notify-terminal",
            "iterm2",
            "F8176E01-630A-4505-9B2B-0DE870BF4706",
            "Succeeded",
            "echo",
        ])
        .env("PATH", &bin_dir)
        .env("ZZZ_ALERTER_CAPTURE", &capture_path)
        .output()
        .unwrap();

    assert!(output.status.success(), "zzz failed: {output:?}");
    assert_eq!(output.stdout, b"");
    assert_eq!(output.stderr, b"");

    let captured = (0..200)
        .find_map(|_| {
            if let Ok(captured) = fs::read_to_string(&capture_path) {
                if !captured.is_empty() {
                    return Some(captured);
                }
            }
            std::thread::sleep(Duration::from_millis(10));
            None
        })
        .unwrap_or_else(|| {
            panic!(
                "detached alerter worker did not write '{}'",
                capture_path.display()
            )
        });

    assert!(captured.contains("--title\nzzz\n"));
    assert!(captured.contains("--subtitle\nSucceeded\n"));
    assert!(captured.contains("--message\necho\n"));
    assert!(captured.contains("--group\nzzz-iterm2-F8176E01-630A-4505-9B2B-0DE870BF4706\n"));
    assert!(captured.contains("--app-icon\n"));
    assert!(!captured.contains("--sender\n"));

    fs::remove_dir_all(home).unwrap();
}

#[test]
#[cfg(target_os = "macos")]
fn iterm_completion_posts_a_native_notification_without_session_metadata() {
    let home = unique_temp_home();
    fs::create_dir_all(&home).unwrap();

    let output = Command::new("/usr/bin/script")
        .args(["-q", "/dev/null"])
        .arg(zzz_bin())
        .args(["--wait", "sh", "-c", "exit 0"])
        .env("HOME", &home)
        .env("ZDOTDIR", &home)
        .env("SHELL", "/bin/zsh")
        .env("TERM_PROGRAM", "iTerm.app")
        .env("__CFBundleIdentifier", "com.googlecode.iterm2")
        .env_remove("TERM_SESSION_ID")
        .env_remove("USERPROFILE")
        .output()
        .unwrap();

    assert!(output.status.success(), "zzz failed: {output:?}");
    let transcript = String::from_utf8_lossy(&output.stdout);
    assert!(
        transcript.contains("\u{1b}]9;zzz Succeeded: sh\u{1b}\\"),
        "iTerm2 notification escape was not written to the originating TTY:\n{transcript}"
    );

    fs::remove_dir_all(home).unwrap();
}

fn zzz_bin() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_zzz"))
}

#[cfg(unix)]
fn zsh_path() -> Option<&'static str> {
    ["/bin/zsh", "/usr/bin/zsh"]
        .into_iter()
        .find(|path| Path::new(path).exists())
}

/// Poll until exactly one log file exists with the expected content, since the
/// command runs in the background after zzz returns. Panics on timeout.
fn wait_for_single_log(home: &Path, expected_content: &str) -> PathBuf {
    for _ in 0..400 {
        let logs = command_logs(home);
        if logs.len() == 1 {
            if let Ok(content) = fs::read_to_string(&logs[0]) {
                if content == expected_content {
                    return logs[0].clone();
                }
            }
        }
        std::thread::sleep(Duration::from_millis(25));
    }

    panic!(
        "expected a single log with {expected_content:?}, found: {:?}",
        command_logs(home)
    );
}

fn command_logs(home: &Path) -> Vec<PathBuf> {
    let mut logs = Vec::new();
    let commands_dir = home.join(".commands");
    if !commands_dir.exists() {
        return logs;
    }

    for date_entry in fs::read_dir(commands_dir).unwrap() {
        let date_path = date_entry.unwrap().path();
        if !date_path.is_dir() {
            continue;
        }

        for log_entry in fs::read_dir(date_path).unwrap() {
            logs.push(log_entry.unwrap().path());
        }
    }

    logs.sort();
    logs
}

fn unique_temp_home() -> PathBuf {
    let mut path = std::env::temp_dir();
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let counter = UNIQUE_COUNTER.fetch_add(1, Ordering::Relaxed);
    path.push(format!(
        "zzz-test-{}-{}-{}",
        std::process::id(),
        nanos,
        counter
    ));
    path
}
