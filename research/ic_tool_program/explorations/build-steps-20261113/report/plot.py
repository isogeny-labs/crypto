"""Plots recorded values only: summary.json (summarise.sh), the median ns
per stored pair of each build step, with the min-max over processes."""
import json, sys
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
here = sys.argv[1]
s = json.load(open(f"{here}/summary.json"))
steps = ["setup", "row_add", "row_key", "stream_push", "sort_fill", "filter_merge", "assemble"]
labels = ["set-up", "additions", "keys", "stream push", "counting sort + filter fill", "filter merge", "assembly"]
cols = ["#bab0ac", "#4c78a8", "#72b7b2", "#e45756", "#f58518", "#eeca3b", "#9d755d"]
rows = ["k0n53", "k0n61"]
fig, ax = plt.subplots(figsize=(8.2, 2.9))
for i, row in enumerate(rows):
    left = 0.0
    for st, lab, col in zip(steps, labels, cols):
        v = s[row]["per_pair_ns"][st]["median"]
        ax.barh(i, v, left=left, color=col, edgecolor="white", label=lab if i == 0 else None)
        if v > 2:
            ax.text(left + v / 2, i, f"{v:.1f}", ha="center", va="center", fontsize=7, color="white")
        left += v
    w = s[row]["per_pair_ns"]["whole"]
    ax.plot([w["median"]], [i], "k|", ms=18)
    ax.text(w["median"] + 0.4, i, f"whole {w['median']:.1f}", va="center", fontsize=7)
ax.set_yticks(range(len(rows)))
ax.set_yticklabels([f"2^44.3 (k0n53)\n{s['k0n53']['stored_pairs_per_build']/1e6:.2f} M pairs", f"2^47.2 (k0n61)\n{s['k0n61']['stored_pairs_per_build']/1e6:.2f} M pairs"], fontsize=8)
ax.set_xlabel("ns per stored pair (median of 5 processes, one thread)")
ax.set_xlim(0, 42); ax.invert_yaxis(); ax.grid(axis="x", alpha=0.25)
ax.legend(fontsize=6.5, ncol=4, loc="upper center", bbox_to_anchor=(0.5, -0.32), frameon=False)
ax.set_title("Folded pair-table build: where a stored pair's time goes", fontsize=9)
fig.tight_layout(); fig.savefig(f"{here}/report/steps.svg"); fig.savefig(f"{here}/report/steps.png", dpi=150)
