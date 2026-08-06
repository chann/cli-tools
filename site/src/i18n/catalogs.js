import {
  installAll,
  iterm2KoreanControlCommand,
  iterm2KoreanControlRestoreCommand,
  tools as commandTools,
  utilityGroups as commandGroups,
} from "../data/tools.js";

function localizeTools(items) {
  return items.map((item, index) => ({
    id: commandTools[index].id,
    name: commandTools[index].name,
    label: item.label,
    summary: item.summary,
    detail: item.detail,
    examples: commandTools[index].examples,
    output: commandTools[index].output,
  }));
}

function localizeUtilityGroups(groups) {
  return groups.map((group, index) => ({
    id: commandGroups[index].id,
    name: group.name,
    description: group.description,
    commands: commandGroups[index].commands.map((command) => ({ ...command })),
  }));
}

function createCatalog(copy) {
  return {
    ...copy,
    install: { ...copy.install, command: installAll },
    itermKeys: {
      ...copy.itermKeys,
      command: iterm2KoreanControlCommand,
      restoreCommand: iterm2KoreanControlRestoreCommand,
    },
    tools: localizeTools(copy.tools),
    utility: {
      ...copy.utility,
      groups: localizeUtilityGroups(copy.utility.groups),
    },
  };
}

const ko = createCatalog({
  meta: {
    title: "cli-tools | 반복 명령을 줄이는 Rust 도구 모음",
    description:
      "저장소 분석, Git 관리, 데이터 변환, 백그라운드 실행을 하나의 Rust workspace에서 다루는 다섯 개의 CLI 도구.",
    socialDescription:
      "분석, 정리, 변환, 백그라운드 실행을 필요한 도구만 설치해 바로 처리하세요.",
    imageAlt: "cli-tools 명령 흐름을 보여주는 제품 화면",
  },
  shell: {
    skip: "본문으로 건너뛰기",
    brandHome: "cli-tools 홈",
    navLabel: "주요 메뉴",
    navTools: "도구",
    navInstall: "설치",
    menuOpen: "메뉴 열기",
    menuClose: "메뉴 닫기",
    mobileNavLabel: "모바일 메뉴",
    exploreTools: "도구 살펴보기",
    viewGitHub: "GitHub에서 코드 보기",
    themeLabel: "테마 선택",
    themeTitle: "테마",
    themeSystem: "시스템",
    themeLight: "라이트",
    themeDark: "다크",
    languageLabel: "언어 선택",
    projectInfo: "프로젝트 정보",
    footerCopy: "CHANN의 Rust CLI 모음. MIT License.",
    footerNav: "하단 메뉴",
    privacy: "개인정보 처리 안내",
    terms: "이용 안내",
    noScript: "도구 탐색기와 복사 기능을 사용하려면 JavaScript가 필요합니다.",
  },
  hero: {
    eyebrow: "5개의 Rust CLI",
    title: ["반복 명령은 줄이고,", "만드는 일에 집중하세요."],
    summary:
      "저장소 분석부터 Git 정리, 데이터 변환, 백그라운드 실행까지. 필요한 도구만 설치해 터미널에서 바로 씁니다.",
    action: "설치 명령 보기",
    facts: ["5개 바이너리", "로그인 없음", "MIT License"],
    terminalLabel: "cli-tools의 실제 명령 실행 예시",
  },
  benefits: {
    label: "01 / 결과",
    title: "하루에 몇 번씩 하던 일을, 한 번의 명령으로.",
    description:
      "작은 작업을 위해 맥락을 바꾸는 시간을 줄이고 결과를 파일과 로그로 남깁니다.",
    items: [
      {
        title: "보고서가 남는 분석",
        description:
          "코드 규모와 Git 이력을 함께 읽고 HTML, Markdown, CSV, JSON으로 다시 씁니다.",
        command: "code-cost · work-summary",
      },
      {
        title: "변환을 한 줄로",
        description:
          "JSON, YAML, 식별자, 네트워크, 파일 작업을 브라우저를 오가지 않고 처리합니다.",
        command: "dev-tools",
      },
      {
        title: "프롬프트는 바로 복귀",
        description:
          "긴 명령은 백그라운드로 보내고 로그 경로와 완료 알림으로 결과를 놓치지 않습니다.",
        command: "zzz",
      },
      {
        title: "Git 관리도 같은 흐름으로",
        description:
          "브랜치 정리, 상태 검사, 마커 탐색, changelog 생성을 하나의 명령 체계로 묶습니다.",
        command: "git-tools",
      },
    ],
  },
  tagline: {
    lines: [
      ["터미널을", "떠나지", "않고,"],
      ["분석하고", "정리하고", "다음", "작업으로."],
    ],
  },
  install: {
    label: "02 / 설치",
    title: "필요한 도구만 설치하세요.",
    description: "저장소를 복제한 뒤 원하는 바이너리만 골라 설치합니다.",
    codeLabel: "로컬 경로에서 설치",
    buildLabel: "전체 빌드",
    buildAria: "전체 빌드 명령 코드",
    testLabel: "전체 테스트",
    testAria: "전체 테스트 명령 코드",
  },
  explorer: {
    label: "03 / 도구 모음",
    title: "5가지 재미있는 도구, 그리고 실용성까지.",
    description:
      "도구를 선택하면 실제 옵션과 바로 실행할 수 있는 예시를 확인할 수 있습니다.",
    tabsLabel: "CLI 도구 선택",
    exampleSuffix: "예시",
    outputLabel: "출력",
  },
  tools: [
    {
      label: "저장소 가치 측정",
      summary: "코드 규모와 Git 이력을 함께 읽어 개발 비용과 프로젝트 가치를 추정합니다.",
      detail: "LOC, 언어 난이도, 복잡도, 성숙도, 기여자 지표를 표와 파일로 확인할 수 있습니다.",
    },
    {
      label: "Git 작업 요약",
      summary: "커밋 기록을 기간별로 묶어 활동, 예상 작업 시간, 기여 가치를 요약합니다.",
      detail: "오늘, 이번 주, 이번 달 필터와 직접 지정한 날짜 범위를 모두 지원합니다.",
    },
    {
      label: "Git 유지보수",
      summary: "브랜치 정리, 마커 스캔, 건강도, changelog, commit 흐름을 한곳에 모읍니다.",
      detail: "반복되는 저장소 점검을 하위 명령으로 나누어 필요한 검사만 빠르게 실행합니다.",
    },
    {
      label: "개발자 유틸리티",
      summary: "JSON, 인코딩, 네트워크, 텍스트, 시스템 작업을 짧은 하위 명령으로 제공합니다.",
      detail: "작은 웹 도구를 찾거나 일회성 스크립트를 작성하는 대신 터미널에서 바로 처리합니다.",
    },
    {
      label: "백그라운드 실행",
      summary: "대화형 셸로 명령을 백그라운드 실행하고 출력을 날짜별 로그에 저장합니다.",
      detail: "프롬프트는 즉시 돌려받고, macOS에서는 완료 알림과 원래 터미널 포커스를 지원합니다.",
    },
  ],
  itermKeys: {
    label: "04 / macOS · iTerm2",
    title: "한글 입력은 그대로, 터미널 단축키도 그대로.",
    description:
      "iTerm2 3.6.11에서 물리 Control-C와 Control-G를 입력 언어와 무관한 PTY 바이트로 연결합니다. 입력 소스를 바꾸거나 상주 키보드 도구를 설치하지 않습니다.",
    mappingLabel: "물리 키와 PTY 바이트 매핑",
    physicalLabel: "물리 키",
    byteLabel: "PTY 바이트",
    mappingNote: "iTerm2가 한글 조합 문자 대신 물리 키 위치를 읽고 정확한 제어 바이트를 전송합니다.",
    safeguards: [
      { title: "입력 유지", description: "한글 입력 소스를 ABC로 전환하지 않고 iTerm2 안에서만 동작합니다." },
      { title: "충돌 차단", description: "전역 및 프로필 매핑을 먼저 검사하고 다른 동작과 겹치면 변경 전에 멈춥니다." },
      { title: "정확한 복원", description: "개인 백업 영수증으로 소유한 두 항목만 제거하고 이후 사용자 변경은 덮어쓰지 않습니다." },
    ],
    codeLabel: "iTerm2 한글 단축키 설정",
    restoreCodeLabel: "영수증으로 복원",
    note: "uv와 iTerm2 Python API 승인이 필요합니다. apply가 출력한 개인 영수증의 절대 경로만 restore에 사용하세요.",
  },
  zzz: {
    label: "05 / 백그라운드",
    title: "명령은 백그라운드로. 결과는 로그로.",
    descriptionBefore: "는 프롬프트를 바로 돌려주고 완료 여부를 알림으로 알려줍니다.",
    behaviors: [
      { title: "즉시 복귀", description: "대화형 셸의 alias와 function을 그대로 사용합니다." },
      { title: "로그 저장", description: "출력을 날짜와 시간 기준 경로에 자동으로 남깁니다." },
      {
        title: "macOS 알림",
        description: "alerter를 설치하면 원래 iTerm2 세션이나 Terminal 탭으로 돌아갑니다.",
      },
    ],
    codeLabel: "zzz 시작하기",
    note: "alerter가 없으면 terminal-notifier 또는 일반 완료 알림으로 폴백합니다.",
    previewLabel: "zzz 실행 흐름 예시",
  },
  utility: {
    label: "06 / 유틸리티",
    title: "반복 작업을 한 명령으로.",
    descriptionBefore: "는 데이터, 네트워크, 코드, 파일 작업을 주제별 하위 명령으로 묶습니다.",
    tabsLabel: "dev-tools 주제",
    commandSuffix: "명령 선택",
    exampleSuffix: "예시",
    groups: [
      { name: "데이터 형식", description: "구조화 데이터를 검사하고 서로 변환합니다." },
      { name: "ID와 보안", description: "식별자와 개발용 보안 값을 생성하고 검사합니다." },
      { name: "네트워크", description: "주소, 포트, DNS, 인증서를 터미널에서 확인합니다." },
      { name: "텍스트와 코드", description: "텍스트를 가공하고 JSON에서 타입을 생성합니다." },
      { name: "파일과 시스템", description: "로컬 파일, 프로세스, 시스템 정보를 빠르게 다룹니다." },
    ],
  },
  analysis: {
    label: "07 / 분석",
    title: "저장소를 읽고, 숫자로 남기세요.",
    description: "코드와 Git 이력을 같은 기준으로 읽어 규모와 작업 흐름을 설명합니다.",
    items: [
      {
        name: "code-cost",
        description: "언어별 코드량, 난이도, 복잡도, 성숙도를 분석해 비용과 가치를 추정합니다.",
        command: "code-cost --export report.html",
        aria: "code-cost 분석 명령 코드",
      },
      {
        name: "work-summary",
        description: "커밋 간격과 변경량을 바탕으로 기간별 활동과 예상 작업 시간을 요약합니다.",
        command: "work-summary --month",
        aria: "work-summary 분석 명령 코드",
      },
      {
        name: "git-tools",
        description: "분석 뒤에는 health, scan, changelog로 저장소 상태를 바로 정리합니다.",
        command: "git-tools pulse",
        aria: "git-tools 분석 명령 코드",
      },
    ],
  },
  workflow: {
    label: "08 / 시작 흐름",
    title: "복제하고, 고르고, 바로 확인합니다.",
    description: "별도 계정이나 설정 화면 없이 익숙한 Cargo 흐름으로 시작합니다.",
    steps: [
      {
        number: "01",
        title: "필요한 바이너리를 고릅니다",
        description: "분석, Git 관리, 변환, 백그라운드 실행 중 지금 필요한 도구만 선택합니다.",
      },
      {
        number: "02",
        title: "로컬 소스에서 설치합니다",
        description: "저장소를 복제하고 cargo install 명령으로 선택한 crate를 설치합니다.",
      },
      {
        number: "03",
        title: "도움말과 테스트로 확인합니다",
        description: "각 명령의 도움말을 확인하고 전체 workspace 테스트로 함께 검증할 수 있습니다.",
      },
    ],
  },
  faq: {
    label: "09 / 자주 묻는 질문",
    title: "설치 전에 궁금한 점.",
    description: "도구 선택부터 데이터 처리 방식까지 먼저 확인하세요.",
    items: [
      {
        question: "다섯 도구를 모두 설치해야 하나요?",
        answer: "아닙니다. 각 바이너리는 독립 설치 대상입니다. 지금 필요한 crate의 cargo install 명령만 실행하면 됩니다.",
      },
      {
        question: "공식 설치 경로는 무엇인가요?",
        answer: "현재는 GitHub 저장소를 복제한 뒤 로컬 경로에서 설치합니다. 설치 섹션의 명령을 그대로 복사할 수 있습니다.",
      },
      {
        question: "명령이 데이터를 외부로 전송하나요?",
        answer: "분석과 변환은 기본적으로 로컬에서 실행됩니다. DNS, HTTP, 날씨처럼 네트워크가 필요한 명령만 요청한 대상에 연결합니다.",
      },
      {
        question: "zzz는 어떤 환경에서 유용한가요?",
        answer: "긴 빌드와 테스트를 자주 돌리는 Unix 셸에서 유용합니다. macOS에서는 완료 알림과 원래 터미널 세션 복귀도 지원합니다.",
      },
      {
        question: "업데이트와 삭제는 어떻게 하나요?",
        answer: "최신 소스를 받은 뒤 같은 cargo install 명령에 force 옵션을 사용해 갱신합니다. 삭제할 때는 cargo uninstall과 바이너리 이름을 사용합니다.",
      },
      {
        question: "비용이나 계정이 필요한가요?",
        answer: "필요하지 않습니다. cli-tools는 MIT License로 공개되어 있으며 사이트에도 가입이나 결제 단계가 없습니다.",
      },
    ],
  },
  final: {
    label: "오픈 소스 / MIT",
    title: "필요한 도구부터 설치하세요.",
    description: "무료로 공개된 Rust workspace입니다. 로그인 없이 원하는 바이너리만 고르면 됩니다.",
    risk: "무료 · 로그인 없음 · MIT License",
    action: "설치 명령 보기",
  },
  ui: {
    copy: "복사",
    copying: "복사 중",
    copied: "복사됨",
    copyError: "복사 실패",
    copiedStatus: "클립보드에 복사했습니다.",
    copyErrorStatus: "브라우저의 클립보드 권한을 확인해 주세요.",
    codeSuffix: "코드",
  },
  legal: {
    updated: "2026.08.03 업데이트",
    navLabel: "법적 안내 메뉴",
    backHome: "홈으로 돌아가기",
    privacy: {
      title: "개인정보 처리 안내",
      description: "cli-tools 웹사이트의 데이터 처리 방식과 저장 항목 안내.",
      intro: "cli-tools 웹사이트는 가입, 결제, 문의 양식을 운영하지 않으며 방문자를 식별하는 자체 분석 도구를 사용하지 않습니다.",
      sections: [
        {
          title: "사이트가 저장하는 항목",
          body: "테마와 언어 선택은 현재 브라우저의 localStorage에만 저장됩니다. 이 값은 서버로 전송되지 않으며 브라우저 설정에서 언제든 삭제할 수 있습니다.",
        },
        {
          title: "명령어 복사",
          body: "복사 버튼은 브라우저 Clipboard API를 사용합니다. 복사한 명령과 클립보드 내용은 cli-tools 웹사이트로 전송되지 않습니다.",
        },
        {
          title: "호스팅과 외부 링크",
          body: "사이트는 GitHub Pages에서 제공됩니다. GitHub는 서비스 운영에 필요한 요청 정보를 처리할 수 있습니다.",
        },
        {
          title: "변경 사항",
          body: "데이터 처리 방식이 달라지면 이 페이지의 설명과 업데이트 날짜를 함께 수정합니다.",
        },
      ],
    },
    terms: {
      title: "이용 안내",
      description: "cli-tools 소프트웨어와 웹사이트 이용 안내.",
      intro: "cli-tools는 개발자가 자신의 환경에서 설치하고 실행하는 오픈 소스 Rust CLI 모음입니다.",
      sections: [
        {
          title: "소프트웨어 라이선스",
          body: "소스 코드와 바이너리 사용 조건은 저장소의 MIT License를 따릅니다. 사용, 복사, 수정, 배포 시 해당 라이선스 고지를 유지해 주세요.",
        },
        {
          title: "설치와 실행",
          body: "사용자는 각 명령의 도움말과 실행 대상을 확인할 책임이 있습니다. 시스템 상태에 영향을 주는 옵션은 자신의 환경에서 검토한 뒤 실행해 주세요.",
        },
        {
          title: "보증 범위",
          body: "소프트웨어는 MIT License에 명시된 대로 제공됩니다. 특정 목적에 대한 적합성, 중단 없는 동작, 결과의 완전성을 별도로 보증하지 않습니다.",
        },
        {
          title: "비용과 계정",
          body: "이 웹사이트에는 가입이나 결제 단계가 없습니다. 외부 서비스와 연결하는 명령의 비용과 정책은 해당 서비스의 조건을 따릅니다.",
        },
      ],
    },
  },
  notFound: {
    navLabel: "오류 페이지 메뉴",
    title: "요청한 페이지를 찾을 수 없습니다.",
    description: "요청한 cli-tools 페이지를 찾을 수 없습니다.",
    intro: "주소를 다시 확인하거나 도구 안내 첫 화면으로 돌아가세요.",
    action: "cli-tools 홈으로 돌아가기",
  },
});

const en = createCatalog({
  meta: {
    title: "cli-tools | Rust utilities for repetitive commands",
    description:
      "Five Rust CLI tools for repository analysis, Git maintenance, data conversion, and background execution in one workspace.",
    socialDescription:
      "Install only the tools you need for analysis, cleanup, conversion, and background tasks.",
    imageAlt: "Product view showing the cli-tools command workflow",
  },
  shell: {
    skip: "Skip to main content",
    brandHome: "cli-tools home",
    navLabel: "Primary navigation",
    navTools: "Tools",
    navInstall: "Install",
    menuOpen: "Open menu",
    menuClose: "Close menu",
    mobileNavLabel: "Mobile navigation",
    exploreTools: "Explore tools",
    viewGitHub: "View code on GitHub",
    themeLabel: "Choose theme",
    themeTitle: "Theme",
    themeSystem: "System",
    themeLight: "Light",
    themeDark: "Dark",
    languageLabel: "Choose language",
    projectInfo: "Project information",
    footerCopy: "Rust CLI tools by CHANN. MIT License.",
    footerNav: "Footer navigation",
    privacy: "Privacy",
    terms: "Terms",
    noScript: "JavaScript is required for the tool explorer and copy controls.",
  },
  hero: {
    eyebrow: "5 Rust CLIs",
    title: ["Cut the repetitive commands.", "Focus on what you build."],
    summary:
      "Analyze repositories, maintain Git, convert data, and run background tasks. Install only what you need and use it directly in your terminal.",
    action: "View install commands",
    facts: ["5 binaries", "No account", "MIT License"],
    terminalLabel: "Real cli-tools command examples",
  },
  benefits: {
    label: "01 / OUTCOMES",
    title: "Turn the work you repeat every day into one command.",
    description: "Switch context less often and leave useful results in files and logs.",
    items: [
      {
        title: "Analysis that leaves a report",
        description: "Read code size and Git history together, then export HTML, Markdown, CSV, or JSON.",
        command: "code-cost · work-summary",
      },
      {
        title: "Convert in one line",
        description: "Handle JSON, YAML, identifiers, networks, and files without bouncing between browser tools.",
        command: "dev-tools",
      },
      {
        title: "Get your prompt back",
        description: "Send long commands to the background and keep their log path and completion notice.",
        command: "zzz",
      },
      {
        title: "Keep Git in the same flow",
        description: "Bring branch cleanup, health checks, marker scans, and changelogs into one command family.",
        command: "git-tools",
      },
    ],
  },
  tagline: {
    lines: [
      ["Stay in", "the", "terminal."],
      ["Analyze,", "organize,", "and keep", "building."],
    ],
  },
  install: {
    label: "02 / INSTALL",
    title: "Install only the tools you need.",
    description: "Clone the repository, then choose the binaries you want to install.",
    codeLabel: "Install from a local path",
    buildLabel: "Build everything",
    buildAria: "Full build command code",
    testLabel: "Test everything",
    testAria: "Full test command code",
  },
  explorer: {
    label: "03 / TOOLKIT",
    title: "Five tools. One workflow.",
    description: "Choose a tool to see real options and examples you can run immediately.",
    tabsLabel: "Choose a CLI tool",
    exampleSuffix: "example",
    outputLabel: "Output",
  },
  tools: [
    {
      label: "Repository value estimate",
      summary: "Read code size and Git history together to estimate development cost and project value.",
      detail: "Inspect LOC, language difficulty, complexity, maturity, and contributor metrics in tables and files.",
    },
    {
      label: "Git work summary",
      summary: "Group commit history by period to summarize activity, estimated work time, and contribution value.",
      detail: "Filter by today, this week, this month, or a date range you provide.",
    },
    {
      label: "Git maintenance",
      summary: "Bring branch cleanup, marker scans, health, changelogs, and commit flows into one place.",
      detail: "Split repeated repository checks into subcommands so you run only the inspection you need.",
    },
    {
      label: "Developer utilities",
      summary: "Use short subcommands for JSON, encoding, network, text, and system tasks.",
      detail: "Handle small jobs directly in the terminal instead of finding a web tool or writing a disposable script.",
    },
    {
      label: "Background execution",
      summary: "Run commands in the background through your interactive shell and save output in dated logs.",
      detail: "Get the prompt back immediately, with completion notifications and terminal focus support on macOS.",
    },
  ],
  itermKeys: {
    label: "04 / macOS · iTerm2",
    title: "Keep Korean input. Keep terminal shortcuts.",
    description:
      "On iTerm2 3.6.11, map physical Control-C and Control-G to language-independent PTY bytes. The helper neither switches input sources nor installs a resident keyboard tool.",
    mappingLabel: "Physical keys mapped to PTY bytes",
    physicalLabel: "Physical key",
    byteLabel: "PTY byte",
    mappingNote: "iTerm2 reads the physical key position instead of a composed Korean character and sends the exact control byte.",
    safeguards: [
      { title: "Input stays put", description: "Keep Korean selected and scope the behavior to iTerm2 instead of switching to ABC." },
      { title: "Conflicts stop", description: "Audit global and profile mappings first, then stop before changing anything if another action overlaps." },
      { title: "Exact restore", description: "Use a private ownership receipt to remove only the two managed entries without overwriting later user edits." },
    ],
    codeLabel: "Configure Korean control keys in iTerm2",
    restoreCodeLabel: "Restore from a receipt",
    note: "Requires uv and iTerm2 Python API approval. Restore only with the absolute private receipt path printed by apply.",
  },
  zzz: {
    label: "05 / BACKGROUND",
    title: "Commands in the background. Results in logs.",
    descriptionBefore: " returns your prompt immediately and notifies you when the command finishes.",
    behaviors: [
      { title: "Immediate return", description: "Keep using aliases and functions from your interactive shell." },
      { title: "Saved logs", description: "Store output automatically in paths organized by date and time." },
      { title: "macOS alerts", description: "With alerter installed, return to the original iTerm2 session or Terminal tab." },
    ],
    codeLabel: "Get started with zzz",
    note: "Without alerter, zzz falls back to terminal-notifier or a standard completion alert.",
    previewLabel: "Example zzz execution flow",
  },
  utility: {
    label: "06 / UTILITIES",
    title: "Make repetitive tasks one command.",
    descriptionBefore: " groups data, network, code, and file work into focused subcommands.",
    tabsLabel: "dev-tools topics",
    commandSuffix: "command selection",
    exampleSuffix: "example",
    groups: [
      { name: "Data formats", description: "Validate structured data and convert between formats." },
      { name: "IDs and security", description: "Generate and inspect identifiers and development security values." },
      { name: "Network", description: "Inspect addresses, ports, DNS, and certificates from the terminal." },
      { name: "Text and code", description: "Transform text and generate types from JSON." },
      { name: "Files and system", description: "Work quickly with local files, processes, and system information." },
    ],
  },
  analysis: {
    label: "07 / ANALYSIS",
    title: "Read the repository. Keep the numbers.",
    description: "Use the same frame for code and Git history to explain project size and work patterns.",
    items: [
      { name: "code-cost", description: "Analyze code volume, difficulty, complexity, and maturity by language to estimate cost and value.", command: "code-cost --export report.html", aria: "code-cost analysis command code" },
      { name: "work-summary", description: "Summarize activity and estimated work time from commit intervals and change volume.", command: "work-summary --month", aria: "work-summary analysis command code" },
      { name: "git-tools", description: "After analysis, use health, scan, and changelog to organize repository state.", command: "git-tools pulse", aria: "git-tools analysis command code" },
    ],
  },
  workflow: {
    label: "08 / GET STARTED",
    title: "Clone, choose, and verify.",
    description: "Start with the Cargo workflow you know, without a separate account or settings screen.",
    steps: [
      { number: "01", title: "Choose the binaries you need", description: "Pick only the analysis, Git, conversion, or background tools you need now." },
      { number: "02", title: "Install from local source", description: "Clone the repository and install each selected crate with cargo install." },
      { number: "03", title: "Check help and tests", description: "Read each command's help and verify the complete workspace with its test suite." },
    ],
  },
  faq: {
    label: "09 / FAQ",
    title: "Questions before you install.",
    description: "Check tool selection and data behavior before you begin.",
    items: [
      { question: "Do I need to install all five tools?", answer: "No. Each binary installs independently. Run cargo install only for the crate you need now." },
      { question: "What is the official installation path?", answer: "Clone the GitHub repository and install from the local path. You can copy the exact commands from the install section." },
      { question: "Do commands send data elsewhere?", answer: "Analysis and conversion run locally by default. Only commands that require a network, such as DNS, HTTP, or weather, connect to the target you request." },
      { question: "Where is zzz most useful?", answer: "It is useful in Unix shells where long builds and tests run often. On macOS it also supports completion alerts and returning to the original terminal session." },
      { question: "How do I update or uninstall?", answer: "Pull the latest source and add the force option to the same cargo install command. Use cargo uninstall with the binary name to remove it." },
      { question: "Do I need an account or payment?", answer: "No. cli-tools is available under the MIT License, and the site has no sign-up or payment step." },
    ],
  },
  final: {
    label: "OPEN SOURCE / MIT",
    title: "Install the tool you need first.",
    description: "A free, open Rust workspace. Choose any binary without creating an account.",
    risk: "Free · No account · MIT License",
    action: "View install commands",
  },
  ui: {
    copy: "Copy",
    copying: "Copying",
    copied: "Copied",
    copyError: "Copy failed",
    copiedStatus: "Copied to the clipboard.",
    copyErrorStatus: "Check your browser's clipboard permission.",
    codeSuffix: "code",
  },
  legal: {
    updated: "Updated 2026-08-03",
    navLabel: "Legal navigation",
    backHome: "Back to home",
    privacy: {
      title: "Privacy notice",
      description: "How the cli-tools website handles data and stored preferences.",
      intro: "The cli-tools website has no sign-up, payment, or contact forms and uses no first-party analytics that identify visitors.",
      sections: [
        { title: "What this site stores", body: "Theme and language choices are stored only in this browser's localStorage. They are not sent to a server and can be removed in your browser settings." },
        { title: "Copying commands", body: "Copy buttons use the browser Clipboard API. Copied commands and clipboard contents are not sent to the cli-tools website." },
        { title: "Hosting and external links", body: "The site is served by GitHub Pages. GitHub may process request information needed to operate the service." },
        { title: "Changes", body: "If data handling changes, this page and its update date will be revised together." },
      ],
    },
    terms: {
      title: "Terms of use",
      description: "Terms for using cli-tools software and this website.",
      intro: "cli-tools is a collection of open-source Rust CLIs that developers install and run in their own environments.",
      sections: [
        { title: "Software license", body: "Use of the source and binaries follows the repository's MIT License. Keep the license notice when using, copying, modifying, or distributing the software." },
        { title: "Installation and execution", body: "You are responsible for reviewing each command's help and target. Review options that affect system state in your own environment before running them." },
        { title: "Warranty", body: "The software is provided as stated in the MIT License. Fitness for a particular purpose, uninterrupted operation, and complete results are not separately warranted." },
        { title: "Cost and accounts", body: "This website has no sign-up or payment step. Costs and policies for commands that connect to external services follow those services' terms." },
      ],
    },
  },
  notFound: {
    navLabel: "Error page navigation",
    title: "We could not find that page.",
    description: "The requested cli-tools page could not be found.",
    intro: "Check the address or return to the cli-tools guide.",
    action: "Return to cli-tools home",
  },
});

const ja = createCatalog({
  meta: {
    title: "cli-tools | 繰り返しコマンドを減らす Rust ツール集",
    description: "リポジトリ分析、Git 管理、データ変換、バックグラウンド実行を一つの Rust workspace で扱う5つの CLI ツール。",
    socialDescription: "分析、整理、変換、バックグラウンド実行に必要なツールだけをインストールして、すぐに使えます。",
    imageAlt: "cli-tools のコマンドフローを示す製品画面",
  },
  shell: {
    skip: "本文へ移動",
    brandHome: "cli-tools ホーム",
    navLabel: "メインメニュー",
    navTools: "ツール",
    navInstall: "インストール",
    menuOpen: "メニューを開く",
    menuClose: "メニューを閉じる",
    mobileNavLabel: "モバイルメニュー",
    exploreTools: "ツールを見る",
    viewGitHub: "GitHub でコードを見る",
    themeLabel: "テーマを選択",
    themeTitle: "テーマ",
    themeSystem: "システム",
    themeLight: "ライト",
    themeDark: "ダーク",
    languageLabel: "言語を選択",
    projectInfo: "プロジェクト情報",
    footerCopy: "CHANN の Rust CLI コレクション。MIT License。",
    footerNav: "フッターメニュー",
    privacy: "プライバシー",
    terms: "利用案内",
    noScript: "ツール選択とコピー機能には JavaScript が必要です。",
  },
  hero: {
    eyebrow: "5つの Rust CLI",
    title: ["繰り返しのコマンドを減らし、", "作ることに集中しましょう。"],
    summary: "リポジトリ分析から Git 整理、データ変換、バックグラウンド実行まで。必要なツールだけを選び、ターミナルですぐに使えます。",
    action: "インストールコマンドを見る",
    facts: ["5つのバイナリ", "アカウント不要", "MIT License"],
    terminalLabel: "cli-tools の実際のコマンド実行例",
  },
  benefits: {
    label: "01 / 成果",
    title: "毎日繰り返す作業を、一つのコマンドに。",
    description: "小さな作業のためのコンテキスト切り替えを減らし、結果をファイルとログに残します。",
    items: [
      { title: "レポートが残る分析", description: "コード規模と Git 履歴を一緒に読み、HTML、Markdown、CSV、JSON へ書き出します。", command: "code-cost · work-summary" },
      { title: "変換を一行で", description: "JSON、YAML、識別子、ネットワーク、ファイル操作をブラウザに移動せず処理します。", command: "dev-tools" },
      { title: "プロンプトへすぐ復帰", description: "長いコマンドをバックグラウンドへ送り、ログの場所と完了通知を受け取れます。", command: "zzz" },
      { title: "Git 管理も同じ流れで", description: "ブランチ整理、状態確認、マーカー検索、changelog 作成を一つのコマンド体系にまとめます。", command: "git-tools" },
    ],
  },
  tagline: {
    lines: [
      ["ターミナルを", "離れず", "に、"],
      ["分析して、", "整理して、", "次の", "作業へ。"],
    ],
  },
  install: {
    label: "02 / インストール",
    title: "必要なツールだけをインストール。",
    description: "リポジトリを複製し、使いたいバイナリを選んでインストールします。",
    codeLabel: "ローカルパスからインストール",
    buildLabel: "すべてをビルド",
    buildAria: "全体ビルドコマンドのコード",
    testLabel: "すべてをテスト",
    testAria: "全体テストコマンドのコード",
  },
  explorer: {
    label: "03 / ツール集",
    title: "5つのツール、一つの作業フロー。",
    description: "ツールを選ぶと、実際のオプションとすぐ実行できる例を確認できます。",
    tabsLabel: "CLI ツールを選択",
    exampleSuffix: "の例",
    outputLabel: "出力",
  },
  tools: [
    { label: "リポジトリ価値の測定", summary: "コード規模と Git 履歴を一緒に読み、開発費用とプロジェクト価値を推定します。", detail: "LOC、言語難易度、複雑度、成熟度、貢献者指標を表やファイルで確認できます。" },
    { label: "Git 作業の要約", summary: "コミット履歴を期間別にまとめ、活動、推定作業時間、貢献価値を要約します。", detail: "今日、今週、今月のフィルターと指定した日付範囲に対応します。" },
    { label: "Git メンテナンス", summary: "ブランチ整理、マーカー検索、健全性、changelog、commit の流れを一か所にまとめます。", detail: "繰り返すリポジトリ確認をサブコマンドに分け、必要な検査だけをすばやく実行します。" },
    { label: "開発者ユーティリティ", summary: "JSON、エンコード、ネットワーク、テキスト、システム作業を短いサブコマンドで提供します。", detail: "小さな Web ツールを探したり一時的なスクリプトを書いたりせず、ターミナルで処理します。" },
    { label: "バックグラウンド実行", summary: "対話型シェルでコマンドをバックグラウンド実行し、出力を日付別ログに保存します。", detail: "プロンプトはすぐ戻り、macOS では完了通知と元のターミナルへの復帰を支援します。" },
  ],
  itermKeys: {
    label: "04 / macOS · iTerm2",
    title: "韓国語入力のまま、ターミナルショートカットも使う。",
    description: "iTerm2 3.6.11 で、物理 Control-C と Control-G を入力言語に依存しない PTY バイトへ割り当てます。入力ソースの切り替えや常駐キーボードツールは不要です。",
    mappingLabel: "物理キーと PTY バイトの割り当て",
    physicalLabel: "物理キー",
    byteLabel: "PTY バイト",
    mappingNote: "iTerm2 が韓国語の組み立て文字ではなく物理キーの位置を読み、正確な制御バイトを送信します。",
    safeguards: [
      { title: "入力を維持", description: "韓国語を選択したまま、ABC へ切り替えず iTerm2 内だけで動作します。" },
      { title: "競合で停止", description: "グローバルとプロファイルの割り当てを先に検査し、別の動作と重なる場合は変更前に停止します。" },
      { title: "正確に復元", description: "非公開の所有権レシートで管理対象の2項目だけを削除し、後から行ったユーザー変更は上書きしません。" },
    ],
    codeLabel: "iTerm2 の韓国語コントロールキー設定",
    restoreCodeLabel: "レシートから復元",
    note: "uv と iTerm2 Python API の許可が必要です。apply が表示した非公開レシートの絶対パスだけを restore に使用してください。",
  },
  zzz: {
    label: "05 / バックグラウンド",
    title: "コマンドはバックグラウンドへ。結果はログへ。",
    descriptionBefore: "はプロンプトをすぐ戻し、コマンドの完了を通知します。",
    behaviors: [
      { title: "すぐに復帰", description: "対話型シェルの alias と function をそのまま使えます。" },
      { title: "ログを保存", description: "出力を日付と時刻ごとのパスに自動保存します。" },
      { title: "macOS 通知", description: "alerter を入れると、元の iTerm2 セッションや Terminal タブへ戻れます。" },
    ],
    codeLabel: "zzz を始める",
    note: "alerter がない場合は terminal-notifier または標準の完了通知にフォールバックします。",
    previewLabel: "zzz の実行フロー例",
  },
  utility: {
    label: "06 / ユーティリティ",
    title: "繰り返し作業を一つのコマンドに。",
    descriptionBefore: "はデータ、ネットワーク、コード、ファイル操作をテーマ別サブコマンドにまとめます。",
    tabsLabel: "dev-tools のテーマ",
    commandSuffix: "のコマンド選択",
    exampleSuffix: "の例",
    groups: [
      { name: "データ形式", description: "構造化データを検査し、形式を相互変換します。" },
      { name: "ID とセキュリティ", description: "識別子と開発用のセキュリティ値を生成して検査します。" },
      { name: "ネットワーク", description: "アドレス、ポート、DNS、証明書をターミナルで確認します。" },
      { name: "テキストとコード", description: "テキストを加工し、JSON から型を生成します。" },
      { name: "ファイルとシステム", description: "ローカルファイル、プロセス、システム情報をすばやく扱います。" },
    ],
  },
  analysis: {
    label: "07 / 分析",
    title: "リポジトリを読み、数字に残す。",
    description: "コードと Git 履歴を同じ基準で読み、規模と作業の流れを説明します。",
    items: [
      { name: "code-cost", description: "言語別のコード量、難易度、複雑度、成熟度を分析し、費用と価値を推定します。", command: "code-cost --export report.html", aria: "code-cost 分析コマンドのコード" },
      { name: "work-summary", description: "コミット間隔と変更量から、期間別の活動と推定作業時間を要約します。", command: "work-summary --month", aria: "work-summary 分析コマンドのコード" },
      { name: "git-tools", description: "分析後は health、scan、changelog でリポジトリの状態を整理します。", command: "git-tools pulse", aria: "git-tools 分析コマンドのコード" },
    ],
  },
  workflow: {
    label: "08 / はじめ方",
    title: "複製して、選んで、すぐ確認。",
    description: "別のアカウントや設定画面なしで、使い慣れた Cargo の流れから始めます。",
    steps: [
      { number: "01", title: "必要なバイナリを選ぶ", description: "分析、Git 管理、変換、バックグラウンド実行から今必要なツールだけを選びます。" },
      { number: "02", title: "ローカルソースから入れる", description: "リポジトリを複製し、cargo install で選んだ crate をインストールします。" },
      { number: "03", title: "ヘルプとテストで確認", description: "各コマンドのヘルプを読み、workspace 全体のテストでまとめて検証できます。" },
    ],
  },
  faq: {
    label: "09 / よくある質問",
    title: "インストール前の疑問。",
    description: "ツールの選び方からデータの扱いまで、先に確認できます。",
    items: [
      { question: "5つのツールをすべて入れる必要がありますか？", answer: "いいえ。各バイナリは個別にインストールできます。今必要な crate の cargo install だけを実行してください。" },
      { question: "正式なインストール方法は？", answer: "GitHub リポジトリを複製し、ローカルパスからインストールします。インストール欄のコマンドをそのままコピーできます。" },
      { question: "コマンドはデータを外部送信しますか？", answer: "分析と変換は基本的にローカルで動きます。DNS、HTTP、天気などネットワークが必要なコマンドだけが指定先へ接続します。" },
      { question: "zzz はどの環境で便利ですか？", answer: "長いビルドやテストをよく実行する Unix シェルで便利です。macOS では完了通知と元のターミナルセッションへの復帰にも対応します。" },
      { question: "更新と削除の方法は？", answer: "最新ソースを取得し、同じ cargo install に force オプションを付けて更新します。削除には cargo uninstall とバイナリ名を使います。" },
      { question: "費用やアカウントは必要ですか？", answer: "必要ありません。cli-tools は MIT License で公開され、サイトに登録や支払いの手順はありません。" },
    ],
  },
  final: {
    label: "オープンソース / MIT",
    title: "必要なツールから始めましょう。",
    description: "無料で公開された Rust workspace です。ログインせず、必要なバイナリだけを選べます。",
    risk: "無料 · アカウント不要 · MIT License",
    action: "インストールコマンドを見る",
  },
  ui: {
    copy: "コピー",
    copying: "コピー中",
    copied: "コピー済み",
    copyError: "コピー失敗",
    copiedStatus: "クリップボードにコピーしました。",
    copyErrorStatus: "ブラウザのクリップボード権限を確認してください。",
    codeSuffix: "コード",
  },
  legal: {
    updated: "2026.08.03 更新",
    navLabel: "法的案内メニュー",
    backHome: "ホームへ戻る",
    privacy: {
      title: "プライバシー案内",
      description: "cli-tools Web サイトのデータ処理と保存項目について。",
      intro: "cli-tools Web サイトには登録、決済、問い合わせフォームがなく、訪問者を識別する独自の分析ツールも使用しません。",
      sections: [
        { title: "サイトが保存する項目", body: "テーマと言語の選択は、このブラウザの localStorage にだけ保存されます。サーバーには送信されず、ブラウザ設定からいつでも削除できます。" },
        { title: "コマンドのコピー", body: "コピーボタンはブラウザの Clipboard API を使います。コピーしたコマンドやクリップボードの内容はサイトへ送信されません。" },
        { title: "ホスティングと外部リンク", body: "サイトは GitHub Pages で提供されます。GitHub はサービス運営に必要なリクエスト情報を処理することがあります。" },
        { title: "変更", body: "データ処理方法が変わった場合は、このページの説明と更新日を同時に修正します。" },
      ],
    },
    terms: {
      title: "利用案内",
      description: "cli-tools ソフトウェアと Web サイトの利用案内。",
      intro: "cli-tools は、開発者が自分の環境にインストールして実行するオープンソース Rust CLI 集です。",
      sections: [
        { title: "ソフトウェアライセンス", body: "ソースコードとバイナリの利用条件はリポジトリの MIT License に従います。利用、複製、変更、配布時はライセンス表示を維持してください。" },
        { title: "インストールと実行", body: "各コマンドのヘルプと実行対象を確認する責任は利用者にあります。システム状態に影響するオプションは、自分の環境で確認してから実行してください。" },
        { title: "保証範囲", body: "ソフトウェアは MIT License に記載のとおり提供されます。特定目的への適合性、継続動作、結果の完全性を別途保証しません。" },
        { title: "費用とアカウント", body: "この Web サイトに登録や支払いの手順はありません。外部サービスへ接続するコマンドの費用と方針は、そのサービスの条件に従います。" },
      ],
    },
  },
  notFound: {
    navLabel: "エラーページメニュー",
    title: "ページが見つかりません。",
    description: "指定された cli-tools ページが見つかりません。",
    intro: "アドレスを確認するか、ツール案内の最初のページへ戻ってください。",
    action: "cli-tools ホームへ戻る",
  },
});

const zh = createCatalog({
  meta: {
    title: "cli-tools | 减少重复命令的 Rust 工具集",
    description: "在一个 Rust workspace 中完成仓库分析、Git 管理、数据转换和后台运行的五个 CLI 工具。",
    socialDescription: "只安装需要的工具，即可处理分析、整理、转换和后台任务。",
    imageAlt: "展示 cli-tools 命令流程的产品界面",
  },
  shell: {
    skip: "跳到主要内容",
    brandHome: "cli-tools 首页",
    navLabel: "主导航",
    navTools: "工具",
    navInstall: "安装",
    menuOpen: "打开菜单",
    menuClose: "关闭菜单",
    mobileNavLabel: "移动端菜单",
    exploreTools: "浏览工具",
    viewGitHub: "在 GitHub 查看代码",
    themeLabel: "选择主题",
    themeTitle: "主题",
    themeSystem: "跟随系统",
    themeLight: "浅色",
    themeDark: "深色",
    languageLabel: "选择语言",
    projectInfo: "项目信息",
    footerCopy: "CHANN 的 Rust CLI 工具集。MIT License。",
    footerNav: "页脚导航",
    privacy: "隐私说明",
    terms: "使用说明",
    noScript: "工具浏览器和复制功能需要 JavaScript。",
  },
  hero: {
    eyebrow: "5 个 Rust CLI",
    title: ["减少重复命令，", "专注于创造。"],
    summary: "从仓库分析、Git 整理到数据转换和后台运行。只安装需要的工具，直接在终端使用。",
    action: "查看安装命令",
    facts: ["5 个二进制文件", "无需账号", "MIT License"],
    terminalLabel: "cli-tools 实际命令运行示例",
  },
  benefits: {
    label: "01 / 成果",
    title: "把每天重复的工作变成一条命令。",
    description: "减少为小任务切换上下文的时间，并把结果保存在文件和日志中。",
    items: [
      { title: "留下报告的分析", description: "同时读取代码规模和 Git 历史，并导出为 HTML、Markdown、CSV 或 JSON。", command: "code-cost · work-summary" },
      { title: "一行完成转换", description: "无需在浏览器工具间切换，即可处理 JSON、YAML、标识符、网络和文件。", command: "dev-tools" },
      { title: "立即回到提示符", description: "把长命令发送到后台，并保留日志路径和完成通知。", command: "zzz" },
      { title: "用同一流程管理 Git", description: "把分支整理、状态检查、标记扫描和 changelog 生成统一到一组命令中。", command: "git-tools" },
    ],
  },
  tagline: {
    lines: [
      ["无需", "离开", "终端，"],
      ["分析、", "整理，", "继续", "下一项工作。"],
    ],
  },
  install: {
    label: "02 / 安装",
    title: "只安装需要的工具。",
    description: "克隆仓库，然后选择要安装的二进制文件。",
    codeLabel: "从本地路径安装",
    buildLabel: "构建全部工具",
    buildAria: "完整构建命令代码",
    testLabel: "测试全部工具",
    testAria: "完整测试命令代码",
  },
  explorer: {
    label: "03 / 工具集",
    title: "五个工具，一套工作流。",
    description: "选择工具即可查看真实选项和可以立即运行的示例。",
    tabsLabel: "选择 CLI 工具",
    exampleSuffix: "示例",
    outputLabel: "输出",
  },
  tools: [
    { label: "评估仓库价值", summary: "同时读取代码规模和 Git 历史，估算开发成本和项目价值。", detail: "通过表格和文件查看 LOC、语言难度、复杂度、成熟度和贡献者指标。" },
    { label: "Git 工作摘要", summary: "按时间段汇总提交历史，概括活动、预计工时和贡献价值。", detail: "支持今天、本周、本月筛选和自定义日期范围。" },
    { label: "Git 维护", summary: "集中处理分支整理、标记扫描、健康检查、changelog 和 commit 流程。", detail: "把重复的仓库检查拆成子命令，只运行当前需要的检查。" },
    { label: "开发者实用工具", summary: "用简短子命令处理 JSON、编码、网络、文本和系统任务。", detail: "无需寻找网页工具或编写一次性脚本，直接在终端完成小任务。" },
    { label: "后台运行", summary: "通过交互式 shell 在后台运行命令，并把输出保存到按日期组织的日志。", detail: "立即取回提示符；在 macOS 上还支持完成通知和返回原终端。" },
  ],
  itermKeys: {
    label: "04 / macOS · iTerm2",
    title: "保持韩文输入，也保留终端快捷键。",
    description: "在 iTerm2 3.6.11 中，将物理 Control-C 和 Control-G 映射为不受输入语言影响的 PTY 字节。无需切换输入源，也无需安装常驻键盘工具。",
    mappingLabel: "物理按键与 PTY 字节映射",
    physicalLabel: "物理按键",
    byteLabel: "PTY 字节",
    mappingNote: "iTerm2 读取物理按键位置，而不是组合后的韩文字符，并发送准确的控制字节。",
    safeguards: [
      { title: "保持输入", description: "继续使用韩文输入，不切换到 ABC，且行为仅限于 iTerm2。" },
      { title: "冲突即停止", description: "先检查全局和配置文件映射；如果与其他操作重叠，则在修改前停止。" },
      { title: "精确恢复", description: "使用私有所有权收据只删除两个受管条目，不覆盖用户之后的修改。" },
    ],
    codeLabel: "配置 iTerm2 韩文控制键",
    restoreCodeLabel: "使用收据恢复",
    note: "需要 uv 和 iTerm2 Python API 授权。restore 只能使用 apply 输出的私有收据绝对路径。",
  },
  zzz: {
    label: "05 / 后台运行",
    title: "命令在后台，结果进日志。",
    descriptionBefore: "会立即返还提示符，并在命令完成时通知你。",
    behaviors: [
      { title: "立即返回", description: "继续使用交互式 shell 中的 alias 和 function。" },
      { title: "保存日志", description: "按日期和时间自动保存输出。" },
      { title: "macOS 通知", description: "安装 alerter 后，可返回原来的 iTerm2 会话或 Terminal 标签页。" },
    ],
    codeLabel: "开始使用 zzz",
    note: "如果没有 alerter，zzz 会回退到 terminal-notifier 或普通完成通知。",
    previewLabel: "zzz 运行流程示例",
  },
  utility: {
    label: "06 / 实用工具",
    title: "用一条命令处理重复工作。",
    descriptionBefore: "把数据、网络、代码和文件任务整理为按主题划分的子命令。",
    tabsLabel: "dev-tools 主题",
    commandSuffix: "命令选择",
    exampleSuffix: "示例",
    groups: [
      { name: "数据格式", description: "检查结构化数据并在格式之间转换。" },
      { name: "ID 与安全", description: "生成并检查标识符和开发用安全值。" },
      { name: "网络", description: "在终端检查地址、端口、DNS 和证书。" },
      { name: "文本与代码", description: "处理文本并从 JSON 生成类型。" },
      { name: "文件与系统", description: "快速处理本地文件、进程和系统信息。" },
    ],
  },
  analysis: {
    label: "07 / 分析",
    title: "读懂仓库，留下数字。",
    description: "用同一标准读取代码和 Git 历史，说明项目规模与工作方式。",
    items: [
      { name: "code-cost", description: "按语言分析代码量、难度、复杂度和成熟度，估算成本与价值。", command: "code-cost --export report.html", aria: "code-cost 分析命令代码" },
      { name: "work-summary", description: "根据提交间隔和变更量，汇总分阶段活动与预计工时。", command: "work-summary --month", aria: "work-summary 分析命令代码" },
      { name: "git-tools", description: "分析后使用 health、scan 和 changelog 整理仓库状态。", command: "git-tools pulse", aria: "git-tools 分析命令代码" },
    ],
  },
  workflow: {
    label: "08 / 开始使用",
    title: "克隆、选择、立即验证。",
    description: "无需单独账号或设置页面，沿用熟悉的 Cargo 流程即可开始。",
    steps: [
      { number: "01", title: "选择需要的二进制文件", description: "从分析、Git 管理、转换和后台运行中，只选当前需要的工具。" },
      { number: "02", title: "从本地源码安装", description: "克隆仓库，并用 cargo install 安装选中的 crate。" },
      { number: "03", title: "通过帮助和测试验证", description: "查看每条命令的帮助，并用完整 workspace 测试统一验证。" },
    ],
  },
  faq: {
    label: "09 / 常见问题",
    title: "安装前常见问题。",
    description: "先了解工具选择和数据处理方式。",
    items: [
      { question: "必须安装全部五个工具吗？", answer: "不需要。每个二进制文件都可以独立安装。只需为当前需要的 crate 运行 cargo install。" },
      { question: "官方安装方式是什么？", answer: "目前需要克隆 GitHub 仓库并从本地路径安装。可直接复制安装区域中的命令。" },
      { question: "命令会把数据发送到外部吗？", answer: "分析和转换默认在本地运行。只有 DNS、HTTP、天气等需要网络的命令才会连接你指定的目标。" },
      { question: "zzz 适合什么环境？", answer: "它适合经常运行长时间构建和测试的 Unix shell。在 macOS 上还支持完成通知和返回原终端会话。" },
      { question: "如何更新或卸载？", answer: "拉取最新源码，并在同一条 cargo install 命令中加入 force 选项。卸载时使用 cargo uninstall 和二进制名称。" },
      { question: "需要付费或注册账号吗？", answer: "不需要。cli-tools 以 MIT License 开源，网站也没有注册或付款步骤。" },
    ],
  },
  final: {
    label: "开源 / MIT",
    title: "从需要的工具开始。",
    description: "免费开放的 Rust workspace。无需登录，只需选择想要的二进制文件。",
    risk: "免费 · 无需账号 · MIT License",
    action: "查看安装命令",
  },
  ui: {
    copy: "复制",
    copying: "正在复制",
    copied: "已复制",
    copyError: "复制失败",
    copiedStatus: "已复制到剪贴板。",
    copyErrorStatus: "请检查浏览器的剪贴板权限。",
    codeSuffix: "代码",
  },
  legal: {
    updated: "更新于 2026.08.03",
    navLabel: "法律说明导航",
    backHome: "返回首页",
    privacy: {
      title: "隐私说明",
      description: "cli-tools 网站的数据处理和存储项目说明。",
      intro: "cli-tools 网站不提供注册、付款或联系表单，也不使用识别访客的第一方分析工具。",
      sections: [
        { title: "网站存储的内容", body: "主题和语言选择只保存在当前浏览器的 localStorage 中，不会发送到服务器，并可随时在浏览器设置中删除。" },
        { title: "复制命令", body: "复制按钮使用浏览器 Clipboard API。复制的命令和剪贴板内容不会发送到 cli-tools 网站。" },
        { title: "托管与外部链接", body: "网站由 GitHub Pages 提供。GitHub 可能会处理服务运行所需的请求信息。" },
        { title: "变更", body: "如果数据处理方式发生变化，本页说明和更新日期会同步修改。" },
      ],
    },
    terms: {
      title: "使用说明",
      description: "cli-tools 软件和网站的使用说明。",
      intro: "cli-tools 是由开发者在自己的环境中安装并运行的开源 Rust CLI 工具集。",
      sections: [
        { title: "软件许可", body: "源代码和二进制文件的使用遵循仓库中的 MIT License。使用、复制、修改或分发软件时，请保留许可声明。" },
        { title: "安装与运行", body: "用户有责任查看每条命令的帮助和目标。影响系统状态的选项应先在自己的环境中确认再运行。" },
        { title: "保证范围", body: "软件按 MIT License 所述提供，不另行保证特定用途适用性、持续运行或结果完整性。" },
        { title: "费用与账号", body: "本网站没有注册或付款步骤。连接外部服务的命令所产生的费用和政策遵循相应服务条款。" },
      ],
    },
  },
  notFound: {
    navLabel: "错误页导航",
    title: "找不到请求的页面。",
    description: "找不到请求的 cli-tools 页面。",
    intro: "请检查地址，或返回工具指南首页。",
    action: "返回 cli-tools 首页",
  },
});

export const catalogs = { ko, en, ja, zh };

export function getMessages(locale) {
  return catalogs[locale] || catalogs.ko;
}
