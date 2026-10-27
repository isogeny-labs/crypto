#!/usr/bin/env python3
"""Cross-check native generated models against frozen research workloads."""
from __future__ import annotations

import json
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
SOURCE = ROOT / "docs/curves/sources/generated.json"
ARCHIVES = {
    "glsfp2": ROOT / "research/ext_endo_rho_20261023/sessions-a1/gls-fp2/records.jsonl",
    "subfieldfp3": ROOT / "research/ext_endo_rho_20261023/sessions-a1/subfield-fp3/records.jsonl",
}


def pack(coords: list[int], p: int) -> int:
    return sum(int(v) * p**i for i, v in enumerate(coords))


class GeneratedExtensionCurves(unittest.TestCase):
    def test_models_match_archived_subgroups_and_generators(self):
        doc = json.loads(SOURCE.read_text())
        archived = [r for r in doc["curves"]
                    if r.get("generator_call", "").startswith(
                        ("gls_fp2::", "subfield_fp3::"))]
        rows = {r["slug"]: r for r in archived}
        self.assertEqual(len(archived), 18)
        self.assertEqual(len(set(rows)), 18)
        for prefix, archive in ARCHIVES.items():
            seen = {}
            with archive.open() as stream:
                for line in stream:
                    curve = json.loads(line)["workload"]["curve"]
                    slug = curve["slug"]
                    if slug in seen:
                        self.assertEqual(curve, seen[slug], slug)
                    else:
                        seen[slug] = curve
            self.assertEqual(len(seen), 9, prefix)
            for slug, curve in seen.items():
                self.assertIn(slug, rows)
                row = rows[slug]
                self.assertTrue(row["generator_call"].startswith(
                    "gls_fp2::" if prefix == "glsfp2" else "subfield_fp3::"))
                self.assertEqual(row["icv1"], curve["icv1"])
                self.assertEqual(int(row["group_order"]), curve["group_order"])
                self.assertEqual(int(row["subgroup_order"]), curve["r"])
                self.assertEqual(int(row["cofactor"]), curve["cofactor"])
                p = int(row["field"]["p"])
                self.assertEqual(
                    [pack(row["generator"][axis], p) for axis in ("x", "y")],
                    [int(v, 16) for v in curve["generator"]],
                )

    def test_sparse_la_models_match_frozen_specs(self):
        rows = {r["slug"]: r for r in json.loads(SOURCE.read_text())["curves"]}
        base = ROOT / "research/sparse_la_timing_20261018/specs"
        fp4_spec = json.loads((base / "SPEC-fp4-p29.json").read_text())["workloads"]["curves"][0]
        prime_spec = json.loads((base / "SPEC-prime-scaling.json").read_text())["workloads"]["curves"][0]
        self.assertEqual(fp4_spec, {"kind": "fp4_torsion", "p": 29, "seed": 1, "max_cofactor": 16})
        self.assertEqual(prime_spec, {"kind": "prime_search", "bits": 30, "seed": 59297})
        fp4 = rows["icv1-fp5k4-tm926-7bc5b359"]
        prime = rows["icv1-fp30-t31711-b403411a"]
        self.assertEqual(fp4["generator_call"], "gaudry_quartic_d4::find_d4_curve(29, 1, 16)")
        self.assertEqual(prime["generator_call"], "find_prime_order_curve(30, 59297)")
        self.assertEqual(fp4["field"]["degree"], 4)
        self.assertEqual((int(fp4["group_order"]), int(fp4["subgroup_order"]), int(fp4["cofactor"])),
                         (708208, 44263, 16))
        self.assertEqual(int(prime["group_order"]),
                         int(prime["subgroup_order"]) * int(prime["cofactor"]))


if __name__ == "__main__":
    unittest.main()
