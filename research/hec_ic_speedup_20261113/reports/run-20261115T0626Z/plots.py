#!/usr/bin/env python3
"""S4 figures from s4-20261115/summary.json (record 20261115T0626Z-hec-s4-line-sieve).

    python3 plots.py ../../s4-20261115/summary.json
writes relcost.svg/png (field products per relation, relation phase, walk vs
sieve, log scale, min-max whiskers) and ratios.svg/png (P1, P2, P3 per row
with the E13/E14/E15 bars).  Counts are deterministic.
"""
import json
import sys

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

S = json.load(open(sys.argv[1]))["rows"]
rows = sorted(S.values(), key=lambda r: (r["g"], r["p"]))
labels = [f"g{r['g']} p{r['p']}\nm={r['m']}" for r in rows]
x = range(len(rows))
REC = "record 20261115T0626Z-hec-s4-line-sieve"

fig, ax = plt.subplots(figsize=(10, 4.2))
for off, arm, col in ((-0.18, "walk", "#7a7a7a"), (0.18, "sieve", "#2a7ab9")):
    vals = [r[f"{arm}_prod_rel"] for r in rows]
    lo = [r[f"{arm}_prod_rel"] - r[f"{arm}_prod_rel_range"][0] for r in rows]
    hi = [r[f"{arm}_prod_rel_range"][1] - r[f"{arm}_prod_rel"] for r in rows]
    ax.bar([i + off for i in x], vals, 0.34, yerr=[lo, hi], capsize=2, color=col,
           label=f"{arm} ({'Solver::Dense walk' if arm == 'walk' else 'full-base line sieve'})")
ax.set_yscale("log")
ax.set_xticks(list(x))
ax.set_xticklabels(labels, fontsize=7)
ax.set_ylabel("field products per relation\n(relation phase, median of 5 seeds)")
ax.legend(fontsize=8, loc="upper left")
ax.grid(axis="y", alpha=0.3)
fig.text(0.01, 0.01, f"count-ops build, verification excluded; {REC}", fontsize=7, color="#555")
fig.tight_layout(rect=(0, 0.04, 1, 1))
for ext in ("svg", "png"):
    fig.savefig(f"relcost.{ext}", dpi=150)

fig, ax = plt.subplots(figsize=(10, 4.2))
for off, k, col, bar in ((-0.25, "P1", "#2a7ab9", 5), (0, "P2", "#58a55c", 10), (0.25, "P3", "#d08c2b", 2)):
    ax.bar([i + off for i in x], [r[k] for r in rows], 0.24, color=col, label=f"{k} (bar {bar}×)")
    ax.hlines(bar, -0.5, len(rows) - 0.5, colors=col, linestyles="dashed", linewidth=0.8)
ax.set_yscale("log")
ax.set_xticks(list(x))
ax.set_xticklabels(labels, fontsize=7)
ax.set_ylabel("walk / sieve (ratio of medians)")
ax.legend(fontsize=8, loc="upper right")
ax.grid(axis="y", alpha=0.3)
fig.text(0.01, 0.01, f"P1 products/rel, P2 inversions/rel, P3 products rel+solve; bars apply to g3 and g4 p≥127 (P3: p≤1021); {REC}",
         fontsize=7, color="#555")
fig.tight_layout(rect=(0, 0.04, 1, 1))
for ext in ("svg", "png"):
    fig.savefig(f"ratios.{ext}", dpi=150)
