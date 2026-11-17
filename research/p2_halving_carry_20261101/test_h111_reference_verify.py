import json
from pathlib import Path
import tempfile
import unittest

import p2_halving_carry_lab as lab
from research import h111_reference_verify as reference


class H111ReferenceVerifyTests(unittest.TestCase):
    def test_independent_verifier_accepts_quick_artifact(self):
        result = lab.run_experiment(
            curves=lab.CURVES[:1],
            multipliers=lab.GENERATOR_MULTIPLIERS[:1],
            control_replicates=False,
        )
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "quick.json"
            path.write_text(json.dumps(result))
            report = reference.verify(path)
        self.assertTrue(report["verified"])
        self.assertEqual(report["curve_rows"], 1)
        self.assertEqual(report["targets"], 82)

    def test_independent_verifier_rejects_changed_carry(self):
        result = lab.run_experiment(
            curves=lab.CURVES[:1],
            multipliers=lab.GENERATOR_MULTIPLIERS[:1],
            control_replicates=False,
        )
        result["curves"][0]["generators"][0]["target_records"][0]["kappa"] ^= 1
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "tampered.json"
            path.write_text(json.dumps(result))
            with self.assertRaises(AssertionError):
                reference.verify(path)


if __name__ == "__main__":
    unittest.main()
