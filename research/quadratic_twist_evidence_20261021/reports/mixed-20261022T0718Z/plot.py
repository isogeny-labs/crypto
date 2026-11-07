"""Document layout only: plot native-recorded counts; no research arithmetic."""
import json
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt

here = Path(__file__).resolve().parent
receipt = json.loads((here / "../../native/mixed-20261022T0718Z/audit.json").read_text())
data = receipt["by_length"]
fig, ax = plt.subplots(figsize=(9.1, 3.4), layout="constrained")
width = 0.24
for offset, key, label, color in [
    (-width, "candidate_words", "Candidate words", "#64748b"),
    (0, "accepted_words", "Actual zero words", "#2563eb"),
    (width, "mixed_accepted_words", "Mixed zero words", "#0d9488"),
]:
    values = [row[key] for row in data]
    positions = [i + offset for i in range(len(data))]
    ax.bar(positions, values, width=width, label=label, color=color)
    for pos, value in zip(positions, values):
        ax.text(pos, value * 1.3 if value else 130, f"{value:,}",
                ha="center", va="bottom", fontsize=7.4, rotation=35)
ax.set_yscale("log")
ax.set_ylim(100, 300_000_000)
ax.set_xticks(range(len(data)), [str(row["length"]) for row in data])
ax.set_xlabel("Exact word length (ordered signed words, repetition allowed)")
ax.set_ylabel("Word count (log scale)")
ax.set_title("942 models / 274 subgroup pairs: finite relation census", fontsize=12)
ax.grid(axis="y", alpha=0.2)
ax.set_axisbelow(True)
ax.legend(loc="upper left", fontsize=8, frameon=False)
fig.suptitle("mixed-20261022T0718Z · independent native receipt · exact counts, no sampling error",
             fontsize=8, y=1.04)
for ext in ["svg", "pdf", "png"]:
    fig.savefig(here / f"word-counts.{ext}", dpi=150, bbox_inches="tight")
