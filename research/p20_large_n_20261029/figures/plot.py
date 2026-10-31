# Layout only: plots the paired ratios recorded by `ecbench compare --weight` in
# analysis/compare-*.json (576 pairs each). Run from the study directory:
#   python3 figures/plot.py
import json
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

W = ("0", "0.1", "1")
def ops(a, b, w):
    o = json.load(open(f"analysis/compare-{a}__{b}-w{w}.json"))["ops"]
    return o["ratio_b_over_a"], o["ci95_bootstrap"]

fig, axs = plt.subplots(1, 2, figsize=(10, 3.8))
for j, (b, lab, col) in enumerate((("n32", "N 32 (pre-XP-20261016 baseline)", "#6b7280"),
                                   ("n64", "N 64 (candidate)", "#dc2626"),
                                   ("n90", "N 90 (descriptive)", "#7c3aed"))):
    for i, w in enumerate(W):
        r, (lo, hi) = ops("inc", b, w)
        axs[0].errorbar(i + (j - 1) * 0.2, r, yerr=[[r - lo], [hi - r]], fmt="o", color=col, capsize=3,
                        label=lab if i == 0 else None)
axs[0].axhline(1.0, color="#2563eb", lw=0.8, ls="--", label="N 45 baseline")
axs[0].set_title("Σ S_arm / Σ S_inc(N 45), cluster-bootstrap 95%", fontsize=9)
for j, (b, lab, col) in enumerate((("inc", "N 45 (baseline)", "#2563eb"), ("n64", "N 64", "#dc2626"))):
    for i, w in enumerate(W):
        r, (lo, hi) = ops("rho-neg", b, w)
        axs[1].errorbar(i + (j - 0.5) * 0.2, r, yerr=[[r - lo], [hi - r]], fmt="o", color=col, capsize=3,
                        label=lab if i == 0 else None)
axs[1].set_title("Σ S_arm / Σ S_rho-neg: distance remaining to matched rho", fontsize=9)
for ax in axs:
    ax.set_xticks(range(3)); ax.set_xticklabels([f"w = {w}" for w in W]); ax.legend(fontsize=7)
fig.suptitle("XP-20261029, 20-bit prime field, 3 fresh curves × 32 targets × 6 rounds (L2, isolab-eks)", fontsize=9)
fig.tight_layout()
fig.savefig("figures/ratios.svg"); fig.savefig("figures/ratios.png", dpi=150)
