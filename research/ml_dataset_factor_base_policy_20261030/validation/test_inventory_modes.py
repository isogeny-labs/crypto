import hashlib
import json
from pathlib import Path
import subprocess
import tempfile
import unittest

from inventory_modes import audit, regular_git_mode


class Modes(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.root = Path(self.tmp.name)
        subprocess.run(["git", "init", "-q", self.root], check=True)
        self.path = self.root / "executable.sh"
        self.path.write_bytes(b"#!/bin/sh\nexit 0\n")
        self.path.chmod(0o700)
        subprocess.run(["git", "add", "executable.sh"], cwd=self.root, check=True)
        subprocess.run(["git", "-c", "core.hooksPath=/dev/null", "-c", "user.name=Fixture", "-c", "user.email=fixture@invalid",
                        "commit", "-qm", "fixed executable fixture"], cwd=self.root, check=True)
        self.source = subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=self.root).decode().strip()
        data = self.path.read_bytes()
        blob = subprocess.check_output(["git", "hash-object", "executable.sh"], cwd=self.root).decode().strip()
        self.previous = self.root / "prior.json"
        self.previous.write_text(json.dumps({"schema": "exact-source-materialization/v1", "files": 1,
            "bytes": len(data), "entries": [{"path": "executable.sh", "git_blob": blob,
                "sha256": hashlib.sha256(data).hexdigest(), "bytes": len(data)}]}))

    def tearDown(self):
        self.tmp.cleanup()

    def test_mode_is_bound_even_when_skip_worktree_hides_diff(self):
        subprocess.run(["git", "update-index", "--skip-worktree", "executable.sh"], cwd=self.root, check=True)
        self.path.chmod(0o600)
        # This recreates the gap: content hashes match and ordinary diff passes.
        subprocess.run(["git", "diff", "--exit-code", "HEAD"], cwd=self.root, check=True)
        with self.assertRaisesRegex(ValueError, "mode differs"):
            audit(self.root, self.source, self.previous)

    def test_private_permissions_preserved_by_git_mode_semantics(self):
        result = audit(self.root, self.source, self.previous)
        self.assertEqual(result["entries"][0]["git_mode"], "100755")
        self.assertEqual(self.path.stat().st_mode & 0o777, 0o700)
        self.path.chmod(0o755)
        self.assertEqual(audit(self.root, self.source, self.previous)["entries"], result["entries"])

    def test_group_execute_is_not_git_owner_execute(self):
        self.path.chmod(0o610)
        self.assertEqual(regular_git_mode(self.path), "100644")
        with self.assertRaisesRegex(ValueError, "mode differs"):
            audit(self.root, self.source, self.previous)

    def test_content_change_still_rejected_after_mode_repair(self):
        self.path.write_bytes(b"#!/bin/sh\nexit 1\n")
        with self.assertRaisesRegex(ValueError, "content differs"):
            audit(self.root, self.source, self.previous)

    def test_symlink_and_directory_are_not_regular_inputs(self):
        self.path.unlink()
        self.path.symlink_to(self.previous)
        with self.assertRaisesRegex(ValueError, "not a regular"):
            audit(self.root, self.source, self.previous)
        self.path.unlink()
        self.path.mkdir()
        with self.assertRaisesRegex(ValueError, "not a regular"):
            audit(self.root, self.source, self.previous)


if __name__ == "__main__":
    unittest.main()
