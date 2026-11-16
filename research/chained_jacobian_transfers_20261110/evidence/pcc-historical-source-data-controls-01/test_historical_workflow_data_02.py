"""File/Git metadata controls; the original attestation verifier is unchanged."""
import copy
import importlib.util
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import unittest

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
import prepare_historical_workflow_data_02 as views

spec = importlib.util.spec_from_file_location("original_pcc_source_data_test", HERE / "pcc.py")
pcc = importlib.util.module_from_spec(spec)
sys.modules[spec.name] = pcc
spec.loader.exec_module(pcc)


class HistoricalSourceDataControls(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.repo = self.root / "repo"
        (self.repo / "pcc/claims").mkdir(parents=True)
        self.legacy = self.repo / views.LEGACY
        self.legacy.parent.mkdir(parents=True)
        self.current = b"name: actual current workflow\n"
        self.old = b"name: authentic historical workflow\n"
        self.legacy.write_bytes(self.current)
        subprocess.run(["git", "init", "--quiet", str(self.repo)], check=True, capture_output=True)
        subprocess.run(["git", "add", "--", views.LEGACY], cwd=self.repo, check=True, capture_output=True)
        subprocess.run(["git", "-c", "user.name=Source data control", "-c", "user.email=source-data-control@example.invalid", "commit", "--quiet", "-m", "fixture current workflow"], cwd=self.repo, check=True, capture_output=True)
        self.canonical = self.repo / "evidence/entire-old-workflow.yml"
        self.canonical.parent.mkdir()
        self.canonical.write_bytes(self.old)
        self.proof = {"kind": "attestation", "subject": {"path": views.LEGACY, "sha256": views.digest(self.old)}}
        self.claim = self.repo / "pcc/claims/PCC-fixture.json"
        self.claim.write_text(json.dumps({"id": "PCC-fixture", "kind": "statement", "claim_type": "data", "proofs": [self.proof]}))
        self.manifest = {"schema": "pcc.historical-workflow-data-address/v2", "claim": "PCC-fixture", "claim_sha256": views.digest(self.claim.read_bytes()), "proof_index": 0, "legacy_path": views.LEGACY, "canonical_path": self.canonical.relative_to(self.repo).as_posix(), "sha256": views.digest(self.old)}
        self.receipt = self.root / "fresh-hold/receipt.json"
        self.addCleanup(self.preserve)

    def preserve(self):
        output = os.environ.get("PCC_SOURCE_VIEW_CONTROLS_OUT")
        if output:
            destination = Path(output) / self._testMethodName
            destination.parent.mkdir(parents=True, exist_ok=True)
            shutil.copytree(self.root, destination, symlinks=True)
            (destination / "original-fixture-root.txt").write_text(str(self.root) + "\n")

    def verdict(self):
        reasons, notes = [], []
        verdict = pcc.verify_attestation(self.proof, self.repo, {}, reasons, notes)
        return verdict, reasons, notes

    def prepare(self, manifest=None):
        return views.prepare(self.repo, manifest or self.manifest, self.receipt)

    def test_complete_prepare_original_attestation_restore(self):
        self.assertEqual(self.verdict()[0], "fail")
        record = self.prepare()
        self.assertTrue(self.legacy.is_symlink())
        self.assertEqual(self.legacy.read_bytes(), self.old)
        self.assertEqual(Path(record["held_current"]).read_bytes(), self.current)
        self.assertEqual(self.verdict()[0:2], ("not_covered", []))
        restored = views.restore(self.receipt)
        self.assertEqual(restored["state"], "exact-current-source-restored")
        self.assertFalse(self.legacy.is_symlink())
        self.assertEqual(self.legacy.read_bytes(), self.current)
        self.assertEqual(self.canonical.read_bytes(), self.old)
        self.assertEqual(Path(record["held_current"]).read_bytes(), self.current)
        self.assertEqual(self.verdict()[0], "fail") # Old data are not current source.
        self.assertEqual(views.restore(self.receipt)["state"], "exact-current-source-restored")

    def test_changed_canonical_rejects_without_changing_live_source(self):
        self.canonical.write_bytes(b"different whole canonical file\n")
        with self.assertRaises(ValueError): self.prepare()
        self.assertEqual(self.legacy.read_bytes(), self.current)
        self.assertFalse(self.receipt.exists())

    def test_changed_claim_rejects(self):
        self.claim.write_text(self.claim.read_text()+"\n")
        with self.assertRaises(ValueError): self.prepare()
        self.assertEqual(self.legacy.read_bytes(), self.current)

    def test_nondataclaim_rejects_even_with_rebound_claim_hash(self):
        claim = json.loads(self.claim.read_text())
        claim["kind"] = "theorem"
        self.claim.write_text(json.dumps(claim))
        self.manifest["claim_sha256"] = views.digest(self.claim.read_bytes())
        with self.assertRaises(ValueError): self.prepare()
        self.assertEqual(self.legacy.read_bytes(), self.current)

    def test_conflicting_historical_address_user_rejects(self):
        other = {"id": "PCC-other", "kind": "statement", "claim_type": "data", "proofs": [{"kind": "attestation", "subject": {"path": views.LEGACY, "sha256": "0"*64}}]}
        (self.repo / "pcc/claims/PCC-other.json").write_text(json.dumps(other))
        with self.assertRaises(ValueError): self.prepare()
        self.assertEqual(self.legacy.read_bytes(), self.current)

    def test_undeclared_second_user_of_same_digest_rejects(self):
        other = json.loads(self.claim.read_text());other["id"] = "PCC-other"
        (self.repo / "pcc/claims/PCC-other.json").write_text(json.dumps(other))
        with self.assertRaises(ValueError): self.prepare()

    def test_outside_or_missing_canonical_rejects(self):
        changed = copy.deepcopy(self.manifest);changed["canonical_path"] = "../outside.yml"
        with self.assertRaises(ValueError): self.prepare(changed)
        self.canonical.unlink()
        with self.assertRaises(ValueError): self.prepare()
        self.assertEqual(self.legacy.read_bytes(), self.current)

    def test_modified_or_symlinked_current_source_is_preserved(self):
        self.legacy.write_bytes(b"uncommitted actual edits\n")
        with self.assertRaises(ValueError): self.prepare()
        self.assertEqual(self.legacy.read_bytes(), b"uncommitted actual edits\n")
        self.legacy.unlink();self.legacy.symlink_to(self.canonical)
        with self.assertRaises(ValueError): self.prepare()
        self.assertTrue(self.legacy.is_symlink())

    def test_unrelated_or_changed_cleanup_target_is_preserved(self):
        record = self.prepare()
        self.legacy.unlink();self.legacy.write_bytes(b"unrelated new source\n")
        with self.assertRaises(ValueError): views.restore(self.receipt)
        self.assertEqual(self.legacy.read_bytes(), b"unrelated new source\n")
        self.assertEqual(Path(record["held_current"]).read_bytes(), self.current)

    def test_changed_held_source_is_not_used_for_restore(self):
        record = self.prepare()
        Path(record["held_current"]).write_bytes(b"whole changed held source\n")
        with self.assertRaises(ValueError): views.restore(self.receipt)
        self.assertEqual(self.legacy.read_bytes(), self.old)

    def test_other_address_and_inrepo_receipt_are_refused(self):
        changed = copy.deepcopy(self.manifest);changed["legacy_path"] = "pcc/keyring.json"
        with self.assertRaises(ValueError): self.prepare(changed)
        with self.assertRaises(ValueError): views.prepare(self.repo, self.manifest, self.repo / "hold/receipt.json")
        self.assertEqual(self.legacy.read_bytes(), self.current)


if __name__ == "__main__":
    unittest.main()
