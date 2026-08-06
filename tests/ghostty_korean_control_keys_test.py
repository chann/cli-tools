from __future__ import annotations

import json
import io
import os
import pathlib
import stat
import subprocess
import sys
import tempfile
import unittest

from scripts import ghostty_korean_control_keys as module


ROOT = pathlib.Path(__file__).resolve().parents[1]
SCRIPT = ROOT / "scripts" / "ghostty_korean_control_keys.py"


class CommandSurfaceTests(unittest.TestCase):
    def test_exposes_the_reversible_command_surface(self):
        completed = subprocess.run(
            [sys.executable, str(SCRIPT), "--help"],
            check=False,
            capture_output=True,
            text=True,
        )

        self.assertEqual(completed.returncode, 0, completed.stderr)
        self.assertIn("{preflight,apply,verify,restore}", completed.stdout)

    def test_restore_requires_an_absolute_setting_history_path(self):
        with self.assertRaisesRegex(
            module.MigrationError,
            "absolute setting history path",
        ):
            module.absolute_path("relative/setting_history.json")


class ConfigurationPlannerTests(unittest.TestCase):
    def test_appends_only_missing_bindings_without_changing_existing_bytes(self):
        before = b"font-family = JetBrains Mono\nkeybind = ctrl+n=new_window\n"
        effective = "keybind = ctrl+n=new_window\n"

        plan = module.plan_configuration(before, effective)

        self.assertEqual(plan.before, before)
        self.assertTrue(plan.after.startswith(before))
        self.assertEqual(
            plan.owned_bindings,
            {
                "ctrl+c": r"text:\x03",
                "ctrl+g": r"text:\x07",
            },
        )
        self.assertIn(b"keybind = ctrl+c=text:\\x03\n", plan.owned_append)
        self.assertIn(b"keybind = ctrl+g=text:\\x07\n", plan.owned_append)
        self.assertEqual(plan.after, before + plan.owned_append)

    def test_stops_when_an_effective_target_has_a_different_action(self):
        effective = "keybind = ctrl+c=copy_to_clipboard\n"

        with self.assertRaisesRegex(module.MigrationError, r"ctrl\+c.*conflict"):
            module.plan_configuration(b"", effective)

    def test_stops_when_a_prefixed_binding_owns_the_same_trigger(self):
        effective = "keybind = global:performable:ctrl+c=toggle_quick_terminal\n"

        with self.assertRaisesRegex(module.MigrationError, r"ctrl\+c.*conflict"):
            module.plan_configuration(b"", effective)

    def test_adds_only_the_target_missing_from_the_effective_keymap(self):
        effective = "keybind = ctrl+c=text:\\x03\n"

        plan = module.plan_configuration(b"theme = dark\n", effective)

        self.assertEqual(plan.owned_bindings, {"ctrl+g": r"text:\x07"})
        self.assertNotIn(b"ctrl+c", plan.owned_append)
        self.assertIn(b"keybind = ctrl+g=text:\\x07", plan.owned_append)

    def test_refuses_an_edited_or_partial_managed_block(self):
        before = b"# BEGIN cli-tools ghostty-korean-control-keys\n"

        with self.assertRaisesRegex(module.MigrationError, "managed block"):
            module.plan_configuration(before, "")

    def test_refuses_managed_markers_in_reverse_order(self):
        before = (
            b"# END cli-tools ghostty-korean-control-keys\n"
            b"# BEGIN cli-tools ghostty-korean-control-keys\n"
        )

        with self.assertRaisesRegex(module.MigrationError, "managed block"):
            module.plan_configuration(before, "")

    def test_restore_removes_exact_owned_append_and_keeps_later_edits(self):
        before = b"theme = dark\n"
        plan = module.plan_configuration(before, "")
        current = plan.after + b"font-size = 13\n"

        restored = module.remove_owned_append(current, plan.owned_append)

        self.assertEqual(restored, before + b"font-size = 13\n")

    def test_restore_refuses_an_owned_append_changed_after_apply(self):
        plan = module.plan_configuration(b"theme = dark\n", "")
        edited = plan.after.replace(b"text:\\x03", b"text:\\x04")

        with self.assertRaisesRegex(module.MigrationError, "managed block changed"):
            module.remove_owned_append(edited, plan.owned_append)


class ConfigSelectionTests(unittest.TestCase):
    def test_selects_the_only_file_with_real_directives(self):
        with tempfile.TemporaryDirectory() as directory:
            root = pathlib.Path(directory)
            xdg = root / "xdg-config"
            macos = root / "macos-config"
            xdg.write_text("theme = dark\n", encoding="utf-8")
            macos.write_text("# generated template\n", encoding="utf-8")

            selected = module.select_managed_config((xdg, macos))

            self.assertEqual(selected, xdg)

    def test_refuses_to_guess_between_multiple_configured_files(self):
        with tempfile.TemporaryDirectory() as directory:
            root = pathlib.Path(directory)
            first = root / "first"
            second = root / "second"
            first.write_text("theme = dark\n", encoding="utf-8")
            second.write_text("font-size = 12\n", encoding="utf-8")

            with self.assertRaisesRegex(module.MigrationError, "multiple"):
                module.select_managed_config((first, second))


class SettingHistoryTests(unittest.TestCase):
    def test_history_is_private_and_records_the_exact_owned_append(self):
        plan = module.plan_configuration(b"theme = dark\n", "")

        with tempfile.TemporaryDirectory() as directory:
            history_path = module.create_setting_history(
                pathlib.Path("/tmp/ghostty-config"),
                plan,
                pathlib.Path(directory) / "history",
                ghostty_version="1.3.1",
            )

            self.assertEqual(history_path.name, "setting_history.json")
            self.assertEqual(
                stat.S_IMODE(history_path.parent.stat().st_mode),
                0o700,
            )
            for path in history_path.parent.iterdir():
                self.assertEqual(stat.S_IMODE(path.stat().st_mode), 0o600)
            history = json.loads(history_path.read_text(encoding="utf-8"))
            self.assertEqual(history["config_path"], "/tmp/ghostty-config")
            self.assertEqual(
                history["owned_append"].encode("utf-8"),
                plan.owned_append,
            )
            self.assertEqual(
                (history_path.parent / "config.before").read_bytes(),
                plan.before,
            )

    def test_loader_rejects_outside_paths_symlinks_and_tampering(self):
        plan = module.plan_configuration(b"theme = dark\n", "")

        with tempfile.TemporaryDirectory() as directory:
            base = pathlib.Path(directory)
            root = base / "history"
            history_path = module.create_setting_history(
                pathlib.Path("/tmp/ghostty-config"),
                plan,
                root,
                ghostty_version="1.3.1",
            )
            outside = base / "outside.json"
            outside.write_bytes(history_path.read_bytes())
            with self.assertRaisesRegex(module.MigrationError, "trusted"):
                module.load_setting_history(outside, root)

            link = history_path.parent / "linked-history.json"
            os.symlink(history_path, link)
            with self.assertRaisesRegex(module.MigrationError, "symlink"):
                module.load_setting_history(link, root)

            history = json.loads(history_path.read_text(encoding="utf-8"))
            history["owned_bindings"]["ctrl+c"] = r"text:\x04"
            history_path.write_text(json.dumps(history), encoding="utf-8")
            with self.assertRaisesRegex(module.MigrationError, "owned bindings"):
                module.load_setting_history(history_path, root)


class TransactionalWriteTests(unittest.TestCase):
    def test_write_preserves_mode_and_extended_attributes(self):
        with tempfile.TemporaryDirectory() as directory:
            path = pathlib.Path(directory) / "config"
            before = b"theme = dark\n"
            path.write_bytes(before)
            path.chmod(0o640)
            subprocess.run(
                [
                    "/usr/bin/xattr",
                    "-w",
                    "user.cli-tools-test",
                    "preserved",
                    str(path),
                ],
                check=True,
            )
            plan = module.plan_configuration(before, "")

            module.write_config_transactionally(
                path,
                plan,
                verify=lambda: self.assertEqual(path.read_bytes(), plan.after),
            )

            self.assertEqual(path.read_bytes(), plan.after)
            self.assertEqual(stat.S_IMODE(path.stat().st_mode), 0o640)
            self.assertEqual(
                subprocess.run(
                    [
                        "/usr/bin/xattr",
                        "-p",
                        "user.cli-tools-test",
                        str(path),
                    ],
                    check=True,
                    capture_output=True,
                    text=True,
                ).stdout.rstrip("\n"),
                "preserved",
            )

    def test_verification_failure_restores_exact_original_file(self):
        with tempfile.TemporaryDirectory() as directory:
            path = pathlib.Path(directory) / "config"
            before = b"theme = dark\n"
            path.write_bytes(before)
            path.chmod(0o640)
            plan = module.plan_configuration(before, "")

            with self.assertRaisesRegex(module.MigrationError, "restored"):
                module.write_config_transactionally(
                    path,
                    plan,
                    verify=lambda: (_ for _ in ()).throw(
                        RuntimeError("invalid effective keymap")
                    ),
                )

            self.assertEqual(path.read_bytes(), before)
            self.assertEqual(stat.S_IMODE(path.stat().st_mode), 0o640)

    def test_stale_config_aborts_before_write(self):
        with tempfile.TemporaryDirectory() as directory:
            path = pathlib.Path(directory) / "config"
            before = b"theme = dark\n"
            path.write_bytes(before)
            plan = module.plan_configuration(before, "")
            path.write_bytes(b"theme = light\n")
            verified = []

            with self.assertRaisesRegex(module.MigrationError, "changed"):
                module.write_config_transactionally(
                    path,
                    plan,
                    verify=lambda: verified.append(True),
                )

            self.assertEqual(path.read_bytes(), b"theme = light\n")
            self.assertEqual(verified, [])


class LocalEnvironmentTests(unittest.TestCase):
    def test_config_candidates_follow_ghostty_macos_precedence(self):
        home = pathlib.Path("/Users/example")

        candidates = module.config_candidates(home, None)

        self.assertEqual(
            candidates,
            (
                home / ".config/ghostty/config.ghostty",
                home / ".config/ghostty/config",
                home
                / "Library/Application Support/com.mitchellh.ghostty/config.ghostty",
                home / "Library/Application Support/com.mitchellh.ghostty/config",
            ),
        )

    def test_version_reader_uses_the_fixed_app_bundle_binary(self):
        calls = []

        def fake_run(command, **kwargs):
            calls.append((command, kwargs))
            return subprocess.CompletedProcess(
                command,
                0,
                stdout="Ghostty 1.3.1\n\nVersion\n",
                stderr="",
            )

        version = module.read_ghostty_version(run=fake_run)

        self.assertEqual(version, "1.3.1")
        self.assertEqual(
            calls[0][0],
            [str(module.GHOSTTY_BINARY), "+version"],
        )
        self.assertTrue(calls[0][1]["check"])

    def test_effective_reader_and_validator_use_read_only_actions(self):
        calls = []

        def fake_run(command, **kwargs):
            calls.append(command)
            return subprocess.CompletedProcess(
                command,
                0,
                stdout="keybind = ctrl+n=new_window\n",
                stderr="",
            )

        module.validate_config(run=fake_run)
        output = module.read_effective_keybinds(run=fake_run)

        self.assertEqual(output, "keybind = ctrl+n=new_window\n")
        self.assertEqual(
            calls,
            [
                [str(module.GHOSTTY_BINARY), "+validate-config"],
                [str(module.GHOSTTY_BINARY), "+list-keybinds", "--plain"],
            ],
        )


class CommandExecutionTests(unittest.TestCase):
    @staticmethod
    def fake_runner(config_path, *, reject_validation=False):
        calls = []

        def run(command, **kwargs):
            calls.append(command)
            action = command[1]
            if action == "+version":
                return subprocess.CompletedProcess(
                    command,
                    0,
                    stdout="Ghostty 1.3.1\n",
                    stderr="",
                )
            if action == "+validate-config":
                if reject_validation:
                    raise subprocess.CalledProcessError(1, command)
                return subprocess.CompletedProcess(
                    command,
                    0,
                    stdout="",
                    stderr="",
                )
            if action == "+list-keybinds":
                content = config_path.read_text(encoding="utf-8")
                lines = ["keybind = ctrl+n=new_window"]
                if "keybind = ctrl+c=text:\\x03" in content:
                    lines.append("keybind = ctrl+c=text:\\x03")
                if "keybind = ctrl+g=text:\\x07" in content:
                    lines.append("keybind = ctrl+g=text:\\x07")
                return subprocess.CompletedProcess(
                    command,
                    0,
                    stdout="\n".join(lines) + "\n",
                    stderr="",
                )
            raise AssertionError(f"unexpected command: {command}")

        return run, calls

    def test_preflight_is_read_only_and_reports_the_managed_file(self):
        with tempfile.TemporaryDirectory() as directory:
            home = pathlib.Path(directory)
            config = home / ".config/ghostty/config"
            config.parent.mkdir(parents=True)
            before = b"keybind = ctrl+n=new_window\n"
            config.write_bytes(before)
            run, calls = self.fake_runner(config)
            output = io.StringIO()

            module.run_command(
                module.build_parser().parse_args(["preflight"]),
                run=run,
                home=home,
                xdg_config_home=None,
                backup_root=home / "history",
                output=output,
            )

            self.assertEqual(config.read_bytes(), before)
            self.assertFalse((home / "history").exists())
            self.assertIn(str(config), output.getvalue())
            self.assertIn("ctrl+c -> text:\\x03", output.getvalue())
            self.assertIn("PREFLIGHT OK", output.getvalue())
            self.assertIn(
                [str(module.GHOSTTY_BINARY), "+list-keybinds", "--plain"],
                calls,
            )

    def test_apply_writes_history_before_config_and_preserves_existing_keys(self):
        with tempfile.TemporaryDirectory() as directory:
            home = pathlib.Path(directory)
            root = home / "history"
            config = home / ".config/ghostty/config"
            config.parent.mkdir(parents=True)
            config.write_text("keybind = ctrl+n=new_window\n", encoding="utf-8")
            run, _ = self.fake_runner(config)
            output = io.StringIO()

            module.run_command(
                module.build_parser().parse_args(["apply"]),
                run=run,
                home=home,
                xdg_config_home=None,
                backup_root=root,
                output=output,
            )

            history_path = next(root.glob("*/setting_history.json"))
            content = config.read_text(encoding="utf-8")
            self.assertIn("keybind = ctrl+n=new_window", content)
            self.assertIn("keybind = ctrl+c=text:\\x03", content)
            self.assertIn("keybind = ctrl+g=text:\\x07", content)
            self.assertIn(str(history_path), output.getvalue())
            self.assertIn("APPLIED", output.getvalue())

    def test_restore_keeps_later_unrelated_config_edits(self):
        with tempfile.TemporaryDirectory() as directory:
            home = pathlib.Path(directory)
            root = home / "history"
            config = home / ".config/ghostty/config"
            config.parent.mkdir(parents=True)
            config.write_text("keybind = ctrl+n=new_window\n", encoding="utf-8")
            run, _ = self.fake_runner(config)
            module.run_command(
                module.build_parser().parse_args(["apply"]),
                run=run,
                home=home,
                xdg_config_home=None,
                backup_root=root,
                output=io.StringIO(),
            )
            history_path = next(root.glob("*/setting_history.json"))
            with config.open("ab") as handle:
                handle.write(b"font-size = 13\n")

            output = io.StringIO()
            module.run_command(
                module.build_parser().parse_args(
                    ["restore", "--history", str(history_path)]
                ),
                run=run,
                home=home,
                xdg_config_home=None,
                backup_root=root,
                output=output,
            )

            self.assertEqual(
                config.read_bytes(),
                b"keybind = ctrl+n=new_window\nfont-size = 13\n",
            )
            self.assertIn("RESTORED", output.getvalue())

    def test_verify_rejects_a_missing_target_without_writing(self):
        with tempfile.TemporaryDirectory() as directory:
            home = pathlib.Path(directory)
            config = home / ".config/ghostty/config"
            config.parent.mkdir(parents=True)
            before = b"keybind = ctrl+n=new_window\n"
            config.write_bytes(before)
            run, _ = self.fake_runner(config)

            with self.assertRaisesRegex(module.MigrationError, "incomplete"):
                module.run_command(
                    module.build_parser().parse_args(["verify"]),
                    run=run,
                    home=home,
                    xdg_config_home=None,
                    output=io.StringIO(),
                )

            self.assertEqual(config.read_bytes(), before)


if __name__ == "__main__":
    unittest.main()
