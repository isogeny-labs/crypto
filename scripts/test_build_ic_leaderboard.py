#!/usr/bin/env python3
"""Focused tests for the leaderboard's IC curve-record reader."""
import copy
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import build_ic_leaderboard as builder  # noqa: E402


class CurveRecords(unittest.TestCase):
    def frozen_metadata_fixture(self):
        historical = {
            "slug": "curve", "source": "receipt.json", "measurement": 5,
            "curve_record": builder.curve_record({}, "curve"),
            "factor_base": {"points": 10, "refs": [], "link_status": "no_curves_yaml_record"},
        }
        current = copy.deepcopy(historical)
        current["curve_record"]["curves_yaml_key"] = "registered_curve"
        current["factor_base"]["refs"] = ["current_registry_reference"]
        pin = {builder.gate.board_key(historical): builder.gate.row_digest(builder.stable(historical))}
        return historical, current, pin

    def test_registry_refresh_preserves_exact_frozen_measurement_payload(self):
        historical, current, pin = self.frozen_metadata_fixture()
        builder.preserve_frozen_curve_metadata([current], pin)
        self.assertEqual(current, historical)

    def test_changed_measurement_cannot_use_metadata_compatibility(self):
        _, current, pin = self.frozen_metadata_fixture()
        current["measurement"] = 6
        before = copy.deepcopy(current)
        builder.preserve_frozen_curve_metadata([current], pin)
        self.assertEqual(current, before)
        self.assertNotEqual(builder.gate.row_digest(builder.stable(current)), next(iter(pin.values())))

    def test_unfrozen_row_keeps_current_registry_metadata(self):
        _, current, _ = self.frozen_metadata_fixture()
        before = copy.deepcopy(current)
        builder.preserve_frozen_curve_metadata([current], {})
        self.assertEqual(current, before)

    def test_real_icv1_identity_field_with_digit_is_read(self):
        lines = """schema_version: 1
curves:
  rec_a:
    icv1_identity:
      slug: icv1-f2m47-t22705043-f4e44623
    factor_base_refs: []
    factor_base_link_status: not_reconciled
""".splitlines()
        records, count = builder.parse_curve_records(lines)
        self.assertEqual(count, 1)
        self.assertEqual(records["icv1-f2m47-t22705043-f4e44623"]["key"], "rec_a")

    def test_null_identity_is_not_a_registry_slug(self):
        lines = """schema_version: 1
curves:
  unregistered:
    icv1_identity:
      slug: null
    factor_base_refs: []
    factor_base_link_status: not_reconciled
""".splitlines()
        records, count = builder.parse_curve_records(lines)
        self.assertEqual(count, 1)
        self.assertEqual(records, {})

    def test_duplicate_slug_fails_closed(self):
        lines = """schema_version: 1
curves:
  first:
    icv1_identity:
      slug: icv1-f2m47-t22705043-f4e44623
    factor_base_refs: []
    factor_base_link_status: not_reconciled
  second:
    icv1_identity:
      slug: icv1-f2m47-t22705043-f4e44623
    factor_base_refs: []
    factor_base_link_status: not_reconciled
""".splitlines()
        with self.assertRaisesRegex(SystemExit, "duplicate ICV1 slug"):
            builder.parse_curve_records(lines)

    def test_malformed_factor_base_fields_fail_closed(self):
        lines = """schema_version: 1
curves:
  rec_a:
    icv1_identity:
      slug: icv1-f2m47-t22705043-f4e44623
    factor_base_refs: malformed
    factor_base_link_status: not_reconciled
""".splitlines()
        with self.assertRaisesRegex(SystemExit, "unreadable factor_base_refs"):
            builder.parse_curve_records(lines)

    def test_all_24_board_curves_resolve(self):
        data = builder.build()
        self.assertEqual(data["curve_records"]["board_curves"], 24)
        self.assertEqual(data["curve_records"]["board_curves_in_yaml"], 24)
        records = data["curve_records"]["current_board_records"]
        self.assertEqual(len(records), 24)
        self.assertTrue(all(record["curves_yaml_key"] for record in records.values()))


if __name__ == "__main__":
    unittest.main()
