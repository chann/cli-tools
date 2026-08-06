from __future__ import annotations

import argparse
import dataclasses
import datetime as dt
import hashlib
import json
import os
import pathlib
import re
import stat
import subprocess
import sys
import tempfile
from collections.abc import Callable, Sequence


class MigrationError(RuntimeError):
    pass


EXPECTED_GHOSTTY_VERSION = "1.3.1"
GHOSTTY_BINARY = pathlib.Path("/Applications/Ghostty.app/Contents/MacOS/ghostty")
XATTR_BINARY = pathlib.Path("/usr/bin/xattr")
SETTING_HISTORY_SCHEMA_VERSION = 1
BEGIN_MARKER = "# BEGIN cli-tools ghostty-korean-control-keys"
END_MARKER = "# END cli-tools ghostty-korean-control-keys"
TARGET_BINDINGS = {
    "ctrl+c": r"text:\x03",
    "ctrl+g": r"text:\x07",
}
BACKUP_ROOT = (
    pathlib.Path.home()
    / "Library"
    / "Application Support"
    / "cli-tools"
    / "ghostty-korean-control-keys"
)


@dataclasses.dataclass(frozen=True)
class ConfigPlan:
    before: bytes
    after: bytes
    owned_bindings: dict[str, str]
    owned_append: bytes


@dataclasses.dataclass(frozen=True)
class SettingHistory:
    schema_version: int
    ghostty_version: str
    created_at: str
    config_path: str
    before_hash: str
    after_hash: str
    owned_bindings: dict[str, str]
    owned_append: str
    result: str


@dataclasses.dataclass(frozen=True)
class EnvironmentSnapshot:
    ghostty_version: str
    config_path: pathlib.Path
    effective_keybinds: str
    plan: ConfigPlan


def content_hash(content: bytes) -> str:
    return hashlib.sha256(content).hexdigest()


def effective_target_actions(output: str) -> dict[str, str]:
    actions: dict[str, str] = {}
    pattern = re.compile(
        r"^keybind = "
        r"(?P<prefix>(?:(?:all|global|unconsumed|performable):)*)"
        r"(?P<trigger>ctrl\+[cg])=(?P<action>.*)$"
    )
    for line in output.splitlines():
        match = pattern.match(line)
        if match is None:
            continue
        prefix = match.group("prefix")
        action = match.group("action")
        actions[match.group("trigger")] = f"{prefix}{action}"
    return actions


def _managed_block(content: bytes) -> bytes | None:
    begin = BEGIN_MARKER.encode()
    end = END_MARKER.encode()
    begin_count = content.count(begin)
    end_count = content.count(end)
    if begin_count == 0 and end_count == 0:
        return None
    if begin_count != 1 or end_count != 1:
        raise MigrationError("The managed block markers are incomplete")
    start = content.index(begin)
    try:
        finish = content.index(end, start) + len(end)
    except ValueError as error:
        raise MigrationError(
            "The managed block markers are in reverse order"
        ) from error
    if finish < len(content) and content[finish : finish + 1] == b"\n":
        finish += 1
    block = content[start:finish]
    lines = block.decode("utf-8").splitlines()
    allowed = {
        f"keybind = {trigger}={action}" for trigger, action in TARGET_BINDINGS.items()
    }
    body = lines[1:-1]
    if (
        not body
        or lines[0] != BEGIN_MARKER
        or lines[-1] != END_MARKER
        or any(line not in allowed for line in body)
        or len(body) != len(set(body))
    ):
        raise MigrationError("The managed block was edited")
    return block


def plan_configuration(before: bytes, effective_output: str) -> ConfigPlan:
    try:
        before.decode("utf-8")
    except UnicodeDecodeError as error:
        raise MigrationError("The Ghostty config is not valid UTF-8") from error

    actions = effective_target_actions(effective_output)
    for trigger, action in actions.items():
        if action != TARGET_BINDINGS[trigger]:
            raise MigrationError(
                f"{trigger} has a conflicting effective action: {action}"
            )

    existing_block = _managed_block(before)
    if existing_block is not None:
        if any(
            actions.get(trigger) != expected
            for trigger, expected in TARGET_BINDINGS.items()
        ):
            raise MigrationError(
                "The managed block is not active in the effective keymap"
            )
        return ConfigPlan(before, before, {}, b"")

    additions = {
        trigger: action
        for trigger, action in TARGET_BINDINGS.items()
        if trigger not in actions
    }
    if not additions:
        return ConfigPlan(before, before, {}, b"")

    prefix = b"" if not before else (b"\n" if before.endswith(b"\n") else b"\n\n")
    lines = [BEGIN_MARKER]
    lines.extend(
        f"keybind = {trigger}={action}" for trigger, action in additions.items()
    )
    lines.append(END_MARKER)
    owned_append = prefix + ("\n".join(lines) + "\n").encode("utf-8")
    return ConfigPlan(
        before=before,
        after=before + owned_append,
        owned_bindings=additions,
        owned_append=owned_append,
    )


def remove_owned_append(current: bytes, owned_append: bytes) -> bytes:
    if not owned_append:
        raise MigrationError("The setting history owns no managed block")
    if current.count(owned_append) != 1:
        raise MigrationError("The managed block changed after apply")
    return current.replace(owned_append, b"", 1)


def _has_directive(path: pathlib.Path) -> bool:
    try:
        text = path.read_text(encoding="utf-8")
    except UnicodeDecodeError as error:
        raise MigrationError(f"Config is not valid UTF-8: {path}") from error
    return any(
        stripped and not stripped.startswith("#")
        for stripped in (line.strip() for line in text.splitlines())
    )


def select_managed_config(candidates: Sequence[pathlib.Path]) -> pathlib.Path:
    existing = [path for path in candidates if path.exists()]
    configured = [path for path in existing if _has_directive(path)]
    if len(configured) > 1:
        rendered = ", ".join(str(path) for path in configured)
        raise MigrationError(f"Refusing to guess between multiple configs: {rendered}")
    if configured:
        return configured[0]
    if existing:
        return existing[-1]
    if not candidates:
        raise MigrationError("No Ghostty config candidates were provided")
    return pathlib.Path(candidates[0])


def _atomic_write_private(path: pathlib.Path, content: bytes) -> None:
    descriptor, temporary_name = tempfile.mkstemp(dir=path.parent)
    temporary = pathlib.Path(temporary_name)
    try:
        os.fchmod(descriptor, 0o600)
        with os.fdopen(descriptor, "wb") as handle:
            descriptor = -1
            handle.write(content)
            handle.flush()
            os.fsync(handle.fileno())
        os.replace(temporary, path)
    finally:
        if descriptor >= 0:
            os.close(descriptor)
        try:
            temporary.unlink()
        except FileNotFoundError:
            pass


def create_setting_history(
    config_path: pathlib.Path,
    plan: ConfigPlan,
    root: pathlib.Path = BACKUP_ROOT,
    *,
    ghostty_version: str,
) -> pathlib.Path:
    root.mkdir(parents=True, exist_ok=True, mode=0o700)
    os.chmod(root, 0o700)
    timestamp = dt.datetime.now(dt.UTC).strftime("%Y%m%dT%H%M%S.%fZ")
    history_directory = root / timestamp
    try:
        history_directory.mkdir(mode=0o700)
    except FileExistsError as error:
        raise MigrationError(
            "Setting history already exists for this timestamp"
        ) from error

    setting_history = SettingHistory(
        schema_version=SETTING_HISTORY_SCHEMA_VERSION,
        ghostty_version=ghostty_version,
        created_at=dt.datetime.now(dt.UTC).isoformat().replace("+00:00", "Z"),
        config_path=str(config_path),
        before_hash=content_hash(plan.before),
        after_hash=content_hash(plan.after),
        owned_bindings=dict(plan.owned_bindings),
        owned_append=plan.owned_append.decode("utf-8"),
        result="prepared",
    )
    history_path = history_directory / "setting_history.json"
    _atomic_write_private(history_directory / "config.before", plan.before)
    _atomic_write_private(
        history_path,
        json.dumps(
            dataclasses.asdict(setting_history),
            ensure_ascii=False,
            sort_keys=True,
            separators=(",", ":"),
        ).encode("utf-8")
        + b"\n",
    )
    return history_path


def load_setting_history(
    path: pathlib.Path,
    root: pathlib.Path = BACKUP_ROOT,
) -> SettingHistory:
    candidate = pathlib.Path(path)
    root_path = pathlib.Path(root)
    trusted_root = root_path.resolve()
    root_absolute = root_path.absolute()
    candidate_absolute = candidate.absolute()
    if candidate.is_symlink():
        raise MigrationError("Refusing to load setting history through a symlink")
    try:
        resolved = candidate.resolve(strict=True)
    except FileNotFoundError as error:
        raise MigrationError("The requested setting history does not exist") from error
    if not resolved.is_relative_to(trusted_root):
        raise MigrationError("The setting history is outside the trusted root")
    try:
        relative_parts = candidate_absolute.relative_to(root_absolute).parts
    except ValueError as error:
        raise MigrationError(
            "The setting history is outside the trusted root"
        ) from error
    current = root_absolute
    for part in relative_parts:
        current /= part
        if current.is_symlink():
            raise MigrationError("Refusing to load setting history through a symlink")

    try:
        raw = json.loads(resolved.read_text(encoding="utf-8"))
        before = (resolved.parent / "config.before").read_bytes()
    except (OSError, UnicodeError, json.JSONDecodeError) as error:
        raise MigrationError("Could not parse the setting history") from error
    if not isinstance(raw, dict):
        raise MigrationError("The setting history is not a dictionary")
    expected_fields = {field.name for field in dataclasses.fields(SettingHistory)}
    if set(raw) != expected_fields:
        raise MigrationError("The setting history has unexpected fields")
    if (
        type(raw.get("schema_version")) is not int
        or raw["schema_version"] != SETTING_HISTORY_SCHEMA_VERSION
    ):
        raise MigrationError("Unsupported setting history schema")
    owned_bindings = raw.get("owned_bindings")
    if (
        not isinstance(owned_bindings, dict)
        or not owned_bindings
        or any(
            trigger not in TARGET_BINDINGS or action != TARGET_BINDINGS[trigger]
            for trigger, action in owned_bindings.items()
        )
    ):
        raise MigrationError("The setting history has invalid owned bindings")
    hashes = (raw.get("before_hash"), raw.get("after_hash"))
    if not all(
        isinstance(value, str)
        and len(value) == 64
        and all(character in "0123456789abcdef" for character in value)
        for value in hashes
    ):
        raise MigrationError("The setting history has invalid hashes")
    config_path = pathlib.Path(str(raw.get("config_path", "")))
    owned_append_text = raw.get("owned_append")
    if (
        not config_path.is_absolute()
        or not isinstance(raw.get("ghostty_version"), str)
        or not isinstance(raw.get("created_at"), str)
        or not isinstance(owned_append_text, str)
        or raw.get("result") != "prepared"
    ):
        raise MigrationError("The setting history is invalid")
    owned_append = owned_append_text.encode("utf-8")
    block = _managed_block(owned_append)
    if block is None:
        raise MigrationError("The setting history has no managed block")
    recorded_actions = effective_target_actions(
        "\n".join(
            line
            for line in block.decode("utf-8").splitlines()
            if line.startswith("keybind = ")
        )
    )
    if recorded_actions != owned_bindings:
        raise MigrationError("The setting history has invalid owned bindings")
    if (
        content_hash(before) != raw["before_hash"]
        or content_hash(before + owned_append) != raw["after_hash"]
    ):
        raise MigrationError("The setting history does not match its backup")
    try:
        return SettingHistory(**raw)
    except TypeError as error:
        raise MigrationError("The setting history is invalid") from error


def config_candidates(
    home: pathlib.Path,
    xdg_config_home: pathlib.Path | None,
) -> tuple[pathlib.Path, ...]:
    xdg_root = xdg_config_home or home / ".config"
    xdg = xdg_root / "ghostty"
    macos = home / "Library" / "Application Support" / "com.mitchellh.ghostty"
    return (
        xdg / "config.ghostty",
        xdg / "config",
        macos / "config.ghostty",
        macos / "config",
    )


def read_ghostty_version(*, run: Callable = subprocess.run) -> str:
    try:
        completed = run(
            [str(GHOSTTY_BINARY), "+version"],
            check=True,
            capture_output=True,
            text=True,
        )
    except (OSError, subprocess.CalledProcessError) as error:
        raise MigrationError("Could not read the installed Ghostty version") from error
    first_line = completed.stdout.splitlines()[0] if completed.stdout else ""
    prefix = "Ghostty "
    if not first_line.startswith(prefix):
        raise MigrationError("Ghostty returned an invalid version")
    return first_line.removeprefix(prefix).strip()


def validate_config(*, run: Callable = subprocess.run) -> None:
    try:
        run(
            [str(GHOSTTY_BINARY), "+validate-config"],
            check=True,
            capture_output=True,
            text=True,
        )
    except (OSError, subprocess.CalledProcessError) as error:
        raise MigrationError("Ghostty rejected the effective configuration") from error


def read_effective_keybinds(*, run: Callable = subprocess.run) -> str:
    try:
        completed = run(
            [str(GHOSTTY_BINARY), "+list-keybinds", "--plain"],
            check=True,
            capture_output=True,
            text=True,
        )
    except (OSError, subprocess.CalledProcessError) as error:
        raise MigrationError(
            "Could not read Ghostty's effective keybindings"
        ) from error
    return completed.stdout


def _read_xattrs(path: pathlib.Path) -> dict[str, str]:
    if not path.exists():
        return {}
    names = subprocess.run(
        [str(XATTR_BINARY), str(path)],
        check=True,
        capture_output=True,
        text=True,
    ).stdout.splitlines()
    return {
        name: "".join(
            subprocess.run(
                [str(XATTR_BINARY), "-px", name, str(path)],
                check=True,
                capture_output=True,
                text=True,
            ).stdout.split()
        )
        for name in names
    }


def _atomic_replace_config(
    path: pathlib.Path,
    content: bytes,
    *,
    mode: int,
    xattrs: dict[str, str],
) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    descriptor, temporary_name = tempfile.mkstemp(dir=path.parent)
    temporary = pathlib.Path(temporary_name)
    try:
        os.fchmod(descriptor, mode)
        with os.fdopen(descriptor, "wb") as handle:
            descriptor = -1
            handle.write(content)
            handle.flush()
            os.fsync(handle.fileno())
        for name, hexadecimal_value in xattrs.items():
            subprocess.run(
                [
                    str(XATTR_BINARY),
                    "-wx",
                    name,
                    hexadecimal_value,
                    str(temporary),
                ],
                check=True,
                capture_output=True,
                text=True,
            )
        os.replace(temporary, path)
    finally:
        if descriptor >= 0:
            os.close(descriptor)
        try:
            temporary.unlink()
        except FileNotFoundError:
            pass


def write_config_transactionally(
    path: pathlib.Path,
    plan: ConfigPlan,
    *,
    verify: Callable[[], None],
) -> None:
    if path.is_symlink():
        raise MigrationError("Refusing to replace a Ghostty config symlink")
    existed = path.exists()
    current = path.read_bytes() if existed else b""
    if current != plan.before:
        raise MigrationError("The Ghostty config changed after preflight")
    if plan.after == plan.before:
        return
    mode = stat.S_IMODE(path.stat().st_mode) if existed else 0o600
    xattrs = _read_xattrs(path)
    try:
        _atomic_replace_config(path, plan.after, mode=mode, xattrs=xattrs)
        verify()
    except Exception as apply_error:
        try:
            if existed:
                _atomic_replace_config(
                    path,
                    plan.before,
                    mode=mode,
                    xattrs=xattrs,
                )
            else:
                path.unlink(missing_ok=True)
            if existed and path.read_bytes() != plan.before:
                raise MigrationError("The rollback did not restore the config")
        except Exception as restore_error:
            raise MigrationError(
                "Apply failed and automatic restoration also failed: "
                f"{type(apply_error).__name__}; "
                f"{type(restore_error).__name__}"
            ) from restore_error
        raise MigrationError(
            "Apply failed; the original Ghostty config was restored"
        ) from apply_error


def inspect_environment(
    *,
    run: Callable = subprocess.run,
    home: pathlib.Path,
    xdg_config_home: pathlib.Path | None,
) -> EnvironmentSnapshot:
    version = read_ghostty_version(run=run)
    if version != EXPECTED_GHOSTTY_VERSION:
        raise MigrationError(
            f"Expected Ghostty {EXPECTED_GHOSTTY_VERSION}, found {version}"
        )
    validate_config(run=run)
    config_path = select_managed_config(config_candidates(home, xdg_config_home))
    if config_path.is_symlink():
        raise MigrationError("Refusing to manage a Ghostty config symlink")
    before = config_path.read_bytes() if config_path.exists() else b""
    effective = read_effective_keybinds(run=run)
    plan = plan_configuration(before, effective)
    return EnvironmentSnapshot(
        ghostty_version=version,
        config_path=config_path,
        effective_keybinds=effective,
        plan=plan,
    )


def _keybind_lines(output: str) -> set[str]:
    return {line for line in output.splitlines() if line.startswith("keybind = ")}


def verify_applied_keybinds(before: str, after: str) -> None:
    missing = _keybind_lines(before) - _keybind_lines(after)
    if missing:
        raise MigrationError(
            "Existing Ghostty keybindings changed during apply: "
            + ", ".join(sorted(missing))
        )
    actions = effective_target_actions(after)
    if actions != TARGET_BINDINGS:
        raise MigrationError(
            "The physical Control-C/Control-G configuration is incomplete"
        )


def verify_restored_keybinds(
    before: str,
    after: str,
    owned_bindings: dict[str, str],
) -> None:
    prefixes = tuple(f"keybind = {trigger}=" for trigger in owned_bindings)
    expected = {
        line for line in _keybind_lines(before) if not line.startswith(prefixes)
    }
    missing = expected - _keybind_lines(after)
    if missing:
        raise MigrationError(
            "Existing Ghostty keybindings changed during restore: "
            + ", ".join(sorted(missing))
        )


def _print_plan(snapshot: EnvironmentSnapshot, output: object) -> None:
    print(f"Ghostty: {snapshot.ghostty_version}", file=output)
    print(f"Managed config: {snapshot.config_path}", file=output)
    if snapshot.plan.owned_bindings:
        for trigger, action in snapshot.plan.owned_bindings.items():
            print(f"ADD {trigger} -> {action}", file=output)
    else:
        print("No keybinding changes required", file=output)


def run_command(
    args: argparse.Namespace,
    *,
    run: Callable = subprocess.run,
    home: pathlib.Path,
    xdg_config_home: pathlib.Path | None,
    backup_root: pathlib.Path = BACKUP_ROOT,
    output: object = sys.stdout,
) -> None:
    snapshot = inspect_environment(
        run=run,
        home=home,
        xdg_config_home=xdg_config_home,
    )

    if args.command == "preflight":
        _print_plan(snapshot, output)
        print("PREFLIGHT OK: no files were changed", file=output)
        return

    if args.command == "verify":
        actions = effective_target_actions(snapshot.effective_keybinds)
        if actions != TARGET_BINDINGS:
            raise MigrationError(
                "The physical Control-C/Control-G configuration is incomplete"
            )
        print(
            f"VERIFIED config hash: {content_hash(snapshot.plan.before)}", file=output
        )
        for trigger, action in TARGET_BINDINGS.items():
            print(f"VERIFIED {trigger} -> {action}", file=output)
        return

    if args.command == "apply":
        _print_plan(snapshot, output)
        if not snapshot.plan.owned_bindings:
            print("ALREADY CONFIGURED: no files were changed", file=output)
            return
        history_path = create_setting_history(
            snapshot.config_path,
            snapshot.plan,
            backup_root,
            ghostty_version=snapshot.ghostty_version,
        )
        print(f"Private setting history: {history_path}", file=output)

        def verify_apply() -> None:
            validate_config(run=run)
            after = read_effective_keybinds(run=run)
            verify_applied_keybinds(snapshot.effective_keybinds, after)

        write_config_transactionally(
            snapshot.config_path,
            snapshot.plan,
            verify=verify_apply,
        )
        print(f"APPLIED config hash: {content_hash(snapshot.plan.after)}", file=output)
        return

    if args.command != "restore":
        raise MigrationError(f"Unsupported command: {args.command}")

    setting_history = load_setting_history(args.history, backup_root)
    if setting_history.ghostty_version != snapshot.ghostty_version:
        raise MigrationError(
            "The setting history was created for a different Ghostty version"
        )
    history_config = pathlib.Path(setting_history.config_path)
    if history_config != snapshot.config_path:
        raise MigrationError(
            "The setting history does not match the active managed config"
        )
    current = history_config.read_bytes()
    owned_append = setting_history.owned_append.encode("utf-8")
    restored = remove_owned_append(current, owned_append)
    clean_restore = content_hash(current) == setting_history.after_hash
    restore_plan = ConfigPlan(
        before=current,
        after=restored,
        owned_bindings={},
        owned_append=b"",
    )

    def verify_restore() -> None:
        validate_config(run=run)
        after = read_effective_keybinds(run=run)
        verify_restored_keybinds(
            snapshot.effective_keybinds,
            after,
            setting_history.owned_bindings,
        )
        if clean_restore:
            remaining = effective_target_actions(after)
            if any(trigger in remaining for trigger in setting_history.owned_bindings):
                raise MigrationError(
                    "An owned Ghostty keybinding remained after restore"
                )

    write_config_transactionally(
        history_config,
        restore_plan,
        verify=verify_restore,
    )
    if not clean_restore:
        print(
            "WARNING: removed the managed block and preserved later config edits",
            file=output,
        )
    print(f"RESTORED config hash: {content_hash(restored)}", file=output)


def absolute_path(value: str) -> pathlib.Path:
    path = pathlib.Path(value)
    if not path.is_absolute():
        raise MigrationError("restore requires an absolute setting history path")
    return path


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        description="Safely configure physical Control-C and Control-G in Ghostty"
    )
    subparsers = parser.add_subparsers(dest="command", required=True)
    subparsers.add_parser("preflight")
    subparsers.add_parser("apply")
    subparsers.add_parser("verify")
    restore_parser = subparsers.add_parser("restore")
    restore_parser.add_argument(
        "--history",
        required=True,
        type=absolute_path,
    )
    return parser


def main() -> int:
    args = build_parser().parse_args()
    xdg_value = os.environ.get("XDG_CONFIG_HOME")
    xdg_path = pathlib.Path(xdg_value) if xdg_value else None
    try:
        run_command(
            args,
            home=pathlib.Path.home(),
            xdg_config_home=xdg_path,
        )
    except MigrationError as error:
        print(f"ERROR: {error}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
