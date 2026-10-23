#!/usr/bin/env python3
"""Fixtures for the merge sweeper's gate (.github/scripts/merge_sweeper.py)."""
from __future__ import annotations

import os
import sys
import unittest
from unittest import mock

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import merge_sweeper as ms  # noqa: E402

REPO = "aburan28/crypto"


def run(name, status="completed", conclusion="success"):
    return {"name": name, "status": status, "conclusion": conclusion}


def pr(number=1, labels=(), draft=False, base="main", head_repo=REPO, sha="abc"):
    return {"number": number, "title": f"PR {number}", "draft": draft,
            "labels": [{"name": l} for l in labels],
            "base": {"ref": base},
            "head": {"sha": sha, "repo": {"full_name": head_repo}}}


class CheckVerdict(unittest.TestCase):
    def test_all_green_passes(self):
        self.assertEqual(ms.check_verdict([run("a"), run("b", conclusion="skipped"),
                                           run("c", conclusion="neutral")], []),
                         ("pass", []))

    def test_no_checks_is_not_a_pass(self):
        self.assertEqual(ms.check_verdict([], [])[0], "none")

    def test_queued_check_is_pending(self):
        self.assertEqual(ms.check_verdict([run("a"), run("b", "queued", None)], []),
                         ("pending", ["b"]))

    def test_cancelled_and_timed_out_fail(self):
        for bad in ("cancelled", "timed_out", "failure", "action_required", "stale"):
            verdict, names = ms.check_verdict([run("a"), run("b", conclusion=bad)], [])
            self.assertEqual(verdict, "fail", bad)
            self.assertEqual(names, [f"b ({bad})"])

    def test_failure_outranks_pending(self):
        verdict, _ = ms.check_verdict([run("a", "queued", None),
                                       run("b", conclusion="failure")], [])
        self.assertEqual(verdict, "fail")

    def test_own_check_is_ignored(self):
        self.assertEqual(ms.check_verdict([run("a"), run("sweep", "in_progress", None)], []),
                         ("pass", []))

    def test_only_newest_commit_status_per_context_counts(self):
        statuses = [{"context": "ci/x", "state": "success"},
                    {"context": "ci/x", "state": "failure"}]
        self.assertEqual(ms.check_verdict([run("a")], statuses), ("pass", []))
        statuses.reverse()
        self.assertEqual(ms.check_verdict([run("a")], statuses)[0], "fail")


class BlockingReviews(unittest.TestCase):
    def test_latest_decisive_review_wins(self):
        reviews = [{"user": {"login": "r"}, "state": "CHANGES_REQUESTED"},
                   {"user": {"login": "r"}, "state": "COMMENTED"},
                   {"user": {"login": "s"}, "state": "CHANGES_REQUESTED"},
                   {"user": {"login": "s"}, "state": "APPROVED"}]
        self.assertEqual(ms.blocking_reviews(reviews), ["r"])


class Assess(unittest.TestCase):
    def setUp(self):
        self.runs, self.statuses, self.reviews = [run("a")], [], []
        self.detail = {"head": {"sha": "abc"}, "mergeable": True,
                       "mergeable_state": "clean"}

        def paged(path, key=None, max_pages=20):
            if "/check-runs" in path:
                return self.runs
            if "/statuses" in path:
                return self.statuses
            if "/reviews" in path:
                return self.reviews
            raise AssertionError(path)

        patches = [mock.patch.object(ms, "paged", side_effect=paged),
                   mock.patch.object(ms, "pr_detail", side_effect=lambda *_: self.detail)]
        for p in patches:
            p.start()
            self.addCleanup(p.stop)

    def bucket(self, p=None):
        return ms.assess(REPO, p or pr(), "main")[0]

    def test_green_clean_pr_is_ready(self):
        self.assertEqual(ms.assess(REPO, pr(), "main"), ("ready", ""))

    def test_holds(self):
        for label in sorted(ms.HOLD_LABELS):
            self.assertEqual(self.bucket(pr(labels=[label])), "held", label)
        self.assertEqual(self.bucket(pr(draft=True)), "held")
        self.assertEqual(self.bucket(pr(head_repo="someone/fork")), "held")
        self.assertEqual(self.bucket(pr(base="feature")), "held")

    def test_unclean_states_are_not_ready(self):
        for state, bucket in (("dirty", "conflict"), ("unstable", "pending"),
                              ("blocked", "pending"), ("unknown", "pending")):
            self.detail["mergeable_state"] = state
            self.assertEqual(self.bucket(), bucket, state)
        self.detail.update(mergeable=False, mergeable_state="clean")
        self.assertEqual(self.bucket(), "conflict")

    def test_head_moved_is_not_ready(self):
        self.detail["head"]["sha"] = "def"
        self.assertEqual(self.bucket(), "pending")

    def test_changes_requested_holds(self):
        self.reviews = [{"user": {"login": "r"}, "state": "CHANGES_REQUESTED"}]
        self.assertEqual(self.bucket(), "held")

    def test_red_or_missing_checks_are_not_ready(self):
        self.runs = [run("a", conclusion="cancelled")]
        self.assertEqual(self.bucket(), "fail")
        self.runs = []
        self.assertEqual(self.bucket(), "pending")


class Merge(unittest.TestCase):
    def test_merge_pins_the_checked_head_and_uses_a_merge_commit(self):
        with mock.patch.object(ms, "api") as api:
            self.assertEqual(ms.merge(REPO, pr(number=7, sha="f00")), "merged")
        api.assert_called_once_with("PUT", f"repos/{REPO}/pulls/7/merge",
                                    {"merge_method": "merge", "sha": "f00"})


if __name__ == "__main__":
    unittest.main()
