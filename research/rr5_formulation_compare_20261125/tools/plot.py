#!/usr/bin/env python3
"""Degree reached and wall time vs matrix-entry cap, per formulation (planted4 and query1)."""
import json, subprocess, sys, matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
R = sys.argv[1]
rows = []
for panel in ("ladder-01", "ladder-02"):
    try:
        rows += json.loads(subprocess.check_output([sys.executable, f"{R}/tools/summarize.py", f"{R}/evidence/{panel}"]))
    except Exception:
        pass
arms = [("coupled", "coupled circuit (122 vars)", "#212529"), ("sectors", "five-sector family, max degree", "#4c6ef5"),
        ("sector4", "sector 4 alone (59 vars)", "#91a7ff"), ("graeffe", "collapsed Graeffe (47 vars)", "#f08c00"),
        ("deflated", "Graeffe + 6 polar (412 vars)", "#e03131")]
fig, axes = plt.subplots(1, 2, figsize=(12.5, 4.4))
for target, mk in (("planted4", "o"), ("query1", "s")):
    for arm, lab, col in arms:
        pts = sorted((r["matrix_cap"], r) for r in rows if r["target"] == target and r["arm"] == arm)
        if not pts: continue
        xs = [c for c, _ in pts]
        deg = [max(d for d in r["degree"] if d is not None) if isinstance(r["degree"], list) else r["degree"] for _, r in pts]
        wall = [r["wall_s"] for _, r in pts]
        kw = dict(color=col, marker=mk, ls="-" if target == "planted4" else "--", label=f"{lab}" if target == "planted4" else None)
        axes[0].plot(xs, deg, **kw); axes[1].plot(xs, wall, **kw)
axes[0].set_xscale("log"); axes[0].set_xlabel("F4 matrix-entry admission cap"); axes[0].set_ylabel("F4 degree reached before stop")
axes[0].set_title("Progress before the resource stop (no arm completed)", fontsize=10)
axes[1].set_xscale("log"); axes[1].set_yscale("log"); axes[1].set_xlabel("F4 matrix-entry admission cap"); axes[1].set_ylabel("wall seconds to stop")
axes[1].set_title("Time to the resource stop", fontsize=10)
axes[0].legend(fontsize=7); 
for a in axes: a.grid(alpha=.3)
fig.suptitle("RR5 formulations on frozen B677 M31, identical targets (solid: planted sector-4 target, dashed: public query 1)", fontsize=10)
fig.tight_layout()
for ext in ("svg", "pdf", "png"):
    fig.savefig(f"{R}/figures/ladder.{ext}", dpi=150)
