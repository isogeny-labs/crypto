"""Adversarial input controls for the frozen K13 wall analyzer."""

import unittest
from unittest.mock import patch

import analyze


def panel():
    records = []
    for target_index in range(analyze.TARGETS):
        workload = {
            "curve_spec": analyze.EXPECTED_CURVE,
            "curve": {"curve_uid": analyze.EXPECTED_CURVE_UID, "registered": True},
            "target_law": "hash_to_subgroup_v1",
            "target_seed": analyze.TARGET_SEED,
            "target_index": target_index,
            "planted": None,
            "workload_id": f"W{target_index}",
            "workload_sha256": f"{target_index:064x}",
            "target": ["0x1", "0x2"],
            "target_counter": 0,
        }
        for round_no in range(analyze.WARMUPS + analyze.ROUNDS):
            for arm in analyze.ARMS:
                wall = 100 if arm in ("batch-m3", "batch-m3-aa") else 1000
                records.append({
                    "record_id": f"{target_index}-{round_no}-{arm}",
                    "arm": arm,
                    "workload": workload,
                    "round": round_no,
                    "warmup": round_no == 0,
                    "outcome": {"status": "verified", "matches_target": True},
                    "time": {"process_wall_ns": wall},
                    "isolation": {"level": "L2", "node": 0},
                    "phases": [{"name": name, "wall_ns": 1}
                               for name in analyze.EXPECTED_PHASES],
                    "method": {"family": "ic"},
                    "binary_sha256": "a" * 64,
                    "env_class_id": "one-host",
                    "algorithm_seed": round_no + 1,
                    "session_id": "one-session",
                })
    work = [{"record_id": record["record_id"], "work": {"Err": "unpriced"}}
            for record in records]
    return records, work


class AnalyzeAdmissionTest(unittest.TestCase):
    def test_missing_work_cannot_pass_wall_rule(self):
        records, _ = panel()
        result = analyze.analyze(records, [])
        self.assertFalse(result["work_records_complete"])
        self.assertFalse(result["panel_complete"])
        self.assertFalse(result["all_four_f456_rules_met"])
        self.assertEqual(len(result["missing_work_records"]), 560)

    def test_complete_work_errors_preserve_wall_comparison(self):
        records, work = panel()
        result = analyze.analyze(records, work)
        self.assertTrue(result["work_records_complete"])
        self.assertTrue(result["panel_complete"])
        self.assertTrue(result["all_four_f456_rules_met"])
        self.assertEqual(result["work_complete_count_by_arm"], {})
        self.assertEqual(result["nodes"], [0])

    def test_extra_and_duplicate_work_rows_fail_closed(self):
        records, work = panel()
        extra = {"record_id": "outside-panel", "work": {"Err": "unpriced"}}
        extra_result = analyze.analyze(records, work + [extra])
        self.assertFalse(extra_result["panel_complete"])
        self.assertEqual(extra_result["extra_work_records"], ["outside-panel"])
        duplicate_result = analyze.analyze(records, work + [work[0]])
        self.assertFalse(duplicate_result["panel_complete"])
        self.assertIn("duplicate_work_row", [reason
            for row in duplicate_result["invalid_rows"] for reason in row["reasons"]])

    def test_every_interval_uses_frozen_seed(self):
        records, work = panel()
        seeds = []

        def interval_control(_targets, lower, _upper, seed):
            seeds.append(seed)
            return [0.95, 1.05] if lower == 0.025 else [0.09, 0.11]

        with patch.object(analyze, "interval", side_effect=interval_control):
            result = analyze.analyze(records, work)
        self.assertTrue(result["all_four_f456_rules_met"])
        self.assertEqual(seeds, [analyze.SEED] * (1 + len(analyze.COMPARISONS)))


if __name__ == "__main__":
    unittest.main()
