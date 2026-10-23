#!/usr/bin/env python3
"""Fail-closed metadata checks for exact labels in frozen reports."""
import unittest
from unittest.mock import patch

import check_curve_names as names


REPORT = "research/f4_f5_history_20261012/reports/run-20261012T0009Z/REPORT.md"


class ArchivedLabelTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.text = (names.REPO / REPORT).read_text()
        cls.resolver = names.Resolver()

    def test_exact_frozen_labels_resolve_to_the_reviewed_models(self):
        hits = names.find_retired(self.text, REPORT, self.resolver)
        self.assertEqual(len(hits), 2)
        for _, _, found, replacement in hits:
            self.assertTrue(names.archived_label(REPORT, self.text, found, replacement))
        with patch.object(names, "tracked", return_value=[REPORT]):
            self.assertEqual(names.lint(self.resolver, {}, False), [])

    def test_changed_hash_unknown_label_wrong_model_and_different_path_are_rejected(self):
        label = "K_1 / 2^23"
        model = "icv1-f2m23-tm5197-1f85e9e1"
        self.assertFalse(names.archived_label(REPORT, self.text + "\n", label, model))
        self.assertFalse(names.archived_label(REPORT, self.text, "K_1 / 2^19", model))
        self.assertFalse(names.archived_label(REPORT, self.text, label, "icv1-f2m17-tm101-00378d4e"))
        self.assertFalse(names.archived_label("ordinary/README.md", self.text, label, model))
        self.assertFalse(names.archived_label(REPORT, self.text, label, None))

    def test_diff_keeps_only_the_exact_archived_labels(self):
        diff = (f"+++ b/{REPORT}\n@@ -0,0 +1,3 @@\n"
                "+K_1 / 2^23\n+K_1 / 2^17\n+K_1 / 2^19\n")
        with patch.object(names, "git", side_effect=["", diff]):
            problems = names.lint_diff("base", self.resolver)
        self.assertEqual(len(problems), 1)
        self.assertIn("K_1 / 2^19", problems[0])

    def test_diff_preserves_frozen_source_and_still_lints_live_source(self):
        diff = ("+++ b/research/study/evidence/source/sat.rs\n"
                "@@ -0,0 +1 @@\n+K_1 / 2^19\n"
                "+++ b/src/live.rs\n@@ -0,0 +1 @@\n+K_1 / 2^19\n")
        with patch.object(names, "git", side_effect=["", diff]):
            problems = names.lint_diff("base", self.resolver)
        self.assertEqual(len(problems), 1)
        self.assertTrue(problems[0].startswith("src/live.rs:"))

    def test_archived_raw_metric_keys_require_exact_path_hash_and_label(self):
        report = "benchmarks/20261017-volcano-ecdlp-census/VALIDATOR_REPORT.md"
        text = (names.REPO / report).read_text()
        slug = "icv1-fp26-tm6998-a41c0795"
        self.assertTrue(names.archived_slug(report, text, slug))
        self.assertFalse(names.archived_slug(report, text + "\n", slug))
        self.assertFalse(names.archived_slug("ordinary/REPORT.md", text, slug))
        self.assertFalse(names.archived_slug(report, text, "icv1-fp26-tm6998-00000000"))
        with patch.object(names, "tracked", return_value=[report]):
            self.assertEqual(names.lint(self.resolver, {}, False), [])


if __name__ == "__main__":
    unittest.main()
