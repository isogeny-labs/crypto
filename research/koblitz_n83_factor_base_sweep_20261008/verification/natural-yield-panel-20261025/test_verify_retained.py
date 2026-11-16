"""Evidence-integrity controls for the committed hosted N83 receipts."""

import json
from pathlib import Path
import shutil
import tempfile
import unittest

from verify_retained import ROOT, replay


class RetainedReplayTests(unittest.TestCase):
    def test_exact_retained_receipts(self):
        result = replay()
        self.assertEqual(result["status"], "PASS_RETAINED_24_CENSORED_ARMS")
        self.assertEqual(result["solver_arms"], 24)
        self.assertIsNone(result["selected_best_total_runtime"])

    def test_changed_solver_stream_fails_closed(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary) / "evidence"
            shutil.copytree(ROOT, root)
            stream = root / "t1_m0/n83-pattern-results/hash51_A1/solver.stdout.gz"
            stream.write_bytes(stream.read_bytes() + b"forged")
            with self.assertRaisesRegex(ValueError, "retained SHA-256"):
                replay(root)

    def test_changed_provider_job_fails_closed(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary) / "evidence"
            shutil.copytree(ROOT, root)
            path = root / "provider-jobs.json"
            jobs = json.loads(path.read_bytes())
            jobs["jobs"][0]["conclusion"] = "failure"
            path.write_text(json.dumps(jobs))
            with self.assertRaisesRegex(ValueError, "five terminal provider jobs"):
                replay(root)


if __name__ == "__main__":
    unittest.main()
