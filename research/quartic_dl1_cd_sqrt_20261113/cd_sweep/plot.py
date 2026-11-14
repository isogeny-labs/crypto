#!/usr/bin/env python3
"""DL cost ratio against c_D = 2.0, by c_D and addition price (records only, from analysis.json)."""
import json, math
import matplotlib; matplotlib.use("Agg")
import matplotlib.pyplot as plt
a = json.load(open("analysis.json"))["ratios"]
QS = [262111, 524287, 1048507, 2097091, 4194271]
fig, axs = plt.subplots(1, 3, figsize=(12, 3.8), sharey=True)
for ax, w in zip(axs, ["0.0", "0.5", "1.0"]):
    for arm, c, col in (("C155", 1.55, "#2471a3"), ("C170", 1.7, "#c0392b"), ("C180", 1.8, "#27ae60")):
        for y, ls in (("DL", "-"), ("DL1", ":"), ("D", "--")):
            v = [a[f"{arm}/{y}/{q}/{w}"]["geomean"] for q in QS]
            ax.plot([q.bit_length() for q in QS], v, color=col, ls=ls, marker="o", ms=3, label=f"c_D = {c}, {y}")
    ax.axhline(1, color="#333", lw=0.8); ax.grid(alpha=0.3)
    ax.set_title(f"additions priced at {w} M-eq", fontsize=9); ax.set_xlabel("k")
axs[0].set_ylabel("cold M-eq / c_D = 2.0 (geo-mean over seeds)")
axs[2].legend(fontsize=6, ncol=1, loc="upper right")
fig.tight_layout(); fig.savefig("sweep.svg"); fig.savefig("sweep.png", dpi=200); print("ok")
