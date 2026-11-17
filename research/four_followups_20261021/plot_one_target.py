"""Render the recorded 28-bit single-target 36-cell price grid."""
import csv
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.colors import LogNorm

ROOT = Path(__file__).resolve().parent
GRID = ROOT / "results/pencil-pilot/f48383-one-target-price-grid.csv"
rows = list(csv.DictReader(GRID.open()))
assert len(rows) == 36
squares = [0.0, 0.5, 1.0]
inversions = [1.0, 10.0, 100.0, 1000.0]
natives = [0.0, 0.1, 1.0]
values = {(float(r["native_GAE_per_logical"]), float(r["S_per_M"]),
           float(r["I_per_M"])): float(r["pencil_over_rho"]) for r in rows}
fig, axes = plt.subplots(1, 3, figsize=(11.5, 4.2), sharey=True)
scale = LogNorm(vmin=min(values.values()), vmax=max(values.values()))
for ax, native in zip(axes, natives):
    matrix = [[values[(native, square, inversion)] for inversion in inversions]
              for square in squares]
    im = ax.imshow(matrix, cmap="viridis", norm=scale, aspect="auto")
    ax.set_xticks(range(4), [str(int(x)) for x in inversions])
    ax.set_yticks(range(3), [str(x) for x in squares])
    ax.set_xlabel("Inversion cost / M")
    ax.set_title(f"Native logical = {native:g} GAE")
    for i, line in enumerate(matrix):
        for j, value in enumerate(line):
            ax.text(j, i, f"{value:,.0f}×" if value >= 100 else f"{value:.1f}×",
                    ha="center", va="center", fontsize=8,
                    color="white" if scale(value) < 0.55 else "black")
axes[0].set_ylabel("Squaring cost / M")
fig.suptitle("Pencil / matched rho: one 28-bit target, full cold setup charged", fontsize=12)
fig.subplots_adjust(left=0.07, right=0.87, bottom=0.15, top=0.82, wspace=0.18)
colorbar_axis = fig.add_axes([0.9, 0.2, 0.015, 0.56])
fig.colorbar(im, cax=colorbar_axis, label="Ratio (logarithmic colour scale)")
for ext in ("svg", "png"):
    fig.savefig(ROOT / f"pencil-one-target-grid.{ext}", dpi=180, bbox_inches="tight")
