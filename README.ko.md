# CLI 도구 모음 (CLI Tools Collection)

개발자를 위한 강력한 Rust 기반 CLI 도구 모음입니다. 코드베이스 분석, 가치 산정, 업무 생산성 추적을 위해 설계되었습니다.

## 아키텍처

이 프로젝트는 모듈형 아키텍처로 구축되었습니다:

- **`cli-core`**: UI 테마, 출력 포맷팅(Table, JSON, CSV, HTML, Markdown), 설정 관리를 위한 공통 기능을 제공하는 공유 라이브러리입니다.
- **`code-cost`**: 전체 저장소를 분석하여 총 금전적 가치를 추산합니다.
- **`work-summary`**: Git 히스토리를 분석하여 최근 업무 활동과 생산성을 요약합니다.
- **`git-tools`**: 브랜치 정리, 코드 스캔 등 개발자 생산성을 위한 도구 모음입니다.
- **`dev-tools`**: 개발자들이 자주 사용하는 데이터 변환 및 시스템 확인 도구들이 포함된 "맥가이버 칼" 같은 유틸리티 모음입니다.

## 도구

### code-cost

코드 저장소를 분석하고 개발 노력, 복잡성, 프로젝트 성숙도를 기반으로 금전적 가치를 계산합니다.

#### 주요 기능

- **포괄적인 코드 분석**
  - 코드 라인 수(LOC) 및 상세 분류 (코드, 주석, 공백)
  - 언어별 가중 난이도 점수를 적용한 다국어 분석
  - LOC 및 언어 요인을 기반으로 한 순환 복잡도 추정
  - 프로젝트 성숙도 점수 (테스트, 문서화, 저장소 기간, 기여자 수)

- **Git 저장소 분석**
  - 커밋 횟수 및 히스토리
  - 기여자 분석
  - 저장소 생성 이후 기간 추적

- **금전적 가치 계산**
  - 추정 개발 시간 계산
  - 사용자 정의 가능한 시간당 단가 (기본값: ₩10,030 - 2025년 대한민국 최저임금)
  - 언어별 난이도 가중치 (Rust: 1.5x, C++: 1.4x, Go: 1.3x 등)
  - 복잡도 및 성숙도 보너스 적용
  - 사용된 기술에 대한 학습 시간 추산
  - **토큰 기반 비용 추산**: **Claude Opus 4.7 xhigh** 가격 정책($5/1M input tokens) 기준 비용 계산

- **고급 분석 기능**
  - 상세 프로젝트 지표 (복잡도, 성숙도, 코드 품질)
  - 퍼센트 단위의 언어별 비중 분석
  - **AI 사용량 추정**: 패턴 분석을 통한 AI 보조 개발 비중 추산
  - 개발자 레벨별 비용 분석 (Junior부터 Principal까지)
  - 테스트 커버리지 통계

- **다양한 출력 형식**
  - 아름다운 색상의 터미널 UI
  - 상세 분석 모드 (기본값)
  - 요약 모드 (`--simple`)
  - JSON (`json`) 및 보기 좋은 JSON (`json-pretty`)
  - **CSV**, **HTML**, **Markdown**으로 내보내기

#### 설치 방법

```bash
cargo install --path crates/code-cost
```

#### 사용법

```bash
# 기본 분석 (현재 디렉토리)
code-cost

# 특정 경로 분석
code-cost ~/projects/my-app ../other-repo

# 요약 모드 (표만 출력)
code-cost --simple

# 개발자 레벨별 분석 표시
code-cost --dev-levels

# JSON 출력
code-cost --format json-pretty

# 결과 내보내기
code-cost --export report.html
code-cost --export report.md
code-cost --export report.csv
```

---

### work-summary

Git 커밋 히스토리를 분석하여 시간 추정 및 가치 계산을 포함한 의미 있는 업무 활동 요약을 생성합니다.

#### 주요 기능

- **Git 커밋 분석**
  - Diff 추적을 포함한 상세 커밋 히스토리
  - 커밋당 파일 변경 통계
  - 작성자 및 타임스탬프 정보
  - 커밋 내 언어별 변경 사항 추적

- **하이브리드 시간 추정**
  - **시간 간격 기반**: 커밋 사이의 간격 측정 (최대 4시간 제한)
  - **코드 변경 기반**: 추가/삭제된 라인 수와 복잡도로 노력 추정
  - 높은 정확도를 위한 가중치 적용 하이브리드 알고리즘

- **업무 패턴 분석**
  - 시간대별 커밋 분포 (피크 시간대)
  - 일일 활동 추적 (가장 활발한 요일)
  - 커밋 빈도 및 활성일 비율

- **가치 계산**
  - 개발자 레벨별 추정치 (Junior부터 Principal까지)
  - 기본 시간당 단가: ₩10,030 (2025년 최저임금)
  - 커밋 규모에 따른 복잡도 조정 가치 추정

- **기여자 통계**
  - 기여자별 커밋 수 및 라인 통계
  - 기여도 백분율 분석
  - 상위 기여자 순위

- **유연한 필터링**
  - 날짜 범위: `--from`, `--to` (YYYY-MM-DD)
  - 빠른 필터: `--today`, `--week`, `--month`
  - 제한: 최근 N개의 커밋만 분석 (`--limit N`)

- **출력 옵션**
  - 상세 모드: 포괄적 분석 (기본값)
  - 요약 모드: 기본 요약 정보 (`--simple`)
  - JSON 내보내기 지원

#### 설치 방법

```bash
cargo install --path crates/work-summary
```

#### 사용법

```bash
# 최근 30일 분석 (기본값)
work-summary

# 빠른 필터
work-summary --today
work-summary --week
work-summary --month

# 특정 날짜 범위
work-summary --from 2025-01-01 --to 2025-01-31

# 커밋 수 제한
work-summary --limit 20

# 요약 모드
work-summary --simple
# JSON으로 내보내기
work-summary --export summary.json
```

---

### git-tools

개발 환경을 건강하게 유지하기 위한 개발자 상태 체크 및 편의 도구입니다.

#### 주요 기능

- **Git 브랜치 정리 (Cleanup)**
  - `main` 또는 `master` 브랜치에 이미 병합된 브랜치들을 식별합니다.
  - 실수로 인한 삭제를 방지하기 위해 기본적으로 드라이 런(Dry-run) 모드로 동작합니다.
  - `--force` 옵션을 통해 병합된 여러 브랜치를 한 번에 삭제할 수 있습니다.

- **마커 스캔 (Scan)**
  - 코드베이스에서 `TODO`, `FIXME`, `BUG`, `HACK`, `OPTIMIZE` 마커를 찾아냅니다.
  - `.gitignore` 설정을 자동으로 존중하여 스캔합니다.
  - 커맨드라인을 통해 스캔할 마커 목록을 사용자 정의할 수 있습니다.

- **프로젝트 건강도 체크 (Health)**
  - 다음과 같은 필수 프로젝트 파일의 존재 여부를 확인합니다:
    - `README.md` (문서화)
    - `LICENSE` (법적 라이선스)
    - `.gitignore` (Git 관리 위생)
    - `.git` (저장소 설정)
    - `tests/` (테스트 수트)
    - CI 설정 (Github Actions 등)
  - 건강도 점수와 함께 상세한 통과/실패 보고서를 제공합니다.

#### 설치 방법

```bash
cargo install --path crates/git-tools
```

#### 사용법

```bash
# Git 브랜치 정리
git-tools cleanup
git-tools cleanup --force
git-tools cleanup --target develop

# 마커 스캔
git-tools scan
git-tools scan --markers "TODO,DEBUG"

# 프로젝트 건강도 체크
git-tools health
git-tools health --verbose
```

---

### dev-tools

개발자들이 공통 데이터 변환 및 시스템 확인을 처리하기 위해 자주 사용하는 작고 유용한 유틸리티 모음입니다.

#### 주요 기능

- **UUID 생성**: 단일 또는 다중 UUID v4/v7 문자열을 생성합니다.
- **Base64 도구**: 문자열이나 파일을 Base64로 빠르게 인코딩 및 디코딩합니다.
- **URL 도구**: 웹 개발을 위한 URL 인코딩 및 디코딩을 수행합니다.
- **JSON 포매터**: JSON 문자열을 예쁘게 출력하거나 압축하며, 유효성 검사 및 JSONPath 쿼리를 지원합니다.
- **모델 생성**: JSON을 TypeScript 인터페이스, Rust 구조체 또는 Go 구조체로 변환합니다.
- **포트 관리자**: 특정 포트를 사용 중인 프로세스를 확인하고 선택적으로 종료할 수 있습니다.
- **해시 생성기**: 문자열이나 파일에 대해 SHA-256, MD5, SHA-1 해시를 생성합니다.
- **시간 변환기**: Unix 타임스탬프와 ISO8601 문자열 간 변환 또는 현재 시간을 확인합니다.
- **프로젝트 트리**: `.gitignore`를 존중하며 프로젝트 구조를 시각화합니다.
- **IP 정보**: 지역 정보와 함께 로컬 및 공용 IP 주소를 빠르게 확인합니다.
- **모스 부호**: 모스 부호 문자열을 인코딩 및 디코딩합니다.
- **비밀번호 도구**: 안전한 비밀번호를 생성하고 강도를 체크합니다.
- **무음 명령 실행기**: 터미널 출력 없이 포그라운드 명령을 실행하고 stdout을 `~/.commands`에 저장합니다.

#### 설치 방법

```bash
cargo install --path crates/dev-tools --force
```

#### 사용법

```bash
# UUID 생성
dev-tools uuid --count 5 --v7

# Base64 인코딩/디코딩
dev-tools base64 "hello world"
dev-tools base64 --decode "aGVsbG8gd29ybGQ="

# JSON 및 모델 생성
dev-tools json '{"a":1,"b":2}'
dev-tools rust '{"name": "test", "age": 20}'
dev-tools go '{"name": "test", "age": 20}'
dev-tools typescript '{"name": "test", "age": 20}'

# 포트 관리
dev-tools port 8080
dev-tools port 8080 --kill

# 파일 해시 및 체크섬
dev-tools hash README.md --file
dev-tools checksum README.md --file --algo sha512

# 시간 변환
dev-tools time
dev-tools time 1740000000

# 비밀번호 강도
dev-tools password --check "P@ssw0rd123"

# 모스 부호
dev-tools morse "HELLO WORLD"
dev-tools morse --decode ".... . .-.. .-.. --- / .-- --- .-. .-.. -.."

# IP 정보 확인
dev-tools ip

# 무음 명령 로그 저장
dev-tools silent echo "hello"
zzz echo "hello"
dev-tools silent git status --short
zzz git status --short
dev-tools silent python script.py

# 대화형 셸을 통해 shell alias/function도 실행할 수 있습니다
zzz update-agents

# 로그 저장 위치:
# ~/.commands/{yymmdd}/{hhmmss}-{command_name}.log
# 예: ~/.commands/260605/224512-echo.log
```

## 가치 계산 알고리즘

### Code Cost 알고리즘

1. **기본 시간**: `LOC / 20` (시간당 평균 20라인 가정).
2. **언어 가중치**: 언어 복잡도에 따른 승수 (예: Rust 1.5x, JS 1.0x).
3. **복잡도 승수**: 프로젝트 지표를 1.0x - 2.0x 범위로 매핑.
4. **성숙도 보너스**: 테스트, 문서화, 히스토리가 좋은 프로젝트에 최대 30% 보너스.
5. **학습 시간**: 프로젝트의 기술 스택을 익히는 데 필요한 추정 시간.

### Work Summary 알고리즘 (하이브리드)

1. **시간 간격 (60%)**: 커밋 사이의 실제 경과 시간을 측정하며, 긴 공백은 제한합니다.
2. **코드 변경 (40%)**: 변경량과 복잡도를 기반으로 노력을 추정합니다.
3. **복잡도 요인**: 변경된 파일 수와 총 라인 수를 기반으로 한 승수 (0.8x - 1.4x).

### 토큰 기반 비용 알고리즘 (Claude Opus 4.7 xhigh)

1. **토큰 근사치**: `characters / 3.5` (코드 분석용 휴리스틱).
2. **팽창 계수 (Inflation Factor)**: `1.35x` (Opus 4.7 xhigh 토크나이저 및 추론 노력을 반영).
3. **가격 정책**: `$5.00 / 1M tokens` (Input 기준).
4. **환율**: 지역 비용 추산을 위해 `1,400 KRW/USD` 고정 환율 적용.

## 프로젝트 구조

```
cli-tools/
├── crates/
│   ├── cli-core/           # 공통 기반 (UI, I/O, 포맷팅)
│   ├── code-cost/          # 저장소 가치 분석기
│   ├── git-tools/          # 개발자 생산성 도구
│   └── work-summary/       # Git 업무 생산성 요약기
```

## 라이선스

MIT 라이선스 - 상세 내용은 [LICENSE](LICENSE) 파일을 참조하세요.

## 저자

CHANN
