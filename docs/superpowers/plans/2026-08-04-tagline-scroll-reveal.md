# Tagline Scroll Reveal Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Restore literal spaces in the existing two-line tagline and replace its imperceptible color transition with the reference site's scroll-progress word reveal.

**Architecture:** Keep localized word arrays as the content source. `TaglineReveal` owns one heading ref and one `useScroll` progress value, while a focused `TaglineWord` maps a normalized slice of that progress to opacity with `useTransform`; React fragments insert real spaces between words without changing the authored line wrappers.

**Tech Stack:** React 19, Motion 12 (`motion/react`), Vite 8, Vitest 4, Testing Library, CSS, GitHub Pages

## Global Constraints

- Keep the current localized tagline copy and its intentional two-line layout.
- Render exactly one literal space between adjacent words, including the boundary between authored lines, and no trailing space after the final word.
- Use scroll-progress opacity from `0.22` to `1` with offsets `start 0.85` and `end 0.5`.
- Preserve `word-break: keep-all`, light/dark tokens, responsive typography, and every section outside the tagline.
- When `prefers-reduced-motion` is active, attach no animated opacity style and render every word fully visible.
- Add no dependency and no new animation outside the tagline.
- Stage explicit paths only, never bypass hooks, use no force push, and prove `HEAD...@{u} = 0 0` after publication.

---

## File structure

- `site/src/App.jsx`: own tagline scroll progress, word ranges, literal separators, and reduced-motion behavior.
- `site/src/App.test.jsx`: lock exact text spacing, word count, reduced-motion output, and removal of synthetic spacing.
- `site/src/App.motion.test.jsx`: exercise the real default-motion render and lock each word's pre-reveal opacity.
- `site/src/styles.css`: keep word boxes inline while removing margin-based fake spaces.

---

### Task 1: Restore semantic spacing and add the scroll-linked reveal

**Files:**
- Modify: `site/src/App.jsx:10-11,138-175`
- Modify: `site/src/App.test.jsx:176-187`
- Create: `site/src/App.motion.test.jsx`
- Modify: `site/src/styles.css:830-859`

**Interfaces:**
- Consumes: `messages.tagline.lines: string[][]`, `useReducedMotion(): boolean | null`, Motion's `useScroll({ target, offset })`, and `useTransform(progress, range, output)`.
- Produces: `TaglineWord({ children, progress, range, animate })`, exact heading text `터미널을 떠나지 않고, 분석하고 정리하고 다음 작업으로.`, and seven `.tagline__word` elements for Korean.

- [ ] **Step 1: Write the failing spacing and motion contract**

Extend the imports in `site/src/App.test.jsx` to use the already imported
`readFileSync`, then replace the tagline-only assertions in
`completes the landing argument from benefits through FAQ` with:

```jsx
const tagline = screen.getByRole("heading", {
  name: "터미널을 떠나지 않고, 분석하고 정리하고 다음 작업으로.",
});
const taglineWords = [...container.querySelectorAll(".tagline__word")];

expect(tagline.textContent).toBe(
  "터미널을 떠나지 않고, 분석하고 정리하고 다음 작업으로.",
);
expect(taglineWords).toHaveLength(7);
expect(taglineWords.every((word) => word.style.opacity === "")).toBe(true);
```

Add a focused spacing test immediately after it:

```jsx
test("uses literal spaces instead of CSS margins between tagline words", () => {
  const style = document.createElement("style");
  style.textContent = readFileSync("src/styles.css", "utf8");
  document.head.append(style);
  const { container } = renderApp();
  const firstWord = container.querySelector(".tagline__word");

  expect(getComputedStyle(firstWord).marginRight).toBe("0px");

  style.remove();
});
```

Create `site/src/App.motion.test.jsx` to load the real component after enabling
default motion and assert the visible pre-reveal state:

```jsx
import { render, screen, waitFor } from "@testing-library/react";
import { expect, test, vi } from "vitest";
import { I18nProvider } from "./i18n/context";

test("starts each tagline word at muted opacity when motion is allowed", async () => {
  window.matchMedia.mockImplementation((query) => ({
    matches: false,
    media: query,
    onchange: null,
    addEventListener: vi.fn(),
    removeEventListener: vi.fn(),
    addListener: vi.fn(),
    removeListener: vi.fn(),
    dispatchEvent: vi.fn(),
  }));
  const { default: App } = await import("./App");
  const { container } = render(
    <I18nProvider locale="ko">
      <App />
    </I18nProvider>,
  );
  const tagline = screen.getByRole("heading", {
    name: "터미널을 떠나지 않고, 분석하고 정리하고 다음 작업으로.",
  });
  const words = [...container.querySelectorAll(".tagline__word")];

  expect(tagline.textContent).toBe(
    "터미널을 떠나지 않고, 분석하고 정리하고 다음 작업으로.",
  );
  await waitFor(() => {
    expect(words.map((word) => word.style.opacity)).toEqual(
      Array(7).fill("0.22"),
    );
  });
});
```

- [ ] **Step 2: Run the focused test and verify RED**

Run:

```bash
cd site
pnpm vitest run src/App.test.jsx src/App.motion.test.jsx
```

Expected: FAIL because the heading `textContent` has no literal spaces, computed
word margin is 12px, and default-motion words have no opacity value.

- [ ] **Step 3: Add the scroll-progress word component**

Change the React and Motion imports at the top of `site/src/App.jsx` to:

```jsx
import {
  AnimatePresence,
  motion,
  useReducedMotion,
  useScroll,
  useTransform,
} from "motion/react";
import { Fragment, useEffect, useMemo, useRef, useState } from "react";
```

Insert this focused component directly before `TaglineReveal`:

```jsx
function TaglineWord({ children, progress, range, animate }) {
  const opacity = useTransform(progress, range, [0.22, 1]);

  return (
    <motion.span
      className="tagline__word"
      style={animate ? { opacity } : undefined}
    >
      {children}
    </motion.span>
  );
}
```

- [ ] **Step 4: Replace the time-based tagline mapping**

Replace `TaglineReveal` in `site/src/App.jsx` with:

```jsx
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
    <section className="tagline section-shell" aria-labelledby="tagline-heading">
      <h2 className="tagline__copy" id="tagline-heading" ref={taglineRef}>
        {taglineLines.map((line) => (
          <span className="tagline__line" key={line.join(" ")}>
            {line.map((word) => {
              const wordIndex = globalWordIndex;
              globalWordIndex += 1;
              const isFinalWord = globalWordIndex === wordCount;

              return (
                <Fragment key={`${word}-${wordIndex}`}>
                  <TaglineWord
                    progress={scrollYProgress}
                    range={[wordIndex / wordCount, globalWordIndex / wordCount]}
                    animate={!reduceMotion}
                  >
                    {word}
                  </TaglineWord>
                  {isFinalWord ? null : " "}
                </Fragment>
              );
            })}
          </span>
        ))}
      </h2>
    </section>
  );
}
```

The unconditional `useTransform` call stays inside `TaglineWord`, satisfying the
Rules of Hooks. Reduced motion removes only the animated style; it does not alter
the text tree.

- [ ] **Step 5: Remove CSS-generated word spacing**

Replace the tagline word rules in `site/src/styles.css` with:

```css
.tagline__word {
  display: inline-block;
}
```

Do not change `.tagline__line`, typography, section height, or responsive rules.

- [ ] **Step 6: Run focused and complete verification**

Run:

```bash
cd site
pnpm vitest run src/App.test.jsx src/App.motion.test.jsx
pnpm check
git diff --check
```

Expected: the focused file passes, all site tests pass, the localized production
build and route verifier pass, and the diff contains no whitespace errors.

- [ ] **Step 7: Review the complete code diff**

Run:

```bash
git status --short
git diff -- site/src/App.jsx site/src/App.test.jsx site/src/App.motion.test.jsx site/src/styles.css
git diff --cached
```

Confirm the diff changes only tagline spacing, opacity progress, reduced-motion
behavior, and their tests. Confirm no secret-pattern path is present.

- [ ] **Step 8: Commit and push the verified feature**

Run:

```bash
git add site/src/App.jsx site/src/App.test.jsx site/src/App.motion.test.jsx site/src/styles.css
git diff --cached --check
git commit -m "fix(site): restore tagline spacing and scroll reveal"
git status --short
git push
git log -1 --oneline
git rev-list --left-right --count HEAD...@{u}
```

Expected: the commit succeeds without bypass flags, the ordinary push succeeds,
and local/tracking parity is `0 0`.

---

### Task 2: Verify the built and published interaction

**Files:**
- Verify: `site/dist/index.html`
- Verify: `site/dist/assets/*.js`
- Verify: `https://chann.github.io/cli-tools/`

**Interfaces:**
- Consumes: the production build from Task 1 and GitHub Pages deployment for the pushed `main` commit.
- Produces: browser evidence for exact text, progressive opacity, reduced motion, responsive layout, console health, deployment success, and live SHA parity.

- [ ] **Step 1: Serve the production build locally**

Run in a persistent terminal:

```bash
cd site
pnpm preview --host 127.0.0.1 --port 4173
```

Expected: Vite serves the page at
`http://127.0.0.1:4173/cli-tools/` without rebuilding source.

- [ ] **Step 2: Verify the default-motion interaction in a real browser**

Open `http://127.0.0.1:4173/cli-tools/` at 1440x900 and 375x812. At each width:

1. Read `#tagline-heading` and confirm its accessible text is exactly
   `터미널을 떠나지 않고, 분석하고 정리하고 다음 작업으로.`
2. Capture word opacity before the section enters, while it crosses the configured
   range, and after it completes; confirm the seven values advance sequentially
   from approximately `0.22` to `1` and reverse when scrolling upward.
3. Confirm the two authored lines remain intact, page horizontal overflow is `0`,
   and the browser console contains no error.
4. Repeat in resolved light and dark themes.

- [ ] **Step 3: Verify reduced motion**

Emulate `prefers-reduced-motion: reduce`, reload the local page, and inspect all
seven `.tagline__word` elements.

Expected: every word has computed opacity `1`, the exact spaced heading text is
unchanged, and scrolling does not change opacity.

- [ ] **Step 4: Verify GitHub Pages and the public page**

Run:

```bash
gh run list --workflow pages-build-deployment --limit 5
curl -fsSI https://chann.github.io/cli-tools/
git rev-parse HEAD
git rev-parse @{u}
git ls-remote origin refs/heads/main
git rev-list --left-right --count HEAD...@{u}
git status --short
```

Expected: the Pages run for the feature commit succeeds, the public URL returns
HTTP 200, local/tracking/live-remote SHAs match, parity is `0 0`, and the worktree
is clean.

- [ ] **Step 5: Repeat the interaction checks on the public page**

Open `https://chann.github.io/cli-tools/` and repeat the exact heading-text,
desktop/mobile, light/dark, default-motion, reduced-motion, overflow, and console
checks from Steps 2 and 3.

Expected: the public page matches the verified local production build.
