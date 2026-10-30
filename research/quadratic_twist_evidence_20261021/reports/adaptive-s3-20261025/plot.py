"""Layout of already audited coverage-data.json values. No research arithmetic."""
import json
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.patches import Patch

matplotlib.rcParams["pdf.use14corefonts"] = True
here = Path(__file__).resolve().parent
data = json.loads((here / "coverage-data.json").read_text())
rows = data["fixture"]
labels = [r["label"].replace(" ", "\n") for r in rows]
xs = list(range(len(rows)))
fig, axes = plt.subplots(1, 2, figsize=(10.4, 3.7))
colors = ["#355c7d", "#457b9d", "#a1592b", "#d98930"]
for row, x, color in zip(rows, xs, colors):
    axes[0].bar(x - 0.17, row["matrix_degree"], 0.32, color=color)
    axes[0].bar(x + 0.17, row["expanded_peak"], 0.32, color=color, alpha=0.38,
                edgecolor=color, hatch="//")
    axes[1].bar(x, row["useful_rows"], 0.60, color=color)
axes[0].set_xticks(xs, labels)
axes[0].set_ylabel("Polynomial degree")
axes[0].set_title("Named adaptive solver")
axes[0].set_ylim(0, 7)
axes[0].legend(handles=[Patch(facecolor="#536678", label="matrix degree"),
                        Patch(facecolor="#536678", alpha=0.38, hatch="//",
                              label="expanded peak")], frameon=False, fontsize=8, loc="upper right")
axes[1].set_xticks(xs, labels)
axes[1].set_ylabel("Independent useful rows")
axes[1].set_title("Actual signed point relations")
axes[1].set_ylim(0, 3)
axes[1].set_yticks([0, 1, 2])
fig.suptitle("F43 / order 11, first target; adaptive-s3-20261025", fontsize=11)
fig.text(0.5, 0.015, "98 cases and 1,384 target comparisons audited; bars show one declared fixture", ha="center", fontsize=8)
fig.tight_layout(rect=(0, 0.045, 1, 0.95))
fig.savefig(here / "degrees.pdf")
svg = here / "degrees.svg"
fig.savefig(svg)
svg.write_text("\n".join(line.rstrip() for line in svg.read_text().splitlines()) + "\n")
plt.close(fig)
