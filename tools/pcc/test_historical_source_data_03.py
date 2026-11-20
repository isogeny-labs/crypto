"""Custody/refusal controls; synthetic current files are not scientific evidence."""
import importlib.util
import json
import os
from pathlib import Path
import shutil
import stat
import subprocess
import tempfile
import unittest

SPEC = importlib.util.spec_from_file_location(
    "isolated_source_data", Path(__file__).with_name("prepare_historical_source_data_03.py"))
helper = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(helper)
FIXTURES = Path(__file__).resolve().parents[2]


class IsolatedSourceDataControls(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name).resolve()
        self.source = self.root / "actual-source"
        self.source.mkdir()
        self.data = self.root / "isolated-data"
        for relative in ["pcc/claims", "pcc/historical-source-data-03"]:
            shutil.copytree(FIXTURES / relative, self.source / relative)
        self.spec = json.loads((self.source / "pcc/historical-source-data-03/addresses.json").read_text())
        self.current = {}
        for relative in helper.EXPECTED:
            path = self.source / relative
            path.parent.mkdir(parents=True, exist_ok=True)
            self.current[relative] = ("complete synthetic current source: " + relative + "\n").encode()
            path.write_bytes(self.current[relative])
        self.git(self.source, "init", "-q")
        self.git(self.source, "config", "user.name", "Custody control")
        self.git(self.source, "config", "user.email", "custody@example.invalid")
        self.git(self.source, "add", "-A")
        self.git(self.source, "commit", "-qm", "synthetic current-source fixture")
        self.git(self.source, "worktree", "add", "--quiet", "--detach", str(self.data), "HEAD")
        self.receipt = self.root / "external-custody/receipt.json"

    def git(self, repo, *args):
        return subprocess.check_output(["git", "-C", str(repo), *args],
                                       env=dict(os.environ, GIT_NO_LAZY_FETCH="1"),
                                       stderr=subprocess.PIPE)

    def prepare(self):
        return helper.prepare(self.source, self.data, self.spec, self.receipt)

    def assert_source_exact(self):
        for path, expected in self.current.items():
            self.assertEqual((self.source / path).read_bytes(), expected)
            self.assertFalse((self.source / path).is_symlink())
        self.assertEqual(self.git(self.source, "status", "--porcelain"), b"")

    def test_separate_data_tree_round_trip_preserves_actual_source(self):
        modes = {p: stat.S_IMODE((self.data / p).stat().st_mode) for p in self.current}
        receipt = self.prepare()
        self.assertEqual(receipt["state"], "isolated-historical-data-view-active")
        self.assert_source_exact()
        for path, spec in helper.EXPECTED.items():
            self.assertTrue((self.data / path).is_symlink())
            self.assertEqual(helper.digest((self.data / path).read_bytes()), spec["sha256"])
        result = helper.restore(self.receipt)
        self.assertEqual(result["state"], "exact-data-sources-restored-actual-source-unchanged")
        for path, expected in self.current.items():
            self.assertEqual((self.data / path).read_bytes(), expected)
            self.assertFalse((self.data / path).is_symlink())
            self.assertEqual(stat.S_IMODE((self.data / path).stat().st_mode), modes[path])
        self.assert_source_exact()
        self.assertEqual(self.git(self.data, "status", "--porcelain"), b"")

    def test_restore_is_idempotent(self):
        self.prepare()
        helper.restore(self.receipt)
        helper.restore(self.receipt)
        self.assert_source_exact()

    def test_actual_checkout_cannot_be_used_as_data_tree(self):
        with self.assertRaisesRegex(ValueError, "separate Git worktree"):
            helper.prepare(self.source, self.source, self.spec, self.receipt)
        self.assertFalse(self.receipt.parent.exists())
        self.assert_source_exact()

    def test_unrelated_repository_is_refused(self):
        unrelated = self.root / "unrelated"
        self.git(self.source, "clone", "--quiet", str(self.source), str(unrelated))
        self.git(unrelated, "checkout", "--quiet", "--detach")
        with self.assertRaisesRegex(ValueError, "share Git"):
            helper.prepare(self.source, unrelated, self.spec, self.receipt)
        self.assert_source_exact()

    def test_data_worktree_nested_under_source_is_refused(self):
        nested = self.source / "nested-data"
        self.git(self.source, "worktree", "add", "--quiet", "--detach", str(nested), "HEAD")
        try:
            with self.assertRaisesRegex(ValueError, "non-nested separate"):
                helper.prepare(self.source, nested, self.spec, self.receipt)
            self.assertFalse(self.receipt.parent.exists())
        finally:
            self.git(self.source, "worktree", "remove", "--force", str(nested))
        self.assert_source_exact()

    def test_source_nested_under_data_is_refused_before_mutation(self):
        with self.assertRaisesRegex(ValueError, "non-nested separate"):
            helper.prepare(self.data / "nested-source", self.data, self.spec, self.receipt)
        self.assertFalse(self.receipt.parent.exists())
        self.assert_source_exact()

    def test_manifest_nonmapping_record_is_refused_before_mutation(self):
        self.spec["records"][0] = "not an address record"
        with self.assertRaisesRegex(ValueError, "complete two declared"):
            self.prepare()
        self.assertFalse(self.receipt.parent.exists())
        self.assert_source_exact()

    def test_changed_held_mode_is_refused_before_any_restore(self):
        receipt = self.prepare()
        row = receipt["records"][1]
        path = Path(row["held_current"])
        os.chmod(path, row["mode"] ^ stat.S_IXUSR)
        with self.assertRaisesRegex(ValueError, "held whole current"):
            helper.restore(self.receipt)
        self.assertTrue(all((self.data / p).is_symlink() for p in self.current))
        self.assert_source_exact()
        os.chmod(path, row["mode"])
        helper.restore(self.receipt)

    def test_attached_data_branch_is_refused(self):
        self.git(self.data, "checkout", "--quiet", "-b", "data-branch")
        with self.assertRaisesRegex(ValueError, "detached HEAD"):
            self.prepare()
        self.assert_source_exact()

    def test_different_data_head_is_refused(self):
        self.git(self.data, "commit", "--allow-empty", "-qm", "different data HEAD")
        with self.assertRaisesRegex(ValueError, "HEAD differ"):
            self.prepare()
        self.assert_source_exact()

    def test_dirty_actual_source_is_preserved_and_refused(self):
        path = self.source / next(iter(self.current))
        path.write_bytes(b"unrelated current edit")
        with self.assertRaisesRegex(ValueError, "actual source checkout"):
            self.prepare()
        self.assertEqual(path.read_bytes(), b"unrelated current edit")
        self.assertFalse(self.receipt.parent.exists())

    def test_dirty_data_current_file_is_preserved_and_refused(self):
        path = self.data / next(iter(self.current))
        path.write_bytes(b"unrelated data edit")
        with self.assertRaisesRegex(ValueError, "current data/source bytes"):
            self.prepare()
        self.assertEqual(path.read_bytes(), b"unrelated data edit")
        self.assert_source_exact()

    def test_changed_frozen_claim_is_refused_before_holds(self):
        path = self.data / "pcc/claims/PCC-20261115-9134a9.json"
        path.write_bytes(path.read_bytes() + b"\n")
        with self.assertRaisesRegex(ValueError, "frozen claim bytes"):
            self.prepare()
        self.assertFalse(self.receipt.parent.exists())
        self.assert_source_exact()

    def test_changed_canonical_object_is_refused(self):
        row = self.spec["records"][0]
        (self.data / row["canonical_path"]).write_bytes(b"changed")
        with self.assertRaisesRegex(ValueError, "frozen pin"):
            self.prepare()
        self.assertFalse(self.receipt.parent.exists())
        self.assert_source_exact()

    def test_outside_canonical_symlink_is_refused(self):
        row = self.spec["records"][0]
        path = self.data / row["canonical_path"]
        outside = self.root / "outside"
        outside.write_bytes(path.read_bytes())
        path.unlink()
        path.symlink_to(outside)
        with self.assertRaisesRegex(ValueError, "regular contained file"):
            self.prepare()
        self.assert_source_exact()

    def test_additional_address_user_is_refused(self):
        claim = json.loads((self.data / "pcc/claims/PCC-20261115-9134a9.json").read_text())
        claim["id"] = "PCC-20261115-other"
        (self.data / "pcc/claims/extra.json").write_text(json.dumps(claim))
        with self.assertRaisesRegex(ValueError, "additional user"):
            self.prepare()
        self.assertFalse(self.receipt.parent.exists())
        self.assert_source_exact()

    def test_tampered_held_file_cannot_overwrite_any_address(self):
        receipt = self.prepare()
        held = Path(receipt["records"][1]["held_current"])
        saved = held.read_bytes()
        held.write_bytes(b"changed held current")
        with self.assertRaisesRegex(ValueError, "held whole current"):
            helper.restore(self.receipt)
        self.assertTrue(all((self.data / p).is_symlink() for p in self.current))
        self.assert_source_exact()
        held.write_bytes(saved)
        helper.restore(self.receipt)

    def test_unrelated_cleanup_target_is_not_overwritten(self):
        receipt = self.prepare()
        row = receipt["records"][0]
        path = self.data / row["legacy_path"]
        path.unlink()
        path.write_bytes(b"unrelated replacement")
        with self.assertRaisesRegex(ValueError, "changed or unrelated"):
            helper.restore(self.receipt)
        self.assertEqual(path.read_bytes(), b"unrelated replacement")
        self.assert_source_exact()
        path.unlink()
        path.symlink_to(row["canonical"])
        helper.restore(self.receipt)

    def test_actual_source_edit_during_view_is_not_overwritten(self):
        self.prepare()
        relative = next(iter(self.current))
        path = self.source / relative
        path.write_bytes(b"current source independently changed")
        with self.assertRaisesRegex(ValueError, "actual source checkout"):
            helper.restore(self.receipt)
        self.assertEqual(path.read_bytes(), b"current source independently changed")
        path.write_bytes(self.current[relative])
        helper.restore(self.receipt)

    def test_parent_redirect_to_actual_source_is_refused(self):
        self.prepare()
        relative = next(iter(self.current))
        parent = (self.data / relative).parent
        saved_parent = parent.with_name(parent.name + ".retained-original-parent")
        source_parent = (self.source / relative).parent
        inode = (self.source / relative).stat().st_ino
        parent.rename(saved_parent)
        parent.symlink_to(source_parent, target_is_directory=True)
        try:
            with self.assertRaisesRegex(ValueError, "parent became a symlink"):
                helper.restore(self.receipt)
            self.assertEqual((self.source / relative).stat().st_ino, inode)
            self.assert_source_exact()
        finally:
            parent.unlink()
            saved_parent.rename(parent)
            helper.restore(self.receipt)

    def remove_from_committed_tree_but_keep_file(self, relative):
        old_head = self.git(self.source, "rev-parse", "HEAD").decode().strip()
        self.git(self.source, "rm", "--cached", "--", relative)
        self.git(self.source, "commit", "-qm", "remove only committed metadata in negative fixture")
        new_head = self.git(self.source, "rev-parse", "HEAD").decode().strip()
        self.git(self.data, "update-ref", "HEAD", new_head, old_head)

    def test_uncommitted_frozen_claim_is_refused(self):
        name = "pcc/claims/PCC-20261115-9134a9.json"
        original = (self.source / name).read_bytes()
        self.remove_from_committed_tree_but_keep_file(name)
        with self.assertRaisesRegex(ValueError, "claim is not committed"):
            self.prepare()
        self.assertEqual((self.source / name).read_bytes(), original)
        self.assertFalse(self.receipt.parent.exists())

    def test_uncommitted_canonical_data_is_refused(self):
        name = self.spec["records"][0]["canonical_path"]
        original = (self.source / name).read_bytes()
        self.remove_from_committed_tree_but_keep_file(name)
        with self.assertRaisesRegex(ValueError, "data is not committed"):
            self.prepare()
        self.assertEqual((self.source / name).read_bytes(), original)
        self.assertFalse(self.receipt.parent.exists())

    def test_partial_prepare_has_complete_recoverable_custody(self):
        second = self.spec["records"][1]["legacy_path"]
        target = self.data / second
        temporary = target.with_name(target.name + ".historical-data-address-03")
        temporary.write_bytes(b"unowned temporary file")
        with self.assertRaisesRegex(ValueError, "unowned temporary"):
            self.prepare()
        self.assertTrue(self.receipt.is_file())
        helper.restore(self.receipt)
        self.assertEqual(temporary.read_bytes(), b"unowned temporary file")
        self.assert_source_exact()
        for path, expected in self.current.items():
            self.assertEqual((self.data / path).read_bytes(), expected)

    def test_custody_inside_either_worktree_is_refused(self):
        for root in (self.source, self.data):
            with self.subTest(root=root), self.assertRaisesRegex(ValueError, "external custody"):
                helper.prepare(self.source, self.data, self.spec, root / "new-hold/receipt.json")
            self.assertFalse((root / "new-hold").exists())
        self.assert_source_exact()


if __name__ == "__main__":
    unittest.main()
