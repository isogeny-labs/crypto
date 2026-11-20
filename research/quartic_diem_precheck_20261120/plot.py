#!/usr/bin/env python3
"""Log-log plot of per-curve Q_diem_2lp costs vs p. [pcc:unverified]"""
import json, math, collections, sys
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
R = [json.loads(l) for l in open("raw/run.jsonl") if '"kind":"run"' in l and "2lp" in l]
series = {
    "la_mul (LA count)": lambda r: r["la_mul"],
    "FPM mid (total)": lambda r: r["cantor_mul"] + 10 * r["cantor_inv"] + r["rel_mul"] + 9 * r["la_mul"],
    "rel_mul (relations + descent)": lambda r: r["rel_mul"],
}
fig, ax = plt.subplots(figsize=(6.4, 4.4))
cols = ["#2a6fb0", "#c0392b", "#7f8c8d"]
for (name, f), c in zip(series.items(), cols):
    cur = collections.defaultdict(list)
    for r in R:
        cur[(r["p"], r["curve_seed"])].append(math.log(f(r)))
    xs = [p for (p, _) in cur]; ys = [math.exp(sum(v) / len(v)) for v in cur.values()]
    ax.scatter(xs, ys, s=14, color=c, label=name, alpha=0.8)
ps = [1031, 32771]
for slope, ls in [(1.0, ":"), (1.4, "--")]:
    y0 = 3.6e6
    ax.plot(ps, [y0 * (p / 1031) ** slope for p in ps], ls, color="k", lw=0.8, label=f"slope {slope}")
ax.set_xscale("log"); ax.set_yscale("log")
ax.set_xlabel("p"); ax.set_ylabel("counted operations per solve (curve geo-mean)")
ax.set_title("Diem 2LP on prime-order plane quartics [pcc:unverified]", fontsize=10)
ax.legend(fontsize=8); fig.tight_layout()
fig.savefig("fig_scaling.svg"); fig.savefig("fig_scaling.png", dpi=130)
