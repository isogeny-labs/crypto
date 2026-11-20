#!/usr/bin/env python3
"""Reduces results.json (written by the frozen analyze.py) to summary.json: ratios with 95% intervals per weight, mean S per arm,
phase split, native counts, wall times, controls. Recorded ecbench outputs only; no new statistics."""
import json, os, gzip
HERE = os.path.dirname(os.path.abspath(__file__)); _p = os.path.join(HERE, "results.json")
R = json.load(open(_p) if os.path.exists(_p) else gzip.open(_p + ".gz", "rt"))
W = ["0", "0.1", "1"]
S = {"ratios": {}, "mean_S": {}, "phases": R["phases"], "native": R["native"], "wall": R["wall"], "session": R["session"],
     "identity": R["identity"], "same_walk": R["same_walk"]}
for key, c in R["compare"].items():
    n, pair, w = key.split("|"); w = w[2:]
    o = c["ops"]; S["ratios"].setdefault(pair, {})[w] = {"ratio": o["ratio_b_over_a"], "lo": o["ci95"][0], "hi": o["ci95"][1], "pairs": o["pairs"]}
    for side in ("a", "b"):
        S["mean_S"].setdefault(c[side]["arm"], {})[w] = c[side]["mean_s"]
S["band_S"] = {row["arm"]: dict(zip(W, row["mean_s"])) for row in R["band"]["rows"]}
S["band_ranks"] = {f'{x["a"]} vs {x["b"]}': x["verdict"] for x in R["band"]["ranks"]}
json.dump(S, open(os.path.join(HERE, "summary.json"), "w"), indent=1, sort_keys=True)
for pair, v in sorted(S["ratios"].items()):
    print(f"  {pair:24s} " + "  ".join(f"w={w}: {v[w]['ratio']:.3f} [{v[w]['lo']:.3f},{v[w]['hi']:.3f}]" for w in W if w in v))
print("identity", S["identity"], "same_walk", S["same_walk"])
