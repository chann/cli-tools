# macOS Korean Input and Terminal Control Keys Research

Status: implemented; target chords physically verified in iTerm2

## Executive Summary

The first implementation should not switch the macOS input source and should
not install a global keyboard event daemon. It should make the terminal send
the intended control byte directly when a physical control chord is pressed.

For the current machine, start with two global iTerm2 key bindings:

- physical `Control-C` sends `0x03`;
- physical `Control-G` sends `0x07`; and
- iTerm2 interprets those bindings by physical key, ignoring input language.

This preserves Korean as the selected input source, avoids Accessibility or
Input Monitoring permission, and fixes the event at the narrowest layer that
can still produce the missing PTY byte. Ghostty and Terminal.app have analogous
terminal-native mapping mechanisms and can be added only if they are needed.

A CLI can be valuable later as a reversible configuration manager and byte
probe, but a CLI process alone cannot intercept keys after it exits. Runtime
interception would require a daemon, event tap, or virtual keyboard and brings
permissions, Secure Input limitations, event-loop failure modes, and more
invasive system-wide behavior.

## Current-Machine Evidence

The following state was inspected without changing keyboard or terminal
preferences:

- macOS `26.5.2` on Apple silicon;
- active input source:
  `com.apple.inputmethod.Korean.2SetKorean`;
- enabled typing sources: ABC and Apple's 2-Set Korean input method;
- current process terminal environment: iTerm2 `3.6.11`;
- also installed: Ghostty `1.3.1` and Terminal.app `2.15`;
- Karabiner-Elements and Hammerspoon are not installed;
- a fresh local diagnostic PTY has `intr = ^C`, `icanon`, and `isig`; and
- neither iTerm2's global/profile key maps nor Ghostty's effective key map has
  an explicit `Control-C` or `Control-G` binding.

The diagnostic TTY evidence shows that the local default is prepared to turn
byte `0x03` into the interrupt action. The actual iTerm2 profile must be checked
again during the physical-key acceptance test, but nothing found points to a
shell-side remapping. The leading hypothesis is therefore that the failure
occurs before the byte reaches the TTY.

The current Ghostty configuration also uses `Control-A` as a key-sequence
prefix and `Control-N` for a new window. A blanket `Control-A` through
`Control-Z` replacement would silently destroy those user bindings, so any
solution must begin with only the requested keys and reject conflicts.

This investigation did not synthesize keyboard events or edit live terminal
settings. Physical-key runtime proof remains an explicit acceptance gate for
implementation.

## Where the Failure Occurs

The relevant path is:

```text
physical key
  -> macOS key event (key code + modifiers + interpreted characters)
  -> input method / terminal key binding
  -> terminal encoder
  -> PTY byte stream
  -> termios or foreground program
```

AppKit exposes a hardware-independent key value, raw virtual key code, and
modifier flags on an `NSEvent`. InputMethodKit likewise describes input events
as Unicode values, the generating key code, and modifiers. A terminal can
therefore recognize a physical chord before asking the active input method for
printable Korean text.

Once `0x03` reaches a canonical TTY with `VINTR` configured as `^C`, the terminal
driver generates the interrupt signal. `Control-G` is byte `0x07`; shells,
Readline-style editors, and terminal applications may interpret it as abort or
bell. If the terminal or input method never emits these bytes, `stty`, zsh
`bindkey`, tmux, or an application key map cannot reconstruct the missing
keystroke downstream.

Primary references:

- [Apple: `NSEvent.charactersIgnoringModifiers`](https://developer.apple.com/documentation/appkit/nsevent/charactersignoringmodifiers)
- [Apple: InputMethodKit key-event model](https://developer.apple.com/documentation/inputmethodkit/imkserverinput)
- [POSIX: `stty` control characters and `VINTR`](https://pubs.opengroup.org/onlinepubs/009695099/utilities/stty.html)
- [Apple Shell Scripting Primer: `Control-C` abort](https://developer.apple.com/library/archive/documentation/OpenSource/Conceptual/ShellScripting/CommandLInePrimer/CommandLine.html)
- [Apple Shell Scripting Primer: entering `Control-G`](https://developer.apple.com/library/archive/documentation/OpenSource/Conceptual/ShellScripting/BeforeYouBegin/BeforeYouBegin.html)

## Existing Terminal-Native Capabilities

### iTerm2

iTerm2 provides the exact primitives needed:

- global key bindings apply to every profile unless a profile overrides them;
- **Interpret key bindings based on physical key, ignoring input language**
  makes a binding follow the keyboard position rather than the active input
  language;
- bindings created by iTerm2 `3.5.0beta19` or later store the key code needed
  for this behavior; and
- **Send Hex Code** writes explicit bytes to the terminal session.

The current iTerm2 version is new enough, both configured profiles were
inspected, and neither profile overrides `Control-C` or `Control-G`. The minimal
configuration is therefore:

| Physical chord | iTerm2 action | PTY byte |
| --- | --- | --- |
| `Control-C` | Send Hex Code | `0x03` |
| `Control-G` | Send Hex Code | `0x07` |

These should be global bindings with physical-key interpretation enabled. They
should be created or imported with key-code metadata, not copied from an old
pre-3.5 binding.

iTerm2 can import/export global key bindings, and its Python API can read and
replace profile key maps. The UI or an import artifact is safer for an initial
two-key change than editing the live preferences plist directly.

Primary references:

- [iTerm2: global key bindings and physical-key interpretation](https://iterm2.com/documentation-preferences-keys.html)
- [iTerm2: profile key mappings and Send Hex Code](https://iterm2.com/documentation-preferences-profiles-keys.html)
- [iTerm2 Python API: profile key mappings](https://iterm2.com/python-api/profile.html)
- [iTerm2 maintainer issue: Send Hex Code `0x03` workaround](https://gitlab.com/gnachman/iterm2/-/issues/7508)

### Ghostty

Ghostty key-binding triggers support named keys derived from USB HID-related key
codes, and the `text` action sends an explicit string to the PTY. Its own
documentation uses `text:\x15` as the example for sending `Control-U`.

The corresponding candidate bindings are:

```ini
keybind = ctrl+c=text:\x03
keybind = ctrl+g=text:\x07
```

Ghostty `1.3.1` is installed and its effective key map has no conflicting
`Control-C` or `Control-G` entry. Its current `Control-A` sequences and
`Control-N` action must remain untouched.

Primary references:

- [Ghostty: custom key bindings](https://ghostty.org/docs/config/keybind)
- [Ghostty: `text` action reference](https://ghostty.org/docs/config/keybind/reference)
- [Ghostty: configuration locations and reload](https://ghostty.org/docs/config)

### Terminal.app

Terminal.app supports per-profile custom key combinations with an action and a
user-supplied string. Unlike iTerm2's global bindings, the settings apply only
to the selected profile, so every used profile must be audited. A raw control
character string is plausible, but the exact UI/export representation should
be captured from a known-good profile and runtime-tested before automating it.

Primary references:

- [Apple: Terminal profile keyboard settings](https://support.apple.com/guide/terminal/trmlkbrd/mac)
- [Apple: create custom function keys in Terminal](https://support.apple.com/guide/terminal/trml108/mac)

## Why Input-Source Switching Is Not the First Choice

Tools such as `im-select`, `macism`, and Hammerspoon can query and switch the
selected macOS input source. They solve editor-mode workflows that change to
ABC on leaving insert mode, but they do not make a one-shot CLI continue
observing future key events.

A `Control`-down switch to ABC followed by a `Control`-up restore is especially
unattractive on this machine:

- it changes visible global input state instead of emitting one PTY byte;
- it can disturb active Korean composition;
- restoration must survive missed key-up events, app switches, crashes, and
  sleep; and
- `macism` documents a CJK input-source race on macOS 26 and currently uses a
  conservative 150 ms workaround, with 100 ms being the lowest stable value in
  its reported macOS 26.4.1 tests.

That latency is too high for a modifier-down critical path and demonstrates why
input-source switching is not equivalent to terminal-native byte injection.

Primary references:

- [`im-select` project and usage](https://github.com/daipeihust/im-select)
- [`im-select` macOS implementation using Text Input Source Services](https://raw.githubusercontent.com/daipeihust/im-select/master/macOS/im-select/im-select/main.m)
- [`macism` and its macOS 26 CJK switching measurements](https://github.com/laishulu/macism)
- [Hammerspoon: query or set the current input source](https://www.hammerspoon.org/docs/hs.keycodes.html)

## Why a Global Event Tap Is a Fallback

Quartz Event Services can filter low-level input before foreground-app
delivery, so a custom daemon or Hammerspoon event tap could recognize physical
keys, restrict handling to terminal bundle identifiers, suppress the original
event, and post a replacement event.

This is materially more complex than terminal-native mappings:

- listening and posting event access must be authorized by macOS;
- synthetic-event feedback loops need an ownership marker;
- a slow callback can be disabled by the system and must be re-enabled;
- target-app and modifier state can change between key down and key up; and
- Secure Input prevents Hammerspoon from intercepting keyboard events.

Karabiner-Elements offers well-defined `frontmost_application_if` and
`input_source_if` conditions and posts modified events through a virtual
keyboard. It is a reasonable system-wide remapping platform, but it is not
installed here and it still emits keyboard events rather than terminal PTY
bytes. It should be considered only after a measured native-mapping failure or
if the requested scope expands beyond terminal applications.

Primary references:

- [Apple: Quartz Event Services](https://developer.apple.com/documentation/coregraphics/quartz-event-services)
- [Apple: `CGEventTapCreate`](https://developer.apple.com/documentation/coregraphics/cgevent/tapcreate(tap:place:options:eventsofinterest:callback:userinfo:))
- [Apple: event-tap timeout handling](https://developer.apple.com/documentation/coregraphics/cgeventtype/tapdisabledbytimeout)
- [Hammerspoon: event taps and Secure Input limitation](https://www.hammerspoon.org/docs/hs.eventtap.html)
- [Karabiner-Elements: input event modification chain](https://karabiner-elements.pqrs.org/docs/manual/misc/event-modification-chaining/)
- [Karabiner-Elements: frontmost-application condition](https://karabiner-elements.pqrs.org/docs/json/complex-modifications-manipulator-definition/conditions/frontmost-application/)
- [Karabiner-Elements: input-source condition](https://karabiner-elements.pqrs.org/docs/json/complex-modifications-manipulator-definition/conditions/input-source/)

## Options

### Option A: Terminal-Native Mappings

Configure iTerm2 first, then add Ghostty or Terminal.app mappings only for
terminals that are actually used.

Advantages:

- smallest behavioral scope;
- Korean remains the active input source;
- no daemon, virtual keyboard, or privacy permission;
- direct control over the PTY byte; and
- easy to test and remove.

Trade-offs:

- each terminal has a different configuration format;
- Terminal.app settings are per profile; and
- every new control chord needs an explicit conflict check.

Recommendation: choose this first.

### Option B: Reversible Configuration CLI

Build a macOS-specific CLI that manages Option A rather than intercepting
runtime input. Its useful surface would be:

```text
term-keys doctor
term-keys plan --keys c,g
term-keys apply --keys c,g --terminals detected
term-keys probe
term-keys restore
```

The CLI should:

- detect installed terminals, versions, active input source, and active
  profiles;
- inspect effective bindings and stop on any conflicting action;
- show a diff before applying;
- back up every touched file or exported preference object;
- write an ownership receipt containing only the entries it added;
- use terminal-supported APIs/import formats instead of blind plist editing;
- restore only owned entries and refuse to overwrite later user edits; and
- provide a raw-mode byte probe that always restores termios state.

Advantages:

- repeatable setup and audit across machines;
- safer conflict detection than hand-editing many profiles; and
- reusable runtime verification.

Trade-offs:

- more implementation and compatibility work than two native bindings;
- iTerm2 and Terminal.app adapters need different supported mutation paths;
  and
- a configuration CLI does not itself remain in the key-event path.

Recommendation: build this only if repeatability across multiple terminals or
machines is a real requirement after Option A succeeds.

### Option C: Scoped Runtime Remapper

Use Karabiner-Elements, Hammerspoon, or a custom `CGEventTap` LaunchAgent to
intercept terminal chords at runtime.

Advantages:

- one policy can cover several terminal applications; and
- it can eventually support non-terminal applications.

Trade-offs:

- Accessibility/Input Monitoring and possibly event-posting permission;
- Secure Input gaps;
- daemon lifecycle, crash recovery, and feedback-loop handling;
- higher chance of global keyboard regressions; and
- input-source switching latency if switching is used as the workaround.

Recommendation: reserve this for a proven native-terminal limitation or an
explicit macOS-wide scope.

## Proposed Delivery Sequence

1. Add only the two iTerm2 global physical-key mappings for `Control-C` and
   `Control-G`.
2. Run the acceptance matrix below while the selected input source remains
   Korean.
3. If iTerm2 passes, stop; do not build a daemon.
4. Add Ghostty's two explicit `text` bindings only if Ghostty is part of the
   desired daily workflow.
5. Capture and validate a Terminal.app profile export before automating its
   mapping.
6. Decide whether cross-machine repeatability justifies the Option B CLI.
7. Consider Option C only after recording a native-mapping failure that cannot
   be fixed with an explicit PTY byte binding.

## Acceptance Matrix

Implementation is not complete until all selected terminal/profile rows have
direct runtime evidence.

| Check | Expected result |
| --- | --- |
| Input source before test | `com.apple.inputmethod.Korean.2SetKorean` |
| `Control-C` in a raw-byte probe with signals disabled | exactly `03` |
| `Control-G` in the same probe | exactly `07` |
| `Control-C` during `sleep 30` | foreground process receives interrupt |
| Input source after test | still `com.apple.inputmethod.Korean.2SetKorean` |
| Korean text input after shortcuts | composition still works |
| Existing terminal shortcuts | unchanged |
| Existing Ghostty `Control-A` sequences and `Control-N` | unchanged |
| iTerm2 Default and tmux profiles | both inherit the global bindings |
| Remove or restore | original configuration and behavior return |

The byte probe must save the current termios state, temporarily disable
canonical input, echo, and signal generation, restore state on every normal or
signal exit path, and display byte values without interpreting them. Source
inspection and unit tests do not substitute for the physical Korean-input
keypress test.

## Implementation Evidence (2026-08-06)

The selected iTerm2-only configuration is active on iTerm2 `3.6.11`. The
implementation audited the global map and both configured profiles without
adding profile-local mappings:

- `Default [D24B5D35-BE7E-4C85-8072-439C74BCE0DA]`;
- `tmux [DF566FE8-2D85-46C9-9184-5EC59ABAF16F]`;
- before-map hash
  `a84de76c228e1613145f69d51efc0530a73e9412e8e16046295c97f19e1af106`;
- after-map hash
  `6e6d37ddcf18a30bfc9884ba783ea6482b39e511efd6fb123b33b63cccdfd998`;
- `0x63-0x40000-0x8 -> {"Action": 11, "Text": "0x03"}`; and
- `0x67-0x40000-0x5 -> {"Action": 11, "Text": "0x07"}`.

The first apply produced a private receipt, live read-back passed, and an exact
restore drill returned the map to the before hash. The originally absent
`LanguageAgnosticKeyBindings` key was proven absent again through a fresh
preference-domain export. A second apply then produced the active receipt:

```text
~/Library/Application Support/cli-tools/iterm2-korean-control-keys/
  20260806T015400.204770Z/receipt.json
```

Its directory and receipt modes are `0700` and `0600`. The receipt owns only
the two entries above and records that the physical-key preference was
originally absent.

iTerm2 `3.6.11` rejects the Python client's documented `None` unset request as
`INVALID_VALUE`. After explicit user approval, the exact-absence path was
limited to one key: set `LanguageAgnosticKeyBindings` to `false` through iTerm2,
delete only that persisted key with `/usr/bin/defaults`, export the domain
again, and fail unless the key is absent. Tests reject every other deletion
key. Normal map and preference writes remain on iTerm2's API.

With Apple's 2-Set Korean input source active, the user ran the raw probe and
reported:

```text
PASS: 03 07
```

The user also confirmed normal Korean composition afterward. Combined with the
existing `VINTR = ^C` and `ISIG` evidence, the physical `Control-C` byte reaches
the terminal interrupt path and physical `Control-G` reaches the PTY as `0x07`.
The pre-existing global mapping objects remained canonically identical, and
both profiles were proven to inherit the global C/G mappings without a local
override. A separate physical replay of the pre-existing
`Control-Command-=` and `Shift-Return` shortcuts was not reported, so their
runtime behavior is supported by structural preservation rather than a second
manual spot check.

## Decision

Approve Option A if the desired scope is terminal applications and preserving
Korean as the active input source. After approval, write the implementation
design and plan for the smallest iTerm2-first change, including backup,
conflict detection, physical-key runtime proof, and an explicit decision on
whether Ghostty and Terminal.app belong in the first implementation.

Choose Option B only if the configuration must be reproducible across multiple
terminals or machines. Choose Option C only if the requirement is macOS-wide or
Option A fails under direct runtime testing.
