# CLI Tools Collection

A collection of powerful Rust-based CLI tools for developers, designed for codebase analysis, value estimation, and work productivity tracking.

## Architecture

This project is built with a modular architecture:

- **`cli-core`**: A shared library providing common functionality for UI theming, output formatting (Table, JSON, CSV, HTML, Markdown), and configuration management.
- **`code-cost`**: Analyzes entire repositories to estimate their total monetary value.
- **`work-summary`**: Analyzes Git history to summarize recent work activity and productivity.
- **`dev-pulse`**: A collection of tools for developer productivity, including branch cleanup and code scanning.
- **`dev-utils`**: A "Swiss Army Knife" of small, frequently used developer utilities (UUID, Base64, JSON, Port, etc.).

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

---

### dev-pulse

Developer pulse check and convenience tools to keep your development environment healthy.

#### Features

- **Git Branch Cleanup**
  - Identifies branches that have already been merged into `main` or `master`.
  - Dry-run by default to prevent accidental deletions.
  - Delete multiple merged branches at once with `--force`.

- **Marker Scanning**
  - Scans the codebase for `TODO`, `FIXME`, `BUG`, `HACK`, and `OPTIMIZE` markers.
  - Automatically respects `.gitignore`.
  - Customizable marker list via command line.

- **Project Health Check**
  - Verifies the presence of essential project files:
    - `README.md` (Documentation)
    - `LICENSE` (Legal)
    - `.gitignore` (Git hygiene)
    - `.git` (Repository setup)
    - `tests/` (Test suite)
    - CI Configuration (Github Actions, etc.)
  - Provides a health score and detailed pass/fail report.

- **.env Validator**
  - Compares `.env` against `.env.example` to ensure all required environment variables are configured.
  - Lists missing keys and provides tips for configuration.

- **Changelog Generator**
  - Automatically generates a grouped changelog from Git history.
  - Recognizes conventional commit types (feat, fix, docs, etc.).
  - Supports range filtering (`--from`, `--to`) and commit limits.

- **Commit Wizard**
  - Interactive terminal UI for creating Conventional Commits.
  - Guides you through type selection, scoping, description, and breaking changes.
  - Automatically handles staged changes and creates the commit.

#### Installation

```bash
cargo install --path crates/dev-pulse
```

#### Usage

```bash
# Git Branch Cleanup
dev-pulse cleanup
dev-pulse cleanup --force
dev-pulse cleanup --target develop

# Marker Scanning
dev-pulse scan
dev-pulse scan --markers "TODO,DEBUG"

# Project Health Check
dev-pulse health
dev-pulse health --verbose

# .env Validation
dev-pulse env

# Changelog Generation
dev-pulse changelog
dev-pulse changelog --from v1.0.0 --limit 10

# Interactive Commit
dev-pulse commit
```

---

### dev-utils

A collection of small, frequently used utilities for developers to handle common data transformations and system checks.

#### Features

- **UUID Generation**: Generate single or multiple UUID v4/v7 strings.
- **Base64 Tool**: Quick encoding and decoding of strings/files to/from Base64.
- **URL Tool**: URL encoding and decoding for web development.
- **JSON Formatter**: Pretty-print or minify JSON strings with validation, query with JSONPath.
- **Model Generation**: Convert JSON to TypeScript interfaces, Rust structs, or Go structs.
- **Port Manager**: Check which processes are using a port and optionally kill them.
- **Hash Generator**: Generate SHA-256, MD5, SHA-1 hashes for strings or files.
- **Time Converter**: Convert between Unix timestamps and ISO8601 strings, or get current time.
- **Project Tree**: Visualize project structure while respecting `.gitignore`.
- **IP Info**: Quickly check your local and public IP addresses with location data.
- **Morse Code**: Encode and decode Morse code strings.
- **Password Tool**: Generate secure passwords and check their strength.

#### Installation

```bash
cargo install --path crates/dev-utils
```

#### Usage

```bash
# Generate UUIDs
dev-utils uuid --count 5 --v7

# Base64 Encoding/Decoding
dev-utils base64 "hello world"
dev-utils base64 --decode "aGVsbG8gd29ybGQ="

# JSON and Model Generation
dev-utils json '{"a":1,"b":2}'
dev-utils rust '{"name": "test", "age": 20}'
dev-utils go '{"name": "test", "age": 20}'
dev-utils typescript '{"name": "test", "age": 20}'

# Port Management
dev-utils port 8080
dev-utils port 8080 --kill

# File Hashing and Checksums
dev-utils hash README.md --file
dev-utils checksum README.md --file --algo sha512

# Time Conversion
dev-utils time
dev-utils time 1740000000

# Password Strength
dev-utils password --check "P@ssw0rd123"

# Morse Code
dev-utils morse "HELLO WORLD"
dev-utils morse --decode ".... . .-.. .-.. --- / .-- --- .-. .-.. -.."

# IP Information
dev-utils ip
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
│   ├── dev-pulse/          # Developer productivity tools
│   └── work-summary/       # Git work productivity summarizer
```

## License

MIT License - See [LICENSE](LICENSE) file for details.

## Author

CHANN
