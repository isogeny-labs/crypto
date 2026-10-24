"""Focused controls for sparse-safe lab-browser source discovery."""

import hashlib
import os
import pathlib
import subprocess
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


class HistoricalCoverJoinTests(unittest.TestCase):
    def test_expanded_registry_preserves_only_matching_frozen_certificates(self):
        curves = [
            {"slug": "icv1-old", "model_json": '{"a":1}'},
            {"slug": "icv1-new", "model_json": '{"a":2}'},
        ]
        report = {
            "schema_version": "curve-covers/v1",
            "registry_sha256": "frozen-registry",
            "curves": [{"slug": "icv1-old", "model_sha256": hashlib.sha256(
                curves[0]["model_json"].encode()).hexdigest(), "status": "verified_over_declared_field"}],
        }
        with (patch.object(browser, "sha256_file", return_value="current-registry"),
              patch.object(browser, "FROZEN_COVERS_REGISTRY_SHA256", "frozen-registry"),
              patch.object(browser, "FROZEN_COVERS_COUNT", 1)):
            self.assertEqual(browser.attach_covers(curves, report), {"icv1-old"})
            self.assertEqual(curves[0]["hyperelliptic_cover"], report["curves"][0])
            self.assertIsNone(curves[1]["hyperelliptic_cover"])
            report["curves"][0]["model_sha256"] = "0" * 64
            with self.assertRaisesRegex(ValueError, "cover model digest mismatch"):
                browser.attach_covers(curves, report)
            report["curves"][0]["model_sha256"] = hashlib.sha256(
                curves[0]["model_json"].encode()).hexdigest()
            report["registry_sha256"] = "current-registry"
            with self.assertRaisesRegex(ValueError, "current cover report omits"):
                browser.attach_covers(curves, report)


if __name__ == "__main__":
    unittest.main()
