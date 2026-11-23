#!/usr/bin/env python3
"""Render recorded medians only; no sampling or retiming."""
import json
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

HERE = Path(__file__).resolve().parent
DATA = json.loads((HERE.parent.parent / "RESULTS.json").read_text())
assert DATA["run_id"] == "run-20261116T141419Z"
ARMS = ("hash", "hash-aa", "flat-direct-u32", "paged", "inline")
COLORS = ("#164a72", "#79a6c7", "#c85d34", "#78a06b", "#926aa6")
fig, axes = plt.subplots(1, 3, figsize=(12.8, 4.1), layout="constrained")
for ax, cell in zip(axes, DATA["cells"], strict=True):
    values = [cell["arms"][arm]["combined_median_ns"] for arm in ARMS]
    assert all(cell["arms"][arm]["repetitions"] == 101 for arm in ARMS)
    bars = ax.bar(range(5), values, color=COLORS, width=0.72)
    ax.set_title(f"K{cell['n']} · {cell['first_keys']:,} retained keys")
    ax.set_xticks(range(5), ["hash", "A/A", "flat", "paged", "inline"], rotation=35, ha="right")
    ax.set_ylim(0, max(values) * 1.20)
    ax.grid(axis="y", color="#dddddd", linewidth=0.7)
    ax.set_axisbelow(True)
    ax.set_ylabel("Median build + lookup (ns)")
    for bar, val in zip(bars, values, strict=True):
        ax.annotate(f"{val:,}", (bar.get_x() + bar.get_width()/2, bar.get_height()),
                    xytext=(0, 3), textcoords="offset points", ha="center", fontsize=7)
fig.suptitle("Pair-index component timings · run-20261116T141419Z · 101 repetitions per arm", fontsize=12)
fig.text(0.5, -0.03, "Each panel has its own nanosecond scale. Fixed precomputed pair and probe keys; shared L0 host.",
         ha="center", fontsize=9)
fig.savefig(HERE / "chart.svg", bbox_inches="tight")
svg = HERE / "chart.svg"
svg.write_text("\n".join(line.rstrip() for line in svg.read_text().splitlines()) + "\n")
plt.close(fig)
