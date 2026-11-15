#!/usr/bin/env python3
"""Summarise the frozen 30-seed oversampling run (protocol.json). Reads rows/*.jsonl,
writes full_summary.json. Pilot statistics size the frozen run; they are not
the measurement and support no claim."""
import json, glob, os, random, statistics as st, math
HERE = os.path.dirname(os.path.abspath(__file__))
FR = ["f0.1", "f0.25", "f0.5"]
def boot(xs, n=10000, seed=1):
    rng = random.Random(seed); m = len(xs)
    ms = sorted(sum(rng.choice(xs) for _ in range(m)) / m for _ in range(n))
    return [ms[int(0.025 * n)], ms[int(0.975 * n) - 1]]
out = {"schema": "logs_oversampling.full_summary/v1", "cells": {}}
for path in sorted(glob.glob(os.path.join(HERE, "rows", "C*.jsonl"))):
    cell = os.path.basename(path)[:-6]
    rows = [json.loads(l) for l in open(path)]
    failed = [r for r in rows if not r.get("verified")]
    by = {}
    for r in rows: by.setdefault(r["seed_index"], {})[r["arm"]] = r
    seeds = sorted(by)
    f0 = [by[s]["f0"] for s in seeds]
    la_per_op = st.median(r["linear_algebra"]["seconds"] / max(r["linear_algebra"]["row_ops"], 1) for r in f0)
    coll_per_trial = st.median(r["collection"]["seconds_prorated"] / r["collection"]["trials"] for r in f0)
    lam = la_per_op / coll_per_trial
    c0 = st.mean(r["collection"]["trials"] + lam * r["linear_algebra"]["row_ops"] for r in f0)
    cell_out = {"seeds": len(seeds), "rows": len(rows), "failed_arms": len(failed),
                "columns": f0[0]["columns"], "lambda_hat_trials_per_row_op": lam,
                "mean_C_f0_trials_at_lambda_hat": c0,
                "f0_core_dimension": [r["linear_algebra"]["core_dimension"] for r in f0],
                "f0_la_share_of_wall": st.median(r["linear_algebra"]["seconds"] / r["total_wall_seconds"] for r in f0),
                "arms": {}}
    aa = [by[s]["f0_aa"]["linear_algebra"]["seconds"] - by[s]["f0"]["linear_algebra"]["seconds"] for s in seeds]
    cell_out["aa_la_seconds_diff"] = {"values": aa, "max_abs": max(abs(x) for x in aa)}
    for arm in FR + ["logs_default"]:
        dt = [by[s][arm]["collection"]["trials"] - by[s]["f0"]["collection"]["trials"] for s in seeds]
        dr = [by[s][arm]["linear_algebra"]["row_ops"] - by[s]["f0"]["linear_algebra"]["row_ops"] for s in seeds]
        dw = [by[s][arm]["total_wall_seconds"] - by[s]["f0"]["total_wall_seconds"] for s in seeds]
        a = {"delta_trials": dt, "delta_row_ops": dr, "delta_wall_s_mean": st.mean(dw), "by_lambda": {}}
        for name, l in [("0", 0.0), ("lambda_hat/10", lam / 10), ("lambda_hat", lam), ("10*lambda_hat", 10 * lam)]:
            dc = [t + l * r for t, r in zip(dt, dr)]
            sd = st.stdev(dc) if len(dc) > 1 else float("nan")
            # seeds for a 95% CI half-width of 1% of mean C(f0)
            n_req = math.ceil((1.96 * sd / (0.01 * c0)) ** 2) if sd == sd else None
            a["by_lambda"][name] = {"mean": st.mean(dc), "sd": sd, "ci95_boot": boot(dc),
                                    "seeds_for_halfwidth_1pct_C0": n_req}
        cell_out["arms"][arm] = a
    out["cells"][cell] = cell_out
json.dump(out, open(os.path.join(HERE, "full_summary.json"), "w"), indent=1)
for c, d in out["cells"].items():
    print(f"{c}: cols {d['columns']} failed {d['failed_arms']} lam {d['lambda_hat_trials_per_row_op']:.4g} C0 {d['mean_C_f0_trials_at_lambda_hat']:.1f} core {d['f0_core_dimension']} LA share {d['f0_la_share_of_wall']:.3%}")
    for arm, a in d["arms"].items():
        L = a["by_lambda"]
        print(f"   {arm:12s} dTrials {st.mean(a['delta_trials']):8.1f} dRowOps {st.mean(a['delta_row_ops']):8.1f} | dC@lam {L['lambda_hat']['mean']:8.1f} CI {[round(x,1) for x in L['lambda_hat']['ci95_boot']]} @10lam {L['10*lambda_hat']['mean']:8.1f} | n1% {L['lambda_hat']['seeds_for_halfwidth_1pct_C0']}")
