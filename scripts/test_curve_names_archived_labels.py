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


class HistoricalOracleProtocolTests(unittest.TestCase):
    protocol = "research/ideas_execution_20261018/oracle_integration/PROTOCOL.md"
    label = "K_0/GF(2^7)"
    slug = "icv1-f2m7-t13-616700dd"
    folder = "research/ideas_execution_20261018/oracle_integration/"
    snapshot_folder = folder + "source-snapshots/rust-lint-pre-repair-eaedaf1c1/"
    predecessor_sha256 = "3e2c19a3bd0e08d65fd001d01f9588b552c2391723230d6ca9176dd7702e2074"
    snapshot_manifest_sha256 = "2d0215f5cec540c33bc486642b5f87c79f09a9171234bb9f8d3a7ad183d6baec"

    @classmethod
    def setUpClass(cls):
        cls.text = (names.REPO / cls.protocol).read_text()
        cls.resolver = names.Resolver()
        cls.binding = json.loads((names.REPO / cls.protocol).with_name(
            "historical-label-binding.json").read_text())

    def verified_historical_source(self, *, v2=None, overrides=None):
        overrides = {} if overrides is None else overrides

        def read(path):
            return overrides[path] if path in overrides else (names.REPO / path).read_bytes()

        if v2 is None:
            v2 = json.loads(read(self.folder + "historical-label-binding-v2.json"))
        self.assertEqual(v2["schema"], "crypto.historical-curve-label-binding/v2")
        self.assertEqual(v2["predecessor_binding"], self.folder + "historical-label-binding.json")
        self.assertEqual(v2["predecessor_binding_sha256"], self.predecessor_sha256)
        predecessor = read(v2["predecessor_binding"])
        self.assertEqual(hashlib.sha256(predecessor).hexdigest(), self.predecessor_sha256)
        self.assertEqual(v2["snapshot_manifest"], self.snapshot_folder + "manifest.json")
        self.assertEqual(v2["snapshot_manifest_sha256"], self.snapshot_manifest_sha256)
        manifest_bytes = read(v2["snapshot_manifest"])
        self.assertEqual(hashlib.sha256(manifest_bytes).hexdigest(), self.snapshot_manifest_sha256)
        manifest = json.loads(manifest_bytes)
        self.assertEqual(manifest["schema"], "crypto.historical-native-source-snapshots/v1")
        self.assertEqual(manifest["capture_head"], "eaedaf1c132ed1f1ff101edd10c75369ab5e2dfe")
        paths = [entry["original_path"] for entry in manifest["sources"]]
        self.assertEqual(len(paths), len(set(paths)))
        self.assertEqual(set(paths), {
            "examples/ideas_bounded_memory_export.rs", "examples/ideas_oracle_components.rs",
            "examples/ideas_oracle_factorial_summary.rs", "src/cryptanalysis/ecbench/workload.rs",
            "src/cryptanalysis/gaudry_quartic.rs", "src/cryptanalysis/jacobian_transfer/mod.rs",
        })
        historical = None
        for entry in manifest["sources"]:
            self.assertEqual(entry["snapshot_path"], self.snapshot_folder + "original/" + entry["original_path"] + ".source")
            source = read(entry["snapshot_path"])
            self.assertEqual(hashlib.sha256(source).hexdigest(), entry["sha256"])
            blob = b"blob " + str(len(source)).encode() + b"\0" + source
            self.assertEqual(hashlib.sha1(blob).hexdigest(), entry["git_blob"])
            if entry["original_path"] == self.binding["native_source"]:
                historical = source
                self.assertEqual(entry["sha256"], self.binding["native_source_sha256"])
                self.assertEqual(entry["source_commit"], v2["historical_source_commit"])
                self.assertEqual(entry["git_blob"], v2["historical_source_blob"])
                self.assertEqual(entry["snapshot_path"], v2["historical_source_snapshot"])
        self.assertEqual(v2["historical_source_commit"], "a64f374747836eeac709bfade8cb851e46cf1443")
        self.assertEqual(v2["historical_source_blob"], "697b9ce42735024d82c3ae82411c46460a65120b")
        self.assertIsNotNone(historical)
        return historical

    def verify_raw_source_manifest(self, binding, source_record):
        self.assertEqual(hashlib.sha256(source_record).hexdigest(), binding["raw_source_manifest_sha256"])
        self.assertEqual(source_record.decode().strip(),
                         f'{binding["native_source_sha256"]}  {binding["native_source"]}')

    def test_historical_protocol_source_archive_and_model_binding(self):
        b = self.binding
        data = self.text.encode()
        self.assertEqual(b["protocol"], self.protocol)
        self.assertEqual(hashlib.sha256(data).hexdigest(), b["protocol_sha256"])
        blob = b"blob " + str(len(data)).encode() + b"\0" + data
        self.assertEqual(hashlib.sha1(blob).hexdigest(), b["historical_protocol_blob"])
        self.assertEqual(b["historical_commit"], "a64f374747836eeac709bfade8cb851e46cf1443")
        self.assertIn("Frozen before the first trial", self.text)
        source = self.verified_historical_source()
        self.assertEqual(hashlib.sha256(source).hexdigest(), b["native_source_sha256"])
        self.assertIn(b"KoblitzCurve::new(0, 7)", source)
        archive = names.REPO / b["raw_archive"]
        self.assertEqual(hashlib.sha256(archive.read_bytes()).hexdigest(), b["raw_archive_sha256"])
        with tarfile.open(archive, "r:gz") as tar:
            raw = tar.extractfile(b["raw_certificate_member"]).read()
            source_record = tar.extractfile(b["raw_source_manifest_member"]).read()
        self.assertEqual(hashlib.sha256(raw).hexdigest(), b["raw_certificate_sha256"])
        self.verify_raw_source_manifest(b, source_record)
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

    def test_source_snapshot_and_predecessor_tampering_are_rejected(self):
        v2 = json.loads((names.REPO / (self.folder + "historical-label-binding-v2.json")).read_bytes())
        snapshot = v2["historical_source_snapshot"]
        for path in [snapshot, v2["predecessor_binding"], v2["snapshot_manifest"]]:
            with self.subTest(tampered_bytes=path), self.assertRaises(AssertionError):
                self.verified_historical_source(overrides={path: (names.REPO / path).read_bytes() + b"\n"})
        for field, value in [
            ("historical_source_snapshot", self.binding["native_source"]),
            ("historical_source_commit", "0" * 40),
            ("historical_source_blob", "0" * 40),
            ("predecessor_binding_sha256", "0" * 64),
            ("snapshot_manifest_sha256", "0" * 64),
        ]:
            changed = dict(v2, **{field: value})
            with self.subTest(tampered_binding_field=field), self.assertRaises(AssertionError):
                self.verified_historical_source(v2=changed)

    def test_historical_source_does_not_depend_on_the_current_working_example(self):
        historical = self.verified_historical_source(overrides={
            self.binding["native_source"]: b"a later independently maintained example\n",
        })
        self.assertEqual(hashlib.sha256(historical).hexdigest(), self.binding["native_source_sha256"])

    def test_historical_raw_manifest_keeps_original_source_path(self):
        with tarfile.open(names.REPO / self.binding["raw_archive"], "r:gz") as tar:
            record = tar.extractfile(self.binding["raw_source_manifest_member"]).read()
        self.verify_raw_source_manifest(self.binding, record)
        changed = record.replace(self.binding["native_source"].encode(),
                                 (self.snapshot_folder + "original/" + self.binding["native_source"] + ".source").encode())
        self.assertNotEqual(changed, record)
        with self.assertRaises(AssertionError):
            self.verify_raw_source_manifest(self.binding, changed)

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
