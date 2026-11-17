"""Refuse stale catalog bindings while retaining supported and unsupported rows."""

import hashlib
import json
from pathlib import Path
import tempfile
import unittest

from check_curve_catalog_consistency import check_catalog


class CatalogBindings(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.catalog = self.root / "docs/curves"
        self.catalog.mkdir(parents=True)
        sources = ["checker.rs", "models.rs", "links.rs"]
        for name in sources:
            path = self.root / "src/bin/curve_cover_check" / name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(name.encode())
        (self.root / "src/bin/curve_cover_check.rs").write_bytes(b"main")
        registry = b'{"curves":[{"slug":"one"},{"slug":"two"}]}\n'
        (self.catalog / "registry.json").write_bytes(registry)
        digest = hashlib.sha256(registry).hexdigest()
        self.report = {"registry_sha256": digest,
                       "checker_source_sha256": hashlib.sha256(b"checker.rsmodels.rslinks.rsmain").hexdigest(),
                       "curves": [{}, {}],
                       "summary": {"verified": 1, "unsupported": 1, "invalid_input": 0}}
        self.write("covers.json", self.report)
        self.write("cover-links.yaml", {"registry_sha256": digest})

    def write(self, name, value):
        (self.catalog / name).write_text(json.dumps(value))

    def test_current_mixed_coverage_passes(self):
        check_catalog(self.root)

    def test_changed_registry_requires_regeneration(self):
        with (self.catalog / "registry.json").open("ab") as handle:
            handle.write(b" ")
        with self.assertRaisesRegex(ValueError, "stale registry"):
            check_catalog(self.root)

    def test_stale_graph_is_refused(self):
        self.write("cover-links.yaml", {"registry_sha256": "0" * 64})
        with self.assertRaisesRegex(ValueError, "cover-links.yaml binds a stale registry"):
            check_catalog(self.root)

    def test_changed_checker_requires_regeneration(self):
        (self.root / "src/bin/curve_cover_check/links.rs").write_bytes(b"changed")
        with self.assertRaisesRegex(ValueError, "stale checker source"):
            check_catalog(self.root)

    def test_partial_report_is_refused(self):
        self.report["curves"].pop()
        self.write("covers.json", self.report)
        with self.assertRaisesRegex(ValueError, "every registry row"):
            check_catalog(self.root)

    def test_incomplete_summary_is_refused(self):
        self.report["summary"]["unsupported"] = 0
        self.write("covers.json", self.report)
        with self.assertRaisesRegex(ValueError, "every report row"):
            check_catalog(self.root)


if __name__ == "__main__":
    unittest.main()
