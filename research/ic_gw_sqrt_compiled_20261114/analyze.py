#!/usr/bin/env python3
"""Lane "chain" (compiled square root), scored by PROTOCOL.md. usage: analyze.py RESULTS_DIR
Writes analysis.json and tables.md next to this file. Every number is [pcc:unverified]."""
import json, math, os, sys

HERE = os.path.dirname(os.path.abspath(__file__))
res = sys.argv[1]
PH = ["fb", "relations", "matrix", "linear_algebra", "log_recovery", "descent", "verify"]
QS = [262111, 524287, 1048507, 2097091, 4194271]
YS = ["D", "DL1", "DL"]
SEEDS = (1, 2, 3)
TIMING = ("wall", "cpu")


def load(label, q, s):
    p = os.path.join(res, "units", f"{label}_{q}_{s}.jsonl")
    rs = [json.loads(l) for l in open(p) if l.strip()]
    return [r for r in rs if r.get("rec") == "ic"], [r for r in rs if r.get("rec") == "unit"][0]


def counted(r):
    return {k: v for k, v in r.items() if not k.endswith(TIMING) and k != "load1" and not k.startswith("s.controls")}


def cpu(recs, phase="relations", y=None):
    return sum(r.get(phase + ".cpu", 0.0) for r in recs if y is None or r["arm"] == y)


def tot(recs, y):
    return sum(r.get(p + ".cpu", 0.0) for r in recs if r["arm"] == y for p in PH)


def same(a, b):
    """list of mismatching counted fields between two unit record lists"""
    if len(a) != len(b):
        return ["record count"]
    out = []
    for x, z in zip(a, b):
        cx, cz = counted(x), counted(z)
        if set(cx) != set(cz):
            out.append(f"{x['arm']} keys {sorted(set(cx) ^ set(cz))[:3]}")
        out += [f"{x['arm']}.{k}" for k in cx if k in cz and cx[k] != cz[k]]
    return out


U = {}
fails, g1, nfields, loads = [], [], 0, []
for lab in ("REF_p1", "CHAIN_p1", "REF_p2", "CHAIN_p2"):
    for q in QS:
        for s in SEEDS:
            recs, meta = load(lab, q, s)
            U[(lab, q, s)] = recs
            loads.append((meta["load1"], lab, q, s))
            if meta["rc"] != 0 or not recs or any(not r["ok"] or r["k_found"] != r["k"] for r in recs if r["arm"] in YS):
                fails.append(f"{lab} {q}_{s}")
            if {r["arm"] for r in recs} != set(YS):
                fails.append(f"{lab} {q}_{s} arms")
for q in QS:
    for s in SEEDS:
        for p in (1, 2):
            a, b = U[(f"REF_p{p}", q, s)], U[(f"CHAIN_p{p}", q, s)]
            nfields += sum(len(counted(r)) for r in a)
            d = same(a, b)
            if d:
                g1.append(f"pass{p} {q}_{s}: {d[:4]}")
        d = same(U[("REF_p1", q, s)], U[("REF_p2", q, s)])
        if d:
            g1.append(f"REF p1 vs p2 {q}_{s}: {d[:4]}")
AA = {}
for q in (2097091, 4194271):
    a, ma = load("AA1", q, 1)
    b, mb = load("AA2", q, 1)
    d = same(a, b) + same(a, U[("REF_p1", q, 1)])
    if d:
        g1.append(f"AA {q}: {d[:4]}")
    AA[q] = {"relations_cpu": [cpu(a), cpu(b)], "ratio_2_over_1": cpu(b) / cpu(a), "total_cpu_ratio_DL": tot(b, "DL") / tot(a, "DL"),
             "load1": [ma["load1"], mb["load1"]]}
floor = max(abs(v["ratio_2_over_1"] - 1) for v in AA.values())

gm = lambda v: math.exp(sum(map(math.log, v)) / len(v))
R = {}
for q in QS:
    for s in SEEDS:
        e = {}
        for key, f in (("rel_all", lambda r: cpu(r)), ("rel_D", lambda r: cpu(r, y="D")), ("rel_DL1", lambda r: cpu(r, y="DL1")),
                       ("rel_DL", lambda r: cpu(r, y="DL")), ("tot_D", lambda r: tot(r, "D")), ("tot_DL1", lambda r: tot(r, "DL1")),
                       ("tot_DL", lambda r: tot(r, "DL"))):
            ps = [f(U[(f"CHAIN_p{p}", q, s)]) / f(U[(f"REF_p{p}", q, s)]) for p in (1, 2)]
            e[key] = {"pass1": ps[0], "pass2": ps[1], "mean": sum(ps) / 2}
        e["ref_p2_over_p1_rel"] = cpu(U[("REF_p2", q, s)]) / cpu(U[("REF_p1", q, s)])
        R[f"{q}/{s}"] = e
prim = {k: v["rel_all"]["mean"] for k, v in R.items()}
mx = max(prim.values())
hl_below1 = mx < 1
hl_floor = mx < 1 - floor
decision = ("improvement in CPU (L0)" if hl_below1 and hl_floor else
            "below 1 at every (q, seed) but not separated from the A/A floor" if hl_below1 else "no robust improvement")
highload = sorted({(l, lab, q, s) for l, lab, q, s in loads if float(l) > 2.0})
# sensitivity: drop (q, seed) cells where any of its four units had load1 > 2.0 at lock acquisition
bad = {(q, s) for l, lab, q, s in loads if float(l) > 2.0}
prim_clean = {k: v for k, v in prim.items() if tuple(map(int, k.split("/"))) not in bad}
mx_clean = max(prim_clean.values()) if prim_clean else None
per_q = {q: {k: gm([R[f"{q}/{s}"][k]["mean"] for s in SEEDS]) for k in ("rel_all", "rel_D", "rel_DL1", "rel_DL", "tot_D", "tot_DL1", "tot_DL")} for q in QS}
out = {"gates": {"G1_counted_identical": not g1, "G1_mismatches": g1, "counted_fields_compared_ref_vs_chain": nfields,
                 "verify_failures": fails},
       "aa": AA, "aa_floor": floor, "cells": R, "per_q_geomean": per_q, "max_cell_ratio": mx, "headline_below_1": hl_below1,
       "headline_below_floor": hl_floor, "decision": decision, "load1_over_2_units": [list(h) for h in highload],
       "sensitivity_max_cell_ratio_excluding_load_gt_2": mx_clean, "excluded_cells": sorted(f"{q}/{s}" for q, s in bad)}
json.dump(out, open(os.path.join(HERE, "analysis.json"), "w"), indent=1)

L = ["# Compiled square root: tables (analyze.py)", "",
     f"G1 (counted fields identical, every REF/CHAIN pair, REF p1 vs p2, A/A): **{'PASS' if not g1 else 'FAIL'}**, "
     f"{nfields} counted fields compared REF vs CHAIN; verification failures: {len(fails)}.", "",
     f"A/A floor (|REF2/REF1 - 1| of summed relations.cpu, max over q = 2097091, 4194271): {floor:.4f} "
     f"(per size: " + ", ".join(f"{q}: {v['ratio_2_over_1']:.4f}" for q, v in AA.items()) + ").", "",
     f"Primary: relations.cpu CHAIN/REF (summed over D, DL1, DL arms and attempts; mean of the two passes). "
     f"Max over the 15 cells: {mx:.4f}. Below 1 everywhere: {hl_below1}. Below 1 - floor ({1 - floor:.4f}): {hl_floor}. "
     f"**Decision: {decision}.**", "",
     "| q | s1 | s2 | s3 | geomean | DL arm only | total CPU DL | total CPU DL1 | total CPU D |", "|--:|--:|--:|--:|--:|--:|--:|--:|--:|"]
for q in QS:
    c = [R[f"{q}/{s}"]["rel_all"]["mean"] for s in SEEDS]
    L.append(f"| {q} | " + " | ".join(f"{x:.3f}" for x in c) + f" | {per_q[q]['rel_all']:.3f} | {per_q[q]['rel_DL']:.3f} | "
             f"{per_q[q]['tot_DL']:.3f} | {per_q[q]['tot_DL1']:.3f} | {per_q[q]['tot_D']:.3f} |")
L += ["", f"Cells with any unit at load1 > 2.0 at lock acquisition: {sorted(f'{q}/{s}' for q, s in bad) or 'none'}; "
      f"max cell ratio without them: {mx_clean if mx_clean is None else round(mx_clean, 4)}.", "",
      "Pass-to-pass REF relations.cpu ratio (p2/p1): " + ", ".join(f"{k}: {v['ref_p2_over_p1_rel']:.3f}" for k, v in R.items()), ""]
open(os.path.join(HERE, "tables.md"), "w").write("\n".join(L) + "\n")
print("\n".join(L[:12]))
