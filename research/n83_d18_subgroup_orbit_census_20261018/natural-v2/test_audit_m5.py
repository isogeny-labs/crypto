"""Tamper checks against the retained m=5 Spot receipts."""

import json
import os
from pathlib import Path
import shutil
import tempfile
import unittest

import audit_m5 as audit


FIXTURE_ROOT = Path(os.environ.get("N83_M5_AUDIT_ROOT", Path(__file__).resolve().parent))


class AuditM5Tests(unittest.TestCase):
    def copied(self, name):
        temp = tempfile.TemporaryDirectory()
        self.addCleanup(temp.cleanup)
        root = Path(temp.name)
        shutil.copytree(FIXTURE_ROOT / "spot-arms" / name, root / "spot-arms" / name)
        return root

    def test_retained_prerequisites_and_first_natural_cap(self):
        for mode, group in (("preflight", audit.PREFLIGHTS),
                            ("positive", audit.CONTROLS)):
            for name, arm, fixture in group:
                with self.subTest(name=name):
                    row = audit.audit_one(FIXTURE_ROOT, name, arm, fixture, mode)
                    self.assertEqual(row["mode"], mode)
        row = audit.audit_one(FIXTURE_ROOT, "n-u0", "U", 0, "natural")
        self.assertEqual(row["status"], "UNKNOWN_budget")
        self.assertIsNone(row["selected_best_total_runtime"])

    def test_forged_cap_success_is_rejected(self):
        root = self.copied("n-u0")
        path = root / "spot-arms/n-u0/worker.json.cap.json"
        value = json.loads(path.read_text())
        value["status"] = "PASS_NATURAL_RELATION_PENDING_REPLAY"
        path.write_text(json.dumps(value))
        with self.assertRaisesRegex(ValueError, "cap receipt"):
            audit.audit_one(root, "n-u0", "U", 0, "natural")

    def test_swapped_base_identity_is_rejected(self):
        root = self.copied("n-u0")
        path = root / "spot-arms/n-u0/worker.json.progress.jsonl"
        rows = [json.loads(line) for line in path.read_text().splitlines()]
        rows[-1]["object_blake3"] = audit.BASES["D"][0]
        path.write_text("\n".join(json.dumps(row) for row in rows) + "\n")
        with self.assertRaisesRegex(ValueError, "progress identity"):
            audit.audit_one(root, "n-u0", "U", 0, "natural")

    def test_network_enabled_container_is_rejected(self):
        root = self.copied("p-d0")
        path = root / "spot-arms/p-d0/container-inspect.json"
        value = json.loads(path.read_text())
        value[0]["HostConfig"]["NetworkMode"] = "bridge"
        path.write_text(json.dumps(value))
        with self.assertRaisesRegex(ValueError, "network/root"):
            audit.audit_one(root, "p-d0", "D", 0, "preflight")

    def test_source_swap_is_rejected(self):
        root = self.copied("p-u0")
        path = root / "spot-arms/p-u0/worker.json.phases.jsonl"
        rows = [json.loads(line) for line in path.read_text().splitlines()]
        rows[0]["source_commit"] = "0" * 40
        path.write_text("\n".join(json.dumps(row) for row in rows) + "\n")
        with self.assertRaisesRegex(ValueError, "phase identity"):
            audit.audit_one(root, "p-u0", "U", 0, "preflight")


if __name__ == "__main__":
    unittest.main()
