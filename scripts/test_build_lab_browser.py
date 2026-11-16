"""Focused controls for sparse-safe sources and exact cover joins."""

import hashlib
import os
import pathlib
import subprocess
import tempfile
import unittest
from unittest import mock

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
        (self.root / "tracked" / "deleted.json").write_text(
            '{"source":"deleted"}\n', encoding="utf-8"
        )
        (self.root / "tracked" / "nested" / "inside.json").write_text(
            '{"source":"nested"}\n', encoding="utf-8"
        )
        self.git("add", "tracked")
        self.git("commit", "-qm", "fixture")
        self.off_oid = self.git("rev-parse", "HEAD:tracked/off.json", capture=True).strip()
        self.git("update-index", "--skip-worktree", "tracked/off.json")
        os.unlink(self.root / "tracked" / "off.json")
        os.unlink(self.root / "tracked" / "deleted.json")
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
        self.addCleanup(source.close)
        self.assertEqual(source.git_env["GIT_NO_LAZY_FETCH"], "1")
        self.assertEqual(
            source.glob_files("tracked/*.json"),
            ["tracked/live.json", "tracked/off.json"],
        )
        self.assertEqual(
            source.glob_files("tracked/**/*.json"),
            ["tracked/live.json", "tracked/nested/inside.json", "tracked/off.json"],
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
        self.assertTrue(browser._glob_match("tracked", "tracked/**"))
        self.assertTrue(browser._glob_match("tracked/nested", "tracked/**"))
        self.assertTrue(browser._glob_match("tracked/nested/inside.json", "**/*.json"))
        self.assertFalse(browser._glob_match("tracked/.hidden", "tracked/**"))
        self.assertFalse(browser._glob_match("tracked/.hidden/inside.json", "**/*.json"))

    def test_missing_sparse_blob_fails_closed_with_its_path(self):
        source = browser.RepositoryInputs(str(self.root))
        self.addCleanup(source.close)
        loose = self.root / ".git" / "objects" / self.off_oid[:2] / self.off_oid[2:]
        loose.unlink()
        with self.assertRaisesRegex(OSError, r"HEAD:tracked/off\.json"):
            source.read_text("tracked/off.json")

    def test_core_leaderboard_is_scanned_beyond_the_general_file_limit(self):
        core_id = "IC1N83Ckb1a1fb183264PDP4rootRCguidedLAgaussTDdirectISO0h6f30a77abf9e"
        other_id = "IC1N83Ckb1a1fb191232PDP4rootRCguidedLAgaussTDdirectISO0h37b9b2d5dc7a"
        directory = self.root / "docs" / "ic"
        directory.mkdir(parents=True)
        (directory / "leaderboard.json").write_text('{"candidate":"' + core_id + '"}\n')
        (directory / "large-note.json").write_text('{"candidate":"' + other_id + '"}\n')
        self.git("add", "docs")
        self.git("commit", "-qm", "candidate inputs")
        source = browser.RepositoryInputs(str(self.root))
        self.addCleanup(source.close)
        with mock.patch.object(browser, "MAX_SCAN_BYTES", 32):
            rows = browser.candidate_identities(source)
        self.assertEqual([row["candidate_id"] for row in rows], [core_id])
        self.assertEqual(rows[0]["where"], ["docs/ic/leaderboard.json"])

    def test_non_git_root_fails_closed(self):
        with tempfile.TemporaryDirectory() as root:
            with self.assertRaisesRegex(RuntimeError, "cannot enumerate committed"):
                browser.RepositoryInputs(root)

    def test_batch_reads_preserve_empty_binary_and_newline_paths(self):
        fixtures = {"empty.json": b"", "line\nbreak.json": b"first\nsecond\0tail"}
        for name, data in fixtures.items():
            (self.root / name).write_bytes(data)
            self.git("add", "--", name)
        self.git("commit", "-qm", "batch fixture")
        self.git("update-index", "--skip-worktree", "--", *fixtures)
        for name in fixtures:
            (self.root / name).unlink()
        with browser.RepositoryInputs(str(self.root)) as source:
            # Exercise both uncached HEAD requests and resolved object IDs.
            for name, data in fixtures.items():
                self.assertEqual(source.read_bytes(name), data)
            source.prefetch_sizes(list(fixtures))
            for _ in range(3):
                for name, data in fixtures.items():
                    self.assertEqual(source.read_bytes(name), data)
                    self.assertEqual(source.size(name), len(data))
            (self.root / "empty.json").write_bytes(b"materialized local edit")
            self.assertEqual(source.read_bytes("empty.json"), b"materialized local edit")
            self.assertEqual(source.size("empty.json"), 23)
            reader = source._blob_reader
        self.assertIsNotNone(reader.poll())

    def test_batch_reads_preserve_head_when_the_index_differs(self):
        blob = subprocess.check_output(
            ["git", "hash-object", "-w", "--stdin"], cwd=self.root,
            input=b"different staged bytes\n",
        ).decode().strip()
        self.git("update-index", "--cacheinfo", f"100644,{blob},tracked/off.json")
        self.git("update-index", "--skip-worktree", "tracked/off.json")
        with browser.RepositoryInputs(str(self.root)) as source:
            source.prefetch_sizes(["tracked/off.json"])
            self.assertEqual(source.size("tracked/off.json"), 18)
            self.assertEqual(source.read_text("tracked/off.json"), '{"source":"head"}\n')

    def test_batch_read_reports_object_loss_after_metadata_lookup(self):
        with browser.RepositoryInputs(str(self.root)) as source:
            source.prefetch_sizes(["tracked/off.json"])
            loose = self.root / ".git" / "objects" / self.off_oid[:2] / self.off_oid[2:]
            loose.unlink()
            with self.assertRaisesRegex(OSError, "tracked/off.json"):
                source.read_bytes("tracked/off.json")
            self.assertIsNone(source._blob_reader)

    def test_build_closes_batch_child_when_generation_fails(self):
        readers = []
        def fail_after_read(source):
            source.read_bytes("tracked/off.json")
            readers.append(source._blob_reader)
            raise ValueError("generation failed")
        with mock.patch.object(browser, "ROOT", str(self.root)), \
             mock.patch.object(browser, "_build", side_effect=fail_after_read):
            with self.assertRaisesRegex(ValueError, "generation failed"):
                browser.build()
        self.assertEqual(len(readers), 1)
        self.assertIsNotNone(readers[0].poll())

    def test_glob_prefix_pruning_matches_full_scan(self):
        for name in ("abc.json", "[x].json", "λ.json", ".hidden.json"):
            (self.root / "tracked" / name).write_text("{}")
        self.git("add", "-f", "tracked")
        with browser.RepositoryInputs(str(self.root)) as source:
            for pattern in ("tracked/abc.json", "tracked/ab*.json",
                            "tracked/[[]x].json", "tracked/[a-z]*.json",
                            "tracked/**/*.json", "**/inside.json",
                            "tracked/.hidden.json", "tracked/λ.json",
                            "missing/**"):
                with self.subTest(pattern=pattern):
                    expected = sorted(path for path in source.tracked_files
                                      if browser._glob_match(path, pattern)
                                      and source.exists(path))
                    self.assertEqual(source.glob_files(pattern), expected)


class CoverJoinTests(unittest.TestCase):
    def test_older_catalog_keeps_new_model_visible_as_unassessed(self):
        curves = [
            {"slug": "icv1-a", "model_json": '{"model":"a"}'},
            {"slug": "icv1-b", "model_json": '{"model":"b"}'},
        ]
        report = {"schema_version": "curve-covers/v1", "registry_sha256": "a" * 64,
                  "curves": [{"slug": "icv1-a", "model_sha256":
                              hashlib.sha256(curves[0]["model_json"].encode()).hexdigest()}]}
        with mock.patch.object(browser, "sha256_file", return_value="b" * 64):
            self.assertEqual(browser.attach_covers(curves, report), {"icv1-a"})
            self.assertEqual(curves[0]["hyperelliptic_cover"], report["curves"][0])
            self.assertIsNone(curves[1]["hyperelliptic_cover"])
            bad = {**report, "curves": [{**report["curves"][0], "model_sha256": "0" * 64}]}
            with self.assertRaisesRegex(ValueError, "cover model digest mismatch"):
                browser.attach_covers(curves, bad)
            with self.assertRaisesRegex(ValueError, "current cover report must have one finding"):
                browser.attach_covers(curves, {**report, "registry_sha256": "b" * 64})


if __name__ == "__main__":
    unittest.main()
