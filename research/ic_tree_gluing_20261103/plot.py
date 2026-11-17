#!/usr/bin/env python3
"""Lay out recorded msolve values (rows.tsv from parse.sh) as d_vs_t.svg. No computation of results."""
import csv, re, sys
from collections import defaultdict
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

rows = list(csv.DictReader(open(sys.argv[1] if len(sys.argv) > 1 else "rows.tsv"), delimiter="\t"))
cell = defaultdict(list)
for r in rows:
    m, t, k, arm = re.match(r"m(\d)_t(\d)_k(\d)_(\w)\.log", r["file"]).groups()
    cell[(int(m), arm)].append((int(t), int(r["Dsel"]), int(r["width"]), r["exit"] == "0"))

colors = {"C": "#c0504d", "T": "#2e75b6", "F": "#7f7f7f"}
names = {"C": "C caterpillar (plain chain)", "T": "T tree, cherry-symmetric (new)", "F": "F full symmetric S_{m+1}(e)"}
fig, axes = plt.subplots(2, 3, figsize=(12, 7), sharex=True)
for j, m in enumerate((2, 3, 4)):
    for arm in "CTF":
        pts = sorted(cell.get((m, arm), []))
        if not pts: continue
        for row, idx in ((0, 1), (1, 2)):
            ax = axes[row][j]
            off = {"C": -0.08, "T": 0.0, "F": 0.08}[arm]; ts = [p[0] + off for p in pts]; ys = [p[idx] for p in pts]
            ax.plot(ts, ys, "-", color=colors[arm], alpha=0.5)
            for p in pts:
                ax.plot(p[0] + off, p[idx], "o" if p[3] else "^", color=colors[arm],
                        mfc=colors[arm] if p[3] else "none", ms=7)
    axes[0][j].set_title(f"m = {m}")
    axes[0][j].set_ylabel("solving degree D (^ = lower bound, timeout)")
    axes[1][j].set_ylabel("widest Macaulay matrix (columns)")
    axes[1][j].set_yscale("log")
    axes[1][j].set_xlabel("tower depth t  (|V| = 2^t)")
for arm in "CTF":
    axes[0][0].plot([], [], "o-", color=colors[arm], label=names[arm])
axes[0][0].legend(fontsize=8, loc="upper left")
fig.suptitle("msolve 0.6.5 F4 (grevlex, 1 thread), p = 786433, 2 planted on-curve targets per cell "
             "(identical across targets except where shown)", fontsize=9)
fig.tight_layout()
fig.savefig("d_vs_t.svg"); fig.savefig("d_vs_t.png", dpi=130)
