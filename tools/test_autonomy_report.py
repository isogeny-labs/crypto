#!/usr/bin/env python3
"""Tests for tools/autonomy_report.py: evidence requirement, content-addressed
ids, tamper detection, unsupervised-success logic, Wilson interval and report files.

    python3 tools/test_autonomy_report.py
"""
from __future__ import annotations

import contextlib
import datetime as dt
import hashlib
import io
import json
import sys
import tempfile
import unittest
import xml.etree.ElementTree as ET
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import autonomy_report as ar  # noqa: E402

NOW = dt.datetime(2026, 11, 20, tzinfo=dt.timezone.utc)      # 5 days after the seed episodes
LATER = dt.datetime(2026, 12, 20, tzinfo=dt.timezone.utc)    # window elapsed


def run(d: Path, *args: str) -> tuple[int, str]:
    out = io.StringIO()
    with contextlib.redirect_stdout(out), contextlib.redirect_stderr(io.StringIO()):
        try:
            code = ar.main(["--dir", str(d), *args])
        except SystemExit as exc:
            code = exc.code if isinstance(exc.code, int) else 2
    return code, out.getvalue()


def sha(s: str) -> str:
    return hashlib.sha256(s.encode()).hexdigest()


class AutonomyTest(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.d = Path(self.tmp.name) / "episodes"

    def tearDown(self):
        self.tmp.cleanup()

    def episode(self, *extra: str, result="success", merged=True, ci=True, ended="2026-11-15T01:00:00Z",
                task_class="docs-tooling") -> str:
        args = ["add-episode", "--session", "s1", "--task", "t", "--task-class", task_class,
                "--started", "2026-11-15T00:00:00Z", "--ended", ended, "--result", result,
                "--evidence", "https://example.invalid/pr/1", *extra]
        if merged:
            args.append("--merged")
        if ci:
            args.append("--ci-green-at-merge")
        code, out = run(self.d, *args)
        self.assertEqual(code, 0, out)
        return out.strip()

    def iv(self, eid: str, cls: str, text: str = "m", classifier="haiku") -> str:
        code, out = run(self.d, "add-intervention", "--episode", eid, "--ts", "2026-11-15T00:30:00Z",
                        "--class", cls, "--text-sha256", sha(text), "--classifier", classifier)
        self.assertEqual(code, 0, out)
        return out.strip()

    def check(self, eid: str, name: str, value: str):
        code, out = run(self.d, "set-check", "--episode", eid, "--check", name, "--value", value,
                        "--evidence", "git log")
        self.assertEqual(code, 0, out)

    def state(self, eid: str, at=NOW) -> str:
        return ar.episode_state(eid, ar.load(self.d), at)["state"]

    def test_supersedes_replaces_episode_in_summary(self):
        a = self.episode("--pr", "7", result="partial", merged=False)
        b = self.episode("--pr", "7", "--supersedes", a)
        s = ar.summarize(ar.load(self.d), None, NOW)
        self.assertEqual(s["episodes"], 1)
        code, _ = run(self.d, "add-episode", "--session", "s", "--task", "t", "--task-class", "other",
                      "--started", "2026-11-15T00:00:00Z", "--ended", "2026-11-15T01:00:00Z",
                      "--result", "success", "--supersedes", "E-000000000000", "--evidence", "e")
        self.assertNotEqual(code, 0)
        self.assertTrue(b.startswith("E-"))

    def test_nudge_fails_and_classifier_rank(self):
        a = self.episode()
        self.check(a, "reverted", "false")
        self.assertEqual(self.state(a), "unsupervised_success")
        self.iv(a, "redirect", "continue", "haiku")
        self.assertEqual(self.state(a), "fail")
        self.iv(a, "approve", "continue", "opus")     # opus outranks haiku
        self.assertEqual(self.state(a), "unsupervised_success")
        self.iv(a, "nudge", "continue", "sonnet")     # sonnet does not outrank opus
        self.assertEqual(self.state(a), "unsupervised_success")
        self.iv(a, "nudge", "continue", "human")      # human outranks all
        self.assertEqual(self.state(a), "fail")

    def test_evidence_required(self):
        code, _ = run(self.d, "add-episode", "--session", "s", "--task", "t", "--task-class", "other",
                      "--started", "2026-11-15T00:00:00Z", "--ended", "2026-11-15T01:00:00Z",
                      "--result", "success")
        self.assertNotEqual(code, 0)
        code, _ = run(self.d, "add-episode", "--session", "s", "--task", "t", "--task-class", "other",
                      "--started", "2026-11-15T00:00:00Z", "--ended", "2026-11-15T01:00:00Z",
                      "--result", "success", "--evidence", "  ")
        self.assertNotEqual(code, 0)

    def test_content_addressing_and_tamper(self):
        eid = self.episode()
        self.assertTrue(eid.startswith("E-"))
        self.assertEqual(run(self.d, "verify")[0], 0)
        path = self.d / f"{eid}.json"
        body = json.loads(path.read_text())
        self.assertEqual(ar.entry_id(body), eid)
        body["result"] = "fail"
        path.write_text(json.dumps(body))
        code, out = run(self.d, "verify")
        self.assertEqual(code, 1)
        self.assertIn("does not match id", out)

    def test_dangling_reference_rejected(self):
        code, _ = run(self.d, "add-intervention", "--episode", "E-000000000000", "--ts",
                      "2026-11-15T00:30:00Z", "--class", "redirect", "--text-sha256", sha("x"),
                      "--classifier", "rule")
        self.assertNotEqual(code, 0)

    def test_text_hash_only(self):
        eid = self.episode()
        iid = self.iv(eid, "hook", "stop-hook:untracked-files", "rule")
        body = json.loads((self.d / f"{iid}.json").read_text())
        self.assertEqual(body["text_sha256"], sha("stop-hook:untracked-files"))
        self.assertNotIn("stop-hook", json.dumps(body).replace(body["text_sha256"], ""))
        code, _ = run(self.d, "add-intervention", "--episode", eid, "--ts", "2026-11-15T00:30:00Z",
                      "--class", "hook", "--text-sha256", "not-a-hash", "--classifier", "rule")
        self.assertNotEqual(code, 0)

    def test_success_needs_revert_check(self):
        eid = self.episode()
        self.assertEqual(self.state(eid), "pending")          # window not elapsed
        self.assertEqual(self.state(eid, LATER), "pending")   # no check recorded: data missing
        self.check(eid, "reverted", "false")
        self.assertEqual(self.state(eid), "unsupervised_success")

    def test_redirect_fails_unblock_does_not(self):
        a = self.episode()
        self.check(a, "reverted", "false")
        self.iv(a, "unblock", "token")
        self.iv(a, "approve", "ok")
        self.iv(a, "hook", "h", "rule")
        self.assertEqual(self.state(a), "unsupervised_success")
        self.iv(a, "redirect", "wrong")
        self.assertEqual(self.state(a), "fail")

    def test_human_label_overrides_classifier(self):
        a = self.episode()
        self.check(a, "reverted", "false")
        self.iv(a, "redirect", "msg", "haiku")
        self.assertEqual(self.state(a), "fail")
        self.iv(a, "question", "msg", "human")
        self.assertEqual(self.state(a), "unsupervised_success")
        s = ar.summarize(ar.load(self.d), None, NOW)
        self.assertEqual(s["classifier_accuracy"]["audited"], 1)
        self.assertEqual(s["classifier_accuracy"]["agree"], 0)

    def test_revert_and_validator_fail_latest_check_wins(self):
        a = self.episode()
        self.check(a, "reverted", "false")
        self.assertEqual(self.state(a), "unsupervised_success")
        self.check(a, "reverted", "true")
        self.assertEqual(self.state(a), "fail")
        b = self.episode("--pr", "2")
        self.check(b, "reverted", "false")
        self.check(b, "validator_blocking", "true")
        self.assertEqual(self.state(b), "fail")

    def test_ci_and_result_and_open_pr(self):
        a = self.episode(ci=False)
        self.check(a, "reverted", "false")
        self.assertEqual(self.state(a), "fail")
        self.assertEqual(self.state(self.episode(result="abandoned", merged=False)), "fail")
        self.assertEqual(self.state(self.episode(result="partial", merged=False)), "pending")
        self.assertEqual(self.state(self.episode(merged=False)), "pending")

    def test_wilson(self):
        self.assertEqual(ar.wilson(0, 0), (0.0, 1.0))
        lo, hi = ar.wilson(10, 10)
        self.assertGreater(lo, 0.65)
        self.assertEqual(hi, 1.0)
        lo, hi = ar.wilson(5, 10)
        self.assertAlmostEqual(lo, 0.2366, places=3)
        self.assertAlmostEqual(hi, 0.7634, places=3)

    def test_self_recovery_and_cost(self):
        a = self.episode("--incident", "ci-failure:SELF", "--incident", "blocked-command:HUMAN",
                         "--tokens", "1000", "--usd", "2.5")
        self.check(a, "reverted", "false")
        s = ar.summarize(ar.load(self.d), None, NOW)
        self.assertEqual((s["incidents"]["self"], s["incidents"]["total"]), (1, 2))
        self.assertEqual(s["cost"]["tokens_per_success"], 1000)
        self.assertEqual(s["cost"]["usd_per_success"], 2.5)

    def test_report_writes_files_and_svg_parses(self):
        a = self.episode("--pr", "1")
        self.check(a, "reverted", "false")
        b = self.episode("--pr", "2", ended="2026-11-23T01:00:00Z", task_class="lean")
        self.iv(b, "redirect")
        self.episode("--pr", "3", merged=False, result="partial")
        out = Path(self.tmp.name) / "report"
        code, text = run(self.d, "report", "--out", str(out))
        self.assertEqual(code, 0)
        self.assertTrue((out / "AUTONOMY.md").exists())
        self.assertTrue((out / "autonomy.svg").exists())
        self.assertIn("Unsupervised success", text)
        self.assertEqual((out / "AUTONOMY.md").read_text(), text)
        root = ET.fromstring((out / "autonomy.svg").read_text())
        self.assertTrue(root.tag.endswith("svg"))
        self.assertEqual(run(self.d, "verify")[0], 0)

    def test_empty_report_svg_parses(self):
        out = Path(self.tmp.name) / "report"
        self.assertEqual(run(self.d, "report", "--out", str(out))[0], 0)
        ET.fromstring((out / "autonomy.svg").read_text())

    def test_since_filter(self):
        self.episode(ended="2026-11-15T01:00:00Z")
        self.episode(ended="2026-11-25T01:00:00Z")
        s = ar.summarize(ar.load(self.d), "2026-11-20", NOW)
        self.assertEqual(s["episodes"], 1)

    def test_audit_sample_deterministic(self):
        eid = self.episode()
        for i in range(30):
            self.iv(eid, "approve", f"m{i}")
        _, a = run(self.d, "audit-sample", "--rate", "0.1", "--seed", "7")
        _, b = run(self.d, "audit-sample", "--rate", "0.1", "--seed", "7")
        self.assertEqual(a, b)
        self.assertGreaterEqual(len(a.split()), 1)
        self.assertLess(len(a.split()), 15)

    def test_classify_prompt(self):
        _, out = run(self.d, "classify-prompt")
        for word in ar.CLASSES + ("text_sha256", "reason"):
            self.assertIn(word, out)


if __name__ == "__main__":
    unittest.main()
