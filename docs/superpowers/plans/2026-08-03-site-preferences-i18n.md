# Site Preferences and Internationalization Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Repair the logo-less header, expose direct theme choices, apply keep-all wrapping to prose, and publish Korean-first static routes for English, Japanese, and Simplified Chinese with refresh-safe language selection.

**Architecture:** Keep Vite and React without adding dependencies. Move shared chrome behavior and locale state into focused modules, define one catalog contract for all four locales, and use a small Vite build plugin to render localized static shells and legal/error assets while React consumes the same catalog for interactive sections.

**Tech Stack:** Vite 8, React 19, Vitest 4, Testing Library, plain CSS, GitHub Pages

## Global Constraints

- Korean remains at `/cli-tools/`; English uses `/cli-tools/en/`, Japanese uses `/cli-tools/ja/`, and Simplified Chinese uses `/cli-tools/zh/`.
- The pathname is the active-locale source of truth; `cli-tools-locale` stores the last explicit selection, and an explicit URL always wins.
- `cli-tools-theme` stores exactly `system`, `light`, or `dark`; system mode follows `prefers-color-scheme` changes.
- Natural-language text inherits `word-break: keep-all`; `pre`, `code`, `kbd`, and `samp` preserve literal code wrapping.
- CLI names, commands, paths, output formats, and machine-readable output are never translated.
- Do not add an i18n, router, or UI-component dependency.
- Preserve keyboard navigation, reduced motion, no-JavaScript static header/hero/footer content, and the existing light/dark palette.
- Stage explicit paths only, never bypass hooks, push every green checkpoint normally, and prove `HEAD...@{u} = 0 0` after each push.

---

## File structure

- `site/src/runtime/chrome.js`: theme, language, and mobile-menu controllers with injectable browser dependencies.
- `site/src/runtime/chrome.test.js`: unit tests for theme modes, media changes, storage failures, language navigation, and menu behavior.
- `site/src/i18n/locale.js`: supported-locale metadata and pathname/document mapping helpers.
- `site/src/i18n/catalogs.js`: complete `ko`, `en`, `ja`, and `zh` message catalogs.
- `site/src/i18n/catalogs.test.js`: catalog-shape, representative-copy, and fixed-command tests.
- `site/src/i18n/context.jsx`: React context resolved before first render.
- `site/src/i18n/render.js`: escaped static landing, legal, error, metadata, and structured-data rendering.
- `site/src/i18n/render.test.js`: static-route and metadata tests.
- `site/src/data/tools.js`: locale-neutral command contracts only.
- `site/src/App.jsx`: interactive content sourced from the locale context.
- `site/src/main.jsx`: resolve locale, initialize chrome, and mount the localized app.
- `site/index.html`, `site/en/index.html`, `site/ja/index.html`, `site/zh/index.html`: minimal locale entry markers transformed by Vite.
- `site/vite.config.js`: localized HTML inputs and build-time asset emission.
- `site/scripts/verify-build.mjs`: production artifact contract for all locale routes.
- `site/src/styles.css`: keep-all typography, balanced header grid, preference controls, and mobile layout.
- `site/src/App.test.jsx`: localized component and interaction coverage.
- `site/src/test/setup.js`: reset locale/theme DOM state between tests.
- `site/package.json`: make `pnpm check` verify tests, build, and generated routes.
- `site/public/privacy.html`, `site/public/terms.html`, `site/public/404.html`, `site/public/sitemap.xml`: remove superseded hand-authored outputs once the build renderer owns them.

---

### Task 1: Keep natural-language text together

**Files:**
- Modify: `site/src/styles.css`
- Modify: `site/src/App.test.jsx`

**Interfaces:**
- Consumes: existing global CSS cascade and focusable code-region contract.
- Produces: a page-wide keep-all prose rule with explicit literal-code exceptions.

- [ ] **Step 1: Write the failing CSS contract test**

Add to `site/src/App.test.jsx`:

```jsx
test("keeps prose together without changing literal code wrapping", () => {
  const css = readFileSync("src/styles.css", "utf8");

  expect(css).toMatch(/body\s*{[^}]*word-break:\s*keep-all;/s);
  expect(css).toMatch(/pre,\s*code,\s*kbd,\s*samp\s*{[^}]*word-break:\s*normal;/s);
});
```

- [ ] **Step 2: Run the focused test and verify RED**

Run: `pnpm vitest run src/App.test.jsx -t "keeps prose together"`

Expected: FAIL because `body` has no `word-break: keep-all` contract.

- [ ] **Step 3: Implement the minimal CSS contract**

Add to the existing global typography rules in `site/src/styles.css`:

```css
body {
  word-break: keep-all;
}

pre,
code,
kbd,
samp {
  word-break: normal;
}
```

Do not replace existing `white-space`, `overflow-wrap`, or focusable-region rules.

- [ ] **Step 4: Verify GREEN and the site gate**

Run:

```bash
pnpm vitest run src/App.test.jsx -t "keeps prose together"
pnpm check
git diff --check
```

Expected: focused test PASS, complete tests PASS, Vite build PASS, diff check clean.

- [ ] **Step 5: Commit and push the typography checkpoint**

```bash
git add site/src/styles.css site/src/App.test.jsx
git diff --cached --check
git commit -m "style(site): keep natural-language text together"
git push
git rev-list --left-right --count HEAD...@{u}
```

Expected parity: `0 0`.

---

### Task 2: Replace the cycling theme button with direct choices

**Files:**
- Create: `site/src/runtime/chrome.js`
- Create: `site/src/runtime/chrome.test.js`
- Modify: `site/index.html`
- Modify: `site/src/main.jsx`
- Modify: `site/src/styles.css`
- Modify: `site/src/App.test.jsx`
- Modify: `site/src/test/setup.js`

**Interfaces:**
- Consumes: `data-theme`, `data-theme-mode`, `cli-tools-theme`, and the current mobile-menu IDs.
- Produces: `THEME_MODES`, `resolveTheme(mode, prefersDark)`, and `initChrome(options)` returning a cleanup function.

- [ ] **Step 1: Write failing theme-controller tests**

Create `site/src/runtime/chrome.test.js` with fixtures containing
`[data-theme-option]` buttons and assert:

```js
expect(resolveTheme("system", true)).toBe("dark");
expect(resolveTheme("light", true)).toBe("light");

initChrome({ document, storage: localStorage, media });
document.querySelector('[data-theme-option="dark"]').click();
expect(document.documentElement.dataset.themeMode).toBe("dark");
expect(document.documentElement.dataset.theme).toBe("dark");
expect(localStorage.getItem("cli-tools-theme")).toBe("dark");
expect(document.querySelector('[data-theme-option="dark"]')?.getAttribute("aria-pressed")).toBe("true");
```

Also assert that a media `change` updates only system mode and a throwing storage
adapter does not prevent the visual change.

- [ ] **Step 2: Run the focused tests and verify RED**

Run: `pnpm vitest run src/runtime/chrome.test.js`

Expected: FAIL because `src/runtime/chrome.js` does not exist.

- [ ] **Step 3: Implement the controller**

Create `site/src/runtime/chrome.js` with this public shape:

```js
export const THEME_MODES = ["system", "light", "dark"];

export function resolveTheme(mode, prefersDark) {
  if (mode === "dark") return "dark";
  if (mode === "light") return "light";
  return prefersDark ? "dark" : "light";
}

export function initChrome({
  document = window.document,
  storage = window.localStorage,
  media = window.matchMedia("(prefers-color-scheme: dark)"),
  navigate = (href) => window.location.assign(href),
} = {}) {
  const root = document.documentElement;
  const buttons = [...document.querySelectorAll("[data-theme-option]")];
  const applyTheme = (mode) => {
    root.dataset.themeMode = mode;
    root.dataset.theme = resolveTheme(mode, media.matches);
    buttons.forEach((button) => {
      button.setAttribute("aria-pressed", String(button.dataset.themeOption === mode));
    });
  };

  // The complete implementation also registers storage, language, media, and
  // mobile-menu listeners and returns one cleanup function for those listeners.
}
```

Use guarded storage writes and update every duplicated desktop/mobile theme option.

- [ ] **Step 4: Replace header markup and old inline behavior**

In `site/index.html`:

- replace `#theme-toggle` with labelled desktop and mobile groups containing
  `button[data-theme-option="system|light|dark"]`;
- keep the early head script for flash-free theme resolution;
- remove the inline cycling-theme and menu script;
- keep `cli-tools` as the brand link and the existing navigation semantics.

In `site/src/main.jsx`, call `initChrome()` before mounting React.

- [ ] **Step 5: Balance the responsive header**

In `site/src/styles.css`:

- use content-sized desktop tracks and 8–12px gaps;
- group preference controls on one quiet surface;
- style pressed theme options without changing the semantic primary color;
- move full theme choices into the mobile menu at the existing mobile breakpoint;
- keep brand, language control, and menu button within 296px at 320px viewport.

- [ ] **Step 6: Verify GREEN and commit**

Run:

```bash
pnpm vitest run src/runtime/chrome.test.js src/App.test.jsx
pnpm check
git diff --check
```

Then explicitly stage the seven changed paths, commit, push, and prove `0 0`:

```bash
git add site/src/runtime/chrome.js site/src/runtime/chrome.test.js site/index.html site/src/main.jsx site/src/styles.css site/src/App.test.jsx site/src/test/setup.js
git diff --cached --check
git commit -m "fix(site): refine header and theme controls"
git push
git rev-list --left-right --count HEAD...@{u}
```

---

### Task 3: Define the four-locale contract and React content

**Files:**
- Create: `site/src/i18n/locale.js`
- Create: `site/src/i18n/catalogs.js`
- Create: `site/src/i18n/catalogs.test.js`
- Create: `site/src/i18n/context.jsx`
- Modify: `site/src/data/tools.js`
- Modify: `site/src/App.jsx`
- Modify: `site/src/App.test.jsx`
- Modify: `site/src/main.jsx`
- Modify: `site/src/test/setup.js`

**Interfaces:**
- Produces: `LOCALES`, `localeFromPath(pathname)`, `localizedPath(locale, documentName)`, `catalogs`, `getMessages(locale)`, `I18nProvider`, and `useI18n()`.
- `useI18n()` returns `{ locale, messages }` and throws a clear error outside its provider.

- [ ] **Step 1: Write failing locale and catalog tests**

Assert exact locale metadata:

```js
expect(localeFromPath("/cli-tools/")).toBe("ko");
expect(localeFromPath("/cli-tools/en/")).toBe("en");
expect(localeFromPath("/cli-tools/ja/privacy.html")).toBe("ja");
expect(localeFromPath("/cli-tools/zh/terms.html")).toBe("zh");
expect(localizedPath("ko", "privacy")).toBe("/cli-tools/privacy.html");
expect(localizedPath("zh", "landing")).toBe("/cli-tools/zh/");
```

Flatten the Korean key tree and require every locale to have the identical key set.
Assert representative localized headlines and that every command example is byte-
identical across catalogs.

- [ ] **Step 2: Run locale tests and verify RED**

Run: `pnpm vitest run src/i18n/catalogs.test.js`

Expected: FAIL because locale modules do not exist.

- [ ] **Step 3: Implement locale metadata and catalogs**

Define the registry in `locale.js`:

```js
export const LOCALES = {
  ko: { prefix: "", htmlLang: "ko", ogLocale: "ko_KR", label: "한국어", shortLabel: "KO" },
  en: { prefix: "en", htmlLang: "en", ogLocale: "en_US", label: "English", shortLabel: "EN" },
  ja: { prefix: "ja", htmlLang: "ja", ogLocale: "ja_JP", label: "日本語", shortLabel: "JA" },
  zh: { prefix: "zh", htmlLang: "zh-Hans", ogLocale: "zh_CN", label: "简体中文", shortLabel: "中文" },
};
```

Catalogs must contain metadata, shell, hero, sections, interactive states, tool
descriptions, utility-group prose, FAQ, final CTA, privacy, terms, not-found, and
no-script keys. Keep commands in `data/tools.js`; catalogs reference tool IDs.

- [ ] **Step 4: Localize React without prop drilling**

Wrap `<App />` in `I18nProvider` from `main.jsx`. Replace module-level Korean copy
in `App.jsx` with values from `messages`. Derive benefits, workflow steps, FAQs,
tool prose, accessible labels, and copy status from the active catalog.

- [ ] **Step 5: Verify localized React behavior**

Render the app with each locale and assert one headline, one tool description, one
FAQ, localized copy feedback, and unchanged CLI commands. Verify all existing tab
and clipboard tests still pass.

Do not commit yet: localized routes are required consumers of this catalog and the
feature is not independently complete.

---

### Task 4: Generate static locale routes and localized document chrome

**Files:**
- Create: `site/src/i18n/render.js`
- Create: `site/src/i18n/render.test.js`
- Create: `site/en/index.html`
- Create: `site/ja/index.html`
- Create: `site/zh/index.html`
- Create: `site/scripts/verify-build.mjs`
- Modify: `site/index.html`
- Modify: `site/vite.config.js`
- Modify: `site/package.json`
- Modify: `site/public/legal.css`
- Delete: `site/public/privacy.html`
- Delete: `site/public/terms.html`
- Delete: `site/public/404.html`
- Delete: `site/public/sitemap.xml`

**Interfaces:**
- Consumes: locale metadata and catalogs from Task 3.
- Produces: `renderLanding(locale)`, `renderLegal(locale, kind)`, `renderNotFound()`, and `renderSitemap()`; Vite outputs all public locale routes.

- [ ] **Step 1: Write failing static-render tests**

For every locale, call `renderLanding(locale)` and assert:

```js
expect(html).toContain(`<html lang="${LOCALES[locale].htmlLang}"`);
expect(html).toContain(`data-locale="${locale}"`);
expect(html).toContain('rel="alternate" hreflang="ko"');
expect(html).toContain('data-theme-option="system"');
expect(html).not.toMatch(/\{\{[^}]+\}\}/);
```

Assert localized title, description, FAQ JSON-LD, hero copy, language links, legal
content, absolute `/cli-tools/` asset paths, and escaped text.

- [ ] **Step 2: Run static-render tests and verify RED**

Run: `pnpm vitest run src/i18n/render.test.js`

Expected: FAIL because the renderer does not exist.

- [ ] **Step 3: Implement escaped renderers and Vite inputs**

Implement `escapeHtml` and `jsonForScript` helpers; never interpolate catalog text
without the appropriate helper. Render the current static header, hero, footer,
early theme bootstrap, metadata, and no-script copy from the active catalog.

Use four minimal locale entry files and a Vite `transformIndexHtml` hook that
replaces each marker with `renderLanding(locale)`. Configure Rollup HTML inputs for
root, `en`, `ja`, and `zh`. Emit localized privacy/terms pages, one path-aware 404,
and a locale-aware sitemap during bundle generation.

- [ ] **Step 4: Connect language selection**

Extend `initChrome()` to read `select[data-language-select]`, store the selected
locale, and navigate to its exact option URL. Duplicate selectors in mobile chrome
must stay synchronized. Storage exceptions must not stop navigation.

- [ ] **Step 5: Add production artifact verification**

`site/scripts/verify-build.mjs` must read:

```text
dist/index.html
dist/en/index.html
dist/ja/index.html
dist/zh/index.html
dist/privacy.html
dist/en/privacy.html
dist/ja/privacy.html
dist/zh/privacy.html
dist/terms.html
dist/en/terms.html
dist/ja/terms.html
dist/zh/terms.html
dist/404.html
dist/sitemap.xml
```

It must assert locale-specific `lang`, headline, canonical, no unresolved source
entry, and one shared hashed JS application asset. Update `pnpm check` to run
Vitest, Vite build, and this verifier in order.

- [ ] **Step 6: Verify the complete internationalization checkpoint**

Run:

```bash
pnpm check
git diff --check
```

Expected: all tests PASS, Vite multi-page build PASS, route verifier PASS.

- [ ] **Step 7: Commit and push**

Review the complete i18n diff, explicitly stage only Task 3 and Task 4 paths, then:

```bash
git commit -m "feat(site): add persistent locale routes"
git push
git rev-list --left-right --count HEAD...@{u}
```

Expected parity: `0 0`.

---

### Task 5: Browser, accessibility, deployment, and public-route proof

**Files:**
- Modify only if QA reveals a defect; any correction receives its own test and commit.

**Interfaces:**
- Consumes: production `site/dist` and the Pages deployment from Task 4.
- Produces: evidence for responsive layout, persistence, accessibility, deployment, and remote parity.

- [ ] **Step 1: Run a local production preview**

Run `pnpm preview --host 127.0.0.1` from `site/` and use the announced port.

- [ ] **Step 2: Verify locale and theme persistence in a real browser**

At minimum:

- open Korean root, select English, confirm `/cli-tools/en/`, reload, and confirm
  English text and `html[lang="en"]`;
- switch to Japanese and Chinese and confirm `/ja/` and `/zh/` with no console errors;
- select system, light, and dark; reload after each explicit mode and confirm both
  `data-theme-mode` and resolved `data-theme`;
- change mocked/emulated system preference and confirm only system mode follows it.

- [ ] **Step 3: Verify responsive and accessible states**

At 1440, 960, 720, 390, 375, and 320px, inspect all four locales for:

- no document horizontal overflow;
- no mid-word CJK breaks in headings and controls;
- header controls inside the viewport with usable touch targets;
- keyboard focus for theme, language, menu, tabs, and code regions;
- zero automated accessibility violations in resolved light and dark themes.

- [ ] **Step 4: Run the final local gate and audit Git**

Run:

```bash
pnpm check
git diff --check
git status --short --branch
git log --oneline <starting-sha>..HEAD
git rev-list --left-right --count HEAD...@{u}
```

If QA required a correction, write its failing regression test, verify RED/GREEN,
commit it with an outcome-based Conventional Commit, push, and re-run this audit.

- [ ] **Step 5: Verify Pages and public routes**

Require the workflow for the final SHA to complete successfully. Verify 200 responses
and representative localized content at:

```text
https://chann.github.io/cli-tools/
https://chann.github.io/cli-tools/en/
https://chann.github.io/cli-tools/ja/
https://chann.github.io/cli-tools/zh/
```

Confirm local, tracking, and `git ls-remote origin refs/heads/main` SHAs are identical.
