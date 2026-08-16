# CLI Tools Project Instructions

This project is a collection of Rust-based CLI utilities.

## General Conventions

- **Error Handling**: Use `anyhow::Result` for fallible functions. Use `context()` to provide meaningful error messages.
- **UI & Theming**: Use the `cli_core::ui::Theme` for consistent terminal output colors and icons.
- **Output Formatting**: Prefer using `cli_core::output` formatters (Table, JSON, CSV, etc.) for any data-heavy output.
- **Async**: Use `tokio` for any I/O bound tasks, especially network requests.
- **Git Integration**: Use `git2` for repository analysis and `ignore` for respecting `.gitignore`.

## Workspace Structure

- `crates/cli-core`: Shared foundation.
- `crates/dev-tools`: General purpose developer utilities.
- `crates/work-summary`: Git-based work analyzer.
- `crates/git-tools`: Project health and maintenance tools.
- `crates/code-cost`: Repository valuation tool.

## Implementation Guide for `dev-tools`

When adding a new command to `dev-tools`:
1. Create a new module in `crates/dev-tools/src/commands/`.
2. Register the module in `crates/dev-tools/src/commands/mod.rs`.
3. Add a new variant to the `Commands` enum in `crates/dev-tools/src/commands/mod.rs`.
4. Add a dispatch arm in `crates/dev-tools/src/main.rs`.
5. Implement the logic in the new module, using `cli-core` where appropriate.
