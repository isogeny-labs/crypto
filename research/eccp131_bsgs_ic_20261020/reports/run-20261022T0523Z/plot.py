"""Plot only the four frozen raw ECCp-131 arity-arm measurements."""

import json
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt


root = Path(__file__).parent
rows = json.loads((root / "raw" / "manifest.json").read_text())["arms"]
fig, axes = plt.subplots(1, 2, figsize=(8.2, 3.0), sharex=True)
for seed, color in ((131, "#23648e"), (137, "#d27828")):
    paired = [next(r for r in rows if r["name"] == f"seed{seed}-arity{a}") for a in (2, 3)]
    axes[0].plot([2, 3], [r["cold_wall_seconds"] for r in paired], marker="o", color=color, label=f"seed {seed}")
    axes[1].plot([2, 3], [r["probe"]["elapsed_ms"] / 1000 for r in paired], marker="o", color=color)
for ax, title in zip(axes, ("Cold process wall", "Exact relation probe")):
    ax.set_title(title)
    ax.set_xlabel("Maximum signed base points per relation")
    ax.set_ylabel("Seconds")
    ax.set_xticks([2, 3])
    ax.set_ylim(bottom=0)
    ax.grid(axis="y", alpha=0.25)
axes[0].legend(frameon=False, loc="upper left")
fig.suptitle("ECCp-131, B=128; 32 setup + 16 target candidates per process (n=2 seeds/arm)")
fig.tight_layout()
fig.savefig(root / "arity_cost.svg")
fig.savefig(root / "arity_cost.png", dpi=180)
