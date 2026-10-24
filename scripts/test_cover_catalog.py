"""Metadata joins only. Cover mathematics is tested in the Rust binary."""
import copy
import unittest
import update_curve_standards as standards

import build_lab_browser as browser
import cover_catalog


class CoverCatalogTests(unittest.TestCase):
    def test_complete_catalog_join(self):
        data = browser.build()
        self.assertEqual(data["counts"]["curves"], len(browser.load(browser.REGISTRY)["curves"]))
        catalog = set(browser.load(cover_catalog.MANIFEST)["slugs"])
        self.assertEqual(sum(c["slug"] in catalog for c in data["curves"]), len(catalog))
        for curve in data["curves"]:
            finding = curve["hyperelliptic_cover"]
            if curve["slug"] not in catalog:
                # Census rows outside the cover catalog carry no finding.
                self.assertIsNone(finding)
                self.assertEqual(curve["cover_links"], [])
                continue
            self.assertEqual(finding["slug"], curve["slug"])
            self.assertEqual(finding["status"], "verified_over_declared_field")
            self.assertTrue(curve["cover_links"])
            self.assertTrue(curve["cover_links"][0]["cover_uid"].startswith("urn:hc-model:1:sha256:"))
        self.assertIn(browser.COVERS, data["sources"])
        self.assertIn(browser.COVER_LINKS, data["sources"])
        self.assertIn(cover_catalog.MANIFEST, data["sources"])

    def test_source_inventory_and_idempotent_merge(self):
        catalog = browser.load(browser.REGISTRY)
        imported = browser.load("docs/curves/standards/registry.json")
        self.assertEqual(standards.merge(copy.deepcopy(catalog), imported), catalog)
        by_model = {row["model_json"]: row for row in catalog["curves"]}
        for source_row in imported["curves"]:
            merged = by_model[source_row["model_json"]]
            self.assertIn("docs/curves/standards/registry.json", merged["sources"])
            for provenance in source_row["standards_provenance"]:
                self.assertIn(provenance, merged["standards_provenance"])
        coverage = browser.load(browser.STANDARDS_COVERAGE)
        records = coverage["records"]
        source_rows = sum((browser.load("docs/curves/standards/" + name)["curves"]
                           for name in ("parameters.json", "supplemental.json")), [])
        self.assertEqual({r["source_id"] for r in records}, {r["source_id"] for r in source_rows})
        self.assertEqual(len(records), len(source_rows))
        slugs = {c["slug"] for c in catalog["curves"]}
        for row in records:
            if row["status"] == "imported":
                self.assertIn(row["icv1_slug"], slugs)
            else:
                self.assertIsNone(row["exists"])
                self.assertTrue(row["reason"])

    def test_stale_and_ambiguous_findings_rejected(self):
        registry = browser.load(browser.REGISTRY)
        curves = browser.curve_rows(registry, browser.load(browser.LEADERBOARD))
        catalog = cover_catalog.subset(registry, browser.load(cover_catalog.MANIFEST))
        original = browser.load(browser.COVERS)
        browser.attach_covers(copy.deepcopy(curves), copy.deepcopy(original), catalog)
        for mutation in ("catalog", "model", "missing", "duplicate"):
            report = copy.deepcopy(original)
            if mutation == "catalog":
                report["catalog_rows_sha256"] = "0" * 64
            elif mutation == "model":
                report["curves"][0]["model_sha256"] = "0" * 64
            elif mutation == "missing":
                report["curves"].pop()
            else:
                report["curves"].append(report["curves"][0])
            with self.subTest(mutation=mutation), self.assertRaises(ValueError):
                browser.attach_covers(copy.deepcopy(curves), report, catalog)

    def test_census_rows_leave_the_catalog_digest(self):
        registry = browser.load(browser.REGISTRY)
        manifest = browser.load(cover_catalog.MANIFEST)
        digest = cover_catalog.rows_sha256(cover_catalog.subset(registry, manifest))
        self.assertEqual(digest, browser.load(browser.COVERS)["catalog_rows_sha256"])
        grown = copy.deepcopy(registry)
        grown["curves"].insert(0, dict(grown["curves"][0], slug="census-row-outside-the-catalog"))
        self.assertEqual(cover_catalog.rows_sha256(cover_catalog.subset(grown, manifest)), digest)
        grown["curves"][1]["sources"] = grown["curves"][1]["sources"] + ["edited"]
        self.assertNotEqual(cover_catalog.rows_sha256(cover_catalog.subset(grown, manifest)), digest)
        with self.assertRaises(ValueError):
            cover_catalog.subset(registry, dict(manifest, slugs=manifest["slugs"] + ["absent"],
                                                count=manifest["count"] + 1))


if __name__ == "__main__":
    unittest.main()
