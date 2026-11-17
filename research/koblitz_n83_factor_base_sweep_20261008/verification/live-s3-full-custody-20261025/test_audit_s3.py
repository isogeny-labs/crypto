"""Fail-closed controls for the saved two-pass N83 S3 custody receipt."""

import hashlib
import json
from pathlib import Path
import shutil
import tempfile
import unittest

from audit_s3 import check, classify, expected_digest


HERE = Path(__file__).resolve().parent
EVIDENCE = HERE / "evidence-01"


class S3CustodyTests(unittest.TestCase):
    def test_current_saved_receipt_and_repeat(self):
        result = check(EVIDENCE)
        self.assertEqual(result["status"],
                         "PASS_SAVED_RECEIPT_INTERNAL_CONSISTENCY")
        self.assertEqual(result["archive_copies"], 80)
        self.assertEqual(result["unique_compressed_digests"], 77)
        self.assertEqual(result["repeated_get_object_count"], 77)

    def test_exact_archive_namespaces(self):
        self.assertIsNone(classify(
            "factor-bases/icv1/etc/koblitz_n83_factor_base_sweep_20261008/other/x.jsonl.gz"))
        key = next(row["key"] for row in
                   json.loads((EVIDENCE / "replay.json").read_bytes())["rows"]
                   if row["kind"] == "cached_input")
        self.assertEqual(classify(key)[0], "cached_input")
        self.assertEqual(len(expected_digest(key)), 64)

    def test_changed_sha_with_updated_receipt_digest_still_fails(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary) / "evidence"
            shutil.copytree(EVIDENCE, root)
            path = root / "replay.json"
            data = json.loads(path.read_bytes())
            data["rows"][0]["sha256"] = "0" * 64
            path.write_text(json.dumps(data, indent=2, sort_keys=True) + "\n")
            comparison_path = root / "repeat-comparison.json"
            comparison = json.loads(comparison_path.read_bytes())
            comparison["second_pass_receipt_sha256"] = hashlib.sha256(
                path.read_bytes()).hexdigest()
            comparison_path.write_text(
                json.dumps(comparison, indent=2, sort_keys=True) + "\n")
            with self.assertRaisesRegex(ValueError,
                                        "same content bytes in both fresh GET rounds"):
                check(root)

    def test_changed_first_pass_fails_hash_binding(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary) / "evidence"
            shutil.copytree(EVIDENCE, root)
            path = root / "first-pass.json"
            data = json.loads(path.read_bytes())
            data["elapsed_wall_seconds_L0"] += 1
            path.write_text(json.dumps(data, indent=2, sort_keys=True) + "\n")
            with self.assertRaisesRegex(ValueError,
                                        "two saved GET rounds and comparison digests"):
                check(root)


if __name__ == "__main__":
    unittest.main()
