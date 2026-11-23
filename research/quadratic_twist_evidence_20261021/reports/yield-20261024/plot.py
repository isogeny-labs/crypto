"""Presentation of the recorded Lean-certified counts only."""
import json
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

p = Path(__file__).resolve().parent
d = json.loads((p / "coverage-data.json").read_text())
fig, ax = plt.subplots(figsize=(7.8, 3.4), constrained_layout=True)
ax.bar(d["labels"], d["raw_zero_counts"], color=["#657f91", "#0d7a85"], width=.55)
ax.bar_label(ax.containers[0], padding=4)
ax.set_ylim(0, 4)
ax.set_yticks([0, 1, 2, 3, 4])
ax.set_ylabel("Raw zero relations (count)")
ax.set_title("F5 control: 9 scalar candidates per point · yield-20261024")
ax.text(.5, .92, "All three twist rows are zero modulo 3", transform=ax.transAxes,
        ha="center", fontsize=10)
ax.spines[["top", "right"]].set_visible(False)
fig.savefig(p / "raw-yield.svg")
fig.savefig(p / "raw-yield.pdf")
fig.savefig(p / "raw-yield.png", dpi=150)
