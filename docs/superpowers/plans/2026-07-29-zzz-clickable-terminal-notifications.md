# zzz Clickable Terminal Notifications Implementation Plan

> Execute this plan test-first from the project-local
> `.worktrees/zzz-clickable-terminal-notifications` worktree.

**Goal:** Make macOS `zzz` completion notifications show the launching
terminal's icon and restore keyboard focus to the exact launching iTerm2 session
or Terminal.app tab when clicked.

**Architecture:** Capture the terminal target before detaching. For iTerm2,
resolve the inherited session UUID to the outer TTY and emit Herdr-style OSC 9
so iTerm2 owns the icon and exact click context. For Terminal.app, use
`terminal-notifier -appIcon -execute` and a validated internal `zzz` focus mode.
Keep the current `osascript` notification as a best-effort fallback.

**Tech Stack:** Rust 2021, POSIX shell generation, macOS AppleScript, iTerm2
OSC 9, `terminal-notifier` 2.x, Cargo unit and integration tests.

---

## Task 1: Create the isolated implementation checkout

**Files:**

- Reuse: `.gitignore`
- Create checkout: `.worktrees/zzz-clickable-terminal-notifications`
- Create branch: `feat/zzz-clickable-terminal-notifications`

1. Confirm `.worktrees/` is ignored and the main checkout contains only the
   known user-owned README and JSON changes.
2. Create the project-local worktree from the latest `main`.
3. Run:

   ```bash
   cargo build
   cargo test --workspace --all-targets
   ```

4. Stop if the clean worktree does not pass its baseline.

## Task 2: Capture and validate launch-time terminal targets

**Files:**

- Create: `crates/cli-core/src/command_log/macos_notification.rs`
- Modify: `crates/cli-core/src/command_log/mod.rs`

1. Register a macOS-only notification module.
2. Write failing unit tests for:
   - iTerm2 recognition from `TERM_PROGRAM` and bundle identifier;
   - UUID extraction from `ITERM_SESSION_ID`;
   - rejection of malformed session identifiers;
   - Terminal.app recognition;
   - valid `/dev/tty*` locators; and
   - rejection of traversal, whitespace, and control characters in locators.
3. Run the focused tests and record the expected RED result.
4. Implement small terminal/environment value types and validators.
5. Separate pure environment parsing from side effects so lookup results can be
   injected in tests.
6. Run the focused tests and verify GREEN.

## Task 3: Resolve the outer iTerm2 TTY

**Files:**

- Modify: `crates/cli-core/src/command_log/macos_notification.rs`

1. Write failing tests proving the iTerm2 lookup:
   - passes the UUID as an `osascript` positional argument;
   - accepts only a validated TTY returned on stdout;
   - trims a single trailing newline; and
   - rejects empty, malformed, and unsuccessful lookup results.
2. Add constant AppleScript that traverses iTerm2 windows, tabs, and sessions,
   returning the matching session's `tty`.
3. Invoke `/usr/bin/osascript` synchronously before the command detaches.
4. Keep AppleScript source constant; never interpolate the UUID.
5. Verify focused tests.
6. From a shell inside Herdr, compare the resolved TTY with the outer iTerm2
   session TTY to prove it does not select Herdr's inner pane PTY.

## Task 4: Build terminal-owned iTerm2 notifications

**Files:**

- Modify: `crates/cli-core/src/command_log/macos_notification.rs`
- Modify: `crates/cli-core/src/command_log/mod.rs`

1. Write failing tests for:
   - exact OSC 9 framing (`ESC ] 9 ; … ESC \`);
   - distinct `Succeeded` and `Failed` messages;
   - command basename only, never command arguments;
   - removal of ESC, BEL, ST, newlines, carriage returns, and tabs;
   - shell quoting of the captured outer TTY; and
   - notification failure preserving the command status.
2. Implement OSC sanitization based on Herdr's control-sequence safety rules.
3. Build a best-effort shell command that writes the OSC sequence directly to
   the captured outer TTY with notification output suppressed.
4. Refactor the detached wrapper to accept prebuilt success and failure
   notification commands captured at launch time.
5. Run command-log unit tests and existing `zzz` integration tests.

## Task 5: Build exact Terminal.app click restoration

**Files:**

- Modify: `crates/cli-core/src/command_log/macos_notification.rs`
- Modify: `crates/cli-core/src/command_log/mod.rs`
- Modify: `crates/zzz/src/main.rs`
- Create: `crates/zzz/tests/focus.rs`

1. Write failing tests for deterministic notifier resolution:
   - executable entries found through `PATH`;
   - `/opt/homebrew/bin/terminal-notifier`;
   - `/usr/local/bin/terminal-notifier`; and
   - no notifier available.
2. Write failing tests for notifier arguments:
   - `-title zzz`;
   - outcome subtitle;
   - sanitized command basename;
   - Terminal.app icon when present;
   - `-execute` with the exact TTY locator; and
   - shell quoting for executable and locator paths.
3. Write failing unit and integration tests for the internal focus mode:
   - accept only `terminal`;
   - accept exactly one validated TTY;
   - reject extra arguments;
   - reject unknown kinds and malformed locators; and
   - pass the locator to constant AppleScript as a positional argument.
4. Implement controlling-TTY capture in the foreground.
5. Resolve the Terminal.app icon from its standard application bundles.
6. Implement the notifier command and `osascript` fallback.
7. Add the reserved `--__zzz-focus-terminal terminal <tty>` dispatch to the
   `zzz` binary.
8. Implement constant Terminal.app AppleScript that selects the matching tab,
   raises its containing window, and activates Terminal without creating UI.
9. Run focused module and binary tests.

## Task 6: Verify and publish the code checkpoint

**Files:**

- `crates/cli-core/src/command_log/macos_notification.rs`
- `crates/cli-core/src/command_log/mod.rs`
- `crates/zzz/src/main.rs`
- `crates/zzz/tests/focus.rs`

1. Run:

   ```bash
   cargo fmt --all -- --check
   cargo test -p cli-core command_log
   cargo test -p zzz --all-targets
   cargo clippy -p cli-core -p zzz --all-targets -- -D warnings
   git diff --check
   ```

2. Inspect the diff for argument exposure, unsafe AppleScript interpolation,
   status clobbering, or non-macOS regressions.
3. Commit only the code and tests:

   ```bash
   git add \
     crates/cli-core/src/command_log/macos_notification.rs \
     crates/cli-core/src/command_log/mod.rs \
     crates/zzz/src/main.rs \
     crates/zzz/tests/focus.rs
   git commit -m "feat(zzz): focus launching terminal from notifications"
   ```

4. Push the feature branch immediately and verify local/upstream parity.

## Task 7: Runtime-prove the interaction

1. Install `zzz` from the feature worktree:

   ```bash
   cargo install --path crates/zzz --force
   ```

2. In the current iTerm2 session inside Herdr:
   - launch one successful delayed command;
   - launch one failing delayed command;
   - move to another session before each completes;
   - click each notification;
   - type and clear an unsent sentinel to prove keyboard input returned to the
     original split; and
   - visually verify the iTerm2 icon.
3. Repeat success and failure from direct iTerm2 if needed to distinguish outer
   TTY forwarding from direct terminal delivery.
4. In Terminal.app:
   - record the source tab TTY;
   - launch success and failure;
   - move to another tab or app;
   - click each notification;
   - verify selected tab/window and keyboard input; and
   - visually verify the Terminal.app icon.
5. Close a target session before clicking and confirm no replacement UI is
   created.
6. Confirm the expected log files and command statuses remain correct.

## Task 8: Merge code without disturbing user changes

1. Do not modify README files on the feature branch because both README files
   already contain unrelated user-owned edits in the main checkout.
2. Fast-forward `main` to the verified code branch. The code paths do not
   overlap the existing user changes.
3. Push `main` immediately and verify `main...origin/main` is `0 0`.
4. Confirm the original README and JSON edits remain present and unstaged.

## Task 9: Document with partial staging

**Files:**

- Modify: `README.md`
- Modify: `README.ko.md`

1. Add concise English and Korean documentation covering:
   - iTerm2 native clickable notifications and Herdr support;
   - exact Terminal.app tab restoration;
   - `terminal-notifier` required only for full Terminal.app behavior;
   - generic fallback behavior; and
   - no replacement window when the original target has closed.
2. Use `git diff` to distinguish new notification hunks from the user's JSON
   documentation.
3. Stage only the new notification hunks with `git add -p`.
4. Inspect both cached and unstaged diffs before committing.
5. Commit:

   ```bash
   git commit -m "docs(zzz): document terminal-aware notifications"
   ```

6. Push immediately and verify parity.

## Task 10: Full verification and final audit

1. Run:

   ```bash
   cargo fmt --all -- --check
   cargo test --workspace --all-targets
   cargo clippy --workspace --all-targets -- -D warnings
   cargo build --workspace
   git diff --check
   ```

2. Verify the installed `zzz --version` matches the checkout package version.
3. Re-run a short successful and failing notification smoke test.
4. Confirm every task commit is on `origin/main`.
5. Confirm `main...origin/main` reports `0 0`.
6. Confirm the only remaining changes are the pre-existing user-owned README
   and JSON work.
7. Remove the feature worktree only after every verification passes, using the
   branch-finishing workflow and a recoverable sequence.
