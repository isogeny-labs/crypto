#!/usr/bin/env python3
"""reach_graph.svg: the Coppersmith reach of S_{m+1} against what the IC needs.

For m = 2 … 5: the Jochemsz–May basic-strategy reach δ_t at shift levels
t = 1 … 6 (reach.py), its limit δ_∞ = 2/(m(m+1)d), the W10 family-wide
ceiling 1/(m·d) for any shift family of one polynomial, the measured
sublattice reach of sublattice_search.py (m = 2 only, 40-bit p), and the
reach the program needs, δ = 1/(m+1). Values in reach_graph.diagram.json.
"""
import json
import os
from fractions import Fraction

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

from reach import delta_inf, delta_t  # noqa: E402

HERE = os.path.dirname(os.path.abspath(__file__))
INK, INK2, MUTED, GRID = "#0b0b0b", "#52514e", "#8a8984", "#e6e5e1"
COL = {2: "#2a78d6", 3: "#eb6834", 4: "#1baf7a", 5: "#6250d6"}
MEASURED = {1: 0.0396, 2: 0.0865, 3: 0.1080}  # sublattice_search.py, full lattice = best sublattice
needed = {2: Fraction(1, 2), 3: Fraction(1, 4), 4: Fraction(1, 5), 5: Fraction(1, 6)}

fig, ax = plt.subplots(figsize=(9.5, 5.6))
data = {}
labels = []
for m in (2, 3, 4, 5):
    d = 2 ** (m - 1)
    ts = list(range(1, 7))
    ds = [float(delta_t(m, d, t)[0]) for t in ts]
    data[m] = dict(d=d, delta_t=ds, delta_inf=float(delta_inf(m, d)), ceiling=1 / (m * d), needed=float(needed[m]))
    ax.plot(ts, ds, "o-", color=COL[m], lw=1.6, ms=4, label=f"m = {m}: basic-strategy reach δ_t", zorder=3)
    ax.axhline(float(delta_inf(m, d)), color=COL[m], lw=0.9, ls=":", zorder=2)
    ax.axhline(1 / (m * d), color=COL[m], lw=0.9, ls="--", zorder=2)
    ax.axhline(float(needed[m]), color=COL[m], lw=2.2, alpha=0.25, zorder=1)
    labels.append((float(needed[m]), f"needed {needed[m]}", COL[m], 8))
    labels.append((1 / (m * d), f"W10 ceiling 1/{m * d}", COL[m], 7.5))
    labels.append((float(delta_inf(m, d)), f"δ_∞ = {delta_inf(m, d)}", COL[m], 7.5))
ax.plot(list(MEASURED), list(MEASURED.values()), "s", color=COL[2], mfc="white", ms=6, mew=1.5,
        label="m = 2: measured, full lattice = best sublattice (40-bit p)", zorder=4)
# right-hand labels, nudged apart in log space so coincident lines stay legible
import math  # noqa: E402
labels.sort()
placed = []
for y, txt, c, fs in labels:
    ly = math.log10(y)
    if placed and ly - placed[-1] < 0.085:
        ly = placed[-1] + 0.085
    placed.append(ly)
    ax.text(6.15, 10 ** ly, txt, va="center", fontsize=fs, color=c)
ax.set_yscale("log")
ax.set_ylim(1e-4, 0.8)
ax.set_xlim(0.8, 7.6)
ax.set_xticks(range(1, 7))
ax.set_xlabel("shift level t (lattice dimension (dt+1)^m)", fontsize=9, color=INK2)
ax.set_ylabel("reach δ: unknowns |x_i| ≤ p^δ recoverable (log scale)", fontsize=9, color=INK2)
ax.grid(axis="y", color=GRID, lw=0.8, which="major")
for sp in ("top", "right"):
    ax.spines[sp].set_visible(False)
ax.tick_params(colors=INK2, labelsize=8.5)
ax.legend(loc="lower right", fontsize=7.6, frameon=False)
fig.text(0.02, 0.985, "S₅ small roots: Coppersmith reach against the IC need (lead L3, 2026-10-18)",
         ha="left", va="top", fontsize=11.5, color=INK, fontweight="bold")
fig.text(0.02, 0.945, "dotted: basic-strategy limit δ_∞ = 2/(m(m+1)d); dashed: W10 ceiling 1/(m·d) for any shift family of one polynomial; "
         "wide band: the need (1/2, 1/4, 1/5, 1/6 at m = 2 … 5; program §2.4)", ha="left", va="top", fontsize=8, color=INK2)
fig.text(0.01, 0.005, "Exact rationals from reach.py; measured points are the Howgrave-Graham determinant bound (sublattice_search.py), no LLL run. "
         "No run, no S changes.", fontsize=7.2, color=MUTED, va="bottom")
fig.tight_layout(rect=(0, 0.03, 1, 0.92))
fig.savefig(os.path.join(HERE, "reach_graph.svg"), format="svg")
fig.savefig(os.path.join(HERE, "reach_graph.png"), dpi=150)
json.dump({"schema": "reach_graph/v1", "per_m": data, "measured_m2": MEASURED},
          open(os.path.join(HERE, "reach_graph.diagram.json"), "w"), indent=1)
print("wrote reach_graph.svg/.png/.diagram.json")
