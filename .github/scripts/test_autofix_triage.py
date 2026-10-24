"""Offline tests for the autofix queue throttles in autofix_triage.py.

No network, no `gh`, no git: GitHub is replaced by a fixture object that
answers the handful of REST paths triage() reads and records every call, so the
tests can also assert which expensive calls were *not* made.

Run: python3 -m unittest discover -s .github/scripts -p 'test_autofix_triage.py' -v
"""
from __future__ import annotations

import argparse
import contextlib
import io
import json
import os
import tempfile
import unittest
from pathlib import Path
from unittest import mock

import autofix_triage as at

REPO = "o/r"
BOT = {"type": "Bot", "login": "github-actions[bot]"}
HUMAN = {"type": "User", "login": "someone"}
OLD = "2020-01-01T00:00:00Z"


def pr(number, *, draft=False, labels=(), fork=False, sha=None, state="open",
       mergeable_state="unstable", mergeable=True):
    return {
        "number": number, "title": f"PR {number}", "state": state, "draft": draft,
        "labels": [{"name": l} for l in labels],
        "head": {"ref": f"codex/pr{number}", "sha": sha or f"sha{number}",
                 "repo": {"full_name": "fork/r" if fork else REPO}},
        "base": {"ref": "main"},
        "mergeable": mergeable, "mergeable_state": mergeable_state,
    }


def run(name, conclusion="failure", status="completed", rid=1):
    return {"name": name, "id": rid, "status": status, "conclusion": conclusion,
            "html_url": f"https://example.invalid/{rid}", "app": {"slug": "github-actions"}}


def attempt_comment(n=1, *, user=BOT):
    return {"user": user, "author_association": "NONE",
            "body": f"**autofix** (`m`) pushed abc\n\n<!-- autofix-attempt n={n} -->"}


class FakeGitHub:
    """Answers the REST paths autofix_triage uses and logs every call."""

    def __init__(self, prs, *, checks=None, behind=None, comments=None,
                 queued=0, in_progress=0, fail=()):
        self.prs = {p["number"]: p for p in prs}
        self.checks = checks or {}
        self.behind = behind or {}
        self.comments = comments or {}
        self.counts = {"queued": queued, "in_progress": in_progress}
        self.fail = tuple(fail)       # substrings of paths that raise RuntimeError
        self.calls: list[str] = []
        self.labels: list[tuple] = []
        self.posted: list[tuple] = []

    def __call__(self, path, *args, paginate=False, max_pages=10):
        self.calls.append(path)
        if any(f in path for f in self.fail):
            raise RuntimeError(f"gh api {path}: boom")
        r = f"repos/{REPO}/"
        assert path.startswith(r), path
        rest = path[len(r):]
        if rest.startswith("pulls?"):
            return list(self.prs.values())
        if rest.startswith("pulls/"):
            return self.prs[int(rest.split("/")[1])]
        if rest.startswith("compare/"):
            sha = rest.split("...")[1].split("?")[0]
            return {"behind_by": self.behind.get(sha, 0)}
        if rest.startswith("commits/") and "/check-runs" in rest:
            return self.checks.get(rest.split("/")[1], [])
        if rest.startswith("commits/"):
            return {"commit": {"committer": {"date": OLD}}}
        if rest.startswith("issues/") and "/comments" in rest:
            return self.comments.get(int(rest.split("/")[1]), [])
        if rest.startswith("actions/runs?"):
            status = rest.split("status=")[1].split("&")[0]
            return {"total_count": self.counts[status]}
        raise AssertionError(f"unexpected gh path {path}")

    def called(self, needle):
        return [c for c in self.calls if needle in c]


def args(**kw):
    base = dict(repo=REPO, pr=None, max=5, min_age=45, max_attempts=4,
                max_behind=50, max_queued=40, include_drafts=False, dry_run=False)
    base.update(kw)
    return argparse.Namespace(**base)


class TriageCase(unittest.TestCase):
    def setUp(self):
        env = {k: v for k, v in os.environ.items() if not k.startswith(("AUTOFIX_", "GITHUB_"))}
        patcher = mock.patch.dict(os.environ, env, clear=True)
        patcher.start()
        self.addCleanup(patcher.stop)
        for target in ("time.sleep",):
            p = mock.patch(target)
            p.start()
            self.addCleanup(p.stop)

    def triage(self, gh, **kw):
        def set_labels(repo, number, add=(), remove=(), dry=False):
            gh.labels.append((number, tuple(add), tuple(remove), dry))

        def comment(repo, number, body, dry=False):
            gh.posted.append((number, body, dry))

        with contextlib.ExitStack() as stack:
            stack.enter_context(mock.patch.object(at, "gh", gh))
            stack.enter_context(mock.patch.object(at, "set_labels", set_labels))
            stack.enter_context(mock.patch.object(at, "comment", comment))
            stack.enter_context(mock.patch.object(
                at, "conflict_files", lambda base, head: ["src/a.rs"]))
            stack.enter_context(contextlib.redirect_stdout(io.StringIO()))
            return at.triage(args(**kw))


class CountAttempts(unittest.TestCase):
    def test_markers_sum_and_legacy_reports_count_once(self):
        comments = [
            attempt_comment(1), attempt_comment(2),
            {"user": BOT, "author_association": "NONE",
             "body": "**autofix** could not fix this PR"},          # legacy
            {"user": BOT, "author_association": "NONE",
             "body": "**autofix:** Sonnet already attempted"},       # hand-off
            {"user": BOT, "author_association": "NONE", "body": "<!-- autofix-attempt n=0 -->"},
        ]
        self.assertEqual(at.count_attempts(comments), 4)

    def test_human_comments_cannot_change_the_count(self):
        spoof = {"user": HUMAN, "author_association": "MEMBER",
                 "body": "<!-- autofix-attempt n=9 -->"}
        legacy = {"user": HUMAN, "author_association": "MEMBER",
                  "body": "**autofix** looks fine"}
        self.assertEqual(at.count_attempts([attempt_comment(1), spoof, legacy]), 1)

    def test_maintainer_reset_restarts_the_count(self):
        reset = {"user": HUMAN, "author_association": "COLLABORATOR",
                 "body": "/autofix reset please"}
        got = at.count_attempts([attempt_comment(2), reset, attempt_comment(1)])
        self.assertEqual(got, 1)

    def test_reset_from_outsider_or_bot_is_ignored(self):
        outsider = {"user": HUMAN, "author_association": "NONE", "body": "/autofix reset"}
        bot = {"user": BOT, "author_association": "MEMBER", "body": "/autofix reset"}
        self.assertEqual(at.count_attempts([attempt_comment(2), outsider, bot]), 2)

    def test_empty(self):
        self.assertEqual(at.count_attempts([]), 0)


class QueueDepth(unittest.TestCase):
    def test_pause_threshold_is_strictly_above_limit(self):
        self.assertIsNone(at.pause_reason(40, 40))
        self.assertIn("41", at.pause_reason(41, 40) or "")
        self.assertIn("AUTOFIX_MAX_QUEUED=40", at.pause_reason(41, 40) or "")

    def test_own_run_is_discounted_and_zero_disables(self):
        self.assertIsNone(at.pause_reason(41, 40, own=1))
        self.assertIsNone(at.pause_reason(10_000, 0))

    def test_queue_depth_sums_queued_and_in_progress(self):
        gh = FakeGitHub([], queued=12, in_progress=30)
        with mock.patch.object(at, "gh", gh):
            self.assertEqual(at.queue_depth(REPO), 42)
        self.assertEqual(len(gh.calls), 2)


class Selection(TriageCase):
    def test_failing_pr_is_queued_with_expected_fields(self):
        gh = FakeGitHub([pr(1)], checks={"sha1": [run("tests"), run("lint-fmt", rid=2)]})
        q = self.triage(gh)
        self.assertEqual([x["number"] for x in q], [1])
        self.assertEqual(q[0]["tier"], "sonnet")           # "tests" is not mechanical
        self.assertEqual(q[0]["head_ref"], "codex/pr1")
        self.assertFalse(q[0]["conflict"])

    def test_mechanical_failure_goes_to_haiku(self):
        gh = FakeGitHub([pr(1)], checks={"sha1": [run("rustfmt")]})
        self.assertEqual(self.triage(gh)[0]["tier"], "haiku")

    def test_drafts_forks_and_opt_out_labels_are_never_queued(self):
        prs = [pr(1, draft=True), pr(2, fork=True), pr(3, labels=["autofix:off"]),
               pr(4, labels=["needs-human"]), pr(5, labels=["autofix:running"]),
               pr(6, state="closed")]
        gh = FakeGitHub(prs, checks={f"sha{i}": [run("tests")] for i in range(1, 7)})
        self.assertEqual(self.triage(gh), [])
        self.assertEqual(gh.called("check-runs"), [])
        self.assertEqual(gh.called("/compare/"), [])

    def test_fork_pr_is_not_queued_even_when_requested_explicitly(self):
        gh = FakeGitHub([pr(2, fork=True)], checks={"sha2": [run("tests")]})
        self.assertEqual(self.triage(gh, pr=[2]), [])

    def test_drafts_only_with_the_explicit_flag(self):
        gh = FakeGitHub([pr(1, draft=True)], checks={"sha1": [run("tests")]})
        self.assertEqual(self.triage(gh), [])
        self.assertEqual(len(self.triage(gh, include_drafts=True)), 1)

    def test_green_pr_is_skipped_without_the_commit_lookup(self):
        gh = FakeGitHub([pr(1)], checks={"sha1": [run("tests", "success")]})
        self.assertEqual(self.triage(gh), [])
        self.assertEqual(gh.called("/commits/sha1?"), [])
        self.assertEqual([c for c in gh.calls if c.endswith("commits/sha1")], [])

    def test_green_pr_with_ladder_labels_gets_them_reset(self):
        gh = FakeGitHub([pr(1, labels=["autofix:haiku", "autofix:sonnet"])],
                        checks={"sha1": [run("tests", "success")]})
        self.assertEqual(self.triage(gh), [])
        self.assertEqual(gh.labels, [(1, (), ("autofix:haiku", "autofix:sonnet"), False)])

    def test_pending_checks_wait(self):
        gh = FakeGitHub([pr(1)], checks={"sha1": [run("a"), run("b", None, "in_progress")]})
        self.assertEqual(self.triage(gh), [])

    def test_quiet_period_uses_head_commit_age(self):
        gh = FakeGitHub([pr(1)], checks={"sha1": [run("tests")]})
        with mock.patch.object(at.time, "time", lambda: 1577836800 + 60):  # OLD + 1 min
            self.assertEqual(self.triage(gh), [])
        self.assertEqual(len(self.triage(gh)), 1)

    def test_max_per_sweep(self):
        prs = [pr(i) for i in range(1, 6)]
        gh = FakeGitHub(prs, checks={f"sha{i}": [run("tests")] for i in range(1, 6)})
        self.assertEqual(len(self.triage(gh, max=2)), 2)

    def test_conflicting_pr_is_queued(self):
        gh = FakeGitHub([pr(1, mergeable=False, mergeable_state="dirty")], checks={})
        q = self.triage(gh)
        self.assertEqual(len(q), 1)
        self.assertTrue(q[0]["conflict"])

    def test_sonnet_label_hands_off_to_a_human(self):
        gh = FakeGitHub([pr(1, labels=["autofix:sonnet"])], checks={"sha1": [run("tests")]})
        self.assertEqual(self.triage(gh), [])
        self.assertEqual(gh.labels[0][:2], (1, ("needs-human",)))
        self.assertTrue(gh.posted[0][1].startswith("**autofix:** "))

    def test_haiku_label_escalates_to_sonnet(self):
        gh = FakeGitHub([pr(1, labels=["autofix:haiku"])], checks={"sha1": [run("rustfmt")]})
        self.assertEqual(self.triage(gh)[0]["tier"], "sonnet")


class BehindMain(TriageCase):
    def test_far_behind_pr_is_skipped_before_any_check_or_commit_call(self):
        gh = FakeGitHub([pr(1)], behind={"sha1": 51}, checks={"sha1": [run("tests")]})
        self.assertEqual(self.triage(gh), [])
        self.assertEqual(gh.called("check-runs"), [])
        self.assertEqual(gh.called("comments"), [])

    def test_exactly_at_the_limit_is_still_eligible(self):
        gh = FakeGitHub([pr(1)], behind={"sha1": 50}, checks={"sha1": [run("tests")]})
        self.assertEqual(len(self.triage(gh)), 1)

    def test_far_behind_conflicting_pr_is_skipped_too(self):
        gh = FakeGitHub([pr(1, mergeable=False, mergeable_state="dirty")], behind={"sha1": 400})
        self.assertEqual(self.triage(gh), [])

    def test_limit_is_configurable_and_zero_disables(self):
        gh = FakeGitHub([pr(1)], behind={"sha1": 120}, checks={"sha1": [run("tests")]})
        self.assertEqual(len(self.triage(gh, max_behind=200)), 1)
        self.assertEqual(len(self.triage(gh, max_behind=0)), 1)
        self.assertEqual(self.triage(gh, max_behind=100), [])

    def test_unknown_distance_fails_closed(self):
        gh = FakeGitHub([pr(1)], checks={"sha1": [run("tests")]}, fail=("/compare/",))
        self.assertEqual(self.triage(gh), [])
        self.assertEqual(gh.called("check-runs"), [])

    def test_explicit_request_bypasses_the_behind_skip(self):
        gh = FakeGitHub([pr(1)], behind={"sha1": 900}, checks={"sha1": [run("tests")]})
        self.assertEqual(len(self.triage(gh, pr=[1])), 1)
        self.assertEqual(gh.called("/compare/"), [])

    def test_env_default_is_50(self):
        self.assertEqual(at.env_int("AUTOFIX_MAX_BEHIND", at.DEFAULT_MAX_BEHIND), 50)
        with mock.patch.dict(os.environ, {"AUTOFIX_MAX_BEHIND": "7"}):
            self.assertEqual(at.env_int("AUTOFIX_MAX_BEHIND", 50), 7)
        with mock.patch.dict(os.environ, {"AUTOFIX_MAX_BEHIND": "junk"}), \
                contextlib.redirect_stderr(io.StringIO()):
            self.assertEqual(at.env_int("AUTOFIX_MAX_BEHIND", 50), 50)


class TrustCleanState(TriageCase):
    def test_off_by_default_so_check_runs_decide(self):
        gh = FakeGitHub([pr(1, mergeable_state="clean")], checks={"sha1": [run("tests")]})
        self.assertEqual(len(self.triage(gh)), 1)

    def test_when_enabled_clean_pr_costs_no_check_run_call(self):
        gh = FakeGitHub([pr(1, mergeable_state="clean")], checks={"sha1": [run("tests")]})
        with mock.patch.dict(os.environ, {"AUTOFIX_TRUST_CLEAN_STATE": "true"}):
            self.assertEqual(self.triage(gh), [])
        self.assertEqual(gh.called("check-runs"), [])


class AttemptCap(TriageCase):
    def test_pr_at_the_cap_gets_needs_human_and_is_not_queued(self):
        comments = {1: [attempt_comment(2), attempt_comment(2)]}
        gh = FakeGitHub([pr(1)], checks={"sha1": [run("tests")]}, comments=comments)
        self.assertEqual(self.triage(gh, max_attempts=4), [])
        self.assertEqual(gh.labels, [(1, ("needs-human",), (), False)])
        self.assertIn("4 of 4", gh.posted[0][1])
        self.assertIn("/autofix reset", gh.posted[0][1])
        self.assertNotIn("autofix-attempt", gh.posted[0][1])   # hand-off is not an attempt

    def test_below_the_cap_is_queued(self):
        gh = FakeGitHub([pr(1)], checks={"sha1": [run("tests")]},
                        comments={1: [attempt_comment(2), attempt_comment(1)]})
        self.assertEqual(len(self.triage(gh, max_attempts=4)), 1)

    def test_cap_survives_the_green_label_reset(self):
        # Labels are gone (PR went green once) but the comments remain.
        gh = FakeGitHub([pr(1, labels=[])], checks={"sha1": [run("tests")]},
                        comments={1: [attempt_comment(1)] * 4})
        self.assertEqual(self.triage(gh), [])
        self.assertEqual(gh.labels[0][1], ("needs-human",))

    def test_reset_comment_reopens_the_budget(self):
        reset = {"user": HUMAN, "author_association": "OWNER", "body": "/autofix reset"}
        gh = FakeGitHub([pr(1)], checks={"sha1": [run("tests")]},
                        comments={1: [attempt_comment(4), reset]})
        self.assertEqual(len(self.triage(gh)), 1)

    def test_zero_disables_the_cap_and_skips_the_comment_call(self):
        gh = FakeGitHub([pr(1)], checks={"sha1": [run("tests")]},
                        comments={1: [attempt_comment(50)]})
        self.assertEqual(len(self.triage(gh, max_attempts=0)), 1)
        self.assertEqual(gh.called("comments"), [])

    def test_comment_history_only_fetched_for_prs_that_would_be_queued(self):
        gh = FakeGitHub([pr(1), pr(2)], checks={"sha1": [run("tests", "success")],
                                                "sha2": [run("tests")]})
        self.triage(gh)
        self.assertEqual(len(gh.called("comments")), 1)
        self.assertIn("/issues/2/", gh.called("comments")[0])

    def test_unreadable_history_fails_closed(self):
        gh = FakeGitHub([pr(1)], checks={"sha1": [run("tests")]}, fail=("/comments",))
        self.assertEqual(self.triage(gh), [])
        self.assertEqual(gh.labels, [])

    def test_truncated_history_counts_as_over_the_cap(self):
        gh = FakeGitHub([pr(1)], checks={"sha1": [run("tests")]},
                        comments={1: [{"user": HUMAN, "author_association": "NONE",
                                       "body": "x"}] * 1000})
        self.assertEqual(self.triage(gh), [])
        self.assertEqual(gh.labels[0][1], ("needs-human",))

    def test_dry_run_flows_through_to_label_and_comment(self):
        gh = FakeGitHub([pr(1)], checks={"sha1": [run("tests")]},
                        comments={1: [attempt_comment(4)]})
        self.triage(gh, dry_run=True)
        self.assertTrue(gh.labels[0][3])
        self.assertTrue(gh.posted[0][2])


class Main(TriageCase):
    def run_main(self, gh, *argv, env=None):
        with tempfile.TemporaryDirectory() as tmp:
            out = Path(tmp) / "out"
            out.write_text("")
            full = {"GITHUB_OUTPUT": str(out), "GITHUB_REPOSITORY": REPO, **(env or {})}
            with mock.patch.dict(os.environ, full), \
                    mock.patch.object(at, "gh", gh), \
                    mock.patch.object(at, "set_labels", lambda *a, **k: None), \
                    mock.patch.object(at, "comment", lambda *a, **k: None), \
                    mock.patch.object(at, "conflict_files", lambda *a: []), \
                    mock.patch("sys.argv", ["autofix_triage.py", *argv]), \
                    contextlib.redirect_stdout(io.StringIO()) as stdout, \
                    contextlib.redirect_stderr(io.StringIO()):
                rc = at.main()
            written = dict(l.split("=", 1) for l in out.read_text().splitlines())
            return rc, written, stdout.getvalue()

    def test_deep_queue_pauses_the_sweep_before_listing_any_pr(self):
        gh = FakeGitHub([pr(1)], checks={"sha1": [run("tests")]}, queued=30, in_progress=20)
        rc, out, log = self.run_main(gh)
        self.assertEqual(rc, 0)
        self.assertEqual((out["matrix"], out["count"], out["paused"]), ("[]", "0", "true"))
        self.assertIn("paused", log)
        self.assertIn("50", log)
        self.assertEqual(gh.called("pulls"), [])

    def test_shallow_queue_sweeps_normally(self):
        gh = FakeGitHub([pr(1)], checks={"sha1": [run("tests")]}, behind={"sha1": 0},
                        queued=3, in_progress=4)
        rc, out, _ = self.run_main(gh, "--max-attempts", "0")
        self.assertEqual((out["count"], out["paused"]), ("1", "false"))
        self.assertEqual(json.loads(out["matrix"])[0]["number"], 1)

    def test_threshold_comes_from_env(self):
        gh = FakeGitHub([pr(1)], checks={"sha1": [run("tests")]}, behind={"sha1": 0},
                        queued=5, in_progress=5)
        _, out, _ = self.run_main(gh, env={"AUTOFIX_MAX_QUEUED": "9", "AUTOFIX_MAX_ATTEMPTS": "0"})
        self.assertEqual(out["paused"], "true")
        _, out, _ = self.run_main(gh, env={"AUTOFIX_MAX_QUEUED": "10", "AUTOFIX_MAX_ATTEMPTS": "0"})
        self.assertEqual(out["paused"], "false")

    def test_default_threshold_is_40_with_own_run_discounted(self):
        gh = FakeGitHub([], queued=0, in_progress=41)
        _, out, _ = self.run_main(gh, env={"GITHUB_RUN_ID": "1"})
        self.assertEqual(out["paused"], "false")          # 41 - own run = 40, not above 40
        gh = FakeGitHub([], queued=0, in_progress=42)
        _, out, _ = self.run_main(gh, env={"GITHUB_RUN_ID": "1"})
        self.assertEqual(out["paused"], "true")

    def test_explicit_pr_ignores_the_pause(self):
        gh = FakeGitHub([pr(1)], checks={"sha1": [run("tests")]}, queued=500)
        _, out, _ = self.run_main(gh, "--pr", "1", "--max-attempts", "0")
        self.assertEqual((out["count"], out["paused"]), ("1", "false"))
        self.assertEqual(gh.called("actions/runs"), [])

    def test_zero_disables_the_pause(self):
        gh = FakeGitHub([], queued=500)
        _, out, _ = self.run_main(gh, "--max-queued", "0")
        self.assertEqual(out["paused"], "false")
        self.assertEqual(gh.called("actions/runs"), [])

    def test_queue_api_failure_fails_open_with_a_warning(self):
        gh = FakeGitHub([], fail=("actions/runs",))
        rc, out, log = self.run_main(gh)
        self.assertEqual((rc, out["paused"], out["count"]), (0, "false", "0"))
        self.assertIn("warning", log)


if __name__ == "__main__":
    unittest.main()
