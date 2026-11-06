"""Plots summary.json: legacy / v2 cost ratio per curve with its 95 % interval
(paired by target, fully priced GAE), and mean S per arm.  python3 plot.py"""
import json
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
d = json.load(open("summary.json"))
x = range(len(d))
fig, ax = plt.subplots(1, 2, figsize=(12, 4.2))
r = [o["legacy_over_v2"] for o in d]
lo = [o["legacy_over_v2"] - o["ci"][0] for o in d]
hi = [o["ci"][1] - o["legacy_over_v2"] for o in d]
cols = ["#2a6fbb" if o["ci"][0] > 1 else "#c0392b" if o["ci"][1] < 1 else "#999999" for o in d]
ax[0].bar(list(x), r, color=cols, yerr=[lo, hi], capsize=3)
ax[0].axhline(1, color="grey", lw=0.8)
ax[0].set_xticks(list(x)); ax[0].set_xticklabels([o["cell"] for o in d], fontsize=8)
ax[0].set_ylabel("S(legacy) / S(Brent v2)  (>1: Brent cheaper)")
ax[0].set_title("rho-frob, Brent vs legacy walk: blue cheaper, red dearer, grey no difference", fontsize=9)
for name, colr, off in (("rho-frob", "#888888", -0.2), ("rho-frob-v2", "#2a6fbb", 0.2)):
    ax[1].bar([i + off for i in x], [o["arms"][name]["mean_s"] for o in d], 0.4, label=name, color=colr)
ax[1].set_yscale("log")
ax[1].set_xticks(list(x)); ax[1].set_xticklabels([o["cell"] for o in d], fontsize=8)
ax[1].set_ylabel("mean S (fully priced GAE / sqrt r)")
ax[1].legend(fontsize=8)
ax[1].set_title("mean S per arm", fontsize=9)
fig.tight_layout()
fig.savefig("fig_brent.svg"); fig.savefig("fig_brent.png", dpi=120)
