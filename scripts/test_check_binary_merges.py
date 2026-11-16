#!/usr/bin/env python3
"""Tests for scripts/check_binary_merges.py on throwaway repositories.

    python3 scripts/test_check_binary_merges.py
"""
from __future__ import annotations

import os
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import check_binary_merges as cbm  # noqa: E402

PDF_A = b"%PDF-1.3\n\x00\x01 version A\n%%EOF\n"
PDF_B = b"%PDF-1.3\n\x00\x02 version B, longer\n%%EOF\n"


class Repo(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name)
        self.cwd = os.getcwd()
        os.chdir(self.root)
        self.addCleanup(os.chdir, self.cwd)
        self.git("init", "-q", "-b", "main")
        self.git("config", "user.email", "test@example.invalid")
        self.git("config", "user.name", "test")
        self.git("config", "commit.gpgsign", "false")

    def git(self, *args: str) -> str:
        return subprocess.run(["git", *args], check=True, capture_output=True, text=True).stdout.strip()

    def write(self, path: str, data: bytes) -> None:
        (self.root / path).parent.mkdir(parents=True, exist_ok=True)
        (self.root / path).write_bytes(data)

    def commit(self, message: str) -> str:
        self.git("add", "-A")
        self.git("commit", "-q", "-m", message)
        return self.git("rev-parse", "HEAD")

    def conflicting_branches(self, path: str = "report.pdf", a: bytes = PDF_A, b: bytes = PDF_B) -> str:
        """main and side both change `path` from a common base; returns the base."""
        self.write(path, b"%PDF-1.3\n\x00 base\n%%EOF\n")
        base = self.commit("base")
        self.git("checkout", "-q", "-b", "side")
        self.write(path, b)
        self.commit("side changes it")
        self.git("checkout", "-q", "main")
        self.write(path, a)
        self.commit("main changes it")
        return base

    def merge_resolving(self, path: str, data: bytes, message: str = "Merge side") -> str:
        """Merge side into main, resolve `path` to `data`, commit the merge."""
        subprocess.run(["git", "merge", "--no-ff", "--no-commit", "side"], capture_output=True)
        self.write(path, data)
        self.git("add", path)
        self.git("commit", "-q", "-m", message)
        return self.git("rev-parse", "HEAD")

    def check(self, base: str) -> list[str]:
        return [v for m in cbm.merge_commits(base, "HEAD") for v in cbm.violations(m)]


class BinaryMerges(Repo):
    def test_union_concatenation_is_refused_and_named(self):
        base = self.conflicting_branches()
        self.merge_resolving("report.pdf", PDF_A + b"\n" + PDF_B)
        problems = self.check(base)
        self.assertEqual(len(problems), 1)
        self.assertIn("report.pdf", problems[0])
        self.assertIn("concatenated", problems[0])
        self.assertEqual(cbm.main([base, "HEAD"]), 1)

    def test_taking_one_side_passes(self):
        for side in (PDF_A, PDF_B):
            with self.subTest(side=side[:20]):
                self.setUp()
                base = self.conflicting_branches()
                self.merge_resolving("report.pdf", side)
                self.assertEqual(self.check(base), [])
                self.assertEqual(cbm.main([base, "HEAD"]), 0)

    def test_regenerated_binary_needs_the_trailer(self):
        rebuilt = b"%PDF-1.3\n\x00\x03 rebuilt from merged sources\n%%EOF\n"
        base = self.conflicting_branches()
        self.merge_resolving("report.pdf", rebuilt)
        problems = self.check(base)
        self.assertEqual(len(problems), 1)
        self.assertNotIn("concatenated", problems[0])

        self.setUp()
        base = self.conflicting_branches()
        self.merge_resolving("report.pdf", rebuilt,
                             "Merge side\n\nBinary-Regenerated: report.pdf\n")
        self.assertEqual(self.check(base), [])

    def test_trailer_covers_only_the_named_file(self):
        base = self.conflicting_branches("a.pdf")
        # A second conflicting binary on the same branches.
        self.git("checkout", "-q", "side")
        self.write("b.pdf", PDF_B)
        self.commit("side adds b")
        self.git("checkout", "-q", "main")
        self.write("b.pdf", PDF_A)
        self.commit("main adds b")
        subprocess.run(["git", "merge", "--no-ff", "--no-commit", "side"], capture_output=True)
        self.write("a.pdf", b"%PDF-1.3\n\x00 rebuilt a\n%%EOF\n")
        self.write("b.pdf", PDF_A + PDF_B)
        self.git("add", "a.pdf", "b.pdf")
        self.git("commit", "-q", "-m", "Merge side\n\nBinary-Regenerated: a.pdf\n")
        problems = self.check(base)
        self.assertEqual([p.split(" ")[1].rstrip(":") for p in problems], ["b.pdf"])

    def test_parents_that_agree_leave_nothing_to_combine(self):
        # Both sides carry the same binary; the merge commit edits it. That is an
        # ordinary change made in a merge, not a combination of two versions.
        self.write("report.pdf", PDF_A)
        self.write("x.txt", b"x\n")
        base = self.commit("base")
        self.git("checkout", "-q", "-b", "side")
        self.write("x.txt", b"x side\n")
        self.commit("side")
        self.git("checkout", "-q", "main")
        self.write("y.txt", b"y\n")
        self.commit("main")
        self.merge_resolving("report.pdf", PDF_B)
        self.assertEqual(self.check(base), [])

    def test_text_files_are_out_of_scope(self):
        base = self.conflicting_branches("notes.txt", b"main line\n", b"side line\n")
        self.merge_resolving("notes.txt", b"main line\nside line\n")
        self.assertEqual(self.check(base), [])

    def test_gitattributes_binary_counts_as_binary(self):
        self.write(".gitattributes", b"*.dat binary\n")
        base = self.conflicting_branches("table.dat", b"plain text A\n", b"plain text B\n")
        self.merge_resolving("table.dat", b"plain text A\nplain text B\n")
        self.assertEqual(len(self.check(base)), 1)

    def test_merge_deleting_the_file_passes(self):
        base = self.conflicting_branches()
        subprocess.run(["git", "merge", "--no-ff", "--no-commit", "side"], capture_output=True)
        self.git("rm", "-q", "-f", "report.pdf")
        self.git("commit", "-q", "-m", "Merge side, drop the report")
        self.assertEqual(self.check(base), [])

    def test_range_without_merges_passes(self):
        self.write("report.pdf", PDF_A)
        base = self.commit("base")
        self.write("report.pdf", PDF_B)
        self.commit("ordinary edit")
        self.assertEqual(cbm.main([base, "HEAD"]), 0)


if __name__ == "__main__":
    unittest.main()
