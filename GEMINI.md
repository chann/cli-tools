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
- `crates/dev-utils`: General purpose developer utilities.
- `crates/work-summary`: Git-based work analyzer.
- `crates/dev-pulse`: Project health and maintenance tools.
- `crates/code-cost`: Repository valuation tool.

## Implementation Guide for `dev-utils`

When adding a new command to `dev-utils`:
1. Create a new module in `crates/dev-utils/src/commands/`.
2. Register the module in `crates/dev-utils/src/commands/mod.rs`.
3. Add the command to the `Commands` enum in `crates/dev-utils/src/main.rs`.
4. Implement the logic in the new module, using `cli-core` where appropriate.
