# CLI 도구 모음

[English](README.md)

저장소 분석, Git 워크플로우 관리, 개발자용 데이터 변환, 무음 명령 로그
저장을 위한 Rust 기반 CLI 도구 모음입니다.

## 도구

| 명령어 | Crate | 용도 |
| --- | --- | --- |
| `code-cost` | `crates/code-cost` | 저장소의 개발 비용과 프로젝트 가치를 추정합니다. |
| `work-summary` | `crates/work-summary` | Git 활동, 작업량, 기여 가치를 요약합니다. |
| `git-tools` | `crates/git-tools` | 브랜치 정리, 마커 스캔, 프로젝트 건강도 확인, changelog 생성을 지원합니다. |
| `dev-tools` | `crates/dev-tools` | 데이터, 인코딩, 네트워크, 텍스트, 시스템 유틸리티 모음입니다. |
| `zzz` | `crates/zzz` | 명령을 조용히 실행하고 stdout을 `~/.commands`에 저장합니다. |

`zzz`는 별도의 설치 대상입니다. `dev-tools`를 설치해도 `zzz` 바이너리는
함께 설치되지 않습니다.

## 설치

필요한 도구만 골라 설치할 수 있습니다:

```bash
cargo install --path crates/code-cost --force
cargo install --path crates/work-summary --force
cargo install --path crates/git-tools --force
cargo install --path crates/dev-tools --force
cargo install --path crates/zzz --force
```

전체 workspace 빌드와 테스트:

```bash
cargo build --release --workspace --bins
cargo test --workspace --all-targets
```

## code-cost

코드 규모, 언어 난이도, 복잡도, 성숙도, Git 히스토리를 기반으로 저장소의
금전적 가치를 추정합니다.

```bash
# 현재 디렉토리 분석
code-cost

# 특정 저장소 분석
code-cost ~/projects/my-app ../other-repo

# 간단한 표 출력
code-cost --simple

# 개발자 레벨별 비용 분석 포함
code-cost --dev-levels

# 결과 내보내기
code-cost --format json-pretty
code-cost --export report.html
code-cost --export report.md
code-cost --export report.csv
```

주요 출력:

- 코드, 주석, 공백 라인 수 분석
- 언어별 비중과 난이도 가중치
- Git 기간, 커밋, 기여자 지표
- 복잡도, 성숙도, 코드 품질 점수
- CSV, HTML, Markdown, JSON, 터미널 출력

## work-summary

Git 커밋 히스토리를 분석해 추정 작업 시간, 활동 패턴, 가치 계산이 포함된
업무 요약을 생성합니다.

```bash
# 최근 30일 분석
work-summary

# 빠른 필터
work-summary --today
work-summary --week
work-summary --month

# 날짜 범위와 커밋 수 제한
work-summary --from 2025-01-01 --to 2025-01-31
work-summary --limit 20

# 간단 출력과 JSON 내보내기
work-summary --simple
work-summary --export summary.json
```

추정기는 커밋 간 시간 간격과 코드 변경량 및 복잡도를 함께 사용합니다.

## git-tools

저장소 관리를 위한 개발자 워크플로우 유틸리티입니다.

```bash
# 브랜치 정리
git-tools cleanup
git-tools cleanup --force
git-tools cleanup --target develop

# 마커 스캔
git-tools scan
git-tools scan --markers "TODO,DEBUG"

# 프로젝트 건강도 확인
git-tools health
git-tools health --verbose

# 환경 변수와 changelog 도우미
git-tools env
git-tools changelog
git-tools changelog --from v1.0.0 --limit 10

# 대화형 Conventional Commit 작성
git-tools commit
```

## dev-tools

자주 쓰는 변환과 시스템 확인을 위한 작은 유틸리티 모음입니다.

```bash
# UUID
dev-tools uuid --count 5 --v7

# Base64
dev-tools base64 "hello world"
dev-tools base64 --decode "aGVsbG8gd29ybGQ="

# JSON 문법 검사, 포맷, 압축, 재귀 key 정렬
dev-tools json '{"b":2,"a":1}' --check
dev-tools json '{"b":2,"a":1}' --format
dev-tools json '{"b":2,"a":1}' --minify
dev-tools json '{"b":2,"a":1}' --sort asc
dev-tools json '{"b":2,"a":1}' --sort desc --minify

# 모델 생성
dev-tools typescript '{"name":"test","age":20}'
dev-tools rust '{"name":"test","age":20}'
dev-tools go '{"name":"test","age":20}'

# 포트, 해시, 시간
dev-tools port 8080
dev-tools port 8080 --kill
dev-tools hash README.md --file
dev-tools checksum README.md --file --algo sha512
dev-tools time
dev-tools time 1740000000

# 텍스트, 보안, 네트워크 도우미
dev-tools password --check "P@ssw0rd123"
dev-tools morse "HELLO WORLD"
dev-tools morse --decode ".... . .-.. .-.. --- / .-- --- .-. .-.. -.."
dev-tools ip
```

명령 로그 저장용 `dev-tools silent`도 계속 사용할 수 있습니다:

```bash
dev-tools silent git status --short
dev-tools silent python script.py
```

## zzz

`zzz`는 명령 출력을 조용히 기록하는 독립 실행 명령입니다. Unix에서는 대화형
셸을 통해 실행되므로 shell alias와 function도 사용할 수 있습니다. 명령은
백그라운드에서 실행되어 프롬프트가 즉시 반환되며, macOS에서는 명령이 끝나면
성공 또는 실패 여부를 시스템 알림으로 전송합니다.

클릭 가능한 iTerm2 및 Terminal.app 알림을 사용하려면
[`terminal-notifier`](https://github.com/julienXX/terminal-notifier)를
설치합니다:

```bash
brew install terminal-notifier
```

iTerm2와 Terminal.app에서는 명령을 시작한 터미널 아이콘이 알림에 표시됩니다.
알림을 누르면 Herdr를 실행 중인 iTerm2 세션을 포함해 명령을 시작한 정확한
세션이나 탭으로 키보드 포커스가 돌아갑니다. 처음 사용할 때 macOS가 Automation
권한을 요청할 수 있습니다. `terminal-notifier`가 없으면 두 터미널 모두 정확한
세션 포커스를 지원하지 않는 일반 완료 알림으로 폴백합니다.

Ghostty는 자체 macOS 네이티브 알림 채널을 사용하므로 `terminal-notifier`가
필요하지 않습니다. 알림에는 Ghostty 앱 아이콘이 표시되며, 누르면 명령을 시작한
정확한 Ghostty 화면으로 돌아갑니다. 처음 사용할 때 macOS가 알림 권한을 요청할
수 있습니다. 원래 대상이 닫혔다면 `zzz`는 새 창을 만들지 않습니다.

```bash
zzz echo "hello"
zzz git status --short
zzz update-agents
```

로그 저장 위치:

```text
~/.commands/{yymmdd}/{hhmmss}-{command_name}.log
```

예시:

```text
~/.commands/260605/224512-echo.log
```

## 릴리즈

릴리즈는 Git tag push로 진행됩니다.

1. `Cargo.toml`의 `workspace.package.version`을 변경합니다.
2. `cargo test --workspace --all-targets`를 실행합니다.
3. 일치하는 태그를 만들고 push합니다:

   ```bash
   git tag v0.1.0
   git push origin v0.1.0
   ```

GitHub Actions release workflow는 tag와 workspace 버전이 일치하는지 검증하고,
전체 테스트를 실행한 뒤 플랫폼별 archive를 빌드해 GitHub Release를 생성합니다.
릴리즈 artifact는 다음을 포함합니다:

- Linux x86_64: `tar.gz`
- macOS Intel: `tar.gz`
- macOS Apple Silicon: `tar.gz`
- Windows x86_64: `zip`

공유 버전 규칙은 [versioning.md](versioning.md)를 참고하세요.

## 프로젝트 구조

```text
cli-tools/
├── .github/workflows/release.yml
├── crates/
│   ├── cli-core/       # 공통 UI, 출력, 설정, 명령 로그 구현
│   ├── code-cost/      # 저장소 가치 분석기
│   ├── dev-tools/      # 개발자 유틸리티 모음
│   ├── git-tools/      # Git 워크플로우와 건강도 도구
│   ├── work-summary/   # Git 업무 요약 분석기
│   └── zzz/            # 독립 무음 명령 로그 저장기
├── Cargo.toml
└── versioning.md
```

## 라이선스

MIT 라이선스입니다. [LICENSE](LICENSE)를 참고하세요.

## 저자

CHANN
