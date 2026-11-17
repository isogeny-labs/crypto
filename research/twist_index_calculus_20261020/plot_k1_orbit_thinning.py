#!/usr/bin/env python3
"""Plot the preregistered ideal-oracle orbit-thinning grid."""
from __future__ import annotations

import json
from pathlib import Path

import matplotlib.pyplot as plt

ROOT = Path(__file__).resolve().parent
runs = json.loads((ROOT / "K1_ORBIT_THINNING_v1.json").read_text())["runs"]
fig, axes = plt.subplots(2, 2, figsize=(10.2, 5.8), layout="constrained")
colors = ["#31688e", "#35b779", "#d95f02"]
for col, p in enumerate((73, 97)):
    for seed in range(3):
        recs = [r for r in runs if r["p"] == p and r["seed"] == seed]
        recs.sort(key=lambda r: r["orbit_count"])
        x = [r["orbit_count"] for r in recs]
        axes[0, col].plot(x, [r["attempted_s4_systems_to_stop"] for r in recs],
                          "o-", color=colors[seed], label=f"seed {seed}")
        axes[1, col].plot(x, [r["processed_queries"] for r in recs],
                          "o-", color=colors[seed])
    axes[0, col].set_title(f"p={p}; all 3 seeds reach full rank")
    axes[0, col].set_ylabel("Exact S4 peel attempts to rank")
    axes[1, col].set_ylabel("Residuals to rank")
    axes[1, col].set_xlabel("Complete L2 abscissa orbits retained")
    for row in range(2):
        axes[row, col].set_xticks([r["orbit_count"] for r in runs
                                 if r["p"] == p and r["seed"] == 0])
        axes[row, col].grid(alpha=.2)
axes[0, 0].legend(fontsize=8)
fig.suptitle("Ideal exact-oracle orbit thinning on frozen one-target streams", fontsize=11)
fig.savefig(ROOT / "k1_orbit_thinning.svg")
fig.savefig(ROOT / "k1_orbit_thinning.png", dpi=170)
plt.close(fig)
