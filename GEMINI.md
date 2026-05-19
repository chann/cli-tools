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
- `crates/devtools`: General purpose developer utilities.
- `crates/work-summary`: Git-based work analyzer.
- `crates/gittools`: Project health and maintenance tools.
- `crates/code-cost`: Repository valuation tool.

## Implementation Guide for `devtools`

When adding a new command to `devtools`:
1. Create a new module in `crates/devtools/src/commands/`.
2. Register the module in `crates/devtools/src/commands/mod.rs`.
3. Add the command to the `Commands` enum in `crates/devtools/src/main.rs`.
4. Implement the logic in the new module, using `cli-core` where appropriate.
