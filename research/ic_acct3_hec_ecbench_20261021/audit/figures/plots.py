#!/usr/bin/env python3
"""Instruction-audit figure for research/ic_acct3_hec_ecbench_20261021.

Plots recorded values only: `audit/audit-rows.json`, written from
`ecbench audit-ir` (Callgrind Ir of each re-executed record and its
bitop/v1 total).  Computes no result.

    /tmp/report-venv/bin/python research/ic_acct3_hec_ecbench_20261021/audit/figures/plots.py
"""
import json
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

HERE = Path(__file__).resolve().parent
ROWS = json.loads((HERE.parent / "audit-rows.json").read_text())
SUMMARY = json.loads((HERE.parent.parent / "AUDIT_IR.json").read_text())
BLUE, ORANGE, INK, MUTED = "#2a78d6", "#eb6834", "#333333", "#6b6b6b"
plt.rcParams.update({"font.size": 9, "axes.spines.top": False, "axes.spines.right": False,
                     "axes.grid": True, "grid.color": "#e4e4e4", "grid.linewidth": 0.6,
                     "svg.fonttype": "none"})

fig, ax = plt.subplots(figsize=(6.4, 3.2))
arms = [("hec.ic_fb_walk", BLUE, "IC (hec.ic_fb_walk), 5 runs"),
        ("hec.rho_full_width", ORANGE, "rho (hec.rho_full_width), 40 runs")]
for i, (m, c, label) in enumerate(arms):
    rs = [r for r in ROWS if r["method"] == m]
    ys = [r["ir_per_kilo_bitop"] for r in rs]
    xs = [i + (k - len(ys) / 2) * 0.012 for k in range(len(ys))]
    ax.scatter(xs, ys, s=14, color=c, label=label, zorder=3)
    pooled = next(a for a in SUMMARY["arms"] if a["method"] == m)["ir_per_kilo_bitop_pooled"]
    ax.hlines(pooled, i - 0.3, i + 0.3, color=c, linewidth=1.5)
    ax.annotate(f"pooled {pooled:,.0f}", (i + 0.32, pooled), va="center", fontsize=8, color=INK)
ax.set_xticks([0, 1], ["IC", "rho"])
ax.set_xlim(-0.6, 1.9)
ax.set_ylim(0, 1600)
ax.set_ylabel("Callgrind Ir per 1000 bitop/v1 gates")
ax.set_title("ic_acct3 g2-p41: instructions per counted gate, every run re-executed\n"
             f"45/45 reproduce their recorded bitop/v1 count; IC/rho = {SUMMARY['ic_over_rho_ir_per_gate']:.3f}",
             fontsize=9)
ax.legend(loc="lower right", frameon=False, fontsize=8)
fig.tight_layout()
fig.savefig(HERE / "ir_per_gate.svg")
fig.savefig(HERE / "ir_per_gate.png", dpi=150)
