import {
  ArrowUpRight,
  BellRinging,
  BracketsCurly,
  ChartLineUp,
  Check,
  ClipboardText,
  GitBranch,
} from "@phosphor-icons/react";
import { AnimatePresence, motion, useReducedMotion } from "motion/react";
import { useEffect, useMemo, useRef, useState } from "react";
import {
  installAll,
  tools,
  utilityGroups,
} from "./data/tools";

function CodeBlock({ code, label = "명령어" }) {
  const [copyState, setCopyState] = useState("idle");
  const resetTimer = useRef();

  useEffect(
    () => () => {
      window.clearTimeout(resetTimer.current);
    },
    [],
  );

  const copyCode = async () => {
    setCopyState("copying");
    try {
      await navigator.clipboard.writeText(code);
      setCopyState("copied");
    } catch {
      setCopyState("error");
    }
    window.clearTimeout(resetTimer.current);
    resetTimer.current = window.setTimeout(() => setCopyState("idle"), 1800);
  };

  const buttonLabel = {
    idle: "복사",
    copying: "복사 중",
    copied: "복사됨",
    error: "복사 실패",
  }[copyState];

  return (
    <div className="code-block">
      <div className="code-block__header">
        <span>{label}</span>
        <button
          className="copy-button"
          type="button"
          onClick={copyCode}
          aria-label={`${label} 복사`}
          data-state={copyState}
        >
          {copyState === "copied" ? (
            <Check aria-hidden="true" weight="bold" />
          ) : (
            <ClipboardText aria-hidden="true" weight="bold" />
          )}
          <span>{buttonLabel}</span>
        </button>
      </div>
      <pre role="region" tabIndex={0} aria-label={`${label} 코드`}>
        <code>{code}</code>
      </pre>
      <span className="copy-status" role="status" aria-live="polite">
        {copyState === "error"
          ? "브라우저의 클립보드 권한을 확인해 주세요."
          : copyState === "copied"
            ? "클립보드에 복사했습니다."
            : ""}
      </span>
    </div>
  );
}

function RevealSection({ children, className = "", ...props }) {
  const reduceMotion = useReducedMotion();

  return (
    <motion.section
      className={className}
      initial={
        reduceMotion
          ? false
          : { opacity: 0, y: 64, filter: "blur(12px)" }
      }
      whileInView={
        reduceMotion
          ? undefined
          : { opacity: 1, y: 0, filter: "blur(0px)" }
      }
      viewport={{ once: true, amount: 0.12 }}
      transition={{ duration: 0.9, ease: [0.32, 0.72, 0, 1] }}
      {...props}
    >
      {children}
    </motion.section>
  );
}

const benefits = [
  {
    icon: ChartLineUp,
    title: "보고서가 남는 분석",
    description:
      "코드 규모와 Git 이력을 함께 읽고 HTML, Markdown, CSV, JSON으로 다시 씁니다.",
    command: "code-cost · work-summary",
  },
  {
    icon: BracketsCurly,
    title: "변환을 한 줄로",
    description:
      "JSON, YAML, 식별자, 네트워크, 파일 작업을 브라우저를 오가지 않고 처리합니다.",
    command: "dev-tools",
  },
  {
    icon: BellRinging,
    title: "프롬프트는 바로 복귀",
    description:
      "긴 명령은 백그라운드로 보내고 로그 경로와 완료 알림으로 결과를 놓치지 않습니다.",
    command: "zzz",
  },
  {
    icon: GitBranch,
    title: "Git 관리도 같은 흐름으로",
    description:
      "브랜치 정리, 상태 검사, 마커 탐색, changelog 생성을 하나의 명령 체계로 묶습니다.",
    command: "git-tools",
  },
];

function BenefitsSection() {
  return (
    <RevealSection className="benefits section-shell" id="benefits">
      <div className="section-heading section-heading--left">
        <p className="section-label">01 / 결과</p>
        <h2>하루에 몇 번씩 하던 일을, 한 번의 명령으로.</h2>
        <p>작은 작업을 위해 맥락을 바꾸는 시간을 줄이고 결과를 파일과 로그로 남깁니다.</p>
      </div>
      <div className="benefit-grid">
        {benefits.map(({ icon: Icon, title, description, command }) => (
          <article key={title}>
            <Icon aria-hidden="true" weight="duotone" />
            <div>
              <h3>{title}</h3>
              <p>{description}</p>
            </div>
            <code>{command}</code>
          </article>
        ))}
      </div>
    </RevealSection>
  );
}

const taglineLines = [
  ["터미널을", "떠나지", "않고,"],
  ["분석하고", "정리하고", "다음", "작업으로."],
];

function TaglineReveal() {
  const reduceMotion = useReducedMotion();

  return (
    <section className="tagline section-shell" aria-labelledby="tagline-heading">
      <h2 className="tagline__copy" id="tagline-heading">
        {taglineLines.map((line, lineIndex) => {
          const offset = taglineLines
            .slice(0, lineIndex)
            .reduce((total, words) => total + words.length, 0);

          return (
            <span className="tagline__line" key={line.join(" ")}>
              {line.map((word, wordIndex) => (
                <motion.span
                  className="tagline__word"
                  key={word}
                  initial={reduceMotion ? false : { color: "var(--tagline-muted)" }}
                  whileInView={{ color: "var(--text)" }}
                  viewport={{ once: true, amount: 0.8 }}
                  transition={{
                    duration: reduceMotion ? 0 : 0.8,
                    delay: reduceMotion ? 0 : (offset + wordIndex) * 0.08,
                    ease: [0.32, 0.72, 0, 1],
                  }}
                >
                  {word}
                </motion.span>
              ))}
            </span>
          );
        })}
      </h2>
    </section>
  );
}

function InstallSection() {
  return (
    <RevealSection className="install section-shell" id="install">
      <div className="section-heading">
        <p className="section-label">02 / 설치</p>
        <h2>필요한 도구만 설치하세요.</h2>
        <p>저장소를 복제한 뒤 원하는 바이너리만 골라 설치합니다.</p>
      </div>
      <CodeBlock code={installAll} label="로컬 경로에서 설치" />
      <div className="install__checks">
        <p>
          전체 빌드
          <code>cargo build --release --workspace --bins</code>
        </p>
        <p>
          전체 테스트
          <code>cargo test --workspace --all-targets</code>
        </p>
      </div>
    </RevealSection>
  );
}

function ToolExplorer() {
  const reduceMotion = useReducedMotion();
  const [activeId, setActiveId] = useState(tools[0].id);
  const activeTool = tools.find((tool) => tool.id === activeId) || tools[0];

  const handleTabKey = (event, index) => {
    if (!["ArrowDown", "ArrowUp", "ArrowRight", "ArrowLeft"].includes(event.key)) {
      return;
    }
    event.preventDefault();
    const isNext = event.key === "ArrowDown" || event.key === "ArrowRight";
    const nextIndex = (index + (isNext ? 1 : -1) + tools.length) % tools.length;
    setActiveId(tools[nextIndex].id);
    document.getElementById(`tool-tab-${tools[nextIndex].id}`)?.focus();
  };

  return (
    <RevealSection className="tool-explorer section-shell" id="tools">
      <div className="section-heading">
        <p className="section-label">03 / 도구 모음</p>
        <h2>다섯 도구, 한 작업 흐름.</h2>
        <p>도구를 선택하면 실제 옵션과 바로 실행할 수 있는 예시를 확인할 수 있습니다.</p>
      </div>
      <div className="tool-explorer__layout">
        <div className="tool-tabs" role="tablist" aria-label="CLI 도구 선택">
          {tools.map((tool, index) => (
            <button
              id={`tool-tab-${tool.id}`}
              key={tool.id}
              type="button"
              role="tab"
              aria-selected={activeId === tool.id}
              aria-controls={`tool-panel-${tool.id}`}
              tabIndex={activeId === tool.id ? 0 : -1}
              onClick={() => setActiveId(tool.id)}
              onKeyDown={(event) => handleTabKey(event, index)}
            >
              <span>{tool.name}</span>
              <small>{tool.label}</small>
            </button>
          ))}
        </div>

        <div className="tool-panel-frame">
          <AnimatePresence mode="wait">
            <motion.div
              className="tool-panel"
              id={`tool-panel-${activeTool.id}`}
              key={activeTool.id}
              role="tabpanel"
              aria-labelledby={`tool-tab-${activeTool.id}`}
              initial={reduceMotion ? false : { opacity: 0, y: 10 }}
              animate={{ opacity: 1, y: 0 }}
              exit={reduceMotion ? undefined : { opacity: 0, y: -8 }}
              transition={{
                duration: reduceMotion ? 0 : 0.7,
                ease: [0.32, 0.72, 0, 1],
              }}
            >
              <div className="tool-panel__title">
                <h3>{activeTool.name}</h3>
                <span>{activeTool.label}</span>
              </div>
              <p className="tool-panel__summary">{activeTool.summary}</p>
              <p className="tool-panel__detail">{activeTool.detail}</p>
              <CodeBlock code={activeTool.examples} label={`${activeTool.name} 예시`} />
              <p className="tool-panel__output">
                출력
                <code>{activeTool.output}</code>
              </p>
            </motion.div>
          </AnimatePresence>
        </div>
      </div>
    </RevealSection>
  );
}

function ZzzSection() {
  return (
    <RevealSection className="zzz-feature section-shell" id="zzz">
      <div className="zzz-feature__content">
        <div className="section-heading">
          <p className="section-label">04 / 백그라운드</p>
          <h2>명령은 백그라운드로. 결과는 로그로.</h2>
          <p>
            <code>zzz</code>는 프롬프트를 바로 돌려주고 완료 여부를 알림으로 알려줍니다.
          </p>
        </div>
        <div className="behavior-list">
          <div>
            <strong>즉시 복귀</strong>
            <span>대화형 셸의 alias와 function을 그대로 사용합니다.</span>
          </div>
          <div>
            <strong>로그 저장</strong>
            <span>출력을 날짜와 시간 기준 경로에 자동으로 남깁니다.</span>
          </div>
          <div>
            <strong>macOS 알림</strong>
            <span>alerter를 설치하면 원래 iTerm2 세션이나 Terminal 탭으로 돌아갑니다.</span>
          </div>
        </div>
        <CodeBlock
          code={`brew install vjeantet/tap/alerter

zzz cargo test
zzz --wait cargo test
zzz --print-log make build`}
          label="zzz 시작하기"
        />
        <p className="zzz-feature__note">
          alerter가 없으면 terminal-notifier 또는 일반 완료 알림으로 폴백합니다.
        </p>
      </div>
      <div className="terminal-preview" role="group" aria-label="zzz 실행 흐름 예시">
        <div className="terminal-preview__bar">
          <span aria-hidden="true"><i></i><i></i><i></i></span>
          <small>zzz · background task</small>
        </div>
        <div className="terminal-preview__body">
          <p><span>$</span> zzz cargo test</p>
          <p className="terminal-preview__muted">started · pid 82417</p>
          <p className="terminal-preview__spacer" aria-hidden="true"></p>
          <p><span>✓</span> cargo test</p>
          <p className="terminal-preview__muted">finished in 12.4s · exit 0</p>
          <p className="terminal-preview__path">~/.commands/260801/032451-cargo.log</p>
        </div>
      </div>
    </RevealSection>
  );
}

function UtilityExplorer() {
  const reduceMotion = useReducedMotion();
  const [groupId, setGroupId] = useState(utilityGroups[0].id);
  const activeGroup = utilityGroups.find((group) => group.id === groupId) || utilityGroups[0];
  const [commandName, setCommandName] = useState(activeGroup.commands[0].name);

  const activeCommand = useMemo(
    () =>
      activeGroup.commands.find((command) => command.name === commandName) ||
      activeGroup.commands[0],
    [activeGroup, commandName],
  );

  const selectGroup = (nextGroup) => {
    setGroupId(nextGroup.id);
    setCommandName(nextGroup.commands[0].name);
  };

  const handleGroupKey = (event, index) => {
    if (!["ArrowRight", "ArrowLeft", "Home", "End"].includes(event.key)) {
      return;
    }
    event.preventDefault();
    let nextIndex = index;
    if (event.key === "Home") {
      nextIndex = 0;
    } else if (event.key === "End") {
      nextIndex = utilityGroups.length - 1;
    } else {
      const direction = event.key === "ArrowRight" ? 1 : -1;
      nextIndex = (index + direction + utilityGroups.length) % utilityGroups.length;
    }
    const nextGroup = utilityGroups[nextIndex];
    selectGroup(nextGroup);
    document.getElementById(`utility-tab-${nextGroup.id}`)?.focus();
  };

  return (
    <RevealSection className="utility-explorer section-shell" id="utilities">
      <div className="section-heading">
        <p className="section-label">05 / 유틸리티</p>
        <h2>반복 작업을 한 명령으로.</h2>
        <p>
          <code>dev-tools</code>는 데이터, 네트워크, 코드, 파일 작업을 주제별 하위
          명령으로 묶습니다.
        </p>
      </div>
      <div className="group-tabs" role="tablist" aria-label="dev-tools 주제">
        {utilityGroups.map((group, index) => (
          <button
            key={group.id}
            id={`utility-tab-${group.id}`}
            type="button"
            role="tab"
            aria-selected={group.id === activeGroup.id}
            aria-controls={`utility-panel-${group.id}`}
            tabIndex={group.id === activeGroup.id ? 0 : -1}
            onClick={() => selectGroup(group)}
            onKeyDown={(event) => handleGroupKey(event, index)}
          >
            {group.name}
          </button>
        ))}
      </div>
      <div
        className="utility-stage"
        id={`utility-panel-${activeGroup.id}`}
        role="tabpanel"
        aria-labelledby={`utility-tab-${activeGroup.id}`}
      >
        <div className="utility-stage__intro">
          <h3>{activeGroup.name}</h3>
          <p>{activeGroup.description}</p>
        </div>
        <div
          className="command-picker"
          role="group"
          aria-label={`${activeGroup.name} 명령 선택`}
        >
          {activeGroup.commands.map((command) => (
            <button
              key={command.name}
              type="button"
              aria-pressed={command.name === activeCommand.name}
              onClick={() => setCommandName(command.name)}
            >
              dev-tools {command.name}
            </button>
          ))}
        </div>
        <AnimatePresence mode="wait">
          <motion.div
            className="utility-stage__code"
            key={`${activeGroup.id}-${activeCommand.name}`}
            initial={reduceMotion ? false : { opacity: 0, y: 8 }}
            animate={{ opacity: 1, y: 0 }}
            exit={reduceMotion ? undefined : { opacity: 0, y: -6 }}
            transition={{
              duration: reduceMotion ? 0 : 0.7,
              ease: [0.32, 0.72, 0, 1],
            }}
          >
            <CodeBlock code={activeCommand.code} label={`${activeCommand.name} 예시`} />
          </motion.div>
        </AnimatePresence>
      </div>
    </RevealSection>
  );
}

function AnalysisSection() {
  return (
    <RevealSection className="analysis section-shell" id="analysis">
      <div className="analysis__content">
        <div className="section-heading">
          <p className="section-label">06 / 분석</p>
          <h2>저장소를 읽고, 숫자로 남기세요.</h2>
          <p>코드와 Git 이력을 같은 기준으로 읽어 규모와 작업 흐름을 설명합니다.</p>
        </div>
        <div className="analysis__tools">
          <article>
            <h3>code-cost</h3>
            <p>언어별 코드량, 난이도, 복잡도, 성숙도를 분석해 비용과 가치를 추정합니다.</p>
            <code>code-cost --export report.html</code>
          </article>
          <article>
            <h3>work-summary</h3>
            <p>커밋 간격과 변경량을 바탕으로 기간별 활동과 예상 작업 시간을 요약합니다.</p>
            <code>work-summary --month</code>
          </article>
          <article className="analysis__git">
            <h3>git-tools</h3>
            <p>분석 뒤에는 health, scan, changelog로 저장소 상태를 바로 정리합니다.</p>
            <code>git-tools pulse</code>
          </article>
        </div>
      </div>
    </RevealSection>
  );
}

const workflowSteps = [
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
];

function WorkflowSection() {
  return (
    <RevealSection className="workflow section-shell" id="workflow">
      <div className="section-heading section-heading--left">
        <p className="section-label">07 / 시작 흐름</p>
        <h2>복제하고, 고르고, 바로 확인합니다.</h2>
        <p>별도 계정이나 설정 화면 없이 익숙한 Cargo 흐름으로 시작합니다.</p>
      </div>
      <ol className="workflow-list">
        {workflowSteps.map((step) => (
          <li key={step.number}>
            <span>{step.number}</span>
            <div>
              <h3>{step.title}</h3>
              <p>{step.description}</p>
            </div>
          </li>
        ))}
      </ol>
    </RevealSection>
  );
}

const faqs = [
  {
    question: "다섯 도구를 모두 설치해야 하나요?",
    answer:
      "아닙니다. 각 바이너리는 독립 설치 대상입니다. 지금 필요한 crate의 cargo install 명령만 실행하면 됩니다.",
  },
  {
    question: "공식 설치 경로는 무엇인가요?",
    answer:
      "현재는 GitHub 저장소를 복제한 뒤 로컬 경로에서 설치합니다. 설치 섹션의 명령을 그대로 복사할 수 있습니다.",
  },
  {
    question: "명령이 데이터를 외부로 전송하나요?",
    answer:
      "분석과 변환은 기본적으로 로컬에서 실행됩니다. DNS, HTTP, 날씨처럼 네트워크가 필요한 명령만 요청한 대상에 연결합니다.",
  },
  {
    question: "zzz는 어떤 환경에서 유용한가요?",
    answer:
      "긴 빌드와 테스트를 자주 돌리는 Unix 셸에서 유용합니다. macOS에서는 완료 알림과 원래 터미널 세션 복귀도 지원합니다.",
  },
  {
    question: "업데이트와 삭제는 어떻게 하나요?",
    answer:
      "최신 소스를 받은 뒤 같은 cargo install 명령에 force 옵션을 사용해 갱신합니다. 삭제할 때는 cargo uninstall과 바이너리 이름을 사용합니다.",
  },
  {
    question: "비용이나 계정이 필요한가요?",
    answer:
      "필요하지 않습니다. cli-tools는 MIT License로 공개되어 있으며 사이트에도 가입이나 결제 단계가 없습니다.",
  },
];

function FaqSection() {
  return (
    <RevealSection className="faq section-shell" id="faq">
      <div className="section-heading section-heading--left">
        <p className="section-label">08 / 자주 묻는 질문</p>
        <h2>설치 전에 궁금한 점.</h2>
        <p>도구 선택부터 데이터 처리 방식까지 먼저 확인하세요.</p>
      </div>
      <dl className="faq-list">
        {faqs.map((item) => (
          <div key={item.question}>
            <dt>{item.question}</dt>
            <dd>{item.answer}</dd>
          </div>
        ))}
      </dl>
    </RevealSection>
  );
}

function FinalSection() {
  return (
    <RevealSection className="final-cta section-shell">
      <div>
        <p className="section-label">오픈 소스 / MIT</p>
        <h2>필요한 도구부터 설치하세요.</h2>
        <p className="final-cta__description">
          무료로 공개된 Rust workspace입니다. 로그인 없이 원하는 바이너리만 고르면 됩니다.
        </p>
        <p className="final-cta__risk">무료 · 로그인 없음 · MIT License</p>
      </div>
      <a className="button button--primary" href="#install">
        설치 명령 보기
        <ArrowUpRight aria-hidden="true" weight="bold" />
      </a>
    </RevealSection>
  );
}

export default function App() {
  return (
    <>
      <BenefitsSection />
      <InstallSection />
      <ToolExplorer />
      <TaglineReveal />
      <ZzzSection />
      <UtilityExplorer />
      <AnalysisSection />
      <WorkflowSection />
      <FaqSection />
      <FinalSection />
    </>
  );
}
