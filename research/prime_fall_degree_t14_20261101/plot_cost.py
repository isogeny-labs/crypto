#!/usr/bin/env python3
"""Lay out the recorded cost of the capped F4 runs of this directory (runs/ and preflight/).

Document rendering only (AGENTS.md "Implementation language"): every number is read from the pilot's rows;
the dotted lines are a labelled extrapolation drawn by multiplying the last recorded value, not a fit.
Usage: plot_cost.py [studydir]
"""
import glob
import json
import os
import sys

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt

root = sys.argv[1] if len(sys.argv) > 1 else "."


def rows(pattern):
    out = []
    for f in sorted(glob.glob(os.path.join(root, pattern))):
        for line in open(f):
            r = json.loads(line)
            if "summary" not in r:
                out.append(r)
    return out


study = rows("runs/R-t11-*.jsonl") + rows("runs/T-t14-*.jsonl")
pre = rows("preflight/pf-t12-*.jsonl") + rows("preflight/pf-t13-*.jsonl")
cap5 = [r for r in study + pre if r["max_degree_bound"] == 5]
cap6_study = [r for r in study if r["max_degree_bound"] == 6]
cap6_pre = [r for r in pre if r["max_degree_bound"] == 6]
plt.rcParams.update({"font.family": "DejaVu Sans", "font.size": 9, "axes.spines.top": False,
                     "axes.spines.right": False, "svg.fonttype": "none"})
fig, (a, b) = plt.subplots(1, 2, figsize=(11.2, 4.6))
for ax, key, label in ((a, "wall_s", "wall-clock time of one capped F4 run (s)"),
                       (b, "max_dense_entries", "largest dense block (entries, 4 bytes each)")):
    ax.scatter([r["t"] for r in cap5 if r in study], [r[key] for r in cap5 if r in study], marker="o", s=34,
               color="#1b6ca8", label="cap 5, study rows (t = 11, 14; both draws)", zorder=3)
    ax.scatter([r["t"] for r in cap5 if r in pre], [r[key] for r in cap5 if r in pre], marker="o", s=34,
               facecolors="white", edgecolors="#1b6ca8", label="cap 5, pre-flight (t = 12, 13; draw 0)", zorder=3)
    ax.scatter([r["t"] for r in cap6_study], [r[key] for r in cap6_study], marker="s", s=34, color="#c4602a",
               label="cap 6, study rows (t = 11; refutes, D = 6)", zorder=3)
    for r in cap6_pre:
        ax.scatter([r["t"]], [r[key]], marker="s", s=60, facecolors="white", edgecolors="#c4602a", zorder=3,
                   label="cap 6, pre-flight t = 12: censored at the 7,200 s budget (lower bound)")
        ax.annotate("", xy=(r["t"], r[key] * 2.4), xytext=(r["t"], r[key]),
                    arrowprops=dict(arrowstyle="->", color="#c4602a"))
        ratio = 12.5 if key == "wall_s" else 4.4
        xs = [12, 13, 14]
        ax.plot(xs, [r[key] * ratio ** (x - 12) for x in xs], ls=":", color="#c4602a", lw=1.1,
                label=f"extrapolation x{ratio:g} per level (one pair of points)")
    ax.set_yscale("log")
    ax.set_xlabel("tower height t  (N = 2t)")
    ax.set_ylabel(label)
    ax.set_xticks(range(11, 15))
a.axhline(86400, color="#777", lw=0.8, ls="--")
a.text(11.05, 98000, "86,400 s: Amendment 7 A's budget per system", fontsize=7.5, color="#555")
b.axhline(1.6e9, color="#777", lw=0.8, ls="--")
b.text(11.05, 1.85e9, "1.6e9 entries: this host's --max-dense", fontsize=7.5, color="#555")
a.set_title("time: flat at cap 5, growing at cap 6", loc="left", fontsize=10)
b.set_title("memory: the same", loc="left", fontsize=10)
h, l = b.get_legend_handles_labels()
fig.legend(h, l, frameon=False, fontsize=7.6, loc="lower center", ncol=2, bbox_to_anchor=(0.5, 0.045))
fig.text(0.01, 0.008, "Rows: runs/R-t11-*, runs/T-t14-*, preflight/pf-t1[23]-* of this directory. Kummer, m = 2, P1 = 2013265921, "
         "4 vCPU, 15 GB. The dotted line is an extrapolation, not a measurement.", fontsize=7.4, color="#555")
fig.tight_layout(rect=(0, 0.16, 1, 1))
fig.savefig(os.path.join(root, "cost_vs_t.svg"))
fig.savefig(os.path.join(root, "cost_vs_t.png"), dpi=160)
print("wrote cost_vs_t.svg cost_vs_t.png")
