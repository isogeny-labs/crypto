#!/usr/bin/env python3
"""Incremental factor-base growth vs the charged restart (REF), scored by PROTOCOL.md. usage: analyze.py RESULTS_DIR
RESULTS_DIR/units/<ARM>_<q>_<seed>.jsonl.  Writes analysis.json and tables.md next to this file."""
import json, math, os, sys

HERE = os.path.dirname(os.path.abspath(__file__))
N4V2 = os.path.join(HERE, "..", "quartic_n4v2_20261020", "results")
COMBO = os.path.join(HERE, "..", "quartic_dl1_cd_sqrt_20261113", "combo", "runs", "units")
PH = ["fb", "relations", "matrix", "linear_algebra", "log_recovery", "descent", "verify"]
QS = [262111, 524287, 1048507, 2097091, 4194271]
SEEDS = (1, 2, 3)
GARMS = ["G160", "G155"]
YS = ["DL", "DL1", "D"]
WADD = [0.0, 0.5, 1.0]
HASH = [0.0, 0.1]           # decisive hash prices (M-eq per hash op)
HASH_SENS = [1.0]           # reported, not decisive
TIMING = ("wall", "cpu")
res = sys.argv[1]


def jl(p):
    out = []
    for l in open(p):
        l = l.strip()
        if l:
            try:
                out.append(json.loads(l))
            except Exception:
                pass
    return out


C = {}
for r in jl(os.path.join(N4V2, "summary.json")):
    if r.get("kind") == "ic_size" and r["arm"] == "DL":
        C[r["q"]] = (r["r_inv"], r["r_lmul"])
RHO = {}
_main = jl(os.path.join(N4V2, "summary.json"))
_supp = jl(os.path.join(N4V2, "summary_supp.json"))
for recs, only22 in ((_main, False), (_supp, True)):
    excl = {(r["arm"], r["q"]) for r in recs if r.get("kind") == "rho_excluded"}
    for r in recs:
        if r.get("kind") == "rho_size" and (r["arm"], r["q"]) not in excl and (r["q"] == 4194271) == only22:
            RHO[(r["arm"], r["q"])] = r["meq"]


def unit(arm, q, s):
    return jl(os.path.join(res, "units", f"{arm}_{q}_{s}.jsonl"))


def ic(arm, q, s, y=None):
    return [r for r in unit(arm, q, s) if r.get("rec") == "ic" and (y is None or r["arm"] == y)]


def meq(rs, w, h=0.0, phases=PH):
    ri, rl = C[rs[0]["q"]]
    return sum(r.get(p + ".mul", 0) + ri * r.get(p + ".inv", 0) + rl * r.get(p + ".lmul", 0)
               + w * (r.get(p + ".add", 0) + r.get(p + ".ladd", 0)) + h * r.get(p + ".hash", 0)
               for r in rs for p in phases)


def cpu(rs):
    return sum(r.get(p + ".cpu", 0) for r in rs for p in PH)


def counted(r):
    return {k: v for k, v in r.items() if not k.endswith(TIMING) and k not in ("load1", "rec") and not k.startswith("s.controls")}


gm = lambda v: math.exp(sum(map(math.log, v)) / len(v))
ARMS = ["REF"] + GARMS

# gates
gates = {"units_present": 0, "units_expected": len(ARMS) * len(QS) * len(SEEDS), "rc_nonzero": [], "verify_failures": [],
         "ref_retries": [], "growth_attempts": {}, "aa_ref_vs_combo_B170": {"compared": 0, "mismatches": []},
         "load1_max": 0.0, "load1_gt2": []}
for a in ARMS:
    for q in QS:
        for s in SEEDS:
            p = os.path.join(res, "units", f"{a}_{q}_{s}.jsonl")
            if not os.path.exists(p):
                continue
            gates["units_present"] += 1
            u = [r for r in unit(a, q, s) if r.get("rec") == "unit"]
            if not u or u[0]["rc"] != 0:
                gates["rc_nonzero"].append(f"{a}_{q}_{s}")
            else:
                l1 = float(u[0]["load1"])
                gates["load1_max"] = max(gates["load1_max"], l1)
                if l1 > 2.0:
                    gates["load1_gt2"].append(f"{a}_{q}_{s}:{l1}")
            for y in YS:
                rs = ic(a, q, s, y)
                if not rs or not rs[-1]["ok"] or rs[-1]["k_found"] != rs[-1]["k"]:
                    gates["verify_failures"].append(f"{a}_{q}_{s}/{y}")
                    continue
                if a == "REF" and len(rs) > 1:
                    gates["ref_retries"].append(f"{q}_{s}/{y}:{len(rs)}")
                if a != "REF" and y == "DL":
                    gates["growth_attempts"][f"{a}/{q}/{s}"] = len(rs)
            if a == "REF":
                for y in YS:
                    x = ic("REF", q, s, y)
                    bpath = os.path.join(COMBO, f"B170_{q}_{s}.jsonl")
                    b = [r for r in jl(bpath) if r.get("rec") == "ic" and r["arm"] == y] if os.path.exists(bpath) else []
                    gates["aa_ref_vs_combo_B170"]["compared"] += 1
                    if len(x) != len(b):
                        gates["aa_ref_vs_combo_B170"]["mismatches"].append(f"{q}_{s}/{y}: records {len(x)} vs {len(b)}")
                        continue
                    for r1, r2 in zip(x, b):
                        c1, c2 = counted(r1), counted(r2)
                        d = [k for k in c1 if k in c2 and c1[k] != c2[k]] + [k for k in c1 if k not in c2]
                        if d:
                            gates["aa_ref_vs_combo_B170"]["mismatches"].append(f"{q}_{s}/{y}: {d[:5]}")

complete = gates["units_present"] == gates["units_expected"]
QS = [q for q in QS if all(os.path.exists(os.path.join(res, "units", f"{a}_{q}_{s}.jsonl")) for a in ARMS for s in SEEDS)]  # partial dirs only
GRID = [(w, h) for h in HASH + HASH_SENS for w in WADD]
ratios = {}
cells = {}
for g in GARMS:
    for y in YS:
        for q in QS:
            for (w, h) in GRID:
                rr = [meq(ic(g, q, s, y), w, h) / meq(ic("REF", q, s, y), w, h) for s in SEEDS]
                ratios[f"{g}/{y}/{q}/w{w}/h{h}"] = {"geomean": gm(rr), "per_seed": rr, "max": max(rr), "min": min(rr)}
            cp = [cpu(ic(g, q, s, y)) / cpu(ic("REF", q, s, y)) for s in SEEDS]
            cells[f"{g}/{y}/{q}"] = {"cpu_ratio_per_seed": cp, "cpu_geomean": gm(cp),
                                     "attempts_per_seed": [len(ic(g, q, s, y)) for s in SEEDS]}
# decision (DL only)
verdict = {}
for g in GARMS:
    worst = 0.0
    fails = []
    for q in QS:
        for (w, h) in [(w, h) for h in HASH for w in WADD]:
            mx = ratios[f"{g}/DL/{q}/w{w}/h{h}"]["max"]
            worst = max(worst, mx)
            if not mx < 1:
                fails.append(f"q={q} w={w} h={h}: max seed ratio {mx:.3f}")
    verdict[g] = {"improvement": (not fails) and complete and not gates["verify_failures"] and not gates["rc_nonzero"],
                  "worst_cell_ratio_decisive_grid": worst, "violations": len(fails), "violation_list": fails[:40]}
headline = "improvement" if any(v["improvement"] for v in verdict.values()) else "null"

# per-phase DL ratio at w=0, h=0 (geometric mean over seeds) and hash share
phase = {}
for g in GARMS:
    for q in QS:
        for p in PH:
            rr = []
            for s in SEEDS:
                a, b = meq(ic(g, q, s, "DL"), 0.0, 0.0, [p]), meq(ic("REF", q, s, "DL"), 0.0, 0.0, [p])
                if b > 0:
                    rr.append(a / b)
            phase[f"{g}/{q}/{p}"] = gm(rr) if rr and all(x > 0 for x in rr) else None
hashops = {}
for a in ARMS:
    for q in QS:
        hashops[f"{a}/{q}"] = [sum(r.get(p + ".hash", 0) for r in ic(a, q, s, "DL") for p in PH) for s in SEEDS]
# gap to matched plain rho (w=0 and w=1 rho prices), DL arm, additions free, hash free: distance remaining
gap = {}
for a in ARMS:
    for q in QS:
        m = gm([meq(ic(a, q, s, "DL"), 0.0, 0.0) for s in SEEDS])
        rho = RHO.get(("plain", q))
        if rho:
            gap[f"{a}/{q}"] = {"DL_meq_geomean": m, "rho_meq_w0": rho[0], "rho_meq_w1": rho[2],
                               "ratio_to_rho_w0": m / rho[0], "ratio_to_rho_w1": m / rho[2]}
out = {"complete": complete, "gates": gates, "verdict": verdict, "headline": headline, "ratios": ratios, "cells": cells,
       "phase_ratio_DL_w0_h0": phase, "hash_ops_DL": hashops, "gap_to_plain_rho": gap}
json.dump(out, open(os.path.join(HERE, "analysis.json"), "w"), indent=1)

L = ["# Incremental factor-base growth vs charged restart: tables (analyze.py)", "",
     f"Units present {gates['units_present']}/{gates['units_expected']}; rc != 0: {len(gates['rc_nonzero'])}; "
     f"verification failures: {len(gates['verify_failures'])}; REF retries: {len(gates['ref_retries'])}; "
     f"A/A REF vs combo B170: {gates['aa_ref_vs_combo_B170']['compared']} arm records, "
     f"{len(gates['aa_ref_vs_combo_B170']['mismatches'])} mismatches; load1 max {gates['load1_max']:.2f} "
     f"(> 2.0: {len(gates['load1_gt2'])}).", "",
     f"**Pre-registered verdict: {headline.upper()}** " + "; ".join(
         f"{g}: {'improvement' if v['improvement'] else 'not an improvement'} (worst cell {v['worst_cell_ratio_decisive_grid']:.3f}, "
         f"{v['violations']} violating grid cells)" for g, v in verdict.items()), "",
     "## DL M-eq ratio G / REF (geometric mean over seeds; per-seed max in brackets; ✗ = some seed >= 1)", "",
     "| arm | w_add | h | k = 18 | 19 | 20 | 21 | 22 |", "|:--|--:|--:|--:|--:|--:|--:|--:|"]
for g in GARMS:
    for h in HASH + HASH_SENS:
        for w in WADD:
            cs = []
            for q in QS:
                r = ratios[f"{g}/DL/{q}/w{w}/h{h}"]
                cs.append(f"{r['geomean']:.3f} [{r['max']:.3f}]" + ("" if r["max"] < 1 else " ✗"))
            L.append(f"| {g} | {w} | {h}{' (sens.)' if h in HASH_SENS else ''} | " + " | ".join(cs) + " |")
L += ["", "## DL1 and D at w_add = 0, h = 0 and h = 0.1 (geometric mean over seeds)", "",
      "| arm | IC arm | h | k = 18 | 19 | 20 | 21 | 22 |", "|:--|:--|--:|--:|--:|--:|--:|--:|"]
for g in GARMS:
    for y in ("DL1", "D"):
        for h in HASH:
            L.append(f"| {g} | {y} | {h} | " + " | ".join(f"{ratios[f'{g}/{y}/{q}/w0.0/h{h}']['geomean']:.3f}" for q in QS) + " |")
L += ["", "## Secondary: total CPU ratio G / REF, DL arm (seeds 1, 2, 3; L0 diagnostic, A/A noise up to several percent)", "",
      "| arm | k = 18 | 19 | 20 | 21 | 22 |", "|:--|:--|:--|:--|:--|:--|"]
for g in GARMS:
    L.append(f"| {g} | " + " | ".join(" ".join(f"{x:.2f}" for x in cells[f'{g}/DL/{q}']['cpu_ratio_per_seed']) for q in QS) + " |")
L += ["", "## Attempt records per DL unit (seeds 1, 2, 3); REF has one each unless listed under REF retries", "",
      "| arm | k = 18 | 19 | 20 | 21 | 22 |", "|:--|:--|:--|:--|:--|:--|"]
for g in GARMS:
    L.append(f"| {g} | " + " | ".join(" ".join(str(x) for x in cells[f'{g}/DL/{q}']['attempts_per_seed']) for q in QS) + " |")
L += ["", "## Per-phase DL M-eq ratio G / REF (w = 0, h = 0; geometric mean over seeds)", "",
      "| arm | phase | k = 18 | 19 | 20 | 21 | 22 |", "|:--|:--|--:|--:|--:|--:|--:|"]
for g in GARMS:
    for p in PH:
        L.append(f"| {g} | {p} | " + " | ".join("-" if phase[f'{g}/{q}/{p}'] is None else f"{phase[f'{g}/{q}/{p}']:.3f}" for q in QS) + " |")
L += ["", "## Counted hash operations, DL arm, geometric mean over seeds", "", "| arm | k = 18 | 19 | 20 | 21 | 22 |", "|:--|--:|--:|--:|--:|--:|"]
for a in ARMS:
    L.append(f"| {a} | " + " | ".join(f"{gm([max(x, 1) for x in hashops[f'{a}/{q}']]):.3g}" for q in QS) + " |")
L += ["", "## Distance remaining: DL M-eq / matched plain rho M-eq (w_add = 0, h = 0; rho at its w = 0 and w = 1 prices)", "",
      "| arm | k = 18 | 19 | 20 | 21 | 22 |", "|:--|--:|--:|--:|--:|--:|"]
for a in ARMS:
    L.append(f"| {a} | " + " | ".join(f"{gap[f'{a}/{q}']['ratio_to_rho_w0']:.5f} / {gap[f'{a}/{q}']['ratio_to_rho_w1']:.5f}" for q in QS) + " |")
open(os.path.join(HERE, "tables.md"), "w").write("\n".join(L) + "\n")
print("\n".join(L[:6]))
