#!/usr/bin/env python3
"""Layout only: plot recorded native certificate fields; no algebra/solver."""
import json
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

root = Path(__file__).resolve().parent
data = json.loads((root / "evidence/native/depth4.json").read_text())
labels = ["Full fibre", "Retained base", "Tower"]
keys = ["full_fibre_membership_CI", "retained_membership_CI", "tower_membership_CI"]
curve_keys = ["full_fibre_with_target_cubic_CI", "retained_with_target_cubic_CI", "tower_with_target_cubic_CI"]
fig, axes = plt.subplots(1, 2, figsize=(9.4, 3.8), sharey=True)
for ax, title, fields in zip(axes, ["Membership only", "Membership + target cubic"], [keys, curve_keys]):
    values = [data[k]["ideal_module_regularity"] for k in fields]
    bars = ax.bar(labels, values, color=["#587796", "#b37e31", "#33816a"])
    ax.bar_label(bars, padding=3)
    ax.set_title(title)
    ax.set_ylim(0, 21)
    ax.grid(axis="y", alpha=0.2)
    ax.set_axisbelow(True)
axes[0].set_ylabel("Ideal-module regularity D")
fig.suptitle("P-256 depth-4 component certificates — full relation D unmeasured")
fig.text(0.5, 0.03, "FB1h2704fe49495e; one existing fixture; exact CI derivation, unsigned.\n"
         "Full/retained target ideals differ over extensions. No statistical uncertainty or timing data.",
         ha="center", fontsize=8)
fig.tight_layout(rect=[0, 0.12, 1, 0.94])
for suffix in ["svg", "pdf"]:
    fig.savefig(root / f"components.{suffix}")
