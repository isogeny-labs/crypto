#!/usr/bin/env python3
"""fig_lanes.svg/.png/.pdf from summary.json (the frozen analysis output). Optional args: summary path, output dir."""
import json, os, sys
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
HERE = os.path.dirname(os.path.abspath(__file__))
summ = json.load(open(sys.argv[1] if len(sys.argv) > 1 else os.path.join(HERE, "summary.json")))
out = sys.argv[2] if len(sys.argv) > 2 else HERE
cells = sorted(summ["cells"], key=lambda c: tuple(map(int, c.split(":"))))
labels = [f"2^{c.split(':')[0]}\nT={c.split(':')[1]}" for c in cells]
fig, (a, b) = plt.subplots(1, 2, figsize=(12, 4.4))
S1 = [("rho (all lanes)/rho L32", "best rho (lanes 16-128) / best rho at 32 lanes", "#2a6fdb"),
      ("IC (all B)/IC B32", "best IC (B 32-256) / IC at B = 32", "#d1495b")]
S2 = [("IC B32/rho L32", "untuned: IC B = 32 / rho 32 lanes", "#999999"),
      ("IC (all B)/rho (all lanes)", "both tuned: best IC / best rho", "#222222")]
for ax, series, title in ((a, S1, "What tuning buys each arm"), (b, S2, "Distance remaining to rho")):
    for j, (key, lab, col) in enumerate(series):
        off = (j - 0.5) * 0.3
        for x, c in enumerate(cells):
            d = summ["cells"][c][key]
            ax.plot([x + off] * len(d["per_instance"]), d["per_instance"], "o", color=col, alpha=0.4, ms=4)
            ax.plot([x + off], [d["median"]], "_", color=col, ms=14, mew=2.5, label=lab if x == 0 else None)
    ax.axhline(1, color="#888", lw=0.8, ls="--")
    ax.set_xticks(range(len(cells))); ax.set_xticklabels(labels, fontsize=8)
    ax.set_ylabel("whole-solve time ratio (bar median, dots instances)"); ax.set_title(title)
    ax.legend(frameon=False, fontsize=8)
fig.suptitle("Rho lanes and IC batch tuned on both arms, local Xeon [pcc:unverified]", fontsize=11)
fig.tight_layout()
for ext in ("svg", "png", "pdf"):
    fig.savefig(os.path.join(out, f"fig_lanes.{ext}"), dpi=150 if ext == "png" else None)
