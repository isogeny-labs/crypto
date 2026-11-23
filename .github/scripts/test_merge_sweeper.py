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


def run(name, status="completed", conclusion="success", started="2026-10-23T10:00:00Z"):
    return {"name": name, "status": status, "conclusion": conclusion, "started_at": started}


def pr(number=1, labels=(), draft=False, base="main", head_repo=REPO, sha="abc",
       ref="feature", created="2026-10-20T00:00:00Z", updated="2026-10-20T00:00:00Z"):
    return {"number": number, "title": f"PR {number}", "draft": draft,
            "labels": [{"name": l} for l in labels], "node_id": f"PR_{number}",
            "base": {"ref": base}, "created_at": created, "updated_at": updated,
            "head": {"sha": sha, "ref": ref, "repo": {"full_name": head_repo}}}


T = ms.parse_time


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

    def test_budget_counts_only_started_checks(self):
        checks = [run("queued", "queued", None), run("running", "in_progress", None),
                  run("finished"), run("skipped", conclusion="skipped"),
                  run("neutral", conclusion="neutral")]
        self.assertEqual(ms.checks_ran(checks), 2)

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


class ReviewThreads(unittest.TestCase):
    @staticmethod
    def page(nodes, more=False, cursor=None):
        return {"data": {"repository": {"pullRequest": {"reviewThreads": {
            "nodes": nodes, "pageInfo": {"hasNextPage": more, "endCursor": cursor},
        }}}}}

    def test_every_page_is_read_and_closed_threads_do_not_block(self):
        first = self.page([{"id": "closed", "isResolved": True}], True, "next")
        second = self.page([{"id": "open", "isResolved": False}])
        with mock.patch.object(ms, "api", side_effect=[first, second]) as api:
            self.assertEqual(ms.unresolved_threads(REPO, 7), 1)
        calls = api.call_args_list
        self.assertEqual([call.args[:2] for call in calls], [("POST", "graphql")] * 2)
        self.assertEqual([call.args[2]["variables"]["cursor"] for call in calls], [None, "next"])
        self.assertTrue(all(call.args[2]["variables"]["number"] == 7 for call in calls))

    def test_empty_thread_set_is_clear(self):
        with mock.patch.object(ms, "api", return_value=self.page([])):
            self.assertEqual(ms.unresolved_threads(REPO, 7), 0)

    def test_api_errors_and_missing_data_fail_closed(self):
        responses = [{"errors": [{"message": "denied"}]}, {"data": {"repository": None}},
                     {"data": []}, {"data": {"repository": {"pullRequest": []}}},
                     self.page([{"id": "bad", "isResolved": "false"}])]
        for response in responses:
            with self.subTest(response=response), mock.patch.object(ms, "api", return_value=response):
                with self.assertRaises(RuntimeError):
                    ms.unresolved_threads(REPO, 7)

    def test_repeated_pagination_cursor_fails_closed(self):
        page = self.page([], True, "stalled")
        with mock.patch.object(ms, "api", side_effect=[page, page]):
            with self.assertRaisesRegex(RuntimeError, "did not advance"):
                ms.unresolved_threads(REPO, 7)


class Assess(unittest.TestCase):
    def setUp(self):
        self.runs = [run("gate"), run("size"), run("artifacts")]
        self.statuses, self.reviews, self.threads = [], [], 0
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
                   mock.patch.object(ms, "pr_detail", side_effect=lambda *_: self.detail),
                   mock.patch.object(ms, "unresolved_threads", side_effect=lambda *_: self.threads)]
        for p in patches:
            p.start()
            self.addCleanup(p.stop)

    def bucket(self, p=None):
        return ms.assess(REPO, p or pr(), "main")[0]

    def test_green_clean_pr_is_ready(self):
        self.assertEqual(ms.assess(REPO, pr(), "main"), ("ready", ""))

    def test_holds(self):
        for label in sorted(ms.HOLD_LABELS - {"needs-human"}):
            self.assertEqual(self.bucket(pr(labels=[label])), "held", label)
        self.assertEqual(self.bucket(pr(labels=["needs-human"])), "needs-human")
        self.assertEqual(self.bucket(pr(draft=True)), "draft")
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


    def test_check_budget(self):
        self.runs = [run(f"c{i}") for i in range(31)]
        self.assertEqual(self.bucket(), "over-budget")
        self.runs = [run("gate"), run("size"), run("artifacts")]
        self.runs += [run(f"c{i}") for i in range(27)] + [run("x", conclusion="skipped")]
        self.assertEqual(self.bucket(), "ready")
        self.runs = [run(f"queued{i}", "queued", None) for i in range(31)]
        self.assertEqual(self.bucket(), "pending")

    def test_full_tier_marker_is_required_after_the_tiers_land(self):
        since = T("2026-10-23T06:00:00Z")
        required = [run("gate"), run("size"), run("artifacts")]
        self.runs = required + [run("full-tier", conclusion="skipped")]
        self.assertEqual(ms.assess(REPO, pr(), "main", since=since)[0], "pending")
        self.runs = required + [run("full-tier")]
        self.assertEqual(ms.assess(REPO, pr(), "main", since=since)[0], "ready")
        self.runs = [run(name, started="2026-10-22T00:00:00Z")
                     for name in ("gate", "size", "artifacts")]
        self.assertEqual(ms.assess(REPO, pr(), "main", since=since)[0], "ready")
        self.runs = required
        self.assertEqual(ms.assess(REPO, pr(), "main", since=None)[0], "ready")


    def test_arbitrary_green_check_does_not_replace_merge_gates(self):
        self.runs = [run("lint")]
        self.assertEqual(self.bucket(), "pending")

    def test_each_merge_gate_must_succeed(self):
        for missing in ("gate", "size", "artifacts"):
            with self.subTest(missing=missing):
                self.runs = [run(name) for name in ("gate", "size", "artifacts")
                             if name != missing]
                self.assertEqual(self.bucket(), "pending")
                self.runs.append(run(missing, conclusion="skipped"))
                self.assertEqual(self.bucket(), "pending")

    def test_duplicate_gate_failure_cannot_be_hidden_by_a_success(self):
        self.runs.append(run("gate", conclusion="failure"))
        self.assertEqual(self.bucket(), "fail")

    def test_unresolved_thread_holds_even_when_github_reports_clean(self):
        self.threads = 1
        self.assertEqual(self.bucket(), "held")

    def test_review_thread_read_error_never_becomes_ready(self):
        with mock.patch.object(ms, "unresolved_threads", side_effect=RuntimeError("review unavailable")):
            with self.assertRaisesRegex(RuntimeError, "review unavailable"):
                self.bucket()


class Capacity(unittest.TestCase):
    NOW = T("2026-10-23T12:00:00Z")

    def plan(self, prs, buckets, **kw):
        kw.setdefault("max_ready", 2)
        return [(k, p["number"]) for k, p, _ in
                ms.plan_capacity(prs, buckets, **kw)]

    def test_red_prs_are_parked_and_held_ones_are_left(self):
        prs = [pr(i) for i in range(1, 6)]
        buckets = {1: "fail", 2: "conflict", 3: "over-budget", 4: "needs-human", 5: "held"}
        self.assertEqual(self.plan(prs, buckets),
                         [("park", 1), ("park", 2), ("park", 3), ("park", 4)])

    def test_newest_ready_beyond_the_cap_wait(self):
        prs = [pr(i, created=f"2026-10-2{i}T00:00:00Z") for i in (3, 1, 2)]
        self.assertEqual(self.plan(prs, {1: "pending", 2: "ready", 3: "pending"}),
                         [("wait", 3)])

    def test_drafts_are_never_changed_by_capacity_planning(self):
        prs = [pr(1, draft=True, labels=["ci:waiting"]),
               pr(2, draft=True, labels=["ci:parked"])]
        self.assertEqual(self.plan(prs, {1: "draft", 2: "draft"}), [])

    def test_waiting_labels_prioritize_ready_prs_without_demoting_them(self):
        prs = [pr(1, labels=["ci:waiting"]),
               pr(2, created="2026-10-22T00:00:00Z", labels=["ci:waiting"]),
               pr(3, created="2026-10-23T00:00:00Z")]
        buckets = {1: "ready", 2: "ready", 3: "ready"}
        self.assertEqual(self.plan(prs, buckets),
                         [("wait", 3), ("resume", 1), ("resume", 2)])

    def test_capacity_labels_clear_when_status_changes(self):
        prs = [pr(1, labels=["ci:parked"]),
               pr(2, labels=["ci:waiting"])]
        buckets = {1: "ready", 2: "ready"}
        self.assertEqual(self.plan(prs, buckets), [("unpark", 1), ("resume", 2)])

    def test_api_errors_do_not_clear_existing_capacity_labels(self):
        prs = [pr(1, labels=["ci:parked"]), pr(2, labels=["ci:waiting"])]
        self.assertEqual(self.plan(prs, {1: "error", 2: "error"}), [])

    def test_applying_capacity_actions_does_not_change_draft_state(self):
        target = pr(number=7)
        with mock.patch.object(ms, "add_label") as add_label, \
                mock.patch.object(ms, "comment") as comment:
            self.assertEqual(ms.apply_action(REPO, "park", target, "failing checks",
                                             max_ready=40), "done")
        add_label.assert_called_once_with(REPO, 7, ms.L_PARKED)
        comment.assert_called_once()
        self.assertFalse(target["draft"])


class Merge(unittest.TestCase):
    def test_merge_pins_the_checked_head_and_uses_a_merge_commit(self):
        with mock.patch.object(ms, "api", return_value={"merged": True, "sha": "new"}) as api:
            self.assertEqual(ms.merge(REPO, pr(number=7, sha="f00")), "merged")
        api.assert_called_once_with("PUT", f"repos/{REPO}/pulls/7/merge",
                                    {"merge_method": "merge", "sha": "f00"})


    def test_merge_requires_an_explicit_success_confirmation(self):
        for response in (None, {}, {"merged": False, "message": "refused"},
                         {"merged": "true"}, {"merged": 1}):
            with self.subTest(response=response), mock.patch.object(ms, "api", return_value=response):
                self.assertTrue(ms.merge(REPO, pr()).startswith("refused:"))


if __name__ == "__main__":
    unittest.main()
