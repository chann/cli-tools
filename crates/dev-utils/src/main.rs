mod commands;

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "dev-utils",
    version,
    about = "Common developer utility tools",
    long_about = "A collection of small, frequently used utilities for developers. \
                  Includes UUID generation, Base64 encoding, URL encoding, and more."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Generate shell completion scripts
    Completion {
        /// Shell to generate completion for (bash, zsh, fish, powershell, elvish)
        #[arg(short, long, default_value = "bash")]
        shell: String,
    },
    /// Generate ASCII art from text
    Ascii {
        /// Text to convert
        text: String,
        /// Optional path to a FIGlet font file (.flf)
        #[arg(short, long)]
        font: Option<String>,
    },
    /// Start a simple HTTP static file server
    Serve {
        /// Directory to serve
        #[arg(default_value = ".")]
        path: String,
        /// Port to listen on
        #[arg(short, long, default_value = "8080")]
        port: u16,
    },
    /// Generate UUIDs
    Uuid {
        /// Number of UUIDs to generate
        #[arg(short, long, default_value = "1")]
        count: usize,
        /// Generate UUID v7 instead of v4
        #[arg(long)]
        v7: bool,
    },
    /// Base64 encoding/decoding
    Base64 {
        /// Text or file path to process
        input: String,
        /// Decode instead of encode
        #[arg(short, long)]
        decode: bool,
        /// Treat input as a file path
        #[arg(short, long)]
        file: bool,
        /// Output file path (for decoding binary data)
        #[arg(short, long)]
        output: Option<String>,
        /// Generate data URI (e.g., data:image/png;base64,...)
        #[arg(long)]
        data_uri: bool,
    },
    /// URL encoding/decoding
    Url {
        /// Text to process
        text: String,
        /// Decode instead of encode
        #[arg(short, long)]
        decode: bool,
    },
    /// JSON formatting and querying
    Json {
        /// JSON text to process
        text: String,
        /// Minify instead of pretty-print
        #[arg(short, long)]
        minify: bool,
        /// JSON Path query (e.g., "$.store.book[0].title")
        #[arg(short, long)]
        query: Option<String>,
        /// Convert to YAML
        #[arg(long)]
        yaml: bool,
        /// Convert to TOML
        #[arg(long)]
        toml: bool,
        /// Convert to CSV
        #[arg(long)]
        csv: bool,
        /// Generate JSON Schema
        #[arg(long)]
        schema: bool,
    },
    /// Port utilities
    Port {
        /// Port number
        port: Option<u16>,
        /// Kill the process on this port
        #[arg(short, long)]
        kill: bool,
        /// Wait for the port to become active
        #[arg(short, long)]
        wait: bool,
        /// Timeout in seconds for wait (default: 30)
        #[arg(short, long, default_value = "30")]
        timeout: u64,
        /// List all listening ports
        #[arg(short, long)]
        list: bool,
    },
    /// Hash utilities (SHA-256)
    Hash {
        /// Text or file path to hash
        input: String,
        /// Treat input as a file path
        #[arg(short, long)]
        file: bool,
        /// Compare with another hash or file
        #[arg(short, long)]
        compare: Option<String>,
    },
    /// Time utilities
    Time {
        /// Input (unix timestamp or ISO8601 string). If empty, shows current time.
        input: Option<String>,
    },
    /// Visualize project structure
    Tree {
        /// Target directory
        #[arg(default_value = ".")]
        path: PathBuf,
        /// Maximum depth
        #[arg(short, long)]
        depth: Option<usize>,
    },
    /// Show IP information
    Ip,
    /// Generate random strings
    Random {
        /// Length of the string
        #[arg(short, long, default_value = "16")]
        length: usize,
        /// Include numbers
        #[arg(short, long)]
        numeric: bool,
        /// Include symbols
        #[arg(short, long)]
        symbols: bool,
    },
    /// String case conversion
    Case {
        /// Text to convert
        text: String,
        /// Target case (snake, camel, pascal, kebab, shouty, train)
        #[arg(short, long)]
        to: String,
    },
    /// Inspect or generate JWT token
    Jwt {
        /// JWT token string (for inspection)
        token: Option<String>,
        /// Generate a new HS256 token
        #[arg(short, long)]
        sign: bool,
        /// Payload for generation (JSON string)
        #[arg(short, long)]
        payload: Option<String>,
        /// Secret key for HS256
        #[arg(short, long)]
        secret: Option<String>,
    },
    /// List or search environment variables
    Env {
        /// Filter by name
        filter: Option<String>,
    },
    /// Show system information
    Sys,
    /// Color conversion (Hex <-> RGB)
    Color {
        /// Color value (#RRGGBB or R,G,B)
        input: String,
    },
    /// Data size conversion
    Size {
        /// Value to convert
        value: f64,
        /// Source unit (B, KB, MB, GB, TB)
        #[arg(short, long, default_value = "MB")]
        unit: String,
    },
    /// Generate Lorem Ipsum text
    Lorem {
        /// Number of paragraphs to generate
        #[arg(short, long, default_value = "1")]
        paragraphs: usize,
    },
    /// Generate a secure password or check strength
    Password {
        /// Length of the password to generate
        #[arg(short, long, default_value = "16")]
        length: usize,
        /// Disable numbers
        #[arg(long)]
        no_numbers: bool,
        /// Disable symbols
        #[arg(long)]
        no_symbols: bool,
        /// Disable uppercase letters
        #[arg(long)]
        no_uppercase: bool,
        /// Disable lowercase letters
        #[arg(long)]
        no_lowercase: bool,
        /// Check the strength of a password
        #[arg(short, long)]
        check: Option<String>,
    },
    /// Make an HTTP request
    Http {
        /// HTTP method (GET, POST, etc.)
        #[arg(short, long, default_value = "GET")]
        method: String,
        /// URL to request
        url: String,
        /// Request body
        #[arg(short, long)]
        body: Option<String>,
        /// Request headers (Key: Value)
        #[arg(short, long)]
        header: Vec<String>,
    },
    /// Generate a QR code in the terminal
    Qr {
        /// Text or URL to encode
        text: String,
    },
    /// Path utilities
    Path {
        /// Path to process
        path: String,
        /// Resolve to absolute path (canonicalize)
        #[arg(short, long)]
        resolve: bool,
    },
    /// YAML formatting and conversion
    Yaml {
        /// YAML text to process
        text: String,
        /// Convert to JSON
        #[arg(short, long)]
        json: bool,
        /// Convert to TOML
        #[arg(short, long)]
        toml: bool,
    },
    /// TOML formatting and conversion
    Toml {
        /// TOML text to process
        text: String,
        /// Convert to JSON
        #[arg(short, long)]
        json: bool,
        /// Convert to YAML
        #[arg(short, long)]
        yaml: bool,
    },
    /// Common text operations
    Text {
        /// Text to process
        text: String,
        /// Operation (sort, reverse, unique, trim, upper, lower, replace)
        #[arg(short, long, default_value = "trim")]
        op: String,
        /// Sort in reverse order
        #[arg(long)]
        reverse: bool,
        /// Remove duplicate lines
        #[arg(long)]
        unique: bool,
        /// Text to replace
        #[arg(long)]
        from: Option<String>,
        /// Replacement text
        #[arg(long)]
        to: Option<String>,
    },
    /// Explain a cron expression in plain language
    Cron {
        /// Cron expression (e.g., "0 0 * * *")
        expression: String,
    },
    /// Inspect Unicode characters in a string
    Unicode {
        /// Text to inspect
        text: String,
    },
    /// Show differences between two strings or files
    Diff {
        /// Original text or file path
        old: String,
        /// New text or file path
        new: String,
        /// Treat inputs as file paths
        #[arg(short, long)]
        file: bool,
    },
    /// Test regex matches
    Regex {
        /// Regex pattern
        pattern: String,
        /// Text to test against
        text: String,
    },
    /// Escape/unescape utilities
    Escape {
        /// Text to process
        text: String,
        /// Type of escaping (html, string)
        #[arg(short, long, default_value = "html")]
        kind: String,
        /// Unescape instead of escape
        #[arg(short, long)]
        unescape: bool,
    },
    /// Number base conversion
    Base {
        /// Value to convert
        value: String,
        /// Source base (2, 8, 10, 16)
        #[arg(short, long, default_value = "10")]
        from: u32,
        /// Target base (2, 8, 10, 16)
        #[arg(short, long, default_value = "16")]
        to: u32,
    },
    /// Calculate checksums
    Checksum {
        /// Text or file path to hash
        input: String,
        /// Algorithm (md5, sha1, sha256, sha512)
        #[arg(short, long, default_value = "sha256")]
        algo: String,
        /// Treat input as a file path
        #[arg(short, long)]
        file: bool,
    },
    /// Show a hex dump of a string or file
    Hexview {
        /// Text or file path to dump
        input: String,
        /// Treat input as a file path
        #[arg(short, long)]
        file: bool,
    },
    /// Perform a DNS lookup
    Dns {
        /// Domain name to lookup
        domain: String,
        /// Record type (A, AAAA, MX, TXT, CNAME, NS)
        #[arg(short, long, default_value = "a")]
        record: String,
    },
    /// Gitignore utilities
    Gitignore {
        /// List available templates
        #[arg(short, long)]
        list: bool,
        /// Templates to generate (e.g., rust, macos)
        targets: Vec<String>,
    },
    /// License generator
    License {
        /// List supported licenses
        #[arg(short, long)]
        list: bool,
        /// License type (e.g., mit, apache-2.0)
        #[arg(short, long)]
        kind: Option<String>,
        /// Year for the license (defaults to current year)
        #[arg(short, long)]
        year: Option<i32>,
        /// License holder name
        #[arg(short, long, default_value = "CHANN")]
        holder: String,
    },
    /// Text statistics
    Stat {
        /// Input text or file path
        input: String,
        /// Treat input as a file path
        #[arg(short, long)]
        file: bool,
    },
    /// Domain WHOIS information
    Whois {
        /// Domain name
        domain: String,
    },
    /// Inspect SSL/TLS certificate
    Cert {
        /// Hostname (e.g., google.com)
        host: String,
        /// Port (default: 443)
        #[arg(short, long, default_value = "443")]
        port: u16,
    },
    /// Unit conversion (Temperature, Length, Weight)
    Unit {
        /// Value to convert
        value: f64,
        /// Source unit
        from: String,
        /// Target unit
        to: String,
    },
    /// Currency conversion
    Currency {
        /// Amount to convert
        amount: f64,
        /// Source currency (e.g., USD)
        from: String,
        /// Target currency (e.g., EUR)
        to: String,
    },
    /// Generate ULIDs
    Ulid {
        /// Number of ULIDs to generate
        #[arg(short, long, default_value = "1")]
        count: usize,
    },
    /// Generate NanoIDs
    Nanoid {
        /// Length of the ID
        #[arg(short, long, default_value = "21")]
        length: usize,
    },
    /// Generate a TOTP code
    Totp {
        /// Secret key (Base32 encoded)
        secret: String,
        /// Number of digits (default: 6)
        #[arg(short, long, default_value = "6")]
        digits: usize,
        /// Skew in seconds (default: 0)
        #[arg(short, long, default_value = "0")]
        skew: u8,
    },
    /// Convert CSV to JSON/YAML/Markdown
    Csv {
        /// Input file path
        input: String,
        /// Convert to JSON
        #[arg(long)]
        json: bool,
        /// Convert to YAML
        #[arg(long)]
        yaml: bool,
        /// Convert to Markdown table
        #[arg(long)]
        markdown: bool,
    },
    /// Shorten a URL
    Shorten {
        /// URL to shorten
        url: String,
    },
    /// Extract data from text or file
    Extract {
        /// Text or file path to process
        input: String,
        /// Type of data to extract (email, url, ip, ipv6)
        #[arg(short, long, default_value = "email")]
        kind: String,
        /// Treat input as a file path
        #[arg(short, long)]
        file: bool,
    },
    /// Basic SQL formatter
    Sql {
        /// SQL query to format
        query: String,
    },
    /// Generate a URL-friendly slug
    Slug {
        /// Text to convert
        text: String,
    },
    /// Chmod numeric/symbolic calculator
    Chmod {
        /// Value to convert (e.g., 755 or rwxr-xr-x)
        input: String,
    },
    /// Base32 encoding/decoding
    Base32 {
        /// Text to process
        input: String,
        /// Decode instead of encode
        #[arg(short, long)]
        decode: bool,
    },
    /// Base58 encoding/decoding
    Base58 {
        /// Text to process
        input: String,
        /// Decode instead of encode
        #[arg(short, long)]
        decode: bool,
    },
    /// Base85 encoding/decoding
    Base85 {
        /// Text to process
        input: String,
        /// Decode instead of encode
        #[arg(short, long)]
        decode: bool,
    },
    /// Punycode encoding/decoding
    Punycode {
        /// Text to process
        input: String,
        /// Decode instead of encode
        #[arg(short, long)]
        decode: bool,
    },
    /// HMAC utilities (SHA-256)
    Hmac {
        /// Text to process
        text: String,
        /// Secret key
        #[arg(short, long)]
        key: String,
    },
    /// Binary string conversion
    Binary {
        /// Text or binary string to process
        input: String,
        /// Convert from binary instead of to binary
        #[arg(short, long)]
        from: bool,
    },
    /// Generate a random user agent string
    UserAgent,
    /// Mime type utilities
    Mime {
        /// File path or extension
        input: String,
        /// Treat input as an extension (e.g., "json")
        #[arg(short, long)]
        extension: bool,
    },
    /// Tell a random programmer joke
    Joke,
    /// Generate a secure secret key for web frameworks
    Secret {
        /// Length of the secret key
        #[arg(short, long, default_value = "50")]
        length: usize,
        /// Kind of secret key (default, django, rails, hex, alphanumeric)
        #[arg(short, long, default_value = "default")]
        kind: String,
    },
    /// Generate or inspect Snowflake IDs
    Snowflake {
        /// ID to inspect
        #[arg(short, long)]
        inspect: Option<i64>,
        /// Number of IDs to generate
        #[arg(short, long, default_value = "1")]
        count: usize,
    },
    /// Semantic Versioning utilities
    Semver {
        /// Version string to process
        text: String,
        /// Operation (parse, increment, compare)
        #[arg(short, long, default_value = "parse")]
        op: String,
        /// Part to increment (major, minor, patch)
        #[arg(short, long)]
        increment: Option<String>,
        /// Version to compare against
        #[arg(short, long)]
        compare: Option<String>,
    },
    /// Generate or inspect KSUIDs
    Ksuid {
        /// ID to inspect
        #[arg(short, long)]
        inspect: Option<String>,
        /// Number of IDs to generate
        #[arg(short, long, default_value = "1")]
        count: usize,
    },
    /// XML formatting
    Xml {
        /// XML text to process
        text: String,
        /// Minify instead of pretty-print
        #[arg(short, long)]
        minify: bool,
    },
    /// Check website availability and response time
    Uptime {
        /// URLs to check
        urls: Vec<String>,
    },
    /// Scan remote ports
    Portscan {
        /// Host to scan (e.g., 127.0.0.1 or google.com)
        host: String,
        /// Start port
        #[arg(short, long, default_value = "1")]
        start: u16,
        /// End port
        #[arg(short, long, default_value = "1024")]
        end: u16,
        /// Timeout per port in milliseconds
        #[arg(short, long, default_value = "100")]
        timeout: u64,
    },
    /// Environment file (.env) utilities
    Dotenv {
        /// Operation (init, example, load, compare)
        #[arg(short, long, default_value = "load")]
        op: String,
        /// File path (default: .env)
        #[arg(short, long, default_value = ".env")]
        path: String,
        /// Example file path for compare (default: .env.example)
        #[arg(short, long, default_value = ".env.example")]
        example: String,
    },
    /// Mask sensitive data in text
    Mask {
        /// Text to mask
        text: String,
        /// Type of data to mask (email, phone, card, ip, all)
        #[arg(short, long, default_value = "all")]
        kind: String,
    },
    /// Generate fake data
    Fake {
        /// Type of data to generate (name, email, address, company, etc.)
        kind: String,
        /// Number of items to generate
        #[arg(short, long, default_value = "1")]
        count: usize,
        /// Generate Korean data
        #[arg(long)]
        ko: bool,
    },
    /// MAC address utilities
    Mac {
        /// Show local MAC address
        #[arg(short, long)]
        local: bool,
        /// Number of random MAC addresses to generate
        #[arg(short, long, default_value = "1")]
        count: usize,
    },
    /// Bcrypt password hashing
    Bcrypt {
        /// Password to hash or verify
        password: String,
        /// Hash to verify against
        #[arg(short = 'H', long)]
        hash: Option<String>,
        /// Cost factor (4-31)
        #[arg(short, long)]
        cost: Option<u32>,
    },
    /// Gzip compression/decompression
    Compress {
        /// Input file path
        input: String,
        /// Output file path
        #[arg(short, long)]
        output: String,
        /// Decompress instead of compress
        #[arg(short, long)]
        decompress: bool,
    },
    /// Lookup HTTP status codes
    HttpStatus {
        /// Status code to lookup
        code: Option<u16>,
        /// List all common status codes
        #[arg(short, long)]
        list: bool,
    },
    /// Display ASCII table
    AsciiTable,
    /// Parse a URL into its components
    UrlParse {
        /// URL to parse
        url: String,
    },
    /// GitHub utilities
    Github {
        /// Open a specific issue number
        #[arg(short, long)]
        issue: Option<u32>,
        /// Open a specific pull request number
        #[arg(short, long)]
        pr: Option<u32>,
        /// Lookup user information
        #[arg(short, long)]
        user: Option<String>,
        /// Lookup repository information (owner/repo)
        #[arg(short, long)]
        repo: Option<String>,
    },
    /// Search for Rust crates on crates.io
    Crates {
        /// Search query
        query: String,
    },
    /// Convert JSON to TypeScript interfaces
    Typescript {
        /// JSON text to convert
        text: String,
        /// Name of the interface (default: Root)
        #[arg(short, long, default_value = "Root")]
        name: String,
    },
    /// Show weather information
    Weather {
        /// Optional location (e.g., "Seoul", "London")
        location: Option<String>,
    },
    /// Lookup word definitions
    Dictionary {
        /// Word to lookup
        word: String,
    },
    /// Translate text
    Translate {
        /// Text to translate
        text: String,
        /// Target language code (e.g., "ko", "en", "ja")
        #[arg(short, long, default_value = "ko")]
        to: String,
    },
    /// Benchmark a command
    Bench {
        /// Command to benchmark
        command: String,
        /// Arguments for the command
        #[arg(short, long)]
        args: Vec<String>,
        /// Number of runs
        #[arg(short, long, default_value = "10")]
        count: usize,
    },
    /// Show current user and environment info
    Whoami,
    /// Structural diff for JSON
    JsonDiff {
        /// Original JSON
        left: String,
        /// New JSON
        right: String,
    },
    /// Convert JSON to Rust structs
    Rust {
        /// JSON text to convert
        text: String,
        /// Name of the struct (default: Root)
        #[arg(short, long, default_value = "Root")]
        name: String,
    },
    /// Convert JSON to Go structs
    Go {
        /// JSON text to convert
        text: String,
        /// Name of the struct (default: Root)
        #[arg(short, long, default_value = "Root")]
        name: String,
    },
    /// Convert JSON to Java POJOs
    Java {
        /// JSON text to convert
        text: String,
        /// Name of the class (default: Root)
        #[arg(short, long, default_value = "Root")]
        name: String,
    },
    /// Render Markdown in the terminal
    Md {
        /// Path to the markdown file or raw markdown text
        input: String,
    },
    /// Run a download speed test
    Speedtest,
    /// Morse code encoding/decoding
    Morse {
        /// Text to process
        text: String,
        /// Decode instead of encode
        #[arg(short, long)]
        decode: bool,
    },
    /// Fetch cheat sheets from cheat.sh
    Cheat {
        /// Topic to search for (e.g., rust/vector)
        query: String,
    },
    /// Scan for duplicate files in a directory
    Scan {
        /// Target directory
        #[arg(default_value = ".")]
        path: PathBuf,
        /// Minimum file size in bytes
        #[arg(short, long, default_value = "1024")]
        min_size: u64,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Completion { shell } => {
            commands::completion::run(shell)?;
        }
        Commands::Ascii { text, font } => {
            commands::ascii::generate(&text, font)?;
        }
        Commands::Serve { path, port } => {
            commands::serve::run(path, port).await?;
        }
        Commands::Uuid { count, v7 } => {
            commands::uuid::generate(count, v7)?;
        }
        Commands::Base64 { input, decode, file, output, data_uri } => {
            if decode {
                commands::base64::decode(&input, output)?;
            } else {
                commands::base64::encode(&input, file, data_uri)?;
            }
        }
        Commands::Url { text, decode } => {
            if decode {
                commands::url::decode_url(&text)?;
            } else {
                commands::url::encode_url(&text)?;
            }
        }
        Commands::Json { text, minify, query, yaml, toml, csv, schema } => {
            if yaml {
                commands::json::to_yaml(&text)?;
            } else if toml {
                commands::json::to_toml(&text)?;
            } else if csv {
                commands::json::to_csv(&text)?;
            } else if schema {
                commands::json::to_schema(&text)?;
            } else {
                commands::json::process(&text, !minify, query)?;
            }
        }
        Commands::Port { port, kill, wait, timeout, list } => {
            if list {
                commands::port::list()?;
            } else if let Some(p) = port {
                if kill {
                    commands::port::kill(p)?;
                } else if wait {
                    commands::port::wait(p, timeout)?;
                } else {
                    commands::port::check(p)?;
                }
            } else {
                anyhow::bail!("Port number is required for this operation or use --list");
            }
        }
        Commands::Hash { input, file, compare } => {
            if let Some(c) = compare {
                commands::hash::compare(&input, &c, file)?;
            } else if file {
                commands::hash::hash_file(std::path::Path::new(&input))?;
            } else {
                commands::hash::hash_string(&input)?;
            }
        }
        Commands::Time { input } => {
            if let Some(val) = input {
                commands::time::convert(&val)?;
            } else {
                commands::time::current()?;
            }
        }
        Commands::Tree { path, depth } => {
            commands::tree::print_tree(&path, depth)?;
        }
        Commands::Ip => {
            commands::ip::show().await?;
        }
        Commands::Random { length, numeric, symbols } => {
            commands::random::generate(length, numeric, symbols)?;
        }
        Commands::Case { text, to } => {
            commands::case::convert(&text, &to)?;
        }
        Commands::Jwt { token, sign, payload, secret } => {
            if sign {
                if let (Some(p), Some(s)) = (payload, secret) {
                    commands::jwt::sign(&p, &s)?;
                } else {
                    anyhow::bail!("Sign operation requires --payload and --secret");
                }
            } else if let Some(t) = token {
                commands::jwt::inspect(&t)?;
            } else {
                anyhow::bail!("JWT token is required for inspection or use --sign");
            }
        }
        Commands::Env { filter } => {
            commands::env::list(filter)?;
        }
        Commands::Sys => {
            commands::sys::show()?;
        }
        Commands::Color { input } => {
            commands::color::convert(&input)?;
        }
        Commands::Size { value, unit } => {
            commands::size::convert(value, &unit)?;
        }
        Commands::Lorem { paragraphs } => {
            commands::lorem::generate(paragraphs)?;
        }
        Commands::Password { length, no_numbers, no_symbols, no_uppercase, no_lowercase, check } => {
            if let Some(pwd) = check {
                commands::password::check(&pwd)?;
            } else {
                commands::password::generate(length, !no_numbers, !no_symbols, !no_uppercase, !no_lowercase)?;
            }
        }
        Commands::Http { method, url, body, header } => {
            commands::http::request(method, url, body, header).await?;
        }
        Commands::Qr { text } => {
            commands::qr::generate(&text)?;
        }
        Commands::Path { path, resolve } => {
            if resolve {
                commands::path::resolve(&path)?;
            } else {
                commands::path::normalize(&path)?;
            }
        }
        Commands::Yaml { text, json, toml } => {
            if json {
                commands::yaml::to_json(&text)?;
            } else if toml {
                commands::yaml::to_toml(&text)?;
            } else {
                commands::yaml::format(&text)?;
            }
        }
        Commands::Toml { text, json, yaml } => {
            if json {
                commands::toml::to_json(&text)?;
            } else if yaml {
                commands::toml::to_yaml(&text)?;
            } else {
                commands::toml::format(&text)?;
            }
        }
        Commands::Text { text, op, reverse, unique, from, to } => {
            match op.as_str() {
                "sort" => commands::text::sort(&text, reverse, unique)?,
                "reverse" => commands::text::reverse(&text)?,
                "unique" => commands::text::filter_unique(&text)?,
                "trim" => commands::text::trim(&text)?,
                "upper" => commands::text::to_upper(&text)?,
                "lower" => commands::text::to_lower(&text)?,
                "replace" => {
                    if let (Some(f), Some(t)) = (from, to) {
                        commands::text::replace(&text, &f, &t)?;
                    } else {
                        anyhow::bail!("Replace operation requires --from and --to");
                    }
                }
                _ => anyhow::bail!("Unsupported text operation: {}", op),
            }
        }
        Commands::Cron { expression } => {
            commands::cron::explain(&expression)?;
        }
        Commands::Unicode { text } => {
            commands::unicode::inspect(&text)?;
        }
        Commands::Diff { old, new, file } => {
            commands::diff::compare(&old, &new, file)?;
        }
        Commands::Regex { pattern, text } => {
            commands::regex::test(&pattern, &text)?;
        }
        Commands::Escape { text, kind, unescape } => {
            match kind.as_str() {
                "html" => {
                    if unescape {
                        commands::escape::html_unescape(&text)?;
                    } else {
                        commands::escape::html_escape(&text)?;
                    }
                }
                "string" => {
                    commands::escape::string_escape(&text)?;
                }
                _ => anyhow::bail!("Unsupported escape kind: {}", kind),
            }
        }
        Commands::Base { value, from, to } => {
            commands::base::convert(&value, from, to)?;
        }
        Commands::Checksum { input, algo, file } => {
            commands::checksum::calculate(&input, &algo, file)?;
        }
        Commands::Hexview { input, file } => {
            commands::hexview::view(&input, file)?;
        }
        Commands::Dns { domain, record } => {
            commands::dns::lookup(&domain, &record).await?;
        }
        Commands::Gitignore { list, targets } => {
            if list {
                commands::gitignore::list().await?;
            } else {
                commands::gitignore::generate(targets).await?;
            }
        }
        Commands::License { list, kind, year, holder } => {
            if list {
                commands::license::list()?;
            } else if let Some(k) = kind {
                commands::license::generate(&k, year, &holder)?;
            } else {
                anyhow::bail!("Please specify a license kind or use --list");
            }
        }
        Commands::Stat { input, file } => {
            commands::stat::analyze(&input, file)?;
        }
        Commands::Whois { domain } => {
            commands::whois::lookup(&domain)?;
        }
        Commands::Cert { host, port } => {
            commands::cert::inspect(&host, port).await?;
        }
        Commands::Unit { value, from, to } => {
            commands::unit::convert(value, &from, &to)?;
        }
        Commands::Currency { amount, from, to } => {
            commands::currency::convert(amount, &from, &to).await?;
        }
        Commands::Ulid { count } => {
            commands::ulid::generate(count)?;
        }
        Commands::Nanoid { length } => {
            commands::nanoid::generate(length)?;
        }
        Commands::Totp { secret, digits, skew } => {
            commands::totp::generate(&secret, digits, skew)?;
        }
        Commands::Csv { input, json, yaml, markdown } => {
            commands::csv::convert(&input, json, yaml, markdown)?;
        }
        Commands::Shorten { url } => {
            commands::shorten::shorten(&url).await?;
        }
        Commands::Extract { input, kind, file } => {
            commands::extract::extract(&input, &kind, file)?;
        }
        Commands::Sql { query } => {
            commands::sql::format(&query)?;
        }
        Commands::Slug { text } => {
            commands::slug::generate(&text)?;
        }
        Commands::Chmod { input } => {
            commands::chmod::calculate(&input)?;
        }
        Commands::Base32 { input, decode } => {
            if decode {
                commands::base32::decode_base32(&input)?;
            } else {
                commands::base32::encode_base32(&input)?;
            }
        }
        Commands::Base58 { input, decode } => {
            if decode {
                commands::base58::decode_base58(&input)?;
            } else {
                commands::base58::encode_base58(&input)?;
            }
        }
        Commands::Base85 { input, decode } => {
            if decode {
                commands::base85::decode_base85(&input)?;
            } else {
                commands::base85::encode_base85(&input)?;
            }
        }
        Commands::Punycode { input, decode } => {
            if decode {
                commands::punycode::decode_puny(&input)?;
            } else {
                commands::punycode::encode_puny(&input)?;
            }
        }
        Commands::Hmac { text, key } => {
            commands::hmac::calculate(&text, &key)?;
        }
        Commands::Binary { input, from } => {
            if from {
                commands::binary::from_binary(&input)?;
            } else {
                commands::binary::to_binary(&input)?;
            }
        }
        Commands::UserAgent => {
            commands::user_agent::generate()?;
        }
        Commands::Mime { input, extension } => {
            if extension {
                commands::mime::from_extension(&input)?;
            } else {
                commands::mime::guess(&input)?;
            }
        }
        Commands::Joke => {
            commands::joke::random()?;
        }
        Commands::Secret { length, kind } => {
            commands::secret::generate(length, &kind)?;
        }
        Commands::Snowflake { inspect, count } => {
            if let Some(id) = inspect {
                commands::snowflake::inspect(id)?;
            } else {
                commands::snowflake::generate(count)?;
            }
        }
        Commands::Semver { text, op, increment, compare } => {
            match op.as_str() {
                "parse" => commands::semver::parse(&text)?,
                "increment" => {
                    if let Some(part) = increment {
                        commands::semver::increment(&text, &part)?;
                    } else {
                        anyhow::bail!("Increment operation requires --increment <major|minor|patch>");
                    }
                }
                "compare" => {
                    if let Some(other) = compare {
                        commands::semver::compare(&text, &other)?;
                    } else {
                        anyhow::bail!("Compare operation requires --compare <version>");
                    }
                }
                _ => anyhow::bail!("Unsupported semver operation: {}", op),
            }
        }
        Commands::Ksuid { inspect, count } => {
            if let Some(id) = inspect {
                commands::ksuid::inspect(&id)?;
            } else {
                commands::ksuid::generate(count)?;
            }
        }
        Commands::Xml { text, minify } => {
            commands::xml::format(&text, !minify)?;
        }
        Commands::Uptime { urls } => {
            commands::uptime::check_multiple(urls).await?;
        }
        Commands::Portscan { host, start, end, timeout } => {
            commands::portscan::scan(&host, start, end, timeout).await?;
        }
        Commands::Dotenv { op, path, example } => {
            match op.as_str() {
                "init" => commands::dotenv::init(&path)?,
                "example" => commands::dotenv::example(&path)?,
                "load" => commands::dotenv::load(&path)?,
                "compare" => commands::dotenv::compare(&path, &example)?,
                _ => anyhow::bail!("Unsupported dotenv operation: {}", op),
            }
        }
        Commands::Mask { text, kind } => {
            commands::mask::process(&text, &kind)?;
        }
        Commands::Fake { kind, count, ko } => {
            commands::fake::generate(&kind, count, ko)?;
        }
        Commands::Mac { local, count } => {
            if local {
                commands::mac::show_local()?;
            } else {
                commands::mac::generate(count)?;
            }
        }
        Commands::Bcrypt { password, hash, cost } => {
            if let Some(h) = hash {
                commands::bcrypt::verify_password(&password, &h)?;
            } else {
                commands::bcrypt::hash_password(&password, cost)?;
            }
        }
        Commands::Compress { input, output, decompress } => {
            if decompress {
                commands::compress::gunzip(&input, &output)?;
            } else {
                commands::compress::gzip(&input, &output)?;
            }
        }
        Commands::HttpStatus { code, list } => {
            if list {
                commands::http_status::list()?;
            } else if let Some(c) = code {
                commands::http_status::lookup(c)?;
            } else {
                anyhow::bail!("Status code is required or use --list");
            }
        }
        Commands::AsciiTable => {
            commands::ascii_table::show()?;
        }
        Commands::UrlParse { url } => {
            commands::url_parse::parse(&url)?;
        }
        Commands::Github { issue, pr, user, repo } => {
            if let Some(u) = user {
                commands::github::get_user(&u).await?;
            } else if let Some(r) = repo {
                commands::github::get_repo(&r).await?;
            } else {
                commands::github::open(issue, pr)?;
            }
        }
        Commands::Crates { query } => {
            commands::crates::search(&query).await?;
        }
        Commands::Typescript { text, name } => {
            commands::typescript::from_json(&text, &name)?;
        }
        Commands::Weather { location } => {
            commands::weather::get_weather(location).await?;
        }
        Commands::Dictionary { word } => {
            commands::dictionary::lookup(&word).await?;
        }
        Commands::Translate { text, to } => {
            commands::translate::translate(&text, &to).await?;
        }
        Commands::Bench { command, args, count } => {
            commands::bench::run(&command, args, count)?;
        }
        Commands::Whoami => {
            commands::whoami::show()?;
        }
        Commands::JsonDiff { left, right } => {
            commands::json_diff::compare(&left, &right)?;
        }
        Commands::Rust { text, name } => {
            commands::rust::from_json(&text, &name)?;
        }
        Commands::Go { text, name } => {
            commands::go::from_json(&text, &name)?;
        }
        Commands::Java { text, name } => {
            commands::java::from_json(&text, &name)?;
        }
        Commands::Md { input } => {
            commands::md::render(&input)?;
        }
        Commands::Speedtest => {
            commands::speedtest::run().await?;
        }
        Commands::Morse { text, decode } => {
            if decode {
                commands::morse::decode(&text)?;
            } else {
                commands::morse::encode(&text)?;
            }
        }
        Commands::Cheat { query } => {
            commands::cheat::run(&query).await?;
        }
        Commands::Scan { path, min_size } => {
            commands::scan::run(&path, min_size)?;
        }
    }

    Ok(())
}
