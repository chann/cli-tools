import {
  ArrowUpRight,
  Check,
  ClipboardText,
} from "@phosphor-icons/react";
import { AnimatePresence, motion, useReducedMotion } from "motion/react";
import { useEffect, useMemo, useRef, useState } from "react";
import {
  installAll,
  repositoryUrl,
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
      initial={false}
      whileInView={
        reduceMotion
          ? undefined
          : { opacity: [0.94, 1], y: [10, 0] }
      }
      viewport={{ once: true, amount: 0.12 }}
      transition={{ duration: 0.5, ease: [0.16, 1, 0.3, 1] }}
      {...props}
    >
      {children}
    </motion.section>
  );
}

function InstallSection() {
  return (
    <RevealSection className="install section-shell" id="install">
      <div className="section-heading">
        <p className="section-label">01 / 설치</p>
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
        <p className="section-label">02 / 도구 모음</p>
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
              transition={{ duration: reduceMotion ? 0 : 0.22 }}
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
          <p className="section-label">03 / 백그라운드</p>
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
        <p className="section-label">04 / 유틸리티</p>
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
            transition={{ duration: reduceMotion ? 0 : 0.2 }}
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
          <p className="section-label">05 / 분석</p>
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

function FinalSection() {
  return (
    <RevealSection className="final-cta section-shell">
      <div>
        <p className="section-label">OPEN SOURCE / MIT</p>
        <h2>Rust workspace 그대로 시작하세요.</h2>
        <p>필요한 바이너리만 설치하고, 전체 workspace 테스트로 한 번에 검증할 수 있습니다.</p>
      </div>
      <a className="button button--primary" href={repositoryUrl}>
        GitHub
        <ArrowUpRight aria-hidden="true" weight="bold" />
      </a>
    </RevealSection>
  );
}

export default function App() {
  return (
    <>
      <InstallSection />
      <ToolExplorer />
      <ZzzSection />
      <UtilityExplorer />
      <AnalysisSection />
      <FinalSection />
    </>
  );
}
