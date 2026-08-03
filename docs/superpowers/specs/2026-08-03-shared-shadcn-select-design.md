# Shared shadcn preference Select design

Date: 2026-08-03
Status: Approved

## Context

The landing-page header currently renders language with a native `<select>` and
theme with a separately styled imperative menu. Their trigger heights, corner
radii, chevrons, and right padding therefore drift even when each control looks
reasonable in isolation. The language arrow is especially close to the right
edge.

This design supersedes the custom preference-control portion of
`2026-08-03-site-preferences-i18n-design.md`. Locale routes, theme storage,
localized static output, and every other contract from that design remain in
force.

## Goals

- Render language and theme with the same real shadcn/ui `Select` primitives.
- Use one shared trigger composition for height, radius, surface, typography,
  focus treatment, chevron, and item layout.
- Increase the chevron's right inset from the current 8px to the shadcn trigger's
  12px horizontal padding.
- Preserve Korean-first localized routes and refresh-safe language selection.
- Preserve system, light, and dark theme modes and flash-free stored-theme
  resolution.
- Preserve the current restrained palette, compact GNB, mobile menu, and
  no-horizontal-overflow contract.

## Non-goals

- Redesigning the rest of the website around Tailwind utilities.
- Replacing the existing color system, typography, navigation, or content.
- Adding search, typeahead, multi-select, or locale negotiation.
- Changing legal-page language links or the localized URL structure.

## Selected approach

Initialize shadcn/ui for the existing Vite JavaScript application with the Radix
base and CSS variables, then add the official `Select` source through the shadcn
CLI. Tailwind v4 is introduced only as the component styling engine; the existing
hand-authored site CSS remains authoritative for the rest of the product.

Both preferences use a shared `PreferenceSelect` wrapper composed from:

- `Select`
- `SelectTrigger`
- `SelectValue`
- `SelectContent`
- `SelectGroup`
- `SelectItem`

This is preferable to a shared `DropdownMenu` because both controls represent one
selected value from a fixed list. It is preferable to two native selects because
the request explicitly requires shadcn/ui as the common component foundation.

## Component architecture

### shadcn foundation

- `components.json` records the actual shadcn registry, Radix base, aliases, and
  global CSS file.
- `src/components/ui/select.jsx` is generated from the official registry and is
  treated as owned source code.
- `src/lib/utils.js` provides the generated `cn()` utility.
- Vite and the JavaScript project config resolve the `@/` alias.
- Semantic shadcn color variables map to the site's existing light and dark
  tokens instead of introducing a second visual theme.

### Shared preference component

`src/components/preferences.jsx` owns `PreferenceSelect` and renders both desktop
and mobile preference sets from one React state tree. The two sets are portaled
into static header hosts so they cannot drift in state or markup.

The trigger uses the standard shadcn default height of 36px, 12px horizontal
padding, one standard chevron, and one shared radius. Language and theme may use
different content widths, but their component anatomy and visual tokens are
identical. Dropdown content aligns to the trigger edge and uses the same item
height, selected indicator, and focus treatment.

### Static shell and enhancement

The build-time renderer keeps localized preference hosts and lightweight fallback
controls in the initial HTML. The fallback controls share the final geometry, so
replacing them after JavaScript loads does not shift the GNB. One React preference
root clears those fallbacks and portals the shadcn controls into both hosts.

The early head theme script remains unchanged and resolves the stored theme before
paint. The main React content continues to mount independently below the static
hero.

## State and data flow

### Language

1. The active pathname resolves the locale before rendering.
2. Both shadcn language selects receive that locale as their controlled value.
3. Selecting another locale writes `cli-tools-locale` in guarded storage.
4. Navigation uses the locale registry's equivalent landing route.
5. The selected URL remains the source of truth after reload.

### Theme

1. The early bootstrap resolves `cli-tools-theme` into `data-theme-mode` and
   `data-theme`.
2. The shared preference root initializes from `data-theme-mode`.
3. Selecting system, light, or dark updates both data attributes and guarded
   storage immediately.
4. A media-query listener updates the resolved theme only while system mode is
   active.
5. Desktop and mobile controls stay synchronized because they share one state.

Storage failures never block the current visual change or language navigation.

## Responsive behavior

- Desktop keeps both compact shadcn selects in the preference cluster.
- At 720px and below, only the compact language select remains in the fixed GNB;
  the full language and theme selects remain in the mobile menu.
- The desktop language trigger uses the short locale label; mobile uses the full
  localized language label.
- Acceptance widths remain 320, 375, 390, 720, 960, and 1440px for all four
  locales, with zero horizontal overflow.

## Accessibility

- Each trigger has a localized accessible label.
- Radix Select provides listbox semantics, selected state, focus management,
  Escape handling, and arrow-key navigation.
- `SelectItem` instances always remain inside `SelectGroup`.
- Visible focus treatment uses the existing semantic focus token.
- Reduced-motion preferences disable nonessential overlay animation.

## Failure handling

- If React fails before enhancement, the localized static shell and stable
  fallback geometry remain visible.
- Invalid stored themes resolve to system; invalid locale paths remain Korean or
  use the localized 404 contract as already defined.
- A blocked storage API does not stop either preference from working for the
  current visit.

## Verification

1. Component tests prove both preferences use the same shadcn Select slots and
   keep desktop/mobile values synchronized.
2. Interaction tests prove locale navigation, guarded storage, explicit themes,
   and system-theme media changes.
3. CSS contract tests prove 36px trigger height, common radius, 12px right
   padding, and identical chevron geometry.
4. `pnpm check` proves the full Vitest suite, localized production build, four
   landing pages, eight legal pages, 404 page, and sitemap.
5. Browser QA covers keyboard selection, reload persistence, both themes, all
   four locales, responsive widths, console errors, and Axe violations.
6. The final push requires Pages build/deploy success and local, tracking, and
   live-remote `0 0` parity.

## Realtime checkpoints

1. `docs(site): design shared shadcn selects`
2. `refactor(site): share shadcn preference selects`

Any correction discovered after publication becomes a new verified commit; no
pushed checkpoint is rewritten.
