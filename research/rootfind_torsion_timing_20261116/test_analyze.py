import copy
import unittest

import analyze


def fixture():
    records = []
    seq = 0
    for target in analyze.TARGETS:
        for round_id in range(9):
            for arm in analyze.ARMS:
                seq += 1
                process = {"f6-ic": 500, "harvest-m3": 200,
                           "torsion-m3": 160, "torsion-m3-aa": 160}[arm]
                records.append({
                    "seq": seq, "arm": arm, "round": round_id, "warmup": round_id == 0,
                    "algorithm_seed": str(1000 + target * 10 + round_id),
                    "binary_sha256": "f" * 64,
                    "method": {"params": {"oracle": analyze.ORACLES[arm]}},
                    "factor_base": {"fb_sha256": analyze.EXPECTED_BASE_SHA},
                    "workload": {
                        "target_index": target, "target_seed": 202611151301,
                        "target": [hex(target + 1), hex(target + 2)],
                        "workload_sha256": str(target),
                        "curve_spec": {"kind": "koblitz", "a": 0, "n": 13},
                        "curve": {"curve_uid": analyze.EXPECTED_CURVE_UID,
                                  "r": 2003, "cofactor": 4, "registered": True},
                    },
                    "outcome": {"status": "verified", "matches_target": True},
                    "isolation": {"level": "L0"},
                    "phases": [{"name": name} for name in sorted(analyze.PHASES)],
                    "time": {"process_wall_ns": process,
                             "method_call_wall_ns": process // 2,
                             "solve_wall_ns": process // 3},
                    "cost": {"total_gae": process},
                })
    return records


class AnalyzeControls(unittest.TestCase):
    def test_complete_and_fixed_ratio(self):
        result = analyze.analyze(fixture())
        self.assertTrue(result["complete"])
        self.assertEqual(result["valid_measured_pairs"], 32)
        self.assertEqual(result["warmups"], 16)
        self.assertAlmostEqual(
            result["ratios"]["torsion-m3/harvest-m3"]["process_wall_ns"]["ratio_of_sums"],
            0.8,
        )

    def test_missing_arm_censors_interval(self):
        rows = fixture()
        rows = [r for r in rows if not (r["arm"] == "f6-ic" and r["round"] == 1
                                         and r["workload"]["target_index"] == 0)]
        result = analyze.analyze(rows)
        self.assertFalse(result["complete"])
        self.assertEqual(result["valid_measured_pairs"], 31)
        self.assertEqual(result["ratios"], {})

    def test_changed_target_and_wrong_answer_censor(self):
        rows = fixture()
        for row in rows:
            if row["arm"] == "torsion-m3" and row["round"] == 2:
                row["workload"]["target"] = ["0xdead", "0xbeef"]
            if row["arm"] == "harvest-m3" and row["round"] == 3:
                row["outcome"]["matches_target"] = False
        result = analyze.analyze(rows)
        self.assertFalse(result["complete"])
        self.assertEqual(result["valid_measured_pairs"], 24)

    def test_duplicate_warmup_refused(self):
        rows = fixture()
        rows.append(copy.deepcopy(rows[0]))
        with self.assertRaisesRegex(ValueError, "duplicate warmup"):
            analyze.analyze(rows)


if __name__ == "__main__":
    unittest.main()
