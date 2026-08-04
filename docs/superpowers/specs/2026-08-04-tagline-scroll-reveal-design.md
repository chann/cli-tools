# Tagline spacing and scroll reveal design

Date: 2026-08-04
Status: Approved

## Context

The landing page tagline is split into word-level React elements so each word can
animate independently:

```text
터미널을 떠나지 않고,
분석하고 정리하고 다음 작업으로.
```

The current markup places adjacent word elements next to one another without real
space characters. CSS margins create visual gaps, but copied text, assistive
technology, and styles-disabled output collapse the words. The current animation
also changes only the word color after each word enters the viewport, so the
movement is too subtle to communicate the intended reveal.

The reference at `https://chann.github.io/skills/` preserves literal spaces and
maps the section's scroll progress across the words, progressively changing each
word from muted to fully visible.

## Goals

- Keep the current Korean tagline, translations, and intentional two-line layout.
- Preserve real spaces between words in rendered text and accessible output.
- Reveal words sequentially as the reader scrolls through the tagline section.
- Make the animation legible without adding decorative motion elsewhere.
- Show the complete sentence immediately when reduced motion is requested.
- Preserve responsive typography, natural-language `word-break: keep-all`, and
  all existing page behavior.

## Non-goals

- Replacing the tagline with the reference site's copy.
- Changing the landing-page information architecture, typography, or palette.
- Adding animation to the hero, benefits, or other sections.
- Changing locale catalogs or their word-level data shape.

## Considered approaches

### 1. Scroll-progress opacity reveal — selected

Measure the tagline heading's progress through the viewport and assign each word
a consecutive progress range. Within its range, a word transitions from muted
opacity to full opacity. Literal spaces are rendered after the word elements.

This is the closest match to the reference. It makes scrolling directly control
the reveal and avoids a one-time animation firing before the reader notices it.

### 2. One-time staggered entrance

Use `whileInView` to animate each word upward and into view once. This is visually
stronger than the current color transition, but the animation is time-based rather
than scroll-linked and can complete while the section is only partially visible.

### 3. CSS class-based fade

Use an intersection observer to add one visible class and stagger transitions
with CSS delays. This is simpler but provides only a binary entered/not-entered
state and does not reproduce the reference's continuous scroll response.

## Component design

`TaglineReveal` remains the owner of the section and localized line data. It adds
a heading ref and derives one shared scroll-progress value from that heading.

A small word component receives the shared progress, its normalized start/end
range, and the reduced-motion state. It maps the range to an opacity from the
muted state to fully visible. The ranges are calculated from each word's global
position across all lines, so the reveal order continues naturally across the
line break.

Every word except the final word is followed by one literal space in the React
output, including the boundary between the two authored lines. CSS no longer uses
right margins as a substitute for text spacing. The line wrapper continues to
render as a block, preserving the authored two-line composition in every locale.

## Motion behavior

The scroll tracker uses the tagline heading as its target. The reveal begins as
the heading approaches the lower portion of the viewport and completes before it
passes the viewport midpoint, matching the reference's readable pacing.

Words remain fully opaque after their progress range completes. Scrolling upward
reverses the opacity continuously because the visual state is derived from scroll
position rather than a persistent entered flag.

When `prefers-reduced-motion` is active, no scroll-linked opacity style is
attached. Every word renders at the normal text color and full opacity from the
first paint.

## Accessibility and failure handling

- The heading remains one semantic sentence split only by the authored line
  wrappers. Literal spaces preserve its spoken and copied form.
- The animation does not alter focus order, pointer behavior, or document flow.
- If motion is reduced, the static content is complete and visually identical to
  the animation's finished state.
- Without JavaScript, the localized static shell remains available and the React
  section degrades to the existing no-script behavior.
- Existing contrast and light/dark theme tokens remain unchanged.

## Testing and verification

1. Component tests assert the exact tagline text, including spaces and the
   intentional line boundary.
2. Component tests retain the expected word count for the Korean catalog and
   verify every word element participates in the reveal contract.
3. Source and style tests reject margin-based synthetic spacing.
4. `pnpm check` runs the full Vitest suite, production build, and generated-route
   verification.
5. Browser QA checks the tagline before, during, and after scrolling at desktop
   and mobile widths in light and dark themes.
6. Browser QA checks `prefers-reduced-motion`, console errors, horizontal overflow,
   and the accessible heading text.
7. After publication, verify the GitHub Pages deployment and the live Korean page.

## Commit plan

1. `docs(site): define tagline scroll reveal design`
2. `fix(site): restore tagline spacing and scroll reveal`

Each commit is pushed normally. Published history is not rewritten.
