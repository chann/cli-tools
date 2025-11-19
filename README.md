# CLI Tools Collection

A collection of powerful Rust-based CLI tools for developers.

## Tools

### code-cost

Analyze code repositories and calculate their monetary value based on development effort, complexity, and various metrics.

#### Features

- **Comprehensive Code Analysis**
  - Lines of code (LOC) with breakdown (code, comments, blanks)
  - Multiple programming languages with weighted difficulty scores
  - Cyclomatic complexity estimation
  - Project maturity scoring (tests, documentation, age)

- **Git Repository Analysis**
  - Commit count and history
  - Contributor analysis
  - Repository age tracking

- **Monetary Value Calculation**
  - Estimated development hours
  - Customizable hourly rate (default: ₩10,030 - 2025 South Korea minimum wage)
  - Language difficulty multipliers (Rust: 1.5x, C++: 1.4x, Go: 1.3x, etc.)
  - Complexity and maturity bonuses
  - Learning time estimation

- **Multiple Output Formats**
  - Beautiful colored table output
  - JSON (compact and pretty-printed)
  - CSV export
  - HTML report
  - Markdown documentation

#### Installation

```bash
cargo install --path crates/code-cost
```

Or build from source:

```bash
cargo build --release
```

#### Usage

Analyze current directory:
```bash
code-cost
```

Analyze multiple repositories:
```bash
code-cost ~/project1 ~/project2 ~/project3
```

JSON output:
```bash
code-cost --format json-pretty
```

Export to CSV:
```bash
code-cost --export report.csv
```

Export to HTML:
```bash
code-cost --export report.html
```

Custom hourly rate:
```bash
code-cost --hourly-rate 50000
```

#### Example Output

```
🔍 Code Cost Analyzer

ℹ Analyzing: .
✓ Analysis completed

┌────────────┬────────────┬────────┬─────────┬────────────┬──────────────────┐
│ Repository ┆ Lines      ┆ Files  ┆ Commits ┆ Est. Hours ┆ Total Cost (KRW) │
╞════════════╪════════════╪════════╪═════════╪════════════╪══════════════════╡
│ cli-tools  ┆       1257 ┆     19 ┆       1 ┆      197.3 ┆ ₩   1,979,217    │
└────────────┴────────────┴────────┴─────────┴────────────┴──────────────────┘

📊 Summary
  Total repositories: 1
  Total estimated hours: 197.3 hours
  Total estimated cost: ₩1,979,217
```

#### Value Calculation Algorithm

The tool uses a sophisticated algorithm to estimate code value:

1. **Base Calculation**: Lines of code ÷ 20 (assumes 20 lines per hour)
2. **Language Weights**: Different languages have different complexity multipliers
   - Rust: 1.5x
   - C++/C: 1.4x
   - Go: 1.3x
   - Java/C#/TypeScript: 1.2x
   - Python/Ruby: 1.1x
   - JavaScript: 1.0x
3. **Complexity Multiplier**: Based on code complexity (1.0 - 2.0x)
4. **Maturity Bonus**: Up to 30% extra for well-maintained projects (tests, docs, multiple contributors)
5. **Learning Time**: Estimated time needed to learn the technologies used

## Project Structure

```
cli-tools/
├── Cargo.toml              # Workspace configuration
├── crates/
│   ├── cli-core/           # Shared library for all tools
│   │   ├── src/
│   │   │   ├── output/     # Output formatters (table, JSON, CSV, HTML, MD)
│   │   │   ├── config/     # Configuration management
│   │   │   └── ui/         # UI theming and colors
│   └── code-cost/          # Code value analyzer tool
│       └── src/
│           ├── analyzer/   # Repository analysis logic
│           ├── calculator/ # Cost calculation
│           ├── metrics/    # Code metrics collection
│           └── git/        # Git repository analysis
└── README.md
```

## Development

### Prerequisites

- Rust 1.75+ (2021 edition)
- Cargo

### Building

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

### Adding a New Tool

1. Create a new crate in `crates/`:
   ```bash
   cargo new crates/your-tool
   ```

2. Add it to workspace in root `Cargo.toml`:
   ```toml
   [workspace]
   members = [
       "crates/cli-core",
       "crates/code-cost",
       "crates/your-tool",
   ]
   ```

3. Use `cli-core` for common functionality:
   ```toml
   [dependencies]
   cli-core = { path = "../cli-core" }
   ```

## Design Principles

- **Clean Architecture**: Separation of concerns with clear module boundaries
- **Extensibility**: Easy to add new tools and features
- **Maintainability**: Consistent code style and well-documented
- **Performance**: Efficient Rust implementation
- **User Experience**: Beautiful terminal UI with colored output

## License

MIT License - See LICENSE file for details

## Author

CHANN

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
