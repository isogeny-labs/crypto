import json, os
import matplotlib; matplotlib.use("Agg")
import matplotlib.pyplot as plt
here = os.path.dirname(os.path.abspath(__file__))
d = json.load(open(f"{here}/summary.json"))
arms = [("c1", "c1"), ("c1-sib8-p256", "c1-sib8-p256"), ("c1-sib8-p256-sg", "c1-sib8-p256-sg"), ("sib8-p256", "sib8-p256 (C=2)")]
cols = ["#999999", "#4477aa", "#cc3311", "#228833"]
fig, ax = plt.subplots(1, 3, figsize=(12, 4), sharey=True)
for i, (key, title) in enumerate((("w0", "GAE, w = 0"), ("w1", "GAE, w = 1"), ("bitop", "bitop/v1"))):
    for j, (arm, lab) in enumerate(arms):
        for k, bits in enumerate(("20", "22", "24")):
            r = d[bits][f"rho-neg|{arm}"][key]
            ax[i].errorbar(k + (j - 1.5) * 0.15, r[0], yerr=[[r[0] - r[1]], [r[2] - r[0]]], fmt="o", color=cols[j], label=lab if k == 0 else None, capsize=3)
    ax[i].axhline(1, color="#222", ls="--", lw=1)
    ax[i].set_xticks(range(3)); ax[i].set_xticklabels(["P20", "P22", "P24"]); ax[i].set_title(title)
ax[0].set_ylabel("arm / rho-neg (95% interval)"); ax[0].legend(fontsize=8)
fig.suptitle("Table B, C = 1 wwalk with siblings vs rho-neg (dashed = 1) [pcc:unverified]")
fig.tight_layout()
for e in ("svg", "png", "pdf"): fig.savefig(f"{here}/tableb_c1sib.{e}")
