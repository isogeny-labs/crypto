#!/usr/bin/env python3
"""Analysis of genus_ladder_bounded_core_20261122 (frozen before data). Reads run_P.jsonl, writes analysis.json.
Every figure [pcc:unverified]. Usage: analyze.py [dir] ; env BD_PRIMES overrides the prime list (pilot checks only)."""
import json, math, os, random, sys, collections
D = sys.argv[1] if len(sys.argv) > 1 else "."
PS = [int(x) for x in os.environ.get("BD_PRIMES", "32771,65537,131101,262147").split(",")]
UNM = "C_double_lp+pre"; PLAIN = UNM + "+sge(8,3)"
ARMS = {"unmerged": UNM, "plain": PLAIN, "ex0.3": PLAIN + "+ex(0.3,0.01)",
        "bd2": PLAIN + "+bd(8,0.1,3,2)", "bd1": PLAIN + "+bd(8,0.1,3,1)"}
KAPPA = 3
def load(fn):
    out = []
    for l in open(fn):
        l = l.strip()
        if l: out.append(json.loads(l))
    return out
runs = collections.defaultdict(list); inst = {}
for p in PS:
    for r in load(os.path.join(D, "run_%d.jsonl" % p)):
        if r.get("kind") == "run": runs[(p, r["arm"])].append(r)
        elif r.get("kind") == "instance": inst[(p, r["curve_seed"])] = r
def fpm(r, k=KAPPA): return r["cantor_mul"] + 10 * r["cantor_inv"] + r["rel_mul"] + k * r["la_mul"]
def relph(r): return r["cantor_mul"] + 10 * r["cantor_inv"] + r["rel_mul"]
def w_sge_out(r): return r["sge_nnz_out"] / r["sge_rows_out"]
def w_sge_in(r): return r["sge_nnz_in"] / r["sge_rows_in"]
METRICS = {"la_mul": lambda r: r["la_mul"], "la_add": lambda r: r["la_add"], "rel_mul": lambda r: r["rel_mul"],
           "relphase": relph, "fpm1": lambda r: fpm(r, 1), "fpm3": fpm, "fpm9": lambda r: fpm(r, 9),
           "relations": lambda r: r["relations"], "wall_s": lambda r: r["wall_ns"] / 1e9,
           "rows_out": lambda r: r.get("sge_rows_out", 0), "cols_out": lambda r: r.get("sge_cols_out", 0),
           "nnz_out": lambda r: r.get("sge_nnz_out", 0), "nnz_in": lambda r: r.get("sge_nnz_in", 0),
           "w_out": lambda r: w_sge_out(r) if "sge_nnz_out" in r else 0.0,
           "w_in": lambda r: w_sge_in(r) if "sge_nnz_in" in r else 0.0,
           "nnz_per_col_out": lambda r: r["sge_nnz_out"] / r["sge_cols_out"] if "sge_nnz_out" in r else 0.0,
           "core_rows": lambda r: r["core_rows"], "core_cols": lambda r: r["core_cols"],
           "rows_pre": lambda r: r.get("core_rows_pre", r["core_rows"]), "cols_pre": lambda r: r.get("core_cols_pre", r["core_cols"]),
           "w_pre": lambda r: r.get("mean_weight_before", 0.0), "w_sel": lambda r: r.get("mean_weight_after", 0.0),
           "nnz_sel": lambda r: r.get("core_nnz_before_sge", 0), "la_sel_add": lambda r: r.get("la_sel_add", 0),
           "bd_probes": lambda r: r.get("bd_probes", 0), "bd_extra": lambda r: r.get("bd_extra", 0.0),
           "bd_capped": lambda r: float(r.get("bd_capped", False)), "attempts": lambda r: r["attempts"],
           "certified": lambda r: float(r["certified"] and r["ok"])}
def cell_mean(rs, f):
    byc = collections.defaultdict(list)
    for r in rs: byc[r["curve_seed"]].append(f(r))
    return sum(sum(v) / len(v) for v in byc.values()) / len(byc)
def boot_cells(rs, rng):
    byc = collections.defaultdict(list)
    for r in rs: byc[r["curve_seed"]].append(r)
    keys = sorted(byc); out = []
    for _ in keys:
        c = byc[rng.choice(keys)]
        out.extend(rng.choice(c) for _ in c)
    return out
def fit(xs, ys):
    n = len(xs); mx = sum(xs) / n; my = sum(ys) / n
    return sum((x - mx) * (y - my) for x, y in zip(xs, ys)) / sum((x - mx) ** 2 for x in xs)
def pct(v, q):
    v = sorted(v); i = q * (len(v) - 1); lo = int(math.floor(i)); hi = min(lo + 1, len(v) - 1)
    return v[lo] + (v[hi] - v[lo]) * (i - lo)
A = {"primes": PS, "kappa": KAPPA, "cells": {}, "n_runs": {}, "all_certified": True}
for p in PS:
    for nm, arm in ARMS.items():
        rs = runs.get((p, arm), [])
        A["n_runs"]["%d|%s" % (p, nm)] = len(rs)
        if not rs: continue
        A["all_certified"] &= all(r["certified"] and r["ok"] for r in rs)
        A["cells"]["%d|%s" % (p, nm)] = {m: cell_mean(rs, f) for m, f in METRICS.items()}
# bootstrap: exponents and paired ratios
rng = random.Random(20261122); NB = 4000
logp = [math.log(p) for p in PS]
names = [n for n in ARMS if all(runs.get((p, ARMS[n])) for p in PS)]
EXP_M = ["la_mul", "fpm3", "relphase", "fpm1", "fpm9"]
point = {n: {m: fit(logp, [math.log(A["cells"]["%d|%s" % (p, n)][m]) for p in PS]) for m in EXP_M} for n in names}
draws = {n: {m: [] for m in EXP_M} for n in names}; rat = collections.defaultdict(list); diffs = collections.defaultdict(list)
for _ in range(NB):
    # same curve resample for every arm: draw once per prime from the plain arm's curve list
    bs = {}
    for p in PS:
        keys = sorted({r["curve_seed"] for r in runs[(p, PLAIN)]})
        pick = [rng.choice(keys) for _ in keys]
        sub = {}
        for n in names:
            byc = collections.defaultdict(list)
            for r in runs[(p, ARMS[n])]: byc[r["curve_seed"]].append(r)
            sub[n] = [(c, byc[c]) for c in pick]
        seedpick = {i: [rng.random() for _ in range(8)] for i in range(len(pick))}  # shared uniform draws for seeds
        bs[p] = (sub, seedpick)
    cm = {}
    for p in PS:
        sub, sp = bs[p]
        for n in names:
            vals = {m: 0.0 for m in EXP_M + ["w_out"]}
            for i, (c, rs) in enumerate(sub[n]):
                chosen = [rs[int(u * len(rs))] for u in sp[i][:len(rs)]]
                for m in vals: vals[m] += sum(METRICS[m](r) for r in chosen) / len(chosen)
            cm[(p, n)] = {m: v / len(sub[n]) for m, v in vals.items()}
    ex = {}
    for n in names:
        for m in EXP_M:
            e = fit(logp, [math.log(cm[(p, n)][m]) for p in PS]); draws[n][m].append(e); ex[(n, m)] = e
        for p in PS:
            for m in ("fpm3", "la_mul", "relphase", "w_out"):
                rat[(p, n, m)].append(cm[(p, n)][m] / cm[(p, "plain")][m])
    for n in names:
        if n != "plain":
            diffs[(n, "fpm3")].append(ex[(n, "fpm3")] - ex[("plain", "fpm3")])
            diffs[(n, "la_mul")].append(ex[(n, "la_mul")] - ex[("plain", "la_mul")])
    for n in names:
        if n == "unmerged": continue
        wv = [cm[(p, n)]["w_out"] for p in PS]
        diffs[(n, "w_spread")].append(max(wv) / min(wv))
iv = lambda v: {"lo": pct(v, 0.025), "hi": pct(v, 0.975)}
A["exponents"] = {n: {m: dict(est=point[n][m], **iv(draws[n][m])) for m in EXP_M} for n in names}
A["exp_diff_vs_plain"] = {"%s|%s" % k: dict(est=point[k[0]][k[1]] - point["plain"][k[1]], **iv(v)) for k, v in diffs.items() if k[1] in ("fpm3", "la_mul")}
A["w_spread"] = {n: dict(est=max(A["cells"]["%d|%s" % (p, n)]["w_out"] for p in PS) / min(A["cells"]["%d|%s" % (p, n)]["w_out"] for p in PS), **iv(diffs[(n, "w_spread")])) for n in names if n != "unmerged"}
A["ratio_to_plain"] = {"%d|%s|%s" % k: dict(est=A["cells"]["%d|%s" % (k[0], k[1])][k[2]] / A["cells"]["%d|plain" % k[0]][k[2]], **iv(v)) for k, v in rat.items()}
# B1: identity of the plain rows with the earlier studies
EARLIER = {32771: "../genus_ladder_sge_scale_20261115/sge_g3.jsonl", 65537: "../genus_ladder_g3_highp_20261121/run_65537.jsonl",
           131101: "../genus_ladder_g3_highp_20261121/run_131101.jsonl", 262147: "../genus_ladder_g3_highp_20261121/run_262147.jsonl"}
SKIP = {"wall_ns", "order_search_wall_s"}
b1 = {"compared": 0, "mismatch": [], "instances_equal": True}
if os.environ.get("BD_SKIP_B1") is None:
    for p in PS:
        old = collections.defaultdict(dict); oinst = {}
        for r in load(os.path.join(D, EARLIER[p])):
            if r.get("kind") == "run" and r["p"] == p: old[(r["curve_seed"], r["seed"], r["arm"])] = r
            elif r.get("kind") == "instance" and r["p"] == p: oinst[r["curve_seed"]] = r
        for (pp, cs), r in inst.items():
            if pp == p and cs in oinst and any(r.get(k) != oinst[cs].get(k) for k in oinst[cs] if k not in SKIP): b1["instances_equal"] = False
        for arm in (UNM, PLAIN):
            for r in runs.get((p, arm), []):
                o = old.get((r["curve_seed"], r["seed"], arm))
                if o is None: b1["mismatch"].append([p, r["curve_seed"], r["seed"], arm, "missing earlier row"]); continue
                b1["compared"] += 1
                for k in o:
                    if k in SKIP: continue
                    if r.get(k) != o[k]: b1["mismatch"].append([p, r["curve_seed"], r["seed"], arm, k, o[k], r.get(k)])
A["B1"] = b1
# scorecard
sc = {}
for n in ("bd2", "bd1"):
    if n not in names: continue
    e = A["exponents"][n]["la_mul"]; pe = A["exponents"]["plain"]
    c262 = A["cells"]["262147|%s" % n] if 262147 in PS else None
    sc[n] = {"B2_w_spread_max_over_min": A["w_spread"][n]["est"], "B2_holds": A["w_spread"][n]["est"] <= 1.10,
             "B3_la_exp": e["est"], "B3_hi": e["hi"], "B3_holds": e["est"] <= 1.42 and e["hi"] < 1.47,
             "B4_fpm3_over_plain_262147": c262["fpm3"] / A["cells"]["262147|plain"]["fpm3"] if c262 else None,
             "B4_i_holds": bool(c262 and c262["fpm3"] <= A["cells"]["262147|plain"]["fpm3"]),
             "B4_ii_fpm3_exp": A["exponents"][n]["fpm3"]["est"], "B4_ii_plain_fpm3_exp": pe["fpm3"]["est"],
             "B4_ii_holds": A["exponents"][n]["fpm3"]["est"] < pe["fpm3"]["est"]}
sc["B1_holds"] = A["all_certified"] and not b1["mismatch"] and b1["instances_equal"] and b1["compared"] > 0
sc["all_certified"] = A["all_certified"]
A["scorecard"] = sc
json.dump(A, open(os.path.join(D, "analysis.json"), "w"), indent=1, sort_keys=True)
print(json.dumps(sc, indent=1))
