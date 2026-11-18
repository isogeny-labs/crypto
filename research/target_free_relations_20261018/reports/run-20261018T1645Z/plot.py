#!/usr/bin/env python3
"""Lays out the recorded F4 solving degrees of the target-free study
(research-visuals: recorded values only; readings come from `dreg_score target_free`).

    plot.py RUNS_DIR OUT_DIR [RUN_ID [EXTRA_DIR]]

EXTRA_DIR (optional) holds deviation records (the raised-cap pass,
runs_highcap/); they are drawn as stars beside the registered records.

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
extra = sys.argv[4] if len(sys.argv) > 4 else None
os.makedirs(out, exist_ok=True)

recs = []
for p in sorted(glob.glob(os.path.join(runs, "cell-*.jsonl"))):
    for line in open(p):
        if line.strip():
            recs.append(json.loads(line))
extra_recs = []
if extra:
    for p in sorted(glob.glob(os.path.join(extra, "cell-*.jsonl"))):
        for line in open(p):
            if line.strip():
                extra_recs.append(json.loads(line))
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
# Deviation records: raised cap (8 GiB step matrices), drawn as stars to the right.
for r in extra_recs:
    if (r["n"], r["ell"]) not in cells:
        continue
    ci = cells.index((r["n"], r["ell"]))
    tw = "".join(str(t) for t in r["twists"])
    ai = [k for k, a in enumerate(arms) if a[0] == r["arm"] and a[1] == r["basis"] and (a[2] is None or a[2] == tw)][0]
    color = arms[ai][4]
    measured = not (r["timed_out"] or r["oversize"])
    x = ci + (ai - 2.5) * 0.12 + 0.045
    lab = "raised cap 8 GiB (deviation), finished" if measured else None
    if lab in labelled:
        lab = None
    labelled.add(lab)
    ax.scatter([x], [r["solving_degree"]], marker="*", s=110, color=color, edgecolors="black", linewidths=0.6, label=lab, zorder=3)
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

# --- Second graph: recorded F4 sizes per arm (largest step matrix, wall seconds) ---
groups = []  # (label, color, records)
def pick(rs, arm, basis, tw=None):
    out_ = []
    for r in rs:
        if r["arm"] != arm or r["basis"] != basis:
            continue
        if tw is not None and "".join(str(t) for t in r["twists"]) != tw:
            continue
        out_.append(r)
    return out_
for basis in ("gp", "random"):
    tg = {(r["n"], r["ell"], r["draw"]): r for r in pick(recs, "target_m3", basis)}
    groups.append((f"target m = 3, {basis}", "#1f77b4" if basis == "gp" else "#7f7f7f", list(tg.values()), "registered, finished"))
    for tw in ("0000", "0123"):
        groups.append((f"target-free [{','.join(tw)}], {basis}", "#2ca02c" if basis == "gp" else "#d62728",
                       pick(recs, "target_free", basis, tw), "registered, stopped by the 1 GiB cap"))
    if extra_recs:
        groups.append((f"target-free [0,0,0,0], {basis}, 8 GiB cap", "#2ca02c" if basis == "gp" else "#d62728",
                       pick(extra_recs, "target_free", basis, "0000"), "deviation, finished"))
fig, axes = plt.subplots(1, 2, figsize=(11, 4.6))
for ax, key, ylabel, scale in ((axes[0], "matrix_cols_max", "largest F4 step matrix: columns", 1),
                               (axes[1], "secs", "wall seconds per F4 run (not isolated)", 1)):
    for gi, (label, color, rs, status) in enumerate(groups):
        ys = [r[key] / scale for r in rs]
        if not ys:
            continue
        marker = "*" if "8 GiB" in label else ("s" if "target m" in label else "o")
        filled = "finished" in status
        ax.scatter([gi] * len(ys), ys, marker=marker, s=60 if marker != "*" else 120,
                   color=color if filled else "none", edgecolors=color if marker != "*" else "black")
    ax.set_yscale("log")
    ax.set_xticks(range(len(groups)))
    ax.set_xticklabels([g[0] for g in groups], rotation=60, ha="right", fontsize=7)
    ax.set_ylabel(ylabel)
    ax.grid(axis="y", alpha=0.3, which="both")
fig.suptitle(f"Recorded F4 sizes at (n, ℓ) = (16, 4), run {run_id}: filled = finished, open = stopped by the 1 GiB step-matrix cap\n"
             "(an open marker is the size reached before the stop, a lower bound); stars: raised-cap deviation", fontsize=8.5)
fig.tight_layout()
fig.savefig(os.path.join(out, "target_free_sizes.svg"))
fig.savefig(os.path.join(out, "target_free_sizes.png"), dpi=150)
print("ok sizes")
