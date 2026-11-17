#!/usr/bin/env python3
"""Tests for tools/lessons.py: status derivation, evidence requirement,
content-addressed ids, tamper detection and outcome statistics.

    python3 tools/test_lessons.py
"""
from __future__ import annotations

import contextlib
import io
import json
import sys
import tempfile
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import lessons  # noqa: E402


def run(d: Path, *args: str) -> tuple[int, str]:
    out = io.StringIO()
    with contextlib.redirect_stdout(out), contextlib.redirect_stderr(io.StringIO()):
        try:
            code = lessons.main(["--dir", str(d), *args])
        except SystemExit as exc:
            code = exc.code if isinstance(exc.code, int) else 2
    return code, out.getvalue()


class LessonsTest(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.d = Path(self.tmp.name) / "entries"

    def tearDown(self):
        self.tmp.cleanup()

    def add(self) -> str:
        code, out = run(self.d, "add", "--scope", "ecbench-measure", "--trigger", "t", "--action", "a",
                        "--evidence", "https://example.invalid/job/1")
        self.assertEqual(code, 0)
        return out.strip()

    def test_evidence_required(self):
        code, _ = run(self.d, "add", "--trigger", "t", "--action", "a")
        self.assertNotEqual(code, 0)

    def test_status_progression(self):
        lid = self.add()
        self.assertTrue(lid.startswith("L-"))
        entries = lessons.load(self.d)
        self.assertEqual(lessons.lesson_status(lid, entries)["status"], "candidate")
        run(self.d, "test", "--lesson", lid, "--outcome", "confirm", "--evidence", "e1")
        # the same evidence twice counts once
        run(self.d, "test", "--lesson", lid, "--outcome", "confirm", "--evidence", "e1", "--context", "again")
        self.assertEqual(lessons.lesson_status(lid, lessons.load(self.d))["status"], "candidate")
        run(self.d, "test", "--lesson", lid, "--outcome", "confirm", "--evidence", "e2")
        self.assertEqual(lessons.lesson_status(lid, lessons.load(self.d))["status"], "adopted")
        run(self.d, "test", "--lesson", lid, "--outcome", "refute", "--evidence", "e3")
        self.assertEqual(lessons.lesson_status(lid, lessons.load(self.d))["status"], "contested")
        run(self.d, "retire", "--lesson", lid, "--reason", "r", "--evidence", "e4")
        self.assertEqual(lessons.lesson_status(lid, lessons.load(self.d))["status"], "retired")

    def test_unknown_lesson_rejected(self):
        code, _ = run(self.d, "test", "--lesson", "L-000000000000", "--outcome", "confirm", "--evidence", "e")
        self.assertNotEqual(code, 0)

    def test_tamper_detected(self):
        lid = self.add()
        self.assertEqual(run(self.d, "verify")[0], 0)
        path = self.d / f"{lid}.json"
        body = json.loads(path.read_text())
        body["action"] = "something else"
        path.write_text(json.dumps(body))
        code, out = run(self.d, "verify")
        self.assertEqual(code, 1)
        self.assertIn("does not match id", out)

    def test_outcome_stats(self):
        lid = self.add()
        for i, result in enumerate(("success", "success", "fail")):
            code, _ = run(self.d, "outcome", "--scope", "ecbench-measure", "--task", f"task-{i}",
                          "--result", result, "--applied", lid, "--evidence", f"ev-{i}")
            self.assertEqual(code, 0)
        s = lessons.outcome_stats(lessons.load(self.d), None)
        self.assertEqual(s["ecbench-measure"]["n"], 3)
        self.assertEqual(s["ecbench-measure"]["success"], 2)
        lo, hi = s["ecbench-measure"]["ci"]
        self.assertTrue(0.0 <= lo <= 0.5 <= hi <= 1.0)

    def test_brief_scoped(self):
        self.add()
        _, out = run(self.d, "brief", "--scope", "ecbench-measure")
        self.assertIn("candidate", out)
        _, out = run(self.d, "brief", "--scope", "validator")
        self.assertIn("none recorded", out)

    def test_wilson(self):
        lo, hi = lessons.wilson(0, 0)
        self.assertEqual((lo, hi), (0.0, 1.0))
        lo, hi = lessons.wilson(10, 10)
        self.assertGreater(lo, 0.65)
        self.assertEqual(hi, 1.0)


if __name__ == "__main__":
    unittest.main()
