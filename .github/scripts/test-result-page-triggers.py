#!/usr/bin/env python3
"""Regressions for the result-page trigger policy, using small YAML fixtures."""
import importlib.util
from pathlib import Path
import unittest

import yaml

spec = importlib.util.spec_from_file_location(
    "result_page_triggers", Path(__file__).with_name("check-result-page-triggers.py"))
policy = importlib.util.module_from_spec(spec)
spec.loader.exec_module(policy)

SCOREBOARD = "docs/index-calculus-scoreboard.html"
STATE = "docs/ic-current-state.html"
WF = ".github/workflows/replay.yml"


def workflow(paths, event="pull_request"):
    return {"on": {event: {"paths": paths}}, "jobs": {"replay": {"steps": []}}}


class ResultPageTriggerTests(unittest.TestCase):
    def test_unlisted_workflow_with_a_page_path_fails(self):
        for page in (SCOREBOARD, STATE):
            failures = policy.problems({WF: workflow(["research/x/**", page])}, allowed={})
            self.assertEqual(len(failures), 1)
            self.assertIn(page, failures[0])

    def test_workflow_without_page_paths_passes(self):
        self.assertFalse(policy.problems({WF: workflow(["research/x/**", "docs/other.html"])}, allowed={}))

    def test_globs_that_cover_a_page_count(self):
        for pattern in ("docs/**", "**/*.html", "docs/*.html", "docs/index-calculus-*.html",
                        "docs/ic-current-stat?.html", "docs/[i]c-current-state.html"):
            self.assertTrue(policy.problems({WF: workflow([pattern])}, allowed={}), pattern)

    def test_globs_that_miss_the_pages_pass(self):
        for pattern in ("docs/*/*.html", "docs/ic/**", "*.html", "docs/index-calculus-scoreboard.htm"):
            self.assertFalse(policy.problems({WF: workflow([pattern])}, allowed={}), pattern)

    def test_negation_applies_in_order(self):
        excluded = ["docs/**", "!" + SCOREBOARD, "!" + STATE]
        self.assertFalse(policy.problems({WF: workflow(excluded)}, allowed={}))
        reincluded = ["docs/**", "!" + SCOREBOARD, SCOREBOARD, "!" + STATE]
        self.assertTrue(policy.problems({WF: workflow(reincluded)}, allowed={}))

    def test_every_triggering_event_is_checked(self):
        for event in ("pull_request", "pull_request_target", "push", "merge_group"):
            self.assertTrue(policy.problems({WF: workflow([SCOREBOARD], event)}, allowed={}), event)

    def test_paths_ignore_and_unfiltered_workflows_are_out_of_scope(self):
        ignore = {"on": {"pull_request": {"paths-ignore": [SCOREBOARD]}}, "jobs": {}}
        unfiltered = {"on": {"pull_request": None}, "jobs": {}}
        manual = {"on": {"workflow_dispatch": {}, "schedule": [{"cron": "0 0 * * *"}]}, "jobs": {}}
        for document in (ignore, unfiltered, manual):
            self.assertFalse(policy.problems({WF: document}, allowed={}))

    def test_bare_on_key_parsed_as_true_is_read(self):
        document = yaml.safe_load("on:\n  pull_request:\n    paths: ['docs/index-calculus-scoreboard.html']\njobs: {}\n")
        self.assertIn(True, document)
        self.assertTrue(policy.problems({WF: document}, allowed={}))

    def test_allowed_workflow_passes(self):
        allowed = {WF: {"reason": "renders the page"}}
        self.assertFalse(policy.problems({WF: workflow([SCOREBOARD])}, allowed=allowed))

    def test_stale_or_missing_allowlist_entries_fail(self):
        allowed = {WF: {"reason": "renders the page"}}
        self.assertTrue(policy.problems({WF: workflow(["research/x/**"])}, allowed=allowed))
        self.assertTrue(policy.problems({}, allowed=allowed))

    def test_repository_allowlist_matches_main(self):
        files = sorted((policy.REPO_ROOT / ".github/workflows").glob("*.y*ml"))
        workflows = {f.relative_to(policy.REPO_ROOT).as_posix(): yaml.safe_load(f.read_text()) for f in files}
        self.assertEqual(policy.problems(workflows), [])


if __name__ == "__main__":
    unittest.main()
