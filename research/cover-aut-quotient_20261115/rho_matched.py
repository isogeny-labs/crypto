#!/usr/bin/env python3
"""Per-instance matched negation-map rho versus arm A (== arm Q), same E(F_p) instances.
Rho runs are the rescore's 12 'new' rhostrong runs per instance (same l and c_add_e as our paired records, checked).
Cover side is deterministic per instance (one run), so the interval is a bootstrap over the 12 rho runs only,
conditional on the instance; S/rho per instance, then geo-mean over instances. [pcc:unverified]"""
import json, glob, math, os, random
D = os.path.dirname(os.path.abspath(__file__)); R = D + "/results"
RS = D + "/../jv_cover_negrho_rescore_20261101/results/new"
NB = 20000
def geo(x): return math.exp(sum(map(math.log, x)) / len(x))
out = {"unit": "[pcc:unverified] local development check", "bootstrap": NB, "cells": [], "by_prime": {}, "note":
       "rho refs are per-instance means of 12 runs; interval = bootstrap of the 12 rho runs (cover run is one deterministic run)"}
rng = random.Random(20261115)
for p in (1301, 1451, 1601):
    per = []
    for s in (1, 2, 3):
        f = f"{R}/paired_p{p}_s{s}.json"
        if not os.path.exists(f): continue
        c = json.load(open(f)); a = c["arm_A"]
        runs = []
        for g in sorted(glob.glob(f"{RS}/rhostrong_p{p}_s{s}_run*.json")):
            r = json.load(open(g))[0]
            assert r["l"] == c["l"] and r["c_add_e"] == c["c_add_e"] and r["correct"] and r["negation"]
            sl = math.sqrt(r["l"]); o = r["ops"]
            nat = o["canon_cmp"] + o["hash"] + o["dp_check"] + o["scalar_ops"] + o["table_ops"] + o["cycle_cmp"]
            runs.append((r["steps"] / sl, nat / sl))
        n = len(runs); ref0 = sum(x for x, _ in runs) / n; refn = sum(y for _, y in runs) / n
        cell = {"p": p, "seed": s, "n_rho_runs": n, "rho_steps_per_sqrt_l": ref0, "rho_native_per_sqrt_l": refn,
                "S_cover": a["S"], "cover_native_per_sqrt_l": a["native_per_sqrt_l"]}
        for w in (0, 0.1, 1):
            num = a["S"] + w * a["native_per_sqrt_l"]
            pt = num / (ref0 + w * refn)
            bs = sorted(num / (sum(x for x, _ in b) / n + w * sum(y for _, y in b) / n)
                        for b in ([runs[rng.randrange(n)] for _ in range(n)] for _ in range(NB)))
            cell[f"S_over_rho_w{w}"] = pt; cell[f"ci_w{w}"] = [bs[int(.025 * NB)], bs[int(.975 * NB)]]
        # A/A: split rho runs odd/even
        h1 = runs[0::2]; h2 = runs[1::2]
        cell["rho_AA_half_ratio"] = (sum(x for x, _ in h1) / len(h1)) / (sum(x for x, _ in h2) / len(h2))
        out["cells"].append(cell); per.append(cell)
    if per:
        d = {"n_instances": len(per)}
        for w in (0, 0.1, 1):
            v = [c[f"S_over_rho_w{w}"] for c in per]
            d[f"geo_w{w}"] = geo(v); d[f"min_w{w}"] = min(v); d[f"max_w{w}"] = max(v)
            d[f"ci_lo_w{w}"] = geo([c[f"ci_w{w}"][0] for c in per]); d[f"ci_hi_w{w}"] = geo([c[f"ci_w{w}"][1] for c in per])
        out["by_prime"][p] = d
# pooled-reference variant: all 126 'new' rho runs (3 primes x 3 seeds x 14? whatever exists), bootstrap over rho runs AND instances
pool = []
for g in sorted(glob.glob(f"{RS}/rhostrong_p*_s*_run*.json")):
    r = json.load(open(g))[0]; sl = math.sqrt(r["l"]); o = r["ops"]
    pool.append((r["steps"] / sl, (o["canon_cmp"] + o["hash"] + o["dp_check"] + o["scalar_ops"] + o["table_ops"] + o["cycle_cmp"]) / sl))
out["pooled_rho_n"] = len(pool)
out["pooled_rho_steps_per_sqrt_l"] = sum(x for x, _ in pool) / len(pool)
out["pooled_rho_native_per_sqrt_l"] = sum(y for _, y in pool) / len(pool)
out["by_prime_pooled_ref"] = {}
for p in (1301, 1451, 1601):
    cells = [c for c in out["cells"] if c["p"] == p]
    if not cells: continue
    d = {"n_instances": len(cells)}
    for w in (0, 0.1, 1):
        nums = [c["S_cover"] + w * c["cover_native_per_sqrt_l"] for c in cells]
        refw = lambda b: sum(x + w * y for x, y in b) / len(b)
        pt = geo(nums) / refw(pool)
        bs = []
        for _ in range(NB):
            b = [pool[rng.randrange(len(pool))] for _ in range(len(pool))]
            ns = [nums[rng.randrange(len(nums))] for _ in nums]
            bs.append(geo(ns) / refw(b))
        bs.sort(); d[f"geo_w{w}"] = pt; d[f"ci_w{w}"] = [bs[int(.025 * NB)], bs[int(.975 * NB)]]
    out["by_prime_pooled_ref"][p] = d
json.dump(out, open(D + "/rho_matched.json", "w"), indent=1)
for c in out["cells"]:
    print(c["p"], c["seed"], "rho %.4f nat %.2f" % (c["rho_steps_per_sqrt_l"], c["rho_native_per_sqrt_l"]),
          "S/rho w0 %.3f [%.3f,%.3f]" % (c["S_over_rho_w0"], *c["ci_w0"]), "w1 %.3f [%.3f,%.3f]" % (c["S_over_rho_w1"], *c["ci_w1"]), "AA %.3f" % c["rho_AA_half_ratio"])
for p, d in out["by_prime_pooled_ref"].items(): print("pooled-ref", p, d)
print("pooled n", out["pooled_rho_n"], out["pooled_rho_steps_per_sqrt_l"], out["pooled_rho_native_per_sqrt_l"])
