"""Saved manifest coverage and cross-binding controls."""

import hashlib
import json
from pathlib import Path
import shutil
import tempfile
import unittest

from catalog import catalog, EVIDENCE


class ManifestCensusTests(unittest.TestCase):
    def test_all_current_objects_reconcile(self):
        result = catalog()
        self.assertEqual(result["status"], "PASS_77_BASES_RECONCILED")
        self.assertEqual(result["curve_arm_object_counts"], {"a0": 50, "a1": 27})
        self.assertEqual(result["manifest_declared_unique_point_sets"],
                         {"a0": 50, "a1": 15})
        self.assertEqual(len(result["records"]), 77)
        self.assertEqual(result, json.loads((EVIDENCE / "catalog.json").read_bytes()))

    def test_missing_manifest_fails_coverage(self):
        with tempfile.TemporaryDirectory() as temporary:
            evidence = Path(temporary) / "evidence"
            shutil.copytree(EVIDENCE, evidence)
            (evidence / "22.json").unlink()
            with self.assertRaises(FileNotFoundError):
                catalog(evidence)

    def test_changed_manifest_claim_fails_even_if_receipt_sha_is_updated(self):
        with tempfile.TemporaryDirectory() as temporary:
            evidence = Path(temporary) / "evidence"
            shutil.copytree(EVIDENCE, evidence)
            path = evidence / "22.json"
            document = json.loads(path.read_bytes())
            raw = path.read_bytes().replace(
                document["compressed_blake3"].encode(), b"0" * 64, 1)
            path.write_bytes(raw)
            index_path = evidence / "index.json"
            index = json.loads(index_path.read_bytes())
            index[22]["sha256"] = hashlib.sha256(raw).hexdigest()
            index_path.write_text(json.dumps(index, indent=2, sort_keys=True) + "\n")
            with self.assertRaisesRegex(ValueError,
                                        "manifest claims match downloaded content address"):
                catalog(evidence)

    def test_changed_direct_replay_fails_pinned_source_hash(self):
        with tempfile.TemporaryDirectory() as temporary:
            evidence = Path(temporary) / "evidence"
            shutil.copytree(EVIDENCE, evidence)
            path = evidence / "direct-replay.json"
            raw = path.read_bytes().replace(b'"PASS"', b'"FAIL"')
            path.write_bytes(raw)
            with self.assertRaisesRegex(ValueError,
                                        "exact prior independent direct-base replay receipt"):
                catalog(evidence)


if __name__ == "__main__":
    unittest.main()
