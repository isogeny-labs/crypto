#!/usr/bin/env python3
"""Validator ESTIMATE (model, nothing measured): a Kuhn-Struik distinguished-point rho on {P, -P} classes
(retain every solved walk's distinguished points; one hash lookup and one insert per distinguished point).
Per target: steps = sqrt(n/T) (KS continuous limit, D_T^2 = nT, no precomputation) + theta (walk to the
distinguished point after merging), theta = sqrt(2 sqrt(n/T)) (optimal at h = 1, floored at 2);
field cost 1 GAE per step (one affine addition = I + 2M + S, by the definition of the unit at every cI);
hash ops 2 (steps / theta); plus the measured verification constant of the same configuration (ic5's verify
phase, identical across arms).  Fruitless-cycle handling and restarts are not charged (optimistic for rho),
the target walks' first-collision overhead beyond theta is not charged.  Compared with ic5 measured.
usage: dp_estimate.py <rows.jsonl> <out.json> [arm (default ic5)]"""
import json, sys, math, statistics, collections
rows = [json.loads(l) for l in open(sys.argv[1])]
ARM = sys.argv[3] if len(sys.argv) > 3 else "ic5"
def g(v, ci, h): return (ci * v[0] + v[1] + v[2]) / (ci + 3) + h * (v[3] + v[4])
cfg = collections.defaultdict(dict)
for r in rows:
    if r["arm"] == ARM: cfg[(r["pb"], r["T"], r["idx"], r["seed"])] = r
out = {}
for ci in [3, 10, 30]:
    for h in [0, 0.1, 1]:
        for pb, T in sorted({k[:2] for k in cfg}):
            if True:
                rat = []
                for k, r in cfg.items():
                    if k[:2] != (pb, T): continue
                    ks = list(r["phases"]); ver = [b - a for a, b in zip(r["phases"][ks[-2]], r["phases"][ks[-1]])]
                    s = math.sqrt(r["n"] / T)
                    best = min(((s + th) * (1 + 2 * h / th)) for th in [max(2.0, x / 4) for x in range(4, 1024)])
                    dp = best + g(ver, ci, h) / T
                    rat.append(g([r["I"], r["M"], r["S"], r["H"], r["L"]], ci, h) / T / dp)
                out[f"cI{ci}/h{h}/{pb}/{T}"] = round(statistics.median(rat), 3)
json.dump(out, open(sys.argv[2], "w"), indent=1)
for ci in [3, 10, 30]:
    for h in [0, 0.1, 1]:
        print(f"cI={ci} h={h}: {ARM} / KS-DP-rho (estimate) per cell:", [(f"{pb}/{T}", out[f"cI{ci}/h{h}/{pb}/{T}"]) for pb, T in sorted({k[:2] for k in cfg})])
