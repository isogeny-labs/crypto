#!/usr/bin/env python3
"""Join E11 window-engine outputs with the dense reference records.

Checks, per draw:
  (i)   degree-2/3 full-closure dims equal the dense per-degree dims;
  (ii)  soundness: window certifies 1 at degree 4  =>  dense d_F <= 4;
  (iii) recall per policy over dense d_F = 4 draws, and columns vs dense.
Usage: compare.py [--json OUT] e11/validate/*.jsonl ...
"""
import glob
import json
import sys
from collections import defaultdict

S = "research/fall_degree_bounds_20261008"
FS = "research/dreg_fixed_surplus_20260923/runs"
M4 = "research/dreg_m4_chain_20261006/runs"
# reference file -> ladder inputs (for records without v_basis)
REFS = {
    f"{S}/e4/m3_5-2.jsonl": f"{FS}/cell-5-2-7.jsonl",
    f"{S}/e4/m3_7-2.jsonl": f"{FS}/cell-7-2-7.jsonl",
    f"{S}/e4/m3_7-3.jsonl": f"{FS}/cell-7-3-7.jsonl",
    f"{S}/e4/x_m3_9-3.jsonl": f"{FS}/cell-9-3-7.jsonl",
    f"{S}/e4/x_m3_11-4.jsonl": f"{FS}/cell-11-4-6.jsonl",
    f"{S}/e4/x_m3_13-4.jsonl": f"{FS}/cell-13-4-6.jsonl",
    f"{S}/e6/A_m3_13-5.jsonl": f"{S}/e6/inputs_13-5.jsonl",
    f"{S}/e6/A_m3_15-5.jsonl": f"{S}/e6/inputs_15-5.jsonl",
    f"{S}/e4/x_m4_5-2.jsonl": f"{M4}/cell-5-2-8.jsonl",
    f"{S}/e4/x_m4_7-2.jsonl": f"{M4}/cell-7-2-7.u*.jsonl",
    f"{S}/e4/x_m4_9-3.jsonl": f"{M4}/cell-9-3-6.u*.jsonl",
}
SELF = [f"{S}/e6/B_3-18-6.jsonl", f"{S}/e6/B_5-10-2.rerun.jsonl", f"{S}/e6/B_4-13-4.par.jsonl"]
SELF += sorted(glob.glob(f"{S}/e7/E7_*.jsonl")) + sorted(glob.glob(f"{S}/e8/E8_*.jsonl"))


def lines(pat):
    for p in sorted(glob.glob(pat)):
        for ln in open(p):
            if ln.startswith("{"):
                yield json.loads(ln)


def dims(rec):
    if "per_degree" in rec:
        return {k: v["dim"] for k, v in rec["per_degree"].items()}
    return {k: v for k, v in rec.get("closure_dims", {}).items()}


def m_of(rec, default):
    return rec.get("m", default)


def build_refs():
    ref = {}
    for rf, inp in REFS.items():
        m = 4 if "_m4_" in rf else 3
        byk = {(r.get("cell"), r["draw"]): r for r in lines(inp) if "draw" in r and "v_basis" in r}
        for r in lines(rf):
            src = byk.get((r.get("cell"), r["draw"]))
            if src is None:
                continue
            key = (m, tuple(src["v_basis"]), src["x_r"])
            ref[key] = {"d_f": r["d_f"], "dims": dims(r), "file": rf, "cols4": None}
    for rf in SELF:
        for r in lines(rf):
            if r.get("solutions") != 0 or "v_basis" not in r:
                continue
            key = (r["m"], tuple(r["v_basis"]), r["x_r"])
            of4 = r.get("per_degree", {}).get("4", {}).get("of")
            ref[key] = {"d_f": r["d_f"], "dims": dims(r), "file": rf, "cols4": of4}
    return ref


def main():
    args = sys.argv[1:]
    out_json = None
    if args and args[0] == "--json":
        out_json, args = args[1], args[2:]
    ref = build_refs()
    rows = []
    viol, dimbad, joined = [], [], 0
    per = defaultdict(lambda: {"n": 0, "cert": 0, "cols": [], "dense": []})
    for p in args:
        for r in lines(p):
            key = (r["m"], tuple(r["v_basis"]), r["x_r"])
            if key not in ref:
                continue
            joined += 1
            R = ref[key]
            mine = {k: v["dim"] for k, v in r["per_degree"].items()}
            for k, v in mine.items():
                if k in R["dims"] and R["dims"][k] != v:
                    dimbad.append((p, r["draw"], k, v, R["dims"][k]))
            if r["d_f_le3"] is not None and r["d_f_le3"] != R["d_f"]:
                dimbad.append((p, r["draw"], "d_f<=3", r["d_f_le3"], R["d_f"]))
            if r["d_f_le3"] is None and R["d_f"] is not None and R["d_f"] <= 3:
                dimbad.append((p, r["draw"], "missed d_f<=3", None, R["d_f"]))
            n = r["n_vars"]
            from math import comb
            dense = sum(comb(n, d) for d in range(5))
            for pol, w in r["window"].items():
                if w.get("certified") and not (R["d_f"] is not None and R["d_f"] <= 4):
                    viol.append((p, r["draw"], pol, R["d_f"]))
                if R["d_f"] == 4:
                    s = per[(pol, r["m"], r["n"], r["ell"])]
                    s["n"] += 1
                    s["cert"] += bool(w.get("certified"))
                    s["cols"].append(w["cols"])
                    s["dense"].append(dense)
            rows.append({"file": p, "m": r["m"], "n": r["n"], "ell": r["ell"], "draw": r["draw"],
                         "n_vars": n, "dense_d_f": R["d_f"], "d_f_le3": r["d_f_le3"],
                         "window": {k: {kk: w.get(kk) for kk in ("certified", "cols", "dim", "secs")}
                                    for k, w in r["window"].items()}})
    print(f"joined draws: {joined}; degree-2/3 dim or d_F<=3 mismatches: {len(dimbad)}; soundness violations: {len(viol)}")
    for d in dimbad:
        print("  MISMATCH", d)
    for v in viol:
        print("  VIOLATION", v)
    tot = defaultdict(lambda: [0, 0])
    print("policy  m:n:ell  N  certified/dense-d_F=4  cols  dense_cols")
    for (pol, m, n, ell), s in sorted(per.items(), key=lambda x: (x[0][0], x[0][1], x[0][2], x[0][3])):
        N = s["dense"][0]
        print(f"{pol:6} {m}:{n}:{ell}  {s['cert']}/{s['n']}  {s['cols'][0]}  {N}")
        tot[pol][0] += s["cert"]
        tot[pol][1] += s["n"]
    for pol, (a, b) in tot.items():
        print(f"RECALL {pol}: {a}/{b} = {a / b:.3f}")
    if out_json:
        json.dump({"joined": joined, "dim_mismatches": dimbad, "violations": viol,
                   "recall": {p: {"certified": a, "dense_df4": b} for p, (a, b) in tot.items()},
                   "cells": [{"policy": k[0], "m": k[1], "n": k[2], "ell": k[3], "certified": s["cert"],
                              "dense_df4": s["n"], "cols": s["cols"][0], "dense_cols": s["dense"][0]}
                             for k, s in per.items()],
                   "rows": rows}, open(out_json, "w"), indent=1)


if __name__ == "__main__":
    main()
