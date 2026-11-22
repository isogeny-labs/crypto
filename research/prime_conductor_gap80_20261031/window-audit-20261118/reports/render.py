"""Lay out already-recorded metadata only; no curve/count computation."""
import json
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.ticker import ScalarFormatter

here = Path(__file__).resolve().parent
records = [
    json.loads((here.parent / "results" / name).read_text())
    for name in ("target80-b6000000.json", "target80-zero-leftover-window.json")
]
fig, axes = plt.subplots(1, 2, figsize=(9, 4.2))
labels = ["Default window", "Explicit window"]
colors = ["#c08d41", "#477ba3"]
for ax, key, title in zip(
    axes,
    ("leftover_pairs", "rectangles"),
    ("Direct leftover iterations", "Rectangles at block 6,000,000"),
):
    values = [int(record[key]) for record in records]
    ax.bar(labels, values, color=colors, width=0.58)
    ax.set_title(title, fontsize=11)
    ax.set_ylabel("Derived count")
    ax.set_ylim(0, max(values) * 1.24)
    ax.yaxis.set_major_formatter(ScalarFormatter(useMathText=True))
    ax.ticklabel_format(axis="y", style="sci", scilimits=(0, 0))
    for x, value in enumerate(values):
        ax.annotate(f"{value:,}", (x, value), xytext=(0, 6),
                    textcoords="offset points", ha="center", fontsize=9)
    ax.spines[["right", "top"]].set_visible(False)
fig.suptitle("Same 80-bit target: source-derived schedules", fontsize=14)
fig.text(0.5, 0.02,
         "No target run or runtime estimate. The explicit choice removes the tail; rectangles slightly increase.",
         ha="center", fontsize=9)
fig.tight_layout(rect=(0, 0.08, 1, 0.93))
fig.savefig(here / "schedules.svg", bbox_inches="tight")
svg = here / "schedules.svg"
svg.write_text("\n".join(line.rstrip() for line in svg.read_text().splitlines()) + "\n")
