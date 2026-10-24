#!/usr/bin/env python3
"""Tests for ci_fanout_report.py, using a tiny synthetic git repo in a temp dir.

Run:  python3 .github/scripts/test_ci_fanout_report.py
The git-backed tests are skipped (with a message) when PyYAML is missing; the
glob tests need only the standard library.
"""
import importlib.util
import io
import json
import os
from contextlib import redirect_stderr, redirect_stdout
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

HERE = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location("ci_fanout_report", HERE / "ci_fanout_report.py")
fr = importlib.util.module_from_spec(spec)
sys.modules["ci_fanout_report"] = fr
spec.loader.exec_module(fr)

try:
    import yaml  # noqa: F401
    HAVE_YAML = True
except ImportError:
    HAVE_YAML = False


def matches(pattern, path):
    return fr.glob_to_regex(pattern).fullmatch(path) is not None


class GlobTests(unittest.TestCase):
    def test_star_does_not_cross_slash(self):
        self.assertTrue(matches("*.md", "README.md"))
        self.assertFalse(matches("*.md", "docs/README.md"))
        self.assertTrue(matches("docs/*", "docs/a.md"))
        self.assertFalse(matches("docs/*", "docs/sub/a.md"))
        self.assertTrue(matches("src/*/mod.rs", "src/a/mod.rs"))
        self.assertFalse(matches("src/*/mod.rs", "src/a/b/mod.rs"))

    def test_double_star_crosses_slash(self):
        self.assertTrue(matches("docs/**", "docs/a/b/c.md"))
        self.assertTrue(matches("docs/**", "docs/a.md"))
        self.assertTrue(matches("**.rs", "src/a/b.rs"))
        self.assertTrue(matches("**.rs", "b.rs"))
        self.assertTrue(matches("src/**/mod.rs", "src/a/b/mod.rs"))
        self.assertFalse(matches("docs/**", "other/a.md"))
        self.assertFalse(matches("docs/**", "docsx/a.md"))

    def test_question_mark_is_zero_or_one_of_preceding_char(self):
        # GitHub's documented meaning: a quantifier, not a one-char wildcard.
        self.assertTrue(matches("colou?r.txt", "color.txt"))
        self.assertTrue(matches("colou?r.txt", "colour.txt"))
        self.assertFalse(matches("colou?r.txt", "colouur.txt"))
        self.assertFalse(matches("a?c", "abc"))

    def test_plus_and_sets(self):
        self.assertTrue(matches("a+.txt", "aaa.txt"))
        self.assertFalse(matches("a+.txt", ".txt"))
        self.assertTrue(matches("v[0-9].md", "v7.md"))
        self.assertFalse(matches("v[0-9].md", "vx.md"))
        self.assertTrue(matches("[abc]?.md", "a.md"))   # set, then zero-or-one quantifier
        self.assertTrue(matches("[abc]?.md", ".md"))
        self.assertFalse(matches("[abc]?.md", "d.md"))

    def test_escapes_and_literals(self):
        self.assertTrue(matches(r"a\*b", "a*b"))
        self.assertFalse(matches(r"a\*b", "axb"))
        self.assertTrue(matches("a.b", "a.b"))
        self.assertFalse(matches("a.b", "axb"))
        self.assertTrue(matches("?x", "?x"))       # nothing to quantify: literal
        self.assertTrue(matches("a(b).txt", "a(b).txt"))
        self.assertTrue(matches("[x", "[x"))       # unterminated set: literal

    def test_quantifier_after_star_is_literal_not_a_regex_error(self):
        self.assertTrue(matches("a*+", "abc+"))

    def test_ordered_negation(self):
        pats = fr.compile_patterns(["src/**", "!src/gen/**", "src/gen/keep.rs"], [], "t")
        self.assertIsNotNone(fr.ordered_match("src/a.rs", pats))
        self.assertIsNone(fr.ordered_match("src/gen/x.rs", pats))
        hit = fr.ordered_match("src/gen/keep.rs", pats)
        self.assertEqual(hit.raw, "src/gen/keep.rs")  # re-included by the later pattern
        self.assertIsNone(fr.ordered_match("docs/a.md", pats))

    def test_percentile_and_histogram(self):
        self.assertEqual(fr.percentile([1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 0.9), 9)
        self.assertEqual(fr.percentile([], 0.9), 0)
        h = fr.histogram([0, 3, 7, 12, 25, 50, 100])
        self.assertEqual(sum(h.values()), 7)
        self.assertEqual(h["81+"], 1)


WORKFLOWS = {
    # unfiltered, string form of `on`
    "always.yml": "name: always\non: pull_request\njobs: {a: {runs-on: x, steps: []}}\n",
    # list form, plus push; unfiltered
    "list.yml": "on: [push, pull_request]\njobs: {a: {runs-on: x, steps: []}}\n",
    # paths with a negation
    "rust.yml": ("on:\n  pull_request:\n    paths:\n      - 'src/**'\n      - '!src/gen/**'\n"
                 "      - 'Cargo.toml'\njobs: {a: {runs-on: x, steps: []}}\n"),
    # another workflow sharing the Cargo.toml and src/** entries
    "rust2.yml": ("on:\n  pull_request:\n    paths: ['Cargo.toml', 'src/**']\n"
                  "jobs: {a: {runs-on: x, steps: []}}\n"),
    "docs.yml": ("on:\n  pull_request:\n    paths: ['docs/**']\njobs: {a: {runs-on: x, steps: []}}\n"),
    # paths-ignore
    "notdocs.yml": ("on:\n  pull_request:\n    paths-ignore: ['docs/**', '*.md']\n"
                    "jobs: {a: {runs-on: x, steps: []}}\n"),
    # label-only: not per-PR fan-out
    "label.yml": ("on:\n  pull_request:\n    types: [labeled]\njobs: {a: {runs-on: x, steps: []}}\n"),
    # branch filter that never matches main
    "release.yml": ("on:\n  pull_request:\n    branches: ['release/**']\njobs: {a: {runs-on: x, steps: []}}\n"),
    # pull_request_target with its own paths
    "target.yml": ("on:\n  pull_request_target:\n    paths: ['docs/**']\njobs: {a: {runs-on: x, steps: []}}\n"),
    # dispatch only
    "manual.yml": "on: workflow_dispatch\njobs: {a: {runs-on: x, steps: []}}\n",
    # broken YAML must be reported, not crash the report
    "broken.yml": "on: [pull_request\njobs: {\n",
}


class GitRepoCase(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        if not HAVE_YAML:
            raise unittest.SkipTest("PyYAML not installed; git-backed tests skipped")
        cls._tmp = tempfile.TemporaryDirectory()
        cls.repo = cls._tmp.name
        cls.env = dict(os.environ, GIT_AUTHOR_NAME="t", GIT_AUTHOR_EMAIL="t@example.invalid",
                       GIT_COMMITTER_NAME="t", GIT_COMMITTER_EMAIL="t@example.invalid",
                       GIT_CONFIG_GLOBAL=os.devnull, GIT_CONFIG_SYSTEM=os.devnull)
        cls.git("init", "-q", "-b", "main")
        wf = Path(cls.repo, ".github", "workflows")
        wf.mkdir(parents=True)
        for name, text in WORKFLOWS.items():
            (wf / name).write_text(text)
        cls.write("README.md", "hi\n")
        cls.commit("initial")
        # PR 1: docs only
        cls.pr("pr-docs", {"docs/a.md": "a\n"})
        # PR 2: src change
        cls.pr("pr-src", {"src/lib.rs": "x\n"})
        # PR 3: only excluded generated code
        cls.pr("pr-gen", {"src/gen/out.rs": "x\n"})
        # PR 4: markdown at root only
        cls.pr("pr-readme", {"CHANGELOG.md": "x\n"})
        # PR 5: Cargo.toml only
        cls.pr("pr-cargo", {"Cargo.toml": "x\n"})
        # PR 6: a PR that also removes docs.yml, to exercise --workflows-ref merge
        cls.pr("pr-drop-docs", {"other/x.txt": "x\n"}, remove=[".github/workflows/docs.yml"])
        cls.pr("pr-docs2", {"docs/b.md": "b\n"})

    @classmethod
    def tearDownClass(cls):
        if hasattr(cls, "_tmp"):
            cls._tmp.cleanup()

    @classmethod
    def git(cls, *args):
        return subprocess.run(["git", "-C", cls.repo, *args], env=cls.env, check=True,
                              stdout=subprocess.PIPE, stderr=subprocess.PIPE).stdout.decode()

    @classmethod
    def write(cls, rel, text):
        p = Path(cls.repo, rel)
        p.parent.mkdir(parents=True, exist_ok=True)
        p.write_text(text)

    @classmethod
    def commit(cls, msg):
        cls.git("add", "-A")
        cls.git("commit", "-q", "-m", msg)

    @classmethod
    def pr(cls, branch, files, remove=()):
        cls.git("checkout", "-q", "-b", branch, "main")
        for rel, text in files.items():
            cls.write(rel, text)
        for rel in remove:
            Path(cls.repo, rel).unlink()
        cls.commit(f"work on {branch}")
        cls.git("checkout", "-q", "main")
        cls.git("merge", "-q", "--no-ff", "-m", f"Merge pull request from {branch}", branch)

    def report(self, *extra):
        out, err = io.StringIO(), io.StringIO()
        with redirect_stdout(out), redirect_stderr(err):
            code = fr.main(["--repo", self.repo, "--range", "main", "--json", *extra])
        return code, (json.loads(out.getvalue()) if out.getvalue() else None), err.getvalue()


class ReplayTests(GitRepoCase):
    def test_per_pr_counts(self):
        # The workflow set is read at the tip of the range, where docs.yml has
        # been removed by pr-drop-docs.  always.yml and list.yml fire for every PR;
        # label.yml, release.yml (branch filter), manual.yml and broken.yml never do.
        code, rep, _ = self.report()
        self.assertEqual(code, 0)
        self.assertEqual(rep["prs_replayed"], 7)
        by_subject = {p["subject"].split()[-1]: p["fanout"] for p in rep["worst_prs"]}
        expected = {
            "pr-docs": 3,        # always, list, target (docs.yml gone; notdocs ignores docs/**)
            "pr-src": 5,         # always, list, rust, rust2, notdocs
            "pr-gen": 4,         # always, list, rust2, notdocs (rust negates src/gen/**)
            "pr-readme": 2,      # always, list (notdocs ignores *.md)
            "pr-cargo": 5,       # always, list, rust, rust2, notdocs
            "pr-drop-docs": 3,   # always, list, notdocs (other/x.txt, .github/workflows/docs.yml)
            "pr-docs2": 3,       # always, list, target
        }
        self.assertEqual(by_subject, expected)

    def test_summary_statistics(self):
        _, rep, _ = self.report()
        fan = rep["fanout"]   # per-PR counts: [3, 5, 4, 2, 5, 3, 3]
        self.assertEqual(fan["median"], 3)
        self.assertEqual(fan["max"], 5)
        self.assertEqual(fan["min"], 2)
        self.assertEqual(fan["p90"], 5)
        self.assertEqual(fan["mean"], round(25 / 7, 2))
        self.assertEqual(fan["always_on_floor"], 2)
        self.assertEqual(fan["triggered_by_every_pr"], 2)
        self.assertEqual(sum(fan["histogram"].values()), 7)

    def test_universe_classification(self):
        _, rep, _ = self.report()
        uni = rep["workflow_universe"]
        self.assertEqual(sorted(uni["always_on_unfiltered"]), ["always.yml", "list.yml"])
        self.assertEqual(uni["excluded_by_activity_type"], ["label.yml"])
        self.assertEqual(len(uni["parse_errors"]), 1)
        self.assertTrue(uni["parse_errors"][0].startswith("broken.yml"))
        # release.yml has a PR trigger (branch filtered), so it is PR-triggered but never fires on main.
        names = {t["workflow"] for t in rep["top_workflows"]}
        self.assertNotIn("release.yml", names)
        self.assertNotIn("manual.yml", names)
        self.assertNotIn("label.yml", names)

    def test_top_workflows_order(self):
        _, rep, _ = self.report()
        top = rep["top_workflows"]
        self.assertEqual({top[0]["workflow"], top[1]["workflow"]}, {"always.yml", "list.yml"})
        self.assertEqual(top[0]["triggers"], 7)
        self.assertEqual(top[0]["pct_of_prs"], 100.0)

    def test_shared_entry_attribution(self):
        _, rep, _ = self.report("--min-share", "2")
        entries = {e["entry"]: e for e in rep["top_shared_entries"]}
        # 'Cargo.toml' and 'src/**' are listed by both rust.yml and rust2.yml.
        self.assertEqual(set(entries), {"Cargo.toml", "src/**"})
        # src PR (src/lib.rs) and gen PR (src/gen/out.rs; only rust2 sees it):
        # src/** triggers: pr-src -> rust, rust2 (2); pr-gen -> rust2 (1) = 3.
        self.assertEqual(entries["src/**"]["workflow_triggers"], 3)
        self.assertEqual(entries["src/**"]["prs_hit"], 2)
        self.assertEqual(entries["src/**"]["decisive_triggers"], 3)
        # Cargo.toml: pr-cargo -> rust, rust2 = 2
        self.assertEqual(entries["Cargo.toml"]["workflow_triggers"], 2)
        self.assertEqual(entries["Cargo.toml"]["listed_in_workflows"], 2)
        # With min-share 1, docs/** (docs.yml gone at tip, target.yml remains) appears too.
        _, rep1, _ = self.report("--min-share", "1")
        self.assertIn("docs/**", {e["entry"] for e in rep1["top_shared_entries"]})

    def test_workflows_ref_merge_uses_each_merge_commit(self):
        _, rep, _ = self.report("--workflows-ref", "merge")
        by_subject = {p["subject"].split()[-1]: p["fanout"] for p in rep["worst_prs"]}
        # At the pr-docs merge docs.yml still exists: always, list, docs, target = 4.
        self.assertEqual(by_subject["pr-docs"], 4)
        # At the pr-docs2 merge docs.yml is gone: always, list, target = 3.
        self.assertEqual(by_subject["pr-docs2"], 3)
        # The tip-based replay (default) says 3 for pr-docs; per-merge says 4.

    def test_events_option_excludes_pull_request_target(self):
        _, rep, _ = self.report("--events", "pull_request", "--workflows-ref", "merge")
        by_subject = {p["subject"].split()[-1]: p["fanout"] for p in rep["worst_prs"]}
        self.assertEqual(by_subject["pr-docs"], 3)   # always, list, docs

    def test_base_branch_filter(self):
        _, rep, _ = self.report("--base-branch", "release/1.0")
        names = {t["workflow"] for t in rep["top_workflows"]}
        self.assertIn("release.yml", names)

    def test_max_prs_limits_to_most_recent(self):
        _, rep, _ = self.report("--max-prs", "2")
        self.assertEqual(rep["prs_replayed"], 2)

    def test_first_parent_mode_counts_plain_commits_too(self):
        _, rep, _ = self.report("--mode", "first-parent", "--max-prs", "0")
        # First-parent history is the 7 merge commits plus the initial commit,
        # which has no parent and is skipped.
        self.assertEqual(rep["prs_replayed"], 7)

    def test_text_output_smoke(self):
        out, err = io.StringIO(), io.StringIO()
        with redirect_stdout(out), redirect_stderr(err):
            code = fr.main(["--repo", self.repo, "--range", "main"])
        text = out.getvalue()
        self.assertEqual(code, 0)
        for needle in ("Workflows triggered per PR", "Top workflows by trigger frequency",
                       "Shared path entries by fan-out", "Highest fan-out PRs", "Unparseable workflows"):
            self.assertIn(needle, text)

    def test_script_runs_as_a_subprocess(self):
        proc = subprocess.run([sys.executable, str(HERE / "ci_fanout_report.py"), "--repo", self.repo,
                               "--range", "main", "--json"], stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        self.assertEqual(proc.returncode, 0, proc.stderr.decode())
        self.assertEqual(json.loads(proc.stdout)["prs_replayed"], 7)


class BudgetTests(GitRepoCase):
    def test_max_fanout_exceeded_is_nonzero(self):
        code, rep, err = self.report("--max-fanout", "4")
        self.assertEqual(code, fr.EXIT_BUDGET)
        self.assertIn("BUDGET EXCEEDED", err)
        self.assertEqual(rep["budget"]["violations"][0].split()[0], "max")

    def test_max_fanout_met_is_zero(self):
        code, rep, err = self.report("--max-fanout", "5")
        self.assertEqual(code, 0)
        self.assertEqual(rep["budget"]["violations"], [])
        self.assertEqual(err, "")

    def test_max_median(self):
        code, _, err = self.report("--max-median", "1")
        self.assertEqual(code, fr.EXIT_BUDGET)
        self.assertIn("median", err)
        code, _, _ = self.report("--max-median", "10")
        self.assertEqual(code, 0)

    def test_no_budget_flags_never_fail(self):
        code, rep, _ = self.report()
        self.assertEqual(code, 0)
        self.assertNotIn("budget", rep)

    def test_diff_mode_scores_one_change_set(self):
        out, err = io.StringIO(), io.StringIO()
        with redirect_stdout(out), redirect_stderr(err):
            code = fr.main(["--repo", self.repo, "--diff", "pr-src~1", "pr-src", "--json", "--max-fanout", "4"])
        rep = json.loads(out.getvalue())
        self.assertEqual(rep["prs_replayed"], 1)
        # src/lib.rs against the workflow set at pr-src: always, list, rust, rust2, notdocs, docs? no.
        self.assertEqual(rep["fanout"]["max"], 5)
        self.assertEqual(code, fr.EXIT_BUDGET)


class ErrorTests(unittest.TestCase):
    def test_bad_repo_is_exit_2(self):
        with tempfile.TemporaryDirectory() as d:
            err = io.StringIO()
            with redirect_stderr(err), redirect_stdout(io.StringIO()):
                code = fr.main(["--repo", d])
            self.assertEqual(code, fr.EXIT_ERROR)
            self.assertIn("error", err.getvalue())

    def test_missing_pyyaml_message(self):
        # Simulate the import failing; the message must say how to fix it.
        import builtins
        real = builtins.__import__

        def fake(name, *a, **k):
            if name == "yaml":
                raise ImportError("no yaml")
            return real(name, *a, **k)

        builtins.__import__ = fake
        try:
            with self.assertRaises(fr.ReportError) as cm:
                fr.parse_workflow("x.yml", "on: pull_request\n")
        finally:
            builtins.__import__ = real
        self.assertIn("pip install pyyaml", str(cm.exception))


if __name__ == "__main__":
    unittest.main()
