import copy
import io
import json
import os
import pathlib
import plistlib
import stat
import tempfile
import types
import unittest

from scripts import iterm2_korean_control_keys as module


FIXTURE = pathlib.Path(__file__).parent / "fixtures" / "iterm2-global-key-map.json"


def load_fixture():
    return json.loads(FIXTURE.read_text(encoding="utf-8"))


def sample_snapshot(*, persisted=False, persisted_value=None):
    return module.PreferenceSnapshot(
        iterm_version="3.6.11",
        global_map=load_fixture(),
        language_agnostic_effective=False,
        language_agnostic_persisted=persisted,
        language_agnostic_persisted_value=persisted_value,
        profile_maps={"Default [default-guid]": {}, "tmux [tmux-guid]": {}},
    )


class FakePreferenceClient:
    def __init__(self, snapshot=None, *, fail_on_write=None):
        snapshot = snapshot or sample_snapshot()
        self.values = {"GlobalKeyMap": copy.deepcopy(snapshot.global_map)}
        if snapshot.language_agnostic_persisted:
            self.values["LanguageAgnosticKeyBindings"] = (
                snapshot.language_agnostic_persisted_value
            )
        self._profile_maps = copy.deepcopy(snapshot.profile_maps)
        self.fail_on_write = fail_on_write
        self.write_attempts = 0
        self.writes = []

    async def get_preference(self, key):
        if key == "LanguageAgnosticKeyBindings":
            return copy.deepcopy(self.values.get(key, False))
        if key == "LoadPrefsFromCustomFolder":
            return copy.deepcopy(self.values.get(key, False))
        return copy.deepcopy(self.values[key])

    async def set_preference(self, key, value):
        self.write_attempts += 1
        if self.write_attempts == self.fail_on_write:
            self.fail_on_write = None
            raise RuntimeError(f"injected write failure {self.write_attempts}")
        self.writes.append((key, copy.deepcopy(value)))
        if value is None:
            self.values.pop(key, None)
        else:
            self.values[key] = copy.deepcopy(value)

    async def profile_maps(self):
        return copy.deepcopy(self._profile_maps)

    async def snapshot(self):
        return module.PreferenceSnapshot(
            iterm_version="3.6.11",
            global_map=copy.deepcopy(self.values["GlobalKeyMap"]),
            language_agnostic_effective=bool(
                self.values.get("LanguageAgnosticKeyBindings", False)
            ),
            language_agnostic_persisted=(
                "LanguageAgnosticKeyBindings" in self.values
            ),
            language_agnostic_persisted_value=self.values.get(
                "LanguageAgnosticKeyBindings"
            ),
            profile_maps=copy.deepcopy(self._profile_maps),
        )


def receipt_for(snapshot, plan):
    return module.Receipt(
        schema_version=1,
        iterm_version=snapshot.iterm_version,
        created_at="2026-08-05T00:00:00Z",
        before_hash=module.canonical_hash(snapshot.global_map),
        after_hash=module.canonical_hash(plan.after),
        original_language_agnostic_persisted=(
            snapshot.language_agnostic_persisted
        ),
        original_language_agnostic_value=(
            snapshot.language_agnostic_persisted_value
        ),
        owned_entries=copy.deepcopy(plan.owned_additions),
        result="prepared",
    )


class KeymapPlannerTests(unittest.TestCase):
    def test_plan_adds_only_c_and_g_and_preserves_unknown_fields(self):
        before = load_fixture()

        plan = module.plan_keymap(before, {"Default": {}, "tmux": {}})

        self.assertEqual(
            set(plan.owned_additions),
            {"0x63-0x40000-0x8", "0x67-0x40000-0x5"},
        )
        for key, value in before.items():
            self.assertEqual(plan.after[key], value)
        self.assertEqual(
            plan.after["0x63-0x40000-0x8"], {"Action": 11, "Text": "0x03"}
        )
        self.assertEqual(
            plan.after["0x67-0x40000-0x5"], {"Action": 11, "Text": "0x07"}
        )
        self.assertEqual(before, load_fixture())

    def test_incompatible_global_forms_fail_closed(self):
        cases = {
            "portable": "0x314a-0x40000-0x8",
            "legacy_unmodified": "0x63-0x40000",
            "legacy_control": "0x3-0x40000",
            "modified": ":0x3:0x40000",
        }

        for label, key in cases.items():
            with self.subTest(label=label):
                with self.assertRaisesRegex(module.MigrationError, "global"):
                    module.plan_keymap(
                        {key: {"Action": 12, "Text": "wrong"}}, {}
                    )

    def test_incompatible_profile_mapping_names_the_profile(self):
        with self.assertRaisesRegex(module.MigrationError, "tmux"):
            module.plan_keymap(
                {},
                {
                    "tmux": {
                        "0x67-0x40000-0x5": {"Action": 12, "Text": "wrong"}
                    }
                },
            )

    def test_equivalent_portable_binding_is_idempotent(self):
        existing = {
            "0x314a-0x40000-0x8": {"Action": 11, "Text": "0x03"},
            "0x67-0x40000-0x5": {"Action": 11, "Text": "0x07"},
        }

        plan = module.plan_keymap(existing, {})

        self.assertEqual(plan.after, existing)
        self.assertEqual(plan.owned_additions, {})
        self.assertEqual(plan.already_satisfied, ("Control-C", "Control-G"))

    def test_compatible_legacy_binding_does_not_replace_physical_mapping(self):
        legacy = {"0x63-0x40000": {"Action": 11, "Text": "0x03"}}

        plan = module.plan_keymap(legacy, {})

        self.assertEqual(plan.after["0x63-0x40000"], legacy["0x63-0x40000"])
        self.assertEqual(
            plan.owned_additions["0x63-0x40000-0x8"],
            {"Action": 11, "Text": "0x03"},
        )

    def test_malformed_serialization_raises_migration_error(self):
        with self.assertRaisesRegex(
            module.MigrationError, "Unsupported iTerm2 key serialization"
        ):
            module.plan_keymap({"not-a-key": {"Action": 0, "Text": ""}}, {})

    def test_target_action_must_be_a_dictionary(self):
        with self.assertRaisesRegex(module.MigrationError, "action dictionary"):
            module.plan_keymap({"0x63-0x40000-0x8": "not-a-dict"}, {})

    def test_canonical_hash_ignores_dictionary_order(self):
        first = {"b": 2, "a": {"d": 4, "c": 3}}
        second = {"a": {"c": 3, "d": 4}, "b": 2}

        self.assertEqual(module.canonical_hash(first), module.canonical_hash(second))


class BackupAndReceiptTests(unittest.TestCase):
    def test_backup_is_private_and_records_exact_hashes(self):
        snapshot = sample_snapshot()
        plan = module.plan_keymap(snapshot.global_map, snapshot.profile_maps)
        preference_export = b"bplist00fixture"

        with tempfile.TemporaryDirectory() as directory:
            receipt_path = module.create_backup(
                snapshot,
                plan,
                pathlib.Path(directory) / "backups",
                preference_export=preference_export,
            )

            self.assertEqual(
                stat.S_IMODE(receipt_path.parent.stat().st_mode), 0o700
            )
            for path in receipt_path.parent.iterdir():
                self.assertEqual(stat.S_IMODE(path.stat().st_mode), 0o600)
            receipt = json.loads(receipt_path.read_text(encoding="utf-8"))
            self.assertEqual(
                receipt["before_hash"], module.canonical_hash(snapshot.global_map)
            )
            self.assertEqual(receipt["after_hash"], module.canonical_hash(plan.after))
            self.assertFalse(receipt["original_language_agnostic_persisted"])
            self.assertIsNone(receipt["original_language_agnostic_value"])
            self.assertEqual(receipt["result"], "prepared")
            self.assertEqual(
                (receipt_path.parent / "preferences.plist").read_bytes(),
                preference_export,
            )

    def test_explicit_false_is_distinct_from_absent_preference(self):
        absent = plistlib.dumps({"GlobalKeyMap": {}})
        explicit_false = plistlib.dumps(
            {"GlobalKeyMap": {}, "LanguageAgnosticKeyBindings": False}
        )

        self.assertEqual(
            module.language_agnostic_persistence(absent), (False, None)
        )
        self.assertEqual(
            module.language_agnostic_persistence(explicit_false), (True, False)
        )

    def test_receipt_loader_rejects_outside_path_and_symlink(self):
        snapshot = sample_snapshot()
        plan = module.plan_keymap(snapshot.global_map, snapshot.profile_maps)

        with tempfile.TemporaryDirectory() as directory:
            base = pathlib.Path(directory)
            root = base / "backups"
            receipt_path = module.create_backup(
                snapshot, plan, root, preference_export=b"plist"
            )
            outside = base / "outside.json"
            outside.write_text(receipt_path.read_text(encoding="utf-8"))
            with self.assertRaisesRegex(module.MigrationError, "backup root"):
                module.load_receipt(outside, root)

            link = receipt_path.parent / "linked-receipt.json"
            os.symlink(outside, link)
            with self.assertRaisesRegex(module.MigrationError, "symlink"):
                module.load_receipt(link, root)

    def test_receipt_loader_rejects_unapproved_owned_entry(self):
        snapshot = sample_snapshot()
        plan = module.plan_keymap(snapshot.global_map, snapshot.profile_maps)

        with tempfile.TemporaryDirectory() as directory:
            root = pathlib.Path(directory) / "backups"
            receipt_path = module.create_backup(
                snapshot, plan, root, preference_export=b"plist"
            )
            receipt = json.loads(receipt_path.read_text(encoding="utf-8"))
            receipt["owned_entries"]["0x61-0x0"] = {
                "Action": 11,
                "Text": "0x01",
            }
            receipt_path.write_text(json.dumps(receipt), encoding="utf-8")

            with self.assertRaisesRegex(module.MigrationError, "owned entries"):
                module.load_receipt(receipt_path, root)

    def test_receipt_loader_rejects_boolean_schema_version(self):
        snapshot = sample_snapshot()
        plan = module.plan_keymap(snapshot.global_map, snapshot.profile_maps)

        with tempfile.TemporaryDirectory() as directory:
            root = pathlib.Path(directory) / "backups"
            receipt_path = module.create_backup(
                snapshot, plan, root, preference_export=b"plist"
            )
            receipt = json.loads(receipt_path.read_text(encoding="utf-8"))
            receipt["schema_version"] = True
            receipt_path.write_text(json.dumps(receipt), encoding="utf-8")

            with self.assertRaisesRegex(module.MigrationError, "schema"):
                module.load_receipt(receipt_path, root)


class PreferenceMutationTests(unittest.IsolatedAsyncioTestCase):
    async def test_apply_sets_exact_map_and_language_preference(self):
        client = FakePreferenceClient()
        snapshot = await client.snapshot()
        plan = module.plan_keymap(snapshot.global_map, snapshot.profile_maps)

        await module.apply_configuration(client, snapshot, plan)

        self.assertEqual(client.values["GlobalKeyMap"], plan.after)
        self.assertIs(client.values["LanguageAgnosticKeyBindings"], True)

    async def test_second_write_failure_restores_map_and_absent_flag(self):
        client = FakePreferenceClient(fail_on_write=2)
        snapshot = await client.snapshot()
        plan = module.plan_keymap(snapshot.global_map, snapshot.profile_maps)

        with self.assertRaisesRegex(module.MigrationError, "restored"):
            await module.apply_configuration(client, snapshot, plan)

        self.assertEqual(client.values["GlobalKeyMap"], snapshot.global_map)
        self.assertNotIn("LanguageAgnosticKeyBindings", client.values)

    async def test_stale_map_aborts_before_first_write(self):
        client = FakePreferenceClient()
        snapshot = await client.snapshot()
        plan = module.plan_keymap(snapshot.global_map, snapshot.profile_maps)
        client.values["GlobalKeyMap"]["0x61-0x100000"] = {
            "Action": 0,
            "Text": "",
        }

        with self.assertRaisesRegex(module.MigrationError, "changed after preflight"):
            await module.apply_configuration(client, snapshot, plan)

        self.assertEqual(client.writes, [])

    async def test_restore_removes_owned_entries_and_restores_absent_flag(self):
        snapshot = sample_snapshot()
        plan = module.plan_keymap(snapshot.global_map, snapshot.profile_maps)
        client = FakePreferenceClient(snapshot)
        client.values["GlobalKeyMap"] = copy.deepcopy(plan.after)
        client.values["LanguageAgnosticKeyBindings"] = True

        result = await module.restore_configuration(
            client, receipt_for(snapshot, plan)
        )

        self.assertEqual(client.values["GlobalKeyMap"], snapshot.global_map)
        self.assertNotIn("LanguageAgnosticKeyBindings", client.values)
        self.assertTrue(result.language_agnostic_restored)
        self.assertIsNone(result.warning)

    async def test_restore_refuses_edited_owned_entry_before_writing(self):
        snapshot = sample_snapshot()
        plan = module.plan_keymap(snapshot.global_map, snapshot.profile_maps)
        client = FakePreferenceClient(snapshot)
        client.values["GlobalKeyMap"] = copy.deepcopy(plan.after)
        client.values["GlobalKeyMap"]["0x63-0x40000-0x8"]["Text"] = "0x04"
        client.values["LanguageAgnosticKeyBindings"] = True

        with self.assertRaisesRegex(module.MigrationError, "owned entry changed"):
            await module.restore_configuration(client, receipt_for(snapshot, plan))

        self.assertEqual(client.writes, [])

    async def test_restore_keeps_flag_when_unrelated_mapping_changed(self):
        snapshot = sample_snapshot()
        plan = module.plan_keymap(snapshot.global_map, snapshot.profile_maps)
        client = FakePreferenceClient(snapshot)
        client.values["GlobalKeyMap"] = copy.deepcopy(plan.after)
        client.values["GlobalKeyMap"]["0x61-0x100000"] = {
            "Action": 0,
            "Text": "",
        }
        client.values["LanguageAgnosticKeyBindings"] = True

        result = await module.restore_configuration(
            client, receipt_for(snapshot, plan)
        )

        self.assertNotIn("0x63-0x40000-0x8", client.values["GlobalKeyMap"])
        self.assertNotIn("0x67-0x40000-0x5", client.values["GlobalKeyMap"])
        self.assertIs(client.values["LanguageAgnosticKeyBindings"], True)
        self.assertFalse(result.language_agnostic_restored)
        self.assertIn("unrelated mappings changed", result.warning)


class LiveAdapterTests(unittest.IsolatedAsyncioTestCase):
    async def test_raw_rpc_read_generic_write_and_profile_copy(self):
        calls = []

        class FakeRPC:
            @staticmethod
            async def async_get_preference(connection, key):
                calls.append(("get", connection, key))
                result = types.SimpleNamespace(
                    get_preference_result=types.SimpleNamespace(
                        json_value=json.dumps({"kept": {"Version": 2}})
                    )
                )
                return types.SimpleNamespace(
                    preferences_response=types.SimpleNamespace(results=[result])
                )

        profile_map = {"0x61-0x0": {"Action": 0}}

        class FakePartialProfile:
            @staticmethod
            async def async_query(connection, *, properties):
                calls.append(("profiles", connection, properties))
                return [
                    types.SimpleNamespace(
                        name="Default",
                        guid="default-guid",
                        key_mappings=profile_map,
                    )
                ]

        async def fake_set(connection, key, value):
            calls.append(("set", connection, key, copy.deepcopy(value)))

        iterm2_module = types.SimpleNamespace(
            rpc=FakeRPC,
            PartialProfile=FakePartialProfile,
            async_set_preference=fake_set,
        )
        client = module.ItermPreferenceClient(iterm2_module, "connection")

        value = await client.get_preference("GlobalKeyMap")
        await client.set_preference("GlobalKeyMap", {"new": True})
        profiles = await client.profile_maps()
        profile_map["mutated-after-copy"] = {}

        self.assertEqual(value, {"kept": {"Version": 2}})
        self.assertEqual(
            profiles,
            {"Default [default-guid]": {"0x61-0x0": {"Action": 0}}},
        )
        self.assertIn(
            ("set", "connection", "GlobalKeyMap", {"new": True}), calls
        )
        self.assertIn(
            (
                "profiles",
                "connection",
                ["Guid", "Name", "Keyboard Map"],
            ),
            calls,
        )

    async def test_snapshot_rejects_custom_preference_storage(self):
        client = FakePreferenceClient()
        client.values["LoadPrefsFromCustomFolder"] = True

        with self.assertRaisesRegex(module.MigrationError, "custom"):
            await module.build_snapshot(
                client,
                "3.6.11",
                plistlib.dumps({"GlobalKeyMap": load_fixture()}),
            )

    async def test_snapshot_rejects_export_and_api_map_mismatch(self):
        client = FakePreferenceClient()
        exported_map = load_fixture()
        exported_map["0x61-0x0"] = {"Action": 0, "Text": ""}

        with self.assertRaisesRegex(module.MigrationError, "export.*API"):
            await module.build_snapshot(
                client,
                "3.6.11",
                plistlib.dumps({"GlobalKeyMap": exported_map}),
            )


class CommandParserTests(unittest.TestCase):
    def test_accepts_only_narrow_command_surface(self):
        self.assertEqual(module.build_parser().parse_args(["preflight"]).command,
                         "preflight")
        self.assertEqual(module.build_parser().parse_args(["apply"]).command,
                         "apply")
        self.assertEqual(module.build_parser().parse_args(["verify"]).command,
                         "verify")
        parsed = module.build_parser().parse_args(
            ["restore", "--receipt", "/private/tmp/receipt.json"]
        )
        self.assertEqual(parsed.command, "restore")
        self.assertEqual(parsed.receipt, pathlib.Path("/private/tmp/receipt.json"))

    def test_restore_requires_an_absolute_receipt_path(self):
        with self.assertRaisesRegex(
            module.MigrationError, "absolute receipt path"
        ):
            module.absolute_path("relative/receipt.json")


class CommandExecutionTests(unittest.IsolatedAsyncioTestCase):
    @staticmethod
    def fake_iterm2(events):
        class FakeTransaction:
            def __init__(self, connection):
                self.connection = connection

            async def __aenter__(self):
                events.append(("transaction-enter", self.connection))

            async def __aexit__(self, exc_type, exc, traceback):
                events.append(("transaction-exit", self.connection))

        return types.SimpleNamespace(Transaction=FakeTransaction)

    async def test_preflight_and_verify_are_read_only(self):
        export = plistlib.dumps({"GlobalKeyMap": load_fixture()})
        events = []
        client = FakePreferenceClient()
        preflight_output = io.StringIO()

        await module.run_command(
            module.build_parser().parse_args(["preflight"]),
            client,
            self.fake_iterm2(events),
            "connection",
            iterm_version="3.6.11",
            preference_export=export,
            output=preflight_output,
        )

        self.assertEqual(client.writes, [])
        self.assertEqual(events, [])
        self.assertIn("0x63-0x40000-0x8", preflight_output.getvalue())

        snapshot = await client.snapshot()
        plan = module.plan_keymap(snapshot.global_map, snapshot.profile_maps)
        client.values["GlobalKeyMap"] = copy.deepcopy(plan.after)
        client.values["LanguageAgnosticKeyBindings"] = True
        configured_export = plistlib.dumps(
            {
                "GlobalKeyMap": plan.after,
                "LanguageAgnosticKeyBindings": True,
            }
        )
        verify_output = io.StringIO()

        await module.run_command(
            module.build_parser().parse_args(["verify"]),
            client,
            self.fake_iterm2(events),
            "connection",
            iterm_version="3.6.11",
            preference_export=configured_export,
            output=verify_output,
        )

        self.assertEqual(client.writes, [])
        self.assertEqual(events, [])
        self.assertIn("VERIFIED", verify_output.getvalue())

    async def test_apply_creates_backup_before_transactional_writes(self):
        export = plistlib.dumps({"GlobalKeyMap": load_fixture()})
        events = []

        with tempfile.TemporaryDirectory() as directory:
            root = pathlib.Path(directory) / "backups"

            class BackupAwareClient(FakePreferenceClient):
                async def set_preference(self, key, value):
                    self.assert_backup_exists()
                    await super().set_preference(key, value)

                @staticmethod
                def assert_backup_exists():
                    receipts = list(root.glob("*/receipt.json"))
                    if len(receipts) != 1:
                        raise AssertionError("backup was not closed before write")

            client = BackupAwareClient()
            output = io.StringIO()
            await module.run_command(
                module.build_parser().parse_args(["apply"]),
                client,
                self.fake_iterm2(events),
                "connection",
                iterm_version="3.6.11",
                preference_export=export,
                backup_root=root,
                output=output,
            )

            receipt_path = next(root.glob("*/receipt.json"))
            self.assertEqual(client.values["GlobalKeyMap"],
                             module.plan_keymap(load_fixture(), {}).after)
            self.assertIs(client.values["LanguageAgnosticKeyBindings"], True)
            self.assertEqual(
                events,
                [
                    ("transaction-enter", "connection"),
                    ("transaction-exit", "connection"),
                ],
            )
            self.assertIn(str(receipt_path), output.getvalue())
            self.assertIn("APPLIED", output.getvalue())


class LocalEnvironmentTests(unittest.TestCase):
    def test_version_reader_uses_fixed_application_plist(self):
        calls = []

        def fake_run(command, **kwargs):
            calls.append((command, kwargs))
            return types.SimpleNamespace(stdout="3.6.11\n")

        self.assertEqual(module.read_iterm_version(run=fake_run), "3.6.11")
        self.assertEqual(
            calls[0][0],
            [
                "/usr/libexec/PlistBuddy",
                "-c",
                "Print :CFBundleShortVersionString",
                "/Applications/iTerm.app/Contents/Info.plist",
            ],
        )


if __name__ == "__main__":
    unittest.main()
