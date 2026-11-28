#!/usr/bin/env python3
"""Figure for README/REPORT: share of pairs by number of distinct F_p roots of S3, per pair class.
Reads data/results_main.json (frozen run); writes l4_roots.svg and l4_roots.png."""
import json, os
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

here = os.path.dirname(os.path.abspath(__file__))
res = json.load(open(os.path.join(here, "data", "results_main.json")))
cells = [("exhaustive_k16_std", "k = 16, all pairs (exhaustive)"),
         ("sample_k20_std", "k = 20, 2e8 sampled pairs per class"),
         ("sample_k24_std", "k = 24, 5e7 sampled pairs per class")]
classes = ["EE", "TT", "ET", "ZE", "ZT"]
names = {"EE": "(E,E)", "TT": "(twist,twist)", "ET": "(E,twist)", "ZE": "(2-torsion,E)", "ZT": "(2-torsion,twist)"}
cols = {0: "#bbbbbb", 1: "#e69f00", 2: "#0072b2"}
labs = {0: "0 roots", 1: "1 root", 2: "2 distinct roots"}
fig, axes = plt.subplots(1, 3, figsize=(10, 2.8), sharey=True)
for ax, (key, title) in zip(axes, cells):
    tab = [t for t in res["runs"][key]["table"] if not t["x1_eq_x2"] and t["pair_class"] in classes]
    for yi, c in enumerate(classes):
        t = next(t for t in tab if t["pair_class"] == c)
        n = t["pairs"]
        left = 0.0
        for r in (0, 1, 2):
            w = t[f"roots{r}"] / n
            if w > 0:
                ax.barh(yi, w, left=left, color=cols[r], label=labs[r])
            left += w
        ax.text(1.02, yi, f"{n:.3g}", va="center", fontsize=6)
    ax.set_title(title, fontsize=7)
    ax.set_xlim(0, 1.18)
    ax.set_xticks([0, 0.5, 1])
    ax.tick_params(labelsize=7)
    ax.set_yticks(range(len(classes)))
    ax.set_yticklabels([names[c] for c in classes], fontsize=7)
    ax.invert_yaxis()
    ax.set_xlabel("share of pairs (x1 != x2)", fontsize=7)
h, l = axes[0].get_legend_handles_labels()
d = dict(zip(l, h))
fig.legend([d[v] for v in labs.values() if v in d], [v for v in labs.values() if v in d],
           loc="lower center", ncol=3, fontsize=7, frameon=False)
fig.tight_layout(rect=(0, 0.1, 1, 1))
fig.savefig(os.path.join(here, "l4_roots.svg"))
fig.savefig(os.path.join(here, "l4_roots.png"), dpi=70)
