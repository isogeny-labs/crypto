#!/usr/bin/env python3
"""exponent_map.svg: the ECDLP cost exponent alpha per domain and method.

Every value is copied from a committed artefact named in REPORT.md section
1 (frontier bounds), section 3 (index-calculus rounds, heuristics) or read
from the companion sweep's `reports/*/scaling_fit.csv` (alpha +- 1.96 se).
Run from the repository root:

    report-venv/bin/python research/ecdlp_exponent_assessment_20261018/exponent_map.py

It writes exponent_map.svg, exponent_map.png (150 dpi) and
exponent_map.diagram.json (the plotted values) beside itself.
"""
import csv
import json
import math
import os

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(os.path.dirname(HERE))
SWEEP = os.path.join(ROOT, "research", "ecbench_comprehensive_sweep_20261018", "reports")

# Palette: the dataviz reference instance (light surface).
INK = "#0b0b0b"
INK2 = "#52514e"
MUTED = "#8a8984"
GRID = "#e6e5e1"
C_FRONTIER = "#2a78d6"  # measured and fitted, with interval (frontier bounds)
C_SWEEP = "#eb6834"  # this date's companion sweep (alpha +- 1.96 se)
C_FIT = "#1baf7a"  # fitted, interval not available
C_PAPER = "#6250d6"  # heuristic or closed on paper (no measurement)

rows = []  # (domain, label, alpha, lo, hi, class, source)


def add(domain, label, a, lo, hi, cls, src):
    rows.append(dict(domain=domain, label=label, alpha=a, lo=lo, hi=hi, cls=cls, source=src))


# Section 1: frontier bounds (docs/bounds/FRONTIER.md, ECFR1h8b45b672c283).
F = "frontier"
add("Koblitz, medium tier (2^32–2^47)", "rho.signed_frobenius_strong", 0.451, 0.399, 0.512, F, "ECBND1h05a2ea918f3c")
add("Koblitz, medium tier (2^32–2^47)", "claw.pair_table", 0.467, 0.423, 0.499, F, "ECBND1h25ac58612850")
add("Koblitz, medium tier (2^32–2^47)", "bsgs.negation", 0.503, 0.474, 0.531, F, "ECBND1he836821517fb")
add("Koblitz, medium tier (2^32–2^47)", "rho.negation (3 sizes)", 0.473, 0.125, 0.553, F, "ECBND1h83091af19aee")
add("Koblitz, medium tier (2^32–2^47)", "kangaroo.vow (3 sizes)", 0.486, -0.312, 0.571, F, "ECBND1h7f1d4d59799f")
add("Prime field, toy tier (2^17–2^24)", "ic.pipeline, |F| = 32, MITM", 0.841, 0.500, 1.024, F, "ECBND1hbab6d6dc37fc")
add("Prime field, toy tier (2^17–2^24)", "ic.pipeline, |F| = 32, subtract", 1.005, 0.786, 1.099, F, "ECBND1h9d3beb6852c6")

# Section 3.1: Koblitz index-calculus rounds (BOUNDARY_TARGETS_CHARGED_SUPPLEMENT_20261008.md).
add("Koblitz index calculus (fitted total α)", "round 1, n = 41 signed-orbit fold (10 rungs, R² 0.71)", 0.54, None, None, "fit", "BOUNDARY_TARGETS_CHARGED_SUPPLEMENT_20261008.md")
add("Koblitz index calculus (fitted total α)", "round 2, folded pair table (4 rungs, R² 0.82)", 0.43, None, None, "fit", "BOUNDARY_TARGETS_CHARGED_SUPPLEMENT_20261008.md")

# Section 2 and 3.2: on paper.
add("On paper", "generic lower bound, prime-field curves (Shoup)", 0.5, None, None, "paper", "REPORT.md §2")
add("On paper", "Gaudry k = 4 on E(F_{p^4}), Õ(p^{3/2}) = r^{3/8}", 0.375, None, None, "paper", "REPORT.md §3.2")

# Companion sweep of this date: alpha +- 1.96 se from ecbench report.
SWEEPS = [
    ("prime", "Prime field, this sweep (2^16–2^32)", None),
    ("koblitz", "Koblitz K_1, this sweep (2^15–2^37)", None),
    ("prime-ic", "Prime field, IC vs rho, this sweep (2^16–2^25)", {"ic-pipeline", "rho-frozen_reference"}),
]
for name, domain, keep in SWEEPS:
    p = os.path.join(SWEEP, name, "scaling_fit.csv")
    if not os.path.exists(p):
        continue
    for r in csv.DictReader(open(p)):
        if r["arm"].endswith("-aa"):
            continue
        if keep is not None and r["arm"] not in keep:
            continue
        a, se = float(r["alpha"]), float(r["se_alpha"])
        label = f'{r["method"]} ({r["sizes"]} sizes)'
        add(domain, label, a, a - 1.96 * se, a + 1.96 * se, "sweep", f"reports/{name}/scaling_fit.csv")

# ── draw ─────────────────────────────────────────────────────────────
domains = []
for r in rows:
    if r["domain"] not in domains:
        domains.append(r["domain"])

fig_h = 0.34 * len(rows) + 0.5 * len(domains) + 1.6
fig, ax = plt.subplots(figsize=(10.5, fig_h))
y = 0
yticks, ylabels, sep = [], [], []
colors = {"frontier": C_FRONTIER, "sweep": C_SWEEP, "fit": C_FIT, "paper": C_PAPER}
for d in domains:
    sep.append(y + 0.5)
    ax.text(-0.42, y, d, ha="left", va="center", fontsize=9.5, color=INK, fontweight="bold")
    y += 1
    for r in rows:
        if r["domain"] != d:
            continue
        c = colors[r["cls"]]
        if r["lo"] is not None:
            ax.plot([r["lo"], r["hi"]], [y, y], color=c, lw=2, solid_capstyle="round", zorder=2)
            ax.plot([r["lo"], r["lo"]], [y - 0.18, y + 0.18], color=c, lw=1.4, zorder=2)
            ax.plot([r["hi"], r["hi"]], [y - 0.18, y + 0.18], color=c, lw=1.4, zorder=2)
        marker = "o" if r["cls"] != "paper" else "D"
        ax.plot([r["alpha"]], [y], marker=marker, ms=7, color=c, mec="white", mew=1.2, zorder=3)
        txt = f'{r["alpha"]:.3f}' + (f'  [{r["lo"]:.3f}, {r["hi"]:.3f}]' if r["lo"] is not None else "")
        ax.text(1.20, y, txt, ha="left", va="center", fontsize=8.5, color=INK2, family="monospace")
        yticks.append(y)
        ylabels.append(r["label"])
        y += 1
    y += 0.4

ax.axvline(0.5, color=INK2, lw=1.2, ls=(0, (4, 3)), zorder=1)
ax.text(0.5, y - 0.85, "α = 1/2 (generic)", ha="center", va="top", fontsize=8.5, color=INK2)
for s in sep[1:]:
    ax.axhline(s - 0.9, color=GRID, lw=0.8)
ax.set_yticks(yticks)
ax.set_yticklabels(ylabels, fontsize=8.8, color=INK)
ax.invert_yaxis()
ax.set_xlim(-0.42, 1.52)
ax.set_xticks([-0.25, 0.0, 0.25, 0.375, 0.5, 0.75, 1.0])
ax.set_xticklabels(["−0.25", "0", "0.25", "3/8", "1/2", "0.75", "1.0"], fontsize=8.5, color=INK2)
ax.set_xlabel("fitted cost exponent α in r   (cost ∝ r^α; S = gae / √r is flat at α = 1/2)", fontsize=9, color=INK2)
ax.grid(axis="x", color=GRID, lw=0.8)
for sp in ("top", "right", "left"):
    ax.spines[sp].set_visible(False)
ax.spines["bottom"].set_color(INK2)
ax.tick_params(axis="y", length=0)
fig.text(0.02, 0.995, "Where the ECDLP exponent stands, by domain and method", ha="left", va="top", fontsize=11.5, color=INK, fontweight="bold")
handles = [
    plt.Line2D([], [], color=C_FRONTIER, marker="o", lw=2, label="frontier bound: fitted α with 95 % cluster-bootstrap interval"),
    plt.Line2D([], [], color=C_SWEEP, marker="o", lw=2, label="companion sweep (2026-10-18): OLS α ± 1.96 standard errors"),
    plt.Line2D([], [], color=C_FIT, marker="o", lw=0, label="fitted total α of a Koblitz index-calculus round, no interval published"),
    plt.Line2D([], [], color=C_PAPER, marker="D", lw=0, label="on paper: lower bound or heuristic, not measured"),
]
fig.legend(handles=handles, loc="upper left", bbox_to_anchor=(0.02, 0.985), ncol=1, fontsize=8, frameon=False)
fig.text(0.01, 0.004, "Sources: docs/bounds/FRONTIER.md (ECFR1h8b45b672c283), docs/ic/BOUNDARY_TARGETS_CHARGED_SUPPLEMENT_20261008.md,\n"
         "research/ecbench_comprehensive_sweep_20261018/reports/*/scaling_fit.csv. An interval that contains 1/2 does not establish an exponent move.",
         fontsize=7.2, color=MUTED, va="bottom")
fig.tight_layout(rect=(0, 0.03, 1, 0.90))
fig.savefig(os.path.join(HERE, "exponent_map.svg"), format="svg")
fig.savefig(os.path.join(HERE, "exponent_map.png"), dpi=150)
json.dump({"schema": "exponent_map/v1", "rows": rows}, open(os.path.join(HERE, "exponent_map.diagram.json"), "w"), indent=1)
print(f"wrote exponent_map.svg/.png/.diagram.json with {len(rows)} rows")
