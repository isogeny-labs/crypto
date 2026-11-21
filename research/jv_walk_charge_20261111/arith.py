#!/usr/bin/env python3
"""Secondary statistic, declared after the run: arithmetic-mean cost per target, (mean S_U + mean reach) / ref_p,
per prime and M, w = 0 (U2). An attacker pays the arithmetic mean; the protocol's geometric mean damps the tail."""
import json, math, glob
V = "../jv_sieve_line_vs_rho_20261103/results"
ref = {}
for p in (1009, 1511, 1823):
    xs = [r["s_tuned"] for f in glob.glob(f"{V}/rho/rhostrong_p{p}_s*_run*.json") for r in json.load(open(f))]
    ref[p] = sum(xs) / len(xs)
out = {}
for p in (1009, 1511, 1823):
    U = {r["seed"]: r["total_muls"] / (331 * math.sqrt(r["l"])) for r in json.load(open(f"{V}/cover/U_{p}.json"))}
    for m in (8, 32, 128, 512):
        reach = []
        for s in range(17, 25):
            d = json.load(open(f"runs/p{p}_s{s}_m{m}.json")); d = d[0] if isinstance(d, list) else d
            reach.append((d["walk_muls"] + d["transport_muls"] + d["model_muls"]) / (331 * math.sqrt(d["l"])))
        mu, mr = sum(U.values()) / 8, sum(reach) / 8
        out[f"p{p}_M{m}"] = {"route_over_rho": mu / ref[p], "charged_over_rho": (mu + mr) / ref[p], "reach_over_route": mr / mu}
json.dump(out, open("arith.json", "w"), indent=1)
for k, v in out.items(): print(k, {a: round(b, 4) for a, b in v.items()})
