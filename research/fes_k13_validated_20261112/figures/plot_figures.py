"""Plot recorded values only (no computation) from the validator's recompute.json.
Usage: python3 plot_figures.py   (run from this directory; needs matplotlib)."""
import json
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

R = json.load(open("../runpod-a1/validation/recompute.json"))
A = R["arms"]
SID = R["session_id"]
COL = {"ic-descent-fes-f2": "#1b6ca8", "ic-descent-inherited-f4": "#c0561b",
       "ic-pairtable-mitm2": "#6a5acd", "rho-negation": "#555555", "rho-sfstrong-v2": "#2a7f3e"}

def save(fig, name):
    fig.savefig(name + ".svg")
    fig.savefig(name + ".png", dpi=150)

# Figure 1: mean S per arm, 95 % intervals (log-scale cluster t over 8 workloads)
order = ["rho-sfstrong-v2", "rho-negation", "ic-descent-fes-f2", "ic-descent-inherited-f4", "ic-pairtable-mitm2"]
fig, ax = plt.subplots(figsize=(8, 4.8))
for i, a in enumerate(order):
    m = A[a]["mean_s"]; lo, hi = A[a]["mean_s_ci95_log_cluster_t"]
    ax.errorbar(i, m, yerr=[[m - lo], [hi - m]], fmt="o", color=COL[a], capsize=5, ms=7)
    ax.annotate(f"{m:.2f}", (i, m), textcoords="offset points", xytext=(10, -3), fontsize=9)
ax.axhline(A["rho-sfstrong-v2"]["mean_s"], color=COL["rho-sfstrong-v2"], ls="--", lw=1,
           label="matched rho (rho-sfstrong-v2), mean S 2.584")
ax.set_yscale("log"); ax.set_xticks(range(len(order)))
ax.set_xticklabels([o.replace("ic-descent-", "IC ").replace("ic-pairtable-", "IC ") for o in order], rotation=15, fontsize=9)
ax.set_xlim(-0.5, len(order) - 0.5)
ax.set_ylabel("mean S = GAE / sqrt(r)  (charged price, log scale)")
ax.set_title("Mean S per arm on icv1-f2m13-t181-515ee569 (r = 2003)\n48 measured runs per arm; bars: 95 % interval, log-scale cluster t over 8 workloads", fontsize=10)
ax.legend(fontsize=8, loc="upper left"); ax.grid(True, which="both", axis="y", alpha=0.25)
fig.text(0.01, 0.01, f"Session {SID}; values from validator recompute.json [pcc:unverified]. Lower S is cheaper.", fontsize=7)
fig.tight_layout(rect=(0, 0.03, 1, 1)); save(fig, "fig1-mean-s-per-arm"); plt.close(fig)

# Figure 2: band
ws = ["0", "0.1", "1"]
arms = ["ic-descent-fes-f2", "ic-descent-inherited-f4", "rho-sfstrong-v2"]
fig, ax = plt.subplots(figsize=(8, 4.8)); wd = 0.26
for j, a in enumerate(arms):
    vals = [A[a]["band_mean_s"][w] for w in ws]
    xs = [k + (j - 1) * wd for k in range(3)]
    ax.bar(xs, vals, wd, color=COL[a], label=a)
    for x, v in zip(xs, vals):
        ax.annotate(f"{v:.4g}", (x, v), textcoords="offset points", xytext=(0, 3), ha="center", fontsize=8)
ax.set_yscale("log"); ax.set_xticks(range(3)); ax.set_xticklabels([f"w = {w}" for w in ws])
ax.set_ylabel("band mean S (log scale)"); ax.set_xlabel("price w (GAE) per unpriced native operation")
ax.set_title("Band: mean S at w = 0, 0.1, 1 GAE per native operation\nfes-f2 vs inherited-f4 order flips at w = 0 (15.177 vs 15.174): unresolved inside the band", fontsize=10)
ax.legend(fontsize=8, loc="upper left"); ax.grid(True, which="both", axis="y", alpha=0.25)
fig.text(0.01, 0.01, f"Session {SID}; values from validator recompute.json [pcc:unverified]. 48 runs per arm.", fontsize=7)
fig.tight_layout(rect=(0, 0.03, 1, 1)); save(fig, "fig2-band"); plt.close(fig)
