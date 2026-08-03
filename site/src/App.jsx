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
import { useI18n } from "./i18n/context";

function CodeBlock({ code, label }) {
  const { messages } = useI18n();
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
    idle: messages.ui.copy,
    copying: messages.ui.copying,
    copied: messages.ui.copied,
    error: messages.ui.copyError,
  }[copyState];

  return (
    <div className="code-block">
      <div className="code-block__header">
        <span>{label}</span>
        <button
          className="copy-button"
          type="button"
          onClick={copyCode}
          aria-label={`${label} ${messages.ui.copy}`}
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
      <pre
        role="region"
        tabIndex={0}
        aria-label={`${label} ${messages.ui.codeSuffix}`}
      >
        <code>{code}</code>
      </pre>
      <span className="copy-status" role="status" aria-live="polite">
        {copyState === "error"
          ? messages.ui.copyErrorStatus
          : copyState === "copied"
            ? messages.ui.copiedStatus
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

const benefitIcons = [ChartLineUp, BracketsCurly, BellRinging, GitBranch];

function BenefitsSection() {
  const { messages } = useI18n();

  return (
    <RevealSection className="benefits section-shell" id="benefits">
      <div className="section-heading section-heading--left">
        <p className="section-label">{messages.benefits.label}</p>
        <h2>{messages.benefits.title}</h2>
        <p>{messages.benefits.description}</p>
      </div>
      <div className="benefit-grid">
        {messages.benefits.items.map(({ title, description, command }, index) => {
          const Icon = benefitIcons[index];
          return (
            <article key={title}>
              <Icon aria-hidden="true" weight="duotone" />
              <div>
                <h3>{title}</h3>
                <p>{description}</p>
              </div>
              <code>{command}</code>
            </article>
          );
        })}
      </div>
    </RevealSection>
  );
}

function TaglineReveal() {
  const reduceMotion = useReducedMotion();
  const { messages } = useI18n();
  const taglineLines = messages.tagline.lines;

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
  const { messages } = useI18n();

  return (
    <RevealSection className="install section-shell" id="install">
      <div className="section-heading">
        <p className="section-label">{messages.install.label}</p>
        <h2>{messages.install.title}</h2>
        <p>{messages.install.description}</p>
      </div>
      <CodeBlock code={messages.install.command} label={messages.install.codeLabel} />
      <div className="install__checks">
        <p>
          {messages.install.buildLabel}
          <code role="region" tabIndex={0} aria-label={messages.install.buildAria}>
            cargo build --release --workspace --bins
          </code>
        </p>
        <p>
          {messages.install.testLabel}
          <code role="region" tabIndex={0} aria-label={messages.install.testAria}>
            cargo test --workspace --all-targets
          </code>
        </p>
      </div>
    </RevealSection>
  );
}

function ToolExplorer() {
  const reduceMotion = useReducedMotion();
  const { messages } = useI18n();
  const tools = messages.tools;
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
        <p className="section-label">{messages.explorer.label}</p>
        <h2>{messages.explorer.title}</h2>
        <p>{messages.explorer.description}</p>
      </div>
      <div className="tool-explorer__layout">
        <div className="tool-tabs" role="tablist" aria-label={messages.explorer.tabsLabel}>
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
              <CodeBlock
                code={activeTool.examples}
                label={`${activeTool.name} ${messages.explorer.exampleSuffix}`}
              />
              <p className="tool-panel__output">
                {messages.explorer.outputLabel}
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
  const { messages } = useI18n();

  return (
    <RevealSection className="zzz-feature section-shell" id="zzz">
      <div className="zzz-feature__content">
        <div className="section-heading">
          <p className="section-label">{messages.zzz.label}</p>
          <h2>{messages.zzz.title}</h2>
          <p>
            <code>zzz</code>{messages.zzz.descriptionBefore}
          </p>
        </div>
        <div className="behavior-list">
          {messages.zzz.behaviors.map((behavior) => (
            <div key={behavior.title}>
              <strong>{behavior.title}</strong>
              <span>{behavior.description}</span>
            </div>
          ))}
        </div>
        <CodeBlock
          code={`brew install vjeantet/tap/alerter

zzz cargo test
zzz --wait cargo test
zzz --print-log make build`}
          label={messages.zzz.codeLabel}
        />
        <p className="zzz-feature__note">{messages.zzz.note}</p>
      </div>
      <div className="terminal-preview" role="group" aria-label={messages.zzz.previewLabel}>
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
  const { messages } = useI18n();
  const utilityGroups = messages.utility.groups;
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
        <p className="section-label">{messages.utility.label}</p>
        <h2>{messages.utility.title}</h2>
        <p>
          <code>dev-tools</code>{messages.utility.descriptionBefore}
        </p>
      </div>
      <div className="group-tabs" role="tablist" aria-label={messages.utility.tabsLabel}>
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
          aria-label={`${activeGroup.name} ${messages.utility.commandSuffix}`}
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
            <CodeBlock
              code={activeCommand.code}
              label={`${activeCommand.name} ${messages.utility.exampleSuffix}`}
            />
          </motion.div>
        </AnimatePresence>
      </div>
    </RevealSection>
  );
}

function AnalysisSection() {
  const { messages } = useI18n();

  return (
    <RevealSection className="analysis section-shell" id="analysis">
      <div className="analysis__content">
        <div className="section-heading">
          <p className="section-label">{messages.analysis.label}</p>
          <h2>{messages.analysis.title}</h2>
          <p>{messages.analysis.description}</p>
        </div>
        <div className="analysis__tools">
          {messages.analysis.items.map((item, index) => (
            <article className={index === 2 ? "analysis__git" : undefined} key={item.name}>
              <h3>{item.name}</h3>
              <p>{item.description}</p>
              <code role="region" tabIndex={0} aria-label={item.aria}>
                {item.command}
              </code>
            </article>
          ))}
        </div>
      </div>
    </RevealSection>
  );
}

function WorkflowSection() {
  const { messages } = useI18n();

  return (
    <RevealSection className="workflow section-shell" id="workflow">
      <div className="section-heading section-heading--left">
        <p className="section-label">{messages.workflow.label}</p>
        <h2>{messages.workflow.title}</h2>
        <p>{messages.workflow.description}</p>
      </div>
      <ol className="workflow-list">
        {messages.workflow.steps.map((step) => (
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

function FaqSection() {
  const { messages } = useI18n();

  return (
    <RevealSection className="faq section-shell" id="faq">
      <div className="section-heading section-heading--left">
        <p className="section-label">{messages.faq.label}</p>
        <h2>{messages.faq.title}</h2>
        <p>{messages.faq.description}</p>
      </div>
      <dl className="faq-list">
        {messages.faq.items.map((item) => (
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
  const { messages } = useI18n();

  return (
    <RevealSection className="final-cta section-shell">
      <div>
        <p className="section-label">{messages.final.label}</p>
        <h2>{messages.final.title}</h2>
        <p className="final-cta__description">{messages.final.description}</p>
        <p className="final-cta__risk">{messages.final.risk}</p>
      </div>
      <a className="button button--primary" href="#install">
        {messages.final.action}
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
