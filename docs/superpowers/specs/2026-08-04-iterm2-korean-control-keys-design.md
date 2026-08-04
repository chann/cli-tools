# iTerm2 Korean Control Keys Design

Date: 2026-08-04
Status: Approved

## Context

With Apple's 2-Set Korean input source active, the current iTerm2 session does
not reliably deliver the terminal control chords `Control-C` and `Control-G`.
The shell and a fresh diagnostic PTY already use the expected terminal settings,
including `intr = ^C`, so the first fix belongs before the PTY rather than in
zsh, tmux, or each terminal program.

Research on the current machine found iTerm2 `3.6.11`, no conflicting global or
profile mapping for either chord, and no installed Karabiner-Elements or
Hammerspoon runtime. The user selected the narrow iTerm2-only native-mapping
approach before considering other terminals or a system-wide remapper.

## Goals

- Make the physical `Control-C` key chord send byte `0x03` in iTerm2 while the
  Korean input source remains selected.
- Make the physical `Control-G` key chord send byte `0x07` under the same
  conditions.
- Apply the behavior globally in iTerm2 so every profile inherits it unless a
  profile has an explicit, compatible override.
- Preserve every existing global and profile key mapping structurally exactly,
  including fields unknown to the migration.
- Make the change idempotent, conflict-blocking, verifiable, and exactly
  reversible from a private backup when no later edit conflicts.

## Non-goals

- Adding Ghostty or Terminal.app mappings.
- Mapping every `Control-A` through `Control-Z` chord.
- Switching the macOS input source, interrupting Korean composition, or
  installing an input-event daemon, virtual keyboard, LaunchAgent, event tap,
  Karabiner-Elements, or Hammerspoon.
- Shipping a generalized key-management CLI.
- Changing shell, tmux, Readline, editor, or application key bindings.
- Synthesizing key events as a substitute for a physical-key acceptance test.

## Selected Approach

Use two iTerm2 global key mappings and enable iTerm2's built-in
`LanguageAgnosticKeyBindings` preference, which the UI labels **Interpret key
bindings based on physical key, ignoring input language**.

The mappings use iTerm2's **Send Hex Code** action:

| Physical chord | Virtual keycode | Global map key | Action value | Text |
| --- | ---: | --- | ---: | --- |
| `Control-C` | `0x08` | `0x63-0x40000-0x8` | `11` | `0x03` |
| `Control-G` | `0x05` | `0x67-0x40000-0x5` | `11` | `0x07` |

These values are source-validated against iTerm2 `3.6.11`: Cocoa's Control
modifier is `0x40000`, the ANSI C and G virtual keycodes are `0x08` and `0x05`,
and action `11` is `HEX_CODE`. The leading character records the ABC-layout
unmodified character. Once language-agnostic lookup is enabled, iTerm2 compares
the modifier and virtual-keycode portion so the binding follows the physical
key under Korean input.

The physical-key preference is global and therefore changes lookup behavior for
all existing mappings that already carry virtual keycodes. Before mutation, the
implementation must audit those mappings for portable-key collisions. Runtime
verification must also exercise the current shortcuts that contain keycodes;
the change is not accepted merely because the two new entries exist.

## Configuration Boundary

The implementation is a one-time, narrow migration through iTerm2's Python API,
not a resident process and not a new end-user CLI. A repository-tested operator
script may be executed once and copied beside the ownership receipt for future
restore, but it is not installed on `PATH` and does not remain in the keyboard
path after applying the preferences.

Use the generic iTerm2 preference RPC to read and write the raw `GlobalKeyMap`
dictionary and `LanguageAgnosticKeyBindings` value. Do not edit iTerm2's live
plist file and do not replace the map through
`async_set_global_key_bindings()`: that high-level helper decodes and re-encodes
every entry and can discard existing fields such as `Keycode`, `Modifiers`,
`Apply Mode`, and `Escaping`.

The raw dictionary mutation must copy the original object, add only the two
approved entries, and retain all unknown keys and values unchanged. Equality
and hashes use deterministic, canonical JSON so dictionary ordering does not
create false differences. Profile key maps are read for conflict detection only
and are never written.

## Preflight and Conflict Rules

Before writing anything:

1. Confirm the running application is iTerm2 `3.6.11` or a version whose
   mapping contract has been revalidated.
2. Confirm iTerm2's preference-storage mode, then read the effective global map,
   the persisted presence/value of `LanguageAgnosticKeyBindings`, and every
   profile's `Keyboard Map`. The effective values come from iTerm2; persisted
   presence comes from a read-only export of the active preference domain.
3. Canonicalize mappings by modifier plus virtual keycode so a different
   leading character cannot hide a physical-key collision.
4. Check legacy and modified-character forms that can take precedence over the
   portable physical-key lookup.
5. Abort without mutation if either target chord resolves to a different action
   globally or in any profile.
6. Treat an exact existing Send Hex mapping as already satisfied, but do not
   claim ownership of an entry created elsewhere.
7. Re-read and hash the raw global map immediately before the write; abort if it
   changed since preflight.

The current-machine audit is green, but implementation repeats it to close the
gap between research time and mutation time.

## Backup and Ownership Receipt

Before applying, create a timestamped directory beneath:

```text
~/Library/Application Support/cli-tools/iterm2-korean-control-keys/
```

The directory and files use user-only permissions. Store:

- the exact raw pre-change `GlobalKeyMap` JSON;
- whether `LanguageAgnosticKeyBindings` was persisted, plus its original value;
- a full iTerm2 preference export for disaster recovery;
- hashes of the pre-change and intended post-change maps;
- the two exact entries added by this migration; and
- the iTerm2 version, timestamp, and migration result.

The receipt contains configuration metadata only. It must not capture terminal
contents, commands, environment variables, keystrokes, or unrelated application
data beyond the private full preference export.

## Apply and Compensating Rollback

Apply in this order:

1. Write the copied global map containing only the two additions.
2. Set `LanguageAgnosticKeyBindings` to `true`.
3. Read both values back through iTerm2 and compare them with the intended
   values.
4. Confirm every pre-existing map entry is structurally identical after
   canonical serialization.

The iTerm2 API does not make the two preference writes one atomic operation. If
either write or read-back check fails, the same process immediately restores
the captured map and original preference presence/value, then verifies the
restored hashes. A failed rollback is reported prominently with the backup path;
it is never hidden by the original error.

Re-running after a successful apply is a no-op. Re-running after external edits
repeats the complete conflict and hash checks.

## Later Restore Contract

A later restore removes only entries whose keys and values still exactly match
the receipt. If either owned entry was edited, restoration stops rather than
overwriting the user's newer choice.

The physical-key preference returns to its original value or absent state only
when the rest of the current map still matches the recorded post-change state
after subtracting the two owned entries. If unrelated mappings changed after
installation, restore may safely remove the owned entries but must leave the
global physical-key preference enabled and report that conservative decision.
This avoids breaking newer user mappings that may now depend on it.

## Data Flow

```text
physical Control-C or Control-G
  -> macOS key event with modifier and virtual keycode
  -> iTerm2 language-agnostic global map lookup
  -> Send Hex Code action
  -> byte 0x03 or 0x07
  -> PTY
  -> termios or foreground terminal program
```

The selected macOS input source is observed before and after verification and
must remain Apple's 2-Set Korean input method throughout.

## Error Handling

- If iTerm2 is not running or its Python API cannot be reached, stop before
  mutation and report the exact prerequisite.
- If API authorization is denied, do not fall back to direct plist editing.
- If any global or profile collision exists, print the profile, serialized key,
  and action, then stop without writing.
- If the preferences change between preflight and apply, stop and require a new
  preflight rather than merging stale state.
- If read-back differs, perform compensating rollback and verify it.
- If the active input source changes during the acceptance test, invalidate the
  result and repeat with Korean active.
- If physical input still does not deliver the expected byte, restore the
  original preferences before investigating a broader remapper.

## Verification

### Static and Fixture Checks

- Parse current global and profile mapping fixtures without dropping unknown
  fields.
- Prove adding the two entries changes no pre-existing key or value.
- Prove portable-key collision detection catches alternate leading characters
  with the same modifier and virtual keycode.
- Prove legacy and modified-character conflicts fail closed.
- Prove an exact already-present mapping is idempotent.
- Prove stale-map detection aborts before mutation.
- Prove restore refuses to overwrite edited owned entries.

### Live Configuration Checks

- Read back `LanguageAgnosticKeyBindings = true` through iTerm2.
- Read back both global map entries with action `11` and texts `0x03` and
  `0x07`.
- Compare every pre-existing mapping with the backup and prove equality.
- Confirm all profile maps remain unchanged.
- Exercise each pre-existing global mapping that carries a virtual keycode, so
  enabling physical-key lookup does not introduce a regression.

### Physical-Key Acceptance

With Apple's 2-Set Korean input source selected:

1. Run a raw terminal probe with signal handling temporarily disabled and prove
   physical `Control-C` yields exactly byte `03` and physical `Control-G` yields
   exactly byte `07`.
2. Restore terminal attributes even if the probe is interrupted or fails.
3. In a normal iTerm2 shell, prove physical `Control-C` interrupts a foreground
   command and returns control to the prompt.
4. Prove physical `Control-G` reaches a foreground program that reports the
   byte or performs its expected abort/bell behavior.
5. Type Korean text afterward and confirm Korean remains selected and usable.
6. Repeat the byte probe in each currently used iTerm2 profile or prove that the
   profiles inherit the same global mapping without an override.

Synthetic events, preference read-back, and PTY defaults are useful diagnostics
but do not replace the physical-key proof. If the environment cannot observe a
real user keypress, implementation stops at a clearly labeled interaction gate
instead of claiming runtime success.

## Scope Expansion Gate

Ghostty, Terminal.app, a reusable configuration CLI, or a system-wide event
remapper is considered only if this iTerm2-native design passes configuration
checks but fails physical-key acceptance, or if the user later requests broader
coverage. Any expansion requires a new conflict audit and design decision.

## Realtime Checkpoints

1. `docs(iterm2): design Korean control key mappings`
2. `docs(iterm2): plan Korean control key mappings`
3. `fix(iterm2): configure Korean control key mappings`

Each green checkpoint is pushed immediately. Published commits are never
rewritten; follow-up corrections use new commits. Final completion requires
local, tracking, and live-remote parity of `0 0`.
