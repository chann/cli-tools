# Site preferences and internationalization design

Date: 2026-08-03  
Status: Approved by delegated user decision

## Context

The landing page currently combines a static Korean shell in `site/index.html`
with React-rendered sections in `site/src/App.jsx`. Its header previously used a
`>_` mark; removing that mark exposed an oversized grid track and uneven spacing.
Theme selection also cycles through system, light, and dark modes with one button,
so the available choices are not visible before interaction.

The site must remain Korean-first while adding English, Japanese, and Simplified
Chinese. A selected language must remain active after reload.

## Goals

- Remove the header's phantom space and give every control an intentional place.
- Expose system, light, and dark as direct, mutually exclusive theme choices.
- Publish Korean at `/cli-tools/`, English at `/cli-tools/en/`, Japanese at
  `/cli-tools/ja/`, and Simplified Chinese at `/cli-tools/zh/`.
- Preserve the active locale across navigation and reload without a language flash.
- Translate visible copy, interaction feedback, accessibility text, document
  metadata, structured data, and legal/error pages.
- Preserve command names, command examples, paths, output formats, and product
  names exactly where they are part of the CLI contract.
- Keep the existing restrained product visual system, responsive behavior,
  keyboard interactions, and reduced-motion support.

## Non-goals

- Runtime translation services or new localization dependencies.
- Automatic translation of terminal commands or machine-readable output.
- Locale negotiation on a server; GitHub Pages remains the hosting target.
- Additional languages or right-to-left layout in this iteration.

## Considered approaches

### 1. Build-time localized static routes — selected

Generate locale-specific HTML from one template and shared message catalogs. Each
URL returns the correct language on its first response, while React reads the same
locale catalog for interactive sections.

This approach gives direct-route reliability, localized metadata, no post-load
language swap, and one source of truth for translations. It adds a small build
step, but that step is deterministic and testable.

### 2. Client-only SPA localization

Render one HTML shell and choose the locale in JavaScript from the pathname. This
has less build code but creates a Korean-first response for every route, weaker
metadata, and a visible or semantic language mismatch before hydration.

### 3. Four hand-maintained page trees

Maintain separate HTML and React content per locale. Runtime behavior is simple,
but duplicated structure will drift and makes completeness hard to prove.

## Information architecture

The Korean landing page remains the canonical root for compatibility and Korean-
first discovery. Other locales use a prefix:

| Locale | Language | Landing route |
| --- | --- | --- |
| `ko` | Korean | `/cli-tools/` |
| `en` | English | `/cli-tools/en/` |
| `ja` | Japanese | `/cli-tools/ja/` |
| `zh` | Simplified Chinese (`zh-Hans`) | `/cli-tools/zh/` |

Privacy, terms, and not-found content follow the same locale mapping. Language
navigation always maps to the equivalent document, not merely the landing page.

The pathname is the source of truth. Language selection also writes
`cli-tools-locale` to local storage as a durable preference, but an explicit URL
always wins. Reload therefore renders the selected static route directly. New
visitors to the unprefixed root receive Korean.

## Header and preference controls

Desktop layout:

```text
[ cli-tools ]  [ Tools  Install  GitHub ]  [ KO v ]  [ Auto | Light | Dark ]
```

The header uses content-sized tracks and compact gaps rather than a leftover
flexible logo track. The brand remains a home link. Language and theme controls
form one preference cluster with a shared surface treatment.

Theme selection is an accessible three-option control. Each option has a visible
localized label, `aria-pressed`, and a minimum pointer target. System mode follows
`prefers-color-scheme` changes; explicit light or dark modes do not. The selection
is stored in `cli-tools-theme` and applied in an early head script to prevent a
theme flash.

At narrow widths, primary navigation and the full theme control move into the
mobile menu. The fixed header keeps only the brand, compact language selector,
and menu button so it remains usable at 320px without shrinking touch targets.

## Localization architecture

- A locale registry owns supported locale codes, labels, route prefixes, HTML
  language values, and Open Graph locale values.
- Message catalogs share an identical nested key shape. Tests reject missing or
  extra keys.
- Catalog entries cover static shell copy, React section content, tool labels and
  descriptions, UI states, accessible names, metadata, FAQ structured data, and
  legal/error pages.
- A build-time renderer produces every localized HTML entry and `hreflang`
  alternates from the registry and catalogs.
- React receives the locale resolved from the pathname before its first render and
  reads the matching catalog. It does not infer locale from browser language.
- Language links preserve the current document and save the selected locale before
  navigation.
- Fixed CLI terms remain unchanged. Natural-language descriptions around them are
  translated.

## State and failure handling

- Unknown locale prefixes resolve through the existing not-found surface instead
  of silently presenting the wrong language.
- Missing or invalid stored locale values are ignored; Korean remains the default.
- Local-storage failures do not block language navigation or theme changes.
- If JavaScript is unavailable, each route still exposes its localized static
  header, hero, footer, metadata, and no-script guidance.
- Clipboard failures remain visible in the active locale.

## Accessibility and responsive behavior

- Theme options use a labelled group and explicit selected states.
- Language selection has a localized accessible name and native keyboard behavior.
- Focus indicators, Escape-to-close behavior, reduced motion, and existing tablist
  keyboard behavior remain intact.
- Header wrapping and document overflow are acceptance criteria at 320, 375, 390,
  720, 960, and 1440px in every locale.
- Both light and dark resolved themes are audited because selected-state contrast
  can differ from the default page.

## Testing and verification

1. Catalog tests prove key parity and supported locale metadata.
2. Route-generation tests prove all landing, legal, and error outputs exist with
   correct `lang`, canonical, `hreflang`, metadata, and representative copy.
3. Component tests cover localized interactive content, copy feedback, theme
   selection, system-theme changes, language navigation, and storage failures.
4. `pnpm check` runs the complete Vitest suite and production Vite build.
5. Browser QA covers each locale, language persistence after reload, all three
   theme modes, keyboard interaction, console errors, accessibility, and horizontal
   overflow at desktop and narrow mobile widths.
6. After each implementation checkpoint, push normally and prove
   `HEAD...@{u} = 0 0`.
7. After the final push, require successful Pages build/deploy jobs and verify the
   public localized URLs and persisted reload behavior.

## Realtime checkpoints

1. `docs(site): define localization and preferences design`
2. `fix(site): refine header and theme controls`
3. `feat(site): add persistent locale routes`

If final QA reveals a defect, the correction becomes a separate verified commit;
published checkpoints are never rewritten.
