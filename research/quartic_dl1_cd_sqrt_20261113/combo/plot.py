#!/usr/bin/env python3
"""DL and DL1 cost against the frozen c_D = 2.0: c_D = 1.7 alone and with the square-root chain (analysis.json)."""
import json
import matplotlib; matplotlib.use("Agg")
import matplotlib.pyplot as plt
a = json.load(open("analysis.json"))["ratios"]
QS = [262111, 524287, 1048507, 2097091, 4194271]
ks = [q.bit_length() for q in QS]
fig, axs = plt.subplots(1, 2, figsize=(10, 3.8), sharey=True)
for ax, y in zip(axs, ("DL", "DL1")):
    for num, col, lab in (("C170", "#2471a3", "c_D = 1.7"), ("B170", "#c0392b", "c_D = 1.7 + chain")):
        for w, ls in (("0.0", "-"), ("1.0", "--")):
            ax.plot(ks, [a[f"{num}/BASE/{y}/{q}/{w}"]["geomean"] for q in QS], color=col, ls=ls, marker="o", ms=3,
                    label=f"{lab}, additions {'free' if w == '0.0' else 'at 1 M-eq'}")
    ax.axhline(1, color="#333", lw=0.8); ax.grid(alpha=0.3); ax.set_title(y, fontsize=9); ax.set_xlabel("k")
axs[0].set_ylabel("cold M-eq / c_D = 2.0 (geo-mean over seeds)")
axs[1].legend(fontsize=7)
fig.tight_layout(); fig.savefig("combo.svg"); fig.savefig("combo.png", dpi=200); print("ok")
