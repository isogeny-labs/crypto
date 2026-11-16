"""Render recorded pair-collision counts; this script computes no research result."""

import json
from pathlib import Path

import matplotlib.pyplot as plt

HERE = Path(__file__).resolve().parent
records = [json.loads((HERE / f"{cell}.json").read_text()) for cell in ("k13", "k19", "k23")]
cells = [record["cell"].upper() for record in records]
fig, axes = plt.subplots(1, 2, figsize=(9.6, 3.5), layout="constrained")
colors = ["#4778ad", "#40a795", "#d29448"]

for axis, key, title in zip(
    axes,
    ("duplicate_pairs", "independent_rank"),
    ("Repeated pair-sum keys", "Independent homogeneous rows"),
):
    values = [record[key] for record in records]
    bars = axis.bar(cells, values, color=colors, width=0.58)
    axis.bar_label(bars, padding=3)
    axis.set_title(title)
    axis.set_ylabel("Count")
    axis.set_ylim(0, max(values) * 1.17 + 1)
    axis.spines[["top", "right"]].set_visible(False)
    axis.grid(axis="y", alpha=0.2)
    axis.set_axisbelow(True)

fig.suptitle("Fixed pair-table census · run-20261116T0711Z · one exact pass per cell")
for suffix in ("svg", "png"):
    fig.savefig(HERE / f"collision-rank.{suffix}", dpi=150)

svg_path = HERE / "collision-rank.svg"
svg_path.write_text("\n".join(line.rstrip() for line in svg_path.read_text().splitlines()) + "\n")
