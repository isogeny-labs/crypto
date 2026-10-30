"""Document layout only: plot values already computed and recorded by Rust.

This renderer computes no experimental metric, comparison, identity or result.
"""
import json
from pathlib import Path
import sys
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

root = Path(sys.argv[1])
data = json.loads((root / "summary.json").read_text())["plot"]
fig, ax = plt.subplots(figsize=(11, 4.8), constrained_layout=True)
for name, label, color in [("baseline", "Weights 1,1,1", "#46546b"),
                            ("weight2", "Weights 1,1,2", "#007b83"),
                            ("weight4", "Weights 1,1,4", "#c56132")]:
    ax.plot(data["labels"], data[name], "o-", label=label, color=color)
ax.set_ylabel("Recorded pair tests per verified relation")
ax.set_xlabel("Base prime / construction seed; extension degree 3")
ax.set_title("Matched native stage diagnostic; not an end-to-end speedup")
ax.tick_params(axis="x", rotation=55, labelsize=8)
ax.spines[["top", "right"]].set_visible(False)
ax.grid(axis="y", alpha=.2)
ax.legend(frameon=False)
fig.savefig(root / "counts.svg")
fig.savefig(root / "counts.png", dpi=150)
