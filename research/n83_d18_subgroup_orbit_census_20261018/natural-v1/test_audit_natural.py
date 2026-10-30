"""Fail-closed checks for retained natural-arm evidence."""
import json
import shutil
import tempfile
import unittest
from pathlib import Path

import audit_natural


class AuditNaturalTest(unittest.TestCase):
    def setUp(self):
        self.original_root = audit_natural.ROOT
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.addCleanup(setattr, audit_natural, "ROOT", self.original_root)
        audit_natural.ROOT = Path(self.temp.name)
        shutil.copytree(self.original_root / "spot-arms" / "u1", audit_natural.ROOT / "spot-arms" / "u1")
        self.run_dir = audit_natural.ROOT / "spot-arms" / "u1"

    def audit(self):
        return audit_natural.audit_one(*audit_natural.ARMS[0])

    def test_retained_capped_arm_passes(self):
        result = self.audit()
        self.assertEqual(result["status"], "UNKNOWN_budget")
        self.assertIsNone(result["selected_best_total_runtime"])

    def test_forged_success_status_rejected(self):
        path = self.run_dir / "worker.json.cap.json"
        data = json.loads(path.read_text())
        data["status"] = "PASS_NATURAL_RELATION_PENDING_REPLAY"
        path.write_text(json.dumps(data))
        with self.assertRaisesRegex(ValueError, "cap status"):
            self.audit()

    def test_swapped_base_identity_rejected(self):
        path = self.run_dir / "worker.json.progress.jsonl"
        lines = path.read_text().splitlines()
        row = json.loads(lines[0])
        row["object_blake3"] = audit_natural.ARMS[1][2]
        lines[0] = json.dumps(row)
        path.write_text("\n".join(lines) + "\n")
        with self.assertRaisesRegex(ValueError, "progress identity"):
            self.audit()

    def test_missing_progress_rejected(self):
        (self.run_dir / "worker.json.progress.jsonl").unlink()
        with self.assertRaises(FileNotFoundError):
            self.audit()

    def test_spot_interruption_requires_ec2_status(self):
        target = audit_natural.ROOT / "spot-arms" / "u2"
        shutil.copytree(self.original_root / "spot-arms" / "u2", target)
        self.assertEqual(audit_natural.audit_interruption()["status"], "UNKNOWN_spot_interruption")
        path = target / "spot-request.json"
        data = json.loads(path.read_text())
        data["Status"]["Code"] = "fulfilled"
        path.write_text(json.dumps(data))
        with self.assertRaisesRegex(ValueError, "spot status"):
            audit_natural.audit_interruption()


if __name__ == "__main__":
    unittest.main()
