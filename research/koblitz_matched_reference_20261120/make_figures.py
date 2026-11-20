#!/usr/bin/env python3
"""Figure for EXP-KOBLITZ-REF-002 from results.json (Amendment 1 run)."""
import json
import os

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

plt.rcParams["svg.fonttype"] = "none"

HERE = os.path.dirname(os.path.abspath(__file__))
R = json.load(open(os.path.join(HERE, "results.json")))
cur = R["curves"]
ns = sorted(cur, key=lambda n: cur[n]["log2_r"])
lr = [cur[n]["log2_r"] for n in ns]
W = [("0", "#555555"), ("0.1", "#2f6db5"), ("1", "#c8553d")]
fig, (ax, bx) = plt.subplots(1, 2, figsize=(11, 4.2))
for w, col in W:
    r = [cur[n]["ratios"]["rho-v5/rho-strong"][w] for n in ns]
    ax.errorbar(lr, [x["ratio"] for x in r], yerr=[[x["ratio"] - x["lo"] for x in r], [x["hi"] - x["ratio"] for x in r]],
                marker="o", color=col, capsize=3, label=f"tallied, w = {w}")
    a = [cur[n]["audit_ratios"]["rho-v5/rho-strong"][w]["ratio"] for n in ns]
    ax.plot(lr, a, ls="--", color=col, alpha=0.6, label=f"audit (untallied), w = {w}")
ax.axhline(1, color="k", lw=0.8)
ax.set_xlabel("log2 r")
ax.set_ylabel("v5 ÷ strong (paired, 95% CI)")
ax.set_title("v5 ÷ strong with v5's jump selections band-priced")
ax.legend(fontsize=7)
ax.grid(alpha=0.3)
labels = []
for i, n in enumerate(ns):
    m = cur[n]["matched"]
    gen = [a for a in m["0"] if a.startswith("gen-")]
    best = min(gen, key=lambda a: m["0.1"][a]["ratio"])
    labels.append(f"n{n}\n{best}")
    for j, (w, col) in enumerate(W):
        x = m[w][best]
        bx.errorbar(i + (j - 1) * 0.22, x["ratio"], yerr=[[x["ratio"] - x["lo"]], [x["hi"] - x["ratio"]]], fmt="s",
                    color=col, capsize=3, label=f"w = {w}" if i == 0 else None)
bx.axhline(1, color="k", lw=0.8)
bx.set_xticks(range(len(ns)))
bx.set_xticklabels(labels, fontsize=8)
bx.set_ylabel("IC ÷ matched rho = min(strong, tallied v5)")
bx.set_title("distance remaining of the validated generator-orbit IC arm")
bx.legend(fontsize=7)
bx.grid(alpha=0.3, axis="y")
fig.tight_layout()
for ext in ("svg", "png"):
    fig.savefig(os.path.join(HERE, "figures", f"fig1-matched-reference.{ext}"), dpi=150)
print("ok")
