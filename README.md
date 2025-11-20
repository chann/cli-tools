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

### work-summary

Analyze git commit history and generate meaningful work activity summaries with time estimation and value calculation.

#### Features

- **Git Commit Analysis**
  - Complete commit history with diff tracking
  - File change statistics per commit
  - Author and timestamp information
  - Language-specific change tracking

- **Hybrid Time Estimation**
  - Time-gap based estimation (between commits)
  - Code change-based estimation (lines added/deleted with complexity)
  - Weighted average of both methods for accuracy
  - Configurable maximum session gap (4 hours default)

- **Work Pattern Analysis**
  - Hourly commit distribution
  - Daily commit patterns (weekdays vs weekends)
  - Peak working hours identification
  - Commit frequency and active days tracking

- **Value Calculation**
  - Developer level-based estimates (Junior to Principal)
  - Base hourly rate: ₩10,030 (2025 Korean minimum wage)
  - Level multipliers matching code-cost tool
  - Complexity-adjusted value estimation

- **Contributor Statistics**
  - Per-contributor commit counts
  - Lines added/deleted per contributor
  - Contribution percentage breakdown
  - Top contributors ranking

- **Date Filtering**
  - Date range filters: `--from`, `--to` (YYYY-MM-DD)
  - Quick filters: `--today`, `--week`, `--month`
  - Commit count limit: `--limit N`

- **Multiple Output Modes**
  - Simple mode: Basic summary with key metrics
  - Detail mode: Comprehensive analysis with all statistics
  - JSON export support
  - Multiple repository analysis

#### Installation

```bash
cargo install --path crates/work-summary
```

Or build from source:

```bash
cargo build --release --package work-summary
```

#### Usage

**Basic Analysis** (current directory, detailed mode):
```bash
work-summary
```

**Simple Summary**:
```bash
work-summary --simple
```

**Analyze Recent Commits**:
```bash
work-summary --limit 10
```

**Date Range Filtering**:
```bash
# Specific date range
work-summary --from 2025-01-01 --to 2025-01-31

# Quick filters
work-summary --today
work-summary --week
work-summary --month
```

**Multiple Repositories**:
```bash
work-summary ~/project1 ~/project2 ~/project3
```

**Export Results**:
```bash
work-summary --export summary.json --format json
```

**Custom Hourly Rate**:
```bash
work-summary --hourly-rate 15000
```

#### Example Output

**Simple Mode**:
```
Work Summary
v0.1.0

════════════════════════════════════════════════════════════
Total Summary
════════════════════════════════════════════════════════════

Repository: my-project
  Period: 2025-01-01 ~ 2025-01-31
  Commits: 42
  Estimated Hours: 105.5h
  Value (Mid-level): ₩1,587,232
```

**Detail Mode**:
```
════════════════════════════════════════════════════════════════════════════════
Repository: my-project
════════════════════════════════════════════════════════════════════════════════

Basic Information
  Period: 2025-01-01 ~ 2025-01-31
  Total Commits: 42
  Contributors: 3
  Files Changed: 127
  Lines: +2,450 / -856
  Estimated Hours: 105.5h

Recent Commits
┌──────────────────┬────────┬───────────────────────────────────────────────────────┬────────────┐
│ Time             ┆ Author ┆ Message                                               ┆ Changes    │
╞══════════════════╪════════╪═══════════════════════════════════════════════════════╪════════════╡
│ 2025-01-31 18:30 ┆ Alice  ┆ feat: add new authentication module                   ┆ +452 / -23 │
│ 2025-01-31 15:20 ┆ Bob    ┆ fix: resolve memory leak in worker pool               ┆ +18 / -34  │
│ 2025-01-30 09:15 ┆ Alice  ┆ refactor: improve error handling                      ┆ +89 / -67  │
└──────────────────┴────────┴───────────────────────────────────────────────────────┴────────────┘

Language Breakdown
┌────────────┬────────────┬───────────┬────────────┬───────┐
│ Language   ┆ Insertions ┆ Deletions ┆ Net Change ┆ %     │
╞════════════╪════════════╪═══════════╪════════════╪═══════╡
│ Rust       ┆ +1,840     ┆ -456      ┆ +1,384     ┆ 69.5% │
│ TypeScript ┆ +445       ┆ -289      ┆ +156       ┆ 22.2% │
│ Markdown   ┆ +165       ┆ -111      ┆ +54        ┆ 8.3%  │
└────────────┴────────────┴───────────┴────────────┴───────┘

Top Contributors
┌───────┬─────────┬────────────┬───────────┬────────┐
│ Name  ┆ Commits ┆ Insertions ┆ Deletions ┆ %      │
╞═══════╪═════════╪════════════╪═══════════╪════════╡
│ Alice ┆ 25      ┆ +1,520     ┆ -450      ┆ 59.5%  │
│ Bob   ┆ 12      ┆ +680       ┆ -289      ┆ 28.6%  │
│ Carol ┆ 5       ┆ +250       ┆ -117      ┆ 11.9%  │
└───────┴─────────┴────────────┴───────────┴────────┘

Work Patterns
  Peak Hours: 14:00, 15:00, 16:00
  Most Active Day: Wednesday
  Avg Commits/Day: 1.4
  Active Days: 30 / 31

Value Estimates
┌──────────────┬────────────┬─────────────┬─────────────┐
│ Level        ┆ Multiplier ┆ Hourly Rate ┆ Total Value │
╞══════════════╪════════════╪═════════════╪═════════════╡
│ Junior       ┆ 1x         ┆ ₩12,036     ┆ ₩1,058,155  │
│ Mid-level ⭐ ┆ 1.5x       ┆ ₩18,054     ┆ ₩1,587,232  │
│ Senior       ┆ 2x         ┆ ₩24,072     ┆ ₩2,116,310  │
│ Lead         ┆ 2.5x       ┆ ₩30,090     ┆ ₩2,645,387  │
│ Principal    ┆ 3x         ┆ ₩36,108     ┆ ₩3,174,465  │
└──────────────┴────────────┴─────────────┴─────────────┘
```

#### Time Estimation Algorithm

The tool uses a hybrid approach combining two estimation methods:

1. **Time-Gap Method (60% weight)**
   - Measures time between consecutive commits
   - Caps at 4 hours to avoid overnight/weekend gaps
   - Assumes continuous work within reasonable timeframes

2. **Code-Change Method (40% weight)**
   - Estimates based on lines added/deleted
   - Applies language complexity multipliers (same as code-cost)
   - Adjusts for file change complexity (more files = higher complexity)
   - Base rate: 20 lines of code per hour

3. **Final Estimate**
   - Weighted average of both methods
   - Provides balanced estimate considering both time and work volume
   - More accurate than either method alone

#### Use Cases

- **Freelancer invoicing**: Calculate billable hours from git history
- **Project retrospectives**: Understand team work patterns
- **Time tracking**: Estimate time spent on projects retroactively
- **Team analytics**: Analyze contributor activity and patterns
- **Work reports**: Generate professional summaries of development work
- **Portfolio documentation**: Showcase project effort and value

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
│   ├── code-cost/          # Code value analyzer tool
│   │   └── src/
│   │       ├── analyzer/   # Repository analysis logic
│   │       ├── calculator/ # Cost calculation
│   │       ├── metrics/    # Code metrics collection
│   │       └── git/        # Git repository analysis
│   └── work-summary/       # Git commit work summarizer
│       └── src/
│           ├── git/        # Commit history & diff analysis
│           ├── analyzer/   # Work analysis & value calculation
│           ├── patterns/   # Work pattern detection
│           └── summary/    # Summary generation
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
