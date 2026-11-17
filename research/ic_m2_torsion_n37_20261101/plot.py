#!/usr/bin/env python3
"""Lay out recorded per-seed trials (pairs.txt) as SVG + PNG; plotting only, no analysis."""
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

HERE = Path(__file__).resolve().parent
plt.rcParams.update({"font.size": 10, "axes.edgecolor": "#52514e", "axes.spines.top": False,
                     "axes.spines.right": False, "axes.grid": True, "grid.color": "#e4e3df",
                     "figure.facecolor": "#fcfcfb", "axes.facecolor": "#fcfcfb"})
rows = [l.split() for l in open(HERE / "pairs.txt")]
seeds = [r[0][-3:] for r in rows]
off = [int(r[1]) for r in rows]
on = [int(r[6]) for r in rows]
fig, ax = plt.subplots(figsize=(6.6, 4.0))
for i, (a, b) in enumerate(zip(off, on)):
    ax.plot([i, i], [a, b], color="#bdbcb6", lw=1.5, zorder=1)
ax.scatter(range(len(rows)), off, color="#2a78d6", label="torsion off (cycle stop)", zorder=2)
ax.scatter(range(len(rows)), on, color="#eb6834", label="torsion on", zorder=2)
ax.set_yscale("log")
ax.set_xticks(range(len(rows)))
ax.set_xticklabels(seeds)
ax.set_xlabel("seed (last three digits), a = 0, n = 37, cofactor 596")
ax.set_ylabel("decomposition trials to recover t")
ax.legend(frameon=False)
fig.tight_layout()
fig.savefig(HERE / "trials_off_vs_on.svg")
fig.savefig(HERE / "trials_off_vs_on.png", dpi=150)
