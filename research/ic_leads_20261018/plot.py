#!/usr/bin/env python3
"""Lays out two recorded things for RESEARCH_IC_LEADS_20261018.md (no computation):
(1) the lead map: which closure argument each lead must pass, from the note's
table; (2) the coset-indicator pilot: ANF degree of Tr(1/x)|V against ℓ from
coset_indicator_pilot.jsonl, with the measured closure degrees of the anchor
cells (4) and the lower bound at ℓ = 7 for comparison.
Usage: plot.py OUT_DIR
"""
import json
import os
import sys
from collections import defaultdict

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

out = sys.argv[1]
here = os.path.dirname(os.path.abspath(__file__))
os.makedirs(out, exist_ok=True)

# --- (2) coset-indicator pilot --------------------------------------------
rows = [json.loads(l) for l in open(os.path.join(here, "coset_indicator_pilot.jsonl"))]
by = defaultdict(list)
for r in rows:
    kind = "committed draw bases" if "draw" in r["source"] else r["source"].split(":")[0]
    by[kind].append((r["ell"], r["deg_tr_inv"]))
fig, ax = plt.subplots(figsize=(7.5, 4.4))
marks = {"committed draw bases": ("o", "#1f77b4"), "gp": ("s", "#2ca02c"), "random": ("^", "#d62728")}
for kind, pts in by.items():
    m, c = marks.get(kind, ("x", "gray"))
    xs = [p[0] + (0.12 if kind == "random" else -0.12 if kind == "gp" else 0) for p in pts]
    ax.scatter(xs, [p[1] for p in pts], marker=m, color=c, s=36, alpha=0.8, label=f"deg Tr(1/x)|V, {kind} (n = {'43' if kind != 'committed draw bases' else '12–30'})")
ax.plot([2, 21], [1, 20], color="gray", ls=":", lw=1, label="ℓ − 1")
# measured closure degrees of the chained system (previous studies): 4 at ℓ ≤ 6 (m = 3) and ℓ ≤ 4 (m = 4); > 4 at ℓ = 7
ax.scatter([3, 4, 5, 6], [4, 4, 4, 4], marker="D", s=70, facecolors="none", edgecolors="black", label="measured closure degree of the chained S₃ system (committed, m = 3 and 4)")
ax.scatter([7], [5], marker="^", s=80, facecolors="none", edgecolors="black")
ax.annotate("≥ 5 (committed)", (7.2, 5.0), fontsize=8)
ax.set_xlabel("ℓ = dim V")
ax.set_ylabel("Boolean (ANF) degree")
ax.set_xlim(2, 21)
ax.set_ylim(0, 21)
ax.grid(alpha=0.3)
ax.legend(fontsize=7, loc="upper left")
ax.set_title("Pilot: the 2-torsion coset indicator restricted to V has degree ≈ ℓ − 1 on every shape;\nthe measured closure degree sits far below it", fontsize=9)
fig.tight_layout()
fig.savefig(os.path.join(out, "coset_indicator_pilot.svg"))
fig.savefig(os.path.join(out, "coset_indicator_pilot.png"), dpi=150)

# --- (1) lead map ----------------------------------------------------------
arguments = ["generic-group bound\n(table oracles)", "no stable subspace\nat n = 131, 163", "shape closure\nat m ≥ 3", "admissible-degree\ninequality", "measured degree\nwall (this host)"]
leads = [
    ("L1 target-free algebraic relations", ["pass", "pass", "closed: no lower degree", "must still pass", "closed: +1 degree (S4)"]),
    ("L2 dense-bit-row XL at degree 5", ["n/a", "n/a", "n/a", "n/a", "aimed at it"]),
    ("L3 raised-cap sparse closure", ["n/a", "n/a", "n/a", "n/a", "closed: OOM at 7.2 GB"]),
    ("L4 4E-class stratification", ["pass", "pass", "closed: free signs", "n/a", "n/a"]),
    ("L5 coset-indicator degree (pilot)", ["pass", "pass", "closed: ≈ ℓ−1 on every V", "n/a", "n/a"]),
    ("L6 theorem: shape closure m ≥ 3", ["n/a", "n/a", "formalises it", "n/a", "n/a"]),
    ("L7 lemma: admissible degree", ["n/a", "n/a", "n/a", "formalises it", "n/a"]),
    ("L8 K-233 derived row", ["pass", "stable 29-blocks exist", "pass", "D ≤ 3–4 (cols^2.2) / 7–8 (w = 1)", "n/a"]),
    ("L9 G4 stable non-subspace base", ["pass", "escapes (not a subspace)", "untested", "3n unknowns: fails", "n/a"]),
    ("L10 ladder: retire online rows (Lead A)", ["is the bound", "n/a", "n/a", "n/a", "n/a"]),
]
colors = {"pass": "#e8f1fa", "n/a": "#f4f4f4", "escapes": "#dff2df", "untested": "#fff4d6", "must": "#fff4d6", "closed": "#f9dcdc", "fails": "#f9dcdc", "formalises": "#dff2df", "aimed": "#fff4d6", "in flight": "#fff4d6", "accounting": "#e8f1fa", "stable": "#dff2df", "D ≤": "#e8f1fa", "is the": "#e8f1fa"}
fig, ax = plt.subplots(figsize=(11, 5.6))
ax.set_axis_off()
cell_w, cell_h = 1.0, 1.0
for j, a in enumerate(arguments):
    ax.text(j + 0.5 + 2.6, len(leads) + 0.35, a, ha="center", va="bottom", fontsize=8, weight="bold")
for i, (name, cells) in enumerate(leads):
    y = len(leads) - 1 - i
    ax.text(2.5, y + 0.5, name, ha="right", va="center", fontsize=8)
    for j, c in enumerate(cells):
        col = "#f4f4f4"
        for k, v in colors.items():
            if c.startswith(k):
                col = v
                break
        ax.add_patch(plt.Rectangle((j + 2.6, y), cell_w, cell_h, facecolor=col, edgecolor="white"))
        ax.text(j + 0.5 + 2.6, y + 0.5, c, ha="center", va="center", fontsize=7)
ax.set_xlim(0, len(arguments) + 2.7)
ax.set_ylim(-0.2, len(leads) + 1.4)
ax.set_title("Lead map: each lead against the five closure arguments it must survive (from RESEARCH_IC_LEADS_20261018.md)", fontsize=9)
fig.tight_layout()
fig.savefig(os.path.join(out, "lead_map.svg"))
fig.savefig(os.path.join(out, "lead_map.png"), dpi=150)
print("ok")
