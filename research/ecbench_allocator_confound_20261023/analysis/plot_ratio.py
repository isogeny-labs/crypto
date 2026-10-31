#!/usr/bin/env python3
"""Fig. 2: base / variant ratio of the pooled median solve_wall_ns per smoke arm, read from
analysis/ratios.json (written by compare_groups.sh from summary.json).  Plots recorded
summaries only.  Usage: plot_ratio.py STUDY_DIR OUTBASE"""
import json, sys
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
study, outbase = sys.argv[1], sys.argv[2]
r = json.load(open(f"{study}/analysis/ratios.json"))
S = json.load(open(f"{study}/analysis/summary.json"))
V = [("smoke-base__smoke-fix", "fix (trim + probe)", "#2f6db0"),
     ("smoke-base__smoke-notrim", "fix-notrim (probe only)", "#5aa5a0"),
     ("smoke-base__smoke-noprobe", "fix-noprobe (trim only)", "#d19a2a"),
     ("smoke-base__smoke-notrim2", "fix-notrim2 (neither)", "#7b5ea7")]
arms = [a for a in r[V[0][0]] if r[V[0][0]][a]["solve_wall_ns"]]
fig, ax = plt.subplots(figsize=(10, 4.6))
w = 0.2
for k, (key, lab, col) in enumerate(V):
    ys = [r[key][a]["solve_wall_ns"]["ratio_a_over_b"] for a in arms]
    ax.bar([i + (k - 1.5) * w for i in range(len(arms))], ys, w, color=col, label=lab)
ax.axhline(1, color="black", lw=0.8)
ax.set_yscale("log")
ax.set_xticks(range(len(arms)))
ax.set_xticklabels(arms, rotation=25, ha="right", fontsize=8)
ax.set_ylabel("pooled median solve_wall_ns: base / variant (log)")
n = S["smoke-base"]["arms"][arms[0]]["solve_wall_ns"]["n"]
ax.set_title(f"smoke-prime: how many times shorter the median solve reads than on the merge-base binary\n"
             f"(n = {n} per arm for base and fix, 6 for each ablation; descriptive wall, shared 4-vCPU VM; ratio 1 = no change)", fontsize=9)
ax.legend(fontsize=8, ncol=2)
ax.grid(axis="y", alpha=0.3, which="both", lw=0.4)
fig.tight_layout()
fig.savefig(outbase + ".svg")
fig.savefig(outbase + ".png", dpi=150)
