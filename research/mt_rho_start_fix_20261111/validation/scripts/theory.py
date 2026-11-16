#!/usr/bin/env python3
"""Validator: compare measured per-target costs (cI = 10, h = 0, verification excluded) with first-order models.
 anchored rho2x at kappa: phase-1 steps D/2 (ideal), phase-2 steps n/(4D) per target, D = kappa sqrt(nT),
   step = I + 3M + 2S (15/13 GAE at cI = 10);
 Kuhn-Struik rho2x with retention (store every solved walk point): total steps ~ sqrt(D0^2 + nT)/2;
 ic5 at mu = 1: table K/2 xpairs + T n/(4K) xpairs, xpair = I + 2M + 2S (14/13 GAE), K = sqrt(nT/2) (+ combos, relations);
 Kuhn-Struik DP rho on {P, -P} (estimate, nothing measured): steps sqrt(nT) + theta T, step = 1 GAE, 2 hash per theta steps.
usage: theory.py <rows.jsonl> <out.json>"""
import json, sys, math, collections, statistics
rows = [json.loads(l) for l in open(sys.argv[1])]
def g(v, ci=10, h=0): return (ci * v[0] + v[1] + v[2]) / (ci + 3) + h * (v[3] + v[4])
out = {}
by = collections.defaultdict(list)
for r in rows:
    if (r["arm"] == "rho2x" and r["param"] == 0.7) or r["arm"] == "ic5":
        ks = list(r["phases"]); nv = r["phases"][ks[-2]]
        by[(r["arm"], r["pb"], r["T"])].append((g(nv) / r["targets"], r["n"], r["T"]))
for (arm, pb, T), v in sorted(by.items()):
    meas = statistics.median(x[0] for x in v); n = statistics.median(x[1] for x in v)
    if arm == "rho2x":
        D = 0.7 * math.sqrt(n * T); model = (D / 2 + T * n / (4 * D)) * 15 / 13 / T
    else:
        K = math.sqrt(n * T / 2); model = (K / 2 + T * n / (4 * K)) * 14 / 13 / T
    out[f"{arm}/{pb}/{T}"] = dict(measured_noverify=round(meas, 2), first_order_model=round(model, 2), ratio=round(meas / model, 4))
for pb in (22, 24, 28, 32):
    for T in (1024, 4096, 16384):
        n = statistics.median(x[1] for x in by[("ic5", pb, T)])
        ks = math.sqrt(n * T) / 2 * 15 / 13 / T
        s = math.sqrt(n / T); th = max(2.0, math.sqrt(2 * s))
        out[f"ks_rho2x_model/{pb}/{T}"] = round(ks, 2)
        out[f"ks_dp_model_h1/{pb}/{T}"] = dict(theta=round(th, 1), gae_per_target_h0=round(s + th, 2), gae_per_target_h1=round((s + th) * (1 + 2 / th), 2))
json.dump(out, open(sys.argv[2], "w"), indent=1)
for k, v in out.items(): print(k, v)
