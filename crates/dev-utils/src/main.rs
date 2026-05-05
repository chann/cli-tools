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
    /// Generate or inspect UUIDs
    Uuid {
        /// Number of UUIDs to generate
        #[arg(short, long, default_value = "1")]
        count: usize,
        /// Generate UUID v7 instead of v4
        #[arg(long)]
        v7: bool,
        /// ID to inspect
        #[arg(short, long)]
        inspect: Option<String>,
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
        /// Use URL-safe character set
        #[arg(long)]
        url_safe: bool,
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
    /// Hash utilities
    Hash {
        /// Text or file path to hash
        input: String,
        /// Algorithm (md5, sha1, sha256, sha512)
        #[arg(short, long, default_value = "sha256")]
        algo: String,
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
        /// Show file sizes
        #[arg(short, long)]
        size: bool,
        /// Show git status
        #[arg(short, long)]
        git: bool,
    },
    /// Show IP information
    Ip {
        /// Optional IP address or hostname to lookup
        target: Option<String>,
    },
    /// Generate random data
    Random {
        /// Type of random data (string, number, boolean)
        #[arg(short, long, default_value = "string")]
        kind: String,
        /// Number of items to generate
        #[arg(short, long, default_value = "1")]
        count: usize,
        /// Length of the string
        #[arg(short, long, default_value = "16")]
        length: usize,
        /// Minimum value for number
        #[arg(long, default_value = "0")]
        min: usize,
        /// Maximum value for number
        #[arg(long, default_value = "100")]
        max: usize,
        /// Include numbers (for string)
        #[arg(short, long)]
        numeric: bool,
        /// Include symbols (for string)
        #[arg(short, long)]
        symbols: bool,
        /// Use uppercase only (for string)
        #[arg(short, long)]
        uppercase: bool,
    },
    /// String case conversion
    Case {
        /// Text to convert
        text: String,
        /// Target case (snake, camel, pascal, kebab, shouty, train)
        #[arg(short, long)]
        to: Option<String>,
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
        #[arg(short = 'S', long)]
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
        /// Number of paragraphs/sentences/words to generate
        #[arg(short, long, default_value = "1")]
        count: usize,
        /// Type of generation (paragraphs, sentences, words)
        #[arg(short, long, default_value = "paragraphs")]
        kind: String,
    },
    /// Generate a secure password or check strength
    Password {
        /// Length of the password to generate
        #[arg(short, long, default_value = "16")]
        length: usize,
        /// Generate a passphrase instead of a random string
        #[arg(short, long)]
        passphrase: bool,
        /// Number of words in the passphrase
        #[arg(short, long, default_value = "5")]
        words: usize,
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
        #[arg(short = 'H', long)]
        header: Vec<String>,
        /// Output file path to save the response body
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// Show detailed request/response info
        #[arg(short, long)]
        verbose: bool,
    },
    /// Generate a QR code in the terminal or as a file
    Qr {
        /// Text or URL to encode
        text: String,
        /// Output file path (e.g., qrcode.png, qrcode.svg)
        #[arg(short, long)]
        output: Option<String>,
        /// Error correction level (L, M, Q, H)
        #[arg(short, long, default_value = "M")]
        level: String,
        /// Size of the QR code in pixels (for file output)
        #[arg(short, long, default_value = "256")]
        size: u32,
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
        /// Operation (sort, reverse, unique, trim, upper, lower, replace, shuffle, count, prefix, suffix, truncate)
        #[arg(short, long, default_value = "trim")]
        op: String,
        /// Sort in reverse order
        #[arg(long)]
        reverse: bool,
        /// Remove duplicate lines
        #[arg(long)]
        unique: bool,
        /// Text to replace or prefix/suffix/truncate value
        #[arg(long)]
        from: Option<String>,
        /// Replacement text
        #[arg(long)]
        to: Option<String>,
        /// Show line numbers
        #[arg(short, long)]
        line_numbers: bool,
        /// Prefix to add to each line
        #[arg(long)]
        prefix: Option<String>,
        /// Suffix to add to each line
        #[arg(long)]
        suffix: Option<String>,
        /// Truncate lines to this length
        #[arg(long)]
        truncate: Option<usize>,
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
        /// Source base (2-36)
        #[arg(short, long, default_value = "10")]
        from: u32,
        /// Target base (2-36). If not provided, shows common bases.
        #[arg(short, long)]
        to: Option<u32>,
        /// Show all common bases (2, 8, 10, 16, 32, 36)
        #[arg(short, long)]
        all: bool,
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
        /// Verify against a hash
        #[arg(short, long)]
        check: Option<String>,
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
        host: Option<String>,
        /// Port (default: 443)
        #[arg(short, long, default_value = "443")]
        port: u16,
        /// Path to a local certificate file (PEM or DER)
        #[arg(short, long)]
        file: Option<PathBuf>,
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
    /// Generate or inspect ULIDs
    Ulid {
        /// Number of ULIDs to generate
        #[arg(short, long, default_value = "1")]
        count: usize,
        /// ID to inspect
        #[arg(short, long)]
        inspect: Option<String>,
    },
    /// Generate NanoIDs
    Nanoid {
        /// Length of the ID
        #[arg(short, long, default_value = "21")]
        length: usize,
        /// Number of IDs to generate
        #[arg(short, long, default_value = "1")]
        count: usize,
        /// Custom alphabet to use
        #[arg(short, long)]
        alphabet: Option<String>,
    },
    /// Generate a TOTP code or a new secret
    Totp {
        /// Secret key (Base32 encoded). If not provided, a new one will be generated.
        secret: Option<String>,
        /// Number of digits (default: 6)
        #[arg(short, long, default_value = "6")]
        digits: usize,
        /// Skew in seconds (default: 0)
        #[arg(short, long, default_value = "0")]
        skew: u8,
        /// Generate a new random secret instead of a code
        #[arg(short, long)]
        generate_secret: bool,
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
        #[arg(required_unless_present = "file")]
        query: Option<String>,
        /// Read query from a file
        #[arg(short, long)]
        file: Option<PathBuf>,
        /// Indentation size (default: 2)
        #[arg(short, long, default_value = "2")]
        indent: usize,
        /// Use tabs for indentation instead of spaces
        #[arg(short, long)]
        tabs: bool,
        /// Use lowercase keywords instead of uppercase
        #[arg(short, long)]
        lowercase: bool,
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
        /// Text or file path to process
        input: String,
        /// Decode instead of encode
        #[arg(short, long)]
        decode: bool,
        /// Treat input as a file path
        #[arg(short, long)]
        file: bool,
        /// Output file path
        #[arg(short, long)]
        output: Option<String>,
    },
    /// Base58 encoding/decoding
    Base58 {
        /// Text or file path to process
        input: String,
        /// Decode instead of encode
        #[arg(short, long)]
        decode: bool,
        /// Treat input as a file path
        #[arg(short, long)]
        file: bool,
        /// Output file path
        #[arg(short, long)]
        output: Option<String>,
    },
    /// Base85 encoding/decoding
    Base85 {
        /// Text or file path to process
        input: String,
        /// Decode instead of encode
        #[arg(short, long)]
        decode: bool,
        /// Treat input as a file path
        #[arg(short, long)]
        file: bool,
        /// Output file path
        #[arg(short, long)]
        output: Option<String>,
    },
    /// Punycode encoding/decoding
    Punycode {
        /// Text to process
        input: String,
        /// Decode instead of encode
        #[arg(short, long)]
        decode: bool,
    },
    /// HMAC utilities
    Hmac {
        /// Text to process
        text: String,
        /// Secret key
        #[arg(short, long)]
        key: String,
        /// Algorithm (md5, sha1, sha256, sha512)
        #[arg(short, long, default_value = "sha256")]
        algo: String,
    },
    /// Binary string conversion
    Binary {
        /// Text or binary string to process
        input: String,
        /// Convert from binary instead of to binary
        #[arg(short, long)]
        from: bool,
    },
    /// Generate or inspect a random user agent string
    UserAgent {
        /// Optional user agent string to inspect
        ua: Option<String>,
    },
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
        /// Number of keys to generate
        #[arg(short, long, default_value = "1")]
        count: usize,
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
        /// Search by name or description
        #[arg(short, long)]
        search: Option<String>,
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
        /// Search for repositories
        #[arg(short, long)]
        search: Option<String>,
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
    /// Scan for duplicate files, empty files/dirs, or broken links
    Scan {
        /// Target directory
        #[arg(default_value = ".")]
        path: PathBuf,
        /// Minimum file size in bytes for duplicate scan
        #[arg(short, long, default_value = "1")]
        min_size: u64,
        /// Scan for duplicate files
        #[arg(short, long)]
        duplicates: bool,
        /// Scan for empty files
        #[arg(short, long)]
        empty: bool,
        /// Scan for empty directories
        #[arg(long)]
        dirs: bool,
        /// Scan for broken symbolic links
        #[arg(short, long)]
        links: bool,
        /// Show all (duplicates, empty, dirs, links)
        #[arg(short, long)]
        all: bool,
    },
    /// Check for broken links in files
    CheckLinks {
        /// Target directory or file
        #[arg(default_value = ".")]
        path: PathBuf,
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
        Commands::Uuid { count, v7, inspect } => {
            if let Some(id) = inspect {
                commands::uuid::inspect(&id)?;
            } else {
                commands::uuid::generate(count, v7)?;
            }
        }
        Commands::Base64 { input, decode, file, output, data_uri, url_safe } => {
            if decode {
                commands::base64::decode(&input, output, url_safe)?;
            } else {
                commands::base64::encode(&input, file, data_uri, url_safe)?;
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
        Commands::Hash { input, algo, file, compare } => {
            if let Some(c) = compare {
                commands::hash::compare(&input, &c, &algo, file)?;
            } else if file {
                commands::hash::hash_file(std::path::Path::new(&input), &algo)?;
            } else {
                commands::hash::hash_string(&input, &algo)?;
            }
        }
        Commands::Time { input } => {
            if let Some(val) = input {
                commands::time::convert(&val)?;
            } else {
                commands::time::current()?;
            }
        }
        Commands::Tree { path, depth, size, git } => {
            commands::tree::print_tree(&path, depth, size, git)?;
        }
        Commands::Ip { target } => {
            commands::ip::show(target).await?;
        }
        Commands::Random { kind, count, length, min, max, numeric, symbols, uppercase } => {
            commands::random::generate(&kind, count, length, min, max, numeric, symbols, uppercase)?;
        }
        Commands::Case { text, to } => {
            commands::case::convert(&text, to.as_deref())?;
        }
        Commands::Jwt { token, sign, payload, secret } => {
            if sign {
                if let (Some(p), Some(s)) = (payload, secret) {
                    commands::jwt::sign(&p, &s)?;
                } else {
                    anyhow::bail!("Sign operation requires --payload and --secret");
                }
            } else if let Some(t) = token {
                commands::jwt::inspect(&t, secret.as_deref())?;
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
        Commands::Lorem { count, kind } => {
            commands::lorem::generate(count, &kind)?;
        }
        Commands::Password { length, passphrase, words, no_numbers, no_symbols, no_uppercase, no_lowercase, check } => {
            if let Some(pwd) = check {
                commands::password::check(&pwd)?;
            } else if passphrase {
                commands::password::generate_passphrase(words)?;
            } else {
                commands::password::generate(length, !no_numbers, !no_symbols, !no_uppercase, !no_lowercase)?;
            }
        }
        Commands::Http { method, url, body, header, output, verbose } => {
            commands::http::request(method, url, body, header, output, verbose).await?;
        }
        Commands::Qr { text, output, level, size } => {
            commands::qr::generate(&text, output, &level, size)?;
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
        Commands::Text { text, op, reverse, unique, from, to, line_numbers, prefix, suffix, truncate } => {
            commands::text::process(
                &text,
                &op,
                reverse,
                unique,
                from,
                to,
                line_numbers,
                prefix,
                suffix,
                truncate,
            )?;
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
        Commands::Base { value, from, to, all } => {
            commands::base::convert(&value, from, to, all)?;
        }
        Commands::Checksum { input, algo, file, check } => {
            if let Some(c) = check {
                commands::checksum::verify(&input, &c, &algo, file)?;
            } else {
                commands::checksum::calculate(&input, &algo, file)?;
            }
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
        Commands::Cert { host, port, file } => {
            if let Some(h) = host {
                commands::cert::inspect_remote(&h, port).await?;
            } else if let Some(f) = file {
                commands::cert::inspect_file(&f)?;
            } else {
                anyhow::bail!("Either host or file must be provided");
            }
        }
        Commands::Unit { value, from, to } => {
            commands::unit::convert(value, &from, &to)?;
        }
        Commands::Currency { amount, from, to } => {
            commands::currency::convert(amount, &from, &to).await?;
        }
        Commands::Ulid { count, inspect } => {
            if let Some(id) = inspect {
                commands::ulid::inspect(&id)?;
            } else {
                commands::ulid::generate(count)?;
            }
        }
        Commands::Nanoid { length, count, alphabet } => {
            commands::nanoid::generate(length, count, alphabet.as_deref())?;
        }
        Commands::Totp { secret, digits, skew, generate_secret } => {
            if generate_secret {
                commands::totp::generate_new_secret()?;
            } else if let Some(s) = secret {
                commands::totp::generate(&s, digits, skew)?;
            } else {
                anyhow::bail!("Secret key is required for TOTP generation, or use --generate-secret");
            }
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
        Commands::Sql { query, file, indent, tabs, lowercase } => {
            let sql = if let Some(f) = file {
                std::fs::read_to_string(f)?
            } else {
                query.unwrap_or_default()
            };
            commands::sql::format(&sql, indent, tabs, lowercase)?;
        }
        Commands::Slug { text } => {
            commands::slug::generate(&text)?;
        }
        Commands::Chmod { input } => {
            commands::chmod::calculate(&input)?;
        }
        Commands::Base32 { input, decode, file, output } => {
            if decode {
                commands::base32::decode_base32(&input, output)?;
            } else {
                commands::base32::encode_base32(&input, file)?;
            }
        }
        Commands::Base58 { input, decode, file, output } => {
            if decode {
                commands::base58::decode_base58(&input, output)?;
            } else {
                commands::base58::encode_base58(&input, file)?;
            }
        }
        Commands::Base85 { input, decode, file, output } => {
            if decode {
                commands::base85::decode_base85(&input, output)?;
            } else {
                commands::base85::encode_base85(&input, file)?;
            }
        }
        Commands::Punycode { input, decode } => {
            if decode {
                commands::punycode::decode_puny(&input)?;
            } else {
                commands::punycode::encode_puny(&input)?;
            }
        }
        Commands::Hmac { text, key, algo } => {
            commands::hmac::calculate(&text, &key, &algo)?;
        }
        Commands::Binary { input, from } => {
            if from {
                commands::binary::from_binary(&input)?;
            } else {
                commands::binary::to_binary(&input)?;
            }
        }
        Commands::UserAgent { ua } => {
            if let Some(val) = ua {
                commands::user_agent::inspect(&val)?;
            } else {
                commands::user_agent::generate()?;
            }
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
        Commands::Secret { length, count, kind } => {
            commands::secret::generate(length, count, &kind)?;
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
        Commands::HttpStatus { code, list, search } => {
            if list {
                commands::http_status::list()?;
            } else if let Some(q) = search {
                commands::http_status::search(&q)?;
            } else if let Some(c) = code {
                commands::http_status::lookup(c)?;
            } else {
                anyhow::bail!("Status code is required, or use --list or --search");
            }
        }
        Commands::AsciiTable => {
            commands::ascii_table::show()?;
        }
        Commands::UrlParse { url } => {
            commands::url_parse::parse(&url)?;
        }
        Commands::Github { issue, pr, user, repo, search } => {
            if let Some(u) = user {
                commands::github::get_user(&u).await?;
            } else if let Some(r) = repo {
                commands::github::get_repo(&r).await?;
            } else if let Some(q) = search {
                commands::github::search(&q).await?;
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
        Commands::Scan { path, min_size, duplicates, empty, dirs, links, all } => {
            let mut opts = commands::scan::ScanOptions {
                duplicates,
                empty,
                dirs,
                links,
                min_size,
            };
            if all || (!duplicates && !empty && !dirs && !links) {
                opts.duplicates = true;
                opts.empty = true;
                opts.dirs = true;
                opts.links = true;
            }
            commands::scan::run(&path, opts)?;
        }
        Commands::CheckLinks { path } => {
            commands::check_links::run(&path).await?;
        }
    }

    Ok(())
}
