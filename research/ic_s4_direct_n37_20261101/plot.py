#!/usr/bin/env python3
"""Lay out recorded values (xl_estimate.csv, the F4 word-XOR medians from the merged ladder) as SVG + PNG;
plotting only, no analysis."""
import csv
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

HERE = Path(__file__).resolve().parent
plt.rcParams.update({"font.size": 10, "axes.edgecolor": "#52514e", "axes.spines.top": False,
                     "axes.spines.right": False, "axes.grid": True, "grid.color": "#e4e3df",
                     "figure.facecolor": "#fcfcfb", "axes.facecolor": "#fcfcfb"})
rows = list(csv.DictReader(open(HERE / "xl_estimate.csv")))
n = [int(r["n"]) for r in rows]
fig, ax = plt.subplots(figsize=(6.8, 4.3))
ax.plot(n, [float(r["log2_monomials"]) for r in rows], "o-", color="#eb6834",
        label="direct S4, XL heuristic: monomials (matrix columns)")
ax.plot(n, [float(r["log2_m2_floor"]) for r in rows], "s--", color="#2a78d6",
        label="m=2 trials, 2^(n-d+1) (d = (n+1)/3)")
ax.plot(n, [float(r["log2_rho"]) for r in rows], "^:", color="#1baf7a", label="strong rho scale, 2^(n/2)")
# measured: inherited F4 solver word XORs (median, merged ecbench ladder, n = 13, 19, 23; timed out at 31)
f4n, f4x = [13, 19, 23], [6.01e6, 5.42e8, 1.06e10]
import math
ax.plot(f4n, [math.log2(v) for v in f4x], "D", color="#52514e", label="measured: F4 solver word XORs (n <= 23)")
ax.axvline(37, color="#bdbcb6", lw=1)
ax.text(37.5, 4, "n = 37", fontsize=8.5, color="#52514e")
ax.set_xlabel("n")
ax.set_ylabel("log2 of cost (different units, shown for scale)")
ax.legend(frameon=False, fontsize=8, loc="upper left")
fig.tight_layout()
fig.savefig(HERE / "s4_direct_scaling.svg")
fig.savefig(HERE / "s4_direct_scaling.png", dpi=150)
