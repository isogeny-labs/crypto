#!/usr/bin/env python3
"""Plot counted one-target ratios and complete-stream exact occupancy."""
from __future__ import annotations

import json
from pathlib import Path

import matplotlib.pyplot as plt

ROOT = Path(__file__).resolve().parent
fig, axes = plt.subplots(1, 2, figsize=(10.5, 4.0), layout="constrained")
colors = ["#31688e", "#35b779", "#d95f02"]
for p in (73, 97):
    for seed in range(3):
        rec = json.loads((ROOT / f"quartic_twist_k1_scale_p{p}_seed{seed}_v1.json").read_text())
        cen = json.loads((ROOT / f"quartic_twist_k1_census_p{p}_seed{seed}_v1.json").read_text())
        x = p + (seed - 1) * 1.6
        ratio = rec["counted_ic_fp_muls"] / rec["rho"]["fp_muls"]
        axes[0].scatter(x, ratio, s=60, color=colors[seed], label=f"seed {seed}" if p == 73 else None)
        axes[0].annotate(str(seed), (x, ratio), xytext=(5, 2), textcoords="offset points", fontsize=8)
        axes[1].scatter(x, cen["total_k1_signed_relations"] / cen["queries"],
                        s=60, color=colors[seed])
        axes[1].scatter(x, cen["uniform_expected_per_query_numerator"] /
                        cen["uniform_expected_per_query_denominator"],
                        marker="x", s=65, linewidths=1.6, color=colors[seed])
axes[0].scatter(37, 116050118 / 348941, s=70, marker="D", color="#440154",
                label="p=37 control")
axes[0].set_yscale("log")
axes[0].set_ylabel("IC / matched twist24 rho (counted Fp M)")
axes[0].set_xlabel("Prime p")
axes[0].legend(fontsize=7)
axes[1].set_ylabel("Signed k=1 relations per residual")
axes[1].set_xlabel("Prime p")
axes[1].text(.03, .97, "circles: exact full-stream census\ncrosses: uniform first moment",
             transform=axes[1].transAxes, va="top", fontsize=8)
for ax in axes:
    ax.grid(alpha=.2)
    ax.set_xticks([37, 73, 97] if ax is axes[0] else [73, 97])
fig.suptitle("Mixed-line quartic twist: counted one-target IC and exact relation occupancy", fontsize=11)
fig.savefig(ROOT / "k1_scaling_overview.svg")
fig.savefig(ROOT / "k1_scaling_overview.png", dpi=170)
plt.close(fig)
