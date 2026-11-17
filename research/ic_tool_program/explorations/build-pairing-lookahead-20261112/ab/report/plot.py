"""Plots recorded values only: summary.json (summarise.sh), which holds every
round's ratios and the medians and intervals. Usage: plot.py <ab dir>"""
import json, sys
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
here = sys.argv[1]
s = json.load(open(f"{here}/summary.json"))
panels = [("build", [("bb", "ab_build_paired", "#e45756", "A/B: unpaired / paired build"),
                     ("qb", "aa_build", "#4c78a8", "A/A")]),
          ("collect", [("lc", "ab_collect_lookahead", "#f58518", "A/B: lookahead 32 / 64"),
                       ("qc", "aa_collect", "#4c78a8", "A/A")]),
          ("set-up", [("bs", "ab_setup_paired", "#e45756", "A/B: pairing"),
                      ("ls", "ab_setup_lookahead", "#f58518", "A/B: lookahead 64"),
                      ("qs", "aa_setup", "#4c78a8", "A/A")])]
fig, axes = plt.subplots(1, 3, figsize=(10.5, 3.9), sharey=True)
for ax, (metric, series) in zip(axes, panels):
    w = 0.7 / len(series)
    for i, row in enumerate(["k0n53", "k0n61"]):
        for j, (k, stat, col, lab) in enumerate(series):
            x = i + (j - (len(series) - 1) / 2) * w
            pts = [r[k] for r in s[row]["ratios"]]
            ax.scatter([x] * len(pts), pts, s=5, color=col, alpha=0.3)
            st = s[row][stat]
            ax.errorbar([x + 0.05], [st["median"]], yerr=[[st["median"] - st["ci95"][0]], [st["ci95"][1] - st["median"]]],
                        fmt="o", ms=4, color=col, capsize=3, label=lab if i == 0 else None)
    ax.axhline(1, color="k", lw=0.6)
    ax.set_xticks([0, 1]); ax.set_xticklabels(["2^44.3 (k0n53)", "2^47.2 (k0n61)"], fontsize=8)
    ax.set_title(f"{metric}: per-round ratio, median, 95% bootstrap", fontsize=8)
    ax.set_ylim(0.85, 1.2); ax.grid(alpha=0.25); ax.legend(fontsize=6.5, loc="upper left")
axes[0].set_ylabel("baseline / arm time (above 1: arm faster)")
fig.text(0.01, 0.01, "44 rounds a row, labels a q b l in a cyclic Latin square; one binary per block (amendment 1); Intel Xeon container", fontsize=6)
fig.tight_layout(rect=(0, 0.03, 1, 1))
fig.savefig(f"{here}/report/ab.svg"); fig.savefig(f"{here}/report/ab.png", dpi=150)
