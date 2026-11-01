"""Lay out recorded independent-auditor counts; no research arithmetic."""
import json
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

here = Path(__file__).resolve().parent
data = json.loads((here / "coverage-data.json").read_text())
fig, axes = plt.subplots(1, 2, figsize=(10.0, 3.6), constrained_layout=True)
for ax, key, title, color in zip(
    axes, ["zero_images", "nontrivial_images"],
    ["Cofactor-kernel images: pairing = 1", "Nonzero twist images: target order r"],
    ["#83919d", "#127b85"],
):
    ax.bar(data["labels"], data[key], color=color, width=.6)
    ax.bar_label(ax.containers[0], padding=3)
    ax.set_title(title, fontsize=10)
    ax.set_xlabel("Shared prime subgroup order r")
    ax.set_ylabel("Auxiliary points (count)")
    ax.set_ylim(0, 2200)
    ax.spines[["top", "right"]].set_visible(False)
    ax.grid(axis="y", alpha=.18)
    ax.set_axisbelow(True)
fig.suptitle("Twist-assisted pairing sweep · pairing-20261024T0442Z", fontsize=12)
fig.savefig(here / "auxiliaries.svg")
fig.savefig(here / "auxiliaries.pdf")
fig.savefig(here / "auxiliaries.png", dpi=150)
