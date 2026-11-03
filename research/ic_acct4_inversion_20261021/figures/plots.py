#!/usr/bin/env python3
"""Figures for research/ic_acct4_inversion_20261021. Plots recorded values only.

Reads analysis/analysis.json (examples/ic_acct4_analyse.rs). Computes nothing.

    /tmp/report-venv/bin/python research/ic_acct4_inversion_20261021/figures/plots.py
"""
import json
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

HERE = Path(__file__).resolve().parent
A = json.loads((HERE.parent / "analysis" / "analysis.json").read_text())
IC, RHO, GREY = "#eb6834", "#2a78d6", "#888888"
plt.rcParams.update({"font.size": 9, "axes.spines.top": False, "axes.spines.right": False,
                     "axes.grid": True, "grid.color": "#e4e4e4", "grid.linewidth": 0.6})


def save(fig, name):
    fig.tight_layout()
    fig.savefig(HERE / f"{name}.svg")
    fig.savefig(HERE / f"{name}.png", dpi=150)
    plt.close(fig)


# 1. IC/rho instructions per counted gate: recorded before, measured after.
rows = A["instructions_per_gate"]
labels = [f"genus {r['genus']}, p = {r['p']}" for r in rows]
fig, ax = plt.subplots(figsize=(6.4, 3.2))
x = range(len(rows))
b1 = ax.bar([i - 0.18 for i in x], [r["ic_over_rho_recorded_before"] for r in rows], 0.34,
            color=GREY, label="before (recorded in hec_bitop_20261020)")
b2 = ax.bar([i + 0.18 for i in x], [r["ic_over_rho"] for r in rows], 0.34,
            color=IC, label="after: inversion and residue tests run as priced")
for bars in (b1, b2):
    for b in bars:
        ax.text(b.get_x() + b.get_width() / 2, b.get_height() + 0.02, f"{b.get_height():.2f}",
                ha="center", fontsize=8)
ax.axhline(1, color="#333", lw=1, ls="--")
ax.set_xticks(list(x), labels)
ax.set_ylabel("IC / rho, instructions per counted gate")
ax.set_ylim(0, 1.35)
ax.set_title("Coverage diagnostic: index calculus against rho (Callgrind)", fontsize=9)
ax.legend(fontsize=7, frameon=False, loc="upper left")
save(fig, "instr_per_gate")

# 2. IC/rho with and without operand re-validation (self-check policy).
alg = A["algorithm_totals"]
labels = [f"g{r['genus']}\np={r['p']}" for r in alg]
fig, ax = plt.subplots(figsize=(6.4, 3.2))
x = range(len(alg))
b1 = ax.bar([i - 0.18 for i in x], [r["ic_over_rho_total"] for r in alg], 0.34, color=GREY,
            label="total (re-checks included)")
b2 = ax.bar([i + 0.18 for i in x], [r["ic_over_rho_algorithm"] for r in alg], 0.34, color=IC,
            label="algorithm (operand re-checks excluded)")
for bars in (b1, b2):
    for b in bars:
        ax.text(b.get_x() + b.get_width() / 2, b.get_height() + 0.008, f"{b.get_height():.3f}",
                ha="center", fontsize=7)
ax.set_xticks(list(x), labels)
ax.set_ylabel("IC / rho, mean bitop/v1 per solve")
ax.set_ylim(0, 0.5)
ax.set_title("Self-check policy: IC/rho with and without operand re-validation", fontsize=9)
ax.legend(fontsize=7, frameon=False, loc="upper left")
save(fig, "self_check_policy")
print("figures written")
