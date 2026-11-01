#!/usr/bin/env python3
"""Render recorded quartic mixed-pattern counters; no research computation."""
from __future__ import annotations

import json
from pathlib import Path

import matplotlib.pyplot as plt

ROOT = Path(__file__).resolve().parent
FILES = [
    "quartic_twist_mixed_polynomial_ic_p37_v3.json",
    "quartic_twist_mixed_polynomial_ic_k12_p37_v1.json",
    "quartic_twist_mixed_polynomial_ic_k1_p37_v1.json",
]
ROWS = [json.loads((ROOT / p).read_text()) for p in FILES]
RHO = ROWS[0]["rho"]
assert all(r["rho"]["fp_muls"] == RHO["fp_muls"] for r in ROWS)
assert all(r["rho"]["group_ops"] == RHO["group_ops"] for r in ROWS)

fig, axes = plt.subplots(1, 2, figsize=(11.2, 4.4), layout="constrained")
labels = ["k=1,2,3", "k=1,2", "k=1", "twist24 rho"]
values = [r["counted_ic_fp_muls"] for r in ROWS] + [RHO["fp_muls"]]
axes[0].bar(labels, values, color=["#31688e", "#35b779", "#fde725", "#482878"])
axes[0].set_yscale("log")
axes[0].set_ylabel("Counted Fp multiplications (log scale)")
axes[0].set_title("One public p=37 target, seed 0")
for i, v in enumerate(values):
    axes[0].text(i, v * 1.08, f"{v:,}", ha="center", va="bottom", fontsize=7, rotation=22)
values = [r["counted_ic_group_ops"] for r in ROWS] + [RHO["group_ops"]]
axes[1].bar(labels, values, color=["#31688e", "#35b779", "#fde725", "#482878"])
axes[1].set_ylabel("Counted group operations")
axes[1].set_title("Separate unit; no GAE conversion")
for i, v in enumerate(values):
    axes[1].text(i, v + 120, f"{v:,}", ha="center", va="bottom", fontsize=8)
for ax in axes:
    ax.tick_params(axis="x", labelrotation=23)
    ax.grid(axis="y", alpha=.2)
fig.suptitle("Quartic mixed-pattern polynomial IC: counted path and same-process rho\n"
             "Native hash/allocator CPU and calibrated GAE absent; not an S or wall comparison", fontsize=11)
fig.savefig(ROOT / "mixed_polynomial_cost.svg")
fig.savefig(ROOT / "mixed_polynomial_cost.png", dpi=150)
plt.close(fig)

PILOTS = [json.loads((ROOT / f"quartic_twist_tagged_s5_k{k}_p37_v{2 if k == 2 else 1}.json").read_text())
          for k in range(5)]
fig, axes = plt.subplots(1, 2, figsize=(10.8, 3.7), layout="constrained")
expected = [2, 2, 1, 0, 0]
found = [sum(len(r["polynomial_signed_relations"]) for r in p["rows"]) for p in PILOTS]
xs = list(range(5))
axes[0].bar([x - .16 for x in xs], expected, width=.32, label="exact oracle", color="#31688e")
axes[0].bar([x + .16 for x in xs], found, width=.32, label="polynomial", color="#35b779")
axes[0].set_xticks(xs, [f"k={x}" for x in xs])
axes[0].set_ylabel("Complete signed relations, first 4 residuals")
axes[0].legend()
work = [sum(r["internal_fp_muls"] for r in p["rows"]) for p in PILOTS]
axes[1].bar(xs, work, color="#482878")
axes[1].set_yscale("log")
axes[1].set_xticks(xs, [f"k={x}" for x in xs])
axes[1].set_ylabel("Internal Fp M over 4 residuals (log scale)")
fig.suptitle("Frozen p=37 pilot: patterns have different solution sets and algebraic systems\n"
             "k=0 missed one; k=4 had a failed system despite zero oracle hits", fontsize=11)
for ax in axes:
    ax.grid(axis="y", alpha=.2)
fig.savefig(ROOT / "mixed_polynomial_pilot.svg")
fig.savefig(ROOT / "mixed_polynomial_pilot.png", dpi=150)
plt.close(fig)
