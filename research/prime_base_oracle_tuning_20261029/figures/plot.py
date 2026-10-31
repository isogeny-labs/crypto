# Layout only: plots values recorded by `ecbench compare` (analysis/compare-*.json)
# and `ecbench band` (select/band-train-P*.json). Nothing here computes a result.
# Run from the study directory: python3 figures/plot.py
import json
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

SIZES = (20, 24, 28, 32)
WEIGHTS = ("0", "0.1", "1")
COL = {"0": "#2563eb", "0.1": "#7c3aed", "1": "#dc2626"}
INC_N = {20: 32, 24: 162, 28: 407, 32: 1024}


def ops(path):
    o = json.load(open(path))["ops"]
    return o["ratio_b_over_a"], o["ci95_bootstrap"]


def save(fig, stem):
    fig.savefig(f"figures/{stem}.svg")
    fig.savefig(f"figures/{stem}.png", dpi=150)
    plt.close(fig)


# Figure 1: holdout paired ratio of totals S_sel / S_inc with cluster-bootstrap 95% CI.
fig, ax = plt.subplots(figsize=(8, 4.2))
for i, b in enumerate(SIZES):
    for j, w in enumerate(WEIGHTS):
        r, (lo, hi) = ops(f"analysis/compare-P{b}-inc__sel-w{w}.json")
        x = i + (j - 1) * 0.18
        ax.errorbar(x, r, yerr=[[r - lo], [hi - r]], fmt="o", color=COL[w], capsize=3,
                    label=f"w = {w}" if i == 0 else None)
ax.axhline(1.0, color="#6b7280", lw=0.8, ls="--")
ax.set_xticks(range(len(SIZES)))
ax.set_xticklabels([f"{b} bits\nN {INC_N[b]}→{n}" for b, n in zip(SIZES, (45, 115, 288, 1024))])
ax.set_ylabel("Σ S_sel / Σ S_inc  (paired, 96 pairs)")
ax.set_title("Holdout: selected cell vs incumbent ic-pair, §5a band weights\n"
             "sessions ECBS1h685271327a8a / ECBS1ha436e6362605 / ECBS1h3c10dafc408f / ECBS1h1901ad2eb0e5",
             fontsize=9)
ax.legend(fontsize=8, title="GAE per native op", title_fontsize=8)
fig.tight_layout()
save(fig, "holdout_ratio")

# Figure 2: train mean S against factor-base size N for the incumbent oracle (mitm2).
fig, axs = plt.subplots(1, 4, figsize=(11, 3.4))
for ax, b in zip(axs, SIZES):
    rows = [r for r in json.load(open(f"select/band-train-P{b}.json"))["rows"]
            if r["arm"].startswith("mitm2-n")]
    rows.sort(key=lambda r: int(r["arm"].split("-n")[1]))
    ns = [int(r["arm"].split("-n")[1]) for r in rows]
    for k, w in enumerate(WEIGHTS):
        ax.plot(ns, [r["mean_s"][k] for r in rows], "o-", color=COL[w], ms=3, label=f"w = {w}")
    ax.axvline(INC_N[b], color="#6b7280", lw=0.8, ls=":")
    ax.set_xscale("log", base=2)
    ax.set_title(f"{b} bits (train, 8 runs/cell)", fontsize=9)
    ax.set_xlabel("factor-base size N")
axs[0].set_ylabel("mean S (GAE/√r)")
axs[0].legend(fontsize=7)
fig.suptitle("Train sweep, oracle mitm:negation_folded=1 (dotted: incumbent N)", fontsize=10)
fig.tight_layout()
save(fig, "train_n_sweep")
