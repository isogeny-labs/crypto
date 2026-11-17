#!/usr/bin/env python3
"""Lay out recorded values (ratios.csv, pairs.txt) as SVG + PNG; plotting only, no analysis."""
import csv
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

HERE = Path(__file__).resolve().parent
C = {"frozen_protocol": "#2a78d6", "posthoc": "#eb6834", "earlier_exploratory": "#1baf7a"}
NAME = {"frozen_protocol": "frozen protocol (8 pairs)", "posthoc": "post-hoc replicate (32 pairs)",
        "earlier_exploratory": "earlier exploratory (4 / 8 pairs)"}
plt.rcParams.update({"font.size": 10, "axes.edgecolor": "#52514e", "axes.spines.top": False,
                     "axes.spines.right": False, "axes.grid": True, "grid.color": "#e4e3df",
                     "figure.facecolor": "#fcfcfb", "axes.facecolor": "#fcfcfb"})

rows = list(csv.DictReader(open(HERE / "ratios.csv")))
fig, ax = plt.subplots(figsize=(6.6, 4.2))
seen = set()
for i, r in enumerate(rows):
    n, g, lo, hi = int(r["n"]), float(r["geomean_on_over_off"]), float(r["ci_lo"]), float(r["ci_hi"])
    x = n + {"frozen_protocol": 0, "posthoc": 0, "earlier_exploratory": 0.7}[r["label"]]
    lab = NAME[r["label"]] if r["label"] not in seen else None
    seen.add(r["label"])
    ax.errorbar([x], [g], yerr=[[g - lo], [hi - g]], fmt="o", color=C[r["label"]], capsize=4, label=lab)
pairs = [l.split() for l in open(HERE / "pairs.txt")]
ax.plot([41 - 1.0] * len(pairs), [float(p[4]) / float(p[1]) for p in pairs], "_", color="#52514e", alpha=0.7,
        label="n = 41 single pairs")
ax.axhline(1, color="#52514e", lw=1, ls=":")
ax.set_yscale("log")
ax.set_xlabel("n (a = 0, cofactor 4)")
ax.set_ylabel("trials ratio, torsion on / off")
ax.set_xticks([23, 31, 41])
ax.legend(frameon=False, fontsize=8, loc="upper left")
fig.tight_layout()
fig.savefig(HERE / "torsion_ratio_vs_n.svg")
fig.savefig(HERE / "torsion_ratio_vs_n.png", dpi=150)
