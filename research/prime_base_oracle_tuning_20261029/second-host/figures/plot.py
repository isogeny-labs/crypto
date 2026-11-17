# Layout only: plots ratios recorded by `ecbench compare --weight` for the original holdout
# session (../analysis/compare-P20-*.json, 96 pairs) and the second-host session
# (analysis/compare-*.json, 240 pairs). Run from second-host/: python3 figures/plot.py
import json
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

W = ("0", "0.1", "1")
def ops(p):
    o = json.load(open(p))["ops"]
    return o["ratio_b_over_a"], o["ci95_bootstrap"]
fig, axs = plt.subplots(1, 2, figsize=(10, 3.8))
for ax, (orig, sec, title) in zip(axs, (
        ("../analysis/compare-P20-inc__sel-w{}.json", "analysis/compare-inc__sel-w{}.json", "Σ S_sel / Σ S_inc"),
        ("../analysis/compare-P20-rho-neg__sel-w{}.json", "analysis/compare-rho-neg__sel-w{}.json", "Σ S_sel / Σ S_rho-neg"))):
    for j, (tmpl, lab, col) in enumerate(((orig, "original host, 2 rounds (96 pairs)", "#2563eb"),
                                         (sec, "second host (EKS c8a, L2), 5 rounds (240 pairs)", "#dc2626"))):
        for i, w in enumerate(W):
            r, (lo, hi) = ops(tmpl.format(w))
            ax.errorbar(i + (j - 0.5) * 0.2, r, yerr=[[r - lo], [hi - r]], fmt="o", color=col, capsize=3,
                        label=lab if i == 0 else None)
    ax.set_xticks(range(3)); ax.set_xticklabels([f"w = {w}" for w in W])
    ax.set_title(title + " (cluster-bootstrap 95%)", fontsize=9)
    if "inc" in title: ax.axhline(1.0, color="#6b7280", lw=0.8, ls="--")
axs[0].legend(fontsize=7)
fig.suptitle("holdout-P20, icv1-fp20-tm565-6d7d1d32: ECBS1h685271327a8a vs second-host ECBS1h517504d65837", fontsize=9)
fig.tight_layout()
fig.savefig("figures/second_host_ratios.svg"); fig.savefig("figures/second_host_ratios.png", dpi=150)
