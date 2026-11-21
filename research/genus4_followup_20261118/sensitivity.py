#!/usr/bin/env python3
"""POST-HOC sensitivities (written after reading analysis.json; not part of the frozen protocol).
1. Monte Carlo stability of arm x's Q3 w = 1 upper bound: the frozen bootstrap (NB = 2000) with 50
   other seeds; the frozen verdict uses seed 20261123 only.
2. IC-only interval against the theoretical plain-rho expectation sqrt(pi r / 2) (no rho sampling noise),
   and against the ideal negation bound sqrt(pi r / 4).
Every number [pcc:unverified]."""
import json, math, os, random, runpy, sys
HERE = os.path.dirname(os.path.abspath(__file__))
sys.argv = ["analyze.py"]
import matplotlib; matplotlib.use("Agg")
ns = runpy.run_path(os.path.join(HERE, "analyze.py"), run_name="sens")
ARMS, Q, keys, interval, mean, ic_cost, q_cost, theo = (ns[k] for k in
    ("ARMS", "Q", "keys_all", "interval", "mean", "ic_cost", "q_cost", "theo"))
out = {"post_hoc": True}
for a in ("x", "w"):
    d = ARMS[a]
    fn = lambda kk, d=d: mean([ic_cost(d[k], 1) for k in kk]) / mean([q_cost(Q[k], "Q3", 1) for k in kk])
    his = [interval(fn, keys, seed=s)["hi"] for s in range(1, 51)]
    out[a] = {"q3_w1_hi_over_50_seeds": {"min": min(his), "median": sorted(his)[25], "max": max(his),
                                         "frac_below_1": sum(h < 1 for h in his) / len(his)}}
    for nm, neg in (("plain_theory", False), ("neg_ideal", True)):
        for wn, w in (("0.1", 0.1), ("1", 1.0)):
            out[a][f"vs_{nm}_w{wn}"] = interval(lambda kk, d=d, w=w, neg=neg: mean([ic_cost(d[k], w) for k in kk]) / mean([theo(k, neg) for k in kk]), keys)
json.dump(out, open(os.path.join(HERE, "sensitivity.json"), "w"), indent=1)
print(json.dumps(out, indent=1))
