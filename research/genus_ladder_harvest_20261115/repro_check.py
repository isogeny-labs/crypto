#!/usr/bin/env python3
"""Reproduction check of the frozen instrument (PROTOCOL.md section 3), layout only.

usage: repro_check.py repro_sge.jsonl repro_parta.jsonl OUT.json
(i) row by row against research/genus_ladder_sge_20261115/sge_g34.jsonl (C_double_lp+pre);
(ii) cell means against research/genus_ladder_20261114/analysis.json (part-A cells).
"""
import json
import sys
from collections import defaultdict

ROOT = __file__.rsplit("/", 1)[0] + "/.."
SKIP = {"wall_ns", "kind"}


def runs(fn):
    return [json.loads(l) for l in open(fn) if json.loads(l).get("kind") == "run"]


def main():
    rs, ra, out = sys.argv[1], sys.argv[2], sys.argv[3]
    report = {"label": "[pcc:unverified]"}
    # (i) row level
    ref = {}
    for r in runs(f"{ROOT}/genus_ladder_sge_20261115/sge_g34.jsonl"):
        if r["arm"] == "C_double_lp+pre":
            ref[(r["p"], r["g"], r["curve_seed"], r["seed"])] = r
    mine = {(r["p"], r["g"], r["curve_seed"], r["seed"]): r for r in runs(rs)
            if r["arm"] == "C_double_lp+pre"}
    mism, fields = [], 0
    for k, a in ref.items():
        b = mine.get(k)
        if b is None:
            mism.append({"key": k, "missing": True})
            continue
        for f, v in a.items():
            if f in SKIP:
                continue
            fields += 1
            if b.get(f) != v:
                mism.append({"key": k, "field": f, "ref": v, "mine": b.get(f)})
    report["rows"] = {"reference_rows": len(ref), "compared_fields": fields,
                      "mismatches": mism, "extra_rows": len(set(mine) - set(ref))}
    # (ii) cell means
    A = json.load(open(f"{ROOT}/genus_ladder_20261114/analysis.json"))
    cells = defaultdict(list)
    for r in runs(ra):
        cells[(r["g"], r["p"], r["arm"])].append(r)
    cm, nc, nf = [], 0, 0
    for c in A["cells"]:
        if c["family"] != "hyperelliptic":
            continue
        key = (c["g"], c["p"], c["arm"])
        if key not in cells:
            continue
        nc += 1
        rr = cells[key]
        if len(rr) != c["runs"]:
            cm.append({"cell": key, "runs_ref": c["runs"], "runs_mine": len(rr)})
        for f, v in c.items():
            if not f.startswith("mean_") or f in ("mean_wall_ns", "mean_ln_r", "mean_steps"):
                continue
            src = f[5:]
            if src not in rr[0]:
                continue
            nf += 1
            m = sum(x[src] for x in rr) / len(rr)
            if abs(m - v) > 1e-9 * max(1.0, abs(v)):
                cm.append({"cell": key, "field": f, "ref": v, "mine": m})
    report["cells"] = {"compared_cells": nc, "compared_means": nf, "mismatches": cm}
    json.dump(report, open(out, "w"), indent=1, default=str)
    print(f"rows: {len(ref)} ref rows, {fields} fields, {len(mism)} mismatches; "
          f"cells: {nc} cells, {nf} means, {len(cm)} mismatches")


if __name__ == "__main__":
    main()
