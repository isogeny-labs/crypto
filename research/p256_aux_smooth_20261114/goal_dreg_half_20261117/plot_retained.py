#!/usr/bin/env python3
"""Layout only: plot recorded native certificate and input-manifest values."""
import json
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

root = Path(__file__).resolve().parent
membership = json.loads((root / "evidence/retained-native/summary.json").read_text())["cells"]
systems = json.loads((root / "evidence/retained-bridge/manifest.json").read_text())["cells"]
fig, axes = plt.subplots(1, 2, figsize=(10, 4))
for arm, offset, color in [("dense", -0.18, "#5a7894"), ("structured", 0.18, "#33816a")]:
    rows = [next(c for c in membership if c["cell"] == f"depth{d}-signed-{arm}") for d in [2, 4, 5]]
    bars = axes[0].bar([x + offset for x in range(3)], [c["ideal_regularity"] for c in rows],
                       width=0.36, color=color, label=arm)
    axes[0].bar_label(bars, padding=2)
for arm, offset, color in [("RB-dense", -0.18, "#5a7894"), ("RB-chain", 0.18, "#33816a")]:
    rows = [next(c for c in systems if c["cell"] == f"depth{d}-m7-{arm}-planted") for d in [4, 5]]
    bars = axes[1].bar([x + offset for x in range(2)], [c["residual_variables"] for c in rows],
                       width=0.36, color=color, label=arm)
    axes[1].bar_label(bars, padding=2)
axes[0].set(xticks=[0, 1, 2], xticklabels=["d2 / 4 points", "d4 / 18 points", "d5 / 42 points"],
            ylabel="Membership ideal-module regularity", title="Exact retained-point membership")
axes[1].set(xticks=[0, 1], xticklabels=["d4, m=7", "d5, m=7"],
            ylabel="Residual variables", title="Complete-system input cost")
for ax in axes:
    ax.legend(fontsize=8)
    ax.grid(axis="y", alpha=0.2)
    ax.set_axisbelow(True)
    ax.margins(y=0.15)
fig.suptitle("Actual P-256 field — matched bases, full relation regularity still unmeasured")
fig.text(0.5, 0.02, "One fixed base per depth; native exact component certificates and deterministic planted input metadata.\n"
         "No statistical uncertainty or timing data; unsigned. Membership degrees do not certify complete-system degrees.",
         ha="center", fontsize=8)
fig.tight_layout(rect=[0, 0.12, 1, 0.94])
for suffix in ["svg", "pdf"]:
    fig.savefig(root / f"retained-comparison.{suffix}")
svg = root / "retained-comparison.svg"
svg.write_text("\n".join(line.rstrip() for line in svg.read_text().splitlines()) + "\n")
