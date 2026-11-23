#!/usr/bin/env python3
"""EXPLORATORY (post-hoc; not in PROTOCOL.md). Poisson fit hits ~ c * exposure * base_points^k over blind arms.
Run: python3 -I exploratory.py > exploratory.txt"""
import json, math, os
here = os.path.dirname(os.path.abspath(__file__)); prev = os.path.join(here, "..", "curve25519_isogeny_seeds_20261114", "runs")
g = json.load(open(os.path.join(here, "groups.json")))
def load(f): a = json.load(open(f)); return (a["hits_verified"], a["f4_muladds"]/1e9, a["base_points"])
sets = {"seeds 0-20 (fit set)": [load(os.path.join(prev, ("iso_L0.json" if s == 0 else "s%d_L0.json" % s))) for s in range(21)],
        "screen arms (16)": [load(os.path.join(here, "runs", "s%d_L0.json" % s)) for s in g["top"] + g["bottom"]]}
sets["all 37"] = sets["seeds 0-20 (fit set)"] + sets["screen arms (16)"]
def prof(d, k):
    c = sum(h for h, e, b in d) / sum(e*b**k for h, e, b in d)
    ll = sum(h*math.log(c*e*b**k) - c*e*b**k - math.lgamma(h+1) for h, e, b in d); return c, ll
for name, d in sets.items():
    best = max((prof(d, k/20)[1], k/20) for k in range(0, 61))
    l1 = prof(d, 1.0)[1]; l2 = prof(d, 2.0)[1]
    # profile likelihood 95% interval for k: ll >= best - 1.92
    ok = [k/20 for k in range(0, 61) if prof(d, k/20)[1] >= best[0] - 1.92]
    print("%-22s MLE exponent k=%.2f, 95%% profile interval [%.2f, %.2f]; loglik(k=1) - best = %.2f, loglik(k=2) - best = %.2f" % (name, best[1], min(ok), max(ok), l1 - best[0], l2 - best[0]))
