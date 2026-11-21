#!/usr/bin/env python3
"""Frozen analysis of PROTOCOL.md (quartic_diem_precheck2_20261121).

Usage: python3 analyze.py raw/run.jsonl [raw/run65537.jsonl ...] > analysis.json

Arms: Q_diem_2lp (OLD, the frozen 2026-11-20 instrument), Q_diem_2lp_alg
(algebraic descent, frozen solve), Q_diem_2lp_new (PRIMARY: algebraic descent
+ linked solve policy).  Every number is [pcc:unverified].
"""
import json
import math
import sys
from collections import defaultdict

import numpy as np

PRIMARY = "Q_diem_2lp_new"
OLD = "Q_diem_2lp"
ALG = "Q_diem_2lp_alg"
GT = "Q_diem_2lp_gt"
PREV = "../quartic_diem_precheck_20261120/raw/run.jsonl"
T975 = {1: 12.706, 2: 4.303, 3: 3.182, 4: 2.776, 5: 2.571, 6: 2.447, 7: 2.365, 8: 2.306}
B = 4000
SEED = 20261121
KEYS = ["la_mul", "la_mul_add", "fpm_mid", "fpm_low", "fpm_high",
        "gae_0", "gae_0.1", "gae_1", "rel_mul"]
PAIRED = ["fpm_mid", "la_mul", "rel_mul", "gae_1", "gae_0", "decomp_trials", "attempts"]
REPRO = ["group_ops", "cantor_mul", "cantor_inv", "rel_mul", "rel_add", "la_mul",
         "la_add", "relations", "decomp_trials", "attempts", "core_rows", "core_cols",
         "fb_cols", "certified", "ok"]


def metrics(r):
    cm, ci = r["cantor_mul"], r["cantor_inv"]
    rel, la, laa = r["rel_mul"], r["la_mul"], r["la_add"]
    go = r["group_ops"]
    return {
        "la_mul": la,
        "la_mul_add": la + laa,
        "fpm_mid": cm + 10 * ci + rel + 9 * la,
        "fpm_low": cm + ci + rel + la,
        "fpm_high": cm + 30 * ci + rel + 9 * la,
        "gae_0": go,
        "gae_0.1": go + 0.1 * (rel + la),
        "gae_1": go + (rel + la),
        "rel_mul": rel,
        "decomp_trials": max(r["decomp_trials"], 1),
        "attempts": max(r["attempts"], 1),
    }


def ols(x, y):
    x, y = np.asarray(x, float), np.asarray(y, float)
    xm = x.mean()
    b = ((x - xm) * (y - y.mean())).sum() / ((x - xm) ** 2).sum()
    return b, y.mean() - b * xm


def gm(v):
    return math.exp(float(np.mean([math.log(x) for x in v])))


def fit(per_p, primes, k, rng):
    xs, ys = [], []
    for p in primes:
        for m in per_p[p]:
            xs.append(math.log(p))
            ys.append(math.log(m[k]))
    b, _ = ols(xs, ys)
    boots = []
    for _ in range(B):
        bx, by = [], []
        for p in primes:
            v = per_p[p]
            for i in rng.integers(0, len(v), len(v)):
                bx.append(math.log(p))
                by.append(math.log(v[i][k]))
        boots.append(ols(bx, by)[0])
    lo, hi = np.percentile(boots, [2.5, 97.5])
    px = np.array([math.log(p) for p in primes])
    py = np.array([np.mean([math.log(m[k]) for m in per_p[p]]) for p in primes])
    bm, am = ols(px, py)
    res = py - (am + bm * px)
    df = len(primes) - 2
    se = math.sqrt((res ** 2).sum() / df / ((px - px.mean()) ** 2).sum()) if df > 0 else float("nan")
    t = T975.get(df, 2.0)
    local = [{"from": primes[i], "to": primes[i + 1],
              "slope": float((py[i + 1] - py[i]) / (px[i + 1] - px[i]))}
             for i in range(len(primes) - 1)]
    return {"ols_curve_level": float(b), "boot95": [float(lo), float(hi)],
            "means_ols": float(bm), "means_t95": [float(bm - t * se), float(bm + t * se)],
            "local_slopes": local}


def arm_block(rows, rng):
    good = [r for r in rows if r.get("certified") and r.get("ok")]
    bad = [r for r in rows if not (r.get("certified") and r.get("ok"))]
    cur = defaultdict(list)
    for r in good:
        cur[(r["p"], r["curve_seed"])].append(metrics(r))
    primes_all = sorted({r["p"] for r in rows})
    per_p = defaultdict(list)
    for (p, _cs), ms in sorted(cur.items()):
        per_p[p].append({k: gm([m[k] for m in ms]) for k in ms[0]})
    dropped = [p for p in primes_all if len(per_p.get(p, [])) < 2]
    primes = [p for p in primes_all if p not in dropped]
    A = {"primes": primes, "dropped_primes": dropped, "n_runs": len(rows),
         "n_uncertified": len(bad),
         "uncertified": [{"p": r["p"], "curve_seed": r["curve_seed"], "seed": r["seed"],
                          "exhausted": r.get("exhausted")} for r in bad],
         "curves_per_prime": {p: len(per_p[p]) for p in primes_all},
         "exponents": {}, "exponents_six_primes": {}, "per_prime": {}}
    for p in primes_all:
        rs = [r for r in good if r["p"] == p]
        if not rs:
            continue
        ms = [metrics(r) for r in rs]
        A["per_prime"][p] = {
            "runs": len(rs),
            "geo_mean": {k: gm([m[k] for m in ms]) for k in ms[0]},
            "la_share_fpm_mid_mean": float(np.mean([9 * m["la_mul"] / m["fpm_mid"] for m in ms])),
            "rel_share_fpm_mid_mean": float(np.mean([m["rel_mul"] / m["fpm_mid"] for m in ms])),
            "fb_cols_mean": float(np.mean([r["fb_cols"] for r in rs])),
            "core_cols_mean": float(np.mean([r["core_cols"] for r in rs])),
            "core_rows_mean": float(np.mean([r["core_rows"] for r in rs])),
            "relations_mean": float(np.mean([r["relations"] for r in rs])),
            "attempts_mean": float(np.mean([r["attempts"] for r in rs])),
            "attempts_max": int(max(r["attempts"] for r in rs)),
            "decomp_trials_mean": float(np.mean([r["decomp_trials"] for r in rs])),
            "la_skipped_total": int(sum(r.get("la_skipped", 0) for r in rs)),
            "decomp_fallbacks_total": int(sum(r.get("decomp_fallbacks", 0) for r in rs)),
            "group_ops_mean": float(np.mean([r["group_ops"] for r in rs])),
            "rel_decomp_mul_mean": float(np.mean([r.get("rel_decomp_mul", float("nan")) for r in rs])),
            "rel_enum_mul_mean": float(np.mean([r.get("rel_enum_mul", float("nan")) for r in rs])),
        }
    if len(primes) >= 3:
        for k in KEYS:
            A["exponents"][k] = fit(per_p, primes, k, rng)
        six = [p for p in primes if p <= 32771]
        if len(six) >= 3 and six != primes:
            for k in KEYS:
                A["exponents_six_primes"][k] = fit(per_p, six, k, rng)
    return A


def paired(runs, num, den, rng):
    """Per (curve, seed) ratio num/den; curve = geo mean over seeds."""
    idx = {(r["p"], r["curve_seed"], r["seed"]): r for r in runs if r["arm"] == den}
    cur = defaultdict(lambda: defaultdict(list))
    for r in runs:
        if r["arm"] != num:
            continue
        d = idx.get((r["p"], r["curve_seed"], r["seed"]))
        if d is None or not (r["certified"] and r["ok"] and d["certified"] and d["ok"]):
            continue
        mn, md = metrics(r), metrics(d)
        for k in PAIRED:
            cur[(r["p"], r["curve_seed"])][k].append(mn[k] / md[k])
    per_p = defaultdict(list)
    for (p, _cs), kv in sorted(cur.items()):
        per_p[p].append({k: gm(v) for k, v in kv.items()})
    out = {"per_prime": {}, "pooled": {}, "ratio_slope": {}}
    primes = sorted(per_p)
    for k in PAIRED:
        for p in primes:
            v = [m[k] for m in per_p[p]]
            boots = [gm([v[i] for i in rng.integers(0, len(v), len(v))]) for _ in range(B)]
            lo, hi = np.percentile(boots, [2.5, 97.5])
            out["per_prime"].setdefault(p, {})[k] = {"geo_mean": gm(v), "boot95": [float(lo), float(hi)],
                                                     "min": min(v), "max": max(v), "curves": len(v)}
        allv = [(p, m[k]) for p in primes for m in per_p[p]]
        boots = []
        for _ in range(B):
            s = []
            for p in primes:
                v = per_p[p]
                s += [v[i][k] for i in rng.integers(0, len(v), len(v))]
            boots.append(gm(s))
        lo, hi = np.percentile(boots, [2.5, 97.5])
        out["pooled"][k] = {"geo_mean": gm([x for _, x in allv]), "boot95": [float(lo), float(hi)]}
        if len(primes) >= 3:
            out["ratio_slope"][k] = fit({p: [{k: m[k]} for m in per_p[p]] for p in primes}, primes, k, rng)
    return out


def same_relations(runs):
    """ALG must see exactly OLD's relations: every field but rel_* equal."""
    keys = ["group_ops", "cantor_mul", "cantor_inv", "la_mul", "la_add", "relations",
            "rels_by_lp", "decomp_trials", "attempts", "core_rows", "core_cols", "certified",
            "ok", "rel_enum_mul", "decomp_relations"]
    old = {(r["p"], r["curve_seed"], r["seed"]): r for r in runs if r["arm"] == OLD}
    n, same, diffs = 0, 0, []
    for r in runs:
        if r["arm"] != ALG:
            continue
        o = old.get((r["p"], r["curve_seed"], r["seed"]))
        if o is None:
            continue
        n += 1
        d = [k for k in keys if r.get(k) != o.get(k)]
        if d:
            diffs.append({"key": [r["p"], r["curve_seed"], r["seed"]], "fields": d})
        else:
            same += 1
    return {"compared": n, "identical": same, "differing": diffs,
            "fallbacks": int(sum(r.get("decomp_fallbacks", 0) for r in runs))}


def repro(runs):
    try:
        prev = [json.loads(l) for l in open(PREV) if l.strip()]
    except OSError:
        return {"error": "previous run not found"}
    prev = {(r["p"], r["curve_seed"], r["seed"]): r for r in prev
            if r.get("kind") == "run" and r.get("arm") == OLD}
    n, same, diffs = 0, 0, []
    for r in runs:
        if r["arm"] != OLD:
            continue
        k = (r["p"], r["curve_seed"], r["seed"])
        if k not in prev:
            continue
        n += 1
        d = [f for f in REPRO if r.get(f) != prev[k].get(f)]
        if d:
            diffs.append({"key": k, "fields": d})
        else:
            same += 1
    return {"compared": n, "identical": same, "differing": diffs}


def main(paths):
    runs = []
    for path in paths:
        for l in open(path):
            if l.strip():
                r = json.loads(l)
                if r.get("family") == "quartic" and r.get("kind") == "run":
                    runs.append(r)
    rng = np.random.default_rng(SEED)
    out = {"n_rows": len(runs), "arms": {}}
    for arm in sorted({r["arm"] for r in runs}):
        out["arms"][arm] = arm_block([r for r in runs if r["arm"] == arm], rng)
    out["paired_new_over_old"] = paired(runs, PRIMARY, OLD, rng)
    out["paired_alg_over_old"] = paired(runs, ALG, OLD, rng)
    out["paired_gt_over_alg"] = paired(runs, GT, ALG, rng)
    out["paired_new_over_gt"] = paired(runs, PRIMARY, GT, rng)
    out["paired_new_over_alg"] = paired(runs, PRIMARY, ALG, rng)
    out["alg_same_relations_as_old"] = same_relations(runs)
    out["old_arm_reproduces_20261120"] = repro(runs)
    a = out["arms"].get(PRIMARY)
    if a and "la_mul" in a["exponents"]:
        k1 = a["exponents"]["la_mul"]["ols_curve_level"] > 1.40
        k2 = a["exponents"]["fpm_mid"]["boot95"][1] >= 1.40
        out["verdict"] = {"arm": PRIMARY, "primes": a["primes"],
                          "K1_la_point_gt_1.40": bool(k1), "K2_total_upper95_ge_1.40": bool(k2),
                          "decision": "STOP" if (k1 or k2) else "PROCEED-TO-LADDER"}
    json.dump(out, sys.stdout, indent=1, default=str)
    print()


if __name__ == "__main__":
    main(sys.argv[1:])
