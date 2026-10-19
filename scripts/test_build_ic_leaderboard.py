#!/usr/bin/env python3
"""Focused tests for the leaderboard's IC curve-record reader."""
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import build_ic_leaderboard as builder  # noqa: E402


class CurveRecords(unittest.TestCase):
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


if __name__ == "__main__":
    unittest.main()
