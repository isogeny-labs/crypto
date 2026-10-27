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

    def test_frozen_unregistered_curve_ids_require_exact_path_hash_and_slug(self):
        cases = {
            "docs/ic/CERTICOM131_IC_PROGRAM.md": {
                "icv1-fp30-t49927-d70f5de2",
                "icv1-fp131-t29529377007934231835-9681b699",
            },
            "research/certicom131_ic_program_20261020/README.md": {
                "icv1-fp30-t49927-d70f5de2",
                "icv1-fp131-t29529377007934231835-9681b699",
            },
            "research/certicom131_ic_program_20261020/pkm_ext/tables/p30.table.md": {
                "icv1-fp30-t49927-d70f5de2",
            },
        }
        for rel, slugs in cases.items():
            with self.subTest(path=rel):
                text = (names.REPO / rel).read_text()
                for slug in slugs:
                    self.assertIn(slug, text)
                    self.assertTrue(names.archived_slug(rel, text, slug))
                    self.assertFalse(names.archived_slug(rel, text + "\n", slug))
                    self.assertFalse(names.archived_slug("ordinary/REPORT.md", text, slug))
                with patch.object(names, "tracked", return_value=[rel]):
                    self.assertEqual(names.lint(self.resolver, {}, False), [])


SESSION_TABLE = "research/ext_endo_rho_20261023/sessions/subfield-fp3.table.md"


class ArchivedUnregisteredSlugTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.text = (names.REPO / SESSION_TABLE).read_text()

    def test_only_exact_frozen_session_table_is_allowlisted(self):
        self.assertTrue(names.archived_unregistered_slug_file(SESSION_TABLE, self.text))
        self.assertFalse(names.archived_unregistered_slug_file(SESSION_TABLE, self.text + "\n"))
        self.assertFalse(names.archived_unregistered_slug_file("research/other.md", self.text))

    def test_normal_lint_allows_the_exact_frozen_table(self):
        resolver = names.Resolver()
        with patch.object(names, "tracked", return_value=[SESSION_TABLE]):
            self.assertEqual(names.lint(resolver, {}, False), [])

    def test_original_gls_table_is_allowed_only_at_its_frozen_hash(self):
        path = "research/ext_endo_rho_20261023/sessions/gls-fp2.table.md"
        text = (names.REPO / path).read_text()
        self.assertTrue(names.archived_unregistered_slug_file(path, text))
        self.assertFalse(names.archived_unregistered_slug_file(path, text + "\n"))
        resolver = names.Resolver()
        with patch.object(names, "tracked", return_value=[path]):
            self.assertEqual(names.lint(resolver, {}, False), [])


if __name__ == "__main__":
    unittest.main()
