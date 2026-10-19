"""Focused controls for sparse-safe lab-browser source discovery."""

import os
import hashlib
import io
import pathlib
import subprocess
import tarfile
import tempfile
import unittest
from unittest.mock import patch

import build_lab_browser as browser


class RepositoryInputsTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.root = pathlib.Path(self.tmp.name)
        self.git("init", "-q")
        self.git("config", "user.email", "lab-browser-test@example.invalid")
        self.git("config", "user.name", "Lab browser test")
        (self.root / "tracked").mkdir()
        (self.root / "tracked" / "nested").mkdir()
        (self.root / "tracked" / "off.json").write_text('{"source":"head"}\n', encoding="utf-8")
        (self.root / "tracked" / "live.json").write_text('{"source":"old"}\n', encoding="utf-8")
        (self.root / "tracked" / "nested" / "inside.json").write_text(
            '{"source":"nested"}\n', encoding="utf-8"
        )
        self.git("add", "tracked")
        self.git("commit", "-qm", "fixture")
        self.off_oid = self.git("rev-parse", "HEAD:tracked/off.json", capture=True).strip()
        self.git("update-index", "--skip-worktree", "tracked/off.json")
        os.unlink(self.root / "tracked" / "off.json")
        (self.root / "tracked" / "live.json").write_text('{"source":"worktree"}\n', encoding="utf-8")
        (self.root / "tracked" / "untracked.json").write_text(
            '{"source":"untracked"}\n', encoding="utf-8"
        )
        (self.root / "tracked" / ".hidden.json").write_text(
            '{"source":"untracked-hidden"}\n', encoding="utf-8"
        )

    def tearDown(self):
        self.tmp.cleanup()

    def git(self, *args, capture=False):
        result = subprocess.run(
            ["git", *args],
            cwd=self.root,
            check=True,
            stdout=subprocess.PIPE if capture else subprocess.DEVNULL,
            text=True,
        )
        return result.stdout or ""

    def test_sparse_head_and_materialized_worktree_are_overlaid(self):
        source = browser.RepositoryInputs(str(self.root))
        self.assertEqual(source.git_env["GIT_NO_LAZY_FETCH"], "1")
        self.assertEqual(
            source.glob_files("tracked/*.json"),
            ["tracked/live.json", "tracked/off.json"],
        )
        self.assertEqual(source.read_text("tracked/off.json"), '{"source":"head"}\n')
        self.assertEqual(source.read_text("tracked/live.json"), '{"source":"worktree"}\n')
        self.assertEqual(source.size("tracked/off.json"), 18)
        self.assertTrue(source.is_directory("tracked"))
        self.assertEqual(source.child_directories("tracked"), ["nested"])
        self.assertFalse(source.exists("tracked/untracked.json"))
        self.assertTrue(browser._glob_match("tracked/off.json", "tracked/**/*.json"))
        self.assertTrue(
            browser._glob_match("tracked/nested/off.json", "tracked/**/*.json")
        )
        self.assertFalse(
            browser._glob_match("tracked/nested/off.md", "tracked/**/*.json")
        )
        self.assertFalse(
            browser._glob_match("tracked/.hidden.json", "tracked/**/*.json")
        )
        self.assertFalse(
            browser._glob_match("tracked/.hidden/off.json", "tracked/**/*.json")
        )
        self.assertTrue(
            browser._glob_match("tracked/.hidden.json", "tracked/.hidden.json")
        )

    def test_missing_sparse_blob_fails_closed_with_its_path(self):
        source = browser.RepositoryInputs(str(self.root))
        loose = self.root / ".git" / "objects" / self.off_oid[:2] / self.off_oid[2:]
        loose.unlink()
        with self.assertRaisesRegex(OSError, r"HEAD:tracked/off\.json"):
            source.read_text("tracked/off.json")

    def test_non_git_root_fails_closed(self):
        with tempfile.TemporaryDirectory() as root:
            with self.assertRaisesRegex(RuntimeError, "cannot enumerate committed"):
                browser.RepositoryInputs(root)

    def archive_fixture(self):
        relative = "tracked/runs.tar.xz"
        data = b'{"schema":"ecbench.session/v1","session_id":"sealed"}\n'
        with tarfile.open(self.root / relative, "w:xz") as tree:
            member = tarfile.TarInfo("runs/session/session.json")
            member.size = len(data)
            tree.addfile(member, io.BytesIO(data))
        digest = hashlib.sha256((self.root / relative).read_bytes()).hexdigest()
        (self.root / (relative + ".sha256")).write_text(digest + "  runs.tar.xz\n")
        self.git("add", relative, relative + ".sha256")
        self.git("commit", "-qm", "sealed archive")
        return relative, data

    def test_archived_session_ignores_stale_untracked_extraction(self):
        archive, expected = self.archive_fixture()
        extracted = self.root / "tracked/runs/session/session.json"
        extracted.parent.mkdir(parents=True)
        extracted.write_bytes(b"stale local extraction")
        with patch.object(browser, "ECBENCH_ARCHIVES", (archive,)):
            source = browser.ArchivedStudyInputs(str(self.root))
        member = "tracked/runs/session/session.json"
        self.assertEqual(source.read_bytes(member), expected)
        self.assertEqual(source.size(member), len(expected))
        self.assertEqual(source.glob_files("tracked/runs/**/session.json"), [member])
        self.assertTrue(source.is_directory("tracked/runs/session"))
        self.assertEqual(source.archive_of[member], archive)

    def test_sparse_archive_is_read_from_its_committed_blob(self):
        archive, expected = self.archive_fixture()
        self.git("update-index", "--skip-worktree", archive, archive + ".sha256")
        (self.root / archive).unlink()
        (self.root / (archive + ".sha256")).unlink()
        with patch.object(browser, "ECBENCH_ARCHIVES", (archive,)):
            source = browser.ArchivedStudyInputs(str(self.root))
        self.assertEqual(source.read_bytes("tracked/runs/session/session.json"), expected)

    def test_corrupt_archive_fails_before_indexing_any_session(self):
        archive, _ = self.archive_fixture()
        with (self.root / archive).open("ab") as file:
            file.write(b"changed")
        with patch.object(browser, "ECBENCH_ARCHIVES", (archive,)):
            with self.assertRaisesRegex(ValueError, "archive checksum mismatch"):
                browser.ArchivedStudyInputs(str(self.root))


if __name__ == "__main__":
    unittest.main()
