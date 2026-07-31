export const repositoryUrl = "https://github.com/chann/cli-tools";

export const installAll = `git clone https://github.com/chann/cli-tools.git
cd cli-tools

cargo install --path crates/code-cost --force
cargo install --path crates/work-summary --force
cargo install --path crates/git-tools --force
cargo install --path crates/dev-tools --force
cargo install --path crates/zzz --force`;

export const tools = [
  {
    id: "code-cost",
    name: "code-cost",
    label: "저장소 가치 측정",
    summary: "코드 규모와 Git 이력을 함께 읽어 개발 비용과 프로젝트 가치를 추정합니다.",
    detail:
      "LOC, 언어 난이도, 복잡도, 성숙도, 기여자 지표를 표와 파일로 확인할 수 있습니다.",
    examples: `code-cost .
code-cost ~/projects/app --dev-levels
code-cost --format json-pretty
code-cost --export report.html`,
    output: "table, JSON, CSV, HTML, Markdown",
  },
  {
    id: "work-summary",
    name: "work-summary",
    label: "Git 작업 요약",
    summary: "커밋 기록을 기간별로 묶어 활동, 예상 작업 시간, 기여 가치를 요약합니다.",
    detail:
      "오늘, 이번 주, 이번 달 필터와 직접 지정한 날짜 범위를 모두 지원합니다.",
    examples: `work-summary --week
work-summary --from 2025-01-01 --to 2025-01-31
work-summary --limit 20
work-summary --export summary.json`,
    output: "table, JSON export",
  },
  {
    id: "git-tools",
    name: "git-tools",
    label: "Git 유지보수",
    summary: "브랜치 정리, 마커 스캔, 건강도, changelog, commit 흐름을 한곳에 모읍니다.",
    detail:
      "반복되는 저장소 점검을 하위 명령으로 나누어 필요한 검사만 빠르게 실행합니다.",
    examples: `git-tools health --verbose
git-tools scan --markers "TODO,DEBUG"
git-tools cleanup --target develop
git-tools changelog --from v1.0.0 --limit 10`,
    output: "terminal reports and changelog",
  },
  {
    id: "dev-tools",
    name: "dev-tools",
    label: "개발자 유틸리티",
    summary: "JSON, 인코딩, 네트워크, 텍스트, 시스템 작업을 짧은 하위 명령으로 제공합니다.",
    detail:
      "작은 웹 도구를 찾거나 일회성 스크립트를 작성하는 대신 터미널에서 바로 처리합니다.",
    examples: `dev-tools uuid --count 5 --v7
dev-tools json '{"b":2,"a":1}' --sort asc
dev-tools typescript '{"name":"test","age":20}'
dev-tools port 8080`,
    output: "terminal and file output",
  },
  {
    id: "zzz",
    name: "zzz",
    label: "백그라운드 실행",
    summary: "대화형 셸로 명령을 백그라운드 실행하고 출력을 날짜별 로그에 저장합니다.",
    detail:
      "프롬프트는 즉시 돌려받고, macOS에서는 완료 알림과 원래 터미널 포커스를 지원합니다.",
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
    name: "데이터 형식",
    description: "구조화 데이터를 검사하고 서로 변환합니다.",
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
    name: "ID와 보안",
    description: "식별자와 개발용 보안 값을 생성하고 검사합니다.",
    commands: [
      { name: "uuid", code: "dev-tools uuid --count 5 --v7" },
      { name: "ulid", code: "dev-tools ulid --count 3" },
      { name: "password", code: 'dev-tools password --check "P@ssw0rd123"' },
      { name: "hash", code: "dev-tools hash README.md --file" },
    ],
  },
  {
    id: "network",
    name: "네트워크",
    description: "주소, 포트, DNS, 인증서를 터미널에서 확인합니다.",
    commands: [
      { name: "port", code: "dev-tools port 8080" },
      { name: "dns", code: "dev-tools dns example.com" },
      { name: "cert", code: "dev-tools cert example.com" },
      { name: "http", code: "dev-tools http https://example.com" },
    ],
  },
  {
    id: "code",
    name: "텍스트와 코드",
    description: "텍스트를 가공하고 JSON에서 타입을 생성합니다.",
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
    name: "파일과 시스템",
    description: "로컬 파일, 프로세스, 시스템 정보를 빠르게 다룹니다.",
    commands: [
      { name: "tree", code: "dev-tools tree . --depth 2" },
      { name: "scan", code: "dev-tools scan . --duplicates" },
      { name: "image", code: "dev-tools image photo.png --format webp" },
      { name: "sys", code: "dev-tools sys" },
    ],
  },
];
