#!/usr/bin/env python3
"""Standard-panel aliases (docs/ic/STANDARD_PANEL.md): the registry pins
each alias to one curve, and the curve-name lint rejects off-panel aliases
in scoped text only."""
import json
import unittest
from unittest.mock import patch

import check_curve_names as names
import curve_id as cid

PANEL = names.REPO / "docs/ic/standard_panel.json"


class StandardPanelTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.res = names.Resolver()
        cls.panel = json.loads(PANEL.read_text())["curves"]
        cls.by_slug = {c["slug"]: c for c in cls.res.reg["curves"]}

    def test_every_panel_row_is_pinned_in_the_registry(self):
        for row in self.panel:
            c = self.by_slug[row["slug"]]
            self.assertEqual(c.get("panel_alias"), row["alias"], row["alias"])
        pinned = [c for c in self.res.reg["curves"] if c.get("panel_alias")]
        self.assertEqual(len(pinned), len(self.panel))

    def test_alias_map_resolves_each_alias_to_its_slug(self):
        amap = json.loads(cid.ALIAS_MAP.read_text())["aliases"]
        for row in self.panel:
            self.assertEqual(amap[cid.normalise_alias(row["alias"])], row["slug"])

    def test_koblitz_aliases_name_a_and_n(self):
        for row in self.panel:
            if row["alias"].startswith("K"):
                a, n = row["alias"][1:].split("-")
                p = self.by_slug[row["slug"]]["params"]
                self.assertEqual((int(p["a"]), int(p["n"])), (int(a), int(n)))

    def lint(self, rel, text):
        with patch.object(names, "tracked", return_value=[rel]), \
             patch("pathlib.Path.read_text", return_value=text):
            return names.lint(self.res, {}, False)

    def test_scoped_text_rejects_off_panel_aliases_only(self):
        text = ("Ran on K0-23, P36a and K1-83.\nAlso P45a, K0-29 and B23a.\n"
                "Not curves: K1-2, `observation-K0-23.json`, P-256.\n")
        found = sorted(p.split("'")[1] for p in self.lint("docs/ic/PROBE.md", text))
        self.assertEqual(found, ["B23a", "K0-29", "P45a"])
        self.assertEqual(len(self.lint("research/probe_20261111/README.md", text)), 3)

    def test_text_before_the_freeze_and_code_fences_are_left_alone(self):
        self.assertEqual(self.lint("research/probe_20261101/README.md", "P45a\n"), [])
        self.assertEqual(self.lint("docs/ic/PROBE.md", "```\nP45a\n```\n"), [])


if __name__ == "__main__":
    unittest.main()
