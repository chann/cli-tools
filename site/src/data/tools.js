export const repositoryUrl = "https://github.com/chann/cli-tools";

export const installAll = `git clone https://github.com/chann/cli-tools.git
cd cli-tools

cargo install --path crates/code-cost --force
cargo install --path crates/work-summary --force
cargo install --path crates/git-tools --force
cargo install --path crates/dev-tools --force
cargo install --path crates/zzz --force`;

export const iterm2KoreanControlCommand = `uv run scripts/iterm2_korean_control_keys.py preflight
uv run scripts/iterm2_korean_control_keys.py apply
uv run scripts/iterm2_korean_control_keys.py verify`;

export const iterm2KoreanControlRestoreCommand = `uv run scripts/iterm2_korean_control_keys.py restore \\
  --history '/absolute/path/to/setting_history.json'`;

export const ghosttyKoreanControlCommand = `python3 scripts/ghostty_korean_control_keys.py preflight
python3 scripts/ghostty_korean_control_keys.py apply
python3 scripts/ghostty_korean_control_keys.py verify`;

export const ghosttyKoreanControlRestoreCommand = `python3 scripts/ghostty_korean_control_keys.py restore \\
  --history '/absolute/path/to/setting_history.json'`;

export const tools = [
  {
    id: "code-cost",
    name: "code-cost",
    examples: `code-cost .
code-cost ~/projects/app --dev-levels
code-cost --format json-pretty
code-cost --export report.html`,
    output: "table, JSON, CSV, HTML, Markdown",
  },
  {
    id: "work-summary",
    name: "work-summary",
    examples: `work-summary --week
work-summary --from 2025-01-01 --to 2025-01-31
work-summary --limit 20
work-summary --export summary.json`,
    output: "table, JSON export",
  },
  {
    id: "git-tools",
    name: "git-tools",
    examples: `git-tools health --verbose
git-tools scan --markers "TODO,DEBUG"
git-tools cleanup --target develop
git-tools changelog --from v1.0.0 --limit 10`,
    output: "terminal reports and changelog",
  },
  {
    id: "dev-tools",
    name: "dev-tools",
    examples: `dev-tools uuid --count 5 --v7
dev-tools json '{"b":2,"a":1}' --sort asc
dev-tools typescript '{"name":"test","age":20}'
dev-tools port 8080`,
    output: "terminal and file output",
  },
  {
    id: "zzz",
    name: "zzz",
    examples: `zzz cargo test
zzz --wait cargo test
zzz --print-log make build
zzz --no-notify long-task
zzz -- rg --files -g '*.rs'`,
    output: "~/.commands/{yymmdd}/{hhmmss}-{command}.log",
  },
];

export const utilityGroups = [
  {
    id: "data",
    commands: [
      {
        name: "json",
        code: `dev-tools json '{"b":2,"a":1}' --check
dev-tools json '{"b":2,"a":1}' --sort asc`,
      },
      {
        name: "yaml",
        code: `dev-tools yaml 'name: cli-tools' --json`,
      },
      {
        name: "csv",
        code: "dev-tools csv records.csv --file --to json",
      },
      {
        name: "toml",
        code: `dev-tools toml 'name = "cli-tools"' --json`,
      },
    ],
  },
  {
    id: "identity",
    commands: [
      { name: "uuid", code: "dev-tools uuid --count 5 --v7" },
      { name: "ulid", code: "dev-tools ulid --count 3" },
      { name: "password", code: 'dev-tools password --check "P@ssw0rd123"' },
      { name: "hash", code: "dev-tools hash README.md --file" },
    ],
  },
  {
    id: "network",
    commands: [
      { name: "port", code: "dev-tools port 8080" },
      { name: "dns", code: "dev-tools dns example.com" },
      { name: "cert", code: "dev-tools cert example.com" },
      { name: "http", code: "dev-tools http https://example.com" },
    ],
  },
  {
    id: "code",
    commands: [
      {
        name: "typescript",
        code: `dev-tools typescript '{"name":"test","age":20}'`,
      },
      { name: "rust", code: `dev-tools rust '{"name":"test","age":20}'` },
      { name: "regex", code: "dev-tools regex '\\d+' 'release-2607'" },
      { name: "md", code: "dev-tools md README.md" },
    ],
  },
  {
    id: "files",
    commands: [
      { name: "tree", code: "dev-tools tree . --depth 2" },
      { name: "scan", code: "dev-tools scan . --duplicates" },
      { name: "image", code: "dev-tools image photo.png --format webp" },
      { name: "sys", code: "dev-tools sys" },
    ],
  },
];
