"""Plots recorded values only: summary.json (summarise.sh), every round's
ratios and the medians and intervals. Usage: plot.py <ab dir>"""
import json, sys
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
here = sys.argv[1]
s = json.load(open(f"{here}/summary.json"))
panels = [("build", "bb", "ab_build", "qb", "aa_build"), ("collect", "bc", "ab_collect", "qc", "aa_collect"),
          ("set-up", "bs", "ab_setup", "qs", "aa_setup")]
fig, axes = plt.subplots(1, 3, figsize=(10.5, 3.9), sharey=True)
for ax, (metric, kb, sb, kq, sq) in zip(axes, panels):
    for i, row in enumerate(["k0n53", "k0n61"]):
        for j, (k, st, col, lab) in enumerate([(kb, sb, "#e45756", "A/B: low layout / top layout"), (kq, sq, "#4c78a8", "A/A: low / low")]):
            x = i + (j - 0.5) * 0.35
            pts = [r[k] for r in s[row]["ratios"]]
            ax.scatter([x] * len(pts), pts, s=5, color=col, alpha=0.3)
            m = s[row][st]
            ax.errorbar([x + 0.06], [m["median"]], yerr=[[m["median"] - m["ci95"][0]], [m["ci95"][1] - m["median"]]],
                        fmt="o", ms=4, color=col, capsize=3, label=lab if i == 0 else None)
    ax.axhline(1, color="k", lw=0.6)
    ax.set_xticks([0, 1]); ax.set_xticklabels(["2^44.3 (k0n53)", "2^47.2 (k0n61)"], fontsize=8)
    ax.set_title(f"{metric}: per-round ratio, median, 95% bootstrap", fontsize=8)
    ax.set_ylim(0.85, 1.3); ax.grid(alpha=0.25); ax.legend(fontsize=6.5, loc="upper left")
axes[0].set_ylabel("baseline / arm time (above 1: top layout faster)")
fig.text(0.01, 0.01, "42 rounds a row, labels a q t cycled over six orders; one binary (39affc25bf, scan-probes); Intel Xeon container", fontsize=6)
fig.tight_layout(rect=(0, 0.03, 1, 1))
fig.savefig(f"{here}/report/ab.svg"); fig.savefig(f"{here}/report/ab.png", dpi=150)
