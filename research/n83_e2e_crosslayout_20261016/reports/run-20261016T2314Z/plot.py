#!/usr/bin/env python3
"""Render the six retained observations; no solver or statistical estimation."""
import json
from pathlib import Path

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

HERE = Path(__file__).resolve().parent
rows = json.loads((HERE / "../../ic1_rows_38976053815.json").read_text())["rows"]
labels = [row["arm"].replace("root", " / root").replace("_", " ") for row in rows]
colors = ["#176b87" if "root11" in row["arm"] else "#b85b20" for row in rows]
plt.rcParams.update({"font.size": 10, "svg.fonttype": "none"})
fig, axes = plt.subplots(1, 2, figsize=(10.4, 4.1), layout="constrained", sharey=True)
for ax, key, scale, title, unit in [
    (axes[0], "cold_process_ms", 1000, "Complete fresh-process IC interval", "seconds"),
    (axes[1], "peak_rss_bytes", 1024 * 1024, "Measured peak resident memory", "MiB"),
]:
    values = [row[key] / scale for row in rows]
    ax.scatter(values, range(len(rows)), c=colors, s=58, zorder=3)
    for index, value in enumerate(values):
        ax.annotate(f"{value:.3f}" if key == "cold_process_ms" else f"{value:.1f}",
                    (value, index), xytext=(7, 7), textcoords="offset points", fontsize=9)
    ax.set_title(title, fontsize=11, pad=12)
    ax.set_xlabel(unit)
    ax.grid(axis="x", color="#d9e0e5", linewidth=0.7)
    ax.spines[["top", "right"]].set_visible(False)
axes[0].set_yticks(range(len(rows)), labels)
axes[0].invert_yaxis()
axes[0].set_xlim(116.5, 120.5)
axes[1].set_xlim(1800, 2750)
fig.suptitle("N83 exposed-point layout screen · run 38976053815", fontsize=14)
fig.savefig(HERE / "observations.svg")
fig.savefig(HERE / "observations.png", dpi=150)

