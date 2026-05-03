# CLI Tools Collection

A collection of powerful Rust-based CLI tools for developers, designed for codebase analysis, value estimation, and work productivity tracking.

## Architecture

This project is built with a modular architecture:

- **`cli-core`**: A shared library providing common functionality for UI theming, output formatting (Table, JSON, CSV, HTML, Markdown), and configuration management.
- **`code-cost`**: Analyzes entire repositories to estimate their total monetary value.
- **`work-summary`**: Analyzes Git history to summarize recent work activity and productivity.

## Tools

### code-cost

Analyze code repositories and calculate their monetary value based on development effort, complexity, and project maturity.

#### Features

- **Comprehensive Code Analysis**
  - Lines of code (LOC) with breakdown (code, comments, blanks)
  - Multiple programming languages with weighted difficulty scores
  - Cyclomatic complexity estimation based on LOC and language factors
  - Project maturity scoring (tests, documentation, repository age, contributor count)

- **Git Repository Analysis**
  - Commit count and history
  - Contributor analysis
  - Repository age tracking

- **Monetary Value Calculation**
  - Estimated development hours
  - Customizable hourly rate (default: ₩10,030 - 2025 South Korea minimum wage)
  - Language difficulty multipliers (Rust: 1.5x, C++: 1.4x, Go: 1.3x, etc.)
  - Complexity and maturity bonuses
  - Learning time estimation for technologies used
  - **Token-based Cost Estimation**: Calculates cost based on **Claude Opus 4.7 xhigh** pricing ($5/1M input tokens)

- **Advanced Analysis Features**
  - Detailed project metrics (complexity, maturity, code quality)
  - Language breakdown with percentages
  - **AI Usage Estimation**: Analyzes patterns to estimate AI-assisted development
  - Developer level cost breakdown (Junior to Principal)
  - Test coverage statistics

- **Multiple Output Formats**
  - Beautiful colored terminal UI
  - Detailed analysis mode (default)
  - Simple mode (`--simple`) for basic summary
  - JSON (`json`) and Pretty-printed JSON (`json-pretty`)
  - Export to **CSV**, **HTML**, and **Markdown**

#### Installation

```bash
cargo install --path crates/code-cost
```

#### Usage

```bash
# Basic analysis (current directory)
code-cost

# Analyze specific paths
code-cost ~/projects/my-app ../other-repo

# Simple mode (table only)
code-cost --simple

# Show developer level breakdown
code-cost --dev-levels

# JSON Output
code-cost --format json-pretty

# Export Results
code-cost --export report.html
code-cost --export report.md
code-cost --export report.csv
```

---

### work-summary

Analyze git commit history and generate meaningful work activity summaries with time estimation and value calculation.

#### Features

- **Git Commit Analysis**
  - Detailed commit history with diff tracking
  - File change statistics per commit
  - Author and timestamp information
  - Language-specific change tracking within commits

- **Hybrid Time Estimation**
  - **Time-gap based**: Measures intervals between commits (capped at 4 hours)
  - **Code-change based**: Estimates effort from lines added/deleted and complexity
  - Weighted hybrid algorithm for high accuracy

- **Work Pattern Analysis**
  - Hourly commit distribution (Peak hours)
  - Daily activity tracking (Most active days)
  - Commit frequency and active day ratios

- **Value Calculation**
  - Developer level-based estimates (Junior to Principal)
  - Base hourly rate: ₩10,030 (2025 KRW minimum wage)
  - Complexity-adjusted value estimation based on commit size

- **Contributor Statistics**
  - Per-contributor commit counts and line stats
  - Contribution percentage breakdown
  - Top contributors ranking

- **Flexible Filtering**
  - Date ranges: `--from`, `--to` (YYYY-MM-DD)
  - Quick filters: `--today`, `--week`, `--month`
  - Limit: `--limit N` most recent commits

- **Output Options**
  - Detail mode: Comprehensive analysis (default)
  - Simple mode: Basic summary summary (`--simple`)
  - JSON export support

#### Installation

```bash
cargo install --path crates/work-summary
```

#### Usage

```bash
# Analyze last 30 days (default)
work-summary

# Quick filters
work-summary --today
work-summary --week
work-summary --month

# Specific date range
work-summary --from 2025-01-01 --to 2025-01-31

# Limit commits
work-summary --limit 20

# Simple mode
work-summary --simple

# Export to JSON
work-summary --export summary.json
```

## Value Calculation Algorithms

### Code Cost Algorithm

1. **Base Hours**: `LOC / 20` (assumes average 20 lines/hour).
2. **Language Weight**: Multiplier based on language complexity (e.g., Rust 1.5x, JS 1.0x).
3. **Complexity Multiplier**: Maps project metrics to 1.0x - 2.0x range.
4. **Maturity Bonus**: Up to 30% bonus for projects with good tests, docs, and history.
5. **Learning Time**: Estimated time required to master the project's tech stack.

### Work Summary Algorithm (Hybrid)

1. **Time-Gap (60%)**: Measures real-time elapsed between commits, capping long gaps.
2. **Code-Change (40%)**: Estimates effort from the volume and complexity of changes.
3. **Complexity Factor**: Multiplier (0.8x - 1.4x) based on the number of files and total lines changed.

### Token-based Cost Algorithm (Claude Opus 4.7 xhigh)

1. **Token Approximation**: `characters / 3.5` (heuristic for code).
2. **Inflation Factor**: `1.35x` (specific to Opus 4.7 xhigh tokenizer and reasoning effort).
3. **Pricing**: `$5.00 / 1M tokens` (Input).
4. **Exchange Rate**: Fixed at `1,400 KRW/USD` for local cost estimation.

## Project Structure

```
cli-tools/
├── crates/
│   ├── cli-core/           # Shared foundation (UI, I/O, Formatting)
│   ├── code-cost/          # Repository value analyzer
│   └── work-summary/       # Git work productivity summarizer
```

## License

MIT License - See [LICENSE](LICENSE) file for details.

## Author

CHANN
