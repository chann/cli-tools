import {
  ArrowRight,
  ArrowUpRight,
  BellRinging,
  BracketsCurly,
  ChartLineUp,
  Check,
  ClipboardText,
  GitBranch,
  LinkSimple,
} from "@phosphor-icons/react";
import {
  AnimatePresence,
  motion,
  useReducedMotion,
  useScroll,
  useTransform,
} from "motion/react";
import { Fragment, useEffect, useMemo, useRef, useState } from "react";
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

function SectionTitle({ anchor, children, className, headingId, headingRef }) {
  return (
    <h2 className={className} id={headingId} ref={headingRef}>
      <a className="section-anchor" href={`#${anchor}`}>
        {children}
        <span className="section-anchor__mark" aria-hidden="true">
          <LinkSimple weight="bold" />
        </span>
      </a>
    </h2>
  );
}

function useInitialHashScroll() {
  useEffect(() => {
    const targetId = window.location.hash.slice(1);
    if (!targetId) {
      return undefined;
    }

    const frameId = window.requestAnimationFrame(() => {
      document.getElementById(targetId)?.scrollIntoView();
    });

    return () => window.cancelAnimationFrame(frameId);
  }, []);
}

function RevealSection({ children, className = "", id, ...props }) {
  const reduceMotion = useReducedMotion();
  const isInitialHashTarget = id && window.location.hash === `#${id}`;

  return (
    <motion.section
      className={className}
      id={id}
      initial={
        reduceMotion || isInitialHashTarget
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
        <SectionTitle anchor="benefits">{messages.benefits.title}</SectionTitle>
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

function TaglineWord({ children, progress, range, animate }) {
  const opacity = useTransform(progress, range, [0.48, 1]);

  return (
    <motion.span
      className="tagline__word"
      style={animate ? { opacity } : undefined}
    >
      {children}
    </motion.span>
  );
}

function TaglineReveal() {
  const reduceMotion = useReducedMotion();
  const { messages } = useI18n();
  const taglineLines = messages.tagline.lines;
  const taglineRef = useRef(null);
  const { scrollYProgress } = useScroll({
    target: taglineRef,
    offset: ["start 0.85", "end 0.5"],
  });
  const wordCount = taglineLines.reduce(
    (total, words) => total + words.length,
    0,
  );
  let globalWordIndex = 0;

  return (
    <section
      className="tagline section-shell"
      id="tagline"
      aria-labelledby="tagline-heading"
    >
      <SectionTitle
        anchor="tagline"
        className="tagline__copy"
        headingId="tagline-heading"
        headingRef={taglineRef}
      >
        {taglineLines.map((line, lineIndex) => (
          <Fragment key={line.join(" ")}>
            <span className="tagline__line">
              {line.map((word, wordIndexInLine) => {
                const wordIndex = globalWordIndex;
                globalWordIndex += 1;
                const isLastWordInLine = wordIndexInLine === line.length - 1;

                return (
                  <Fragment key={`${word}-${wordIndex}`}>
                    <TaglineWord
                      progress={scrollYProgress}
                      range={[wordIndex / wordCount, globalWordIndex / wordCount]}
                      animate={!reduceMotion}
                    >
                      {word}
                    </TaglineWord>
                    {isLastWordInLine ? null : " "}
                  </Fragment>
                );
              })}
            </span>
            {lineIndex === taglineLines.length - 1 ? null : " "}
          </Fragment>
        ))}
      </SectionTitle>
    </section>
  );
}

function InstallSection() {
  const { messages } = useI18n();

  return (
    <RevealSection className="install section-shell" id="install">
      <div className="section-heading">
        <p className="section-label">{messages.install.label}</p>
        <SectionTitle anchor="install">{messages.install.title}</SectionTitle>
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
        <SectionTitle anchor="tools">{messages.explorer.title}</SectionTitle>
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

function ItermKeysSection() {
  const { messages } = useI18n();

  return (
    <RevealSection className="iterm-keys section-shell" id="iterm-korean">
      <div className="iterm-keys__content">
        <div className="section-heading">
          <p className="section-label">{messages.itermKeys.label}</p>
          <SectionTitle anchor="iterm-korean">
            {messages.itermKeys.title}
          </SectionTitle>
          <p>{messages.itermKeys.description}</p>
        </div>
        <div className="behavior-list">
          {messages.itermKeys.safeguards.map((safeguard) => (
            <div key={safeguard.title}>
              <strong>{safeguard.title}</strong>
              <span>{safeguard.description}</span>
            </div>
          ))}
        </div>
        <CodeBlock
          code={messages.itermKeys.command}
          label={messages.itermKeys.codeLabel}
        />
        <CodeBlock
          code={messages.itermKeys.restoreCommand}
          label={messages.itermKeys.restoreCodeLabel}
        />
        <p className="iterm-keys__note">{messages.itermKeys.note}</p>
      </div>
      <div
        className="iterm-keys__mapping"
        role="group"
        aria-label={messages.itermKeys.mappingLabel}
      >
        <div className="iterm-keys__mapping-header">
          <span>{messages.itermKeys.physicalLabel}</span>
          <span>{messages.itermKeys.byteLabel}</span>
        </div>
        <div className="iterm-keys__mapping-row">
          <kbd>Control-C</kbd>
          <ArrowRight aria-hidden="true" weight="bold" />
          <code>0x03</code>
        </div>
        <div className="iterm-keys__mapping-row">
          <kbd>Control-G</kbd>
          <ArrowRight aria-hidden="true" weight="bold" />
          <code>0x07</code>
        </div>
        <p>{messages.itermKeys.mappingNote}</p>
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
          <SectionTitle anchor="zzz">{messages.zzz.title}</SectionTitle>
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
        <SectionTitle anchor="utilities">{messages.utility.title}</SectionTitle>
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
          <SectionTitle anchor="analysis">{messages.analysis.title}</SectionTitle>
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
        <SectionTitle anchor="workflow">{messages.workflow.title}</SectionTitle>
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
        <SectionTitle anchor="faq">{messages.faq.title}</SectionTitle>
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
    <RevealSection className="final-cta section-shell" id="get-started">
      <div>
        <p className="section-label">{messages.final.label}</p>
        <SectionTitle anchor="get-started">{messages.final.title}</SectionTitle>
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
  useInitialHashScroll();

  return (
    <>
      <BenefitsSection />
      <InstallSection />
      <ToolExplorer />
      <TaglineReveal />
      <ItermKeysSection />
      <ZzzSection />
      <UtilityExplorer />
      <AnalysisSection />
      <WorkflowSection />
      <FaqSection />
      <FinalSection />
    </>
  );
}
