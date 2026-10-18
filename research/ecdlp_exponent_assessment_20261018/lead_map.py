#!/usr/bin/env python3
"""lead_map.svg: the leads of LEADS.md by effort and level.

Each row is one lead; the bar is the effort estimate stated in LEADS.md
(person-weeks, log scale), the colour is the level in the bounds protocol
the lead can move, and a hatched bar is blocked on a resource or a
decision. Values are in lead_map.diagram.json. Run from the repository
root with the research-visuals venv:

    report-venv/bin/python research/ecdlp_exponent_assessment_20261018/lead_map.py
"""
import json
import os

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

HERE = os.path.dirname(os.path.abspath(__file__))
INK, INK2, MUTED, GRID = "#0b0b0b", "#52514e", "#8a8984", "#e6e5e1"
LEVEL_COLOR = {
    "exponent": "#2a78d6",
    "constant": "#eb6834",
    "accounting": "#1baf7a",
    "machine": "#6250d6",
}

# (id, label, tier, level, effort_weeks, blocked, probability note)
LEADS = [
    ("L1", "closure degree of the Koblitz chain (m = 4, ℓ = 5, 6)", 1, "exponent", 2.5, False, "measured: degree 4 up to m·ℓ = 24"),
    ("L2", "half-dimension invariant base, implicit past n ≈ 40", 1, "exponent", 2.0, False, "measured: 17× rho at n = 23"),
    ("L3", "S₅ small roots at δ ≈ 1/5, uncovered lattice variants", 1, "exponent", 1.0, False, "closed (this PR)"),
    ("L4", "PKM tower m = 3, N = 21 solving degree", 1, "exponent", 0.5, True, "D ≥ 7 at 4.3 h (#2175); needs -t 8 or 192 GB"),
    ("L5", "genus-3 plane-quartic cover IC: scaling vs rho", 1, "exponent", 3.0, False, "measured ℓ^2.5: not an IC (→ N4)"),
    ("L6", "register F_{p⁴} / genus-3 domains; four-size challenge", 2, "accounting", 1.0, False, "prerequisite of §4.2"),
    ("L7", "τ-slots in production; family-optimum base sizing", 2, "constant", 1.5, False, "up to log₂ m! bits"),
    ("L8", "multi-target vs the S·T² line (PR #2163, K2 charge)", 2, "accounting", 0.5, False, "in flight"),
    ("L9", "a better rho reference (trail truncation, co-Z steps)", 2, "constant", 1.0, False, "moves every × reference"),
    ("L10", "gate-first F6-IC, RREF root defaults, wide-path switch", 2, "machine", 0.4, False, "exact on 4 cells"),
    ("L11", "block structure in Gröbner; several invariant bases", 2, "constant", 0.4, False, "measurable now"),
    ("L12", "ECC2K-130 PDP by a different relation producer", 3, "exponent", 8.0, True, "search power"),
    ("L13", "a method that sees the conductor", 3, "constant", 4.0, True, "low prior; #1888 queued"),
    ("L14", "Galois-invariant base from another algebraic group", 3, "exponent", 4.0, True, "theory; no compute"),
    ("L15", "learned structure (M2b, M3, RELN NA-1)", 3, "constant", 6.0, False, "all cost labels null"),
    ("M1", "IC exponent with a base that scales", 4, "accounting", 0.2, False, "afternoon"),
    ("M2", "re-pin calibration; F4 word-XOR drift; n83 phase counts", 4, "accounting", 1.0, False, "evidence floor"),
    ("M3", "zero-compute ledger items (Assumption 1, S₆ top form)", 4, "accounting", 0.1, False, "zero compute"),
    ("M4", "external Gröbner baselines; root-finder ≥ 2× gate", 4, "machine", 1.0, False, "T17"),
    ("N1", "m = 2 relation graph: does a cycle suffice? (α 2/3 vs 4/7)", 5, "exponent", 0.5, False, "2–3 days; reading, not a win"),
    ("N2", "Frobenius-fold the m = 2 pair table; optimum dimension", 5, "constant", 0.3, False, "≈2–3× on L2's rows"),
    ("N3", "m = 2 on near-prime Koblitz curves (n = 17, 19, 23, 41)", 5, "accounting", 0.2, False, "removes the cofactor confound"),
    ("N4", "relation solve for the plane-quartic cover IC", 5, "exponent", 1.5, False, "tests q^{4/3} for real"),
    ("N5", "curve-names --diff and ecbench report agree on slugs", 5, "machine", 0.1, False, "hours"),
]
TIERS = {1: "1 · where an exponent can still move", 2: "2 · constants on measured rows",
         3: "3 · blocked or long", 4: "4 · measurement and tooling",
         5: "5 · second pass (evening): opened by today's measurements"}

fig_h = 0.36 * len(LEADS) + 0.5 * len(TIERS) + 1.9
fig, ax = plt.subplots(figsize=(10.5, fig_h))
y = 0
ticks, labels = [], []
for t, title in TIERS.items():
    ax.text(0.055, y, title, ha="left", va="center", fontsize=9.5, color=INK, fontweight="bold")
    y += 1
    for lid, label, tier, level, eff, blocked, note in LEADS:
        if tier != t:
            continue
        c = LEVEL_COLOR[level]
        ax.barh(y, eff, left=0.05, height=0.62, color=c if not blocked else "white",
                edgecolor=c, linewidth=1.4, hatch="////" if blocked else None, zorder=2)
        ax.text((0.05 + eff) * 1.08, y, f"{eff:g} wk · {note}", va="center", fontsize=8, color=INK2)
        ticks.append(y)
        labels.append(f"{lid}  {label}")
        y += 1
    y += 0.4
ax.set_xscale("log")
ax.set_xlim(0.05, 40)
ax.set_xticks([0.1, 0.25, 0.5, 1, 2, 4, 8])
ax.set_xticklabels(["0.1", "0.25", "0.5", "1", "2", "4", "8"], fontsize=8.5, color=INK2)
ax.set_xlabel("effort estimate, person-weeks (log scale); bar colour = level the lead can move; hatched = blocked",
              fontsize=9, color=INK2)
ax.set_yticks(ticks)
ax.set_yticklabels(labels, fontsize=8.6, color=INK)
ax.invert_yaxis()
ax.grid(axis="x", color=GRID, lw=0.8)
for sp in ("top", "right", "left"):
    ax.spines[sp].set_visible(False)
ax.spines["bottom"].set_color(INK2)
ax.tick_params(axis="y", length=0)
fig.text(0.02, 0.995, "Further ECDLP leads by effort and level (LEADS.md, 2026-10-18, with the evening pass)", ha="left", va="top",
         fontsize=11.5, color=INK, fontweight="bold")
handles = [plt.Rectangle((0, 0), 1, 1, color=LEVEL_COLOR[k]) for k in LEVEL_COLOR]
fig.legend(handles, [f"{k}: {v}" for k, v in {
    "exponent": "could change α (low probability, decisive test)",
    "constant": "moves S at the same α",
    "accounting": "changes what is charged or how it is read",
    "machine": "wall time or primitive weights only",
}.items()], loc="upper left", bbox_to_anchor=(0.02, 0.975), ncol=2, fontsize=8, frameon=False)
fig.text(0.01, 0.004, "Effort figures are the estimates stated beside each lead in LEADS.md, not measurements. "
         "Ranking within a tier is by expected movement per unit of work.", fontsize=7.2, color=MUTED, va="bottom")
fig.tight_layout(rect=(0, 0.03, 1, 0.91))
fig.savefig(os.path.join(HERE, "lead_map.svg"), format="svg")
fig.savefig(os.path.join(HERE, "lead_map.png"), dpi=150)
json.dump({"schema": "lead_map/v1",
           "leads": [dict(id=a, label=b, tier=c, level=d, effort_weeks=e, blocked=f, note=g) for a, b, c, d, e, f, g in LEADS]},
          open(os.path.join(HERE, "lead_map.diagram.json"), "w"), indent=1)
print(f"wrote lead_map.svg/.png/.diagram.json with {len(LEADS)} leads")
