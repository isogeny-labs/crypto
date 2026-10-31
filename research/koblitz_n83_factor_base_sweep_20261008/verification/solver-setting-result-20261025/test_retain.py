"""Controls for independently retained provider setting artifacts."""

import json
from pathlib import Path
import shutil
import tempfile
import unittest

from retain import HERE, retain


RUN = HERE / "run-40193751812-attempt-1"
FIRST = "hash_4_2_t1_m0"


class RetentionControls(unittest.TestCase):
    def test_retained_partial_binds_exact_provider_artifacts(self):
        with tempfile.TemporaryDirectory() as temp:
            shutil.copytree(RUN / FIRST, Path(temp) / FIRST)
            result = retain(Path(temp))
            self.assertEqual(result["retained_strata"], 1)
            self.assertEqual(result["retained_arms"], 5)
            self.assertEqual(result["solver_status_counts"],
                             {"INDETERMINATE": 5})
            self.assertEqual(result["status"],
                             "BINDING_ONLY_SOURCE_ZIPS_REQUIRED")
            self.assertIsNone(result["selected_best_total_runtime"])

    def test_zip_mutation_fails_provider_digest(self):
        with tempfile.TemporaryDirectory() as temp:
            target = Path(temp) / FIRST
            shutil.copytree(RUN / FIRST, target)
            with (target / "artifact.zip").open("ab") as stream:
                stream.write(b"changed")
            with self.assertRaisesRegex(ValueError, "provider identity"):
                retain(Path(temp))

    def test_forged_audit_fails_identity(self):
        with tempfile.TemporaryDirectory() as temp:
            target = Path(temp) / FIRST
            shutil.copytree(RUN / FIRST, target)
            audit_path = target / "audit.json"
            audit = json.loads(audit_path.read_bytes())
            audit["artifact_zip_sha256"] = "0" * 64
            audit_path.write_text(json.dumps(audit))
            with self.assertRaisesRegex(ValueError, "audit identity"):
                retain(Path(temp))

    def test_complete_run_snapshot_and_second_audit_are_required(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp) / "run"
            shutil.copytree(RUN, root)
            self.assertEqual(retain(root)["retained_strata"], 8)
            run_path = root / "provider-run.json"
            original = run_path.read_bytes()
            run = json.loads(original)
            run["head_sha"] = "0" * 40
            run_path.write_text(json.dumps(run))
            with self.assertRaisesRegex(ValueError, "exact frozen provider run"):
                retain(root)
            run_path.write_bytes(original)
            second = root / "collector-audits" / f"{FIRST}.audit.json"
            audit = json.loads(second.read_bytes())
            audit["artifact_zip_sha256"] = "0" * 64
            second.write_text(json.dumps(audit))
            with self.assertRaisesRegex(ValueError, "two independent audit passes"):
                retain(root)


if __name__ == "__main__":
    unittest.main()
