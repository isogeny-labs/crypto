# Layout only: plots values recorded by `ecbench compare` in P<b>/{wall,counted}/analysis/compare-*.json
# and the earlier sessions' wall blocks named in UNSEALED-EARLIER.md. Nothing is computed here.
# Large inputs are stored through tools/artifacts; pull first. Run from the study directory:
#   python3 figures/plot.py
import json
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

SIZES = (24, 28, 32)
W = ("0", "0.1", "1")
COL = {"0": "#2563eb", "0.1": "#7c3aed", "1": "#dc2626"}
EARLIER = "../ic_srenew_deferred_guard_20261029"

def load(path):
    return json.load(open(path))

fig, axs = plt.subplots(1, 3, figsize=(13, 3.9))

ax = axs[0]
ax.axhspan(0.89, 0.97, color="#fde68a", alpha=0.5, label="§7 prediction 0.89–0.97")
for i, b in enumerate(SIZES):
    w = load(f"P{b}/wall/analysis/compare-srenew__dg-w0.json")["wall"]
    r, (lo, hi) = w["corrected_ratio"], w["corrected_ci95"]
    ax.errorbar(i, r, yerr=[[r - lo], [hi - r]], fmt="o", color="#dc2626", capsize=3,
                label="this study (decides)" if i == 0 else None)
    for k, st in enumerate(("first-study", "v3")):
        e = load(f"{EARLIER}/{st}/P{b}/analysis/compare-srenew__dg-w0.json")["wall"]
        if e.get("corrected_ratio") is not None:
            r, (lo, hi) = e["corrected_ratio"], e["corrected_ci95"]
            ax.errorbar(i + 0.15 + 0.1 * k, r, yerr=[[r - lo], [hi - r]], fmt="o", mfc="none",
                        color="#6b7280", capsize=2, label="earlier sessions (descriptive)" if (i, k) == (1, 0) else None)
ax.axhline(1.0, color="#6b7280", lw=0.8, ls="--")
ax.set_ylim(0.78, 1.02)
ax.set_title("wall: corrected dg / srenew ns per GAE (A/A-corrected, 95%)", fontsize=8)
ax.set_xticks(range(3)); ax.set_xticklabels([f"{b} bits" for b in SIZES])

ax = axs[1]
for i, b in enumerate(SIZES):
    w = load(f"P{b}/wall/analysis/compare-srenew__dg-w0.json")["wall"]
    ax.bar(i - 0.18, w["ns_per_gae_a"], 0.35, color="#9ca3af", label="srenew" if i == 0 else None)
    ax.bar(i + 0.18, w["ns_per_gae_b"], 0.35, color="#dc2626", label="dg" if i == 0 else None)
ax.set_ylabel("ns per GAE (upper bound per op)", fontsize=8)
ax.set_title("wall: Σ solve wall / Σ total GAE, admitted L2 pairs", fontsize=8)
ax.set_xticks(range(3)); ax.set_xticklabels([f"{b} bits" for b in SIZES])

ax = axs[2]
for j, wt in enumerate(W):
    for i, b in enumerate(SIZES):
        o = load(f"P{b}/wall/analysis/compare-srenew__dg-w{wt}.json")["ops"]
        r, (lo, hi) = o["ratio_b_over_a"], o["ci95_bootstrap"]
        ax.errorbar(i + (j - 1) * 0.2, r, yerr=[[r - lo], [hi - r]], fmt="o", color=COL[wt], capsize=3,
                    label=f"w = {wt}" if i == 0 else None)
ax.axhline(1.0, color="#6b7280", lw=0.8, ls="--")
ax.set_title("counted: Σ S_dg / Σ S_srenew (wall session, 95% cluster bootstrap)", fontsize=8)
ax.set_xticks(range(3)); ax.set_xticklabels([f"{b} bits" for b in SIZES])

axs[0].legend(fontsize=7, loc="lower left")
axs[1].legend(fontsize=7, loc="upper left")
axs[2].legend(fontsize=7, loc="center left")
fig.suptitle("XP-20261029-dg-wall: deferred repeat guard vs stride-renew at N_inc, prime field, "
             "2 curves × 24 targets × 6 rounds per size, L2 on isolab-eks c8a.2xlarge", fontsize=9)
fig.tight_layout()
fig.savefig("figures/ratios.svg"); fig.savefig("figures/ratios.png", dpi=150)
