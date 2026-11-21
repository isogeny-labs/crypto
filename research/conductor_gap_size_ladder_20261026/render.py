#!/usr/bin/env python3
"""Render native parameter certificates/planning scenarios; no result arithmetic."""
import json
from pathlib import Path

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.ticker import PercentFormatter

ROOT = Path(__file__).resolve().parent
data = json.loads((ROOT / "results-v2/design.json").read_text())
plt.rcParams.update({"font.size": 10, "axes.spines.top": False, "axes.spines.right": False})


def save(fig, name):
    svg = ROOT / "figures" / f"{name}.svg"
    fig.savefig(svg, bbox_inches="tight")
    svg.write_text("\n".join(line.rstrip() for line in svg.read_text().splitlines()) + "\n")
    fig.savefig(ROOT / "figures" / f"{name}.png", dpi=150, bbox_inches="tight")
    fig.savefig(ROOT / "figures" / f"{name}.pdf", bbox_inches="tight")
    plt.close(fig)


fig, ax = plt.subplots(figsize=(8, 4.7))
size_labels = [40, 47, 54, 61, 112, 160, 256]
positions = {value: i for i, value in enumerate(size_labels)}
for row in data["size_ranges"]:
    upper = row["field_bits"] > 61
    color = "#a46521" if upper else "#175d83"
    y = positions[row["field_bits"]]
    ax.plot([int(row["min_gap"]), int(row["max_gap"])], [y, y], color=color, lw=4)
    ax.scatter([int(row["min_gap"]), int(row["max_gap"])], [y, y], color=color, s=26)
ax.axvline(125000000302579, color="#738294", ls="--", lw=1, label="Earlier 96-bit certified floor gap")
ax.plot([], [], color="#175d83", lw=4, label="Native-size parameter classes; solves pending")
ax.plot([], [], color="#a46521", lw=4, label="Upper parameter classes; models/maps pending")
ax.set_xscale("log")
ax.set_xlabel("Certified prime conductor-gap parameter (logarithmic axis)")
ax.set_ylabel("Field size (bits; categorical rows)")
ax.set_yticks(list(positions.values()), size_labels)
ax.legend(frameon=False, fontsize=8, loc="lower right")
ax.set_title("Verified parameter range; constructions and solves pending")
fig.tight_layout()
save(fig, "parameter-range")

fig, ax = plt.subplots(figsize=(8, 3.6))
for classes, color in [(16, "#8c919c"), (32, "#175d83"), (64, "#267063")]:
    rows = [s for s in data["power_scenarios"]
            if s["classes_per_rung"] == classes
            and s["targets_per_crater"] == 1000
            and s["true_ratio_assumption"] == 1.0111251967610384]
    ax.plot([s["variance_inflation"] for s in rows],
            [s["joint_power_union_lower_bound"] for s in rows],
            "o-", label=f"{classes} classes/rung", color=color)
ax.axhline(.9, color="#a46521", ls="--", lw=1, label="Proposed 90% joint-power target")
ax.set_xticks([1, 2, 4], ["Old fitted variance", "2× variance", "4× variance"])
ax.set_ylim(0, 1.04)
ax.yaxis.set_major_formatter(PercentFormatter(1))
ax.set_ylabel("Projected joint-power lower bound")
ax.set_title("Planning sensitivity — not measured power at larger sizes")
ax.legend(frameon=False, fontsize=8, loc="lower left")
fig.text(.5, .01, "Four rungs; 1,000 targets/crater; two seeds; assumed ratio 1.01112520; 99.75% intervals.", ha="center", fontsize=8)
fig.tight_layout(rect=(0, .07, 1, 1))
save(fig, "planning-sensitivity")
