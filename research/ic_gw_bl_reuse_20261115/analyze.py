#!/usr/bin/env python3
"""Scores PROTOCOL.md. usage: analyze.py RESULTS_DIR   (RESULTS_DIR/units/{ARM}_{q}_{seed}.jsonl)
Writes analysis.json and tables.md next to this file. Every number is [pcc:unverified]."""
import json, math, os, sys, tarfile

HERE = os.path.dirname(os.path.abspath(__file__))
RES = sys.argv[1]
R = os.path.join(HERE, "..")
PH = ["fb", "relations", "matrix", "linear_algebra", "log_recovery", "descent", "verify"]
QS = [262111, 524287, 1048507, 2097091, 4194271]
SEEDS = (1, 2, 3, 4, 5, 6)
YS = ["DL", "DL1", "D"]
WADD = [0.0, 0.5, 1.0]
WH = [0.0, 0.1, 1.0]
C, GAE = {}, {}
for line in open(os.path.join(R, "quartic_n4v2_20261020", "results", "summary.json")):
    r = json.loads(line)
    if r.get("kind") == "ic_size" and r["arm"] == "DL":
        C[r["q"]] = (r["r_inv"], r["r_lmul"]); GAE[r["q"]] = r["gae"]
RHO = {k: v["rho"] for k, v in json.load(open(os.path.join(
    R, "quartic_dl1_cd_sqrt_20261113", "combo", "validation", "VALIDATION.json")))["rho_reference"]["gae_band"].items()}
RHO_IC_OLD = {k: v["ic"] for k, v in json.load(open(os.path.join(
    R, "quartic_dl1_cd_sqrt_20261113", "combo", "validation", "VALIDATION.json")))["rho_reference"]["gae_band"].items()}


def load(path):
    return [json.loads(l) for l in open(path) if l.strip()]


def recs(rows, y=None):
    return [r for r in rows if r.get("rec") == "ic" and (y is None or r["arm"] == y)]


def unit_rows(arm, q, s):
    return load(os.path.join(RES, "units", f"{arm}_{q}_{s}.jsonl"))


def meq(rs, wadd, wh=0.0):
    ri, rl = C[rs[0]["q"]]; g = GAE[rs[0]["q"]]
    return sum(r.get(p + ".mul", 0) + ri * r.get(p + ".inv", 0) + rl * r.get(p + ".lmul", 0)
               + wadd * (r.get(p + ".add", 0) + r.get(p + ".ladd", 0))
               + wh * g * (r.get(p + ".hash", 0) + r.get(p + ".cmp", 0)) for r in rs for p in PH)


def counted(r):
    return {k: v for k, v in r.items() if not k.endswith(("wall", "cpu")) and k != "load1" and not k.startswith("s.controls")}


gm = lambda v: math.exp(sum(map(math.log, v)) / len(v))
ALLOWED = {"linear_algebra.lmul", "linear_algebra.ladd"}
g1 = {"pairs": 0, "mismatches": [], "la_diffs": 0}
g2 = {"pairs": 0, "mismatches": []}
g3 = {"failures": []}
data = {}
for q in QS:
    for s in SEEDS:
        a, b = unit_rows("REF", q, s), unit_rows("BLR", q, s)
        data[(q, s)] = (a, b)
        for arm, rows in (("REF", a), ("BLR", b)):
            u = [r for r in rows if r.get("rec") == "unit"]
            if len(u) != 1 or u[0]["rc"] != 0:
                g3["failures"].append(f"{arm} {q}_{s}: unit rc")
            for r in recs(rows):
                if not r["ok"] or r["k_found"] != r["k"]:
                    g3["failures"].append(f"{arm} {q}_{s}/{r['arm']}: not ok")
        ra, rb = recs(a), recs(b)
        g1["pairs"] += 1
        if len(ra) != len(rb):
            g1["mismatches"].append(f"{q}_{s}: record count {len(ra)} vs {len(rb)}"); continue
        for x, z in zip(ra, rb):
            cx, cz = counted(x), counted(z)
            if set(cx) != set(cz):
                g1["mismatches"].append(f"{q}_{s}/{x['arm']}: key sets differ"); continue
            d = [k for k in cx if cx[k] != cz[k]]
            bad = [k for k in d if not (x["arm"] == "DL" and k in ALLOWED)]
            if bad:
                g1["mismatches"].append(f"{q}_{s}/{x['arm']}: {bad[:6]}")
            if x["arm"] == "DL" and d:
                g1["la_diffs"] += 1
        g2["pairs"] += 1
        for x, z in zip(recs(a, "DL"), recs(b, "DL")):
            if (x.get("s.bl_iters"), x.get("s.bl_narrowed")) != (z.get("s.bl_iters"), z.get("s.bl_narrowed")):
                g2["mismatches"].append(f"{q}_{s}")
g1["pass"] = not g1["mismatches"]; g2["pass"] = not g2["mismatches"]; g3["pass"] = not g3["failures"]

# G4: REF against the committed COMBO records of the fasthash study
g4 = {"compared": 0, "mismatches": []}
tf = tarfile.open(os.path.join(R, "ic_gw_fasthash_20261114", "runs-fasthash-20261114.tar.gz"))
for q in QS:
    for s in (1, 2, 3):
        mine = recs(data[(q, s)][0])
        for p in (1, 2):
            old = recs([json.loads(l) for l in tf.extractfile(f"./units/p{p}_COMBO_{q}_{s}.jsonl").read().decode().splitlines() if l.strip()])
            g4["compared"] += 1
            if len(old) != len(mine):
                g4["mismatches"].append(f"{q}_{s} p{p}: record count"); continue
            for x, z in zip(mine, old):
                if counted(x) != counted(z):
                    g4["mismatches"].append(f"{q}_{s} p{p}/{x['arm']}")
g4["pass"] = not g4["mismatches"]

ratios = {}
for y in YS:
    for q in QS:
        for w in WADD:
            for h in WH:
                rr = [meq(recs(data[(q, s)][1], y), w, h) / meq(recs(data[(q, s)][0], y), w, h) for s in SEEDS]
                ratios[f"{y}/{q}/{w}/{h}"] = {"geomean": gm(rr), "max": max(rr), "min": min(rr), "per_seed": rr}
la = {}
for q in QS:
    rr = [recs(data[(q, s)][1], "DL")[-1]["linear_algebra.lmul"] / recs(data[(q, s)][0], "DL")[-1]["linear_algebra.lmul"] for s in SEEDS]
    la[str(q)] = {"geomean": gm(rr), "per_seed": rr}
cells_primary = [ratios[f"DL/{q}/{w}/0.0"]["per_seed"] for q in QS for w in WADD]
supported = all(x < 1 for c in cells_primary for x in c) and g1["pass"] and g2["pass"] and g3["pass"] and g4["pass"]
band_all = all(x < 1 for q in QS for w in WADD for h in WH for x in ratios[f"DL/{q}/{w}/{h}"]["per_seed"])
other_one = {y: all(abs(x - 1) < 1e-12 for q in QS for w in WADD for h in WH for x in ratios[f"{y}/{q}/{w}/{h}"]["per_seed"]) for y in ("DL1", "D")}

# gap to matched negation-map rho at k = 22, seeds 1-3, w_add = 0
q = 4194271; gap = {}
for tag, ai in (("REF", 0), ("BLR", 1)):
    for wk in ("0", "0.1", "1"):
        vals = [meq(recs(data[(q, s)][ai], "DL"), 0.0, float(wk)) for s in (1, 2, 3)]
        gap[f"{tag}/{wk}"] = sum(vals) / 3
repro = {wk: {"ref": gap[f"REF/{wk}"], "old": RHO_IC_OLD[wk], "rel_err": abs(gap[f"REF/{wk}"] - RHO_IC_OLD[wk]) / RHO_IC_OLD[wk]} for wk in ("0", "0.1", "1")}
reproduced = all(v["rel_err"] < 1e-9 for v in repro.values())
rho_over_ic = {f"{t}/{wk}": RHO[wk] / gap[f"{t}/{wk}"] for t in ("REF", "BLR") for wk in ("0", "0.1", "1")}
ic_over_rho = {k: 1 / v for k, v in rho_over_ic.items()}

out = {"gates": {"G1": g1, "G2": g2, "G3": g3, "G4": g4}, "ratios": ratios, "la_lmul_ratio": la,
       "supported": supported, "band_all_below_1": band_all, "other_arms_identical": other_one,
       "gap_k22": {"ic_mean_meq": gap, "ref_reproduces_old_band": reproduced, "repro": repro,
                   "rho": RHO, "rho_over_ic": rho_over_ic, "ic_over_rho": ic_over_rho}}
json.dump(out, open(os.path.join(HERE, "analysis.json"), "w"), indent=1)

L = ["# BLR / REF tables (analyze.py) [pcc:unverified]", "",
     f"G1 {'pass' if g1['pass'] else 'FAIL'} ({g1['pairs']} pairs, {g1['la_diffs']} with a changed DL linear-algebra field, {len(g1['mismatches'])} other mismatches); "
     f"G2 {'pass' if g2['pass'] else 'FAIL'}; G3 {'pass' if g3['pass'] else 'FAIL'} ({len(g3['failures'])} failures); "
     f"G4 {'pass' if g4['pass'] else 'FAIL'} ({g4['compared']} unit files, {len(g4['mismatches'])} mismatches).", "",
     f"Decision (every (q, seed) and w_add in 0, 0.5, 1, w_h = 0): **{'SUPPORTED' if supported else 'NOT SUPPORTED'}**; also below 1 at every w_h in 0, 0.1, 1: {band_all}.", "",
     "DL cold M-eq BLR/REF (w_h = 0): geomean (max) over seeds 1-6", "",
     "| w_add | k = 18 | 19 | 20 | 21 | 22 |", "|--:|--:|--:|--:|--:|--:|"]
for w in WADD:
    L.append(f"| {w} | " + " | ".join(f"{ratios[f'DL/{q}/{w}/0.0']['geomean']:.4f} ({ratios[f'DL/{q}/{w}/0.0']['max']:.4f})" for q in QS) + " |")
L += ["", "Same, w_h = 0.1 and 1 GAE on hash + cmp (geomean):", "", "| w_add | w_h | k = 18 | 19 | 20 | 21 | 22 |", "|--:|--:|--:|--:|--:|--:|--:|"]
for w in WADD:
    for h in (0.1, 1.0):
        L.append(f"| {w} | {h} | " + " | ".join(f"{ratios[f'DL/{q}/{w}/{h}']['geomean']:.4f}" for q in QS) + " |")
L += ["", f"DL1 and D identical to 1 at every cell: DL1 {other_one['DL1']}, D {other_one['D']}.", "",
      "DL linear_algebra.lmul BLR/REF, geomean over seeds: " + ", ".join(f"k={int(math.log2(q)+0.5)}: {la[str(q)]['geomean']:.4f}" for q in QS), "",
      f"Gap at k = 22 (seeds 1-3, w_add = 0, hash+cmp at w*gae). REF reproduces the frozen combo band (rel. err < 1e-9): **{'yes' if reproduced else 'no'}**.", "",
      "| w | rho (frozen neg-map) | IC REF | IC BLR | rho/IC REF | rho/IC BLR | IC/rho BLR |", "|--:|--:|--:|--:|--:|--:|--:|"]
for wk in ("0", "0.1", "1"):
    L.append(f"| {wk} | {RHO[wk]:.4e} | {gap[f'REF/{wk}']:.4e} | {gap[f'BLR/{wk}']:.4e} | {rho_over_ic[f'REF/{wk}']:.1f} | {rho_over_ic[f'BLR/{wk}']:.1f} | {ic_over_rho[f'BLR/{wk}']:.5f} |")
open(os.path.join(HERE, "tables.md"), "w").write("\n".join(L) + "\n")
print("\n".join(L[:6]))
