# /// script
# requires-python = ">=3.11"
# dependencies = ["iterm2==2.20"]
# ///

from __future__ import annotations

import argparse
import copy
import dataclasses
import datetime as dt
import hashlib
import json
import os
import pathlib
import plistlib
import subprocess
import sys
from collections.abc import Mapping
from typing import Any, Protocol


CONTROL = 0x40000
HEX_CODE_ACTION = 11
GLOBAL_MAP_KEY = "GlobalKeyMap"
LANGUAGE_AGNOSTIC_KEY = "LanguageAgnosticKeyBindings"
RECEIPT_SCHEMA_VERSION = 1
BACKUP_ROOT = (
    pathlib.Path.home()
    / "Library"
    / "Application Support"
    / "cli-tools"
    / "iterm2-korean-control-keys"
)
EXPECTED_ITERM_VERSION = "3.6.11"
CUSTOM_PREFERENCES_KEY = "LoadPrefsFromCustomFolder"
BOOLEAN_PREFERENCE_KEYS = {
    LANGUAGE_AGNOSTIC_KEY,
    CUSTOM_PREFERENCES_KEY,
}


class MigrationError(RuntimeError):
    pass


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
    result: str


@dataclasses.dataclass(frozen=True)
class RestoreResult:
    map_hash: str
    language_agnostic_restored: bool
    warning: str | None


class PreferenceClient(Protocol):
    async def get_preference(self, key: str) -> Any: ...

    async def set_preference(self, key: str, value: Any) -> None: ...

    async def unset_preference(self, key: str) -> None: ...

    async def profile_maps(self) -> dict[str, dict[str, Any]]: ...


class ItermPreferenceClient:
    def __init__(
        self,
        iterm2_module: Any,
        connection: Any,
        *,
        delete_persisted: Any = None,
    ):
        self.iterm2 = iterm2_module
        self.connection = connection
        self._delete_persisted = delete_persisted

    async def get_preference(self, key: str) -> Any:
        response = await self.iterm2.rpc.async_get_preference(
            self.connection,
            key,
        )
        try:
            raw = (
                response.preferences_response.results[0]
                .get_preference_result.json_value
            )
            value = json.loads(raw)
        except (AttributeError, IndexError, TypeError, json.JSONDecodeError) as error:
            raise MigrationError(
                f"iTerm2 returned an invalid value for {key}"
            ) from error
        if key in BOOLEAN_PREFERENCE_KEYS:
            if value is None:
                return False
            if isinstance(value, bool):
                return value
            if type(value) is int and value in {0, 1}:
                return bool(value)
            raise MigrationError(
                f"iTerm2 returned a non-boolean value for {key}"
            )
        return value

    async def set_preference(self, key: str, value: Any) -> None:
        await self.iterm2.async_set_preference(
            self.connection,
            key,
            value,
        )

    async def unset_preference(self, key: str) -> None:
        await self.set_preference(key, False)
        delete = self._delete_persisted or delete_persisted_preference
        delete(key)
        if await self.get_preference(key) is not False:
            raise MigrationError("The unset preference did not read back as false")

    async def profile_maps(self) -> dict[str, dict[str, Any]]:
        profiles = await self.iterm2.PartialProfile.async_query(
            self.connection,
            properties=["Guid", "Name", "Keyboard Map"],
        )
        return {
            f"{profile.name} [{profile.guid}]": copy.deepcopy(
                profile.key_mappings or {}
            )
            for profile in profiles
        }


def canonical_json(value: Any) -> bytes:
    return json.dumps(
        value,
        sort_keys=True,
        separators=(",", ":"),
        ensure_ascii=False,
    ).encode("utf-8")


def canonical_hash(value: Any) -> str:
    return hashlib.sha256(canonical_json(value)).hexdigest()


def language_agnostic_persistence(
    preference_export: bytes,
) -> tuple[bool, bool | None]:
    try:
        preferences = plistlib.loads(preference_export)
    except (plistlib.InvalidFileException, ValueError) as error:
        raise MigrationError("Could not parse the iTerm2 preference export") from error
    if not isinstance(preferences, Mapping):
        raise MigrationError("The iTerm2 preference export is not a dictionary")
    if LANGUAGE_AGNOSTIC_KEY not in preferences:
        return False, None
    value = preferences[LANGUAGE_AGNOSTIC_KEY]
    if not isinstance(value, bool):
        raise MigrationError(
            f"{LANGUAGE_AGNOSTIC_KEY} is persisted with a non-boolean value"
        )
    return True, value


async def build_snapshot(
    client: PreferenceClient,
    iterm_version: str,
    preference_export: bytes,
) -> PreferenceSnapshot:
    if iterm_version != EXPECTED_ITERM_VERSION:
        raise MigrationError(
            f"Expected iTerm2 {EXPECTED_ITERM_VERSION}, found {iterm_version}"
        )
    if await client.get_preference(CUSTOM_PREFERENCES_KEY) is True:
        raise MigrationError("iTerm2 custom preference storage is enabled")

    global_map = await client.get_preference(GLOBAL_MAP_KEY)
    effective_flag = await client.get_preference(LANGUAGE_AGNOSTIC_KEY)
    profile_maps = await client.profile_maps()
    persisted, persisted_value = language_agnostic_persistence(
        preference_export
    )
    if effective_flag is None and not persisted:
        effective_flag = False
    try:
        exported_preferences = plistlib.loads(preference_export)
        exported_map = exported_preferences[GLOBAL_MAP_KEY]
    except (
        KeyError,
        TypeError,
        ValueError,
        plistlib.InvalidFileException,
    ) as error:
        raise MigrationError(
            "The iTerm2 export does not contain a valid global key map"
        ) from error
    if not isinstance(global_map, Mapping):
        raise MigrationError("The global iTerm2 key map is not a dictionary")
    if not isinstance(exported_map, Mapping):
        raise MigrationError(
            "The exported iTerm2 global key map is not a dictionary"
        )
    if canonical_hash(exported_map) != canonical_hash(global_map):
        raise MigrationError(
            "The iTerm2 preference export does not match the API key map"
        )
    if not isinstance(effective_flag, bool):
        raise MigrationError(
            f"{LANGUAGE_AGNOSTIC_KEY} did not resolve to a boolean"
        )
    if not isinstance(profile_maps, Mapping):
        raise MigrationError("The iTerm2 profile maps are not a dictionary")
    return PreferenceSnapshot(
        iterm_version=iterm_version,
        global_map=copy.deepcopy(dict(global_map)),
        language_agnostic_effective=effective_flag,
        language_agnostic_persisted=persisted,
        language_agnostic_persisted_value=persisted_value,
        profile_maps=copy.deepcopy(dict(profile_maps)),
    )


def absolute_path(value: str) -> pathlib.Path:
    path = pathlib.Path(value)
    if not path.is_absolute():
        raise MigrationError("restore requires an absolute receipt path")
    return path


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        description="Safely configure physical Control-C and Control-G in iTerm2"
    )
    subparsers = parser.add_subparsers(dest="command", required=True)
    subparsers.add_parser("preflight")
    subparsers.add_parser("apply")
    subparsers.add_parser("verify")
    restore_parser = subparsers.add_parser("restore")
    restore_parser.add_argument(
        "--receipt",
        required=True,
        type=absolute_path,
    )
    return parser


def read_iterm_version(*, run: Any = subprocess.run) -> str:
    command = [
        "/usr/libexec/PlistBuddy",
        "-c",
        "Print :CFBundleShortVersionString",
        "/Applications/iTerm.app/Contents/Info.plist",
    ]
    try:
        completed = run(
            command,
            check=True,
            capture_output=True,
            text=True,
        )
    except (OSError, subprocess.CalledProcessError) as error:
        raise MigrationError("Could not read the installed iTerm2 version") from error
    version = completed.stdout.strip()
    if not version:
        raise MigrationError("The installed iTerm2 version is empty")
    return version


def export_persistent_domain(*, run: Any = subprocess.run) -> bytes:
    try:
        completed = run(
            [
                "/usr/bin/defaults",
                "export",
                "com.googlecode.iterm2",
                "-",
            ],
            check=True,
            capture_output=True,
        )
    except (OSError, subprocess.CalledProcessError) as error:
        raise MigrationError("Could not export the iTerm2 preference domain") from error
    if not completed.stdout:
        raise MigrationError("The iTerm2 preference export is empty")
    return completed.stdout


def delete_persisted_preference(
    key: str,
    *,
    run: Any = subprocess.run,
    export: Any = export_persistent_domain,
) -> None:
    if key != LANGUAGE_AGNOSTIC_KEY:
        raise MigrationError("Refusing to delete an unapproved preference key")
    try:
        run(
            [
                "/usr/bin/defaults",
                "delete",
                "com.googlecode.iterm2",
                key,
            ],
            check=False,
            capture_output=True,
        )
    except OSError as error:
        raise MigrationError("Could not delete the persisted preference") from error
    persisted, _ = language_agnostic_persistence(export())
    if persisted:
        raise MigrationError("The persisted preference deletion did not verify")


def _print_plan(
    snapshot: PreferenceSnapshot,
    plan: KeymapPlan,
    output: Any,
) -> None:
    print(f"iTerm2 version: {snapshot.iterm_version}", file=output)
    print(
        "Profiles: " + ", ".join(sorted(snapshot.profile_maps)),
        file=output,
    )
    print(f"Before map hash: {canonical_hash(plan.before)}", file=output)
    print(f"After map hash:  {canonical_hash(plan.after)}", file=output)
    for serialized in sorted(plan.owned_additions):
        action = plan.owned_additions[serialized]
        print(
            f"Add {serialized}: Action={action['Action']} Text={action['Text']}",
            file=output,
        )
    print(
        f"Set {LANGUAGE_AGNOSTIC_KEY}=true",
        file=output,
    )


def _configuration_is_complete(
    snapshot: PreferenceSnapshot,
    plan: KeymapPlan,
) -> bool:
    return (
        not plan.owned_additions
        and snapshot.language_agnostic_effective is True
    )


async def run_command(
    args: argparse.Namespace,
    client: PreferenceClient,
    iterm2_module: Any,
    connection: Any,
    *,
    iterm_version: str,
    preference_export: bytes,
    backup_root: pathlib.Path = BACKUP_ROOT,
    output: Any = sys.stdout,
) -> None:
    snapshot = await build_snapshot(
        client,
        iterm_version,
        preference_export,
    )

    if args.command == "restore":
        receipt = load_receipt(args.receipt, backup_root)
        if receipt.iterm_version != snapshot.iterm_version:
            raise MigrationError(
                "The receipt was created for a different iTerm2 version"
            )
        async with iterm2_module.Transaction(connection):
            result = await restore_configuration(client, receipt)
        if result.warning:
            print(f"WARNING: {result.warning}", file=output)
        print(f"RESTORED map hash: {result.map_hash}", file=output)
        return

    plan = plan_keymap(snapshot.global_map, snapshot.profile_maps)
    if args.command == "preflight":
        _print_plan(snapshot, plan, output)
        print("PREFLIGHT OK: no preferences were changed", file=output)
        return

    if args.command == "verify":
        if not _configuration_is_complete(snapshot, plan):
            raise MigrationError(
                "The physical Control-C/Control-G configuration is incomplete"
            )
        print(
            f"VERIFIED map hash: {canonical_hash(snapshot.global_map)}",
            file=output,
        )
        print(f"VERIFIED {LANGUAGE_AGNOSTIC_KEY}=true", file=output)
        return

    if args.command != "apply":
        raise MigrationError(f"Unsupported command: {args.command}")

    _print_plan(snapshot, plan, output)
    receipt_path = create_backup(
        snapshot,
        plan,
        backup_root,
        preference_export=preference_export,
    )
    print(f"Private receipt: {receipt_path}", file=output)
    async with iterm2_module.Transaction(connection):
        await apply_configuration(client, snapshot, plan)

    final_map = await client.get_preference(GLOBAL_MAP_KEY)
    final_flag = await client.get_preference(LANGUAGE_AGNOSTIC_KEY)
    final_profiles = await client.profile_maps()
    if (
        canonical_hash(final_map) != canonical_hash(plan.after)
        or final_flag is not True
        or canonical_hash(final_profiles) != canonical_hash(snapshot.profile_maps)
    ):
        raise MigrationError("The final iTerm2 read-back did not verify")
    print(f"APPLIED map hash: {canonical_hash(final_map)}", file=output)
    print(f"APPLIED {LANGUAGE_AGNOSTIC_KEY}=true", file=output)


def _atomic_write_private(path: pathlib.Path, data: bytes) -> None:
    temporary = path.with_name(f".{path.name}.{os.getpid()}.tmp")
    descriptor: int | None = None
    try:
        descriptor = os.open(
            temporary,
            os.O_WRONLY | os.O_CREAT | os.O_EXCL,
            0o600,
        )
        with os.fdopen(descriptor, "wb") as stream:
            descriptor = None
            stream.write(data)
            stream.flush()
            os.fsync(stream.fileno())
        os.replace(temporary, path)
        os.chmod(path, 0o600)
    finally:
        if descriptor is not None:
            os.close(descriptor)
        try:
            temporary.unlink()
        except FileNotFoundError:
            pass


def create_backup(
    snapshot: PreferenceSnapshot,
    plan: KeymapPlan,
    root: pathlib.Path = BACKUP_ROOT,
    *,
    preference_export: bytes,
) -> pathlib.Path:
    root.mkdir(parents=True, exist_ok=True, mode=0o700)
    os.chmod(root, 0o700)
    timestamp = dt.datetime.now(dt.UTC).strftime("%Y%m%dT%H%M%S.%fZ")
    backup_directory = root / timestamp
    try:
        backup_directory.mkdir(mode=0o700)
    except FileExistsError as error:
        raise MigrationError("A backup with this timestamp already exists") from error

    receipt = Receipt(
        schema_version=RECEIPT_SCHEMA_VERSION,
        iterm_version=snapshot.iterm_version,
        created_at=dt.datetime.now(dt.UTC).isoformat().replace("+00:00", "Z"),
        before_hash=canonical_hash(snapshot.global_map),
        after_hash=canonical_hash(plan.after),
        original_language_agnostic_persisted=(
            snapshot.language_agnostic_persisted
        ),
        original_language_agnostic_value=(
            snapshot.language_agnostic_persisted_value
        ),
        owned_entries=copy.deepcopy(plan.owned_additions),
        result="prepared",
    )
    receipt_path = backup_directory / "receipt.json"
    _atomic_write_private(
        backup_directory / "global-key-map.before.json",
        canonical_json(snapshot.global_map) + b"\n",
    )
    _atomic_write_private(
        backup_directory / "preferences.plist",
        preference_export,
    )
    _atomic_write_private(
        receipt_path,
        canonical_json(dataclasses.asdict(receipt)) + b"\n",
    )
    return receipt_path


def load_receipt(
    path: pathlib.Path,
    root: pathlib.Path = BACKUP_ROOT,
) -> Receipt:
    candidate = pathlib.Path(path)
    root_path = pathlib.Path(root)
    trusted_root = root_path.resolve()
    root_absolute = root_path.absolute()
    candidate_absolute = candidate.absolute()
    if candidate.is_symlink():
        raise MigrationError("Refusing to load a receipt through a symlink")
    try:
        resolved = candidate.resolve(strict=True)
    except FileNotFoundError as error:
        raise MigrationError("The requested receipt does not exist") from error
    if not resolved.is_relative_to(trusted_root):
        raise MigrationError("The receipt is outside the trusted backup root")

    try:
        relative_parts = candidate_absolute.relative_to(root_absolute).parts
    except ValueError as error:
        raise MigrationError(
            "The receipt is outside the trusted backup root"
        ) from error
    current = root_absolute
    for part in relative_parts:
        current /= part
        if current.is_symlink():
            raise MigrationError("Refusing to load a receipt through a symlink")

    try:
        raw = json.loads(resolved.read_text(encoding="utf-8"))
    except (OSError, UnicodeError, json.JSONDecodeError) as error:
        raise MigrationError("Could not parse the migration receipt") from error
    if not isinstance(raw, dict):
        raise MigrationError("The migration receipt is not a dictionary")
    if (
        type(raw.get("schema_version")) is not int
        or raw["schema_version"] != RECEIPT_SCHEMA_VERSION
    ):
        raise MigrationError("Unsupported migration receipt schema")
    expected_fields = {field.name for field in dataclasses.fields(Receipt)}
    if set(raw) != expected_fields:
        raise MigrationError("The migration receipt has unexpected fields")
    if not isinstance(raw.get("owned_entries"), dict):
        raise MigrationError("The migration receipt has invalid owned entries")
    approved_entries = {target.serialized: target.action for target in TARGETS}
    if any(
        key not in approved_entries or action != approved_entries[key]
        for key, action in raw["owned_entries"].items()
    ):
        raise MigrationError("The migration receipt has invalid owned entries")
    original_value = raw.get("original_language_agnostic_value")
    original_value_is_valid = original_value is None or isinstance(
        original_value, bool
    )
    hashes_are_valid = all(
        isinstance(value, str)
        and len(value) == 64
        and all(character in "0123456789abcdef" for character in value)
        for value in (raw.get("before_hash"), raw.get("after_hash"))
    )
    if (
        not isinstance(raw.get("iterm_version"), str)
        or not isinstance(raw.get("created_at"), str)
        or not hashes_are_valid
        or not isinstance(
            raw.get("original_language_agnostic_persisted"), bool
        )
        or not original_value_is_valid
        or (
            raw["original_language_agnostic_persisted"]
            != isinstance(original_value, bool)
        )
        or raw.get("result") != "prepared"
    ):
        raise MigrationError("The migration receipt is invalid")
    try:
        return Receipt(**raw)
    except TypeError as error:
        raise MigrationError("The migration receipt is invalid") from error


def parse_serialized_key(key: str) -> ParsedKey:
    try:
        if key.startswith(":"):
            pieces = key.split(":")
            if len(pieces) == 3 and pieces[0] == "":
                return ParsedKey(
                    "modified",
                    int(pieces[1], 16),
                    int(pieces[2], 16),
                    None,
                )
        else:
            pieces = key.split("-")
            if len(pieces) == 2:
                return ParsedKey(
                    "legacy", int(pieces[0], 16), int(pieces[1], 16), None
                )
            if len(pieces) == 3:
                return ParsedKey(
                    "portable",
                    int(pieces[0], 16),
                    int(pieces[1], 16),
                    int(pieces[2], 16),
                )
    except (AttributeError, TypeError, ValueError) as error:
        raise MigrationError(
            f"Unsupported iTerm2 key serialization: {key}"
        ) from error

    raise MigrationError(f"Unsupported iTerm2 key serialization: {key}")


def target_for(parsed: ParsedKey) -> TargetBinding | None:
    if parsed.modifiers != CONTROL:
        return None

    for target in TARGETS:
        if parsed.kind == "portable" and parsed.keycode == target.keycode:
            return target
        if parsed.kind == "legacy" and parsed.character in {
            target.unmodified,
            target.control_character,
        }:
            return target
        if (
            parsed.kind == "modified"
            and parsed.character == target.control_character
        ):
            return target
    return None


def action_matches(value: Mapping[str, Any], target: TargetBinding) -> bool:
    return (
        value.get("Action") == HEX_CODE_ACTION
        and value.get("Text") == target.text
    )


def plan_keymap(
    global_map: Mapping[str, Any],
    profile_maps: Mapping[str, Mapping[str, Any]],
) -> KeymapPlan:
    before = copy.deepcopy(dict(global_map))
    portable_satisfied: set[str] = set()

    scoped_maps = [("global", global_map), *profile_maps.items()]
    for scope, mapping in scoped_maps:
        for serialized, action in mapping.items():
            parsed = parse_serialized_key(serialized)
            target = target_for(parsed)
            if target is None:
                continue
            if not isinstance(action, Mapping):
                raise MigrationError(
                    f"Conflict in {scope}: {serialized} has no action dictionary"
                )
            if not action_matches(action, target):
                raise MigrationError(
                    f"Conflict in {scope}: {serialized} -> {action!r}"
                )
            if scope == "global" and parsed.kind == "portable":
                portable_satisfied.add(target.name)

    after = copy.deepcopy(before)
    owned_additions: dict[str, Any] = {}
    for target in TARGETS:
        if target.name in portable_satisfied:
            continue
        after[target.serialized] = target.action
        owned_additions[target.serialized] = target.action

    return KeymapPlan(
        before=before,
        after=after,
        owned_additions=owned_additions,
        already_satisfied=tuple(sorted(portable_satisfied)),
    )


async def _assert_snapshot_is_current(
    client: PreferenceClient,
    snapshot: PreferenceSnapshot,
) -> None:
    current_map = await client.get_preference(GLOBAL_MAP_KEY)
    current_flag = await client.get_preference(LANGUAGE_AGNOSTIC_KEY)
    current_profiles = await client.profile_maps()
    if (
        canonical_hash(current_map) != canonical_hash(snapshot.global_map)
        or bool(current_flag) != snapshot.language_agnostic_effective
        or canonical_hash(current_profiles) != canonical_hash(snapshot.profile_maps)
    ):
        raise MigrationError("iTerm2 preferences changed after preflight")


async def _restore_snapshot(
    client: PreferenceClient,
    snapshot: PreferenceSnapshot,
) -> None:
    await client.set_preference(GLOBAL_MAP_KEY, snapshot.global_map)
    if snapshot.language_agnostic_persisted:
        await client.set_preference(
            LANGUAGE_AGNOSTIC_KEY,
            snapshot.language_agnostic_persisted_value,
        )
    else:
        await client.unset_preference(LANGUAGE_AGNOSTIC_KEY)
    restored_map = await client.get_preference(GLOBAL_MAP_KEY)
    restored_flag = await client.get_preference(LANGUAGE_AGNOSTIC_KEY)
    if (
        canonical_hash(restored_map) != canonical_hash(snapshot.global_map)
        or bool(restored_flag) != snapshot.language_agnostic_effective
    ):
        raise MigrationError("Automatic restoration could not be verified")


async def apply_configuration(
    client: PreferenceClient,
    snapshot: PreferenceSnapshot,
    plan: KeymapPlan,
) -> None:
    await _assert_snapshot_is_current(client, snapshot)
    try:
        await client.set_preference(GLOBAL_MAP_KEY, plan.after)
        await client.set_preference(LANGUAGE_AGNOSTIC_KEY, True)
        applied_map = await client.get_preference(GLOBAL_MAP_KEY)
        applied_flag = await client.get_preference(LANGUAGE_AGNOSTIC_KEY)
        if canonical_hash(applied_map) != canonical_hash(plan.after):
            raise MigrationError("The applied global key map did not verify")
        if applied_flag is not True:
            raise MigrationError(
                "The applied language-agnostic preference did not verify"
            )
    except Exception as apply_error:
        try:
            await _restore_snapshot(client, snapshot)
        except Exception as restore_error:
            raise MigrationError(
                "Apply failed and automatic restoration also failed: "
                f"{type(apply_error).__name__}; "
                f"{type(restore_error).__name__}"
            ) from restore_error
        raise MigrationError(
            "Apply failed; the original iTerm2 preferences were restored"
        ) from apply_error


async def restore_configuration(
    client: PreferenceClient,
    receipt: Receipt,
) -> RestoreResult:
    current_map = await client.get_preference(GLOBAL_MAP_KEY)
    current_flag = await client.get_preference(LANGUAGE_AGNOSTIC_KEY)
    if not isinstance(current_map, Mapping):
        raise MigrationError("The current global key map is not a dictionary")

    for serialized, owned_action in receipt.owned_entries.items():
        if current_map.get(serialized) != owned_action:
            raise MigrationError(
                f"Refusing restore because an owned entry changed: {serialized}"
            )

    restored_map = copy.deepcopy(dict(current_map))
    for serialized in receipt.owned_entries:
        del restored_map[serialized]

    map_matches_original = canonical_hash(restored_map) == receipt.before_hash
    warning = None
    if not map_matches_original:
        warning = (
            "Removed owned Control-C/Control-G entries; kept "
            "LanguageAgnosticKeyBindings enabled because unrelated "
            "mappings changed."
        )

    try:
        await client.set_preference(GLOBAL_MAP_KEY, restored_map)
        if map_matches_original:
            if receipt.original_language_agnostic_persisted:
                await client.set_preference(
                    LANGUAGE_AGNOSTIC_KEY,
                    receipt.original_language_agnostic_value,
                )
            else:
                await client.unset_preference(LANGUAGE_AGNOSTIC_KEY)

        verified_map = await client.get_preference(GLOBAL_MAP_KEY)
        verified_flag = await client.get_preference(LANGUAGE_AGNOSTIC_KEY)
        if canonical_hash(verified_map) != canonical_hash(restored_map):
            raise MigrationError("The restored global key map did not verify")
        if map_matches_original:
            expected_flag = bool(receipt.original_language_agnostic_value)
            if bool(verified_flag) != expected_flag:
                raise MigrationError(
                    "The restored language-agnostic preference did not verify"
                )
        elif verified_flag is not True:
            raise MigrationError(
                "The language-agnostic preference was not preserved"
            )
    except Exception as restore_error:
        try:
            await client.set_preference(GLOBAL_MAP_KEY, current_map)
            await client.set_preference(LANGUAGE_AGNOSTIC_KEY, current_flag)
        except Exception as rollback_error:
            raise MigrationError(
                "Restore failed and its compensating rollback also failed: "
                f"{type(restore_error).__name__}; "
                f"{type(rollback_error).__name__}"
            ) from rollback_error
        raise MigrationError(
            "Restore failed; its changes were rolled back"
        ) from restore_error

    return RestoreResult(
        map_hash=canonical_hash(restored_map),
        language_agnostic_restored=map_matches_original,
        warning=warning,
    )


def main(argv: list[str] | None = None) -> int:
    try:
        args = build_parser().parse_args(argv)
        iterm_version = read_iterm_version()
        preference_export = export_persistent_domain()

        import iterm2  # Deferred so standard-library tests stay dependency-free.

        async def execute(connection: Any) -> None:
            client = ItermPreferenceClient(iterm2, connection)
            await run_command(
                args,
                client,
                iterm2,
                connection,
                iterm_version=iterm_version,
                preference_export=preference_export,
            )

        iterm2.run_until_complete(execute)
        return 0
    except MigrationError as error:
        print(f"ERROR: {error}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
