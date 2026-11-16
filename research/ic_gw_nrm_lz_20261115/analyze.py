#!/usr/bin/env python3
"""Scores PROTOCOL.md. usage: analyze.py RESULTS_DIR   (RESULTS_DIR/units/{ARM}_{q}_{seed}.jsonl)
Writes analysis.json and tables.md next to this file. Every number is [pcc:unverified]."""
import json, math, os, sys

HERE = os.path.dirname(os.path.abspath(__file__))
RES = sys.argv[1]
R = os.path.join(HERE, "..")
PH = ["fb", "relations", "matrix", "linear_algebra", "log_recovery", "descent", "verify"]
QS = [262111, 524287, 1048507, 2097091, 4194271]
SEEDS = (1, 2, 3, 4, 5, 6)
ARMS = ["REF", "A0", "NRM", "LZ", "BOTH"]
YS = ["DL", "DL1", "D"]
WADD = [0.0, 0.5, 1.0]
WH = [0.0, 0.1, 1.0]
C, GAE = {}, {}
for line in open(os.path.join(R, "quartic_n4v2_20261020", "results", "summary.json")):
    r = json.loads(line)
    if r.get("kind") == "ic_size" and r["arm"] == "DL":
        C[r["q"]] = (r["r_inv"], r["r_lmul"]); GAE[r["q"]] = r["gae"]
VAL = json.load(open(os.path.join(R, "ic_gw_bl_reuse_20261115", "validation", "VALIDATION.json")))["rho_reference"]["gae_band"]
RHO = {k: v["rho"] for k, v in VAL.items()}
IC_OLD = {k: v["ic"] for k, v in VAL.items()}


def load(path):
    return [json.loads(l) for l in open(path) if l.strip()]


def recs(rows, y=None):
    return [r for r in rows if r.get("rec") == "ic" and (y is None or r["arm"] == y)]


def meq(rs, wadd, wh=0.0):
    ri, rl = C[rs[0]["q"]]; g = GAE[rs[0]["q"]]
    return sum(r.get(p + ".mul", 0) + ri * r.get(p + ".inv", 0) + rl * r.get(p + ".lmul", 0)
               + wadd * (r.get(p + ".add", 0) + r.get(p + ".ladd", 0))
               + wh * g * (r.get(p + ".hash", 0) + r.get(p + ".cmp", 0)) for r in rs for p in PH)


def counted(r):
    return {k: v for k, v in r.items() if not k.endswith(("wall", "cpu")) and k != "load1" and not k.startswith("s.controls")}


gm = lambda v: math.exp(sum(map(math.log, v)) / len(v))
REL = {"relations.mul", "relations.add"}
LAL = {"linear_algebra.lmul", "linear_algebra.ladd"}


def allowed(arm, y, k):
    ok = False
    if arm in ("NRM", "BOTH") and k in REL: ok = True
    if arm in ("LZ", "BOTH") and y in ("D", "DL1") and k in LAL: ok = True
    return ok


D = {}
g0 = {"pairs": 0, "mismatches": []}
g1 = {"pairs": 0, "mismatches": [], "changed": {a: 0 for a in ARMS[2:]}}
g2 = {"failures": []}
g3 = {"mismatches": []}
for q in QS:
    for s in SEEDS:
        for a in ARMS:
            D[(a, q, s)] = load(os.path.join(RES, "units", f"{a}_{q}_{s}.jsonl"))
            u = [r for r in D[(a, q, s)] if r.get("rec") == "unit"]
            if len(u) != 1 or u[0]["rc"] != 0:
                g2["failures"].append(f"{a} {q}_{s}: unit rc")
            for r in recs(D[(a, q, s)]):
                if not r["ok"] or r["k_found"] != r["k"]:
                    g2["failures"].append(f"{a} {q}_{s}/{r['arm']}: not ok")
        ref = recs(D[("REF", q, s)])
        for a in ARMS[1:]:
            ra = recs(D[(a, q, s)])
            tag = f"{a} {q}_{s}"
            if len(ra) != len(ref):
                (g0 if a == "A0" else g1)["mismatches"].append(f"{tag}: record count"); continue
            (g0 if a == "A0" else g1)["pairs"] += 1
            chg = False
            for x, z in zip(ref, ra):
                cx, cz = counted(x), counted(z)
                if set(cx) != set(cz):
                    (g0 if a == "A0" else g1)["mismatches"].append(f"{tag}/{x['arm']}: key sets"); continue
                d = [k for k in cx if cx[k] != cz[k]]
                if a == "A0":
                    if d: g0["mismatches"].append(f"{tag}/{x['arm']}: {d[:6]}")
                else:
                    bad = [k for k in d if not allowed(a, x["arm"], k)]
                    if bad: g1["mismatches"].append(f"{tag}/{x['arm']}: {bad[:6]}")
                    if d: chg = True
                sx = {k: v for k, v in x.items() if k.startswith("s.") and not k.startswith("s.controls")}
                sz = {k: v for k, v in z.items() if k.startswith("s.") and not k.startswith("s.controls")}
                if sx != sz: g3["mismatches"].append(f"{tag}/{x['arm']}")
            if a != "A0" and chg: g1["changed"][a] += 1
for g in (g0, g1, g2, g3): g["pass"] = not (g.get("mismatches") or g.get("failures"))

ratios = {}
for a in ARMS[2:]:
    for y in YS:
        for q in QS:
            for w in WADD:
                for h in WH:
                    rr = [meq(recs(D[(a, q, s)], y), w, h) / meq(recs(D[("REF", q, s)], y), w, h) for s in SEEDS]
                    ratios[f"{a}/{y}/{q}/{w}/{h}"] = {"geomean": gm(rr), "max": max(rr), "min": min(rr), "per_seed": rr}
lmul = {}
for a in ("LZ", "BOTH"):
    for y in ("D", "DL1"):
        lmul[f"{a}/{y}"] = {str(q): gm([sum(r["linear_algebra.lmul"] for r in recs(D[(a, q, s)], y)) /
                                         sum(r["linear_algebra.lmul"] for r in recs(D[("REF", q, s)], y)) for s in SEEDS]) for q in QS}
relmul = {a: {str(q): gm([sum(r["relations.mul"] for r in recs(D[(a, q, s)], "DL")) /
                           sum(r["relations.mul"] for r in recs(D[("REF", q, s)], "DL")) for s in SEEDS]) for q in QS} for a in ("NRM", "BOTH")}

dec = {}
for a in ARMS[2:]:
    dec[a] = {}
    for y in YS:
        prim = all(x < 1 for q in QS for w in WADD for x in ratios[f"{a}/{y}/{q}/{w}/0.0"]["per_seed"])
        band = all(x < 1 for q in QS for w in WADD for h in WH for x in ratios[f"{a}/{y}/{q}/{w}/{h}"]["per_seed"])
        mx = max(x for q in QS for w in WADD for x in ratios[f"{a}/{y}/{q}/{w}/0.0"]["per_seed"])
        dec[a][y] = {"improvement": prim and g0["pass"] and g1["pass"] and g2["pass"] and g3["pass"], "all_cells_below_1": prim, "band_below_1": band, "max_ratio": mx}

# gap to matched negation-map rho at k = 22, seeds 1-3, w_add = 0
q = 4194271; gap = {}
for a in ("REF", "BOTH"):
    for y in ("DL", "DL1"):
        for wk in ("0", "0.1", "1"):
            gap[f"{a}/{y}/{wk}"] = sum(meq(recs(D[(a, q, s)], y), 0.0, float(wk)) for s in (1, 2, 3)) / 3
repro = {wk: {"ref": gap[f"REF/DL/{wk}"], "old": IC_OLD[wk], "rel_err": abs(gap[f"REF/DL/{wk}"] - IC_OLD[wk]) / IC_OLD[wk]} for wk in ("0", "0.1", "1")}
reproduced = all(v["rel_err"] < 1e-9 for v in repro.values())
rho_over_ic = {k: RHO[k.split("/")[2]] / v for k, v in gap.items()}

out = {"gates": {"G0": g0, "G1": g1, "G2": g2, "G3": g3}, "ratios": ratios, "lmul_ratio": lmul, "relations_mul_ratio": relmul, "decision": dec,
       "gap_k22": {"ic_mean_meq": gap, "ref_reproduces_band": reproduced, "repro": repro, "rho": RHO, "rho_over_ic": rho_over_ic}}
json.dump(out, open(os.path.join(HERE, "analysis.json"), "w"), indent=1)

ps = lambda g: "pass" if g["pass"] else "FAIL"
L = ["# NRM / LZ tables (analyze.py) [pcc:unverified]", "",
     f"G0 {ps(g0)} ({g0['pairs']} A0 units, {len(g0['mismatches'])} mismatches); G1 {ps(g1)} ({g1['pairs']} arm units, {len(g1['mismatches'])} mismatches; units with changed fields: {g1['changed']}); "
     f"G2 {ps(g2)} ({len(g2['failures'])} failures); G3 {ps(g3)} ({len(g3['mismatches'])} mismatches).", "",
     "Decision (every (q, seed), w_add in 0, 0.5, 1, w_h = 0; ratio < 1):", "",
     "| arm | y | improvement | max ratio | also below 1 for all w_h |", "|--|--|--|--:|--|"]
for a in ARMS[2:]:
    for y in YS:
        d = dec[a][y]; L.append(f"| {a} | {y} | {d['improvement']} | {d['max_ratio']:.4f} | {d['band_below_1']} |")
for a in ARMS[2:]:
    for y in YS:
        L += ["", f"{a}/REF {y} cold M-eq (w_h = 0): geomean (max) over seeds 1-6", "", "| w_add | k = 18 | 19 | 20 | 21 | 22 |", "|--:|--:|--:|--:|--:|--:|"]
        for w in WADD:
            L.append(f"| {w} | " + " | ".join(f"{ratios[f'{a}/{y}/{q}/{w}/0.0']['geomean']:.4f} ({ratios[f'{a}/{y}/{q}/{w}/0.0']['max']:.4f})" for q in QS) + " |")
L += ["", "relations.mul ratio (DL, geomean): " + "; ".join(f"{a}: " + ", ".join(f"{v:.4f}" for v in relmul[a].values()) for a in relmul),
      "linear_algebra.lmul ratio (geomean): " + "; ".join(f"{k}: " + ", ".join(f"{v:.4f}" for v in d.values()) for k, d in lmul.items()), "",
      f"Gap at k = 22 (seeds 1-3, w_add = 0, hash+cmp at w*gae). REF reproduces the frozen band IC values (rel. err < 1e-9): **{'yes' if reproduced else 'no'}**.", "",
      "| w | rho (frozen neg-map) | IC REF DL | IC BOTH DL | rho/IC REF | rho/IC BOTH | IC/rho BOTH | IC BOTH DL1 | IC/rho BOTH DL1 |", "|--:|--:|--:|--:|--:|--:|--:|--:|--:|"]
for wk in ("0", "0.1", "1"):
    L.append(f"| {wk} | {RHO[wk]:.4e} | {gap[f'REF/DL/{wk}']:.4e} | {gap[f'BOTH/DL/{wk}']:.4e} | {rho_over_ic[f'REF/DL/{wk}']:.1f} | {rho_over_ic[f'BOTH/DL/{wk}']:.1f} | {1/rho_over_ic[f'BOTH/DL/{wk}']:.5f} | {gap[f'BOTH/DL1/{wk}']:.4e} | {1/rho_over_ic[f'BOTH/DL1/{wk}']:.5f} |")
open(os.path.join(HERE, "tables.md"), "w").write("\n".join(L) + "\n")
print("\n".join(L[:14]))
