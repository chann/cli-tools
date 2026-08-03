# Shared shadcn Preference Selects Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Replace the mismatched language native select and custom theme menu with one real shadcn/ui `Select` composition whose chevron has a 12px right inset.

**Architecture:** Add the official Radix-based shadcn `Select` source and the minimum Tailwind v4 foundation to the existing Vite JavaScript app. A single React preference root portals synchronized language and theme selects into desktop and mobile static hosts, while build-time fallback controls preserve localized first paint and the early theme bootstrap prevents a theme flash.

**Tech Stack:** React 19, Vite 8, Tailwind CSS 4, shadcn/ui, Radix UI, Vitest 4, Testing Library, plain build-time HTML rendering, GitHub Pages

## Global Constraints

- Korean stays at `/cli-tools/`; English, Japanese, and Simplified Chinese stay at `/cli-tools/en/`, `/cli-tools/ja/`, and `/cli-tools/zh/`.
- The selected locale URL remains the source of truth after reload.
- `cli-tools-theme` stores exactly `system`, `light`, or `dark`; only system mode follows `prefers-color-scheme` changes.
- Language and theme must both consume the same shadcn `SelectTrigger`, `SelectContent`, `SelectGroup`, and `SelectItem` primitives.
- Every `SelectItem` stays inside `SelectGroup`.
- The shared trigger is 36px high and uses the official `px-3` horizontal padding, proving a 12px chevron right inset.
- Tailwind is limited to shadcn component styling; the existing site CSS remains authoritative elsewhere.
- Preserve the current semantic palette, compact GNB, no-JavaScript localized shell, reduced motion, keep-all prose, and literal code wrapping.
- Keep zero horizontal overflow at 320, 375, 390, 720, 960, and 1440px in every locale.
- Stage explicit paths only, never bypass hooks, push each green checkpoint normally, and prove `HEAD...@{u} = 0 0`.

---

## File structure

- `site/components.json`: shadcn registry, Radix base, JavaScript mode, CSS variables, and aliases.
- `site/jsconfig.json`: `@/*` to `src/*` editor and CLI alias contract.
- `site/package.json`, `site/pnpm-lock.yaml`: Tailwind, shadcn runtime, icon, and class-merging dependencies.
- `site/vite.config.js`: Vite alias and Tailwind plugin while preserving localized multi-page emission.
- `site/src/lib/utils.js`: generated `cn()` helper.
- `site/src/components/ui/select.jsx`: official shadcn `Select` source added by the CLI.
- `site/src/components/preferences.jsx`: shared `PreferenceSelect` plus synchronized desktop/mobile portals.
- `site/src/components/preferences.test.jsx`: Select composition, theme, language, storage-failure, and synchronization tests.
- `site/src/runtime/preferences.js`: framework-independent preference storage and theme-resolution helpers.
- `site/src/runtime/preferences.test.js`: helper edge cases and system-media behavior.
- `site/src/runtime/chrome.js`: mobile-menu behavior only after custom preference logic is removed.
- `site/src/runtime/chrome.test.js`: mobile-menu regression coverage only.
- `site/src/i18n/render.js`: localized preference hosts and stable fallback controls.
- `site/src/i18n/render.test.js`: static-host, fallback, and localized option contracts.
- `site/src/main.jsx`: mount the shared preference root and existing application root.
- `site/src/styles.css`: Tailwind/shadcn semantic token bridge, fallback geometry, portal width utilities, and old custom-dropdown removal.
- `site/src/shadcn-foundation.test.js`: registry, alias, generated-source, semantic-token, and chevron-padding contracts.
- `site/src/header-layout.test.js`: fallback geometry and mobile visibility contracts.
- `site/src/App.test.jsx`: static shell assertions updated from custom menu roles to preference hosts.
- `site/scripts/verify-build.mjs`: production output must include preference hosts and one shared application asset.

---

### Task 1: Add the official shadcn Select foundation

**Files:**
- Create: `site/jsconfig.json`
- Create: `site/components.json` through the shadcn CLI
- Create: `site/src/lib/utils.js` through the shadcn CLI
- Create: `site/src/components/ui/select.jsx` through the shadcn CLI
- Create: `site/src/shadcn-foundation.test.js`
- Modify: `site/package.json`
- Modify: `site/pnpm-lock.yaml`
- Modify: `site/vite.config.js`
- Modify: `site/src/styles.css`

**Interfaces:**
- Consumes: current `--bg`, `--surface`, `--surface-muted`, `--text`, `--muted`, `--line`, `--primary`, `--primary-ink`, and `--focus` theme tokens.
- Produces: `@/components/ui/select`, `@/lib/utils`, shadcn `data-slot` attributes, and Tailwind semantic colors mapped to the current palette.

- [ ] **Step 1: Write the failing shadcn foundation contract**

Create `site/src/shadcn-foundation.test.js`:

```js
import { existsSync, readFileSync } from "node:fs";
import { describe, expect, test } from "vitest";

describe("shadcn Select foundation", () => {
  test("uses an installed Radix shadcn Select in JavaScript mode", () => {
    expect(existsSync("components.json")).toBe(true);
    expect(existsSync("src/components/ui/select.jsx")).toBe(true);
    expect(existsSync("src/lib/utils.js")).toBe(true);

    const config = JSON.parse(readFileSync("components.json", "utf8"));
    const source = readFileSync("src/components/ui/select.jsx", "utf8");
    expect(config.rsc).toBe(false);
    expect(config.tsx).toBe(false);
    expect(config.aliases.ui).toBe("@/components/ui");
    expect(source).toContain('data-slot="select-trigger"');
    expect(source).toContain('data-slot="select-content"');
    expect(source).toContain('data-slot="select-item"');
  });

  test("keeps the standard 36px trigger and 12px horizontal inset", () => {
    const source = readFileSync("src/components/ui/select.jsx", "utf8");
    expect(source).toContain("px-3");
    expect(source).toContain("data-[size=default]:h-9");
    expect(source).toContain("<ChevronDownIcon");
  });

  test("bridges shadcn colors to existing semantic tokens", () => {
    const css = readFileSync("src/styles.css", "utf8");
    expect(css).toContain("@import \"tailwindcss\"");
    expect(css).toContain("--color-background: var(--bg)");
    expect(css).toContain("--color-popover: var(--surface)");
    expect(css).toContain("--color-ring: var(--focus)");
  });
});
```

- [ ] **Step 2: Run the foundation test and verify RED**

Run:

```bash
cd site
pnpm vitest run src/shadcn-foundation.test.js
```

Expected: FAIL because `components.json` and `src/components/ui/select.jsx` do not exist.

- [ ] **Step 3: Add Tailwind v4 and the project alias**

Run:

```bash
cd site
pnpm add -D tailwindcss @tailwindcss/vite
```

Create `site/jsconfig.json`:

```json
{
  "compilerOptions": {
    "baseUrl": ".",
    "paths": {
      "@/*": ["./src/*"]
    }
  }
}
```

Update `site/vite.config.js` without changing localized page inputs:

```js
import tailwindcss from "@tailwindcss/vite";

export default defineConfig({
  base: "/cli-tools/",
  plugins: [localizedPages(), react(), tailwindcss()],
  resolve: {
    alias: {
      "@": fileURLToPath(new URL("./src", import.meta.url)),
    },
  },
  // Existing build and test configuration remains unchanged.
});
```

- [ ] **Step 4: Initialize shadcn and add Select through the official CLI**

Run from `site/`:

```bash
pnpm dlx shadcn@latest init --template vite --base radix --preset nova --yes
pnpm dlx shadcn@latest add select --yes
pnpm dlx shadcn@latest info --json
```

Expected: `components.json` reports Vite, JavaScript, Radix, `src/styles.css`, and
the `@/components/ui` alias; the component list contains `select`. Read every
generated file before continuing. Do not accept removal of existing site CSS.

- [ ] **Step 5: Reconcile the semantic token bridge in the existing CSS**

Keep all existing CSS and add this at the top of `site/src/styles.css`:

```css
@import "tailwindcss";
@import "tw-animate-css";

@custom-variant dark (&:is([data-theme="dark"] *));

@theme inline {
  --color-background: var(--bg);
  --color-foreground: var(--text);
  --color-popover: var(--surface);
  --color-popover-foreground: var(--text);
  --color-muted: var(--surface-muted);
  --color-muted-foreground: var(--muted);
  --color-accent: var(--surface-muted);
  --color-accent-foreground: var(--text);
  --color-border: var(--line);
  --color-input: var(--line);
  --color-ring: var(--focus);
  --color-primary: var(--primary);
  --color-primary-foreground: var(--primary-ink);
  --radius-sm: 8px;
  --radius-md: 10px;
}
```

If the CLI adds generic `body` or universal-border rules, remove those duplicate
rules and keep the existing site's global reset. Keep only the imports, dark
variant, theme bridge, and generated dependencies required by `Select`.

- [ ] **Step 6: Verify the foundation turns GREEN**

Run:

```bash
cd site
pnpm vitest run src/shadcn-foundation.test.js
pnpm build
git diff --check
```

Expected: foundation tests PASS, localized Vite build PASS, diff check clean. Do
not commit yet because the component has no consumer.

---

### Task 2: Build one synchronized preference Select tree

**Files:**
- Create: `site/src/runtime/preferences.js`
- Create: `site/src/runtime/preferences.test.js`
- Create: `site/src/components/preferences.jsx`
- Create: `site/src/components/preferences.test.jsx`
- Modify: `site/src/runtime/chrome.js`
- Modify: `site/src/runtime/chrome.test.js`

**Interfaces:**
- Consumes: `LOCALES`, `localizedPath(locale)`, localized `messages.shell` labels, shadcn `Select` exports, root `data-theme-mode`, and guarded browser dependencies.
- Produces: `THEME_MODES`, `resolveTheme(mode, prefersDark)`, `readPreference(storage, key)`, `writePreference(storage, key, value)`, `applyTheme(documentRef, mode, prefersDark)`, `PreferenceSelect(props)`, and `PreferenceControls(props)`.

- [ ] **Step 1: Write failing preference helper tests**

Create `site/src/runtime/preferences.test.js`:

```js
import { describe, expect, test, vi } from "vitest";
import {
  applyTheme,
  readPreference,
  resolveTheme,
  writePreference,
} from "./preferences";

describe("preference state", () => {
  test("resolves explicit and system themes", () => {
    expect(resolveTheme("system", false)).toBe("light");
    expect(resolveTheme("system", true)).toBe("dark");
    expect(resolveTheme("light", true)).toBe("light");
    expect(resolveTheme("dark", false)).toBe("dark");
  });

  test("applies both theme data attributes", () => {
    applyTheme(document, "dark", false);
    expect(document.documentElement.dataset.themeMode).toBe("dark");
    expect(document.documentElement.dataset.theme).toBe("dark");
  });

  test("fails open when storage is blocked", () => {
    const storage = {
      getItem: vi.fn(() => { throw new Error("blocked"); }),
      setItem: vi.fn(() => { throw new Error("blocked"); }),
    };
    expect(readPreference(storage, "cli-tools-theme")).toBeNull();
    expect(() => writePreference(storage, "cli-tools-theme", "dark")).not.toThrow();
  });
});
```

- [ ] **Step 2: Run helper tests and verify RED**

Run:

```bash
cd site
pnpm vitest run src/runtime/preferences.test.js
```

Expected: FAIL because `src/runtime/preferences.js` does not exist.

- [ ] **Step 3: Implement the framework-independent preference helpers**

Create `site/src/runtime/preferences.js`:

```js
export const THEME_MODES = ["system", "light", "dark"];

export function resolveTheme(mode, prefersDark) {
  if (mode === "dark") return "dark";
  if (mode === "light") return "light";
  return prefersDark ? "dark" : "light";
}

export function readPreference(storage, key) {
  try {
    return storage.getItem(key);
  } catch {
    return null;
  }
}

export function writePreference(storage, key, value) {
  try {
    storage.setItem(key, value);
  } catch {
    // URL navigation and the current visual state still work for this visit.
  }
}

export function applyTheme(documentRef, mode, prefersDark) {
  const safeMode = THEME_MODES.includes(mode) ? mode : "system";
  documentRef.documentElement.dataset.themeMode = safeMode;
  documentRef.documentElement.dataset.theme = resolveTheme(safeMode, prefersDark);
}
```

Run `pnpm vitest run src/runtime/preferences.test.js` and expect PASS.

- [ ] **Step 4: Write failing shared-component tests**

Create desktop and mobile host elements, then render one `PreferenceControls`
tree inside `I18nProvider`. The test must include these assertions:

```jsx
expect(screen.getAllByRole("combobox", { name: "언어 선택" })).toHaveLength(2);
expect(screen.getAllByRole("combobox", { name: "테마 선택" })).toHaveLength(2);
expect(document.querySelectorAll('[data-slot="select-trigger"]')).toHaveLength(4);

const triggerClasses = [...document.querySelectorAll('[data-slot="select-trigger"]')]
  .map((trigger) => trigger.className.replace(/w-\[[^\]]+\]/g, ""));
expect(new Set(triggerClasses).size).toBe(1);
```

Use `userEvent` to select `dark` from the desktop theme control and assert:

```jsx
expect(document.documentElement.dataset.themeMode).toBe("dark");
expect(document.documentElement.dataset.theme).toBe("dark");
expect(localStorage.getItem("cli-tools-theme")).toBe("dark");
expect(screen.getAllByRole("combobox", { name: "테마 선택" })[1]).toHaveTextContent("다크");
```

Select Japanese from the compact language control and assert guarded storage plus
`navigate("/cli-tools/ja/")`. Repeat with a throwing storage adapter to prove
navigation and current-visit theme updates remain functional.

- [ ] **Step 5: Run component tests and verify RED**

Run:

```bash
cd site
pnpm vitest run src/components/preferences.test.jsx
```

Expected: FAIL because `src/components/preferences.jsx` does not exist.

- [ ] **Step 6: Implement the shared shadcn preference component**

Create `site/src/components/preferences.jsx` with this public structure:

```jsx
import { createPortal } from "react-dom";
import { useEffect, useMemo, useState } from "react";
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { LOCALES, localizedPath } from "@/i18n/locale";
import {
  THEME_MODES,
  applyTheme,
  readPreference,
  writePreference,
} from "@/runtime/preferences";

export function PreferenceSelect({ ariaLabel, value, options, widthClass, onValueChange }) {
  return (
    <Select value={value} onValueChange={onValueChange}>
      <SelectTrigger aria-label={ariaLabel} className={widthClass}>
        <SelectValue />
      </SelectTrigger>
      <SelectContent position="popper" align="end">
        <SelectGroup>
          {options.map((option) => (
            <SelectItem key={option.value} value={option.value}>
              {option.label}
            </SelectItem>
          ))}
        </SelectGroup>
      </SelectContent>
    </Select>
  );
}
```

`PreferenceControls` must:

- initialize theme mode from valid `documentRef.documentElement.dataset.themeMode`,
  then valid stored mode, then `system`;
- use one `themeMode` state for both portals;
- build compact language options from `LOCALES[*].shortLabel` and full options
  from `LOCALES[*].label`;
- call `writePreference(storage, "cli-tools-locale", nextLocale)` and
  `navigate(localizedPath(nextLocale))`;
- call `writePreference(storage, "cli-tools-theme", nextMode)` and
  `applyTheme(documentRef, nextMode, media.matches)`;
- subscribe to media changes and only re-resolve while `themeMode === "system"`;
- portal a desktop set into `hosts.desktop` and labelled full-width mobile groups
  into `hosts.mobile`;
- use layout-only width classes: compact language `w-[72px]`, desktop theme
  `w-[104px]`, and mobile controls `w-[136px]`.

- [ ] **Step 7: Remove preference ownership from the chrome controller**

Keep `initChrome()` responsible only for the localized mobile-menu open/close
contract. Delete custom theme-menu, language-select, storage, and media listeners
from `site/src/runtime/chrome.js`. Move the existing theme-resolution assertions
to `preferences.test.js`; keep the two localized mobile-menu assertions in
`chrome.test.js`.

- [ ] **Step 8: Verify shared state and keyboard behavior GREEN**

Run:

```bash
cd site
pnpm vitest run src/runtime/preferences.test.js src/components/preferences.test.jsx src/runtime/chrome.test.js
```

Expected: all helper, component, and mobile chrome tests PASS.

---

### Task 3: Integrate localized static hosts and stable fallback geometry

**Files:**
- Modify: `site/src/i18n/render.js`
- Modify: `site/src/i18n/render.test.js`
- Modify: `site/src/main.jsx`
- Modify: `site/src/styles.css`
- Modify: `site/src/header-layout.test.js`
- Modify: `site/src/App.test.jsx`
- Modify: `site/scripts/verify-build.mjs`

**Interfaces:**
- Consumes: `PreferenceControls`, active locale, localized shell messages, current build-time renderer, and existing mobile-menu hosts.
- Produces: `[data-preference-host="desktop"]`, `[data-preference-host="mobile"]`, `#preferences-root`, and layout-stable fallback controls.

- [ ] **Step 1: Write failing static-renderer and layout contracts**

Replace old custom-menu assertions with:

```js
expect(page.querySelectorAll("[data-preference-host]")).toHaveLength(2);
expect(page.querySelectorAll("[data-preference-fallback]")).toHaveLength(4);
expect(page.getElementById("preferences-root")).not.toBeNull();
expect(page.querySelector("[data-theme-menu]")).toBeNull();
expect(page.querySelector(".language-select")).toBeNull();
```

In `header-layout.test.js`, insert fallback wrappers and assert:

```js
expect(getComputedStyle(fallbackSelect).height).toBe("36px");
expect(getComputedStyle(fallbackSelect).paddingRight).toBe("32px");
expect(getComputedStyle(fallbackSelect).borderRadius).toBe("10px");
expect(css).toMatch(/\.preference-fallback::after\s*{[^}]*right:\s*12px;/s);
```

Run the focused tests and expect RED because the renderer still emits a native
language select and custom theme menu.

- [ ] **Step 2: Replace custom static controls with shared fallback markup**

In `site/src/i18n/render.js`, replace `renderLanguageSelect` and
`renderThemeDropdown` with one `renderPreferenceFallback` helper:

```js
function renderPreferenceFallback({ ariaLabel, value, options, width, name }) {
  const renderedOptions = options
    .map((option) => `<option value="${escapeHtml(option.value)}"${option.value === value ? " selected" : ""}>${escapeHtml(option.label)}</option>`)
    .join("");
  return `<span class="preference-fallback preference-fallback--${width}" data-preference-fallback>
    <select aria-label="${escapeHtml(ariaLabel)}" name="${name}">${renderedOptions}</select>
  </span>`;
}
```

Render compact language plus theme inside the desktop host and full language plus
theme inside the mobile host. Add an empty `<div id="preferences-root"></div>`
next to the static header. Keep the early theme bootstrap unchanged.

- [ ] **Step 3: Mount one React preference tree before the main app**

In `site/src/main.jsx`:

```jsx
const hosts = {
  desktop: document.querySelector('[data-preference-host="desktop"]'),
  mobile: document.querySelector('[data-preference-host="mobile"]'),
};
hosts.desktop?.replaceChildren();
hosts.mobile?.replaceChildren();

ReactDOM.createRoot(document.getElementById("preferences-root")).render(
  <React.StrictMode>
    <I18nProvider locale={locale}>
      <PreferenceControls hosts={hosts} />
    </I18nProvider>
  </React.StrictMode>,
);
```

Then mount the existing `App` root exactly as before and call the mobile-only
`initChrome()` once.

- [ ] **Step 4: Replace old control CSS with fallback and portal layout CSS**

Delete `.language-select`, `.theme-dropdown`, `.theme-dropdown__trigger`,
`.theme-dropdown__content`, and their mobile modifiers. Add:

```css
.preference-host,
.preference-controls {
  display: flex;
  align-items: center;
  gap: 6px;
}

.preference-fallback {
  position: relative;
  display: inline-flex;
}

.preference-fallback select {
  height: 36px;
  padding: 6px 32px 6px 12px;
  appearance: none;
  border: 1px solid var(--line);
  border-radius: 10px;
  background: transparent;
  color: var(--text);
  font: inherit;
  font-size: 0.75rem;
  font-weight: 600;
}

.preference-fallback::after {
  position: absolute;
  top: 50%;
  right: 12px;
  width: 6px;
  height: 6px;
  border-right: 1.5px solid currentColor;
  border-bottom: 1.5px solid currentColor;
  content: "";
  pointer-events: none;
  transform: translateY(-70%) rotate(45deg);
}

.preference-fallback--compact select { width: 72px; }
.preference-fallback--theme select { width: 104px; }
.preference-fallback--mobile select { width: 136px; }
```

Use `.preference-controls--mobile` for the existing vertical labelled mobile
layout. At `max-width: 720px`, hide only `[data-preference-kind="theme"]` in the
desktop host; keep the compact language Select visible.

- [ ] **Step 5: Extend production artifact verification**

For every landing HTML file in `site/scripts/verify-build.mjs`, require:

```js
requireText(landing, 'data-preference-host="desktop"', landingPath);
requireText(landing, 'data-preference-host="mobile"', landingPath);
requireText(landing, 'id="preferences-root"', landingPath);
```

Reject stale custom markup:

```js
if (landing.includes("data-theme-menu") || landing.includes("language-select")) {
  throw new Error(`${landingPath} still contains legacy preference markup`);
}
```

- [ ] **Step 6: Run focused integration tests and complete gate**

Run:

```bash
cd site
pnpm vitest run src/shadcn-foundation.test.js src/components/preferences.test.jsx src/runtime/preferences.test.js src/runtime/chrome.test.js src/i18n/render.test.js src/header-layout.test.js src/App.test.jsx
pnpm check
git diff --check
pnpm dlx shadcn@latest info --json
```

Expected: all tests PASS; Vite emits four localized landing pages, eight legal
pages, localized 404, and sitemap; shadcn reports the `select` component; diff
check is clean.

- [ ] **Step 7: Review and publish the implementation checkpoint**

Review the complete diff, generated component source, dependencies, semantic
tokens, and deletion of legacy control code. Confirm no suspected secret paths.
Stage every implementation path explicitly, then:

```bash
git diff --cached --check
git commit -m "refactor(site): share shadcn preference selects"
git push
git status --short --branch
git rev-list --left-right --count HEAD...@{u}
```

Expected parity: `0 0`.

---

### Task 4: Prove the deployed preference controls

**Files:**
- Modify only if QA discovers a concrete defect; publish each correction as a new verified commit.

**Interfaces:**
- Consumes: built site, local preview, pushed commit SHA, Pages workflow, and public localized routes.
- Produces: runtime evidence for shared component identity, chevron inset, state persistence, accessibility, responsiveness, deployment, and remote parity.

- [ ] **Step 1: Start the production preview and inspect the real DOM**

Run `pnpm preview --host 127.0.0.1` from `site/`, use the announced port, and
verify with `agent-browser`:

```js
const triggers = [...document.querySelectorAll('[data-slot="select-trigger"]')];
const geometry = triggers.map((trigger) => {
  const rect = trigger.getBoundingClientRect();
  const chevron = trigger.querySelector("svg").getBoundingClientRect();
  return {
    height: Math.round(rect.height),
    radius: getComputedStyle(trigger).borderRadius,
    chevronRightInset: Math.round(rect.right - chevron.right),
  };
});
```

Expected for every trigger: height `36`, radius `10px`, chevron right inset `12`.

- [ ] **Step 2: Verify behavior and responsive states**

- Select Japanese and reload; require `/cli-tools/ja/`, `lang="ja"`, and stored
  locale `ja`.
- Select dark and reload; require `data-theme-mode="dark"`, resolved theme
  `dark`, and stored theme `dark`.
- Return to system mode, change emulated system color scheme, and require the
  resolved theme to follow it.
- Open both desktop and mobile selects with keyboard, move with arrow keys,
  choose with Enter, and close with Escape.
- At 320, 375, 390, 720, 960, and 1440px in every locale, require
  `document.documentElement.scrollWidth - innerWidth === 0`.
- Require zero browser console errors.

- [ ] **Step 3: Run accessibility and production checks**

Run `agent-browser a11y --json` in both resolved themes and require zero
violations. Report gradient contrast as `incomplete` rather than a violation if
the existing hero remains unmeasurable. Run `pnpm check` once more on the exact
tree being reported.

- [ ] **Step 4: Wait for Pages and verify the live site**

Use `gh run list --commit <final-sha>` and `gh run watch <run-id> --exit-status`.
After success, verify 200 responses and localized titles at:

- `https://chann.github.io/cli-tools/`
- `https://chann.github.io/cli-tools/en/`
- `https://chann.github.io/cli-tools/ja/`
- `https://chann.github.io/cli-tools/zh/`

Repeat the shared-trigger geometry, language reload, theme reload, mobile
overflow, console-error, and Axe checks against the public deployment.

- [ ] **Step 5: Complete the remote audit**

Run:

```bash
git status --porcelain=v1
git rev-list --left-right --count HEAD...@{u}
git rev-parse HEAD
git rev-parse @{u}
git ls-remote origin refs/heads/main
git log --oneline 0facf73..HEAD
```

Expected: clean worktree, parity `0 0`, identical local/tracking/live-remote SHA,
and only the planned documentation and implementation checkpoints.
