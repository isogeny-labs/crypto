#!/usr/bin/env python3
"""Tests for scripts/check_ic_leaderboard_paths.py: the glob forms it models, the
workflow text it reads, and that it refuses the forms it does not model."""
import sys
import tempfile
import unittest
from unittest.mock import patch
from contextlib import redirect_stderr, redirect_stdout
import io
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import check_ic_leaderboard_paths as chk  # noqa: E402


def match(pat: str, path: str) -> bool:
    return bool(chk.glob_regex(pat).match(path))


class Glob(unittest.TestCase):
    def test_literal_is_exact(self):
        self.assertTrue(match("docs/curves/registry.json", "docs/curves/registry.json"))
        self.assertFalse(match("docs/curves/registry.json", "docs/curves/registry.json.bak"))
        self.assertFalse(match("docs/curves/registry.json", "x/docs/curves/registry.json"))

    def test_dot_is_literal(self):
        self.assertFalse(match("a.json", "aXjson"))

    def test_star_stops_at_slash(self):
        self.assertTrue(match("runs/*/T01.params.json", "runs/k0n41/T01.params.json"))
        self.assertFalse(match("runs/*/T01.params.json", "runs/k0n41/M1/T01.params.json"))
        self.assertTrue(match("tests/ic_*.rs", "tests/ic_framework.rs"))
        self.assertFalse(match("tests/ic_*.rs", "tests/sub/ic_framework.rs"))

    def test_double_star_crosses_slashes(self):
        self.assertTrue(match("docs/ic/**", "docs/ic/runs/a/b.json"))
        self.assertTrue(match("docs/ic/**", "docs/ic/x"))
        self.assertFalse(match("docs/ic/**", "docs/icx/y"))

    def test_double_star_slash_matches_zero_or_more_directories(self):
        pat = "main/**/r*-candidate.price.json"
        self.assertTrue(match(pat, "main/k0n37/M1/r1-candidate.price.json"))
        self.assertTrue(match(pat, "main/r1-candidate.price.json"))
        self.assertFalse(match(pat, "main/k0n37/M1/r1-other.json"))

    def test_unmodelled_forms_are_refused(self):
        for pat in ("a?.json", "a+.json", "[ab].json", "!docs/**"):
            with self.assertRaises(SystemExit):
                chk.glob_regex(pat)


class Workflow(unittest.TestCase):
    def read(self, text: str, event: str) -> list[str] | None:
        with tempfile.TemporaryDirectory() as d:
            f = Path(d) / "w.yml"
            f.write_text(text)
            return chk.event_paths(f, event)

    TEXT = """name: x
on:
  pull_request:
    paths:
      - 'a/**'
      - "b/c.json"   # a comment
      - d/e.txt
  push:
    branches: [main]
    paths:
      - 'z/**'
  workflow_dispatch:
jobs:
  j:
    runs-on: ubuntu-latest
"""

    def test_each_event_has_its_own_list(self):
        self.assertEqual(self.read(self.TEXT, "pull_request"), ["a/**", "b/c.json", "d/e.txt"])
        self.assertEqual(self.read(self.TEXT, "push"), ["z/**"])

    def test_absent_event_is_distinct_from_unfiltered(self):
        self.assertIsNone(self.read(self.TEXT, "schedule"))
        self.assertEqual(self.read("on:\n  push:\n    branches: [main]\n", "push"), [])

    def test_job_name_cannot_invent_an_event(self):
        self.assertIsNone(self.read("on:\n  workflow_dispatch:\njobs:\n  push:\n    paths:\n      - 'z/**'\n", "push"))

    def test_empty_or_inline_paths_are_refused(self):
        for text in ("on:\n  push:\n    paths:\n", "on:\n  push:\n    paths: []\n"):
            with self.assertRaises(SystemExit):
                self.read(text, "push")

    def test_paths_ignore_is_refused(self):
        with self.assertRaises(SystemExit):
            self.read("on:\n  push:\n    paths-ignore:\n      - 'a/**'\n", "push")


class ConfiguredCoverage(unittest.TestCase):
    def check(self, events):
        with tempfile.TemporaryDirectory() as directory:
            workflow = Path(directory) / "w.yml"
            workflow.write_text("on:\n" + events)
            with patch.object(chk, "builder_reads", return_value=[chk.BUILDER, "research/new/summary.json"]), \
                    patch.object(sys, "argv", ["check", str(workflow)]), \
                    redirect_stdout(io.StringIO()), redirect_stderr(io.StringIO()):
                return chk.main()

    def test_owner_main_only_policy_keeps_complete_push_coverage(self):
        self.assertEqual(self.check("  push:\n    paths:\n      - 'scripts/build_ic_leaderboard.py'\n      - 'research/**'\n"), 0)

    def test_missing_push_input_is_still_refused(self):
        self.assertEqual(self.check("  push:\n    paths:\n      - 'scripts/build_ic_leaderboard.py'\n"), 1)

    def test_configured_pull_request_cannot_escape_coverage(self):
        self.assertEqual(self.check("  push:\n  pull_request:\n    paths:\n      - 'scripts/build_ic_leaderboard.py'\n"), 1)

    def test_unfiltered_push_covers_every_input(self):
        self.assertEqual(self.check("  push:\n    branches: [main]\n"), 0)

    def test_no_automatic_event_is_refused(self):
        self.assertEqual(self.check("  workflow_dispatch:\n"), 1)


if __name__ == "__main__":
    unittest.main()
