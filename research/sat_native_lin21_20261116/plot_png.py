#!/usr/bin/env python3
"""PRESENTATION ONLY. Renders fig_conflicts_vs_l.png from fig_data.tsv, which summary.rs wrote.
No number is computed here: medians, minima and maxima are read as printed by the Rust summary;
the three reference curves are the definitions 2^l, 4*2^l and C(2^l,2)."""
import sys, os
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

d = sys.argv[1] if len(sys.argv) > 1 else os.path.dirname(os.path.abspath(__file__))
ser = {}
for line in open(os.path.join(d, "fig_data.tsv")).read().splitlines()[1:]:
    a, l, m, lo, hi = line.split("\t")
    ser.setdefault(a, []).append((int(l), float(m), float(lo), float(hi)))
col = {"lin": "#1f77b4", "lin-nolex": "#7f7f7f", "lin21": "#d62728", "lin21-pu": "#ff7f0e", "scan": "#2ca02c"}
fig, ax = plt.subplots(figsize=(7.6, 5.0), dpi=150)
for a, pts in ser.items():
    xs = [p[0] for p in pts]
    ax.errorbar(xs, [p[1] for p in pts], yerr=[[p[1] - p[2] for p in pts], [p[3] - p[1] for p in pts]], color=col[a],
                marker="x" if a == "scan" else "o", ms=4, lw=1.3, capsize=2, ls="-." if a == "scan" else "-",
                label="plain 2+1 scan: leaves solved" if a == "scan" else f"{a}: conflicts (median, min-max)")
grid = sorted({p[0] for pts in ser.values() for p in pts})
ax.plot(grid, [2 ** l for l in grid], "k:", lw=1, label="2^l")
ax.plot(grid, [4 * 2 ** l for l in grid], color="k", ls=(0, (1, 4)), lw=1, label="4 * 2^l (rule bound)")
ax.plot(grid, [2 ** l * (2 ** l - 1) / 2 for l in grid], "k--", lw=1, label="C(2^l, 2)")
ax.set_yscale("log", base=2)
ax.set_xlabel("l = dim V   (n = 3l + 2)")
ax.set_ylabel("conflicts per non-decomposable target")
ax.set_title("3-summand PDP refutation in LinCdcl: chained pair encoding (lin) vs 2+1 leaf (lin21)", fontsize=9)
ax.grid(True, which="major", alpha=0.3)
ax.legend(fontsize=7, loc="upper left")
fig.tight_layout()
fig.savefig(os.path.join(d, "fig_conflicts_vs_l.png"))
