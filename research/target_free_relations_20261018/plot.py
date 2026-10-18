#!/usr/bin/env python3
"""Lays out the recorded F4 solving degrees of the target-free study
(research-visuals: recorded values only; readings come from `dreg_score target_free`).

    plot.py RUNS_DIR OUT_DIR [RUN_ID]

Reads runs/cell-*.jsonl (one JSON record per arm and draw, as dreg_target_free
prints them) and writes target_free_degrees.svg/.png: per cell, the solving
degree of the target-free arm (plain and twisted, gp and random) beside the
target m = 3 arm on the same basis.  Unfinished records (oversize or timed
out) are drawn as lower bounds at the highest degree that added a basis
element, with an open marker and an arrow.
"""
import glob
import json
import os
import sys
from collections import defaultdict

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
arms = [
    ("target_m3", "gp", None, "target m = 3, gp base", "#1f77b4", "s"),
    ("target_free", "gp", "0000", "target-free plain, gp", "#2ca02c", "o"),
    ("target_free", "gp", "0123", "target-free twisted [0,1,2,3], gp", "#2ca02c", "D"),
    ("target_m3", "random", None, "target m = 3, random base", "#7f7f7f", "s"),
    ("target_free", "random", "0000", "target-free plain, random", "#d62728", "o"),
    ("target_free", "random", "0123", "target-free twisted [0,1,2,3], random", "#d62728", "D"),
]
fig, ax = plt.subplots(figsize=(9, 4.8))
labelled = set()
for ci, (n, ell) in enumerate(cells):
    for ai, (arm, basis, tw, label, color, marker) in enumerate(arms):
        x = ci * 1.0 + (ai - 2.5) * 0.12
        pts = [r for r in recs if r["n"] == n and r["ell"] == ell and r["arm"] == arm and r["basis"] == basis
               and (tw is None or "".join(str(t) for t in r["twists"]) == tw)]
        # the target arm is recorded once per twist file; keep one record per draw
        seen = {}
        for r in pts:
            seen.setdefault(r["draw"], r)
        for k, r in enumerate(sorted(seen.values(), key=lambda r: r["draw"])):
            measured = not (r["timed_out"] or r["oversize"])
            y = r["solving_degree"]
            xx = x + (k - (len(seen) - 1) / 2) * 0.025
            kw = dict(marker=marker, s=46, color=color, label=label if (label not in labelled) else None)
            labelled.add(label)
            if measured:
                ax.scatter([xx], [y], **kw)
            else:
                kw.update(facecolors="none", edgecolors=color)
                kw.pop("color")
                ax.scatter([xx], [y], **kw)
                ax.annotate("", (xx, y + 0.55), (xx, y + 0.05), arrowprops=dict(arrowstyle="->", color=color, lw=0.9))
ax.set_xticks(range(len(cells)))
ax.set_xticklabels([f"(n, ℓ) = ({n}, {ell})\nunknowns {4*ell+n} vs {3*ell+n}" for n, ell in cells])
ax.set_ylabel("F4 solving degree (field equations included)")
ax.set_ylim(2.5, 7)
ax.grid(axis="y", alpha=0.3)
ax.legend(fontsize=7, loc="upper left", ncol=2)
ax.set_title(f"Target-free 4-sum system against the chained m = 3 target system on the same base, run {run_id}\n"
             "filled: F4 finished (measured); open with arrow: unfinished (oversize or timed out), lower bound = highest degree that added a basis element",
             fontsize=8.5)
fig.tight_layout()
fig.savefig(os.path.join(out, "target_free_degrees.svg"))
fig.savefig(os.path.join(out, "target_free_degrees.png"), dpi=150)
print("ok", len(recs), "records")
