#!/usr/bin/env python3
"""Fail-closed metadata checks for exact labels in frozen artifacts."""
import hashlib
import json
import tarfile
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


class HistoricalOracleProtocolTests(unittest.TestCase):
    protocol = "research/ideas_execution_20261018/oracle_integration/PROTOCOL.md"
    label = "K_0/GF(2^7)"
    slug = "icv1-f2m7-t13-616700dd"

    @classmethod
    def setUpClass(cls):
        cls.text = (names.REPO / cls.protocol).read_text()
        cls.resolver = names.Resolver()
        cls.binding = json.loads((names.REPO / cls.protocol).with_name(
            "historical-label-binding.json").read_text())

    def test_historical_protocol_source_archive_and_model_binding(self):
        b = self.binding
        data = self.text.encode()
        self.assertEqual(b["protocol"], self.protocol)
        self.assertEqual(hashlib.sha256(data).hexdigest(), b["protocol_sha256"])
        blob = b"blob " + str(len(data)).encode() + b"\0" + data
        self.assertEqual(hashlib.sha1(blob).hexdigest(), b["historical_protocol_blob"])
        self.assertEqual(b["historical_commit"], "a64f374747836eeac709bfade8cb851e46cf1443")
        self.assertIn("Frozen before the first trial", self.text)
        source = (names.REPO / b["native_source"]).read_bytes()
        self.assertEqual(hashlib.sha256(source).hexdigest(), b["native_source_sha256"])
        self.assertIn(b"KoblitzCurve::new(0, 7)", source)
        archive = names.REPO / b["raw_archive"]
        self.assertEqual(hashlib.sha256(archive.read_bytes()).hexdigest(), b["raw_archive_sha256"])
        with tarfile.open(archive, "r:gz") as tar:
            raw = tar.extractfile(b["raw_certificate_member"]).read()
            source_record = tar.extractfile(b["raw_source_manifest_member"]).read()
        self.assertEqual(hashlib.sha256(raw).hexdigest(), b["raw_certificate_sha256"])
        self.assertEqual(hashlib.sha256(source_record).hexdigest(), b["raw_source_manifest_sha256"])
        self.assertEqual(source_record.decode().strip(),
                         f'{b["native_source_sha256"]}  {b["native_source"]}')
        cert = json.loads(raw)
        curve = self.resolver.koblitz(0, 7)
        model = json.loads(curve["model_json"])
        self.assertEqual(curve["slug"], self.slug)
        self.assertEqual(b["canonical_slug"], self.slug)
        self.assertEqual(b["historical_label"], self.label)
        self.assertEqual((cert["a"], cert["b"], cert["n"]), (0, 1, 7))
        self.assertEqual((model["a"], model["b"]), ("0x0", "0x1"))
        modulus = (1 << cert["n"]) | sum(1 << k for k in cert["modulus_low_terms"])
        self.assertEqual(hex(modulus), model["modulus"])
        self.assertEqual(str(cert["group_order"]), curve["order"])
        expected_model = {
            "form": model["form"], "a": cert["a"], "b": cert["b"],
            "n": cert["n"], "modulus": hex(modulus),
            "group_order": str(cert["group_order"]),
        }
        self.assertEqual(json.dumps(b["model"], sort_keys=True),
                         json.dumps(expected_model, sort_keys=True))
        self.assertEqual(sorted((p["m"], p["ell"]) for p in cert["panels"]), [(3, 4), (4, 3)])
        self.assertTrue(cert["passed"])

    def test_exact_frozen_label_is_checked_without_whole_directory_exemption(self):
        hits = names.find_retired(self.text, self.protocol, self.resolver)
        self.assertEqual([(h[2], h[3]) for h in hits], [(self.label, self.slug)])
        with patch.object(names, "tracked", return_value=[self.protocol]):
            self.assertEqual(names.lint(self.resolver, {}, False), [])
        self.assertFalse(names.archived_label(self.protocol, self.text + "\n", self.label, self.slug))
        self.assertFalse(names.archived_label(self.protocol, self.text, self.label, "wrong-model"))
        self.assertFalse(names.archived_label("ordinary/PROTOCOL.md", self.text, self.label, self.slug))

    def test_diff_rejects_unknown_labels_and_tampered_protocol_bytes(self):
        diff = (f"+++ b/{self.protocol}\n@@ -0,0 +1,2 @@\n"
                f"+{self.label}\n+K_0/GF(2^9)\n")
        with patch.object(names, "git", side_effect=["", diff]):
            problems = names.lint_diff("base", self.resolver)
        self.assertEqual(len(problems), 1)
        self.assertIn("K_0/GF(2^9)", problems[0])
        exact = f"+++ b/{self.protocol}\n@@ -0,0 +1 @@\n+{self.label}\n"
        with patch.object(names, "git", side_effect=["", exact]), \
                patch.object(names.Path, "read_text", return_value=self.text + "\n"):
            problems = names.lint_diff("base", self.resolver)
        self.assertEqual(len(problems), 1)
        self.assertIn(self.label, problems[0])


if __name__ == "__main__":
    unittest.main()
