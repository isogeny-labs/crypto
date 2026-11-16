"""Metadata address controls exercising the unchanged original verifier."""
import copy
import hashlib
import importlib.util
import json
from pathlib import Path
import sys
import tempfile
import unittest

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
import prepare_legacy_evidence_aliases as aliases

spec = importlib.util.spec_from_file_location("original_pcc_alias_test", HERE / "pcc.py")
pcc = importlib.util.module_from_spec(spec)
sys.modules[spec.name] = pcc
spec.loader.exec_module(pcc)


class AddressControls(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.repo = self.root / "repo"
        (self.repo / "pcc/claims").mkdir(parents=True)
        self.canonical = self.repo / "receipt.txt"
        self.canonical.write_bytes(b"authentic immutable publication bytes\n")
        self.legacy = self.root / "old-workspace/receipt.txt"
        self.proof = {"kind": "attestation", "subject": {"path": str(self.legacy), "sha256": aliases.digest(self.canonical)}}
        claim = {"id": "PCC-fixture", "kind": "statement", "claim_type": "data", "proofs": [self.proof]}
        self.claim = self.repo / "pcc/claims/PCC-fixture.json"
        self.claim.write_text(json.dumps(claim))
        self.manifest = {"schema": "pcc.legacy-evidence-addresses/v1", "aliases": [{"claim": "PCC-fixture", "claim_sha256": aliases.digest(self.claim), "proof_index": 0, "legacy_path": str(self.legacy), "canonical_path": "receipt.txt", "sha256": aliases.digest(self.canonical)}]}

    def verify(self):
        reasons, notes = [], []
        verdict = pcc.verify_attestation(self.proof, self.repo, {}, reasons, notes)
        return verdict, reasons

    def test_missing_absolute_address_still_rejects(self):
        verdict, reasons = self.verify()
        self.assertEqual(verdict, "fail")
        self.assertTrue(any("escapes" in r for r in reasons))

    def test_correct_alias_remains_unsigned_under_original_verifier(self):
        receipt = aliases.prepare(self.repo, self.manifest)
        self.assertEqual(self.verify(), ("not_covered", []))
        self.assertEqual(self.canonical.read_bytes(), b"authentic immutable publication bytes\n")
        self.assertFalse(aliases.prepare(self.repo, self.manifest)["addresses"][0]["created"])
        aliases.cleanup(receipt)
        self.assertFalse(self.legacy.exists())

    def test_changed_receipt_digest_rejects(self):
        aliases.prepare(self.repo, self.manifest)
        self.canonical.write_bytes(b"changed\n")
        verdict, reasons = self.verify()
        self.assertEqual(verdict, "fail")
        self.assertTrue(any("does not hash to its pin" in r for r in reasons))

    def test_true_escape_through_alias_rejects(self):
        outside = self.root / "outside.txt"
        outside.write_bytes(self.canonical.read_bytes())
        self.legacy.parent.mkdir()
        self.legacy.symlink_to(outside)
        verdict, reasons = self.verify()
        self.assertEqual(verdict, "fail")
        self.assertTrue(any("escapes" in r for r in reasons))
        with self.assertRaises(ValueError):
            aliases.prepare(self.repo, self.manifest)

    def test_existing_regular_legacy_file_is_not_overwritten(self):
        self.legacy.parent.mkdir()
        self.legacy.write_bytes(b"keep every original byte")
        with self.assertRaises(ValueError):
            aliases.prepare(self.repo, self.manifest)
        self.assertEqual(self.legacy.read_bytes(), b"keep every original byte")

    def test_changed_claim_and_wrong_canonical_pin_reject(self):
        changed = copy.deepcopy(self.manifest)
        changed["aliases"][0]["claim_sha256"] = "0" * 64
        with self.assertRaises(ValueError):
            aliases.prepare(self.repo, changed)
        self.canonical.write_bytes(b"different")
        with self.assertRaises(ValueError):
            aliases.prepare(self.repo, self.manifest)
        self.assertFalse(self.legacy.exists())

    def test_outside_canonical_and_missing_canonical_reject(self):
        changed = copy.deepcopy(self.manifest)
        changed["aliases"][0]["canonical_path"] = "../outside.txt"
        with self.assertRaises(ValueError):
            aliases.prepare(self.repo, changed)
        self.canonical.unlink()
        with self.assertRaises(ValueError):
            aliases.prepare(self.repo, self.manifest)

    def test_cleanup_never_removes_changed_or_unowned_paths(self):
        receipt = aliases.prepare(self.repo, self.manifest)
        self.legacy.unlink()
        self.legacy.write_bytes(b"new unrelated bytes")
        with self.assertRaises(ValueError):
            aliases.cleanup(receipt)
        self.assertEqual(self.legacy.read_bytes(), b"new unrelated bytes")


if __name__ == "__main__":
    unittest.main()
