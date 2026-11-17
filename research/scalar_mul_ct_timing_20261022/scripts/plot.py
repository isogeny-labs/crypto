#!/usr/bin/env python3
"""Lay out summary.json (written by examples/scalar_mul_timing_analyze.rs) as
a figure; every plotted number is already in summary.json.

    plot.py <study_dir> [summary.json] [fig_timing]

Writes <fig>.svg and a 150 dpi PNG of it into <study_dir>; run 2
(PROTOCOL.md, amendment 1) is `summary-run2.json fig_timing_run2`.
"""
import json
import sys
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

# Reference palette (dataviz skill, references/palette.md), light mode.
SURFACE, INK, INK2, GRID = "#fcfcfb", "#0b0b0b", "#52514e", "#e4e3df"
ARMS = (("A", "before (main)", "#2a78d6", "o", "-"),
        ("A2", "before, A/A control", "#8fb8ea", "s", ":"),
        ("B", "after (fix)", "#eb6834", "D", "-"))

plt.rcParams.update({
    "figure.facecolor": SURFACE, "axes.facecolor": SURFACE, "savefig.facecolor": SURFACE,
    "text.color": INK, "axes.labelcolor": INK, "xtick.color": INK2, "ytick.color": INK2,
    "axes.edgecolor": INK2, "axes.grid": True, "grid.color": GRID, "grid.linewidth": 0.6,
    "font.size": 9, "axes.titlesize": 10, "legend.frameon": False, "svg.fonttype": "none",
})

d = Path(sys.argv[1])
summary = sys.argv[2] if len(sys.argv) > 2 else "summary.json"
stem = sys.argv[3] if len(sys.argv) > 3 else "fig_timing"
run = "run 2" if "run2" in summary else "run 1"
s = json.loads((d / summary).read_text())
curves = s["curves"]

fig, axes = plt.subplots(1, len(curves), figsize=(5 * len(curves), 4), sharey=True)
for ax, c in zip(axes, curves):
    for arm, label, color, marker, ls in ARMS:
        cls = c["profile"][arm]["classes"]
        z = [k["leading_zeros"] for k in cls]
        med = [k["relative_median"] for k in cls]
        lo = [k["relative_median"] - k["relative_min"] for k in cls]
        hi = [k["relative_max"] - k["relative_median"] for k in cls]
        ax.errorbar(z, med, yerr=[lo, hi], color=color, marker=marker, ms=5, lw=1.6, ls=ls,
                    capsize=2, label=f"{label} ({c['profile'][arm]['rounds']} rounds)")
    ax.axhline(1.0, color=INK2, lw=0.7, ls="--")
    ax.set_xlabel(f"leading zero bits of the secret scalar (order: {c['order_bits']} bits)")
    ax.set_title(f"{c['curve']}: full-length cost after/before {c['cost_median']:.2f}×")
axes[0].set_ylabel("time ÷ time of full-length scalars\n(median over rounds; bars: round range)")
axes[0].legend(fontsize=8, loc="lower left")
fig.suptitle(f"scalar_mul_secret on the generic affine ladder ({run}): time against scalar length, "
             f"before and after (A/A band δ = {s['delta']:.3f})", fontsize=9)
fig.tight_layout()
fig.savefig(d / f"{stem}.svg")
fig.savefig(d / f"{stem}.png", dpi=150)
print("figure written to", d)
