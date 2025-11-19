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

- **Advanced Analysis Features**
  - Detailed project metrics (complexity, maturity, code quality)
  - Language breakdown with percentages
  - AI usage estimation and code quality scoring
  - Developer level cost breakdown (Junior to Principal)
  - Test coverage statistics

- **Multiple Output Formats**
  - Beautiful colored table output
  - Detailed analysis mode (default) with comprehensive metrics
  - Simple mode (--simple) for basic summary
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

**Basic Analysis** (detailed mode is default):
```bash
code-cost
```

**Simple Mode** (table only):
```bash
code-cost --simple
```

**Developer Level Breakdown**:
```bash
code-cost --dev-levels
```

**Analyze Multiple Repositories**:
```bash
code-cost ~/project1 ~/project2 ~/project3
```

**JSON Output**:
```bash
code-cost --format json-pretty
```

**Export Results**:
```bash
# CSV format
code-cost --export report.csv

# HTML format
code-cost --export report.html

# Markdown format
code-cost --export report.md
```

**Custom Hourly Rate**:
```bash
code-cost --hourly-rate 50000
```

#### Example Output

**Detailed Mode** (default):
```
🔍 Code Cost Analyzer

ℹ Analyzing: .
✓ Analysis completed

┌────────────┬────────────┬────────┬─────────┬────────────┬──────────────────┐
│ Repository ┆ Lines      ┆ Files  ┆ Commits ┆ Est. Hours ┆ Total Cost (KRW) │
╞════════════╪════════════╪════════╪═════════╪════════════╪══════════════════╡
│ cli-tools  ┆       1658 ┆     20 ┆       4 ┆      237.2 ┆ ₩   2,379,027    │
└────────────┴────────────┴────────┴─────────┴────────────┴──────────────────┘

📁 cli-tools

ℹ Languages:
  • Rust - 80.9% (1,342 lines, 15 files)
  • Markdown - 11.9% (198 lines, 1 files)
  • TOML - 6.2% (102 lines, 3 files)
  • JSON - 1.0% (16 lines, 1 files)

ℹ Project Metrics:
  • Complexity Score: 2.84/5.0
  • Maturity Score: 20.8%
  • Code Quality: 34.1%
  • Test Files: 0 (0.0%)

ℹ AI Usage Analysis:
  • Estimated AI Usage: 25.0%
  • Indicators:
    - Consistent file size distribution
    - Use of modern programming languages

📊 Summary
  Total repositories: 1
  Total estimated hours: 237.2 hours
  Total estimated cost: ₩2,379,027
```

**With Developer Level Breakdown** (--dev-levels):
```
ℹ Developer Level Breakdown:
  • Junior       ₩  15,000/hr → ₩3,557,867
  • Mid-level    ₩  25,000/hr → ₩5,929,778
  • Senior       ₩  40,000/hr → ₩9,487,646
  • Lead         ₩  60,000/hr → ₩14,231,469
  • Principal    ₩ 100,000/hr → ₩23,719,115
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
6. **AI Usage Estimation**: Analyzes code patterns to estimate AI-assisted development
   - Low comment ratio with high code quality
   - Consistent file size distribution
   - High complexity with comprehensive testing
   - Modern language usage (Rust, TypeScript, Go, etc.)
   - Strong test coverage
7. **Developer Level Rates**: Market-based hourly rates for South Korea (2025)
   - Junior (1-3 years): ₩15,000/hr
   - Mid-level (3-5 years): ₩25,000/hr
   - Senior (5-10 years): ₩40,000/hr
   - Lead (10+ years, team lead): ₩60,000/hr
   - Principal (architect, senior engineer): ₩100,000/hr

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
