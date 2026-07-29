use anyhow::{bail, Context, Result};
use std::collections::HashSet;
use std::env;
use std::ffi::{OsStr, OsString};
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use super::{shell_quote, TERMINAL_FOCUS_FLAG};

const ITERM_BUNDLE_ID: &str = "com.googlecode.iterm2";
const GHOSTTY_BUNDLE_ID: &str = "com.mitchellh.ghostty";
const TERMINAL_BUNDLE_ID: &str = "com.apple.Terminal";
const OSASCRIPT: &str = "/usr/bin/osascript";

const ITERM_FOCUS_SCRIPT: &str = r#"
on run argv
    set targetId to item 1 of argv
    if application id "com.googlecode.iterm2" is not running then return
    tell application id "com.googlecode.iterm2"
        repeat with terminalWindow in windows
            repeat with terminalTab in tabs of terminalWindow
                repeat with terminalSession in sessions of terminalTab
                    if unique ID of terminalSession is targetId then
                        select terminalWindow
                        select terminalTab
                        tell terminalSession to select
                        activate
                        return
                    end if
                end repeat
            end repeat
        end repeat
    end tell
end run
"#;

const TERMINAL_FOCUS_SCRIPT: &str = r#"
on run argv
    set targetTty to item 1 of argv
    if application id "com.apple.Terminal" is not running then return
    tell application id "com.apple.Terminal"
        activate
        repeat with terminalWindow in windows
            repeat with terminalTab in tabs of terminalWindow
                if tty of terminalTab is targetTty then
                    set selected tab of terminalWindow to terminalTab
                    set frontmost of terminalWindow to true
                    return
                end if
            end repeat
        end repeat
    end tell
end run
"#;

#[derive(Clone, Debug, Eq, PartialEq)]
struct TerminalEnvironment {
    term_program: Option<String>,
    bundle_identifier: Option<String>,
    iterm_session_id: Option<String>,
}

impl TerminalEnvironment {
    fn current() -> Self {
        Self {
            term_program: env::var("TERM_PROGRAM").ok(),
            bundle_identifier: env::var("__CFBundleIdentifier").ok(),
            iterm_session_id: env::var("ITERM_SESSION_ID").ok(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum DetectedTerminal {
    ITerm2 { session_uuid: String },
    Ghostty,
    TerminalApp,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum FocusTarget {
    ITerm2 { session_uuid: String },
    TerminalApp { tty: PathBuf },
}

impl FocusTarget {
    fn kind(&self) -> &'static str {
        match self {
            Self::ITerm2 { .. } => "iterm2",
            Self::TerminalApp { .. } => "terminal",
        }
    }

    fn locator(&self) -> OsString {
        match self {
            Self::ITerm2 { session_uuid } => OsString::from(session_uuid),
            Self::TerminalApp { tty } => tty.as_os_str().to_owned(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Outcome {
    Succeeded,
    Failed,
}

impl Outcome {
    fn label(self) -> &'static str {
        match self {
            Self::Succeeded => "Succeeded",
            Self::Failed => "Failed",
        }
    }
}

pub(super) struct NotificationCommands {
    pub(super) succeeded: String,
    pub(super) failed: String,
}

pub(super) fn completion_commands(command_name: &str) -> NotificationCommands {
    let fallback = || NotificationCommands {
        succeeded: system_notification_command(Outcome::Succeeded, command_name),
        failed: system_notification_command(Outcome::Failed, command_name),
    };

    let target = match detect_terminal(&TerminalEnvironment::current()) {
        Some(DetectedTerminal::ITerm2 { session_uuid }) => FocusTarget::ITerm2 { session_uuid },
        Some(DetectedTerminal::Ghostty) => {
            let Some(tty) = current_tty() else {
                return fallback();
            };
            return NotificationCommands {
                succeeded: terminal_osc_notification_command(
                    &tty,
                    Outcome::Succeeded,
                    command_name,
                ),
                failed: terminal_osc_notification_command(&tty, Outcome::Failed, command_name),
            };
        }
        Some(DetectedTerminal::TerminalApp) => {
            let Some(tty) = current_tty() else {
                return fallback();
            };
            FocusTarget::TerminalApp { tty }
        }
        None => return fallback(),
    };

    let Some(notifier) = resolve_notifier() else {
        return fallback();
    };
    let Ok(executable) = env::current_exe() else {
        return fallback();
    };
    let icon = terminal_icon(&target);

    NotificationCommands {
        succeeded: terminal_notification_command(
            &notifier,
            icon.as_deref(),
            &executable,
            &target,
            Outcome::Succeeded,
            command_name,
        ),
        failed: terminal_notification_command(
            &notifier,
            icon.as_deref(),
            &executable,
            &target,
            Outcome::Failed,
            command_name,
        ),
    }
}

pub(super) fn focus_terminal(kind: &str, locator: &str) -> Result<()> {
    let target = parse_focus_target(kind, locator)?;
    let script = match target {
        FocusTarget::ITerm2 { .. } => ITERM_FOCUS_SCRIPT,
        FocusTarget::TerminalApp { .. } => TERMINAL_FOCUS_SCRIPT,
    };
    let status = Command::new(OSASCRIPT)
        .arg("-e")
        .arg(script)
        .arg("--")
        .arg(target.locator())
        .status()
        .context("Failed to start terminal focus handler")?;

    if status.success() {
        Ok(())
    } else {
        bail!("Terminal focus handler exited with status {status}")
    }
}

fn detect_terminal(environment: &TerminalEnvironment) -> Option<DetectedTerminal> {
    let is_iterm = environment.term_program.as_deref() == Some("iTerm.app")
        || environment.bundle_identifier.as_deref() == Some(ITERM_BUNDLE_ID);
    if is_iterm {
        let session_id = environment.iterm_session_id.as_deref()?;
        let session_uuid = session_id.rsplit(':').next()?;
        return valid_uuid(session_uuid).then(|| DetectedTerminal::ITerm2 {
            session_uuid: session_uuid.to_string(),
        });
    }

    let is_ghostty = environment.term_program.as_deref() == Some("ghostty")
        || environment.bundle_identifier.as_deref() == Some(GHOSTTY_BUNDLE_ID);
    if is_ghostty {
        return Some(DetectedTerminal::Ghostty);
    }

    let is_terminal = environment.term_program.as_deref() == Some("Apple_Terminal")
        || environment.bundle_identifier.as_deref() == Some(TERMINAL_BUNDLE_ID);
    is_terminal.then_some(DetectedTerminal::TerminalApp)
}

fn valid_uuid(value: &str) -> bool {
    value.len() == 36
        && value.bytes().enumerate().all(|(index, byte)| {
            if matches!(index, 8 | 13 | 18 | 23) {
                byte == b'-'
            } else {
                byte.is_ascii_hexdigit()
            }
        })
}

fn valid_tty(path: &Path) -> bool {
    let Some(value) = path.to_str() else {
        return false;
    };
    let Some(suffix) = value.strip_prefix("/dev/tty") else {
        return false;
    };

    !suffix.is_empty()
        && suffix.len() <= 64
        && suffix
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-'))
}

fn parse_tty_output(success: bool, stdout: &[u8]) -> Option<PathBuf> {
    if !success {
        return None;
    }

    let value = std::str::from_utf8(stdout).ok()?.trim();
    let path = PathBuf::from(value);
    valid_tty(&path).then_some(path)
}

fn current_tty() -> Option<PathBuf> {
    let output = Command::new("/usr/bin/tty")
        .stdin(Stdio::inherit())
        .output()
        .ok()?;
    parse_tty_output(output.status.success(), &output.stdout)
}

fn notifier_candidates(path: Option<&OsStr>) -> Vec<PathBuf> {
    let mut candidates: Vec<PathBuf> = path
        .map(env::split_paths)
        .into_iter()
        .flatten()
        .map(|directory| directory.join("terminal-notifier"))
        .collect();
    candidates.extend([
        PathBuf::from("/opt/homebrew/bin/terminal-notifier"),
        PathBuf::from("/usr/local/bin/terminal-notifier"),
    ]);

    let mut seen = HashSet::new();
    candidates.retain(|candidate| seen.insert(candidate.clone()));
    candidates
}

fn resolve_notifier() -> Option<PathBuf> {
    resolve_notifier_with(env::var_os("PATH").as_deref(), is_executable)
}

fn resolve_notifier_with(
    path: Option<&OsStr>,
    is_executable: impl Fn(&Path) -> bool,
) -> Option<PathBuf> {
    notifier_candidates(path)
        .into_iter()
        .find(|candidate| is_executable(candidate))
}

fn is_executable(path: &Path) -> bool {
    fs::metadata(path)
        .map(|metadata| metadata.is_file() && metadata.permissions().mode() & 0o111 != 0)
        .unwrap_or(false)
}

fn terminal_icon(target: &FocusTarget) -> Option<PathBuf> {
    terminal_icon_candidates(target)
        .into_iter()
        .find(|candidate| candidate.is_file())
}

fn terminal_icon_candidates(target: &FocusTarget) -> Vec<PathBuf> {
    match target {
        FocusTarget::ITerm2 { .. } => vec![
            PathBuf::from(
                "/Applications/iTerm.app/Contents/Resources/iTerm2 App Icon for Release.icns",
            ),
            PathBuf::from("/Applications/iTerm.app/Contents/Resources/AppIcon.png"),
        ],
        FocusTarget::TerminalApp { .. } => vec![
            PathBuf::from(
                "/System/Applications/Utilities/Terminal.app/Contents/Resources/Terminal.icns",
            ),
            PathBuf::from("/Applications/Utilities/Terminal.app/Contents/Resources/Terminal.icns"),
        ],
    }
}

fn terminal_notification_command(
    notifier: &Path,
    icon: Option<&Path>,
    executable: &Path,
    target: &FocusTarget,
    outcome: Outcome,
    command_name: &str,
) -> String {
    let click_command = shell_command(
        executable,
        &[
            OsString::from(TERMINAL_FOCUS_FLAG),
            OsString::from(target.kind()),
            target.locator(),
        ],
    );
    let mut arguments = vec![
        OsString::from("-title"),
        OsString::from("zzz"),
        OsString::from("-subtitle"),
        OsString::from(outcome.label()),
        OsString::from("-message"),
        OsString::from(command_name),
    ];

    if let Some(icon) = icon {
        arguments.push(OsString::from("-appIcon"));
        arguments.push(icon.as_os_str().to_owned());
    }

    arguments.push(OsString::from("-execute"));
    arguments.push(OsString::from(click_command));

    format!("{} >/dev/null 2>&1", shell_command(notifier, &arguments))
}

fn terminal_osc_notification_command(tty: &Path, outcome: Outcome, command_name: &str) -> String {
    let command_name: String = command_name
        .chars()
        .map(|character| {
            if character.is_control() {
                ' '
            } else {
                character
            }
        })
        .collect();
    let sequence = format!("\u{1b}]9;zzz {}: {command_name}\u{1b}\\", outcome.label());
    format!(
        "{} > {} 2>/dev/null",
        shell_command(
            Path::new("/usr/bin/printf"),
            &[OsString::from("%s"), OsString::from(sequence)]
        ),
        shell_quote(&tty.to_string_lossy()),
    )
}

fn system_notification_command(outcome: Outcome, command_name: &str) -> String {
    let source = format!(
        "display notification \"{command_name}\" with title \"zzz\" subtitle \"{}\"",
        outcome.label()
    );
    format!(
        "{} >/dev/null 2>&1",
        shell_command(
            Path::new(OSASCRIPT),
            &[OsString::from("-e"), OsString::from(source)]
        )
    )
}

fn shell_command(program: &Path, arguments: &[OsString]) -> String {
    std::iter::once(shell_quote(&program.to_string_lossy()))
        .chain(
            arguments
                .iter()
                .map(|argument| shell_quote(&argument.to_string_lossy())),
        )
        .collect::<Vec<_>>()
        .join(" ")
}

fn parse_focus_target(kind: &str, locator: &str) -> Result<FocusTarget> {
    match kind {
        "iterm2" if valid_uuid(locator) => Ok(FocusTarget::ITerm2 {
            session_uuid: locator.to_string(),
        }),
        "terminal" if valid_tty(Path::new(locator)) => Ok(FocusTarget::TerminalApp {
            tty: PathBuf::from(locator),
        }),
        "iterm2" => bail!("Invalid iTerm2 session locator"),
        "terminal" => bail!("Invalid Terminal.app TTY locator"),
        _ => bail!("Unsupported terminal focus kind '{kind}'"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::OsStr;
    use std::path::{Path, PathBuf};

    fn iterm_environment(session_id: &str) -> TerminalEnvironment {
        TerminalEnvironment {
            term_program: Some("iTerm.app".into()),
            bundle_identifier: Some("com.googlecode.iterm2".into()),
            iterm_session_id: Some(session_id.into()),
        }
    }

    fn iterm_target() -> FocusTarget {
        FocusTarget::ITerm2 {
            session_uuid: "F8176E01-630A-4505-9B2B-0DE870BF4706".into(),
        }
    }

    #[test]
    fn detects_iterm2_and_extracts_the_session_uuid() {
        assert_eq!(
            detect_terminal(&iterm_environment(
                "w0t2p2:F8176E01-630A-4505-9B2B-0DE870BF4706"
            )),
            Some(DetectedTerminal::ITerm2 {
                session_uuid: "F8176E01-630A-4505-9B2B-0DE870BF4706".into(),
            })
        );
    }

    #[test]
    fn rejects_malformed_iterm2_session_ids() {
        assert_eq!(
            detect_terminal(&iterm_environment("w0t2p2:not-a-uuid")),
            None
        );
    }

    #[test]
    fn detects_terminal_app() {
        let environment = TerminalEnvironment {
            term_program: Some("Apple_Terminal".into()),
            bundle_identifier: Some("com.apple.Terminal".into()),
            iterm_session_id: None,
        };

        assert_eq!(
            detect_terminal(&environment),
            Some(DetectedTerminal::TerminalApp)
        );
    }

    #[test]
    fn detects_ghostty_by_term_program_or_bundle_identifier() {
        let by_program = TerminalEnvironment {
            term_program: Some("ghostty".into()),
            bundle_identifier: None,
            iterm_session_id: None,
        };
        let by_bundle = TerminalEnvironment {
            term_program: None,
            bundle_identifier: Some("com.mitchellh.ghostty".into()),
            iterm_session_id: None,
        };

        assert_eq!(
            detect_terminal(&by_program),
            Some(DetectedTerminal::Ghostty)
        );
        assert_eq!(detect_terminal(&by_bundle), Some(DetectedTerminal::Ghostty));
    }

    #[test]
    fn validates_terminal_tty_locators() {
        assert!(valid_tty(Path::new("/dev/ttys014")));
        assert!(valid_tty(Path::new("/dev/tty.usbserial-01")));
        assert!(!valid_tty(Path::new("/tmp/ttys014")));
        assert!(!valid_tty(Path::new("/dev/tty/../../tmp/x")));
        assert!(!valid_tty(Path::new("/dev/tty bad")));
        assert!(!valid_tty(Path::new("/dev/tty\nbad")));
    }

    #[test]
    fn parses_only_successful_valid_tty_output() {
        assert_eq!(
            parse_tty_output(true, b"/dev/ttys014\n"),
            Some(PathBuf::from("/dev/ttys014"))
        );
        assert_eq!(parse_tty_output(false, b"/dev/ttys014\n"), None);
        assert_eq!(parse_tty_output(true, b""), None);
        assert_eq!(parse_tty_output(true, b"/tmp/not-a-tty\n"), None);
    }

    #[test]
    fn current_tty_reads_the_callers_terminal() {
        const CHILD_ENV: &str = "ZZZ_CURRENT_TTY_TEST_CHILD";
        if env::var_os(CHILD_ENV).is_some() {
            let tty = current_tty()
                .map(|path| path.display().to_string())
                .unwrap_or_else(|| "NONE".into());
            println!("ZZZ_CURRENT_TTY={tty}");
            return;
        }

        let output = Command::new("/usr/bin/script")
            .args(["-q", "/dev/null"])
            .arg(env::current_exe().unwrap())
            .args([
                "--exact",
                "command_log::macos_notification::tests::current_tty_reads_the_callers_terminal",
                "--nocapture",
            ])
            .env(CHILD_ENV, "1")
            .output()
            .unwrap();
        let transcript = String::from_utf8_lossy(&output.stdout);

        assert!(
            transcript.contains("ZZZ_CURRENT_TTY=/dev/tty"),
            "child did not observe its pseudo-terminal:\n{transcript}"
        );
    }

    #[test]
    fn notifier_candidates_prefer_path_then_homebrew_locations() {
        assert_eq!(
            notifier_candidates(Some(OsStr::new("/custom/bin:/next/bin"))),
            vec![
                PathBuf::from("/custom/bin/terminal-notifier"),
                PathBuf::from("/next/bin/terminal-notifier"),
                PathBuf::from("/opt/homebrew/bin/terminal-notifier"),
                PathBuf::from("/usr/local/bin/terminal-notifier"),
            ]
        );
    }

    #[test]
    fn notifier_resolution_selects_the_first_executable_candidate() {
        let resolved =
            resolve_notifier_with(Some(OsStr::new("/custom/bin:/next/bin")), |candidate| {
                candidate == Path::new("/next/bin/terminal-notifier")
                    || candidate == Path::new("/opt/homebrew/bin/terminal-notifier")
            });

        assert_eq!(resolved, Some(PathBuf::from("/next/bin/terminal-notifier")));
    }

    #[test]
    fn ghostty_notification_uses_its_native_originating_surface_channel() {
        let command =
            terminal_osc_notification_command(Path::new("/dev/ttys014"), Outcome::Failed, "cargo");

        assert!(command.contains("\u{1b}]9;zzz Failed: cargo\u{1b}\\"));
        assert!(command.contains("'/dev/ttys014'"));
        assert!(!command.contains("terminal-notifier"));
        assert!(!command.contains("'-execute'"));
    }

    #[test]
    fn ghostty_notification_strips_control_characters_from_the_message() {
        let command = terminal_osc_notification_command(
            Path::new("/dev/ttys014"),
            Outcome::Succeeded,
            "cargo\u{1b}]9;forged\nmessage",
        );

        assert!(!command.contains("forged\nmessage"));
        assert_eq!(command.matches('\u{1b}').count(), 2);
    }

    #[test]
    fn notifier_uses_iterm_icon_and_exact_session_focus_target() {
        let command = terminal_notification_command(
            Path::new("/opt/homebrew/bin/terminal-notifier"),
            Some(Path::new(
                "/Applications/iTerm.app/Contents/Resources/iTerm2 App Icon for Release.icns",
            )),
            Path::new("/Applications/CLI Tools/zzz"),
            &iterm_target(),
            Outcome::Failed,
            "cargo",
        );

        assert!(command.contains("'-title' 'zzz'"));
        assert!(command.contains("'-subtitle' 'Failed'"));
        assert!(command.contains("'-message' 'cargo'"));
        assert!(command.contains("'-appIcon'"));
        assert!(command.contains("iTerm2 App Icon for Release.icns"));
        assert!(command.contains("'-execute'"));
        assert!(command.contains("--__zzz-focus-terminal"));
        assert!(command.contains("'iterm2'"));
        assert!(command.contains("F8176E01-630A-4505-9B2B-0DE870BF4706"));
        assert!(command.contains("Applications/CLI Tools/zzz"));
    }

    #[test]
    fn notifier_uses_terminal_icon_and_exact_tty_focus_target() {
        let command = terminal_notification_command(
            Path::new("/opt/homebrew/bin/terminal-notifier"),
            Some(Path::new(
                "/System/Applications/Utilities/Terminal.app/Contents/Resources/Terminal.icns",
            )),
            Path::new("/usr/local/bin/zzz"),
            &FocusTarget::TerminalApp {
                tty: PathBuf::from("/dev/ttys014"),
            },
            Outcome::Succeeded,
            "git",
        );

        assert!(command.contains("'-subtitle' 'Succeeded'"));
        assert!(command.contains("'-message' 'git'"));
        assert!(command.contains("Terminal.icns"));
        assert!(command.contains("'terminal'"));
        assert!(command.contains("'/dev/ttys014'"));
    }

    #[test]
    fn each_terminal_has_its_own_icon_candidates() {
        assert!(terminal_icon_candidates(&iterm_target())[0]
            .to_string_lossy()
            .contains("iTerm"));
        assert!(terminal_icon_candidates(&FocusTarget::TerminalApp {
            tty: PathBuf::from("/dev/ttys014")
        })[0]
            .to_string_lossy()
            .contains("Terminal.app"));
    }

    #[test]
    fn focus_request_accepts_only_supported_valid_targets() {
        assert_eq!(
            parse_focus_target("iterm2", "F8176E01-630A-4505-9B2B-0DE870BF4706").unwrap(),
            iterm_target()
        );
        assert_eq!(
            parse_focus_target("terminal", "/dev/ttys014").unwrap(),
            FocusTarget::TerminalApp {
                tty: PathBuf::from("/dev/ttys014")
            }
        );
        assert!(parse_focus_target("iterm2", "not-a-uuid").is_err());
        assert!(parse_focus_target("terminal", "/tmp/ttys014").is_err());
        assert!(parse_focus_target("unknown", "/dev/ttys014").is_err());
    }

    #[test]
    fn focus_scripts_pass_locators_as_positional_arguments() {
        assert!(ITERM_FOCUS_SCRIPT.contains("item 1 of argv"));
        assert!(ITERM_FOCUS_SCRIPT.contains("unique ID of terminalSession"));
        assert!(ITERM_FOCUS_SCRIPT.contains("is not running then return"));
        assert!(ITERM_FOCUS_SCRIPT.contains("tell terminalSession to select"));
        assert!(!ITERM_FOCUS_SCRIPT.contains("F8176E01"));
        assert!(
            ITERM_FOCUS_SCRIPT
                .find("tell terminalSession to select")
                .unwrap()
                < ITERM_FOCUS_SCRIPT.rfind("activate").unwrap()
        );

        assert!(TERMINAL_FOCUS_SCRIPT.contains("item 1 of argv"));
        assert!(TERMINAL_FOCUS_SCRIPT.contains("tty of terminalTab"));
        assert!(TERMINAL_FOCUS_SCRIPT.contains("is not running then return"));
        assert!(!TERMINAL_FOCUS_SCRIPT.contains("/dev/ttys014"));
        assert!(
            TERMINAL_FOCUS_SCRIPT.find("activate").unwrap()
                < TERMINAL_FOCUS_SCRIPT
                    .find("set selected tab of terminalWindow")
                    .unwrap()
        );
    }
}
