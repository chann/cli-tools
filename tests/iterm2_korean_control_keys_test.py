import json
import pathlib
import unittest

from scripts import iterm2_korean_control_keys as module


FIXTURE = pathlib.Path(__file__).parent / "fixtures" / "iterm2-global-key-map.json"


def load_fixture():
    return json.loads(FIXTURE.read_text(encoding="utf-8"))


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


if __name__ == "__main__":
    unittest.main()
