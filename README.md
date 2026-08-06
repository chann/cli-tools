# CLI Tools Collection

[한국어](README.ko.md)

Rust-based command-line tools for repository analysis, Git workflow maintenance,
everyday developer utilities, and silent command logging.

The repository also includes a reversible macOS helper that keeps physical
`Control-C` and `Control-G` working in iTerm2 while Korean input stays selected.

## Tools

| Command | Crate | Purpose |
| --- | --- | --- |
| `code-cost` | `crates/code-cost` | Estimate a repository's development cost and project value. |
| `work-summary` | `crates/work-summary` | Summarize Git activity, effort, and contribution value. |
| `git-tools` | `crates/git-tools` | Clean branches, scan markers, check project health, and manage changelogs. |
| `dev-tools` | `crates/dev-tools` | Run common data, encoding, network, text, and system utilities. |
| `zzz` | `crates/zzz` | Run a command silently and save stdout to `~/.commands`. |

`zzz` is a separate install target. Installing `dev-tools` no longer installs the
`zzz` binary.

## Installation

Install only the tools you need:

```bash
cargo install --path crates/code-cost --force
cargo install --path crates/work-summary --force
cargo install --path crates/git-tools --force
cargo install --path crates/dev-tools --force
cargo install --path crates/zzz --force
```

Build or test the full workspace:

```bash
cargo build --release --workspace --bins
cargo test --workspace --all-targets
```

## iTerm2 Korean Control Keys (macOS)

On iTerm2 `3.6.11`, `scripts/iterm2_korean_control_keys.py` maps physical
`Control-C` to PTY byte `0x03` and physical `Control-G` to `0x07`. It does not
switch the input source, run a resident process, or request Accessibility or
Input Monitoring permission.

This helper requires macOS, exactly iTerm2 `3.6.11`,
[`uv`](https://docs.astral.sh/uv/), and enabled and authorized iTerm2 Python API
access. Run each command from the repository root:

```bash
# Inspect global and profile mappings without changing preferences
uv run scripts/iterm2_korean_control_keys.py preflight

# Create a private backup and apply the two mappings
uv run scripts/iterm2_korean_control_keys.py apply

# Read the live configuration back
uv run scripts/iterm2_korean_control_keys.py verify
```

`apply` stops before writing if it finds a conflicting global or profile
mapping. It prints a private receipt under
`~/Library/Application Support/cli-tools/iterm2-korean-control-keys/`; receipt
directories use mode `0700` and files use `0600`.

To restore, pass the literal absolute receipt path printed by `apply`—do not
use a glob:

```bash
uv run scripts/iterm2_korean_control_keys.py restore \
  --receipt '/absolute/path/printed-by-apply/receipt.json'
```

Restore removes only entries owned by that receipt and refuses to overwrite an
owned entry changed later. It restores the original physical-key preference,
including an originally absent value, when the managed map returns to its
original state. If unrelated mappings changed meanwhile, it conservatively
keeps that preference enabled.

Verify the real terminal path with Korean input selected:

```bash
swift scripts/current_macos_input_source.swift
python3 scripts/probe_control_bytes.py
```

Press physical `Control-C`, then physical `Control-G`. The probe must print
`PASS: 03 07`; confirm Korean composition still works afterward. See the
[research and acceptance matrix](docs/research/2026-08-04-macos-korean-terminal-control-keys.md)
for the rationale and broader terminal findings.

## code-cost

Analyze repositories and estimate monetary value from code size, language
difficulty, complexity, maturity, and Git history.

```bash
# Analyze the current directory
code-cost

# Analyze specific repositories
code-cost ~/projects/my-app ../other-repo

# Compact table output
code-cost --simple

# Include developer-level cost breakdown
code-cost --dev-levels

# Export results
code-cost --format json-pretty
code-cost --export report.html
code-cost --export report.md
code-cost --export report.csv
```

Key outputs:

- LOC breakdown by code, comments, and blanks
- Language distribution and weighted difficulty
- Git age, commit, and contributor metrics
- Complexity, maturity, and code-quality scoring
- CSV, HTML, Markdown, JSON, and terminal output

## work-summary

Analyze Git commit history and produce a work summary with estimated time,
activity patterns, and value calculation.

```bash
# Analyze the last 30 days
work-summary

# Quick filters
work-summary --today
work-summary --week
work-summary --month

# Date range and limit
work-summary --from 2025-01-01 --to 2025-01-31
work-summary --limit 20

# Compact output and JSON export
work-summary --simple
work-summary --export summary.json
```

The estimator combines commit time gaps with code-change volume and complexity.

## git-tools

Developer workflow utilities for repository maintenance.

```bash
# Branch cleanup
git-tools cleanup
git-tools cleanup --force
git-tools cleanup --target develop

# Marker scan
git-tools scan
git-tools scan --markers "TODO,DEBUG"

# Project health
git-tools health
git-tools health --verbose

# Environment and changelog helpers
git-tools env
git-tools changelog
git-tools changelog --from v1.0.0 --limit 10

# Interactive Conventional Commit wizard
git-tools commit
```

## dev-tools

A collection of small utilities for common transformations and system checks.

```bash
# UUIDs
dev-tools uuid --count 5 --v7

# Base64
dev-tools base64 "hello world"
dev-tools base64 --decode "aGVsbG8gd29ybGQ="

# JSON syntax check, formatting, minification, and recursive key sorting
dev-tools json '{"b":2,"a":1}' --check
dev-tools json '{"b":2,"a":1}' --format
dev-tools json '{"b":2,"a":1}' --minify
dev-tools json '{"b":2,"a":1}' --sort asc
dev-tools json '{"b":2,"a":1}' --sort desc --minify

# Model generation
dev-tools typescript '{"name":"test","age":20}'
dev-tools rust '{"name":"test","age":20}'
dev-tools go '{"name":"test","age":20}'

# Ports, hashes, and time
dev-tools port 8080
dev-tools port 8080 --kill
dev-tools hash README.md --file
dev-tools checksum README.md --file --algo sha512
dev-tools time
dev-tools time 1740000000

# Text, security, and network helpers
dev-tools password --check "P@ssw0rd123"
dev-tools morse "HELLO WORLD"
dev-tools morse --decode ".... . .-.. .-.. --- / .-- --- .-. .-.. -.."
dev-tools ip
```

`dev-tools silent` remains available for command logging:

```bash
dev-tools silent git status --short
dev-tools silent python script.py
```

## zzz

`zzz` is a standalone shortcut for silent command logging. It runs the command
through your interactive shell on Unix, so shell aliases and functions are
available. The command is detached and runs in the background, returning to the
prompt immediately (no trailing `&` needed); its output is captured to the log
file. On macOS, `zzz` sends a system notification when the background command
finishes, reporting whether it succeeded or failed.

Install [`alerter`](https://github.com/vjeantet/alerter) for clickable iTerm2
and Terminal.app notifications on current macOS releases:

```bash
brew install vjeantet/tap/alerter
```

In iTerm2 and Terminal.app, the notification uses the launching terminal's icon.
Clicking it returns keyboard focus to the exact launching session or tab,
including an iTerm2 session that hosts Herdr. macOS may ask for Automation
and notification permission the first time. `zzz` keeps the click listener in a
detached worker, so `--wait` still returns as soon as the command and notification
launch finish. Unread alerts and their click workers expire after 10 minutes
instead of leaving helper processes behind indefinitely.

The legacy
[`terminal-notifier`](https://github.com/julienXX/terminal-notifier) remains a
fallback when `alerter` is unavailable. If neither tool is installed, these
terminals fall back to a generic completion notification without exact-session
focus.

Ghostty uses its built-in native macOS notification channel, so it does not
require either notification tool. The notification carries Ghostty's app icon, and
clicking it returns to the exact originating Ghostty surface. macOS may ask for
notification permission the first time. If an originating target has closed,
`zzz` does not create a replacement window.

```bash
zzz echo "hello"
zzz git status --short
zzz update-agents
zzz --wait cargo test
zzz --print-log make build
```

Logs are saved to:

```text
~/.commands/{yymmdd}/{hhmmss}-{command_name}.log
```

Run `zzz --help` for the styled command reference. Useful options include:

- `--no-notify` — skip the completion notification.
- `-w, --wait` — wait for completion and return the command's exit status.
- `-p, --print-log` — print the log path immediately after launch.
- `--color auto|always|never` — control help and error colors.

Options placed after the command are passed through unchanged. Use an explicit
`--` to make the boundary between `zzz` options and the command unmistakable:

```bash
zzz -- rg --files -g '*.rs'
```

Example:

```text
~/.commands/260605/224512-echo.log
```

## Release

Releases are driven by Git tags.

1. Update `workspace.package.version` in `Cargo.toml`.
2. Run `cargo test --workspace --all-targets`.
3. Create and push a matching tag:

   ```bash
   git tag v0.1.0
   git push origin v0.1.0
   ```

The GitHub Actions release workflow validates that the tag matches the workspace
version, runs the workspace tests, builds platform archives, and creates a GitHub
Release. Release artifacts include:

- Linux x86_64: `tar.gz`
- macOS Intel: `tar.gz`
- macOS Apple Silicon: `tar.gz`
- Windows x86_64: `zip`

See [versioning.md](versioning.md) for the shared versioning rules.

## Project Structure

```text
cli-tools/
├── .github/workflows/release.yml
├── crates/
│   ├── cli-core/       # Shared UI, output, config, and command logging
│   ├── code-cost/      # Repository value analyzer
│   ├── dev-tools/      # Developer utility collection
│   ├── git-tools/      # Git workflow and health tools
│   ├── work-summary/   # Git work summary analyzer
│   └── zzz/            # Standalone silent command logger
├── scripts/             # iTerm2 migration and terminal probes
├── Cargo.toml
└── versioning.md
```

## License

MIT License. See [LICENSE](LICENSE).

## Author

CHANN
