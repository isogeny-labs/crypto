#!/usr/bin/env python3
"""Addendum A, H7: score singles_then_spectral on the fresh confirmation streams."""
import json, sys, os
import numpy as np
D = sys.argv[1] if len(sys.argv) > 1 else "results_confirm"
NAME = {"Random": "random", "SmallestLp": "smallest_lp", "Degree": "degree", "Spectral": "spectral",
        "CoreThenRandom": "core_then_random", "SinglesThenSpectral": "singles_then_spectral"}
def tab(path):
    t = {}
    for r in json.load(open(path))["repeats"]:
        for b in r["budgets"]:
            for o in b["outcomes"]:
                t.setdefault((b["fraction"], NAME[o["policy"]]), []).append(o["rank"]["left_nullity"])
    return t
def ci(v):
    v = np.asarray(v, float); m = v[np.random.default_rng(7).integers(0, len(v), (10000, len(v)))].mean(1)
    return [float(v.mean()), float(np.percentile(m, 2.5)), float(np.percentile(m, 97.5))]
F = [0.25, 0.1, 0.05, 0.02, 0.01]
relab = [s for s in ["fp3r", "fp4r", "fp5r", "syn2r", "real"] if os.path.exists(f"{D}/{s}.json")]
plain = ["fp3", "fp4", "fp5"]
out, a_ok, b_ok = {"a": {}, "b": {}}, True, True
for s in relab:
    t = tab(f"{D}/{s}.json")
    for f in F:
        for c in ["smallest_lp", "degree", "spectral"]:
            r = ci(np.array(t[(f, "singles_then_spectral")]) / np.array(t[(f, c)]))
            out["a"][f"{s}@{f}/{c}"] = r; a_ok &= r[1] > 1
for s in plain:
    t = tab(f"{D}/{s}.json")
    for f in F:
        r = ci(np.array(t[(f, "singles_then_spectral")]) / np.array(t[(f, "smallest_lp")]))
        out["b"][f"{s}@{f}"] = r; b_ok &= r[1] >= 0.99
out["streams_a"] = relab
out["H7a"], out["H7b"] = a_ok, b_ok
out["verdict"] = "CONFIRMED" if a_ok and b_ok else "REJECTED"
json.dump(out, open(f"{D}/h7_verdict.json", "w"), indent=1)
worst_a = min(out["a"].items(), key=lambda kv: kv[1][1]); worst_b = min(out["b"].items(), key=lambda kv: kv[1][1])
print("streams (a):", relab)
print("H7a:", a_ok, "worst", worst_a[0], [round(x, 3) for x in worst_a[1]])
print("H7b:", b_ok, "worst", worst_b[0], [round(x, 3) for x in worst_b[1]])
print("H7:", out["verdict"])
