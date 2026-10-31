#!/usr/bin/env python3
"""Render the exact orbit-distinct equal-column control from its audited CSV."""

import csv
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt
import numpy as np


ROOT = Path(__file__).resolve().parent
with (ROOT / "gls_equal_columns_v2.csv").open(newline="") as stream:
    rows = [row for row in csv.DictReader(stream) if row["flat_policy"] == "distinct"]
assert [row["p"] for row in rows] == ["101", "211", "307"]

x = np.arange(3)
target_counts = [int(row["nonzero_targets"]) for row in rows]
flat_coverage = [100 * int(row["flat_covered"]) / target_counts[i] for i, row in enumerate(rows)]
fold_coverage = [100 * int(row["fold_covered"]) / target_counts[i] for i, row in enumerate(rows)]
cost_ratio = [float(row["f2_mul_ratio"]) for row in rows]

plt.rcParams.update({
    "font.family": "DejaVu Sans",
    "font.size": 9,
    "svg.hashsalt": "gls-equal-columns-v2",
})
fig, axes = plt.subplots(1, 2, figsize=(8.6, 3.35), layout="constrained")
blue, orange = "#286aa0", "#be6345"

ax = axes[0]
ax.bar(x - 0.17, flat_coverage, 0.34, label="Flat: 12 orbit reps", color=orange)
ax.bar(x + 0.17, fold_coverage, 0.34, label="Folded: 24 points", color=blue)
ax.set_ylim(0, 110)
ax.set_xticks(x, [f"p={row['p']}" for row in rows])
ax.set_ylabel("Nonzero targets with a relation (%)")
ax.set_title("Exact signed width-three coverage")
ax.grid(axis="y", color="#dedede", linewidth=0.7)
ax.set_axisbelow(True)
ax.legend(loc="upper center", bbox_to_anchor=(0.5, 0.83), fontsize=8, frameon=False)
for xx, yy in zip(x - 0.17, flat_coverage):
    ax.text(xx, yy + 1.5, f"{yy:.1f}", ha="center", va="bottom", fontsize=8)
for xx, yy in zip(x + 0.17, fold_coverage):
    ax.text(xx, yy + 1.5, f"{yy:.1f}", ha="center", va="bottom", fontsize=8)

ax = axes[1]
ax.axhline(1, color="#333333", linewidth=0.9)
ax.bar(x, cost_ratio, 0.55, color=[orange if value < 1 else blue for value in cost_ratio])
ax.set_ylim(0, 2.35)
ax.set_xticks(x, [f"p={row['p']}" for row in rows])
ax.set_title("Cold $F_{p^2}$ count: flat / folded")
ax.grid(axis="y", color="#dedede", linewidth=0.7)
ax.set_axisbelow(True)
for xx, yy in zip(x, cost_ratio):
    ax.text(xx, yy + 0.055, f"{yy:.3f}", ha="center", va="bottom", fontsize=8)

fig.suptitle("Same 12 unknown logs; orbit completion helps only the sparse cell", fontsize=11)
fig.savefig(ROOT / "gls_equal_columns_v2.svg", bbox_inches="tight", metadata={"Date": None})
fig.savefig(ROOT / "gls_equal_columns_v2.png", dpi=150, bbox_inches="tight")
plt.close(fig)
