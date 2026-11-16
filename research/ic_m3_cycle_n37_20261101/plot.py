#!/usr/bin/env python3
"""Lay out recorded values (ratios.csv, runs_m3.txt, runs_m2_ref.txt) as SVG + PNG; plotting only, no analysis."""
import csv
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

HERE = Path(__file__).resolve().parent
plt.rcParams.update({"font.size": 10, "axes.edgecolor": "#52514e", "axes.spines.top": False,
                     "axes.spines.right": False, "axes.grid": True, "grid.color": "#e4e3df",
                     "figure.facecolor": "#fcfcfb", "axes.facecolor": "#fcfcfb"})
m3 = [l.split() for l in open(HERE / "runs_m3.txt")]
m2 = [l.split() for l in open(HERE / "runs_m2_ref.txt")]
rat = list(csv.DictReader(open(HERE / "ratios.csv")))
ds = [12, 10, 8]
fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(10.2, 4.1))
# left: per-seed trials ratio m=3 / m=2 and geometric mean with 95% bootstrap
pos = {d: i for i, d in enumerate(ds)}
for d in ds:
    t2 = {r[1]: int(r[2]) for r in m2 if int(r[0]) == d}
    pts = [int(r[2]) / t2[r[1]] for r in m3 if int(r[0]) == d]
    ax1.plot([pos[d]] * len(pts), pts, "_", color="#bdbcb6", ms=11, zorder=1)
g = [float(r["trials_geomean"]) for r in rat]
lo = [float(r["trials_lo"]) for r in rat]
hi = [float(r["trials_hi"]) for r in rat]
ax1.errorbar(range(3), g, yerr=[[a - b for a, b in zip(g, lo)], [b - a for a, b in zip(g, hi)]], fmt="o",
             color="#2a78d6", capsize=4, zorder=2, label="paired geometric mean, 95% bootstrap")
ax1.axhline(1, color="#52514e", lw=1, ls=":")
ax1.set_yscale("log")
ax1.set_xticks(range(3)); ax1.set_xticklabels([f"d={d}\n(K={r['K']})" for d, r in zip(ds, rat)])
ax1.set_ylabel("decomposer calls, m=3 / m=2 (cycle stop)")
ax1.set_title("cost ratio per seed", fontsize=10)
ax1.legend(frameon=False, fontsize=8, loc="upper right")
# right: relations needed before the stop
def mean(xs): return sum(xs) / len(xs)
r3 = [mean([float(r[5]) for r in m3 if int(r[0]) == d]) for d in ds]
r2 = [mean([float(r[5]) for r in m2 if int(r[0]) == d]) for d in ds]
w = 0.36
ax2.bar([i - w / 2 for i in range(3)], r2, w, color="#1baf7a", label="m=2 (two summands)")
ax2.bar([i + w / 2 for i in range(3)], r3, w, color="#eb6834", label="m=3 (2+1 split)")
ax2.set_yscale("log")
ax2.set_xticks(range(3)); ax2.set_xticklabels([f"d={d}" for d in ds])
ax2.set_ylabel("relations used before the stop (mean)")
ax2.set_title("relations needed", fontsize=10)
ax2.legend(frameon=False, fontsize=8, loc="upper right")
fig.tight_layout()
fig.savefig(HERE / "m3_vs_m2.svg")
fig.savefig(HERE / "m3_vs_m2.png", dpi=150)
