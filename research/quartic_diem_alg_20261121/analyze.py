#!/usr/bin/env python3
"""Analysis of quartic_alg.jsonl, as fixed in PROTOCOL.md section 4. Writes analysis.json.

FPM_mid = cantor_mul + 10 cantor_inv + rel_mul + 9 la_mul (la_mul includes la_sge).
Ratios: ratio of per-q means over the runs, percentile bootstrap (2000 resamples over runs,
seed 20261121). Exponents: least-squares slope of ln mean on ln q, same bootstrap.
"""
import json, math, random, sys
from collections import defaultdict

RUNS = sys.argv[1] if len(sys.argv) > 1 else "quartic_alg.jsonl"
BOUNDR = sys.argv[2] if len(sys.argv) > 2 else None
rows = [json.loads(l) for l in open(RUNS)]
runs = [r for r in rows if r.get("kind") == "run" and r.get("family") == "quartic"]
inst = {(r["p"], r["curve_seed"]): r for r in rows if r.get("kind") == "instance" and r.get("family") == "quartic"}
SCAN, ALG, ALGS, RHO, FULL = "Q_diem_2lp", "Q_diem_2lp+alg", "Q_diem_2lp+alg+sge(8,3)", "Q_rho_neg", "Q_diem_full"
key = lambda r: (r["p"], r["curve_seed"], r["seed"])
by = defaultdict(dict)
for r in runs:
    by[r["arm"]][key(r)] = r

def _fill_rho():
    """Where this run skipped rho (cap), take the bound-r row of the same instance and seed
    (AMENDMENT1.md); instance identity is checked below."""
    if not BOUNDR:
        return []
    filled = []
    for l in open(BOUNDR):
        o = json.loads(l)
        if o.get("kind") == "run" and o.get("family") == "quartic" and o["arm"] == "Q_rho_neg":
            if (o["p"], o["curve_seed"]) in inst and key(o) not in by["Q_rho_neg"]:
                by["Q_rho_neg"][key(o)] = o
                filled.append(list(key(o)))
    return filled

def fpm(r):
    return r["cantor_mul"] + 10 * r["cantor_inv"] + r.get("rel_mul", 0) + 9 * r.get("la_mul", 0)

def mean(v):
    return sum(v) / len(v)

rng = random.Random(20261121)
B = 2000
primes = sorted({r["p"] for r in runs})

def ratio_ci(num, den, ps):
    ks = sorted(k for k in num if k[0] == ps and k in den)
    a = [num[k] for k in ks]; b = [den[k] for k in ks]
    est = mean(a) / mean(b)
    bs = []
    for _ in range(B):
        idx = [rng.randrange(len(ks)) for _ in ks]
        bs.append(mean([a[i] for i in idx]) / mean([b[i] for i in idx]))
    bs.sort()
    return {"p": ps, "n": len(ks), "ratio": est, "lo": bs[int(0.025 * B)], "hi": bs[int(0.975 * B) - 1]}

def val(arm, f):
    return {k: f(r) for k, r in by[arm].items()}

def slope(xs, ys):
    mx, my = mean(xs), mean(ys)
    return sum((x - mx) * (y - my) for x, y in zip(xs, ys)) / sum((x - mx) ** 2 for x in xs)

def exponent(arm, f):
    d = val(arm, f)
    per = {p: [v for k, v in d.items() if k[0] == p] for p in primes}
    xs = [math.log(p) for p in primes]
    est = slope(xs, [math.log(mean(per[p])) for p in primes])
    bs = []
    for _ in range(B):
        ys = []
        for p in primes:
            s = [per[p][rng.randrange(len(per[p]))] for _ in per[p]]
            ys.append(math.log(mean(s)))
        bs.append(slope(xs, ys))
    bs.sort()
    return {"arm": arm, "exponent": est, "lo": bs[int(0.025 * B)], "hi": bs[int(0.975 * B) - 1],
            "means": {p: mean(per[p]) for p in primes}}

RHO_FILLED = _fill_rho()
A = {"rho_rows_from_boundr": RHO_FILLED, "runs_total": len(runs), "primes": primes,
     "not_certified": [r for r in runs if r["arm"] != RHO and not (r.get("certified") and r.get("ok"))],
     "rho_not_certified": [r for r in runs if r["arm"] == RHO and r["kind"] == "run" and not (r.get("certified") and r.get("ok"))],
     "arms": sorted(by), "cells": []}
for arm in sorted(by):
    for p in primes:
        v = [r for k, r in by[arm].items() if k[0] == p]
        if not v:
            continue
        c = {"arm": arm, "p": p, "runs": len(v), "fpm_mid": mean([fpm(r) for r in v])}
        for f in ("cantor_mul", "cantor_inv", "rel_mul", "la_mul", "rel_enum_mul", "rel_decomp_mul",
                  "la_sge_mul", "decomp_trials", "decomp_fallbacks", "relations", "core_rows", "core_cols",
                  "attempts", "group_ops"):
            xs = [r[f] for r in v if f in r]
            if xs:
                c["mean_" + f] = mean(xs)
        A["cells"].append(c)

# Q1: identity of the relation phase
# (a) Scan vs Algebraic: every counted field except the decomposition share of rel_mul must agree.
# (b) with the merge: the relation phase (everything before the solve) must agree; group_ops,
#     cantor_mul and cantor_inv also contain the certificate multiplications [t']G of the
#     candidates the solves return, which the merge may change (reported, not required equal).
rel_fields = ["relations", "rels_by_lp", "decomp_trials", "trials", "core_rows", "core_cols", "attempts",
              "fb_cols", "certified", "ok"]
cert_fields = ["group_ops", "cantor_mul", "cantor_inv"]
q1 = {"diffs": [], "cert_phase_diffs": [], "scan_vs": {}}
for other in (ALG, ALGS):
    n = ok = okcert = 0
    for k, s in by[SCAN].items():
        o = by[other].get(k)
        if o is None:
            continue
        n += 1
        d = [f for f in rel_fields if s.get(f) != o.get(f)]
        so = s["rel_mul"] - s.get("rel_enum_mul", 0) - s.get("rel_decomp_mul", 0)
        oo = o["rel_mul"] - o["rel_enum_mul"] - o["rel_decomp_mul"]
        if so != oo:
            d.append("rel_other_mul")
        if s.get("rel_enum_mul") != o["rel_enum_mul"]:
            d.append("rel_enum_mul")
        if other == ALG and s["la_mul"] != o["la_mul"]:
            d.append("la_mul")
        c = [f for f in cert_fields if s.get(f) != o.get(f)]
        if d:
            q1["diffs"].append({"arm": other, "key": list(k), "fields": d})
        else:
            ok += 1
        if not c:
            okcert += 1
        else:
            q1["cert_phase_diffs"].append({"arm": other, "key": list(k), "group_ops": [s["group_ops"], o["group_ops"]],
                                           "cantor_mul": [s["cantor_mul"], o["cantor_mul"]]})
    q1["scan_vs"][other] = {"pairs": n, "relation_phase_identical": ok, "group_ops_and_cantor_identical": okcert}
A["Q1"] = q1
# FPM composition per arm and q (means)
comp = []
for arm in (SCAN, ALG, ALGS, RHO):
    for p in primes:
        v = [r for k, r in by[arm].items() if k[0] == p]
        c = {"arm": arm, "p": p,
             "pic": mean([r["cantor_mul"] + 10 * r["cantor_inv"] for r in v]),
             "rel": mean([r.get("rel_mul", 0) for r in v]),
             "la9": mean([9 * r.get("la_mul", 0) for r in v])}
        if arm != RHO:
            c["rel_enum"] = mean([r["rel_enum_mul"] for r in v if "rel_enum_mul" in r]) if any("rel_enum_mul" in r for r in v) else None
            c["rel_decomp"] = mean([r["rel_decomp_mul"] for r in v if "rel_decomp_mul" in r]) if any("rel_decomp_mul" in r for r in v) else None
        comp.append(c)
A["composition"] = comp
A["Q2_rel_mul_alg_over_scan"] = [ratio_ci(val(ALG, lambda r: r["rel_mul"]), val(SCAN, lambda r: r["rel_mul"]), p) for p in primes]
A["Q2_rel_decomp_alg_over_scan"] = [ratio_ci(val(ALG, lambda r: r["rel_decomp_mul"]), val(SCAN, lambda r: r["rel_decomp_mul"]), p) for p in primes]
A["Q2_rel_mul_algsge_over_scan"] = [ratio_ci(val(ALGS, lambda r: r["rel_mul"]), val(SCAN, lambda r: r["rel_mul"]), p) for p in primes]
A["Q3_fpm_algsge_over_scan"] = [ratio_ci(val(ALGS, fpm), val(SCAN, fpm), p) for p in primes]
A["fpm_alg_over_scan"] = [ratio_ci(val(ALG, fpm), val(SCAN, fpm), p) for p in primes]
A["la_mul_algsge_over_alg"] = [ratio_ci(val(ALGS, lambda r: r["la_mul"]), val(ALG, lambda r: r["la_mul"]), p) for p in primes]
A["Q5_fpm_algsge_over_rho"] = [ratio_ci(val(ALGS, fpm), val(RHO, fpm), p) for p in primes if by[RHO]]
A["fpm_scan_over_rho"] = [ratio_ci(val(SCAN, fpm), val(RHO, fpm), p) for p in primes if by[RHO]]
A["fpm_alg_over_rho"] = [ratio_ci(val(ALG, fpm), val(RHO, fpm), p) for p in primes if by[RHO]]
A["Q4_exponents"] = [exponent(a, fpm) for a in (SCAN, ALG, ALGS)] + ([exponent(RHO, fpm)] if by[RHO] else [])
A["rho_skipped"] = [r for r in rows if r.get("kind") == "skip"]

# cross-check against the bound-r study (default arms)
if BOUNDR:
    old = [json.loads(l) for l in open(BOUNDR)]
    oldinst = {(r["p"], r["curve_seed"]): r for r in old if r.get("kind") == "instance" and r.get("family") == "quartic"}
    same_inst = all(k in oldinst and all(oldinst[k][f] == inst[k][f] for f in ("f", "order", "r", "h", "n_points")) for k in inst)
    oldruns = {(r["arm"],) + key(r): r for r in old if r.get("kind") == "run" and r.get("family") == "quartic"}
    cmpf = ["certified", "ok", "group_ops", "cantor_mul", "cantor_inv", "rel_mul", "rel_add", "la_mul", "la_add", "fb_cols",
            "trials", "relations", "rels_by_lp", "decomp_trials", "core_rows", "core_cols", "attempts", "steps"]
    n = same = 0
    bad = []
    for arm in (FULL, SCAN, RHO):
        for k, r in by[arm].items():
            o = oldruns.get((arm,) + k)
            if o is None:
                continue
            n += 1
            d = [f for f in cmpf if f in r or f in o if r.get(f) != o.get(f)]
            if d:
                bad.append({"arm": arm, "key": list(k), "fields": d})
            else:
                same += 1
    A["boundr_crosscheck"] = {"instances_identical": same_inst, "instances": len(inst), "rows_compared": n,
                              "rows_identical": same, "differences": bad}
json.dump(A, open("analysis.json", "w"), indent=1, default=str)
print(json.dumps({k: A[k] for k in A if k.startswith("Q") or k.startswith("boundr")}, indent=1, default=str)[:6000])
