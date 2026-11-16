"""Plot the pilot's paired differences vs f0 (derived in ../../full/analyze.py
from rows/*.jsonl). No new computation beyond reading pilot_summary.json."""
import json, os, statistics as st
import matplotlib; matplotlib.use("svg")
import matplotlib.pyplot as plt
HERE = os.path.dirname(os.path.abspath(__file__))
S = json.load(open(os.path.join(HERE, "..", "..", "full", "full_summary.json")))
arms = ["f0.1", "f0.25", "f0.5", "logs_default"]
cells = list(S["cells"])
fig, axes = plt.subplots(1, 2, figsize=(10, 4.2))
w = 0.2
for k, arm in enumerate(arms):
    xs, ys, lo, hi = [], [], [], []
    for i, c in enumerate(cells):
        L = S["cells"][c]["arms"][arm]["by_lambda"]["lambda_hat"]
        xs.append(i + (k - 1.5) * w); ys.append(L["mean"])
        lo.append(L["mean"] - L["ci95_boot"][0]); hi.append(L["ci95_boot"][1] - L["mean"])
    axes[0].bar(xs, ys, w, yerr=[lo, hi], capsize=3, label=arm)
    rx = [i + (k - 1.5) * w for i in range(len(cells))]
    ry = [st.mean(S["cells"][c]["arms"][arm]["delta_row_ops"]) for c in cells]
    axes[1].bar(rx, ry, w, label=arm)
for ax in axes:
    ax.axhline(0, color="black", lw=0.8)
    ax.set_xticks(range(len(cells)))
    ax.set_xticklabels([f"{c}\n({S['cells'][c]['columns']} cols)" for c in cells])
axes[0].set_ylabel("ΔC vs f0 at lambda_hat (trials; mean, 95% bootstrap CI)")
axes[0].set_title("Total counted cost: above 0 = oversampling costs more")
axes[1].set_ylabel("Δ LA row_ops vs f0 (mean)")
axes[1].set_title("Linear-algebra work alone")
axes[0].legend(fontsize=8)
fig.suptitle("Oversampling frozen run, 30 seeds/cell, run-20261111T0602Z (local, unisolated; derived values)", fontsize=10)
fig.tight_layout()
fig.savefig(os.path.join(HERE, "full_deltas.svg"))
fig.savefig(os.path.join(HERE, "full_deltas.png"), dpi=150)
