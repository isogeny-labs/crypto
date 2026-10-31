#!/usr/bin/env python3
"""Focused tests for the leaderboard's IC curve-record reader."""
import copy
import sys
import unittest
from pathlib import Path
from unittest.mock import patch

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


class FrozenEngineeringComponent(unittest.TestCase):
    def with_summary(self, change):
        original_load = builder.load

        def load_with_change(rel):
            value = original_load(rel)
            if rel == builder.SOURCES["f4_fp_ic_effect"]:
                change(value)
            return value

        return patch.object(builder, "load", side_effect=load_with_change)

    def test_four_frozen_rows_match_raw_dlp_arithmetic(self):
        study = builder.f4_fp_ic_component()
        self.assertEqual({(r["p"], r["seed"]) for r in study["dlp"]},
                         {(53, 1), (53, 2), (101, 1), (101, 2)})

    def test_f4_fp_result_is_rendered_with_its_engineering_scope(self):
        data = builder.build()
        study = data["f4_fp_ic_component"]
        self.assertEqual(study["class"], "engineering")
        self.assertEqual(len(study["dlp"]), 4)
        for row in study["dlp"]:
            self.assertEqual(row["solved"], [True, True])
            self.assertEqual(row["correct"], [True, True])
        md = builder.markdown(data)
        html = builder.page(data, standalone=True)
        self.assertIn("## F_p Gröbner engine (`f4_fp`)", md)
        self.assertIn("S/rho is a lower-bound comparison", md)
        self.assertIn("no recovered-scalar certificate for an independent replay", md)
        self.assertIn('id="f4-fp-ic-component"', html)
        self.assertIn("Frozen engineering summary", html)
        self.assertIn("no recovered-scalar certificate for an independent replay", html)

    def test_incorrect_frozen_result_fails_closed(self):
        def corrupt(study):
            study["dlp"][0]["correct"] = [True, False]

        with self.with_summary(corrupt):
            with self.assertRaisesRegex(SystemExit, "incomplete or inconsistent"):
                builder.f4_fp_ic_component()

    def test_corrupt_frozen_ratio_fails_closed(self):
        def corrupt(study):
            study["dlp"][0]["s_ratio"] += 1

        with self.with_summary(corrupt):
            with self.assertRaisesRegex(SystemExit, "incomplete or inconsistent"):
                builder.f4_fp_ic_component()

    def test_self_consistent_but_false_s_values_fail_against_raw_rows(self):
        def corrupt(study):
            row = study["dlp"][0]
            row["s_before"] *= 2
            row["s_ratio"] = row["s_before"] / row["s_after"]
            row["s_over_rho_before"] = row["s_before"] / row["rho_s_mean"]

        with self.with_summary(corrupt):
            with self.assertRaisesRegex(SystemExit, "incomplete or inconsistent"):
                builder.f4_fp_ic_component()

    def test_false_rho_mean_fails_against_four_raw_runs(self):
        def corrupt(study):
            row = study["dlp"][0]
            row["rho_s_mean"] *= 2
            row["s_over_rho_before"] = row["s_before"] / row["rho_s_mean"]
            row["s_over_rho_after"] = row["s_after"] / row["rho_s_mean"]

        with self.with_summary(corrupt):
            with self.assertRaisesRegex(SystemExit, "incomplete or inconsistent"):
                builder.f4_fp_ic_component()

    def test_changed_raw_multiplication_total_fails_closed(self):
        original_load = builder.load

        def corrupt_raw(rel):
            value = original_load(rel)
            if rel == builder.SOURCES["f4_fp_ic_main"]:
                value[0]["total_muls"] *= 2
            return value

        with patch.object(builder, "load", side_effect=corrupt_raw):
            with self.assertRaisesRegex(SystemExit, "incomplete or inconsistent"):
                builder.f4_fp_ic_component()

    def test_raw_phase_counter_must_sum_to_recorded_total(self):
        original_load = builder.load

        def corrupt_raw(rel):
            value = original_load(rel)
            if rel == builder.SOURCES["f4_fp_ic_new"]:
                value[0]["f4_muls"] += 1
            return value

        with patch.object(builder, "load", side_effect=corrupt_raw):
            with self.assertRaisesRegex(SystemExit, "incomplete or inconsistent"):
                builder.f4_fp_ic_component()


if __name__ == "__main__":
    unittest.main()
