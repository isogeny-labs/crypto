"""Synthetic regressions for exact historical source selection."""
from __future__ import annotations

import hashlib
from pathlib import Path
import subprocess
import tempfile
import unittest

from frozen_source_selector import exact_historical_bytes, materialize


def run(repo: Path, *args: str) -> None:
    subprocess.run(["git", *args], cwd=repo, check=True, capture_output=True)


def sha(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


class FrozenSourceSelectorTests(unittest.TestCase):
    def repo_with_commits(self) -> tuple[tempfile.TemporaryDirectory, Path, str, bytes]:
        td = tempfile.TemporaryDirectory()
        repo = Path(td.name)
        run(repo, "init", "-q")
        run(repo, "config", "user.name", "Selector Test")
        run(repo, "config", "user.email", "selector@example.invalid")
        path = repo / "src/pinned.rs"
        path.parent.mkdir(parents=True)
        pinned = b"historical pinned bytes\n"
        path.write_bytes(pinned)
        run(repo, "add", "src/pinned.rs")
        run(repo, "commit", "-qm", "pinned")
        pinned_commit = subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=repo, text=True).strip()
        path.write_bytes(b"current changed bytes\n")
        run(repo, "commit", "-qam", "current")
        return td, repo, pinned_commit, pinned

    def test_exact_current_pinned_bytes_are_accepted(self):
        with tempfile.TemporaryDirectory() as tmp:
            repo = Path(tmp)
            run(repo, "init", "-q")
            run(repo, "config", "user.name", "Selector Test")
            run(repo, "config", "user.email", "selector@example.invalid")
            path = repo / "src/pinned.rs"
            path.parent.mkdir(parents=True)
            pinned = b"pinned now\n"
            path.write_bytes(pinned)
            run(repo, "add", "src/pinned.rs")
            run(repo, "commit", "-qm", "pinned")
            data, commit = exact_historical_bytes(repo, "src/pinned.rs", sha(pinned))
            self.assertEqual(data, pinned)
            self.assertEqual(commit, subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=repo, text=True).strip())

    def test_changed_current_source_selects_the_exact_older_pinned_blob(self):
        td, repo, pinned_commit, pinned = self.repo_with_commits()
        with td:
            with tempfile.TemporaryDirectory() as target_dir:
                target = Path(target_dir)
                selected = materialize(repo, target, "src/pinned.rs", sha(pinned))
                self.assertEqual(selected["source_commit"], pinned_commit)
                self.assertEqual(selected["materialized_sha256"], sha(pinned))
                self.assertNotEqual(selected["current_checkout_sha256"], selected["expected_sha256"])
                self.assertEqual((repo / "src/pinned.rs").read_bytes(), b"current changed bytes\n")
                self.assertEqual((target / "src/pinned.rs").read_bytes(), pinned)

    def test_changed_historical_blob_is_rejected_instead_of_using_current(self):
        td, repo, _, _ = self.repo_with_commits()
        with td, self.assertRaises(LookupError):
            exact_historical_bytes(repo, "src/pinned.rs", sha(b"not present in history"))


if __name__ == "__main__":
    unittest.main()
