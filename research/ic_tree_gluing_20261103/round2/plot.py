#!/usr/bin/env python3
"""Lay out recorded D values (rows.tsv: round-2 logs plus round-1 rows) as separator.svg. No computation."""
import csv, re
from collections import defaultdict
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

cell = defaultdict(dict)
for r in csv.DictReader(open("rows.tsv"), delimiter="\t"):
    m, t, k, arm = re.match(r"(?:\w+:)?m(\d)_t(\d)_k(\d)_(\w)\.log", r["file"]).groups()
    if k != "0": continue
    done = r["exit"] == "0"
    key = (int(m), arm)
    t = int(t)
    # keep the completed run if there is one, else the highest selected degree
    prev = cell[key].get(t)
    if prev is None or (done and not prev[1]) or (not prev[1] and not done and int(r["Dsel"]) > prev[0]):
        cell[key][t] = (int(r["Dsel"]), done)

colors = {"C": "#c0504d", "T": "#2e75b6", "F": "#7f7f7f", "S": "#4f9a4f"}
names = {"C": "C caterpillar", "T": "T tree (cherry-symmetric)", "F": "F full symmetric",
         "S": "S tree with separators fixed"}
fig, axes = plt.subplots(1, 2, figsize=(10, 4), sharey=True)
for j, m in enumerate((3, 4)):
    ax = axes[j]
    for arm, off in (("C", -0.09), ("T", -0.03), ("F", 0.03), ("S", 0.09)):
        pts = sorted(cell.get((m, arm), {}).items())
        if not pts: continue
        ax.plot([t + off for t, _ in pts], [d for _, (d, _) in pts], "-", color=colors[arm], alpha=0.5)
        for t, (d, done) in pts:
            ax.plot(t + off, d, "o" if done else "^", color=colors[arm], mfc=colors[arm] if done else "none", ms=8)
    ax.axhline(4, color="#4f9a4f", lw=0.6, ls=":")
    ax.set_title(f"m = {m}")
    ax.set_xlabel("tower depth t")
    ax.set_xticks([4, 5, 6])
axes[0].set_ylabel("F4 solving degree D  (open ^ = lower bound, cut off)")
for arm in "CTFS":
    axes[1].plot([], [], "o-", color=colors[arm], label=names[arm])
axes[1].legend(fontsize=8, loc="center right")
fig.suptitle("Fixing the partial-sum separators brings D to the cherry level (4); msolve 0.6.5, p = 786433, target k = 0",
             fontsize=9)
fig.tight_layout()
fig.savefig("separator.svg")
