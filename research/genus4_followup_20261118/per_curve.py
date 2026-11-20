#!/usr/bin/env python3
"""POST HOC (validator finding F7): per-curve arm-x and arm-w ratios at w = 1 against sqrt(pi r/2), the ideal
negation bound sqrt(pi r/4) and measured plain rho Q (Q3). Writes per_curve.json. [pcc:unverified]"""
import json, os, runpy
import matplotlib; matplotlib.use("Agg")
HERE = os.path.dirname(os.path.abspath(__file__))
ns = runpy.run_path(os.path.join(HERE, "analyze.py"), run_name="per_curve")
ARMS, Q, keys, mean, ic_cost, q_cost, theo, inst = (ns[k] for k in
    ("ARMS", "Q", "keys_all", "mean", "ic_cost", "q_cost", "theo", "inst"))
out = {"post_hoc": True, "curves": []}
for c in sorted({k[1] for k in keys}):
    kk = [k for k in keys if k[1] == c]
    row = {"curve_seed": c, "h": inst[c]["h"], "r": inst[c]["r"], "keys": len(kk)}
    for a in ("x", "w"):
        d = ARMS[a]
        ic = mean([ic_cost(d[k], 1) for k in kk])
        row[a] = {"vs_plain_theory_w1": ic / mean([theo(k, False) for k in kk]),
                  "vs_neg_ideal_w1": ic / mean([theo(k, True) for k in kk]),
                  "vs_measured_q_q3_w1": ic / mean([q_cost(Q[k], "Q3", 1) for k in kk])}
    out["curves"].append(row)
json.dump(out, open(os.path.join(HERE, "per_curve.json"), "w"), indent=1)
for r in out["curves"]:
    print(r["curve_seed"], r["h"], {a: {k: round(v, 3) for k, v in r[a].items()} for a in ("x", "w")})
