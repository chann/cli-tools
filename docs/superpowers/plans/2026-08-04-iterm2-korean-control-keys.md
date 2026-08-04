# iTerm2 Korean Control Keys Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Configure iTerm2 `3.6.11` so physical `Control-C` and `Control-G`
emit `0x03` and `0x07` while Apple's 2-Set Korean input source remains active,
with conflict-blocking apply, private backup, conservative restore, and physical
runtime proof.

**Architecture:** Add a narrow Python operator script with pure key-map planning
and receipt logic separated from an iTerm2 Python API adapter. Add a standalone
raw-byte probe and a tiny Swift input-source reader for physical acceptance. The
operator script is run from the repository with `uv`, is never installed on
`PATH`, never edits the live preferences plist directly, and leaves no daemon in
the input path.

**Tech Stack:** Python 3.11+ standard library, `iterm2==2.20`, `uv 0.8.22`,
Python `unittest`, macOS Carbon/HIToolbox through Swift, iTerm2 `3.6.11` Python
API and preference RPC.

## Global Constraints

- Work sequentially in the main thread because the repository `AGENTS.md`
  disables subagent dispatch; use `superpowers:executing-plans` at execution.
- Support only iTerm2 `3.6.11`, physical `Control-C`, and physical `Control-G`.
- Add global mappings `0x63-0x40000-0x8 -> {"Action": 11, "Text": "0x03"}`
  and `0x67-0x40000-0x5 -> {"Action": 11, "Text": "0x07"}`.
- Enable `LanguageAgnosticKeyBindings`; do not switch the macOS input source.
- Do not change profile maps, shell settings, tmux settings, or other terminal
  applications.
- Do not install a daemon, LaunchAgent, event tap, virtual keyboard,
  Karabiner-Elements, Hammerspoon, or a generalized key-management CLI.
- Preserve every unknown field in every pre-existing global map entry.
- Refuse incompatible global, profile, legacy, modified-character, or portable
  physical-key mappings before mutation.
- Store backups only beneath
  `~/Library/Application Support/cli-tools/iterm2-korean-control-keys/` with
  directory mode `0700` and file mode `0600`.
- Use iTerm2's preference API for live mutation; never write its plist directly
  and never call `async_set_global_key_bindings()`.
- A denied or unavailable Python API is a hard stop, not permission to use a
  less safe mutation path.
- Physical user keypresses are required for final byte and interrupt proof;
  synthetic events are not acceptance evidence.
- Stage only task-owned paths, push every green commit immediately, never
  rewrite a published commit, and finish with local/tracking/live-remote
  divergence `0 0`.

---

## File Structure

- Create `scripts/__init__.py`: mark the operator utilities as an importable
  package for standard-library tests; it exports no public command.
- Create `tests/__init__.py`: make the new top-level test directory importable
  for focused `python3 -m unittest tests.<module>` commands.
- Create `scripts/iterm2_korean_control_keys.py`: own target constants,
  serialized-key parsing, conflict detection, canonical hashes, backup receipt,
  iTerm2 preference adapter, apply/read-back/rollback, restore, and the narrow
  `preflight|apply|verify|restore` command surface.
- Create `scripts/probe_control_bytes.py`: put the active terminal into raw mode,
  read `Control-C` then `Control-G`, and restore termios in `finally`.
- Create `scripts/current_macos_input_source.swift`: print the current Text Input
  Source Services identifier without changing it.
- Create `tests/fixtures/iterm2-global-key-map.json`: sanitized representative
  global map with the current unknown action fields retained.
- Create `tests/iterm2_korean_control_keys_test.py`: test pure planning,
  preservation, backup permissions, apply rollback, idempotency, and restore
  conflict behavior with a fake preference client.
- Create `tests/probe_control_bytes_test.py`: prove exact raw bytes and termios
  restoration on a pseudo-terminal.
- Modify `docs/research/2026-08-04-macos-korean-terminal-control-keys.md`: record
  the approved decision, applied preference hashes, backup/receipt location,
  restore drill, physical acceptance, and final result without embedding
  preference contents.
- Modify `docs/superpowers/specs/2026-08-04-iterm2-korean-control-keys-design.md`:
  retain `Status: Approved` and record no post-approval contract drift.

## Reference Contracts

- iTerm2 command-line scripts require the Python API server and may present an
  authorization prompt; do not bypass or weaken that authentication:
  <https://iterm2.com/python-api/tutorial/running.html>
- The pinned current client is `iterm2==2.20`:
  <https://pypi.org/project/iterm2/>
- The mapping serialization and physical-key lookup were validated against the
  local clone of iTerm2 tag `v3.6.11`, especially `iTermKeystroke.m`,
  `iTermKeyMappings.m`, `iTermPreferences.m`, and Python `binding.py`.

---

### Task 1: Pure Key-Map Planner and Collision Audit

**Files:**

- Create: `scripts/__init__.py`
- Create: `tests/__init__.py`
- Create: `scripts/iterm2_korean_control_keys.py`
- Create: `tests/fixtures/iterm2-global-key-map.json`
- Create: `tests/iterm2_korean_control_keys_test.py`

**Interfaces:**

- Produces: `TargetBinding`, `ParsedKey`, `KeymapPlan`, `MigrationError`,
  `canonical_json(value) -> bytes`, `canonical_hash(value) -> str`,
  `parse_serialized_key(key) -> ParsedKey`, and
  `plan_keymap(global_map, profile_maps) -> KeymapPlan`.
- Consumes: raw dictionaries shaped like iTerm2's `GlobalKeyMap` and profile
  `Keyboard Map` values.

- [ ] **Step 1: Create the fixture and write failing preservation tests**

  Put a representative current entry with fields the migration does not own in
  `tests/fixtures/iterm2-global-key-map.json`:

  ```json
  {
    "0x3d-0x140000-0x18": {
      "Action": 25,
      "Apply Mode": 0,
      "Escaping": 2,
      "Text": "Arrange Split Panes Evenly\\nArrange Split Panes Evenly",
      "Version": 2
    },
    "0xd-0x20000-0x24": {
      "Action": 12,
      "Keycode": 13,
      "Modifiers": 131072,
      "Text": "\\\\n",
      "Version": 1
    }
  }
  ```

  In `tests/iterm2_korean_control_keys_test.py`, load the fixture and assert:

  ```python
  def test_plan_adds_only_c_and_g_and_preserves_unknown_fields(self):
      before = load_fixture()
      plan = module.plan_keymap(before, {"Default": {}, "tmux": {}})
      self.assertEqual(set(plan.owned_additions), {
          "0x63-0x40000-0x8",
          "0x67-0x40000-0x5",
      })
      for key, value in before.items():
          self.assertEqual(plan.after[key], value)
      self.assertEqual(plan.after["0x63-0x40000-0x8"],
                       {"Action": 11, "Text": "0x03"})
      self.assertEqual(plan.after["0x67-0x40000-0x5"],
                       {"Action": 11, "Text": "0x07"})
  ```

- [ ] **Step 2: Add failing collision and idempotency tests**

  Add separate tests for these exact cases:

  ```python
  CASES = {
      "portable": "0x314a-0x40000-0x8",
      "legacy_unmodified": "0x63-0x40000",
      "legacy_control": "0x3-0x40000",
      "modified": ":0x3:0x40000",
  }

  def test_incompatible_global_forms_fail_closed(self):
      for label, key in CASES.items():
          with self.subTest(label=label):
              with self.assertRaises(module.MigrationError):
                  module.plan_keymap(
                      {key: {"Action": 12, "Text": "wrong"}}, {})

  def test_incompatible_profile_mapping_names_the_profile(self):
      with self.assertRaisesRegex(module.MigrationError, "tmux"):
          module.plan_keymap({}, {
              "tmux": {"0x67-0x40000-0x5":
                       {"Action": 12, "Text": "wrong"}}
          })

  def test_equivalent_portable_binding_is_idempotent(self):
      existing = {
          "0x314a-0x40000-0x8": {"Action": 11, "Text": "0x03"},
          "0x67-0x40000-0x5": {"Action": 11, "Text": "0x07"},
      }
      plan = module.plan_keymap(existing, {})
      self.assertEqual(plan.after, existing)
      self.assertEqual(plan.owned_additions, {})
  ```

- [ ] **Step 3: Run the focused test file and confirm RED**

  Run:

  ```bash
  python3 -m unittest tests.iterm2_korean_control_keys_test -v
  ```

  Expected: import or attribute failures because the planner is not implemented.

- [ ] **Step 4: Implement exact target models and key parsing**

  Add these immutable contracts to `scripts/iterm2_korean_control_keys.py`:

  ```python
  from __future__ import annotations

  import copy
  import dataclasses
  import hashlib
  import json
  from collections.abc import Mapping
  from typing import Any

  CONTROL = 0x40000
  HEX_CODE_ACTION = 11

  @dataclasses.dataclass(frozen=True)
  class TargetBinding:
      name: str
      keycode: int
      unmodified: int
      control_character: int
      text: str

      @property
      def serialized(self) -> str:
          return f"0x{self.unmodified:x}-0x{CONTROL:x}-0x{self.keycode:x}"

      @property
      def action(self) -> dict[str, Any]:
          return {"Action": HEX_CODE_ACTION, "Text": self.text}

  TARGETS = (
      TargetBinding("Control-C", 0x08, ord("c"), 0x03, "0x03"),
      TargetBinding("Control-G", 0x05, ord("g"), 0x07, "0x07"),
  )

  @dataclasses.dataclass(frozen=True)
  class ParsedKey:
      kind: str
      character: int
      modifiers: int
      keycode: int | None

  @dataclasses.dataclass(frozen=True)
  class KeymapPlan:
      before: dict[str, Any]
      after: dict[str, Any]
      owned_additions: dict[str, Any]
      already_satisfied: tuple[str, ...]

  class MigrationError(RuntimeError):
      pass

  def canonical_json(value: Any) -> bytes:
      return json.dumps(
          value, sort_keys=True, separators=(",", ":"), ensure_ascii=False
      ).encode("utf-8")

  def canonical_hash(value: Any) -> str:
      return hashlib.sha256(canonical_json(value)).hexdigest()
  ```

  Parse only the three iTerm2 forms validated in the design:

  ```python
  def parse_serialized_key(key: str) -> ParsedKey:
      if key.startswith(":"):
          pieces = key.split(":")
          if len(pieces) == 3 and pieces[0] == "":
              return ParsedKey("modified", int(pieces[1], 16),
                               int(pieces[2], 16), None)
      pieces = key.split("-")
      if len(pieces) == 2:
          return ParsedKey("legacy", int(pieces[0], 16),
                           int(pieces[1], 16), None)
      if len(pieces) == 3:
          return ParsedKey("portable", int(pieces[0], 16),
                           int(pieces[1], 16), int(pieces[2], 16))
      raise MigrationError(f"Unsupported iTerm2 key serialization: {key}")
  ```

- [ ] **Step 5: Implement fail-closed planning**

  Add helpers that associate a parsed key with C or G by physical keycode for
  portable entries, and by both unmodified/control characters for legacy and
  modified entries. `plan_keymap()` must deep-copy the input, audit global and
  profile maps before adding anything, treat `Action == 11` plus exact `Text`
  as compatible, and avoid a duplicate if an equivalent portable mapping
  already exists. Reject a target-related action that is not a dictionary with
  `MigrationError` instead of allowing an attribute error:

  ```python
  def action_matches(value: Mapping[str, Any], target: TargetBinding) -> bool:
      return (value.get("Action") == HEX_CODE_ACTION and
              value.get("Text") == target.text)

  def target_for(parsed: ParsedKey) -> TargetBinding | None:
      if parsed.modifiers != CONTROL:
          return None
      for target in TARGETS:
          if parsed.kind == "portable" and parsed.keycode == target.keycode:
              return target
          if parsed.kind == "legacy" and parsed.character in {
              target.unmodified, target.control_character
          }:
              return target
          if (parsed.kind == "modified" and
                  parsed.character == target.control_character):
              return target
      return None

  def plan_keymap(global_map: Mapping[str, Any],
                  profile_maps: Mapping[str, Mapping[str, Any]]) -> KeymapPlan:
      before = copy.deepcopy(dict(global_map))
      portable_satisfied: set[str] = set()
      for scope, mapping in [("global", global_map), *profile_maps.items()]:
          for serialized, action in mapping.items():
              target = target_for(parse_serialized_key(serialized))
              if target is None:
                  continue
              if not action_matches(action, target):
                  raise MigrationError(
                      f"Conflict in {scope}: {serialized} -> {action!r}")
              if scope == "global" and "-" in serialized and \
                      len(serialized.split("-")) == 3:
                  portable_satisfied.add(target.name)

      after = copy.deepcopy(before)
      owned: dict[str, Any] = {}
      for target in TARGETS:
          if target.name in portable_satisfied:
              continue
          after[target.serialized] = target.action
          owned[target.serialized] = target.action
      return KeymapPlan(before, after, owned, tuple(sorted(portable_satisfied)))
  ```

  Wrap `ValueError` from malformed hexadecimal components in `MigrationError`
  so the command prints one domain-specific error and exits before mutation.

- [ ] **Step 6: Run tests to GREEN and validate formatting**

  Run:

  ```bash
  python3 -m unittest tests.iterm2_korean_control_keys_test -v
  python3 -m compileall -q scripts tests
  git diff --check
  ```

  Expected: all planner tests pass; compile and whitespace checks are silent.

- [ ] **Step 7: Commit and push the green planner checkpoint**

  ```bash
  git add scripts/__init__.py scripts/iterm2_korean_control_keys.py \
    tests/__init__.py \
    tests/fixtures/iterm2-global-key-map.json \
    tests/iterm2_korean_control_keys_test.py
  git commit -m "feat(iterm2): add safe Korean keymap planner"
  git push origin main
  git fetch origin main
  git rev-list --left-right --count HEAD...refs/remotes/origin/main
  ```

  Expected divergence: `0 0`.

---

### Task 2: Private Backup, Live API Apply, and Conservative Restore

**Files:**

- Modify: `scripts/iterm2_korean_control_keys.py`
- Modify: `tests/iterm2_korean_control_keys_test.py`

**Interfaces:**

- Consumes: `KeymapPlan`, `canonical_hash()`, and `plan_keymap()` from Task 1.
- Produces: `PreferenceClient`, `PreferenceSnapshot`, `Receipt`,
  `create_backup(snapshot, plan, root, preference_export) -> Path`,
  `apply_configuration(client, snapshot, plan) -> None`,
  `restore_configuration(client, receipt) -> RestoreResult`, and commands
  `preflight|apply|verify|restore`.

- [ ] **Step 1: Write failing backup and permissions tests**

  Use `tempfile.TemporaryDirectory()` and assert:

  ```python
  def test_backup_is_private_and_records_exact_hashes(self):
      with tempfile.TemporaryDirectory() as directory:
          snapshot = sample_snapshot()
          plan = module.plan_keymap(snapshot.global_map, snapshot.profile_maps)
          receipt_path = module.create_backup(
              snapshot, plan, pathlib.Path(directory),
              preference_export=b"bplist00fixture")
          self.assertEqual(stat.S_IMODE(receipt_path.parent.stat().st_mode), 0o700)
          for path in receipt_path.parent.iterdir():
              self.assertEqual(stat.S_IMODE(path.stat().st_mode), 0o600)
          receipt = json.loads(receipt_path.read_text())
          self.assertEqual(receipt["before_hash"],
                           module.canonical_hash(snapshot.global_map))
          self.assertEqual(receipt["after_hash"],
                           module.canonical_hash(plan.after))
  ```

  Include separate assertions for an originally absent
  `LanguageAgnosticKeyBindings` key and for an explicitly persisted `false`.

- [ ] **Step 2: Write failing compensation, stale-write, and restore tests**

  Implement a test-only `FakePreferenceClient` that records writes and can fail
  on a selected call. Add these exact behaviors:

  ```python
  async def test_second_write_failure_restores_map_and_absent_flag(self):
      client = FakePreferenceClient(fail_on_write=2)
      snapshot = await client.snapshot()
      plan = module.plan_keymap(snapshot.global_map, snapshot.profile_maps)
      with self.assertRaises(module.MigrationError):
          await module.apply_configuration(client, snapshot, plan)
      self.assertEqual(client.global_map, snapshot.global_map)
      self.assertNotIn("LanguageAgnosticKeyBindings", client.persisted)

  async def test_stale_map_aborts_before_first_write(self):
      client = FakePreferenceClient()
      snapshot = await client.snapshot()
      plan = module.plan_keymap(snapshot.global_map, snapshot.profile_maps)
      client.global_map["0x61-0x100000"] = {"Action": 0, "Text": ""}
      with self.assertRaisesRegex(module.MigrationError, "changed after preflight"):
          await module.apply_configuration(client, snapshot, plan)
      self.assertEqual(client.writes, [])

  async def test_restore_refuses_edited_owned_entry(self):
      client, receipt = configured_client_and_receipt()
      client.global_map["0x63-0x40000-0x8"]["Text"] = "0x04"
      with self.assertRaisesRegex(module.MigrationError, "owned entry changed"):
          await module.restore_configuration(client, receipt)
  ```

  Also test that unrelated post-apply mappings allow owned-entry removal but
  keep `LanguageAgnosticKeyBindings = true` and return a warning result. Add a
  path-safety test that rejects receipts outside the resolved backup root and
  rejects a receipt symlink even when its link name is inside that root.

- [ ] **Step 3: Run the expanded tests and confirm RED**

  ```bash
  python3 -m unittest tests.iterm2_korean_control_keys_test -v
  ```

  Expected: failures for the missing snapshot, backup, apply, and restore APIs.

- [ ] **Step 4: Implement snapshots, receipts, and atomic private files**

  Add immutable data classes with exact fields:

  ```python
  @dataclasses.dataclass(frozen=True)
  class PreferenceSnapshot:
      iterm_version: str
      global_map: dict[str, Any]
      language_agnostic_effective: bool
      language_agnostic_persisted: bool
      language_agnostic_persisted_value: bool | None
      profile_maps: dict[str, dict[str, Any]]

  @dataclasses.dataclass(frozen=True)
  class Receipt:
      schema_version: int
      iterm_version: str
      created_at: str
      before_hash: str
      after_hash: str
      original_language_agnostic_persisted: bool
      original_language_agnostic_value: bool | None
      owned_entries: dict[str, Any]

  @dataclasses.dataclass(frozen=True)
  class RestoreResult:
      map_hash: str
      language_agnostic_restored: bool
      warning: str | None
  ```

  `create_backup()` creates a UTC `YYYYmmddTHHMMSS.ffffffZ` child, writes canonical
  `global-key-map.before.json`, `receipt.json`, and the read-only
  `preferences.plist` export through a same-directory temporary file, `fsync`,
  `os.replace`, and explicit `chmod`. Reject a pre-existing child path rather
  than reusing it. Six-digit UTC microseconds ensure a restore drill followed
  by immediate reapply cannot collide within one second.

- [ ] **Step 5: Implement the iTerm2 preference client without top-level dependency import**

  Add PEP 723 metadata for execution through `uv`, but keep `import iterm2`
  inside `main()` so standard-library tests do not download dependencies:

  ```python
  # /// script
  # requires-python = ">=3.11"
  # dependencies = ["iterm2==2.20"]
  # ///
  ```

  The live client must use raw RPC JSON for global preference reads and the
  generic setter for writes:

  ```python
  class ItermPreferenceClient:
      def __init__(self, iterm2_module: Any, connection: Any):
          self.iterm2 = iterm2_module
          self.connection = connection

      async def get_preference(self, key: str) -> Any:
          response = await self.iterm2.rpc.async_get_preference(
              self.connection, key)
          raw = (response.preferences_response.results[0]
                 .get_preference_result.json_value)
          return json.loads(raw)

      async def set_preference(self, key: str, value: Any) -> None:
          await self.iterm2.async_set_preference(
              self.connection, key, value)

      async def profile_maps(self) -> dict[str, dict[str, Any]]:
          profiles = await self.iterm2.PartialProfile.async_query(
              self.connection,
              properties=["Guid", "Name", "Keyboard Map"])
          return {f"{profile.name} [{profile.guid}]":
                  copy.deepcopy(profile.key_mappings or {})
                  for profile in profiles}
  ```

  Read app version from
  `/Applications/iTerm.app/Contents/Info.plist` with `/usr/libexec/PlistBuddy`
  and require the exact string `3.6.11`. Read `LoadPrefsFromCustomFolder`
  through iTerm2 and abort if true. Export the standard persistent domain with:

  ```bash
  /usr/bin/defaults export com.googlecode.iterm2 <private-backup>/preferences.plist
  ```

  Parse that export with `plistlib` to distinguish an absent physical-key
  preference from an explicitly persisted `false`.

  Put asynchronous fake-client tests on
  `unittest.IsolatedAsyncioTestCase`. Configure the fake's injected write
  failure to fire once, so compensating writes can succeed and their final
  state can be asserted.

- [ ] **Step 6: Implement apply/read-back/compensating rollback**

  Define this protocol so the same orchestration accepts fake and live clients:

  ```python
  from typing import Protocol

  class PreferenceClient(Protocol):
      async def get_preference(self, key: str) -> Any: ...
      async def set_preference(self, key: str, value: Any) -> None: ...
      async def profile_maps(self) -> dict[str, dict[str, Any]]: ...
  ```

  `apply_configuration()` must:

  1. re-read `GlobalKeyMap` and compare its canonical hash with
     `snapshot.global_map`;
  2. set the raw copied map;
  3. set `LanguageAgnosticKeyBindings` to `True`;
  4. read both values back and compare exact canonical values; and
  5. on any failure, restore the original map and set the physical-key
     preference to its prior value or `None` when it was absent, then verify
     restoration before raising `MigrationError`.

  Keep the two writes inside `iterm2.Transaction(connection)` in the live
  command to prevent another API operation from interleaving. Do not describe
  this as atomic: the compensating restore remains mandatory.

- [ ] **Step 7: Implement conservative restore**

  `restore_configuration()` loads the receipt and current map, verifies every
  receipt-owned key still has its exact receipt-owned value, removes only those
  keys, and writes the reduced map. Restore the physical-key preference to its
  original value/absence only when the reduced map hash equals `before_hash`;
  otherwise keep it true and return the warning:

  ```text
  Removed owned Control-C/Control-G entries; kept
  LanguageAgnosticKeyBindings enabled because unrelated mappings changed.
  ```

  Read back and verify the reduced map in both branches.

- [ ] **Step 8: Add the narrow command surface**

  Parse only:

  ```text
  preflight
  apply
  verify
  restore --receipt /absolute/path/to/receipt.json
  ```

  `preflight` and `verify` never write. `apply` prints the planned two-entry
  diff, private receipt path, before/after hashes, profile names, and final
  read-back result, but never prints the full preferences export. Before its
  first preference write, `apply` must successfully create and close the full
  private backup and receipt. `restore` rejects a receipt outside the approved
  backup root, any receipt symlink, or a schema version other than `1`.

- [ ] **Step 9: Run all migration tests to GREEN**

  ```bash
  python3 -m unittest tests.iterm2_korean_control_keys_test -v
  python3 -m compileall -q scripts tests
  uv run --with iterm2==2.20 python -c \
    'import iterm2; print(iterm2.__version__)'
  git diff --check
  ```

  Expected: tests pass; the dependency smoke test prints `2.20`; no live
  preference has been written.

- [ ] **Step 10: Commit and push the green reversible-migration checkpoint**

  ```bash
  git add scripts/iterm2_korean_control_keys.py \
    tests/iterm2_korean_control_keys_test.py
  git commit -m "feat(iterm2): add reversible Korean key migration"
  git push origin main
  git fetch origin main
  git rev-list --left-right --count HEAD...refs/remotes/origin/main
  ```

  Expected divergence: `0 0`.

---

### Task 3: Physical Byte Probe and Input-Source Observation

**Files:**

- Create: `scripts/probe_control_bytes.py`
- Create: `scripts/current_macos_input_source.swift`
- Create: `tests/probe_control_bytes_test.py`

**Interfaces:**

- Produces: `raw_terminal(fd)` context manager,
  `read_expected(fd, expected) -> list[int]`, executable raw probe, and a Swift
  command that prints exactly one input-source identifier line.
- Consumes: a real terminal file descriptor for acceptance or a pseudo-terminal
  slave in tests.

- [ ] **Step 1: Write failing pseudo-terminal tests**

  ```python
  class ProbeTests(unittest.TestCase):
      def test_reads_control_c_then_control_g_as_bytes(self):
          master, slave = pty.openpty()
          try:
              before = termios.tcgetattr(slave)
              with module.raw_terminal(slave):
                  os.write(master, b"\x03\x07")
                  self.assertEqual(
                      module.read_expected(slave, (0x03, 0x07)),
                      [0x03, 0x07])
              self.assertEqual(termios.tcgetattr(slave), before)
          finally:
              os.close(master)
              os.close(slave)

      def test_wrong_byte_fails_and_still_restores_termios(self):
          master, slave = pty.openpty()
          before = termios.tcgetattr(slave)
          try:
              with self.assertRaises(module.ProbeError):
                  with module.raw_terminal(slave):
                      os.write(master, b"x")
                      module.read_expected(slave, (0x03, 0x07))
          finally:
              self.assertEqual(termios.tcgetattr(slave), before)
              os.close(master)
              os.close(slave)
  ```

- [ ] **Step 2: Run the probe tests and confirm RED**

  ```bash
  python3 -m unittest tests.probe_control_bytes_test -v
  ```

  Expected: import failure because the probe does not exist.

- [ ] **Step 3: Implement raw mode with unconditional restoration**

  ```python
  import contextlib
  import os
  import sys
  import termios
  import tty

  class ProbeError(RuntimeError):
      pass

  @contextlib.contextmanager
  def raw_terminal(fd: int):
      original = termios.tcgetattr(fd)
      try:
          tty.setraw(fd, termios.TCSANOW)
          yield
      finally:
          termios.tcsetattr(fd, termios.TCSANOW, original)

  def read_expected(fd: int, expected: tuple[int, ...]) -> list[int]:
      seen: list[int] = []
      for wanted in expected:
          value = os.read(fd, 1)
          if not value or value[0] != wanted:
              actual = "EOF" if not value else f"0x{value[0]:02x}"
              raise ProbeError(f"expected 0x{wanted:02x}, got {actual}")
          seen.append(value[0])
      return seen
  ```

  The executable path requires `sys.stdin.isatty()`, prints
  `Press physical Control-C, then physical Control-G`, runs the context manager,
  and prints `PASS: 03 07`. It never catches `BaseException` outside the context
  manager, so `finally` always restores terminal state.

- [ ] **Step 4: Implement the read-only Swift input-source reader**

  ```swift
  import Carbon
  import Foundation

  let source = TISCopyCurrentKeyboardInputSource().takeRetainedValue()
  guard let pointer = TISGetInputSourceProperty(
      source, kTISPropertyInputSourceID
  ) else {
      FileHandle.standardError.write(Data("missing input source id\n".utf8))
      exit(1)
  }
  let identifier = Unmanaged<CFString>
      .fromOpaque(pointer).takeUnretainedValue() as String
  print(identifier)
  ```

  This file has no setter and cannot change the input source.

- [ ] **Step 5: Run probe tests and read-only smoke checks to GREEN**

  ```bash
  python3 -m unittest tests.probe_control_bytes_test -v
  python3 -m compileall -q scripts tests
  swift scripts/current_macos_input_source.swift
  git diff --check
  ```

  Expected test result: PASS. Expected current source on this machine:
  `com.apple.inputmethod.Korean.2SetKorean`.

- [ ] **Step 6: Commit and push the green acceptance-tool checkpoint**

  ```bash
  git add scripts/probe_control_bytes.py \
    scripts/current_macos_input_source.swift \
    tests/probe_control_bytes_test.py
  git commit -m "test(iterm2): add physical control key probes"
  git push origin main
  git fetch origin main
  git rev-list --left-right --count HEAD...refs/remotes/origin/main
  ```

  Expected divergence: `0 0`.

---

### Task 4: Live Preflight, Restore Drill, Final Apply, and Evidence

**Files:**

- Modify: `docs/research/2026-08-04-macos-korean-terminal-control-keys.md`
- Verify: `docs/superpowers/specs/2026-08-04-iterm2-korean-control-keys-design.md`

**Interfaces:**

- Consumes: all scripts and tests from Tasks 1-3, running iTerm2 `3.6.11`, and
  a real physical keypress from the user at the explicit interaction gate.
- Produces: live iTerm2 global mappings, a final private receipt, exact restore
  proof, physical byte proof, updated research evidence, and Git parity.

- [ ] **Step 1: Re-run the complete non-mutating gate**

  ```bash
  python3 -m unittest discover -s tests -p '*iterm2*test.py' -v
  python3 -m unittest tests.probe_control_bytes_test -v
  python3 -m compileall -q scripts tests
  git diff --check
  swift scripts/current_macos_input_source.swift
  uv run scripts/iterm2_korean_control_keys.py preflight
  ```

  Expected: all tests pass; input source is
  `com.apple.inputmethod.Korean.2SetKorean`; iTerm2 is `3.6.11`; API is enabled;
  custom preference storage is off; C/G have no conflict; the diff contains
  only the two approved entries and the physical-key preference.

- [ ] **Step 2: Stop at any API authorization dialog until the user grants it**

  External iTerm2 scripts may show **Allow Python API Usage?**. The user must
  explicitly grant the one-time request. If denied, record the denial and stop;
  do not enable weaker authentication, run `defaults write`, or edit a plist.

- [ ] **Step 3: Apply once and capture the first private receipt path**

  ```bash
  uv run scripts/iterm2_korean_control_keys.py apply
  uv run scripts/iterm2_korean_control_keys.py verify
  ```

  Expected: read-back shows `LanguageAgnosticKeyBindings = true`, C action `11`
  with `0x03`, G action `11` with `0x07`, unchanged profile hashes, and a receipt
  beneath the approved private backup root.

- [ ] **Step 4: Prove exact restore, then reapply**

  ```bash
  uv run scripts/iterm2_korean_control_keys.py restore \
    --receipt '/absolute/path/printed-by-apply/receipt.json'
  uv run scripts/iterm2_korean_control_keys.py preflight
  uv run scripts/iterm2_korean_control_keys.py apply
  uv run scripts/iterm2_korean_control_keys.py verify
  ```

  Replace the quoted path only with the literal receipt path printed by Step 3;
  do not use a glob or unresolved environment variable. The post-restore hash
  must equal the original pre-change hash and the originally absent preference
  must again be absent. The second apply produces the final active receipt.

- [ ] **Step 5: Enter the physical-key interaction gate with Korean still active**

  In the user's actual iTerm2 session, run:

  ```bash
  swift scripts/current_macos_input_source.swift
  python3 scripts/probe_control_bytes.py
  ```

  The user presses physical `Control-C`, then physical `Control-G`. Expected:

  ```text
  com.apple.inputmethod.Korean.2SetKorean
  PASS: 03 07
  ```

  If the probe sees another byte or hangs, interrupt from a separate session,
  run the exact final-receipt restore command, and do not claim success.

- [ ] **Step 6: Prove normal interrupt and Korean typing**

  In the same physical iTerm2 session:

  ```bash
  python3 -c 'import time; time.sleep(30)'
  ```

  Press physical `Control-C`; expected output contains `KeyboardInterrupt` and
  returns to the prompt. Then type `한글 입력 확인`, press Return, and confirm
  normal Korean composition. Run the Swift reader again and require the same
  Korean identifier.

- [ ] **Step 7: Check inherited profiles and existing keycoded mappings**

  Run `verify` from the currently used Default and tmux iTerm2 profiles. Confirm
  neither profile gained a local mapping. In a disposable split, exercise the
  pre-existing `Control-Command-=` Arrange Split Panes Evenly shortcut and the
  configured `Shift-Return` mapping; both must retain their original action.

- [ ] **Step 8: Record source-accurate evidence without private preference data**

  Update the research document:

  - change status to `implemented and physically verified` only if Steps 1-7
    all pass;
  - record iTerm2 version, input-source identifier, before/after canonical map
    hashes, exact added keys and action/text values, profile names and unchanged
    hashes, restore-drill result, raw probe result `03 07`, normal interrupt
    result, and final receipt path;
  - state any API authorization or physical-interaction limitation separately;
  - do not paste the full preference export or receipt contents.

- [ ] **Step 9: Run the final verification gate**

  ```bash
  python3 -m unittest discover -s tests -p '*test.py' -v
  python3 -m compileall -q scripts tests
  uv run scripts/iterm2_korean_control_keys.py verify
  swift scripts/current_macos_input_source.swift
  git diff --check
  git status --short
  ```

  Expected: tests pass; live verification passes; input source remains Korean;
  only the intended research/spec evidence paths are modified.

- [ ] **Step 10: Commit and push the final verified configuration evidence**

  ```bash
  git add docs/research/2026-08-04-macos-korean-terminal-control-keys.md \
    docs/superpowers/specs/2026-08-04-iterm2-korean-control-keys-design.md
  git commit -m "fix(iterm2): configure Korean control key mappings"
  git push origin main
  git fetch origin main
  ```

  If the spec has no diff after its approval-status checkpoint, omit it from the
  explicit `git add` rather than creating an empty staged path.

- [ ] **Step 11: Prove local, tracking, and live-remote parity**

  ```bash
  git rev-parse HEAD
  git rev-parse refs/remotes/origin/main
  git rev-list --left-right --count HEAD...refs/remotes/origin/main
  git ls-remote --heads origin refs/heads/main
  git status --short --branch
  ```

  Expected: all three SHAs match, divergence is `0 0`, and the worktree is
  clean.
