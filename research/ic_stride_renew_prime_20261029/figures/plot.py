# Layout only: plots the paired ratios recorded by `ecbench compare --weight` in
# P<b>/analysis/compare-*.json (288 pairs each). Run from the study directory:
#   python3 figures/plot.py
import json, os
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

W = ("0", "0.1", "1")
COL = {"0": "#2563eb", "0.1": "#7c3aed", "1": "#dc2626"}
SIZES = [b for b in ("P20", "P24", "P28", "P32") if os.path.exists(f"{b}/analysis/compare-walk__srenew-w0.json")]
def ops(d, a, b, w):
    o = json.load(open(f"{d}/analysis/compare-{a}__{b}-w{w}.json"))["ops"]
    return o["ratio_b_over_a"], o["ci95_bootstrap"]

fig, axs = plt.subplots(1, 2, figsize=(10, 3.8))
for j, w in enumerate(W):
    for i, d in enumerate(SIZES):
        r, (lo, hi) = ops(d, "walk", "srenew", w)
        axs[0].errorbar(i + (j - 1) * 0.2, r, yerr=[[r - lo], [hi - r]], fmt="o", color=COL[w], capsize=3,
                        label=f"w = {w}" if i == 0 else None)
axs[0].axhline(1.0, color="#6b7280", lw=0.8, ls="--", label="walk baseline")
axs[0].set_title("Σ S_srenew / Σ S_walk, cluster-bootstrap 95%", fontsize=9)
for j, (a, lab, col, mk) in enumerate((("walk", "walk (baseline)", "#6b7280", "s"), ("srenew", "stride-renew", "#dc2626", "o"))):
    for i, d in enumerate(SIZES):
        r, (lo, hi) = ops(d, "rho-neg", a, "0")
        axs[1].errorbar(i + (j - 0.5) * 0.2, r, yerr=[[r - lo], [hi - r]], fmt=mk, color=col, capsize=3,
                        label=lab if i == 0 else None)
axs[1].set_yscale("log")
axs[1].set_title("Σ S_arm / Σ S_rho-neg at w = 0: distance remaining to matched rho", fontsize=9)
for ax in axs:
    ax.set_xticks(range(len(SIZES))); ax.set_xticklabels([s.replace("P", "") + " bits" for s in SIZES]); ax.legend(fontsize=7)
fig.suptitle("XP-20261029 stride-renew vs walk, prime field, 2 fresh curves × 24 targets × 6 rounds per size (L2, isolab-eks)", fontsize=9)
fig.tight_layout()
fig.savefig("figures/ratios.svg"); fig.savefig("figures/ratios.png", dpi=150)
