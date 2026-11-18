import copy
import json
from pathlib import Path
import tempfile
import unittest

import h112_reference_verify as reference
import p3_second_digit_carry_lab as lab


EXPECTED_QUICK_DIGEST = "7d52b4889623a8bd65f5451bbe1f6fbe8425d79d672fa33e6087f798401fc4b2"


class H112P3SecondDigitCarryTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.quick = lab.run_experiment(
            curves=lab.CURVES[:1],
            multipliers=lab.GENERATOR_MULTIPLIERS[:1],
            control_replicates=False,
        )

    def test_frozen_family_has_144_unique_non_low_only_predicates(self):
        self.assertEqual(len(lab.PREDICATES), 144)
        self.assertEqual(len({item["id"] for item in lab.PREDICATES}), 144)
        self.assertEqual(sum(item["family"] == "depth" for item in lab.PREDICATES), 72)
        self.assertEqual(sum(item["family"] == "cross" for item in lab.PREDICATES), 72)
        self.assertTrue(
            all(
                item.get("channel") in {"high", "sum", "difference"}
                if item["family"] == "depth"
                else {"low_offset", "high_offset"} <= item.keys()
                for item in lab.PREDICATES
            )
        )

    def test_quick_replay_has_frozen_digest_and_structural_checks(self):
        self.assertEqual(self.quick["scientific_payload_sha256"], EXPECTED_QUICK_DIGEST)
        self.assertEqual(self.quick["summary"]["targets"], 82)
        self.assertEqual(self.quick["summary"]["predicate_count"], 144)
        self.assertTrue(self.quick["summary"]["all_structural_checks_passed"])
        self.assertTrue(self.quick["summary"]["all_torsion_carries_zero"])
        self.assertEqual(self.quick["summary"]["rows_with_exact_predicate"], 0)

    def test_independent_verifier_accepts_quick_replay(self):
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "quick.json"
            path.write_text(json.dumps(self.quick))
            report = reference.verify(path)
        self.assertTrue(report["verified"])
        self.assertEqual(report["curve_rows"], 1)
        self.assertEqual(report["targets"], 82)

    def test_independent_verifier_rejects_tampered_second_digit(self):
        changed = copy.deepcopy(self.quick)
        record = changed["curves"][0]["generators"][0]["target_records"][0]
        record["digits"][1] = (record["digits"][1] + 1) % changed["curves"][0]["parameters"]["p"]
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "tampered.json"
            path.write_text(json.dumps(changed))
            with self.assertRaises(AssertionError):
                reference.verify(path)


if __name__ == "__main__":
    unittest.main()
