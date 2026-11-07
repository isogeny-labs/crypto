#!/usr/bin/env python3
"""Instruction-audit figure for research/ic_acct1_cover_bitop_20261021.

Plots recorded values only: `audit/rows/*.audit.json`, written by
`examples/jv_cover_audit_ir.rs` from Callgrind runs of
`examples/jv_cover_ic_vs_rho_bitops.rs --callgrind 1` (and `--count 0` for
the control).  Computes no result beyond dividing recorded Ir by recorded
gates, as the audit rows already do.

    /tmp/report-venv/bin/python research/ic_acct1_cover_bitop_20261021/audit/figures/plots.py
"""
import json
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

HERE = Path(__file__).resolve().parent
ROWS = HERE.parent / "rows"
BLUE, ORANGE, GREEN, INK, MUTED = "#2a78d6", "#eb6834", "#2f9e6e", "#333333", "#6b6b6b"
plt.rcParams.update({"font.size": 8.5, "axes.spines.top": False, "axes.spines.right": False,
                     "axes.grid": True, "grid.color": "#e4e4e4", "grid.linewidth": 0.6,
                     "svg.fonttype": "none"})

ic = json.loads((ROWS / "ic_503.audit.json").read_text())
ctrl_path = ROWS / "control_ic_503.audit.json"
ctrl = json.loads(ctrl_path.read_text()) if ctrl_path.exists() else None
rho = {p: json.loads((ROWS / f"rho_{p}.audit.json").read_text()) for p in (251, 503)}

order = ["relations", "descent", "linear_algebra", "walk", "model", "factor_base",
         "verification", "transport"]
by = {ph["phase"]: ph for ph in ic["callgrind"]["phases"]}
cby = {ph["phase"]: ph for ph in ctrl["callgrind"]["phases"]} if ctrl else {}

fig, ax = plt.subplots(figsize=(7.4, 3.6))
x = range(len(order))
w = 0.38 if ctrl else 0.6
ax.bar([i - (w / 2 if ctrl else 0) for i in x], [by[p]["ir_per_kilo_bitop"] for p in order], w,
       color=BLUE, label="IC p = 503, counting on (as measured)")
if ctrl:
    ax.bar([i + w / 2 for i in x], [cby[p]["ir_per_kilo_bitop"] for p in order], w, color=GREEN,
           label="IC p = 503, counting off (control; gates from the counted run)")
for p, ls in [(503, "-"), (251, "--")]:
    v = rho[p]["callgrind"]["ir_per_kilo_bitop"]
    ax.axhline(v, color=ORANGE, linestyle=ls, linewidth=1.2, label=f"rho R3 p = {p}: {v:.1f}")
    ax.axhline(2 * v, color=ORANGE, linestyle=ls, linewidth=0.6, alpha=0.5)
ax.set_xticks(list(x), [p.replace("_", " ") for p in order], fontsize=7.5, rotation=20)
ax.set_yscale("log")
ax.set_ylabel("Callgrind Ir per 1000 bitop/v1 gates")
ax.set_title("ic_acct1 instruction audit at the pilot size (code: main + this branch).\n"
             "Thin orange lines: 2x rho, the admission threshold", fontsize=8.5)
ax.legend(frameon=False, fontsize=7, loc="upper right")
fig.tight_layout()
fig.savefig(HERE / "ir_per_gate.svg")
fig.savefig(HERE / "ir_per_gate.png", dpi=150)
