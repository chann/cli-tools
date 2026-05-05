# CLI Tools Improvement Track

This track focuses on enhancing the existing CLI tools with high-quality features and better integration.

## Tasks

- [x] Add `GEMINI.md` with project-specific instructions
- [x] Expand `work-summary` export formats (Markdown, CSV, HTML)
- [x] Improve `dev-utils tree` output formatting
- [x] Enhance `dev-pulse` health check with more industry standard checks
- [x] Improve `dev-utils sql` formatter quality

- [x] **Enhanced `dev-utils speedtest`**: Added latency measurement and upload test.
- [x] **Enhanced `dev-utils github`**: Added repository search functionality with tabular output.
- [x] **Enhanced `dev-pulse health`**: Added dependency lock file verification for deterministic builds.
- [x] **Enhanced `dev-pulse changelog`**: Added Markdown output format support and improved categorization.

## Recent High-Quality Improvements (May 2026)

- [x] **Enhanced `dev-utils tree`**: Added `--size` and `--git` flags, better icons, and Git status integration.
- [x] **Enhanced `dev-utils qr`**: Added `--output` (PNG/SVG support), `--level` (error correction), and `--size` for file output.
- [x] **Enhanced `dev-pulse health`**: Added actionable advice for all checks and new advanced checks (Large Files, Tracked Secrets).
- [x] **Enhanced `dev-utils sql`**: Integrated `sqlformat` crate for robust, high-quality SQL pretty-printing.
- [x] **Enhanced `dev-utils jwt`**: Added signature verification and human-readable timestamps.
- [x] **Enhanced `dev-utils cert`**: Added local file inspection and detailed X509 info.
- [x] **Enhanced `dev-utils sql`**: Improved formatting logic for complex queries and multi-word keywords.
- [x] **Enhanced `dev-utils color`**: Added HSL conversion and advanced palette generation.
- [x] **Enhanced `dev-utils password`**: Added passphrase (diceware) support, entropy calculation, and visual strength meter with actionable suggestions.
- [x] **Enhanced `dev-utils http-status`**: Added search functionality by name/description and high-quality tabular output.
- [x] **Enhanced `dev-utils text`**: Added line numbering, prefix/suffix support, and truncation options.
- [x] **Enhanced `dev-utils sql`**: Added `--indent`, `--tabs`, `--lowercase`, and `--file` options for flexible formatting.
- [x] **Enhanced `dev-utils url-parse`**: Improved output with tabular query parameters and security status indicators.
- [x] **Enhanced `dev-utils crates`**: Integrated `TableFormatter` and added unit-aware download formatting.
- [x] **Enhanced `dev-utils bench`**: Refactored results into a clean tabular format with failure tracking.
- [x] **Enhanced `dev-utils dns`**: Added structured tabular output for all supported record types.
- [x] **Enhanced `dev-utils github`**: Unified `get_user`, `get_repo`, and `search` outputs with `cli-core` components.
- [x] **Enhanced `dev-utils http-status`**: Improved layout and added search status themes.
- [x] **Enhanced `dev-pulse health`**: Added 8 new checks including Pre-commit Hooks, Vuln Scanning, Registry Config, Tests, Examples, and Crate Metadata.
- [x] **Enhanced `dev-pulse` stats**: Refactored language breakdown and summary into high-quality tables.
- [x] **Enhanced `dev-utils ip`**: Added support for specific IP/hostname lookups and high-quality tabular output with geolocation details.
- [x] **Enhanced `dev-utils hash`**: Added support for MD5, SHA1, SHA256, and SHA512 algorithms and improved comparison UI.
- [x] **Enhanced `dev-utils ascii-table`**: Refactored to use `TableFormatter` for a clean, consistent presentation.
- [x] **Enhanced `dev-utils url-parse`**: Unified component analysis and query parameter display with high-quality tables and themes.
- [x] **Enhanced `dev-utils cron`**: Added professional UI, expression breakdown, and countdown for upcoming runs.
- [x] **Enhanced `dev-utils mime`**: Upgraded to professional themed output with `cli-core` components.
- [x] **Enhanced `dev-utils case`**: Added "at-a-glance" table view showing all common case conversions by default.
- [x] **Enhanced `dev-utils chmod`**: Added visual permission breakdown table with Owner/Group/Others details.
- [x] **Enhanced `dev-utils user-agent`**: Integrated `woothee` for high-quality UA inspection and added detailed tabular analysis.
- [x] **Enhanced `dev-utils portscan`**: Parallelized scanning with `tokio` and added high-quality tabular output with service name mapping.
- [x] **Enhanced `dev-utils scan`**: Modernized with `Theme` and `TableFormatter` for professional audit reports.
- [x] **New `dev-utils check-links`**: Implemented a new command to detect and verify broken URLs in files/directories with parallel HEAD requests.
- [x] **Enhanced `dev-utils weather`**: Updated to use project `Theme` for UI consistency.
- [x] **Enhanced `dev-utils sys`**: Overhauled with a professional dashboard using `cli-core` components and filtered network statistics.
- [x] **Enhanced `dev-utils diff`**: Improved with a cleaner themed output, summary of changes, and icon-less color methods in `cli-core`.
- [x] **Enhanced `dev-utils color`**: Upgraded with visual color previews using background-colored blocks and professional palette tables.
- [x] **Enhanced `dev-utils extract`**: Added support for MAC addresses, phone numbers, dates, and credit cards with high-quality tabular output.
- [x] **Enhanced `dev-utils checksum`**: Added verification support with `--check` flag and detailed comparison results.
- [x] **Enhanced `dev-utils json-diff`**: Upgraded to a professional tabular comparison view with clear path-based tracking of additions, deletions, and changes.
- [x] **Enhanced `dev-utils sql`**: Added project-standard themed headers and formatting summary info for better feedback.

## Recent High-Quality Improvements (May 2026)

- [x] **Enhanced `dev-utils base64`**: Upgrade to professional themed output and add better diagnostic info.
- [x] **Enhanced `dev-utils csv`**: Integrate `TableFormatter` for high-quality terminal previews and better conversion logic.
- [x] **Enhanced `dev-utils checksum`**: Refactor to use `Theme` and unify with `hash` logic where appropriate.
- [x] **Enhanced `dev-utils binary`**: Improve conversion UI with a combined view of multiple bases.
- [x] **Enhanced `dev-utils uuid`**: Added `inspect` capability for all versions (including timestamp extraction for v7) and professional themed output.
- [x] **Enhanced `dev-utils ulid`**: Added `inspect` capability with timestamp breakdown and professional tabular output.
- [x] **Enhanced `dev-utils nanoid`**: Added support for custom alphabets and bulk generation with professional UI.
- [x] **Enhanced `dev-utils secret`**: Added support for bulk generation, new predefined kinds (flask, express, url-safe), and themed output.
- [x] **Enhanced `dev-utils totp`**: Added secure secret generation capability and improved output with code validity countdown.
- [x] **Enhanced `dev-utils bcrypt`**: Added professional UI with `Theme` and `TableFormatter`, structured verification results, and cost factor extraction.
- [x] **Enhanced `dev-utils ksuid`**: Added detailed inspection with human-readable timestamps and hexadecimal payload breakdown.
- [x] **Enhanced `dev-utils snowflake`**: Implemented detailed decomposition (timestamp, node, sequence) and added Base62 support.
- [x] **Enhanced `dev-utils base32/58/85`**: Standardized with `base64` quality, adding `--file` and `--output` support and professional themed output.
- [x] **Enhanced `dev-utils hmac`**: Added support for multiple algorithms (MD5, SHA1, SHA256, SHA512) and professional UI.
- [x] **Enhanced `dev-utils env`**: Overhauled with `TableFormatter` for a clean, searchable, and sortable environment variable dashboard.
- [x] **Enhanced `dev-utils punycode`**: Upgraded to professional themed output.
- [x] **Enhanced `dev-utils joke`**: Expanded programmer joke collection and improved presentation with `Theme`.
- [x] **Enhanced `dev-utils lorem`**: Added support for words, sentences, and paragraphs with professional UI.
- [x] **Enhanced `dev-utils random`**: Added bulk generation and support for numbers/booleans with tabular output.
- [x] **Enhanced `dev-utils path`**: Overhauled with detailed analysis (absolute, parent, metadata) in a high-quality table.
- [x] **Enhanced `dev-utils stat`**: Overhauled with detailed textual metrics for strings and comprehensive metadata for files.
- [x] **Enhanced `dev-utils unit`**: Expanded to support length, weight, and temperature conversions with tabular output.
- [x] **Enhanced `dev-utils currency`**: Integrated `frankfurter.app` API for real-time exchange rate conversions.
- [x] **Enhanced `dev-utils hexview`**: Upgraded to a professional colored hex dump with offset and ASCII representation.
- [x] **Enhanced `dev-utils dictionary`**: Upgraded to professional themed output using `Theme` and `TableFormatter`.
- [x] **Enhanced `dev-utils morse`**: Upgraded to professional themed output using `Theme` and added headers.
- [x] **Enhanced `dev-utils shorten`**: Upgraded to professional themed output using `Theme` and added headers.
- [x] **Enhanced `dev-utils url`**: Upgraded to professional themed output using `Theme` and added headers.
- [x] **Enhanced `dev-utils json`**: Added professional headers for YAML, TOML, and Schema outputs using `Theme`.
- [x] **Enhanced `dev-pulse health`**: Added a new "Pending Tasks" check for TODO/FIXME comments.

