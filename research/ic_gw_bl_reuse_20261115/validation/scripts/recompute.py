#!/usr/bin/env python3
"""Validator recompute for ic_gw_bl_reuse_20261115 from the raw tarball (written from PROTOCOL.md and the model in
research/quartic_dl1_cd_sqrt_20261113/combo/analyze.py; the producer's analyze.py was not imported or read).

usage: recompute.py PROD_RUNS_DIR FASTHASH_RUNS_DIR SUMMARY_JSON BAND_VALIDATION_JSON CLAIMED_ANALYSIS_JSON > recompute.json
"""
import hashlib, json, math, os, sys

prod, fh, summ, bandf, claimedf = sys.argv[1:6]
QS = [262111, 524287, 1048507, 2097091, 4194271]
SEEDS = [1, 2, 3, 4, 5, 6]
PH = ["fb", "relations", "matrix", "linear_algebra", "log_recovery", "descent", "verify"]
WADD = [0.0, 0.5, 1.0]
WH = [0.0, 0.1, 1.0]
ARMS = ["D", "DL1", "DL"]

def counted_keys(r):
    return [k for k in r if not (k.endswith("wall") or k.endswith("cpu")) and k != "load1" and not k.startswith("s.controls")]

def load(path):
    return [json.loads(l) for l in open(path) if l.strip()]

# 0. tarball inner checksums
bad = []
for line in open(os.path.join(prod, "SHA256SUMS")):
    h, f = line.split(None, 1); f = f.strip()
    if hashlib.sha256(open(os.path.join(prod, f), "rb").read()).hexdigest() != h:
        bad.append(f)
out = {"inner_sha256sums": {"files": sum(1 for _ in open(os.path.join(prod, "SHA256SUMS"))), "bad": bad}}

C = {}
for line in open(summ):
    r = json.loads(line)
    if r.get("kind") == "ic_size" and r.get("arm") == "DL" and r["q"] in QS:
        C[r["q"]] = (r["r_inv"], r["r_lmul"], r["gae"])
out["frozen_constants"] = {str(q): {"r_inv": C[q][0], "r_lmul": C[q][1], "gae": C[q][2]} for q in QS}

def meq(rs, wadd, wh):
    ri, rl, gae = C[rs[0]["q"]]
    return sum(r[p + ".mul"] + ri * r[p + ".inv"] + rl * r[p + ".lmul"] + wadd * (r[p + ".add"] + r[p + ".ladd"])
               + wh * gae * (r[p + ".hash"] + r[p + ".cmp"]) for r in rs for p in PH)

U = {}
for arm in ("REF", "BLR"):
    for q in QS:
        for s in SEEDS:
            U[(arm, q, s)] = load(os.path.join(prod, "units", f"{arm}_{q}_{s}.jsonl"))
ic = lambda arm, q, s, y=None: [r for r in U[(arm, q, s)] if r.get("rec") == "ic" and (y is None or r["arm"] == y)]

# G1/G2/G3
g1, g2, g3 = {"pairs": 0, "records": 0, "fields": 0, "mismatches": [], "la_diff_cells": 0}, {"mismatches": []}, {"failures": []}
for q in QS:
    for s in SEEDS:
        a, b = ic("REF", q, s), ic("BLR", q, s)
        g1["pairs"] += 1
        if len(a) != len(b) or [x["arm"] for x in a] != [x["arm"] for x in b]:
            g1["mismatches"].append(f"{q}_{s}: record count/order differs"); continue
        ladiff = False
        for x, z in zip(a, b):
            g1["records"] += 1
            kx, kz = counted_keys(x), counted_keys(z)
            if kx != kz:
                g1["mismatches"].append(f"{q}_{s}/{x['arm']}: key sets/order differ"); continue
            for k in kx:
                g1["fields"] += 1
                if x[k] != z[k]:
                    if x["arm"] == "DL" and k in ("linear_algebra.lmul", "linear_algebra.ladd"):
                        ladiff = True
                    else:
                        g1["mismatches"].append(f"{q}_{s}/{x['arm']}/{k}")
            if x["arm"] == "DL":
                for k in ("s.bl_iters", "s.bl_narrowed"):
                    if x.get(k) != z.get(k) or k not in x:
                        g2["mismatches"].append(f"{q}_{s}/{k}")
        g1["la_diff_cells"] += ladiff
        for arm in ("REF", "BLR"):
            for r in ic(arm, q, s):
                if not (r["ok"] is True and r["k_found"] == r["k"]):
                    g3["failures"].append(f"{arm} {q}_{s}/{r['arm']}")
            units = [r for r in U[(arm, q, s)] if r.get("rec") == "unit"]
            if len(units) != 1 or units[0]["rc"] != 0:
                g3["failures"].append(f"{arm} {q}_{s} unit rc")
            for y in ARMS:
                if len(ic(arm, q, s, y)) != 1:
                    g3.setdefault("attempts_not_1", []).append(f"{arm} {q}_{s}/{y}")
g1["pass"] = not g1["mismatches"] and g1["la_diff_cells"] == 30
g2["pass"] = not g2["mismatches"]; g3["pass"] = not g3["failures"]
# G4: REF vs committed fasthash COMBO (p1, p2) seeds 1-3
g4 = {"files": 0, "records": 0, "mismatches": []}
for q in QS:
    for s in (1, 2, 3):
        a = ic("REF", q, s)
        for p in ("p1", "p2"):
            b = [r for r in load(os.path.join(fh, "units", f"{p}_COMBO_{q}_{s}.jsonl")) if r.get("rec") == "ic"]
            g4["files"] += 1
            if len(a) != len(b):
                g4["mismatches"].append(f"{p} {q}_{s}: count"); continue
            for x, z in zip(a, b):
                g4["records"] += 1
                kx, kz = counted_keys(x), counted_keys(z)
                if kx != kz or any(x[k] != z[k] for k in kx):
                    g4["mismatches"].append(f"{p} {q}_{s}/{x['arm']}")
g4["pass"] = not g4["mismatches"]
out["gates"] = {"G1": g1, "G2": g2, "G3": g3, "G4": g4}

# ratios
gm = lambda v: math.exp(sum(map(math.log, v)) / len(v))
ratios = {}
for y in ARMS:
    for q in QS:
        for wa in WADD:
            for wh in WH:
                per = [meq(ic("BLR", q, s, y), wa, wh) / meq(ic("REF", q, s, y), wa, wh) for s in SEEDS]
                ratios[f"{y}/{q}/{wa}/{wh}"] = {"per_seed": per, "geomean": gm(per), "max": max(per), "min": min(per)}
out["ratios"] = ratios
la = {}
for q in QS:
    per = [ic("BLR", q, s, "DL")[0]["linear_algebra.lmul"] / ic("REF", q, s, "DL")[0]["linear_algebra.lmul"] for s in SEEDS]
    la[str(q)] = {"per_seed": per, "geomean": gm(per)}
out["la_lmul_ratio"] = la
prim = [ratios[f"DL/{q}/{wa}/0.0"]["max"] for q in QS for wa in WADD]
allwh = [ratios[f"DL/{q}/{wa}/{wh}"]["max"] for q in QS for wa in WADD for wh in WH]
out["decision"] = {"cells": 30, "max_DL_ratio_primary": max(prim), "SUPPORTED": max(prim) < 1 and all(out["gates"][g]["pass"] for g in ("G1", "G2", "G3", "G4")),
                   "holds_all_wh": max(allwh) < 1,
                   "D_DL1_exactly_1": all(ratios[f"{y}/{q}/{wa}/{wh}"]["max"] == 1.0 and ratios[f"{y}/{q}/{wa}/{wh}"]["min"] == 1.0
                                           for y in ("D", "DL1") for q in QS for wa in WADD for wh in WH)}
# k = 22 band
band = json.load(open(bandf))["rho_reference"]["gae_band"]
q = 4194271
gap = {}
for wkey, w in (("0", 0.0), ("0.1", 0.1), ("1", 1.0)):
    ref = sum(meq(ic("REF", q, s, "DL"), 0.0, w) for s in (1, 2, 3)) / 3
    blr = sum(meq(ic("BLR", q, s, "DL"), 0.0, w) for s in (1, 2, 3)) / 3
    rho = band[wkey]["rho"]
    gap[wkey] = {"rho": rho, "ic_frozen": band[wkey]["ic"], "ic_ref": ref, "ic_blr": blr,
                 "ref_rel_err_vs_frozen": abs(ref - band[wkey]["ic"]) / band[wkey]["ic"],
                 "rho_over_ic_ref": rho / ref, "rho_over_ic_blr": rho / blr, "ic_over_rho_blr": blr / rho, "ic_over_rho_ref": ref / rho}
out["gap_k22"] = gap
out["ref_reproduces_band_1e9"] = all(g["ref_rel_err_vs_frozen"] < 1e-9 for g in gap.values())

# comparison with the producer's analysis.json (claimed values, read by key)
cl = json.load(open(claimedf))
cmp = {"ratio_cells_compared": 0, "ratio_max_rel_diff": 0.0, "la_max_rel_diff": 0.0, "notes": []}
for k, v in ratios.items():
    if k in cl.get("ratios", {}):
        c = cl["ratios"][k]
        for f in ("geomean", "max", "min"):
            cmp["ratio_cells_compared"] += 1
            cmp["ratio_max_rel_diff"] = max(cmp["ratio_max_rel_diff"], abs(c[f] - v[f]) / abs(v[f]))
        if "per_seed" in c:
            for a_, b_ in zip(c["per_seed"], v["per_seed"]):
                cmp["ratio_max_rel_diff"] = max(cmp["ratio_max_rel_diff"], abs(a_ - b_) / abs(b_))
    else:
        cmp["notes"].append(f"missing in claimed: {k}")
for q2 in QS:
    c = cl.get("la_lmul_ratio", {}).get(str(q2))
    if c:
        cmp["la_max_rel_diff"] = max(cmp["la_max_rel_diff"], abs(c["geomean"] - la[str(q2)]["geomean"]) / la[str(q2)]["geomean"])
out["vs_claimed"] = cmp
json.dump(out, sys.stdout, indent=1)
print()
