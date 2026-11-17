#!/usr/bin/env python3
"""Lay out recorded round-5 D values (README table) as round5.svg; no computation."""
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
# (label, D, complete?, reference D of the tree in the same cell)
rows = [("H true\nm3 t4", 6, True, 6), ("H false\nm3 t4", 6, True, 6),
        ("Yv\nm3 t4", 6, True, 6), ("Yv\nm3 t5", 7, False, 6),
        ("L\nm4 t4", 8, False, 7), ("L\nm4 t5", 8, False, 7)]
fig, ax = plt.subplots(figsize=(8, 3.8))
for i, (lab, d, done, ref) in enumerate(rows):
    ax.bar(i - 0.18, ref, width=0.36, color="#2e75b6", label="tree T (same cell)" if i == 0 else None)
    ax.bar(i + 0.18, d, width=0.36, color="#c0504d", fill=done, edgecolor="#c0504d", hatch="" if done else "//",
           label="round-5 arm (hatched = lower bound)" if i == 0 else None)
ax.set_xticks(range(len(rows))); ax.set_xticklabels([r[0] for r in rows], fontsize=8)
ax.set_ylabel("F4 solving degree D"); ax.set_ylim(0, 9)
ax.set_title("Round 5: no tested change lowers D below the tree's (msolve, p = 786433)", fontsize=9)
ax.legend(fontsize=8, loc="upper left")
fig.tight_layout(); fig.savefig("round5.svg")
