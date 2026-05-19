# CLI Tools Improvement Track

This track focuses on enhancing the existing CLI tools with high-quality features and better integration.

## Tasks

- [x] Add `GEMINI.md` with project-specific instructions
- [x] Expand `work-summary` export formats (Markdown, CSV, HTML)
- [x] Improve `dev-tools tree` output formatting
- [x] Enhance `git-tools` health check with more industry standard checks
- [x] Improve `dev-tools sql` formatter quality

- [x] **Enhanced `dev-tools speedtest`**: Added latency measurement and upload test.
- [x] **Enhanced `dev-tools github`**: Added repository search functionality with tabular output.
- [x] **Enhanced `git-tools health`**: Added dependency lock file verification for deterministic builds.
- [x] **Enhanced `git-tools changelog`**: Added Markdown output format support and improved categorization.

## Recent High-Quality Improvements (May 2026)

- [x] **Enhanced `dev-tools tree`**: Added `--size` and `--git` flags, better icons, and Git status integration.
- [x] **Enhanced `dev-tools qr`**: Added `--output` (PNG/SVG support), `--level` (error correction), and `--size` for file output.
- [x] **Enhanced `git-tools health`**: Added actionable advice for all checks and new advanced checks (Large Files, Tracked Secrets).
- [x] **Enhanced `dev-tools sql`**: Integrated `sqlformat` crate for robust, high-quality SQL pretty-printing.
- [x] **Enhanced `dev-tools jwt`**: Added signature verification and human-readable timestamps.
- [x] **Enhanced `dev-tools cert`**: Added local file inspection and detailed X509 info.
- [x] **Enhanced `dev-tools sql`**: Improved formatting logic for complex queries and multi-word keywords.
- [x] **Enhanced `dev-tools color`**: Added HSL conversion and advanced palette generation.
- [x] **Enhanced `dev-tools password`**: Added passphrase (diceware) support, entropy calculation, and visual strength meter with actionable suggestions.
- [x] **Enhanced `dev-tools http-status`**: Added search functionality by name/description and high-quality tabular output.
- [x] **Enhanced `dev-tools text`**: Added line numbering, prefix/suffix support, and truncation options.
- [x] **Enhanced `dev-tools sql`**: Added `--indent`, `--tabs`, `--lowercase`, and `--file` options for flexible formatting.
- [x] **Enhanced `dev-tools url-parse`**: Improved output with tabular query parameters and security status indicators.
- [x] **Enhanced `dev-tools crates`**: Integrated `TableFormatter` and added unit-aware download formatting.
- [x] **Enhanced `dev-tools bench`**: Refactored results into a clean tabular format with failure tracking.
- [x] **Enhanced `dev-tools dns`**: Added structured tabular output for all supported record types.
- [x] **Enhanced `dev-tools github`**: Unified `get_user`, `get_repo`, and `search` outputs with `cli-core` components.
- [x] **Enhanced `dev-tools http-status`**: Improved layout and added search status themes.
- [x] **Enhanced `git-tools health`**: Added 8 new checks including Pre-commit Hooks, Vuln Scanning, Registry Config, Tests, Examples, and Crate Metadata.
- [x] **Enhanced `git-tools` stats**: Refactored language breakdown and summary into high-quality tables.
- [x] **Enhanced `dev-tools ip`**: Added support for specific IP/hostname lookups and high-quality tabular output with geolocation details.
- [x] **Enhanced `dev-tools hash`**: Added support for MD5, SHA1, SHA256, and SHA512 algorithms and improved comparison UI.
- [x] **Enhanced `dev-tools ascii-table`**: Refactored to use `TableFormatter` for a clean, consistent presentation.
- [x] **Enhanced `dev-tools url-parse`**: Unified component analysis and query parameter display with high-quality tables and themes.
- [x] **Enhanced `dev-tools cron`**: Added professional UI, expression breakdown, and countdown for upcoming runs.
- [x] **Enhanced `dev-tools mime`**: Upgraded to professional themed output with `cli-core` components.
- [x] **Enhanced `dev-tools case`**: Added "at-a-glance" table view showing all common case conversions by default.
- [x] **Enhanced `dev-tools chmod`**: Added visual permission breakdown table with Owner/Group/Others details.
- [x] **Enhanced `dev-tools user-agent`**: Integrated `woothee` for high-quality UA inspection and added detailed tabular analysis.
- [x] **Enhanced `dev-tools portscan`**: Parallelized scanning with `tokio` and added high-quality tabular output with service name mapping.
- [x] **Enhanced `dev-tools scan`**: Modernized with `Theme` and `TableFormatter` for professional audit reports.
- [x] **New `dev-tools check-links`**: Implemented a new command to detect and verify broken URLs in files/directories with parallel HEAD requests.
- [x] **Enhanced `dev-tools weather`**: Updated to use project `Theme` for UI consistency.
- [x] **Enhanced `dev-tools sys`**: Overhauled with a professional dashboard using `cli-core` components and filtered network statistics.
- [x] **Enhanced `dev-tools diff`**: Improved with a cleaner themed output, summary of changes, and icon-less color methods in `cli-core`.
- [x] **Enhanced `dev-tools color`**: Upgraded with visual color previews using background-colored blocks and professional palette tables.
- [x] **Enhanced `dev-tools extract`**: Added support for MAC addresses, phone numbers, dates, and credit cards with high-quality tabular output.
- [x] **Enhanced `dev-tools checksum`**: Added verification support with `--check` flag and detailed comparison results.
- [x] **Enhanced `dev-tools json-diff`**: Upgraded to a professional tabular comparison view with clear path-based tracking of additions, deletions, and changes.
- [x] **Enhanced `dev-tools sql`**: Added project-standard themed headers and formatting summary info for better feedback.

## Recent High-Quality Improvements (May 2026)

- [x] **Enhanced `dev-tools base64`**: Upgrade to professional themed output and add better diagnostic info.
- [x] **Enhanced `dev-tools csv`**: Integrate `TableFormatter` for high-quality terminal previews and better conversion logic.
- [x] **Enhanced `dev-tools checksum`**: Refactor to use `Theme` and unify with `hash` logic where appropriate.
- [x] **Enhanced `dev-tools binary`**: Improve conversion UI with a combined view of multiple bases.
- [x] **Enhanced `dev-tools uuid`**: Added `inspect` capability for all versions (including timestamp extraction for v7) and professional themed output.
- [x] **Enhanced `dev-tools ulid`**: Added `inspect` capability with timestamp breakdown and professional tabular output.
- [x] **Enhanced `dev-tools nanoid`**: Added support for custom alphabets and bulk generation with professional UI.
- [x] **Enhanced `dev-tools secret`**: Added support for bulk generation, new predefined kinds (flask, express, url-safe), and themed output.
- [x] **Enhanced `dev-tools totp`**: Added secure secret generation capability and improved output with code validity countdown.
- [x] **Enhanced `dev-tools bcrypt`**: Added professional UI with `Theme` and `TableFormatter`, structured verification results, and cost factor extraction.
- [x] **Enhanced `dev-tools ksuid`**: Added detailed inspection with human-readable timestamps and hexadecimal payload breakdown.
- [x] **Enhanced `dev-tools snowflake`**: Implemented detailed decomposition (timestamp, node, sequence) and added Base62 support.
- [x] **Enhanced `dev-tools base32/58/85`**: Standardized with `base64` quality, adding `--file` and `--output` support and professional themed output.
- [x] **Enhanced `dev-tools hmac`**: Added support for multiple algorithms (MD5, SHA1, SHA256, SHA512) and professional UI.
- [x] **Enhanced `dev-tools env`**: Overhauled with `TableFormatter` for a clean, searchable, and sortable environment variable dashboard.
- [x] **Enhanced `dev-tools punycode`**: Upgraded to professional themed output.
- [x] **Enhanced `dev-tools joke`**: Expanded programmer joke collection and improved presentation with `Theme`.
- [x] **Enhanced `dev-tools lorem`**: Added support for words, sentences, and paragraphs with professional UI.
- [x] **Enhanced `dev-tools random`**: Added bulk generation and support for numbers/booleans with tabular output.
- [x] **Enhanced `dev-tools path`**: Overhauled with detailed analysis (absolute, parent, metadata) in a high-quality table.
- [x] **Enhanced `dev-tools stat`**: Overhauled with detailed textual metrics for strings and comprehensive metadata for files.
- [x] **Enhanced `dev-tools unit`**: Expanded to support length, weight, and temperature conversions with tabular output.
- [x] **Enhanced `dev-tools currency`**: Integrated `frankfurter.app` API for real-time exchange rate conversions.
- [x] **Enhanced `dev-tools hexview`**: Upgraded to a professional colored hex dump with offset and ASCII representation.
- [x] **Enhanced `dev-tools dictionary`**: Upgraded to professional themed output using `Theme` and `TableFormatter`.
- [x] **Enhanced `dev-tools morse`**: Upgraded to professional themed output using `Theme` and added headers.
- [x] **Enhanced `dev-tools shorten`**: Upgraded to professional themed output using `Theme` and added headers.
- [x] **Enhanced `dev-tools url`**: Upgraded to professional themed output using `Theme` and added headers.
- [x] **Enhanced `dev-tools json`**: Added professional headers for YAML, TOML, and Schema outputs using `Theme`.
- [x] **Enhanced `git-tools health`**: Added a new "Pending Tasks" check for TODO/FIXME comments.

