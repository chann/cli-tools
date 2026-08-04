from __future__ import annotations

import copy
import dataclasses
import hashlib
import json
from collections.abc import Mapping
from typing import Any


CONTROL = 0x40000
HEX_CODE_ACTION = 11


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


def canonical_json(value: Any) -> bytes:
    return json.dumps(
        value,
        sort_keys=True,
        separators=(",", ":"),
        ensure_ascii=False,
    ).encode("utf-8")


def canonical_hash(value: Any) -> str:
    return hashlib.sha256(canonical_json(value)).hexdigest()


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
