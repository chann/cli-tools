# CLI 도구 모음

[English](README.md)

저장소 분석, Git 워크플로우 관리, 개발자용 데이터 변환, 무음 명령 로그
저장을 위한 Rust 기반 CLI 도구 모음입니다.

한글 입력을 유지한 채 iTerm2와 Ghostty에서 물리 `Control-C`와 `Control-G`를
사용할 수 있도록 설정하는 복원 가능한 macOS 도우미도 포함합니다.

## 도구

| 명령어 | Crate | 용도 |
| --- | --- | --- |
| `code-cost` | `crates/code-cost` | 저장소의 개발 비용과 프로젝트 가치를 추정합니다. |
| `work-summary` | `crates/work-summary` | Git 활동, 작업량, 기여 가치를 요약합니다. |
| `git-tools` | `crates/git-tools` | 브랜치 정리, 마커 스캔, 프로젝트 건강도 확인, changelog 생성을 지원합니다. |
| `dev-tools` | `crates/dev-tools` | 데이터, 인코딩, 네트워크, 텍스트, 시스템 유틸리티 모음입니다. |
| `prompt-export` | `crates/prompt-export` | Claude Code / Codex 프롬프트와 에이전트 출력을 마크다운으로 내보냅니다. |
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
cargo install --path crates/prompt-export --force
cargo install --path crates/zzz --force
```

전체 workspace 빌드와 테스트:

```bash
cargo build --release --workspace --bins
cargo test --workspace --all-targets
```

## iTerm2 한글 Control 키 설정 (macOS)

iTerm2 `3.6.11`에서 `scripts/iterm2_korean_control_keys.py`는 물리
`Control-C`를 PTY 바이트 `0x03`에, 물리 `Control-G`를 `0x07`에 연결합니다.
입력 소스를 바꾸거나 상주 프로세스를 실행하지 않으며 손쉬운 사용 또는 입력
모니터링 권한을 요청하지 않습니다.

이 도우미는 macOS, 정확히 iTerm2 `3.6.11`,
[`uv`](https://docs.astral.sh/uv/), 활성화하고 승인한 iTerm2 Python API 접근이
필요합니다. 저장소 루트에서 각 명령을 실행하세요:

```bash
# 환경 설정을 바꾸지 않고 전역 및 프로필 매핑 확인
uv run scripts/iterm2_korean_control_keys.py preflight

# 개인 백업을 만들고 두 매핑 적용
uv run scripts/iterm2_korean_control_keys.py apply

# 실제 설정을 다시 읽어 검증
uv run scripts/iterm2_korean_control_keys.py verify
```

`apply`는 충돌하는 전역 또는 프로필 매핑을 발견하면 쓰기 전에 멈춥니다.
비공개 설정 이력은
`~/Library/Application Support/cli-tools/iterm2-korean-control-keys/` 아래에
기록되며 이력 디렉토리는 `0700`, 파일은 `0600` 모드를 사용합니다.

복원할 때는 glob을 사용하지 말고 `apply`가 출력한
`setting_history.json`의 절대 경로를 그대로 전달하세요:

```bash
uv run scripts/iterm2_korean_control_keys.py restore \
  --history '/absolute/path/printed-by-apply/setting_history.json'
```

복원은 해당 설정 이력에 기록된 항목만 제거하고, 관리 항목이 나중에
바뀌었다면 덮어쓰지 않고 중단합니다. 관리한 매핑이 원래 상태로 돌아가면
처음에 값이 없었던 경우까지 포함해 물리 키 설정을 원래대로 복원합니다.
그동안 관련 없는 매핑이 바뀌었다면 해당 설정은 보수적으로 활성 상태로
유지합니다.

한글 입력을 선택한 상태에서 실제 터미널 경로를 확인하세요:

```bash
swift scripts/current_macos_input_source.swift
python3 scripts/probe_control_bytes.py
```

물리 `Control-C`, 물리 `Control-G` 순서로 누르면 `PASS: 03 07`이 출력되어야
합니다. 이어서 한글 조합이 그대로 동작하는지도 확인하세요. 설계 근거와 전체
검증표는 [연구 문서](docs/research/2026-08-04-macos-korean-terminal-control-keys.md)를
참고하세요.

## Ghostty 한글 Control 키 설정 (macOS)

Ghostty `1.3.1`에서 `scripts/ghostty_korean_control_keys.py`는 Ghostty의
네이티브 `text` 액션으로 같은 PTY 바이트 매핑을 추가합니다. macOS의 모든
표준 설정 위치를 검사하고, 접두사가 있거나 다른 동작을 가진
`Control-C`/`Control-G` 충돌을 차단하며, 실제 지시문이 있는 단일 파일만
수정합니다.

이 도우미는 macOS, `/Applications/Ghostty.app`에 설치된 정확히 Ghostty
`1.3.1`, Python 3.11 이상이 필요합니다. 저장소 루트에서 실행하세요:

```bash
# 불러오는 모든 설정 파일을 검증하고 누락된 매핑만 미리 보기
python3 scripts/ghostty_korean_control_keys.py preflight

# 비공개 설정 이력을 저장하고 관리 블록 하나 추가
python3 scripts/ghostty_korean_control_keys.py apply

# Ghostty 유효 키맵과 기존 단축키 보존 여부 검증
python3 scripts/ghostty_korean_control_keys.py verify
```

`apply`는 원래 파일 모드와 확장 속성을 보존합니다.
`setting_history.json`과 `config.before`는
`~/Library/Application Support/cli-tools/ghostty-korean-control-keys/` 아래에
기록되며 이력 디렉토리는 `0700`, 파일은 `0600` 모드를 사용합니다.

복원할 때는 `apply`가 출력한 설정 이력의 절대 경로를 그대로 전달하세요:

```bash
python3 scripts/ghostty_korean_control_keys.py restore \
  --history '/absolute/path/printed-by-apply/setting_history.json'
```

복원은 정확한 관리 블록만 제거합니다. 해당 블록이 나중에 바뀌었다면
중단하고, 관련 없는 설정 변경은 보존합니다. 적용하거나 복원한 뒤 기존
Ghostty 창에서는 설정 reload 단축키를 누르세요. macOS 기본값은
`Command-Shift-,`입니다.

iTerm2 절의 입력 소스 확인 명령과 raw-byte 프로브를 Ghostty 안에서도
사용하세요. 한글 입력을 선택한 상태에서 `PASS: 03 07`이 출력되고 이후 한글
조합이 정상 동작해야 검증이 끝납니다.

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

# crontab 관리 (스케줄 검증과 함께 목록 / 추가 / 삭제 / 수정)
dev-tools crontab
dev-tools crontab add "0 9 * * 1-5" "backup.sh" -m "weekday backup"
dev-tools crontab edit 1 --schedule "@daily"
dev-tools crontab remove 1

# 날짜와 타임존
dev-tools tz                      # 세계 시계
dev-tools tz seoul                # IANA 타임존 검색
dev-tools date-diff 2026-01-01 2026-08-19
dev-tools date-diff 2026-12-25    # 오늘 기준 D-day

# 네트워크, 문서, 접근성
dev-tools subnet 192.168.1.0/24
dev-tools toc README.md --max-depth 3
dev-tools contrast "#767676" "#ffffff"
```

명령 로그 저장용 `dev-tools silent`도 계속 사용할 수 있습니다:

```bash
dev-tools silent git status --short
dev-tools silent python script.py
```

## prompt-export

Claude Code나 Codex에 입력한 프롬프트와 (선택적으로) 에이전트의 답변을
나중에 LLM으로 분석할 수 있도록 마크다운으로 내보냅니다. Claude Code 세션
로그(`~/.claude/projects`)와 Codex 롤아웃(`~/.codex/sessions`)을 읽어,
사람이 직접 입력한 프롬프트(도구 결과, 슬래시 명령 기록, 주입된 컨텍스트는
제외)와 사용자에게 보인 에이전트 텍스트만 남깁니다.

```bash
prompt-export --today                        # 오늘 두 도구에 입력한 프롬프트
prompt-export --week --source claude        # 이번 주 Claude Code 프롬프트
prompt-export --month --role all            # 이번 달 프롬프트 + 에이전트 출력
prompt-export --from 2026-08-01 --to 2026-08-07 --project cli-tools
prompt-export --week -e prompts.md          # 파일로 저장
```

- `--source claude|codex|all` 로그 소스 선택 (기본 `all`)
- `--role user|assistant|all` 내보낼 대상 선택 (기본 `user`)
- `--today` / `--week` (월요일부터) / `--month`, 또는 `--from`/`--to`에
  `YYYY-MM-DD` 날짜 지정. 기간 옵션이 없으면 전체 기록을 내보냅니다
- `--project <substring>` 프로젝트 경로가 일치하는 세션만 유지
- `-e/--export <file>` 마크다운을 stdout 대신 파일로 저장

## zzz

`zzz`는 명령 출력을 조용히 기록하는 독립 실행 명령입니다. Unix에서는 대화형
셸을 통해 실행되므로 shell alias와 function도 사용할 수 있습니다. 명령은
백그라운드에서 실행되어 프롬프트가 즉시 반환되며, macOS에서는 명령이 끝나면
성공 또는 실패 여부를 시스템 알림으로 전송합니다.

### iTerm2

iTerm2 알림은
[네이티브 OSC 9 채널](https://iterm2.com/documentation-escape-codes.html)을
사용하므로 `alerter`나 `terminal-notifier`가 필요하지 않습니다. 먼저
**Settings > Profiles > Terminal > Notification Center alerts**를 켠 뒤, `zzz`를
시험하기 전에 iTerm2 자체 알림을 확인합니다:

```bash
printf '\033]9;zzz notification test\033\\'
zzz --wait true
```

첫 명령으로 알림이 뜨지 않으면 **System Settings > Notifications > iTerm2**와
집중 모드가 알림을 막고 있지 않은지 확인하세요. `zzz`도 같은 escape sequence를
명령을 시작한 TTY에 기록하므로 `TERM_SESSION_ID`가 없거나 형식이 달라도
동작합니다.

### Terminal.app

현재 macOS에서 클릭 가능한 Terminal.app 알림을 사용하려면
[`alerter`](https://github.com/vjeantet/alerter)를 설치하고, `zzz`보다 먼저
의존성을 직접 시험합니다:

```bash
brew install vjeantet/tap/alerter
alerter --message "zzz notification test" --timeout 5
```

알림에는 Terminal.app 아이콘이 표시되며, 누르면 명령을 시작한 탭으로 키보드
포커스가 돌아갑니다. 처음 사용할 때 macOS가 Automation 및 알림 권한을 요청할
수 있습니다. `zzz`는 클릭 대기 프로세스를 분리하므로 `--wait`도 명령과 알림이
시작되면 반환됩니다. 읽지 않은 알림과 클릭 대기 프로세스는 10분 뒤 종료됩니다.

Terminal.app에서는 `alerter`가 없을 때
[`terminal-notifier`](https://github.com/julienXX/terminal-notifier)를 레거시
폴백으로 사용합니다. 두 도구가 모두 없으면 정확한 탭 포커스가 없는 일반 완료
알림으로 폴백합니다.

Ghostty는 자체 macOS 네이티브 알림 채널을 사용하므로 두 외부 알림 도구가
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
│   ├── prompt-export/  # Claude Code / Codex 프롬프트 내보내기
│   ├── work-summary/   # Git 업무 요약 분석기
│   └── zzz/            # 독립 무음 명령 로그 저장기
├── scripts/             # iTerm2/Ghostty 마이그레이션과 터미널 검사 도구
├── Cargo.toml
└── versioning.md
```

## 라이선스

MIT 라이선스입니다. [LICENSE](LICENSE)를 참고하세요.

## 저자

CHANN
