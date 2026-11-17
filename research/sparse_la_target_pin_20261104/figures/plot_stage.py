#!/usr/bin/env python3
"""Plot results/stage-a-summary.json (layout only; every value is a recorded median or a
paired quotient computed by stage_summary.jq)."""
import json
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

HERE = Path(__file__).resolve().parent
rows = json.load(open(HERE.parent / "results/stage-a-summary.json"))
ARMS = [("pin-wiedemann-every", "pin-wiedemann-every (attempt every row)", "#c55a11", "^", "-"),
        ("pin-wiedemann", "pin-wiedemann (spaced attempts)", "#ed7d31", "v", "--"),
        ("igfr", "dense, full rank", "#7f7f7f", "D", ":"),
        ("sparse-cado", "sparse-cado (full rank)", "#1f4e79", "o", "-"),
        ("filter+lanczos+counted", "filter+lanczos, block arithmetic counted", "#70ad47", "s", "--")]

def panels(key, ylabel, title, name):
    ws = [2, 4, 8]
    fig, ax = plt.subplots(1, 3, figsize=(14, 4.2), sharey=True)
    for a, w in zip(ax, ws):
        for arm, lab, col, mk, ls in ARMS:
            pts = sorted((r["columns"], r[key]) for r in rows if r["weight"] == w and r["arm"] == arm and r[key] is not None)
            a.plot([p[0] for p in pts], [p[1] for p in pts], marker=mk, ls=ls, color=col, label=lab)
        a.axhline(1, color="black", lw=1)
        a.set_xscale("log", base=2)
        a.set_yscale("log")
        a.set_title(f"base weight w = {w}")
        a.set_xlabel("columns")
        a.grid(True, which="both", alpha=0.25)
    ax[0].set_ylabel(ylabel)
    ax[0].legend(fontsize=7)
    fig.suptitle(title, fontsize=9)
    for ext, kw in (("svg", {}), ("png", {"dpi": 150})):
        fig.savefig(HERE / f"{name}.{ext}", bbox_inches="tight", **kw)
    plt.close(fig)

panels("ops_over_sg_median", "counted row_ops ÷ structured-gauss (paired, median of 3 seeds)",
       "Stage diagnostic on planted systems (sparse_la_stage/v1, 420 runs, 0 wrong values, 8 full-rank runs censored); line = structured-gauss", "stage_ops_over_sg")
panels("rows_over_sg_median", "relations used ÷ structured-gauss (paired, median of 3 seeds)",
       "Relations needed to finish, relative to the target-pin stop of structured-gauss; line = structured-gauss", "stage_rows_over_sg")
