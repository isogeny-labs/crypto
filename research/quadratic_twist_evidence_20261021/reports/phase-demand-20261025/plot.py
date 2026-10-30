"""Plot recorded native request counts; no research arithmetic or result fitting."""
import json
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

matplotlib.rcParams["pdf.use14corefonts"] = True
here = Path(__file__).resolve().parent
data = json.loads((here / "coverage-data.json").read_text())
fig, axes = plt.subplots(1, 3, figsize=(10.6, 3.1))
for ax, key, label in zip(axes, ["calls", "slots", "adds"],
                          ["Solver calls", "Represented matrix slots", "Point-add requests"]):
    for arm, color in [("main", "#355c7d"), ("twist", "#ba642c")]:
        ax.plot([r["n"] for r in data[arm]], [r[key] for r in data[arm]],
                marker="o", markersize=3, label=arm, color=color)
    ax.set_xlabel("Separate source target nP")
    ax.set_ylabel(label)
    ax.set_xticks([1, 3, 5, 7, 9, 10])
    ax.grid(alpha=0.2)
axes[0].legend(frameon=False, fontsize=8)
fig.suptitle("F43 / r=11 / prefix policy: setup + one target + source verification", fontsize=10)
fig.text(0.5, 0.012, "phase-demand-20261025; exact request units differ across panels; no timing or scalar cost", ha="center", fontsize=8)
fig.tight_layout(rect=(0, 0.055, 1, 0.94))
fig.savefig(here / "demands.pdf")
svg = here / "demands.svg"
fig.savefig(svg)
svg.write_text("\n".join(line.rstrip() for line in svg.read_text().splitlines()) + "\n")
plt.close(fig)
