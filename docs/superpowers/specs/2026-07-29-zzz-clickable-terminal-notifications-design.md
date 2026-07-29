# zzz Clickable Terminal Notifications Design

## Goal

When a macOS completion notification from `zzz` is clicked, return keyboard
focus to the exact terminal session that launched the background command. Show
that terminal application's icon for both success and failure notifications.

The first implementation supports iTerm2 and Terminal.app. Its delivery
boundary must allow a future Ghostty adapter without changing command execution.

## Existing Behavior

`zzz` delegates to `cli_core::command_log`, which launches the user's
interactive shell in a detached process group and returns control to the prompt
immediately. The detached shell redirects command stdout to
`~/.commands/{yymmdd}/{hhmmss}-{command_name}.log`, captures the final status,
and invokes `/usr/bin/osascript` to display a success or failure notification.

The current notification is posted by `osascript`. It does not retain the
terminal session that launched the command, its icon is not the launching
terminal's icon, and clicking it cannot reveal the original window, tab, or
split.

## Herdr Findings

Herdr has two macOS notification paths:

- terminal delivery emits the terminal's native notification control sequence;
- system delivery prefers `terminal-notifier -activate <bundle-id>` and falls
  back to `osascript`.

For iTerm2, Herdr emits OSC 9. iTerm2 attaches its current window, tab, and split
session indices when it parses that sequence. Notification Center therefore
shows the iTerm2 icon, and iTerm2's own click handler reveals the exact source
session. This is stronger and simpler than posting a third-party notification
and reconstructing iTerm2 focus with AppleScript.

The `zzz` process can run inside Herdr. Its controlling TTY then belongs to the
inner Herdr pane, while the inherited `ITERM_SESSION_ID` still identifies the
outer iTerm2 session. `zzz` must resolve that session's outer TTY before it
detaches and write the completion OSC sequence to that TTY.

Terminal.app does not support Herdr's OSC terminal-delivery path. Exact
Terminal.app tab restoration therefore remains an explicit bridge built on
`terminal-notifier -execute` plus a validated AppleScript focus handler.

## Supported Terminal Contract

### iTerm2

At `zzz` invocation time:

1. require `TERM_PROGRAM=iTerm.app` or iTerm2's bundle identifier;
2. parse the UUID portion of `ITERM_SESSION_ID`;
3. use constant AppleScript, with the UUID supplied as a positional argument,
   to find the matching iTerm2 session and return its outer `/dev/tty*` path.

After command completion, write a sanitized OSC 9 message directly to the
captured outer TTY. iTerm2 owns notification presentation, icon selection, and
click restoration. No `terminal-notifier` dependency or custom iTerm2 focus
handler is involved.

If the session no longer exists or its TTY cannot be opened, notification
failure is ignored and the command's original status is preserved.

### Terminal.app

At invocation time, capture the current controlling TTY. Resolve
`terminal-notifier` from `PATH` and the standard Homebrew locations.

After command completion, invoke `terminal-notifier` with:

- title `zzz`;
- subtitle `Succeeded` or `Failed`;
- the sanitized command basename as the message;
- `-appIcon` pointing to Terminal.app's application icon when available; and
- `-execute` containing a shell-quoted internal `zzz` focus command.

When clicked, the internal handler locates the Terminal.app tab whose `tty`
matches the captured locator, selects that tab, raises its containing window,
and activates Terminal.app. If the tab has closed, it activates Terminal.app
without creating a replacement window or tab.

If `terminal-notifier` is unavailable, use the existing generic `osascript`
notification. The fallback still reports completion, but cannot guarantee the
Terminal.app icon or exact-tab click behavior.

### Future Ghostty Support

The delivery model distinguishes terminal-owned control-sequence notifications
from system notifications. A future Ghostty adapter can capture the outer
Ghostty surface and emit its supported notification sequence without changing
the detached command runner.

Ghostty 1.2 exact-session AppleScript automation is outside this change.

## Architecture

Add a macOS-only notification module behind `cli_core::command_log`. It owns:

1. launch-time terminal detection and target capture;
2. success and failure notification command construction;
3. OSC message sanitization and outer-TTY delivery commands;
4. Terminal.app notifier/icon resolution; and
5. Terminal.app locator validation and focus restoration.

The detached wrapper receives fully constructed success and failure commands.
It never re-reads terminal environment variables after the user may have moved
elsewhere.

The `zzz` binary exposes one undocumented, reserved internal focus mode for
Terminal.app clicks. It accepts only a fixed terminal kind and a validated TTY
locator. Normal user commands never reach the focus adapter.

## Data Flow

### iTerm2

1. `zzz` validates its command-line arguments.
2. It resolves `ITERM_SESSION_ID` to the matching outer iTerm2 TTY.
3. The detached interactive-shell command starts and `zzz` returns.
4. The wrapper runs the command and captures its exit status.
5. The wrapper writes a success or failure OSC 9 sequence to the outer TTY.
6. iTerm2 posts the notification with its own icon and source-session context.
7. Clicking the notification makes iTerm2 reveal the original session.

### Terminal.app

1. `zzz` captures the launching TTY, current executable, notifier, and icon.
2. The detached command starts and `zzz` returns.
3. The wrapper posts a success or failure notification through
   `terminal-notifier`.
4. Clicking the notification invokes the internal `zzz` focus mode.
5. The Terminal.app adapter selects the matching tab and activates its window.

## Security and Quoting

Command arguments remain excluded from notification text because they may
contain credentials. Only the already-sanitized executable basename is shown.

OSC text removes ESC, BEL, ST, carriage returns, newlines, and tabs so command
names cannot inject a second terminal control sequence. TTY paths must begin
with `/dev/tty`, contain no path separator after that prefix, and use a small
ASCII allowlist.

Every executable path, icon path, terminal kind, and locator embedded in a
shell command uses the existing single-quote escaping primitive. AppleScript
values are positional `osascript` arguments, never interpolated into source.

Notification delivery is best effort. Its stdout and stderr do not enter the
command log, and its failure never replaces the user's command status.

## Error Handling

- Exit status zero reports `Succeeded`; any non-zero status reports `Failed`.
- Failure to detect a supported terminal uses the generic `osascript` path.
- An iTerm2 lookup or outer-TTY write failure preserves the command status.
- A missing `terminal-notifier` uses the generic notification for Terminal.app.
- A missing Terminal.app icon omits `-appIcon` while preserving click behavior.
- A closed Terminal.app tab activates Terminal.app without creating UI.
- A denied Automation request leaves focus unchanged and shows no extra dialog.
- Non-macOS behavior remains unchanged.

## Testing

### Unit Tests

- Parse iTerm2 session environment values and retain only a valid UUID.
- Resolve an iTerm2 UUID to an outer TTY using injected lookup output.
- Detect Terminal.app and validate its current TTY.
- Reject malformed TTY locators and unknown internal focus kinds.
- Sanitize OSC text and build exact success and failure OSC 9 sequences.
- Build Terminal.app notifier arguments with icon and exact focus command.
- Resolve notifier paths in deterministic priority order.
- Generate constant AppleScript whose values are positional arguments.
- Prove notification failure preserves the user's command status.

### Integration Tests

Use fake notifier and `osascript` executables or injected commands to prove:

- success and failure retain the same launch-time target;
- command stdout remains in the existing log;
- iTerm2 delivery targets the captured outer TTY;
- Terminal.app clicks dispatch the validated internal focus mode; and
- the generic `osascript` fallback remains available.

The existing `zzz` integration tests continue to cover immediate return,
background log writing, interactive-shell aliases, and version output.

### Runtime Verification

After focused and workspace gates:

1. reinstall `zzz` from the verified checkout;
2. launch successful and failing commands from iTerm2, including from inside
   Herdr, move to another session, click each notification, and type an unsent
   sentinel to prove the original split owns keyboard input;
3. repeat from Terminal.app after moving to another tab or application;
4. visually confirm each notification uses the launching terminal's icon; and
5. confirm success and failure logs and command statuses remain unchanged.

## Documentation

Update both README languages to explain:

- iTerm2's built-in clickable notification behavior, including Herdr sessions;
- exact Terminal.app tab restoration;
- `terminal-notifier` is required only for full Terminal.app click behavior;
- generic fallback behavior; and
- no new terminal window is created when a target has closed.

Keep the standalone `zzz` install command unchanged.

## Scope

This change does not add Ghostty 1.2 UI automation, create terminal windows,
surface command arguments in notifications, change `dev-tools silent`, or
alter non-macOS command execution.
