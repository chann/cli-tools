use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

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

    let logs = command_logs(&home);
    assert_eq!(logs.len(), 1);
    let log_name = logs[0].file_name().unwrap().to_string_lossy();
    assert!(
        log_name.ends_with("-echo.log"),
        "unexpected log name: {log_name}"
    );
    assert_eq!(fs::read_to_string(&logs[0]).unwrap(), "hello\n");

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

    let logs = command_logs(&home);
    assert_eq!(logs.len(), 1);
    let log_name = logs[0].file_name().unwrap().to_string_lossy();
    assert!(
        log_name.ends_with("-update-agents.log"),
        "unexpected log name: {log_name}"
    );
    assert_eq!(fs::read_to_string(&logs[0]).unwrap(), "alias-output\n");

    fs::remove_dir_all(home).unwrap();
}

fn zzz_bin() -> PathBuf {
    std::env::var_os("CARGO_BIN_EXE_zzz")
        .map(PathBuf::from)
        .expect("zzz binary target should be built")
}

#[cfg(unix)]
fn zsh_path() -> Option<&'static str> {
    ["/bin/zsh", "/usr/bin/zsh"]
        .into_iter()
        .find(|path| Path::new(path).exists())
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
    path.push(format!(
        "dev-tools-zzz-test-{}-{}",
        std::process::id(),
        nanos
    ));
    path
}
