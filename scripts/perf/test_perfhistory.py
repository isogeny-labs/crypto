"""Unit tests for the chained index and the renderers in perfhistory.py.

    python3 -m unittest scripts/perf/test_perfhistory.py
"""

from __future__ import annotations

import json
import math
import sys
import tempfile
import unittest
import xml.etree.ElementTree as ET
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import perfhistory  # noqa: E402


def snap(rev: str, date: str, kernels: list[tuple[str, str, str, int]], subject: str = "s") -> dict:
    return {
        "schema": perfhistory.SCHEMA,
        "rev": rev * 10,
        "rev_short": (rev * 10)[:12],
        "parents": [],
        "committed_at": date,
        "recorded_at": date,
        "subject": subject,
        "note": "",
        "harness_sha256": "x",
        "valgrind": "valgrind-3.22.0",
        "host": {"cpu": "test cpu"},
        "wall_samples": 1,
        "kernels": [
            {"id": kid, "area": area, "tier": "Quick", "fingerprint": fp, "ir": ir, "wall_ns": 1000}
            for kid, area, fp, ir in kernels
        ],
    }


class ChainedIndex(unittest.TestCase):
    def test_halving_ir_doubles_the_index(self):
        s1 = snap("a", "2026-10-01T00:00:00+00:00", [("x/k", "x", "f", 1000)])
        s2 = snap("b", "2026-10-02T00:00:00+00:00", [("x/k", "x", "f", 500)])
        ch = perfhistory.chained([s1, s2], {})
        self.assertAlmostEqual(ch["per"]["x/k"][1]["index"], 2.0)
        self.assertAlmostEqual(ch["area_index"]["x"][1], 2.0)
        self.assertAlmostEqual(ch["overall"][1], 2.0)

    def test_fingerprint_change_takes_no_credit(self):
        s1 = snap("a", "2026-10-01T00:00:00+00:00", [("x/k", "x", "f", 1000)])
        s2 = snap("b", "2026-10-02T00:00:00+00:00", [("x/k", "x", "g", 10)])
        s3 = snap("c", "2026-10-03T00:00:00+00:00", [("x/k", "x", "g", 5)])
        ch = perfhistory.chained([s1, s2, s3], {})
        self.assertTrue(ch["per"]["x/k"][1]["rebased"])
        self.assertAlmostEqual(ch["per"]["x/k"][1]["index"], 1.0)
        self.assertAlmostEqual(ch["per"]["x/k"][2]["index"], 2.0)

    def test_areas_are_geometric_means_and_weights_apply(self):
        s1 = snap("a", "2026-10-01T00:00:00+00:00",
                  [("x/k1", "x", "f", 100), ("x/k2", "x", "f", 100), ("y/k", "y", "f", 100)])
        s2 = snap("b", "2026-10-02T00:00:00+00:00",
                  [("x/k1", "x", "f", 25), ("x/k2", "x", "f", 100), ("y/k", "y", "f", 100)])
        ch = perfhistory.chained([s1, s2], {})
        self.assertAlmostEqual(ch["area_index"]["x"][1], 2.0)  # sqrt(4 * 1)
        self.assertAlmostEqual(ch["area_index"]["y"][1], 1.0)
        self.assertAlmostEqual(ch["overall"][1], math.sqrt(2.0))  # equal area weights
        ch_w = perfhistory.chained([s1, s2], {"x": 1, "y": 3})
        self.assertAlmostEqual(ch_w["overall"][1], 2.0 ** 0.25)

    def test_absent_kernel_keeps_its_index(self):
        s1 = snap("a", "2026-10-01T00:00:00+00:00", [("x/k", "x", "f", 100), ("x/j", "x", "f", 100)])
        s2 = snap("b", "2026-10-02T00:00:00+00:00", [("x/k", "x", "f", 50)])
        s3 = snap("c", "2026-10-03T00:00:00+00:00", [("x/k", "x", "f", 50), ("x/j", "x", "f", 100)])
        ch = perfhistory.chained([s1, s2, s3], {})
        self.assertFalse(ch["per"]["x/j"][1]["present"])
        self.assertAlmostEqual(ch["per"]["x/j"][2]["index"], 1.0)
        self.assertAlmostEqual(ch["per"]["x/k"][2]["index"], 2.0)

    def test_snapshots_sort_by_commit_date_not_file_name(self):
        with tempfile.TemporaryDirectory() as td:
            d = Path(td)
            later = snap("b", "2026-10-02T00:00:00+00:00", [("x/k", "x", "f", 50)])
            earlier = snap("a", "2026-10-01T00:00:00+00:00", [("x/k", "x", "f", 100)])
            (d / "0.json").write_text(json.dumps(later))
            (d / "1.json").write_text(json.dumps(earlier))
            (d / "junk.json").write_text(json.dumps({"schema": "other"}))
            snaps = perfhistory.load_history(d)
            self.assertEqual([s["rev_short"][:1] for s in snaps], ["a", "b"])


class Renderers(unittest.TestCase):
    def setUp(self):
        self.snaps = [
            snap("a", "2026-10-01T00:00:00+00:00",
                 [("x/k", "x", "f", 1000), ("ecbench/c", "ecbench", "f", 400)], "first"),
            snap("b", "2026-10-02T00:00:00+00:00",
                 [("x/k", "x", "f", 800), ("ecbench/c", "ecbench", "f", 400), ("x/new", "x", "f", 7)], "second"),
        ]
        self.ch = perfhistory.chained(self.snaps, {})

    def test_svg_is_well_formed_and_lists_every_series(self):
        svg = perfhistory.svg_chart(self.snaps, self.ch, "t")
        root = ET.fromstring(svg)
        self.assertTrue(root.tag.endswith("svg"))
        self.assertIn("overall index", svg)
        self.assertIn("ecbench", svg)
        # area x = geometric mean of x/k (1000 -> 800, 1.25x) and x/new (new, 1x)
        self.assertIn(f"x {perfhistory.fmt_x(math.sqrt(1.25))}", svg)

    def test_single_snapshot_renders(self):
        ch = perfhistory.chained(self.snaps[:1], {})
        ET.fromstring(perfhistory.svg_chart(self.snaps[:1], ch, "t"))
        md = perfhistory.render_md(self.snaps[:1], ch, None)
        self.assertIn("Snapshots:** 1", md)

    def test_markdown_has_sections_and_new_marker(self):
        md = perfhistory.render_md(self.snaps, self.ch, "history.svg")
        for section in ("## Areas", "## Snapshots", "## Movers in the latest step", "## Kernels at the latest snapshot"):
            self.assertIn(section, md)
        self.assertIn("![performance history](history.svg)", md)
        self.assertIn("| `x/new` | 7 |", md)
        self.assertIn("| new |", md)
        self.assertIn("-20.0 %", md)

    def test_json_summary(self):
        j = perfhistory.render_json(self.snaps, self.ch)
        self.assertEqual(j["kernels"]["x/k"]["series_ir"], [1000, 800])
        self.assertAlmostEqual(j["snapshots"][1]["areas"]["x"], math.sqrt(1.25))
        self.assertEqual(j["kernels"]["x/new"]["series_ir"], [None, 7])


def series(irs: list[int], fps: list[str] | None = None) -> list[dict]:
    """Snapshots of one kernel `x/k` with the given Ir values, one per day."""
    fps = fps or ["f"] * len(irs)
    return [snap(chr(ord("a") + i), f"2026-10-{i + 1:02d}T00:00:00+00:00", [("x/k", "x", fp, ir)])
            for i, (ir, fp) in enumerate(zip(irs, fps))]


class Regressions(unittest.TestCase):
    def test_flapping_between_two_levels_stays_quiet(self):
        sc = perfhistory.step_changes(series([100, 107, 100, 107, 100, 107]), 0.08)
        self.assertEqual(sc["regressions"], [])
        self.assertEqual(sc["improvements"], [])

    def test_rise_above_recent_max_is_flagged_against_that_max(self):
        sc = perfhistory.step_changes(series([100, 107, 100, 120]), 0.08)
        [r] = sc["regressions"]
        self.assertEqual((r["id"], r["before"], r["previous"], r["after"]), ("x/k", 107, 100, 120))
        self.assertAlmostEqual(r["ratio"], 120 / 107)

    def test_drop_below_recent_min_is_an_improvement(self):
        sc = perfhistory.step_changes(series([100, 95, 80]), 0.08)
        self.assertEqual([r["before"] for r in sc["improvements"]], [95])

    def test_window_stops_at_a_fingerprint_change(self):
        # the old-fingerprint 200 must not hide a rise from 100 to 130
        sc = perfhistory.step_changes(series([200, 100, 130], ["g", "f", "f"]), 0.08)
        self.assertEqual([r["before"] for r in sc["regressions"]], [100])
        sc = perfhistory.step_changes(series([100, 130], ["g", "f"]), 0.08)
        self.assertEqual((sc["regressions"], sc["rebased"]), ([], ["x/k"]))

    def test_markdown_never_claims_a_failure(self):
        md = perfhistory.regressions_md(perfhistory.step_changes(series([100, 120]), 0.08), "http://h")
        self.assertIn("| `x/k` | x | 100 | 120 | **+20.0 %** |", md)
        self.assertIn("never a failing check", md)
        self.assertIn("http://h", md)
        self.assertIn("nothing to compare",
                      perfhistory.regressions_md(perfhistory.step_changes(series([100]), 0.08), None))


class FactorBases(unittest.TestCase):
    def setUp(self):
        def k(kid, ir, fb=None):
            row = {"id": f"ic_factor_base/{kid}", "area": "ic_factor_base", "tier": "Quick", "fingerprint": "f",
                   "ir": ir, "wall_ns": 1, "curve_label": "toy curve", "group_ops": 10}
            if fb:
                row["factor_base"] = fb
            return row
        s = snap("a", "2026-10-01T00:00:00+00:00", [("x/k", "x", "f", 1000)])
        s["kernels"] += [k("rho", 1000),
                         k("orbit", 5000, {"spec": "koblitz-orbit:divisor=0;1", "symmetry": "signed Frobenius",
                                           "columns": 14, "signed_points": 443}),
                         k("subspace", 4000, {"spec": "binary-subspace:dimension=4", "symmetry": "none",
                                              "columns": 5, "signed_points": None})]
        self.snaps = [s]
        self.ch = perfhistory.chained(self.snaps, {})

    def test_table_lists_symmetry_size_and_ratio_to_rho(self):
        md = "\n".join(perfhistory.factor_base_md(self.snaps, self.ch, "fb.svg"))
        self.assertIn("![factor bases](fb.svg)", md)
        self.assertIn("| toy curve | `orbit` | `koblitz-orbit:divisor=0;1` | signed Frobenius | 14 | 443 | 5,000 |", md)
        self.assertIn("| — | 4,000 | 10 | 4.0× |", md)
        self.assertIn("| toy curve | `rho` | rho | — |", md)

    def test_svg_is_well_formed_and_coloured_by_symmetry(self):
        svg = perfhistory.svg_factor_bases(self.snaps, self.ch)
        ET.fromstring(svg)
        self.assertIn(perfhistory.SYMMETRY_COLORS["signed Frobenius"], svg)
        self.assertIn("toy curve", svg)

    def test_no_factor_bases_renders_nothing(self):
        plain = [snap("a", "2026-10-01T00:00:00+00:00", [("x/k", "x", "f", 1000)])]
        ch = perfhistory.chained(plain, {})
        self.assertEqual((perfhistory.svg_factor_bases(plain, ch), perfhistory.factor_base_md(plain, ch, None)),
                         ("", []))

    def test_panels_svg_is_well_formed(self):
        snaps = series([100, 107, 130])
        ET.fromstring(perfhistory.svg_panels(snaps, perfhistory.chained(snaps, {})))


if __name__ == "__main__":
    unittest.main()
