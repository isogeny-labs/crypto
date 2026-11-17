#!/usr/bin/env python3
"""Lay out recorded values (ratios.csv, runs.txt, merged d=12 pairs) as SVG + PNG; plotting only, no analysis."""
import csv
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

HERE = Path(__file__).resolve().parent
plt.rcParams.update({"font.size": 10, "axes.edgecolor": "#52514e", "axes.spines.top": False,
                     "axes.spines.right": False, "axes.grid": True, "grid.color": "#e4e3df",
                     "figure.facecolor": "#fcfcfb", "axes.facecolor": "#fcfcfb"})
rows = list(csv.DictReader(open(HERE / "ratios.csv")))
hk = {int(r["d"]): float(r["h_over_K"]) for r in rows}
fig, ax = plt.subplots(figsize=(6.8, 4.3))
# per-seed pairs
runs = [l.split() for l in open(HERE / "runs.txt")]
for d in (10, 8, 7):
    off = {r[2]: int(r[3]) for r in runs if int(r[0]) == d and r[1] == "0"}
    on = {r[2]: int(r[3]) for r in runs if int(r[0]) == d and r[1] == "1"}
    ax.plot([hk[d]] * len(off), [on[s] / off[s] for s in off], "_", color="#bdbcb6", ms=9, zorder=1)
ref = [l.split() for l in open(HERE.parent / "ic_m2_torsion_n37_20261101" / "pairs.txt")]
ax.plot([hk[12]] * len(ref), [int(r[6]) / int(r[1]) for r in ref], "_", color="#bdbcb6", ms=9, zorder=1,
        label="single pairs")
x = [float(r["h_over_K"]) for r in rows]
g = [float(r["geomean_on_over_off"]) for r in rows]
lo = [float(r["ci_lo"]) for r in rows]
hi = [float(r["ci_hi"]) for r in rows]
ax.errorbar(x, g, yerr=[[a - b for a, b in zip(g, lo)], [b - a for a, b in zip(g, hi)]], fmt="o",
            color="#2a78d6", capsize=4, zorder=2, label="paired geometric mean, 95% bootstrap")
ax.axhline(1, color="#52514e", lw=1, ls=":")
ax.axvline(8.41, color="#eb6834", lw=1.2, ls="--")
ax.text(8.41 * 1.06, 1.12, "N83 a=1: h/K about 8.4", color="#eb6834", fontsize=8.5, rotation=90, va="bottom")
for r in rows:
    ax.annotate(f"d={r['d']}", (float(r["h_over_K"]), float(r["geomean_on_over_off"])), textcoords="offset points",
                xytext=(6, -12), fontsize=8, color="#52514e")
ax.set_xscale("log"); ax.set_yscale("log")
ax.set_xlabel("cofactor / base size, h / K  (a = 0, n = 37, h = 596)")
ax.set_ylabel("trials ratio, torsion on / off")
ax.legend(frameon=False, fontsize=8, loc="upper left")
fig.tight_layout()
fig.savefig(HERE / "torsion_ratio_vs_hk.svg")
fig.savefig(HERE / "torsion_ratio_vs_hk.png", dpi=150)
