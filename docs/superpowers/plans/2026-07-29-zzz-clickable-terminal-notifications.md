# zzz Clickable Terminal Notifications Implementation Plan

> Execute test-first from the project-local
> `.worktrees/zzz-clickable-terminal-notifications` worktree.

**Goal:** Make macOS `zzz` success and failure notifications show the launching
terminal's icon and restore keyboard focus to the exact launching iTerm2 session
or Terminal.app tab when clicked.

**Architecture:** Capture a stable locator before detaching: the iTerm2 session
UUID or Terminal.app TTY. Follow Herdr's macOS system-notification path with
`terminal-notifier`, replacing bundle-only activation with an exact
`-execute` focus callback. Retain `osascript` as the generic fallback.

**Tech Stack:** Rust 2021, POSIX shell generation, macOS AppleScript,
`terminal-notifier` 2.x, Cargo unit and integration tests.

---

## Task 1: Isolate and baseline

1. Ignore project-local `.worktrees/`.
2. Create branch `feat/zzz-clickable-terminal-notifications` at
   `.worktrees/zzz-clickable-terminal-notifications`.
3. Verify:

   ```bash
   cargo build
   cargo test --workspace --all-targets
   ```

4. Keep the main checkout's user-owned README and JSON work untouched.

## Task 2: Capture terminal targets

**Files:**

- Create: `crates/cli-core/src/command_log/macos_notification.rs`
- Modify: `crates/cli-core/src/command_log/mod.rs`

1. Write failing tests for:
   - iTerm2 detection from `TERM_PROGRAM` or bundle identifier;
   - UUID extraction from `ITERM_SESSION_ID`;
   - malformed UUID rejection;
   - Terminal.app detection;
   - valid `/dev/tty*` locators; and
   - traversal, whitespace, and control-character rejection.
2. Implement pure environment parsing and target types.
3. Capture Terminal.app's controlling TTY before detaching.
4. Run focused tests to GREEN.

## Task 3: Build Herdr-derived system notifications

**Files:**

- Modify: `crates/cli-core/src/command_log/macos_notification.rs`
- Modify: `crates/cli-core/src/command_log/mod.rs`

1. Write failing tests for notifier resolution from:
   - `PATH`;
   - `/opt/homebrew/bin/terminal-notifier`;
   - `/usr/local/bin/terminal-notifier`; and
   - unavailable candidates.
2. Write failing tests for distinct iTerm2 and Terminal.app icon candidates.
3. Write failing tests for notifier commands containing:
   - `-title zzz`;
   - `Succeeded` or `Failed`;
   - command basename only;
   - `-appIcon`;
   - `-execute`;
   - exact terminal kind and locator; and
   - shell-quoted executable, icon, and locator paths.
4. Implement notifier and icon resolution.
5. Retain the existing `osascript` notification when the full path cannot be
   constructed.
6. Refactor the detached wrapper to accept prebuilt launch-time success and
   failure commands.
7. Run each notification in a subshell and prove even `exit 9` cannot replace
   the user's command status.

## Task 4: Restore exact focus

**Files:**

- Modify: `crates/cli-core/src/command_log/macos_notification.rs`
- Modify: `crates/cli-core/src/command_log/mod.rs`
- Modify: `crates/zzz/src/main.rs`
- Create: `crates/zzz/tests/focus.rs`

1. Add failing integration tests for:
   - unknown terminal kind;
   - malformed iTerm2 UUID;
   - malformed Terminal.app TTY;
   - missing arguments; and
   - extra arguments.
2. Add reserved dispatch:

   ```text
   --__zzz-focus-terminal <iterm2|terminal> <locator>
   ```

3. Implement constant AppleScript adapters. Locators are positional arguments.
4. Before addressing an app, use System Events to confirm it is already
   running so a click never relaunches a quit terminal.
5. For iTerm2:
   - activate first;
   - locate the UUID;
   - select containing window;
   - select containing tab; and
   - select the exact split session.
6. For Terminal.app:
   - activate first;
   - locate the matching TTY;
   - select its tab; and
   - raise its containing window.
7. Denied Automation permission exits unsuccessfully without a second dialog.
8. Run focus and existing `zzz` tests.

## Task 5: Stabilize parallel test fixtures

**Files:**

- Modify: `crates/cli-core/src/command_log/mod.rs`
- Modify: `crates/zzz/tests/zzz.rs`

1. Reproduce same-timestamp temporary HOME collisions under parallel tests.
2. Add a process-local atomic counter to each timestamp-based fixture name.
3. Re-run the affected tests concurrently.

## Task 6: Verify and publish code

1. Format only changed Rust files; do not mechanically rewrite unrelated
   repository files.
2. Run:

   ```bash
   cargo test -p cli-core command_log
   cargo test -p zzz --all-targets
   cargo clippy -p cli-core -p zzz --all-targets -- -D warnings
   git diff --check
   ```

3. Inspect for secret exposure, AppleScript interpolation, status clobbering,
   and non-macOS regressions.
4. Commit exact code and test paths:

   ```bash
   git commit -m "feat(zzz): focus launching terminal from notifications"
   ```

5. Push the feature branch immediately and verify local/upstream parity.

## Task 7: Runtime proof

### iTerm2

1. Deliver `/usr/bin/true` and `/usr/bin/false` through the built `zzz`.
2. Confirm `zzz / Succeeded / true` and `zzz / Failed / false` through
   `terminal-notifier -list`.
3. Select a different iTerm2 session and move to another app.
4. Execute the exact internal command used by `-execute`.
5. Confirm the frontmost app becomes iTerm2 and the current session UUID equals
   the launching Herdr/iTerm2 UUID.
6. Confirm the command includes the installed iTerm2 `.icns` path.

### Terminal.app

1. Run the equivalent source-tab and second-tab test.
2. Confirm selected tab TTY and frontmost app after the callback.
3. If macOS denies Apple Events, record the exact TCC error and retain unit,
   integration, and script-structure proof; do not alter system permissions
   without separate authorization.

## Task 8: Merge code without disturbing user changes

1. Keep README changes off the feature branch until code is verified.
2. Fast-forward `main` to the feature branch; code paths do not overlap the
   user's dirty README or JSON paths.
3. Push `main` immediately and prove `main...origin/main` is `0 0`.
4. Confirm user-owned changes remain unstaged.

## Task 9: Document with partial staging

**Files:**

- Modify: `README.md`
- Modify: `README.ko.md`

1. Document:
   - clickable iTerm2 and Terminal.app notifications;
   - exact launch-session restoration, including Herdr-hosted iTerm2 sessions;
   - `terminal-notifier` installation for full behavior;
   - macOS Automation permission;
   - generic fallback behavior; and
   - no replacement window for a closed target.
2. Stage only the new notification hunks with `git add -p`.
3. Inspect cached and unstaged diffs separately.
4. Commit:

   ```bash
   git commit -m "docs(zzz): document terminal-aware notifications"
   ```

5. Push immediately and verify parity.

## Task 10: Final verification

1. Run:

   ```bash
   cargo test --workspace --all-targets
   cargo clippy -p cli-core -p zzz --all-targets -- -D warnings
   cargo build --workspace
   git diff --check
   ```

2. Install `zzz` from the verified checkout and confirm `zzz --version`.
3. Re-run short success and failure delivery smoke tests.
4. Confirm every task commit is on `origin/main`.
5. Confirm `main...origin/main` is `0 0`.
6. Confirm only the pre-existing user-owned README and JSON changes remain.
7. Remove the feature worktree only after the branch-finishing workflow
   confirms publication and preservation.
