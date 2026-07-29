# zzz Clickable Terminal Notifications Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make macOS `zzz` success and failure notifications show the launching terminal's icon and restore keyboard focus to the exact launching iTerm2 session or Terminal.app tab when clicked.

**Architecture:** Capture an opaque terminal target before spawning the detached command, construct either clickable `terminal-notifier` commands or the existing `osascript` fallback, and preserve the background command status. Route notification clicks back through a validated internal `zzz` mode that runs constant AppleScript with the locator supplied as a positional argument.

**Tech Stack:** Rust 2021, POSIX shell generation, macOS AppleScript, `terminal-notifier` 2.x, Cargo unit/integration tests.

---

## File Structure

- Create `crates/cli-core/src/command_log/macos_notification.rs`: detect iTerm2
  and Terminal.app targets, resolve notifier/icon paths, build completion
  commands, validate focus locators, and execute constant AppleScript adapters.
- Modify `crates/cli-core/src/command_log/mod.rs`: expose the private focus-mode
  flag and handler, delegate macOS completion command construction, and run
  status-preserving success/failure command lines.
- Modify `crates/zzz/src/main.rs`: dispatch the validated internal focus mode
  before normal command execution.
- Create `crates/zzz/tests/focus.rs`: prove the internal mode rejects unknown
  terminal kinds and malformed locators before invoking AppleScript.
- Modify `README.md`: document clickable notifications, supported terminals,
  fallback behavior, and `terminal-notifier` installation.
- Modify `README.ko.md`: mirror the same behavior and dependency in Korean.

The implementation remains in the existing `command_log` module because
notification delivery is part of command completion. The macOS-only file keeps
terminal adapters out of the platform-neutral process runner.

### Task 1: Detect and validate exact terminal targets

**Files:**
- Create: `crates/cli-core/src/command_log/macos_notification.rs`
- Modify: `crates/cli-core/src/command_log/mod.rs`

- [ ] **Step 1: Register the macOS module and internal focus flag**

Add near the top of `crates/cli-core/src/command_log/mod.rs`:

```rust
#[cfg(target_os = "macos")]
mod macos_notification;

pub const TERMINAL_FOCUS_FLAG: &str = "--__zzz-focus-terminal";
```

Add the public platform wrapper:

```rust
#[cfg(target_os = "macos")]
pub fn focus_terminal(kind: &str, locator: &str) -> Result<()> {
    macos_notification::focus_terminal(kind, locator)
}

#[cfg(not(target_os = "macos"))]
pub fn focus_terminal(_kind: &str, _locator: &str) -> Result<()> {
    anyhow::bail!("Terminal notification focus is only supported on macOS")
}
```

- [ ] **Step 2: Write failing terminal detection tests**

Create `crates/cli-core/src/command_log/macos_notification.rs` with the test
module first:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_iterm_uuid_from_session_environment() {
        let environment = TerminalEnvironment {
            bundle_id: Some("com.googlecode.iterm2".into()),
            term_program: Some("iTerm.app".into()),
            iterm_session_id: Some(
                "w0t2p2:F8176E01-630A-4505-9B2B-0DE870BF4706".into(),
            ),
            term_session_id: None,
            tty: Some("/dev/ttys014".into()),
        };

        assert_eq!(
            detect_terminal_target(&environment),
            Some(TerminalTarget {
                kind: TerminalKind::ITerm2,
                locator: "F8176E01-630A-4505-9B2B-0DE870BF4706".into(),
            })
        );
    }

    #[test]
    fn detects_terminal_app_by_bundle_and_tty() {
        let environment = TerminalEnvironment {
            bundle_id: Some("com.apple.Terminal".into()),
            term_program: Some("Apple_Terminal".into()),
            iterm_session_id: None,
            term_session_id: None,
            tty: Some("/dev/ttys007".into()),
        };

        assert_eq!(
            detect_terminal_target(&environment),
            Some(TerminalTarget {
                kind: TerminalKind::Terminal,
                locator: "/dev/ttys007".into(),
            })
        );
    }

    #[test]
    fn rejects_invalid_iterm_and_terminal_locators() {
        assert!(TerminalTarget::parse("iterm2", "not-a-uuid").is_err());
        assert!(TerminalTarget::parse("terminal", "/tmp/not-a-tty").is_err());
        assert!(TerminalTarget::parse("unknown", "/dev/ttys001").is_err());
    }
}
```

- [ ] **Step 3: Run the detection tests and verify RED**

Run:

```bash
cargo test -p cli-core macos_notification::tests::detects -- --nocapture
cargo test -p cli-core macos_notification::tests::rejects_invalid -- --nocapture
```

Expected: compilation fails because `TerminalEnvironment`, `TerminalTarget`,
`TerminalKind`, and their parser/detector do not exist.

- [ ] **Step 4: Implement terminal target parsing and detection**

Add these definitions above the tests:

```rust
use anyhow::{bail, Context, Result};
use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum TerminalKind {
    ITerm2,
    Terminal,
}

impl TerminalKind {
    fn as_str(self) -> &'static str {
        match self {
            Self::ITerm2 => "iterm2",
            Self::Terminal => "terminal",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct TerminalTarget {
    kind: TerminalKind,
    locator: String,
}

impl TerminalTarget {
    fn parse(kind: &str, locator: &str) -> Result<Self> {
        let kind = match kind {
            "iterm2" if valid_uuid(locator) => TerminalKind::ITerm2,
            "terminal" if valid_tty(locator) => TerminalKind::Terminal,
            "iterm2" => bail!("Invalid iTerm2 session locator"),
            "terminal" => bail!("Invalid Terminal.app TTY locator"),
            _ => bail!("Unsupported terminal kind '{kind}'"),
        };
        Ok(Self {
            kind,
            locator: locator.to_owned(),
        })
    }
}

#[derive(Default)]
struct TerminalEnvironment {
    bundle_id: Option<String>,
    term_program: Option<String>,
    iterm_session_id: Option<String>,
    term_session_id: Option<String>,
    tty: Option<String>,
}

fn valid_uuid(value: &str) -> bool {
    value.len() == 36
        && value.chars().enumerate().all(|(index, ch)| match index {
            8 | 13 | 18 | 23 => ch == '-',
            _ => ch.is_ascii_hexdigit(),
        })
}

fn valid_tty(value: &str) -> bool {
    value
        .strip_prefix("/dev/tty")
        .is_some_and(|suffix| {
            !suffix.is_empty()
                && suffix
                    .chars()
                    .all(|ch| ch.is_ascii_alphanumeric() || ch == '.')
        })
}

fn iterm_uuid(value: &str) -> Option<&str> {
    let candidate = value.rsplit_once(':').map_or(value, |(_, uuid)| uuid);
    valid_uuid(candidate).then_some(candidate)
}

fn detect_terminal_target(environment: &TerminalEnvironment) -> Option<TerminalTarget> {
    let is_iterm = environment.bundle_id.as_deref() == Some("com.googlecode.iterm2")
        || environment.term_program.as_deref() == Some("iTerm.app");
    if is_iterm {
        let session = environment
            .iterm_session_id
            .as_deref()
            .or(environment.term_session_id.as_deref())?;
        return Some(TerminalTarget {
            kind: TerminalKind::ITerm2,
            locator: iterm_uuid(session)?.to_owned(),
        });
    }

    let is_terminal = environment.bundle_id.as_deref() == Some("com.apple.Terminal")
        || environment.term_program.as_deref() == Some("Apple_Terminal");
    if is_terminal {
        let locator = environment.tty.as_deref()?.to_owned();
        return valid_tty(&locator).then_some(TerminalTarget {
            kind: TerminalKind::Terminal,
            locator,
        });
    }

    None
}
```

Capture production values with:

```rust
fn controlling_tty() -> Option<String> {
    let output = Command::new("/usr/bin/tty").output().ok()?;
    if !output.status.success() {
        return None;
    }

    let tty = String::from_utf8(output.stdout).ok()?;
    let tty = tty.trim();
    valid_tty(tty).then(|| tty.to_owned())
}

fn current_terminal_environment() -> TerminalEnvironment {
    TerminalEnvironment {
        bundle_id: env::var("__CFBundleIdentifier").ok(),
        term_program: env::var("TERM_PROGRAM").ok(),
        iterm_session_id: env::var("ITERM_SESSION_ID").ok(),
        term_session_id: env::var("TERM_SESSION_ID").ok(),
        tty: controlling_tty(),
    }
}
```

- [ ] **Step 5: Run detection tests and verify GREEN**

Run:

```bash
cargo test -p cli-core macos_notification::tests::detects -- --nocapture
cargo test -p cli-core macos_notification::tests::rejects_invalid -- --nocapture
```

Expected: all three tests pass.

### Task 2: Build clickable completion commands

**Files:**
- Modify: `crates/cli-core/src/command_log/macos_notification.rs`
- Modify: `crates/cli-core/src/command_log/mod.rs`

- [ ] **Step 1: Write failing notifier resolution and argument tests**

Add to the new module tests:

```rust
#[test]
fn resolves_path_notifier_before_homebrew_fallbacks() {
    let path = std::ffi::OsStr::new("/test/bin:/other/bin");
    let resolved = find_terminal_notifier_with(Some(path), |candidate| {
        candidate == Path::new("/test/bin/terminal-notifier")
            || candidate == Path::new("/opt/homebrew/bin/terminal-notifier")
    });

    assert_eq!(
        resolved,
        Some(PathBuf::from("/test/bin/terminal-notifier"))
    );
}

#[test]
fn clickable_commands_use_icon_and_exact_focus_target() {
    let target = TerminalTarget {
        kind: TerminalKind::ITerm2,
        locator: "F8176E01-630A-4505-9B2B-0DE870BF4706".into(),
    };

    let commands = terminal_notifier_commands(
        Path::new("/opt/homebrew/bin/terminal-notifier"),
        Path::new("/Users/test/bin/zzz"),
        &target,
        Some(Path::new(
            "/Applications/iTerm.app/Contents/Resources/iTerm Icon.icns",
        )),
        "cargo",
    );

    assert_eq!(commands.success.args[0..6], [
        "-title", "zzz", "-subtitle", "Succeeded", "-message", "cargo",
    ]);
    assert!(commands.success.args.windows(2).any(|pair| {
        pair
            == [
                "-appIcon",
                "/Applications/iTerm.app/Contents/Resources/iTerm Icon.icns",
            ]
    }));
    assert!(commands.success.args.windows(2).any(|pair| {
        pair[0] == "-execute"
            && pair[1].contains("--__zzz-focus-terminal")
            && pair[1].contains("F8176E01-630A-4505-9B2B-0DE870BF4706")
    }));
    assert_eq!(commands.failure.args[3], "Failed");
}
```

- [ ] **Step 2: Run notifier tests and verify RED**

Run:

```bash
cargo test -p cli-core macos_notification::tests::resolves_path -- --nocapture
cargo test -p cli-core macos_notification::tests::clickable_commands -- --nocapture
```

Expected: compilation fails because notifier resolution, completion command
types, and builders do not exist.

- [ ] **Step 3: Add completion command types to the parent module**

In `crates/cli-core/src/command_log/mod.rs`, add:

```rust
#[cfg(target_os = "macos")]
#[derive(Debug, Eq, PartialEq)]
struct CompletionCommand {
    program: PathBuf,
    args: Vec<String>,
}

#[cfg(target_os = "macos")]
#[derive(Debug, Eq, PartialEq)]
struct CompletionCommands {
    success: CompletionCommand,
    failure: CompletionCommand,
}

#[cfg(target_os = "macos")]
fn external_command_line(command: &CompletionCommand) -> String {
    std::iter::once(shell_quote(&command.program.to_string_lossy()))
        .chain(command.args.iter().map(|argument| shell_quote(argument)))
        .collect::<Vec<_>>()
        .join(" ")
}
```

- [ ] **Step 4: Implement notifier, icon, and fallback builders**

In `macos_notification.rs`, import the parent types:

```rust
use super::{
    shell_quote, CompletionCommand, CompletionCommands, TERMINAL_FOCUS_FLAG,
};
use std::ffi::OsStr;
```

Implement notifier lookup:

```rust
fn find_terminal_notifier_with(
    path: Option<&OsStr>,
    is_file: impl Fn(&Path) -> bool,
) -> Option<PathBuf> {
    let path_candidates = path
        .into_iter()
        .flat_map(env::split_paths)
        .map(|directory| directory.join("terminal-notifier"));
    let fallback_candidates = [
        PathBuf::from("/opt/homebrew/bin/terminal-notifier"),
        PathBuf::from("/usr/local/bin/terminal-notifier"),
    ];

    path_candidates
        .chain(fallback_candidates)
        .find(|candidate| is_file(candidate))
}

fn find_terminal_notifier() -> Option<PathBuf> {
    find_terminal_notifier_with(env::var_os("PATH").as_deref(), Path::is_file)
}
```

Resolve standard icons:

```rust
fn terminal_icon(target: &TerminalTarget) -> Option<PathBuf> {
    let mut candidates = match target.kind {
        TerminalKind::ITerm2 => vec![
            PathBuf::from(
                "/Applications/iTerm.app/Contents/Resources/iTerm2 App Icon for Release.icns",
            ),
            PathBuf::from(
                "/Applications/iTerm.app/Contents/Resources/AppIcon.png",
            ),
        ],
        TerminalKind::Terminal => vec![PathBuf::from(
            "/System/Applications/Utilities/Terminal.app/Contents/Resources/Terminal.icns",
        )],
    };

    if let Some(home) = env::var_os("HOME") {
        if target.kind == TerminalKind::ITerm2 {
            candidates.insert(
                0,
                PathBuf::from(home).join(
                    "Applications/iTerm.app/Contents/Resources/iTerm2 App Icon for Release.icns",
                ),
            );
        }
    }

    candidates.into_iter().find(|candidate| candidate.is_file())
}
```

Build click and notification commands:

```rust
fn focus_click_command(executable: &Path, target: &TerminalTarget) -> String {
    [
        executable.to_string_lossy().into_owned(),
        TERMINAL_FOCUS_FLAG.to_owned(),
        target.kind.as_str().to_owned(),
        target.locator.clone(),
    ]
    .iter()
    .map(|value| shell_quote(value))
    .collect::<Vec<_>>()
    .join(" ")
}

fn terminal_notifier_command(
    notifier: &Path,
    executable: &Path,
    target: &TerminalTarget,
    icon: Option<&Path>,
    command_name: &str,
    outcome: &str,
) -> CompletionCommand {
    let mut args = vec![
        "-title".into(),
        "zzz".into(),
        "-subtitle".into(),
        outcome.into(),
        "-message".into(),
        command_name.into(),
    ];
    if let Some(icon) = icon {
        args.push("-appIcon".into());
        args.push(icon.to_string_lossy().into_owned());
    }
    args.push("-execute".into());
    args.push(focus_click_command(executable, target));

    CompletionCommand {
        program: notifier.to_owned(),
        args,
    }
}

fn terminal_notifier_commands(
    notifier: &Path,
    executable: &Path,
    target: &TerminalTarget,
    icon: Option<&Path>,
    command_name: &str,
) -> CompletionCommands {
    CompletionCommands {
        success: terminal_notifier_command(
            notifier,
            executable,
            target,
            icon,
            command_name,
            "Succeeded",
        ),
        failure: terminal_notifier_command(
            notifier,
            executable,
            target,
            icon,
            command_name,
            "Failed",
        ),
    }
}
```

Retain the existing generic notification:

```rust
fn osascript_command(command_name: &str, outcome: &str) -> CompletionCommand {
    CompletionCommand {
        program: PathBuf::from("/usr/bin/osascript"),
        args: vec![
            "-e".into(),
            format!(
                "display notification \"{command_name}\" with title \"zzz\" subtitle \"{outcome}\""
            ),
        ],
    }
}

pub(super) fn completion_commands(command_name: &str) -> CompletionCommands {
    let clickable = detect_terminal_target(&current_terminal_environment())
        .zip(find_terminal_notifier())
        .zip(env::current_exe().ok());

    if let Some(((target, notifier), executable)) = clickable {
        let icon = terminal_icon(&target);
        terminal_notifier_commands(
            &notifier,
            &executable,
            &target,
            icon.as_deref(),
            command_name,
        )
    } else {
        CompletionCommands {
            success: osascript_command(command_name, "Succeeded"),
            failure: osascript_command(command_name, "Failed"),
        }
    }
}
```

- [ ] **Step 5: Replace the hard-coded osascript wrapper**

Change `shell_script_with_notification_program` in
`crates/cli-core/src/command_log/mod.rs` to:

```rust
#[cfg(target_os = "macos")]
fn shell_script_with_completion_commands(
    command: &str,
    args: &[String],
    log_path: &Path,
    commands: &CompletionCommands,
) -> String {
    format!(
        "set +e\n{}\nzzz_command_status=$?\n\
         if [ \"$zzz_command_status\" -eq 0 ]; then\n  {} >/dev/null 2>&1\n\
         else\n  {} >/dev/null 2>&1\n\
         fi\nexit \"$zzz_command_status\"",
        shell_script(command, args, log_path),
        external_command_line(&commands.success),
        external_command_line(&commands.failure),
    )
}
```

In macOS `spawn_command`, build the script with:

```rust
let script = match completion_notification {
    CompletionNotification::System => {
        let commands =
            macos_notification::completion_commands(&command_name_from_command(command));
        shell_script_with_completion_commands(command, args, log_path, &commands)
    }
    CompletionNotification::Disabled => shell_script(command, args, log_path),
};
```

Keep the non-macOS branch unchanged.

- [ ] **Step 6: Adapt the existing wrapper tests**

Replace the fake osascript-specific fixture with `CompletionCommands` whose
success argument is `Succeeded` and failure argument is `Failed`. Make the fake
program write all arguments to `ZZZ_TEST_NOTIFICATION_PATH`, and retain these
assertions:

```rust
assert_eq!(
    fs::read_to_string(&fixture.notification_path).unwrap(),
    "Succeeded\n"
);
assert_eq!(status.code(), Some(7));
assert_eq!(
    fs::read_to_string(&fixture.notification_path).unwrap(),
    "Failed\n"
);
```

- [ ] **Step 7: Run notifier and wrapper tests and verify GREEN**

Run:

```bash
cargo test -p cli-core macos_notification::tests -- --nocapture
cargo test -p cli-core command_log::tests::completion_wrapper -- --nocapture
```

Expected: all new module tests and both completion wrapper tests pass.

### Task 3: Restore keyboard focus through the zzz click handler

**Files:**
- Modify: `crates/cli-core/src/command_log/macos_notification.rs`
- Modify: `crates/zzz/src/main.rs`
- Create: `crates/zzz/tests/focus.rs`

- [ ] **Step 1: Write failing focus-script unit tests**

Add:

```rust
#[test]
fn focus_scripts_pass_locators_as_arguments() {
    assert!(ITERM_FOCUS_SCRIPT.contains("item 1 of argv"));
    assert!(!ITERM_FOCUS_SCRIPT.contains("F8176E01"));
    assert!(TERMINAL_FOCUS_SCRIPT.contains("item 1 of argv"));
    assert!(!TERMINAL_FOCUS_SCRIPT.contains("/dev/ttys"));
}
```

- [ ] **Step 2: Write failing hidden-mode integration tests**

Create `crates/zzz/tests/focus.rs`:

```rust
#[cfg(target_os = "macos")]
mod macos {
    use std::process::Command;

    #[test]
    fn focus_mode_rejects_unknown_terminal_kind() {
        let output = Command::new(env!("CARGO_BIN_EXE_zzz"))
            .args([
                cli_core::command_log::TERMINAL_FOCUS_FLAG,
                "unknown",
                "/dev/ttys001",
            ])
            .output()
            .unwrap();

        assert!(!output.status.success());
        assert!(String::from_utf8_lossy(&output.stderr)
            .contains("Unsupported terminal kind"));
    }

    #[test]
    fn focus_mode_rejects_malformed_iterm_locator() {
        let output = Command::new(env!("CARGO_BIN_EXE_zzz"))
            .args([
                cli_core::command_log::TERMINAL_FOCUS_FLAG,
                "iterm2",
                "not-a-uuid",
            ])
            .output()
            .unwrap();

        assert!(!output.status.success());
        assert!(String::from_utf8_lossy(&output.stderr)
            .contains("Invalid iTerm2 session locator"));
    }
}
```

- [ ] **Step 3: Run focus tests and verify RED**

Run:

```bash
cargo test -p cli-core macos_notification::tests::focus_scripts -- --nocapture
cargo test -p zzz --test focus -- --nocapture
```

Expected: the unit test fails because focus script constants do not exist, and
the integration tests fail because `zzz` treats the internal flag as a command.

- [ ] **Step 4: Implement constant AppleScript adapters**

Add constant scripts:

```rust
const ITERM_FOCUS_SCRIPT: &str = r#"
on run argv
    set targetId to item 1 of argv
    tell application id "com.googlecode.iterm2"
        repeat with terminalWindow in windows
            repeat with terminalTab in tabs of terminalWindow
                repeat with terminalSession in sessions of terminalTab
                    if unique ID of terminalSession is targetId then
                        select terminalSession
                        select terminalTab
                        select terminalWindow
                        activate
                        return
                    end if
                end repeat
            end repeat
        end repeat
        activate
    end tell
end run
"#;

const TERMINAL_FOCUS_SCRIPT: &str = r#"
on run argv
    set targetTty to item 1 of argv
    tell application id "com.apple.Terminal"
        repeat with terminalWindow in windows
            repeat with terminalTab in tabs of terminalWindow
                if tty of terminalTab is targetTty then
                    set selected tab of terminalWindow to terminalTab
                    set frontmost of terminalWindow to true
                    activate
                    return
                end if
            end repeat
        end repeat
        activate
    end tell
end run
"#;
```

Implement the validated handler:

```rust
pub(super) fn focus_terminal(kind: &str, locator: &str) -> Result<()> {
    let target = TerminalTarget::parse(kind, locator)?;
    let script = match target.kind {
        TerminalKind::ITerm2 => ITERM_FOCUS_SCRIPT,
        TerminalKind::Terminal => TERMINAL_FOCUS_SCRIPT,
    };

    let status = Command::new("/usr/bin/osascript")
        .arg("-e")
        .arg(script)
        .arg("--")
        .arg(&target.locator)
        .status()
        .context("Failed to start osascript terminal focus handler")?;

    if status.success() {
        Ok(())
    } else {
        bail!("Terminal focus handler exited with status {status}")
    }
}
```

- [ ] **Step 5: Dispatch the internal mode before normal commands**

Update `crates/zzz/src/main.rs`:

```rust
use anyhow::{bail, Result};

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let Some(command) = args.next() else {
        anyhow::bail!("Usage: zzz <command> [args...]");
    };

    if command == cli_core::command_log::TERMINAL_FOCUS_FLAG {
        let Some(kind) = args.next() else {
            bail!("Missing terminal kind for internal focus mode");
        };
        let Some(locator) = args.next() else {
            bail!("Missing terminal locator for internal focus mode");
        };
        if args.next().is_some() {
            bail!("Unexpected arguments for internal focus mode");
        }
        return cli_core::command_log::focus_terminal(&kind, &locator);
    }

    if command == "--version" || command == "-V" {
        println!("zzz {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    cli_core::command_log::run_with_system_notification(&command, args.collect())
}
```

- [ ] **Step 6: Run focus and zzz tests and verify GREEN**

Run:

```bash
cargo test -p cli-core macos_notification::tests::focus_scripts -- --nocapture
cargo test -p zzz --all-targets
```

Expected: all focus validation, `zzz` command logging, alias, and version tests
pass.

### Task 4: Verify and publish the code checkpoint

**Files:**
- Create: `crates/cli-core/src/command_log/macos_notification.rs`
- Modify: `crates/cli-core/src/command_log/mod.rs`
- Modify: `crates/zzz/src/main.rs`
- Create: `crates/zzz/tests/focus.rs`

- [ ] **Step 1: Format and inspect only the implementation paths**

Run:

```bash
cargo fmt --all -- --check
git diff --check -- \
  crates/cli-core/src/command_log/mod.rs \
  crates/cli-core/src/command_log/macos_notification.rs \
  crates/zzz/src/main.rs \
  crates/zzz/tests/focus.rs
git diff -- \
  crates/cli-core/src/command_log/mod.rs \
  crates/cli-core/src/command_log/macos_notification.rs \
  crates/zzz/src/main.rs \
  crates/zzz/tests/focus.rs
```

Expected: formatting and whitespace checks exit zero; the diff contains no JSON
tool changes.

- [ ] **Step 2: Run focused gates**

Run:

```bash
cargo test -p cli-core -p zzz --all-targets
cargo clippy -p cli-core -p zzz --all-targets -- -D warnings
```

Expected: all tests pass and Clippy emits no warnings.

- [ ] **Step 3: Reinstall zzz**

Run:

```bash
cargo install --path crates/zzz --force
zzz --version
```

Expected: installation succeeds and version output equals the workspace package
version.

- [ ] **Step 4: Verify actual iTerm2 success and failure clicks**

From the current iTerm2 session, run:

```bash
zzz sh -c 'sleep 3; printf success'
zzz sh -c 'sleep 3; exit 7'
```

Move keyboard focus to another iTerm2 session before each notification arrives.
Click the notification and assert with iTerm2 AppleScript that the current
session UUID equals the UUID captured from `ITERM_SESSION_ID`. Type a harmless
sentinel such as `printf zzz-focus-ok` without submitting it, then clear it, to
prove the keyboard cursor is in the restored session. Visually confirm the
iTerm2 icon appears on both notifications.

- [ ] **Step 5: Verify actual Terminal.app success and failure clicks**

Open Terminal.app, run the same two commands, and record `tty`. Move to another
Terminal tab or application before completion. Click each notification and
assert that `tty of selected tab of front window` equals the recorded TTY. Type
and clear the same unsent sentinel to prove keyboard input focus. Visually
confirm the Terminal.app icon appears on both notifications.

- [ ] **Step 6: Commit and push the green code checkpoint**

Stage only these explicit paths:

```bash
git add \
  crates/cli-core/src/command_log/mod.rs \
  crates/cli-core/src/command_log/macos_notification.rs \
  crates/zzz/src/main.rs \
  crates/zzz/tests/focus.rs
git diff --cached --check
git commit -m "feat(zzz): focus launching terminal from notifications"
git push
git rev-list --left-right --count HEAD...@{u}
```

Expected: push succeeds and parity is `0 0`. Existing JSON tool changes remain
unstaged.

### Task 5: Document, fully verify, and publish

**Files:**
- Modify: `README.md`
- Modify: `README.ko.md`

- [ ] **Step 1: Update the English zzz documentation**

Replace the macOS notification sentence with:

```markdown
On macOS, `zzz` sends a system notification when the background command
finishes. With [`terminal-notifier`](https://github.com/julienXX/terminal-notifier)
installed, success and failure notifications use the launching terminal's icon;
clicking one restores keyboard focus to the exact launching iTerm2 session or
Terminal.app tab. If the session has closed, `zzz` activates the terminal
without creating a new window. Without `terminal-notifier`, `zzz` falls back to
a generic, non-clickable completion notification.
```

Add the installation command beside the `zzz` install instructions:

```bash
brew install terminal-notifier
```

- [ ] **Step 2: Update the Korean zzz documentation**

Replace the equivalent sentence with:

```markdown
macOS에서는 명령이 끝나면 성공 또는 실패 시스템 알림을 전송합니다.
[`terminal-notifier`](https://github.com/julienXX/terminal-notifier)가 설치되어
있으면 알림에 실행을 시작한 터미널 아이콘을 표시하며, 알림을 클릭하면 정확한
iTerm2 세션 또는 Terminal.app 탭으로 키보드 포커스를 복원합니다. 해당 세션이
이미 닫혔다면 새 창을 만들지 않고 터미널 앱만 활성화합니다.
`terminal-notifier`가 없으면 클릭할 수 없는 일반 완료 알림으로 폴백합니다.
```

Add the same `brew install terminal-notifier` command beside the standalone
`zzz` install instructions.

- [ ] **Step 3: Stage only the README notification hunks**

Both README files already contain unrelated user-owned JSON documentation
changes. Use interactive, path-specific staging:

```bash
git add -p README.md README.ko.md
```

Answer `y` only for the `zzz` install and notification hunks, and `n` for every
JSON command hunk. Then inspect:

```bash
git diff --cached -- README.md README.ko.md
git diff -- README.md README.ko.md
```

Expected: the cached diff contains only clickable-notification documentation;
the working-tree diff retains the unrelated JSON documentation.

- [ ] **Step 4: Run the repository release gates**

Run:

```bash
cargo test --workspace --all-targets
cargo clippy -p cli-core -p zzz --all-targets -- -D warnings
cargo fmt --all -- --check
git diff --cached --check
```

Expected: every command exits zero. Workspace tests cover all crates; strict
Clippy remains scoped because unrelated crates have known pre-existing lint
debt.

- [ ] **Step 5: Commit and push the documentation checkpoint**

Run:

```bash
git commit -m "docs(zzz): explain clickable terminal notifications"
git push
git rev-list --left-right --count HEAD...@{u}
```

Expected: push succeeds and parity is `0 0`.

- [ ] **Step 6: Audit the final request and repository state**

Run:

```bash
git log --oneline ab33862..HEAD
git status --short --branch
git rev-list --left-right --count HEAD...@{u}
```

Verify each requirement against evidence:

- both successful and failed notifications were clicked;
- both clicks restored the exact launching window/tab/session and keyboard
  cursor;
- both notifications displayed the correct launching terminal icon;
- iTerm2 and Terminal.app runtime checks passed;
- the generic fallback and original command status are test-covered;
- all relevant tests, workspace tests, formatting, and scoped strict Clippy
  passed;
- only the known JSON changes remain in the worktree; and
- local/upstream parity is `0 0`.
