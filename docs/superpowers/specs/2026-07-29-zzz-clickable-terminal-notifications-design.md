# zzz Clickable Terminal Notifications Design

## Goal

When a macOS completion notification from `zzz` is clicked, return keyboard
focus to the exact terminal session that launched the background command. Show
that terminal application's icon in both success and failure notifications.

The first implementation supports iTerm2 and Terminal.app. Its terminal adapter
boundary must allow Ghostty 1.3 or newer to be added without changing command
execution or notification delivery.

## Existing Behavior

`zzz` delegates to `cli_core::command_log`, which launches the user's
interactive shell in a detached process group and returns control to the prompt
immediately. The detached shell redirects command stdout to
`~/.commands/{yymmdd}/{hhmmss}-{command_name}.log`, captures the final status,
and invokes `/usr/bin/osascript` to display a success or failure notification.

The current notification does not retain the terminal session that launched the
command. Its icon belongs to the process that posts the notification, and
clicking it cannot select the original terminal window, tab, or split.

## Supported Terminal Contract

### iTerm2

At `zzz` invocation time, capture the UUID portion of `ITERM_SESSION_ID` or
`TERM_SESSION_ID`, together with iTerm2's bundle identifier. When the
notification is clicked, locate the session with that UUID through iTerm2's
AppleScript object model, select the session and its containing tab and window,
and activate iTerm2.

The UUID is stable even if tab or window indices change while the background
command runs. If the session no longer exists, activate iTerm2 without creating
a replacement session.

### Terminal.app

At invocation time, capture the controlling TTY and Terminal.app's bundle
identifier. When the notification is clicked, locate the Terminal tab whose
`tty` property matches the captured value, mark it as the selected tab, bring
its containing window to the front, and activate Terminal.

If the tab no longer exists, activate Terminal without creating a new tab or
window.

### Future Ghostty Support

Represent the target as a terminal kind plus an opaque, terminal-specific
locator. Notification delivery and command execution do not inspect the
locator. A future Ghostty 1.3-or-newer adapter can use Ghostty's AppleScript
terminal identifiers and `focus` command without changing the shared flow.

Ghostty 1.2 is explicitly outside this design because it does not expose the
AppleScript terminal-focus API needed for reliable exact-session restoration.

## Architecture

Add a macOS-only terminal-notification module behind
`cli_core::command_log`. It owns three independent responsibilities:

1. `TerminalTarget` detects and captures the launching terminal session.
2. `NotificationDelivery` produces the completion notification.
3. `TerminalFocus` restores the captured target when the user clicks.

The public command runner accepts the captured notification context rather than
reading terminal environment variables from the detached shell. This preserves
the launch-time session even if the user moves elsewhere before completion.

The `zzz` binary also exposes an undocumented internal focus mode. Notification
activation invokes the same installed executable with the serialized terminal
kind and opaque locator. Normal command parsing rejects malformed internal
focus arguments and never interprets them as a user command.

## Notification Delivery

For full clickable behavior, resolve `terminal-notifier` from `PATH` and the
standard Homebrew locations `/opt/homebrew/bin/terminal-notifier` and
`/usr/local/bin/terminal-notifier`.

The completion wrapper invokes it with:

- title `zzz`;
- subtitle `Succeeded` or `Failed`;
- message containing only the sanitized command basename;
- `-appIcon` pointing to the detected terminal application's icon; and
- `-execute` containing the shell-escaped internal `zzz` focus command.

Use `-appIcon`, not `-sender`. Faking the sender prevents
`terminal-notifier` from receiving the click and running `-execute`.

iTerm2 and Terminal icon locations are resolved from their application bundle
metadata. Standard installation paths are checked first; Spotlight lookup may
be used as a fallback. If no icon can be resolved, omit `-appIcon` rather than
failing notification delivery.

If `terminal-notifier` is unavailable, retain the existing `osascript`
notification as a compatibility fallback. The fallback reports completion but
cannot promise the terminal icon or click-to-session behavior. Documentation
must state that installing `terminal-notifier` is required for the complete
macOS interaction.

## Data Flow

1. `zzz` validates its command-line arguments.
2. On macOS, it captures the current executable path and `TerminalTarget`.
3. The normal detached interactive-shell command starts and `zzz` returns.
4. The detached wrapper runs the command and captures its exit status.
5. The wrapper posts a success or failure notification using the captured
   context.
6. Notification delivery exits without changing the command status.
7. When clicked, `terminal-notifier` launches the internal focus mode.
8. The matching terminal adapter selects the captured session, tab, and window
   and activates the application.

## Security and Quoting

Command arguments remain excluded from notification text because they may
contain credentials. Only the sanitized executable basename is displayed.

Every executable path, icon path, terminal kind, and opaque locator passed
through `-execute` is shell-quoted with the existing single-quote escaping
primitive. The internal focus mode accepts a fixed terminal-kind allowlist and
validates locator syntax before generating AppleScript.

AppleScript values are passed as positional `osascript` arguments where
possible instead of being interpolated into source. Generated scripts contain
only constant program text. Notification and focus failures remain detached
from command output and never write into the command log.

## Error Handling

- A command with exit status zero sends `Succeeded`; any non-zero status sends
  `Failed`.
- Notification delivery failure never replaces the command's original status.
- Failure to detect a supported terminal falls back to the existing generic
  notification.
- A missing `terminal-notifier` falls back to the existing generic
  notification.
- A missing icon omits `-appIcon` while preserving click behavior.
- A closed target session activates the terminal app without creating UI.
- A denied AppleScript automation request leaves the user's current focus
  unchanged and exits the internal handler unsuccessfully without displaying an
  additional error dialog.
- Non-macOS behavior remains unchanged.

## Testing

### Unit Tests

- Parse iTerm2 session environment values and retain only the UUID locator.
- Detect Terminal.app and capture its TTY locator.
- Reject unknown terminal kinds and malformed locators.
- Resolve notifier paths in deterministic priority order.
- Build success and failure notifier arguments with the expected title,
  subtitle, message, icon, and click command.
- Prove all click-command values are shell-quoted.
- Generate iTerm2 and Terminal.app focus scripts without interpolating locator
  values into AppleScript source.

### Integration Tests

Use fake notifier and `osascript` executables to prove:

- success and failure use the same terminal target and icon;
- a notifier failure preserves the background command status;
- command stdout remains in the existing log;
- clicking dispatches the expected internal focus arguments; and
- the `osascript` compatibility fallback remains available.

The existing `zzz` integration tests continue to cover immediate return,
background log writing, interactive-shell alias support, and version output.

### Runtime Verification

After focused and workspace tests, strict scoped Clippy, formatting, and build
checks pass:

1. Reinstall `zzz` from the current checkout.
2. From iTerm2, launch one successful and one failing command, move to a
   different session, click each notification, and verify the original split
   receives keyboard input.
3. Repeat from Terminal.app after moving to another tab or application.
4. Confirm each notification visually uses the launching terminal's icon.
5. Confirm success and failure logs and exit-status behavior remain unchanged.

## Documentation

Update both README languages to describe clickable macOS notifications,
iTerm2 and Terminal.app support, the no-new-window fallback, and the
`terminal-notifier` installation requirement. Keep the existing standalone
`zzz` install command unchanged.

## Scope

This change does not add Ghostty 1.2 UI automation, create terminal windows,
surface command arguments in notifications, change `dev-tools silent`, or
alter non-macOS command execution.
