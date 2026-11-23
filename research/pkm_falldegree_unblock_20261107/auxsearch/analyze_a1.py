#!/usr/bin/env python3
"""Amendment A1 analysis: A1 seeds against the exact finite-p prediction;
also the first sweep against the same prediction (descriptive)."""
import glob, json, math, statistics as st
from analyze import interval
ex = {r["t"]: r for r in json.load(open("exact/predict_exact.json"))["rows"]}
a1 = [json.loads(l) for f in sorted(glob.glob("a1.out.*")) for l in open(f) if l.strip()]
s1 = [json.loads(l) for f in sorted(glob.glob("sweep.out.*")) for l in open(f) if l.strip()]
def stats(R, t):
    P = ex[t]["p_accept"]; n = len(R)
    tries = [r["info"]["aux_curves_tried"] for r in R]
    m = st.mean(tries); r_t = m / 2 ** t
    lo, hi = interval(t, P, n=n, reps=10000)
    z = (m - 1 / P) / (math.sqrt(1 - P) / P / math.sqrt(n))
    return {"t": t, "n": n, "mean_tries": m, "r_t": r_t, "E_exact_over_2t": ex[t]["ratio_to_2t"],
            "interval99": [lo, hi], "holds": lo <= r_t <= hi, "z": z,
            "ops_per_curve": sum(r["info"]["aux_ops_search"]["adds"] + r["info"]["aux_ops_search"]["doubles"] for r in R) / sum(tries)}
out = {"schema": "auxsearch.a1/v1", "a1": [], "first_sweep_vs_exact": [], "combined": []}
for t in (12, 13, 14, 15, 16):
    out["a1"].append(stats([r for r in a1 if r["t"] == t], t))
for t in range(8, 15):
    out["first_sweep_vs_exact"].append(stats([r for r in s1 if r["t"] == t], t))
for t in (12, 13, 14):
    out["combined"].append(stats([r for r in a1 + s1 if r["t"] == t], t))
zs = [r["z"] for r in out["a1"] if r["t"] in (12, 13, 14)]
out["a1_pooled_z_12_14"] = sum(zs) / math.sqrt(3)
out["a1_pooled_miss"] = abs(out["a1_pooled_z_12_14"]) > 2.58
json.dump(out, open("a1_summary.json", "w"), indent=1)
for k in ("a1", "first_sweep_vs_exact", "combined"):
    print(k)
    for r in out[k]:
        print(" ", r["t"], r["n"], round(r["r_t"], 3), "exp", round(r["E_exact_over_2t"], 3),
              [round(x, 3) for x in r["interval99"]], r["holds"], "z=%.2f" % r["z"], round(r["ops_per_curve"]))
print("pooled z (A1, t=12..14) = %.2f miss=%s" % (out["a1_pooled_z_12_14"], out["a1_pooled_miss"]))
