#!/usr/bin/env python3
"""Fit the frozen PROTOCOL.md section 4 exponents from raw/run.jsonl.

Usage: python3 analyze.py raw/run.jsonl > analysis.json
Every number is [pcc:unverified].
"""
import json
import math
import sys
from collections import defaultdict

import numpy as np

T975 = {1: 12.706, 2: 4.303, 3: 3.182, 4: 2.776, 5: 2.571, 6: 2.447, 7: 2.365, 8: 2.306}


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
    }


def ols(x, y):
    x, y = np.asarray(x, float), np.asarray(y, float)
    xm = x.mean()
    b = ((x - xm) * (y - y.mean())).sum() / ((x - xm) ** 2).sum()
    return b, y.mean() - b * xm


def main(path):
    runs = [json.loads(l) for l in open(path) if l.strip()]
    runs = [r for r in runs if r.get("family") == "quartic" and r.get("kind") == "run"]
    out = {"n_rows": len(runs), "arms": {}}
    for arm in sorted({r["arm"] for r in runs}):
        rows = [r for r in runs if r["arm"] == arm]
        bad = [r for r in rows if not (r.get("certified") and r.get("ok"))]
        good = [r for r in rows if r.get("certified") and r.get("ok")]
        # curve-level geometric means
        cur = defaultdict(list)
        desc = defaultdict(list)
        for r in good:
            cur[(r["p"], r["curve_seed"])].append(metrics(r))
            desc[r["p"]].append(r)
        primes_all = sorted({r["p"] for r in rows})
        per_p = defaultdict(list)
        for (p, cs), ms in cur.items():
            per_p[p].append({k: math.exp(np.mean([math.log(m[k]) for m in ms])) for k in ms[0]})
        dropped = [p for p in primes_all if len(per_p.get(p, [])) < 2]
        primes = [p for p in primes_all if p not in dropped]
        A = {"primes": primes, "dropped_primes": dropped, "n_uncertified": len(bad),
             "uncertified": [{"p": r["p"], "curve_seed": r["curve_seed"], "seed": r["seed"],
                              "exhausted": r.get("exhausted")} for r in bad],
             "curves_per_prime": {p: len(per_p[p]) for p in primes_all},
             "exponents": {}, "per_prime": {}}
        for p in primes_all:
            rs = desc[p]
            if not rs:
                continue
            ms = [metrics(r) for r in rs]
            A["per_prime"][p] = {
                "runs": len(rs),
                "geo_mean": {k: math.exp(np.mean([math.log(m[k]) for m in ms])) for k in ms[0]},
                "la_share_fpm_mid_mean": float(np.mean([9 * m["la_mul"] / m["fpm_mid"] for m in ms])),
                "fb_cols_mean": float(np.mean([r["fb_cols"] for r in rs])),
                "core_cols_mean": float(np.mean([r["core_cols"] for r in rs])),
                "core_rows_mean": float(np.mean([r["core_rows"] for r in rs])),
                "relations_mean": float(np.mean([r["relations"] for r in rs])),
                "attempts_max": int(max(r["attempts"] for r in rs)),
                "group_ops_mean": float(np.mean([r["group_ops"] for r in rs])),
            }
        rng = np.random.default_rng(20261120)
        for k in ["la_mul", "la_mul_add", "fpm_mid", "fpm_low", "fpm_high",
                  "gae_0", "gae_0.1", "gae_1", "rel_mul"]:
            xs, ys = [], []
            for p in primes:
                for m in per_p[p]:
                    xs.append(math.log(p))
                    ys.append(math.log(m[k]))
            if len(primes) < 3:
                continue
            b, _ = ols(xs, ys)
            # cluster bootstrap within prime
            boots = []
            for _ in range(4000):
                bx, by = [], []
                for p in primes:
                    v = per_p[p]
                    idx = rng.integers(0, len(v), len(v))
                    for i in idx:
                        bx.append(math.log(p))
                        by.append(math.log(v[i][k]))
                boots.append(ols(bx, by)[0])
            lo, hi = np.percentile(boots, [2.5, 97.5])
            # OLS on per-prime means, t interval
            px = np.array([math.log(p) for p in primes])
            py = np.array([np.mean([math.log(m[k]) for m in per_p[p]]) for p in primes])
            bm, am = ols(px, py)
            res = py - (am + bm * px)
            df = len(primes) - 2
            se = math.sqrt((res ** 2).sum() / df / ((px - px.mean()) ** 2).sum()) if df > 0 else float("nan")
            t = T975.get(df, 2.0)
            local = []
            for i in range(len(primes) - 1):
                local.append({"from": primes[i], "to": primes[i + 1],
                              "slope": float((py[i + 1] - py[i]) / (px[i + 1] - px[i]))})
            A["exponents"][k] = {
                "ols_curve_level": float(b),
                "boot95": [float(lo), float(hi)],
                "means_ols": float(bm),
                "means_t95": [float(bm - t * se), float(bm + t * se)],
                "local_slopes": local,
            }
        out["arms"][arm] = A
    a = out["arms"].get("Q_diem_2lp")
    if a and "la_mul" in a["exponents"]:
        k1 = a["exponents"]["la_mul"]["ols_curve_level"] > 1.40
        k2 = a["exponents"]["fpm_mid"]["boot95"][1] >= 1.40
        out["verdict"] = {"K1_la_point_gt_1.40": bool(k1), "K2_total_upper95_ge_1.40": bool(k2),
                          "decision": "STOP" if (k1 or k2) else "PROCEED-TO-LADDER"}
    json.dump(out, sys.stdout, indent=1, default=str)
    print()


if __name__ == "__main__":
    main(sys.argv[1])
