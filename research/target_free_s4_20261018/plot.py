#!/usr/bin/env python3
"""Lays out the recorded values of the eliminated target-free S4 study
(research-visuals: recorded values only; readings come from `dreg_score s4_free`).

    plot.py RUNS_DIR OUT_DIR [RUN_ID]

Writes s4_degrees.svg/.png: per cell, the F4 solving degree of the target-free
arm (gp plain, gp twisted, random plain) beside the S4 target arm on the same
bases (filled = finished, open with arrow = unfinished lower bound), and the
top-form deficiency (top_forms - top_rank) per arm.
"""
import glob
import json
import os
import sys

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

runs, out = sys.argv[1], sys.argv[2]
run_id = sys.argv[3] if len(sys.argv) > 3 else os.path.basename(out)
os.makedirs(out, exist_ok=True)
recs = []
for p in sorted(glob.glob(os.path.join(runs, "cell-*.jsonl"))):
    for line in open(p):
        if line.strip():
            recs.append(json.loads(line))
cells = sorted({(r["n"], r["ell"]) for r in recs})
tws = lambda r: "".join(str(t) for t in r["twists"])
arms = [
    ("target_s4", "gp", "0000", "target S4, gp", "#1f77b4", "s"),
    ("target_free_s4", "gp", "0000", "target-free S4, gp", "#2ca02c", "o"),
    ("target_s4", "gp", "0123", "target S4, gp twisted", "#17becf", "s"),
    ("target_free_s4", "gp", "0123", "target-free S4, gp twisted [0,1,2,3]", "#2ca02c", "D"),
    ("target_s4", "random", "0000", "target S4, random", "#7f7f7f", "s"),
    ("target_free_s4", "random", "0000", "target-free S4, random", "#d62728", "o"),
]
fig, (ax, bx) = plt.subplots(1, 2, figsize=(12, 4.8), gridspec_kw={"width_ratios": [3, 2]})
seen = set()
for ci, cell in enumerate(cells):
    for ai, (arm, basis, tw, label, color, marker) in enumerate(arms):
        pts = [r for r in recs if (r["n"], r["ell"]) == cell and r["arm"] == arm and r["basis"] == basis and tws(r) == tw]
        x0 = ci + (ai - 2.5) * 0.13
        for k, r in enumerate(pts):
            x = x0 + (k - (len(pts) - 1) / 2) * 0.02
            fin = not (r["timed_out"] or r["oversize"])
            lab = label if label not in seen else None
            seen.add(label)
            if fin:
                ax.scatter([x], [r["solving_degree"]], marker=marker, s=40, color=color, label=lab)
            else:
                ax.scatter([x], [r["solving_degree"]], marker=marker, s=40, facecolors="none", edgecolors=color, label=lab)
                ax.annotate("", (x, r["solving_degree"] + 0.6), (x, r["solving_degree"] + 0.05), arrowprops=dict(arrowstyle="->", color=color, lw=0.9))
        defs = [r["top_forms"] - r["top_rank"] for r in pts]
        if defs and arm == "target_free_s4":
            bx.scatter([ci + (ai - 2.5) * 0.13] * len(defs), defs, marker=marker, s=40, color=color)
for a in (ax, bx):
    a.set_xticks(range(len(cells)))
    a.set_xticklabels([f"({n}, {l})\n{4*l} vs {3*l} unk." for n, l in cells], fontsize=8)
    a.grid(axis="y", alpha=0.3)
ax.set_ylabel("F4 solving degree (field equations included)")
ax.legend(fontsize=7, loc="upper left", ncol=2)
ax.set_title("solving degree: filled = finished; open + arrow = unfinished (lower bound)", fontsize=8.5)
bx.set_ylabel("top-form dependencies (top_forms − top_rank)")
bx.set_title("target-free arm: dependencies among top-degree parts\n(green = gp, red = random; diamond = gp twisted)", fontsize=8.5)
fig.suptitle(f"Eliminated S4: target-free (4ℓ unknowns) against target x_R (3ℓ unknowns) on the same base, run {run_id}", fontsize=9.5)
fig.tight_layout()
fig.savefig(os.path.join(out, "s4_degrees.svg"))
fig.savefig(os.path.join(out, "s4_degrees.png"), dpi=150)
print("ok", len(recs), "records")
