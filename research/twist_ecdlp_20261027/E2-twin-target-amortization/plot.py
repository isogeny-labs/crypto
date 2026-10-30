# plot.py -- plots ONLY recorded values from summary.csv (written by e2_analyze.gp); no computation of results here.
import csv, os, sys
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

D = os.path.dirname(os.path.abspath(__file__))
os.makedirs(os.path.join(D, "plots"), exist_ok=True)
rows = list(csv.DictReader(open(os.path.join(D, "summary.csv"))))
for r in rows:
    for k, v in list(r.items()):
        try:
            r[k] = float(v)
        except ValueError:
            pass
ws = [0.0, 0.1, 1.0]
bitsl = sorted(set(int(r["bits"]) for r in rows))
col = {0.0: "#1f77b4", 0.1: "#ff7f0e", 1.0: "#2ca02c"}

def sel(b, w):
    return sorted([r for r in rows if int(r["bits"]) == b and abs(r["w"] - w) < 1e-9], key=lambda r: r["mu"])

# Fig 1: eps vs mu, per bits, per w (median, bootstrap 95% interval of the median)
fig, axs = plt.subplots(1, len(bitsl), figsize=(4.2 * len(bitsl), 3.6), sharey=True, squeeze=False)
for ax, b in zip(axs[0], bitsl):
    for w in ws:
        s = sel(b, w)
        if not s:
            continue
        x = [r["mu"] for r in s]
        y = [r["eps_med"] for r in s]
        lo = [max(0, r["eps_med"] - r["eps_lo"]) if r["eps_lo"] > -0.5 else 0 for r in s]
        hi = [max(0, r["eps_hi"] - r["eps_med"]) if r["eps_hi"] > -0.5 else 0 for r in s]
        ax.errorbar(x, y, yerr=[lo, hi], marker="o", capsize=3, color=col[w], label="w=%g GAE" % w)
    ax.axhline(0.25, color="red", ls="--", lw=1, label="H_idea bound 0.25")
    ax.axhline(1.0, color="gray", ls=":", lw=1, label="no sharing (eps=1)")
    ax.set_title("%d bits" % b)
    ax.set_xlabel("mu (N = mu p^(1/3))")
    ax.set_ylim(0, 1.4)
ax = axs[0][0]
ax.set_ylabel("eps = C_both / C_E - 1")
ax.legend(fontsize=7)
fig.suptitle("E2: cost of serving E and E' from one table, relative to E alone")
fig.tight_layout()
fig.savefig(os.path.join(D, "plots", "eps_vs_mu.png"), dpi=150)
fig.savefig(os.path.join(D, "plots", "eps_vs_mu.pdf"))

# Fig 2: positive control (same curve, two streams, one table) vs the twin (E and E', one table); ratio shared/separate
fig, axs = plt.subplots(1, 2, figsize=(8.4, 3.6), sharey=True)
for ax, b in zip(axs, [bb for bb in bitsl if bb <= 26][:2]):
    for w in ws:
        s = sel(b, w)
        s = [r for r in s if r["Rpc_n"] > 0]
        if s:
            ax.errorbar([r["mu"] for r in s], [r["Rpc_med"] for r in s],
                        yerr=[[max(0, r["Rpc_med"] - r["Rpc_lo"]) if r["Rpc_lo"] > -0.5 else 0 for r in s], [max(0, r["Rpc_hi"] - r["Rpc_med"]) if r["Rpc_hi"] > -0.5 else 0 for r in s]],
                        marker="s", capsize=3, color=col[w], label="PC1 same-curve shared, w=%g" % w)
        s2 = sel(b, w)
        ax.plot([r["mu"] for r in s2], [1 - r["sav_med"] for r in s2], marker="x", ls="--", color=col[w],
                label="twin E+E' joint / separate, w=%g" % w)
    ax.axhline(1.0, color="gray", ls=":", lw=1)
    ax.set_title("%d bits" % b)
    ax.set_xlabel("mu")
axs[0].set_ylabel("cost shared / cost separate")
axs[0].legend(fontsize=6)
fig.suptitle("E2: detector sees real sharing (PC1), the twin shows none")
fig.tight_layout()
fig.savefig(os.path.join(D, "plots", "pc1_vs_twin.png"), dpi=150)
fig.savefig(os.path.join(D, "plots", "pc1_vs_twin.pdf"))

# Fig 3: distance to matched rho (C_E / measured rho cost), per bits and mu
fig, ax = plt.subplots(figsize=(5.2, 3.8))
for w in ws:
    for mu, mk in [(0.75, "v"), (1.5, "o"), (3.0, "^")]:
        pts = [(r["bits"], r["dist_med"]) for r in rows if abs(r["w"] - w) < 1e-9 and abs(r["mu"] - mu) < 1e-9 and r["dist_n"] > 0]
        if pts:
            ax.plot([p[0] for p in pts], [p[1] for p in pts], marker=mk, color=col[w], ls="none",
                    label="w=%g mu=%g" % (w, mu))
ax.axhline(1.0, color="red", ls="--", lw=1, label="parity with rho")
ax.set_yscale("log")
ax.set_xlabel("bits of p")
ax.set_ylabel("C_E(relation phase) / C_rho (matched, measured)")
ax.legend(fontsize=6, ncol=2)
ax.set_title("E2: distance remaining to Pollard rho")
fig.tight_layout()
fig.savefig(os.path.join(D, "plots", "distance_to_rho.png"), dpi=150)
fig.savefig(os.path.join(D, "plots", "distance_to_rho.pdf"))
print("plots written")
