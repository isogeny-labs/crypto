"""Plots recorded values only: raw/rule.jsonl (output of the instrument's --mode rule)."""
import json, sys
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
import numpy as np
rows = [json.loads(l) for l in open("raw/rule.jsonl")]
rr = [r for r in rows if r["kind"] == "rule_row" and r["n"] == 83]
cells = [(1, 74), (0, 93), (1, 300), (0, 300)]
prices = [0, 0.1, 1]
fig, axes = plt.subplots(1, 2, figsize=(11, 4.4), sharey=True)
colors = {"cyc": "#1f77b4", "onb": "#d95f02"}
for ax, acc in zip(axes, ["faithful", "tidy"]):
    x = np.arange(len(cells)); wd = 0.13
    k = 0
    for arm in ["cyc", "onb"]:
        for p in prices:
            v = []
            for a, K in cells:
                r = [q for q in rr if q["a"] == a and q["k"] == K and q["arm"] == arm and q["accounting"] == acc and q["price"] == p][0]
                v.append(r["ratio"])
            for xi, vi in zip(x + (k - 2.5) * wd, v):
                if vi > 1.2: ax.text(xi, 1.19, f"{vi:.2f}", rotation=90, ha="center", va="top", fontsize=7, color="w")
            ax.bar(x + (k - 2.5) * wd, v, wd, color=colors[arm], alpha=[1.0, 0.65, 0.35][prices.index(p)],
                   label=f"{arm}, c={p}")
            k += 1
    ax.axhline(1.0, color="k", lw=0.8); ax.axhline(1.01, color="gray", ls="--", lw=0.8); ax.axhline(0.99, color="gray", ls="--", lw=0.8)
    ax.set_xticks(x); ax.set_xticklabels([f"n=83 a={a}\nK={K}" for a, K in cells])
    ax.set_title(f"{acc} accounting"); ax.set_ylim(0.97, 1.2)
axes[0].set_ylabel("S_ready(arm) / S_ready(cos)  (axis clipped at 1.2)")
axes[0].legend(fontsize=7, ncol=2, loc="upper center")
fig.suptitle("Item 17: base-dependent oracle cost at n = 83, arm vs compact-orbit-scan (c = native price, GAE/op; dashed = 1% materiality)", fontsize=9)
fig.text(0.01, 0.005, "Source: raw/rule.jsonl from examples/blindspots_17_cyclo_cost.rs --mode rule; cards 01-07. Deterministic counts: no sampling interval.", fontsize=7)
fig.tight_layout(rect=(0, 0.03, 1, 0.95))
fig.savefig("graph.svg"); fig.savefig("graph.png", dpi=150)
