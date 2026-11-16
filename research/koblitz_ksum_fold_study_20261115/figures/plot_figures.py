"""Lays out recorded values only (results.json, written by analyze.py). Computes no result."""
import json, pathlib
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
HERE = pathlib.Path(__file__).resolve().parent; ROOT = HERE.parent
R = json.loads((ROOT / "results.json").read_text())
W = ["0.0", "0.1", "1.0"]; WL = {"0.0": "w=0", "0.1": "w=0.1", "1.0": "w=1"}
# Fig 1: R_w(candidate / strictest lazy incumbent), pooled with CI and per replicate
fig, axs = plt.subplots(1, 2, figsize=(9, 3.6), sharey=False)
for ax, n in zip(axs, ("23", "29")):
    c = R["curves"][n]
    for i, w in enumerate(W):
        o = c["weights"][w]; p = o["h1_pooled"]
        ax.errorbar([i], [p["ratio"]], yerr=[[p["ratio"] - p["lo"]], [p["hi"] - p["ratio"]]], fmt="s", color="#1f4e79", capsize=4, label="pooled, 95% CI" if i == 0 else None)
        ax.scatter([i - 0.18, i, i + 0.18], [x["ratio"] for x in o["h1_per_rep"]], s=14, color="#c55a11", zorder=3, label="replicates" if i == 0 else None)
        aa = o["aa_pooled"]; ax.fill_between([i - 0.3, i + 0.3], aa["lo"], aa["hi"], color="#999", alpha=.35, label="A/A 95%" if i == 0 else None)
        ax.annotate(o["strictest_incumbent"], (i, 0.02), xycoords=("data", "axes fraction"), ha="center", fontsize=7)
    ax.axhline(1, color="k", lw=.8); ax.set_xticks(range(3)); ax.set_xticklabels([WL[w] for w in W])
    ax.set_yscale("log"); ax.set_title(f"K1 n={n}", fontsize=10); ax.set_ylabel("cost(kf3-c1g-sr) / cost(strictest lazy)")
axs[0].legend(fontsize=7, loc="upper left"); fig.tight_layout()
for e in ("svg", "png"): fig.savefig(HERE / f"fig1-ratio.{e}", dpi=150)
# Fig 2: mean S per arm (log), w=0.1
fig, axs = plt.subplots(1, 2, figsize=(9, 3.8))
order = ["rho-strong", "rho-neg", "lazy", "kf3-c1g-sr", "kf3-c1g-w", "k1"]
for ax, n in zip(axs, ("23", "29")):
    m = R["curves"][n]["mean_S"]; arms = [a for a in m if not a.endswith("-full") and a != "rho-strong-aa"]
    arms.sort(key=lambda a: m[a]["0.1"])
    for k, wv, col in ((0, "0", "#9dc3e6"), (1, "0.1", "#1f4e79"), (2, "1", "#c55a11")):
        ax.barh([i + (k - 1) * 0.27 for i in range(len(arms))], [m[a][wv] for a in arms], height=0.26, color=col, label=f"w={wv}")
    ax.set_yticks(range(len(arms))); ax.set_yticklabels(arms, fontsize=7); ax.set_xscale("log"); ax.set_title(f"K1 n={n}: mean S (band-priced)", fontsize=10); ax.invert_yaxis()
axs[0].legend(fontsize=7); fig.tight_layout()
for e in ("svg", "png"): fig.savefig(HERE / f"fig2-S.{e}", dpi=150)
