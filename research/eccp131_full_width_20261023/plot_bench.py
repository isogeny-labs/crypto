#!/usr/bin/env python3
"""Render the retained nine-cell stage benchmark without smoothing."""
import json
from pathlib import Path

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

HERE = Path(__file__).resolve().parent
data = json.loads((HERE / "runs/bench-audit.json").read_text())
fig, ax = plt.subplots(figsize=(7.0, 3.0))
arms = ["small_a", "small_b", "tower"]
colors = {"small_a": "#3274a1", "small_b": "#59a14f", "tower": "#e15759"}
for i, arm in enumerate(arms):
    rows = [r for r in data["rows"] if r["arm"] == arm]
    ax.scatter([i - .10, i, i + .10], [r["screen_seconds"] for r in rows],
               s=55, color=colors[arm], edgecolor="black", linewidth=.45, zorder=3)
    median = data["summaries"][arm]["screen_seconds"]["median"]
    ax.hlines(median, i - .25, i + .25, color=colors[arm], linewidth=2.1)
ax.set_xticks(range(3), ["small A", "small A repeat", "certified tower"])
ax.set_ylabel("Screen wall time (s)")
ax.set_ylim(1.4, 3.15)
ax.grid(axis="y", alpha=.25)
ax.set_title("ECCp-131: 115 columns, 256 setup + 256 target candidates")
fig.text(.5, .005, "Dots: fixed repetitions 0, 1, 2; bars: median. All arms: 270,564 EC adds and zero natural rows.",
         ha="center", va="bottom", fontsize=8)
fig.tight_layout(rect=(0,.075,1,1))
fig.savefig(HERE / "bench_screen.svg")
