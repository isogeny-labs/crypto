#!/usr/bin/env python3
"""Real local promisor-repository regression tests; no network or cloud writes."""
from __future__ import annotations

import os
from pathlib import Path
import subprocess
import tempfile
import unittest

import artifact_prefetch as ap


class BloblessPrefetch(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.source = self.root / "source"
        self.source.mkdir()
        self.g(self.source, "init", "-q")
        self.g(self.source, "config", "user.name", "fixture")
        self.g(self.source, "config", "user.email", "fixture@example.invalid")
        self.g(self.source, "config", "uploadpack.allowFilter", "true")
        self.g(self.source, "config", "uploadpack.allowAnySHA1InWant", "true")
        (self.source / "base.txt").write_text("before\n")
        self.g(self.source, "add", ".")
        self.g(self.source, "commit", "-qm", "base")
        self.base = self.g(self.source, "rev-parse", "HEAD").decode().strip()
        for name, text in [
            ("base.txt", "modified\n"),
            ("space name.log", "new log\n"),
            ("line\nname.bin", "newline path\n"),
            ("other.bin", "another new blob\n"),
        ]:
            (self.source / name).write_text(text)
        os.symlink("../outside-data", self.source / "link")
        self.g(self.source, "add", ".")
        self.g(self.source, "update-index", "--add", "--cacheinfo",
               f"160000,{self.base},gitlink")
        self.g(self.source, "commit", "-qm", "candidate")
        self.head = self.g(self.source, "rev-parse", "HEAD").decode().strip()
        self.clone = self.root / "candidate"
        self.g(self.root, "clone", "-q", "--filter=blob:none", "--no-checkout",
               self.source.as_uri(), str(self.clone))

    @staticmethod
    def g(root, *args):
        result = subprocess.run(
            ["git", "-C", str(root), *args], capture_output=True, check=True,
            env=dict(os.environ, GIT_NO_LAZY_FETCH="1"), timeout=30,
        )
        return result.stdout

    def snapshot(self):
        return {
            "head": self.g(self.clone, "rev-parse", "HEAD"),
            "config": (self.clone / ".git/config").read_bytes(),
            "refs": self.g(self.clone, "for-each-ref", "--format=%(refname) %(objectname)"),
            "index": (self.clone / ".git/index").read_bytes()
            if (self.clone / ".git/index").exists() else None,
        }

    def test_real_missing_blobs_are_fetched_in_batches_without_checkout(self):
        before = self.snapshot()
        blobs = ap.changed_blobs(self.clone, self.base, self.head)
        self.assertEqual(len(blobs), 5, "four regular files plus symlink; exclude gitlink")
        self.assertEqual(len(ap.missing_blobs(self.clone, blobs)), 5)
        result = ap.prefetch(self.clone, self.base, self.head, batch_size=2)
        self.assertEqual(result, {"changed_blobs": 5, "missing_blobs": 5, "batches": 3})
        self.assertEqual(ap.missing_blobs(self.clone, blobs), [])
        self.assertEqual(self.snapshot(), before)
        self.assertFalse((self.clone / "space name.log").exists())
        self.assertFalse((self.clone / "line\nname.bin").exists())
        self.assertFalse((self.clone / "link").is_symlink())
        self.assertFalse((self.root / "outside-data").exists())

    def test_cached_objects_need_no_further_fetch(self):
        ap.prefetch(self.clone, self.base, self.head)
        self.g(self.clone, "remote", "set-url", "origin", "file:///nonexistent-prefetch-fixture")
        result = ap.prefetch(self.clone, self.base, self.head)
        self.assertEqual(result, {"changed_blobs": 5, "missing_blobs": 0, "batches": 0})

    def test_empty_diff_needs_no_fetch(self):
        self.g(self.clone, "remote", "set-url", "origin", "file:///nonexistent-prefetch-fixture")
        self.assertEqual(ap.prefetch(self.clone, self.head, self.head),
                         {"changed_blobs": 0, "missing_blobs": 0, "batches": 0})

    def test_fetch_failure_is_not_reported_as_a_pass(self):
        self.g(self.clone, "remote", "set-url", "origin", "file:///nonexistent-prefetch-fixture")
        with self.assertRaisesRegex(RuntimeError, "Git artifact prefetch failed"):
            ap.prefetch(self.clone, self.base, self.head)

    def test_invalid_revisions_and_batch_limits_fail(self):
        with self.assertRaises(RuntimeError):
            ap.prefetch(self.clone, "not-a-revision", self.head)
        for size in (0, 257):
            with self.assertRaises(ValueError):
                ap.prefetch(self.clone, self.base, self.head, batch_size=size)


if __name__ == "__main__":
    unittest.main()
