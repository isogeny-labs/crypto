#!/usr/bin/env python3
"""Draw requested-backing arithmetic from storage.json; no measurements."""

import json
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt


HERE = Path(__file__).resolve().parent
rows = json.loads((HERE / "storage.json").read_text())["rows"]
degrees = [f"K{r['field_degree']}" for r in rows]
flat = [r["flat_requested_backing_bytes"] / (1024 * 1024) for r in rows]
paged = [r["paged_requested_backing_bytes"] / (1024 * 1024) for r in rows]

fig, ax = plt.subplots(figsize=(8, 4.2))
fig.patch.set_facecolor("#f8fafc")
ax.set_facecolor("#f8fafc")
x = range(len(rows))
width = 0.34
left = ax.bar([i - width / 2 for i in x], flat, width, color="#d95f02", label="Flat direct array")
right = ax.bar([i + width / 2 for i in x], paged, width, color="#148f8c", label="Paged candidate")
ax.set_yscale("log", base=2)
ax.set_ylim(0.025, 128)
ax.set_xticks(list(x), degrees)
ax.set_ylabel("Requested backing (MiB; log₂ scale)")
ax.set_title("Source-derived backing request for three fixed bases")
ax.legend(loc="upper left", frameon=False)
ax.grid(axis="y", color="#d1d5db", alpha=0.7)
ax.set_axisbelow(True)
for bars, values in ((left, flat), (right, paged)):
    for bar, value in zip(bars, values):
        ax.annotate(f"{value:.3f}", (bar.get_x() + bar.get_width() / 2, value),
                    xytext=(0, 4), textcoords="offset points", ha="center", fontsize=8)
fig.text(0.5, 0.005, "Derived bytes only · 3 fixed census cells · no RSS or time measurements",
         ha="center", fontsize=8, color="#475569")
fig.tight_layout(rect=(0, 0.035, 1, 1))
fig.savefig(HERE / "requested_backing.svg", format="svg")
