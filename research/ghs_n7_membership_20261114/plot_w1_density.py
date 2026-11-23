"""Render the recorded W1 coefficient counts and the derived lower bounds.

This file only plots values already in SIDON_BOUND.md and raw run records.
It does not compute finite-field research results.
"""

from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np


Q = (2, 4, 8, 16)
TRACE_ZERO = (63, 4095, 262143, 16777215)
UNION = (56, 3906, 237839, 15110970)
ONE_CUBIC = (42, 2898, 182938, 11490570)
LOWER = (14, 1535, 115128, 7875807)

base = Path(__file__).resolve().parent
x = np.arange(len(Q))
fig, ax = plt.subplots(figsize=(9.2, 4.7))
fig.patch.set_facecolor("white")
ax.bar(x - 0.24, np.array(LOWER) / TRACE_ZERO, width=0.24,
       color="#90b4c7", label="Derived one-cubic lower bound")
ax.bar(x, np.array(ONE_CUBIC) / TRACE_ZERO, width=0.24,
       color="#286f9b", label="Measured one cubic")
ax.bar(x + 0.24, np.array(UNION) / TRACE_ZERO, width=0.24,
       color="#dc9650", label="Measured two-cubic union")
ax.set_xticks(x, [f"q={q}" for q in Q])
ax.set_ylim(0, 1.08)
ax.set_ylabel("Share of nonzero trace-zero coefficients")
ax.set_title("Published genus-14 W1 locus: bound and finite counts")
ax.grid(axis="y", alpha=0.2)
ax.set_axisbelow(True)
ax.legend(frameon=False, ncol=3, loc="upper center", bbox_to_anchor=(0.5, -0.13))
fig.text(0.5, 0.01,
         "Runs 20261115T0614Z, T0616Z, T0619Z; one deterministic count per q; no interval",
         ha="center", fontsize=8, color="#555")
fig.tight_layout(rect=(0, 0.09, 1, 1))
fig.savefig(base / "w1-density.svg")
fig.savefig(base / "w1-density.png", dpi=150)
svg_path = base / "w1-density.svg"
svg_path.write_text("\n".join(line.rstrip() for line in svg_path.read_text().splitlines()) + "\n")
