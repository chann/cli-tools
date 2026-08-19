pub mod ascii;
pub mod ascii_table;
pub mod base;
pub mod base32;
pub mod base58;
pub mod base64;
pub mod base85;
pub mod bcrypt;
pub mod bench;
pub mod binary;
pub mod case;
pub mod cert;
pub mod chmod;
pub mod color;
pub mod cheat;
pub mod check_links;
pub mod checksum;
pub mod completion;
pub mod compress;
pub mod crates;
pub mod cron;
pub mod crontab;
pub mod csv;
pub mod currency;
pub mod date_diff;
pub mod detach;
pub mod dictionary;
pub mod diff;
pub mod dns;
pub mod dotenv;
pub mod encoding;
pub mod env;
pub mod escape;
pub mod extract;
pub mod fake;
pub mod github;
pub mod gitignore;
pub mod go;
pub mod hash;
pub mod hexview;
pub mod hmac;
pub mod http;
pub mod http_status;
pub mod image;
pub mod ip;
pub mod java;
pub mod joke;
pub mod json;
pub mod json_diff;
pub mod jwt;
pub mod ksuid;
pub mod license;
pub mod lorem;
pub mod mac;
pub mod mask;
pub mod md;
pub mod mime;
pub mod morse;
pub mod nanoid;
pub mod password;
pub mod path;
pub mod port;
pub mod portscan;
pub mod punycode;
pub mod qr;
pub mod random;
pub mod regex;
pub mod rust;
pub mod scan;
pub mod secret;
pub mod semver;
pub mod serve;
pub mod shorten;
pub mod silent;
pub mod size;
pub mod slug;
pub mod snowflake;
pub mod speedtest;
pub mod sql;
pub mod stat;
pub mod sys;
pub mod text;
pub mod time;
pub mod toml;
pub mod totp;
pub mod translate;
pub mod tree;
pub mod typescript;
pub mod tz;
pub mod ulid;
pub mod unicode;
pub mod unit;
pub mod uptime;
pub mod url;
pub mod url_parse;
pub mod user_agent;
pub mod uuid;
pub mod weather;
pub mod whoami;
pub mod whois;
pub mod xml;
pub mod yaml;

use clap::Subcommand;
use std::path::PathBuf;

#[derive(Subcommand, Debug)]
pub enum Commands {
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
        /// Check JSON syntax without emitting the document
        #[arg(
            long,
            conflicts_with_all = [
                "format", "minify", "sort", "query", "yaml", "toml", "csv", "schema"
            ]
        )]
        check: bool,
        /// Pretty-print JSON explicitly
        #[arg(
            long,
            conflicts_with_all = ["minify", "yaml", "toml", "csv", "schema"]
        )]
        format: bool,
        /// Minify instead of pretty-print
        #[arg(short, long)]
        minify: bool,
        /// Sort object keys recursively (asc or desc)
        #[arg(
            long,
            value_enum,
            conflicts_with_all = ["yaml", "toml", "csv", "schema"]
        )]
        sort: Option<crate::commands::json::SortOrder>,
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
    /// Calculate the difference between two dates
    DateDiff {
        /// Start date (YYYY-MM-DD, "YYYY-MM-DD HH:MM:SS", RFC3339, or unix timestamp)
        from: String,
        /// End date (defaults to now)
        to: Option<String>,
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
    /// Manage user crontab entries (list, add, remove, edit)
    Crontab {
        #[command(subcommand)]
        action: Option<CrontabAction>,
    },
    /// World clock and IANA timezone lookup
    Tz {
        /// Search query (e.g., "seoul", "new york"). Omit for the world clock.
        query: Option<String>,
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
    /// Run a command silently in the background
    Detach {
        /// Command to run
        command: String,
        /// Arguments for the command
        #[arg(trailing_var_arg = true)]
        args: Vec<String>,
    },
    /// Run a command silently in the foreground and save stdout to ~/.commands
    Silent {
        /// Command to run
        #[arg(allow_hyphen_values = true)]
        command: String,
        /// Arguments for the command
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },
    /// Convert, resize, and transform images (webp, png, jpg, gif, etc.). Supports batch processing for directories.
    Image {
        /// Input image path or directory (for batch processing)
        input: String,
        /// Output file path or directory
        #[arg(short, long)]
        output: Option<String>,
        /// Output format (png, jpg, webp, gif, bmp, ico)
        #[arg(short, long)]
        format: Option<String>,
        /// Print image info without modifying
        #[arg(long)]
        info: bool,
        /// Generate thumbnail of max size (e.g. 256)
        #[arg(short, long)]
        thumbnail: Option<u32>,
        /// Resize width
        #[arg(long)]
        width: Option<u32>,
        /// Resize height
        #[arg(long)]
        height: Option<u32>,
        /// Blur with sigma value (e.g., 2.0)
        #[arg(long)]
        blur: Option<f32>,
        /// Rotate degrees (90, 180, 270)
        #[arg(long)]
        rotate: Option<u32>,
        /// Flip horizontally
        #[arg(long)]
        flip_h: bool,
        /// Flip vertically
        #[arg(long)]
        flip_v: bool,
        /// Convert to grayscale
        #[arg(long)]
        grayscale: bool,
        /// Invert colors
        #[arg(long)]
        invert: bool,
        /// Crop image (format: x,y,width,height)
        #[arg(long)]
        crop: Option<String>,
        /// Adjust brightness (e.g., 20 for brighter, -20 for darker)
        #[arg(long)]
        brighten: Option<i32>,
        /// Adjust contrast (e.g., 10.0, -10.0)
        #[arg(long)]
        contrast: Option<f32>,
        /// Rotate hue by degrees
        #[arg(long)]
        hue: Option<i32>,
    },
}

#[derive(Subcommand, Debug)]
pub enum CrontabAction {
    /// Show crontab entries with descriptions and next run times (default)
    List,
    /// Add an entry (schedule is validated before install)
    Add {
        /// Cron schedule (e.g., "0 9 * * 1-5" or "@daily")
        schedule: String,
        /// Command to run
        command: String,
        /// Comment line to add above the entry
        #[arg(short = 'm', long)]
        comment: Option<String>,
    },
    /// Remove an entry by its list index
    Remove {
        /// Entry number shown by `crontab list`
        index: usize,
    },
    /// Edit an entry's schedule and/or command by its list index
    Edit {
        /// Entry number shown by `crontab list`
        index: usize,
        /// New cron schedule
        #[arg(short, long)]
        schedule: Option<String>,
        /// New command
        #[arg(short, long)]
        command: Option<String>,
    },
}
