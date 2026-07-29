#![cfg(target_os = "macos")]

use std::process::Command;

const FOCUS_FLAG: &str = "--__zzz-focus-terminal";

#[test]
fn focus_mode_rejects_unknown_terminal_kind() {
    let output = Command::new(zzz_bin())
        .args([FOCUS_FLAG, "ghostty", "/dev/ttys014"])
        .output()
        .unwrap();

    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("Unsupported terminal focus kind"));
}

#[test]
fn focus_mode_rejects_malformed_iterm_session() {
    let output = Command::new(zzz_bin())
        .args([FOCUS_FLAG, "iterm2", "not-a-uuid"])
        .output()
        .unwrap();

    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("Invalid iTerm2 session locator"));
}

#[test]
fn focus_mode_rejects_malformed_terminal_tty() {
    let output = Command::new(zzz_bin())
        .args([FOCUS_FLAG, "terminal", "/tmp/not-a-tty"])
        .output()
        .unwrap();

    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("Invalid Terminal.app TTY locator"));
}

#[test]
fn focus_mode_requires_exactly_kind_and_locator() {
    let missing = Command::new(zzz_bin()).arg(FOCUS_FLAG).output().unwrap();
    assert!(!missing.status.success());
    assert!(String::from_utf8_lossy(&missing.stderr).contains("Missing terminal focus kind"));

    let extra = Command::new(zzz_bin())
        .args([FOCUS_FLAG, "terminal", "/dev/ttys014", "extra"])
        .output()
        .unwrap();
    assert!(!extra.status.success());
    assert!(String::from_utf8_lossy(&extra.stderr).contains("Unexpected terminal focus arguments"));
}

fn zzz_bin() -> &'static str {
    env!("CARGO_BIN_EXE_zzz")
}
