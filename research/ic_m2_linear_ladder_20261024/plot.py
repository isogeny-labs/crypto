#!/usr/bin/env python3
"""Lay out the recorded ladder values (fit.json) as SVG + 150 dpi PNG.
Plots recorded values and the fitted lines fit.json states; computes nothing."""
import json
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

HERE = Path(__file__).resolve().parent
C = ["#2a78d6", "#eb6834", "#1baf7a", "#eda100", "#e87ba4", "#008300", "#6250d6", "#e34948"]
INK, INK2, GRID = "#0b0b0b", "#52514e", "#e4e3df"
plt.rcParams.update({"font.size": 10, "axes.edgecolor": INK2, "axes.labelcolor": INK,
                     "xtick.color": INK2, "ytick.color": INK2, "axes.spines.top": False,
                     "axes.spines.right": False, "axes.grid": True, "grid.color": GRID,
                     "grid.linewidth": 0.6, "figure.facecolor": "#fcfcfb", "axes.facecolor": "#fcfcfb"})
fit = json.loads((HERE / "fit.json").read_text())
rungs = [r for r in fit["rungs"] if float(r["cofactor"]) <= 4]


def save(fig, name):
    fig.tight_layout()
    fig.savefig(HERE / f"{name}.svg")
    fig.savefig(HERE / f"{name}.png", dpi=150)
    plt.close(fig)


# 1. Work against n.
fig, ax = plt.subplots(figsize=(7.4, 4.4))
import math
for key, label, color, marker in [("ic_trials", "linear m=2 IC: decomposition trials per solve", C[1], "D"),
                                  ("rho_group_additions", "strong rho: group additions per solve", C[0], "o")]:
    for r in rungs:
        ys = r[key]
        ax.scatter([r["n"]] * len(ys), ys, s=10, color=color, alpha=0.35, linewidths=0)
    ax.scatter([r["n"] for r in rungs], [sum(r[key]) / len(r[key]) for r in rungs], s=46, color=color,
               marker=marker, edgecolor="white", linewidth=1.2, label=label, zorder=4)
    f = fit["fit_small_cofactor"][key]
    xs = [12, 42]
    ax.plot(xs, [2 ** (f["intercept"] + f["slope_log2_per_n"] * x) for x in xs], color=color, lw=1.4, ls="--")
    ax.text(42.3, 2 ** (f["intercept"] + f["slope_log2_per_n"] * 42), f"slope {f['slope_log2_per_n']:.3f}",
            color=color, fontsize=9, va="center")
f4 = fit["f4_reference"]
ax.scatter([p["n"] for p in f4["points"]], [p["median_word_xors"] for p in f4["points"]], s=46, color=C[6], marker="s",
           edgecolor="white", linewidth=1.2, label="F4 algebraic arm (committed): solver word XORs per solve", zorder=4)
ax.text(24.0, f4["points"][-1]["median_word_xors"] * 0.55, f"F4 slope {f4['slope_log2_word_xors_per_n']:.3f}", color=C[6], fontsize=9, va="center")
ax.set_yscale("log", base=2)
ax.set_xlabel("field degree n (Koblitz curves, cofactor ≤ 4)")
ax.set_ylabel("work per one-target solve (log₂ scale)")
ax.set_title("Algebraic IC work against n: linear m=2 vs the F4 arm, and rho", fontsize=11, color=INK, loc="left")
ax.set_xlim(11, 47)
ax.legend(frameon=False, fontsize=8.3, loc="lower right")
save(fig, "work_vs_n")

# 2. Wall-time distance to rho.
fig, ax = plt.subplots(figsize=(7.4, 3.9))
ax.scatter([r["n"] for r in rungs], [r["ic_over_rho_wall"] for r in rungs], s=52, color=C[1], marker="D",
           edgecolor="white", linewidth=1.2, zorder=4)
for r in rungs:
    dy = 8 if r["a"] == 1 else -12
    ax.annotate(f"a={r['a']}: {r['ic_over_rho_wall']:,.0f}×", (r["n"], r["ic_over_rho_wall"]), textcoords="offset points",
                xytext=(7, dy), fontsize=8.5, color=INK2)
ax.axhline(1, color=INK2, lw=1.2, ls="--")
ax.text(12, 1.25, "parity with rho", fontsize=9, color=INK2)
ax.set_yscale("log")
ax.set_xlabel("field degree n")
ax.set_ylabel("mean IC wall ÷ mean rho wall")
ax.set_title("Distance to same-target strong rho (same process, same field code, one thread)",
             fontsize=11, color=INK, loc="left")
save(fig, "distance_to_rho")
