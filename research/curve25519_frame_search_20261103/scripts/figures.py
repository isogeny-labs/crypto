#!/usr/bin/env python3
"""Plot recorded values only: confirm/k{16..24}.json and analysis/summary.json.
Writes figures/coverage_distributions.{svg,png} and figures/gain_vs_t.{svg,png}.
Run: python3 scripts/figures.py  (needs matplotlib)."""
import json, os
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.lines import Line2D
from matplotlib.patches import Patch

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
KS = [16, 18, 20, 22, 24]
TS = [5, 6, 7, 8]
FOOT = ("Source: confirm/k{16,18,20,22,24}.json, analysis/summary.json; "
        "commit: frozen protocol 6a800b1513")
C = {"bits": "#d95f02", "edw": "#1b9e77", "sel": "#7570b3",
     "tr": "#9ecae1", "mo": "#08519c"}

summ = json.load(open(os.path.join(ROOT, "analysis/summary.json")))
cell = {(c["k"], c["t"]): c for c in summ["cells"]}
conf = {k: json.load(open(os.path.join(ROOT, f"confirm/k{k}.json"))) for k in KS}

def cov(k, t, fam):
    return [r["covered"] for r in conf[k]["rows"] if r["t"] == t and r["family"] == fam]

out = os.path.join(ROOT, "figures")
os.makedirs(out, exist_ok=True)
def save(fig, name):
    fig.savefig(os.path.join(out, name + ".svg"))
    fig.savefig(os.path.join(out, name + ".png"), dpi=150)
    plt.close(fig)

# Figure 1
fig, axs = plt.subplots(len(KS), len(TS), figsize=(14, 12.5), squeeze=False)
for i, k in enumerate(KS):
    for j, t in enumerate(TS):
        ax = axs[i][j]
        tr, mo = cov(k, t, "translate"), cov(k, t, "mobius")
        lo, hi = min(tr + mo), max(tr + mo)
        bins = 40 if hi - lo > 40 else range(lo, hi + 2)
        ax.hist(tr, bins=bins, range=(lo, hi + 1) if bins == 40 else None,
                color=C["tr"], alpha=0.8)
        ax.hist(mo, bins=bins, range=(lo, hi + 1) if bins == 40 else None,
                histtype="step", color=C["mo"], lw=1.3)
        c = cell[(k, t)]
        b = cov(k, t, "bits_lo0")[0]
        e = cov(k, t, "edwards_bits")[0]
        for v, col, ls in ((b, C["bits"], "-"), (e, C["edw"], "--"),
                           (c["selected_covered"], C["sel"], "-.")):
            ax.axvline(v, color=col, ls=ls, lw=1.8)
        ax.tick_params(labelsize=7)
        if i == 0: ax.set_title(f"t = {t}", fontsize=10)
        if j == 0: ax.set_ylabel(f"k = {k}\nframes", fontsize=9)
        if i == len(KS) - 1: ax.set_xlabel("covered", fontsize=9)
handles = [Patch(color=C["tr"], alpha=0.8, label="translate frames (n=2000)"),
           Line2D([], [], color=C["mo"], lw=1.3, label="mobius frames (n=2000, outline)"),
           Line2D([], [], color=C["bits"], lw=1.8, label="bits_lo0 (Weierstrass x, lo=0)"),
           Line2D([], [], color=C["edw"], lw=1.8, ls="--", label="edwards_bits (Edwards y)"),
           Line2D([], [], color=C["sel"], lw=1.8, ls="-.", label="selected frame (argmax J)")]
fig.suptitle("Exact m=2 subgroup coverage by coordinate frame (Curve25519 family)",
             fontsize=13, y=0.99)
fig.legend(handles=handles, loc="upper center", ncol=5, fontsize=8.5,
           bbox_to_anchor=(0.5, 0.965), frameon=False)
fig.tight_layout(rect=(0, 0.025, 1, 0.94))
fig.text(0.5, 0.008, FOOT, ha="center", fontsize=7.5, color="#444")
save(fig, "coverage_distributions")

# Figure 2
fig, axs = plt.subplots(1, 2, figsize=(11, 4.8), sharey=True)
cols = plt.cm.viridis([0.05, 0.3, 0.55, 0.75, 0.92])
for ax, key, lab in ((axs[0], "gain_vs_bits_lo0", "selected / bits_lo0"),
                     (axs[1], "gain_vs_mean", "selected / mean translate frame")):
    for k, col in zip(KS, cols):
        ts = [t for t in range(4, 9) if (k, t) in cell]
        ax.plot(ts, [cell[(k, t)][key] for t in ts], "o-", color=col, label=f"k = {k}")
    ax.axhline(1.5, color="k", ls="--", lw=1)
    ax.axhline(1.0, color="gray", ls=":", lw=1)
    ax.set_yscale("log")
    ax.set_xlabel("t"); ax.set_xticks(range(4, 9)); ax.set_title(lab, fontsize=10)
    ax.grid(alpha=0.25, which="both")
    ax.text(4.02, 1.5 * 1.04, "P3 threshold 1.5", fontsize=8)
    ax.text(4.02, 1.0 * 1.04, "1 (no gain)", fontsize=8, color="gray")
axs[0].set_ylabel("coverage ratio (log scale)")
f = cell[(18, 7)]["gain_vs_bits_lo0"]
axs[0].annotate(f"P3 fail: k=18, t=7\n{f:.3f}", xy=(7, f), xytext=(5.6, 1.1),
                fontsize=8.5, arrowprops=dict(arrowstyle="->"),
                bbox=dict(boxstyle="round", fc="white", ec="#c00"))
axs[0].plot([7], [f], "o", ms=11, mfc="none", mec="#c00", mew=1.8)
axs[1].legend(fontsize=8, loc="upper right")
fig.suptitle("Gain of the selected frame in exact m=2 subgroup coverage vs t", fontsize=12)
fig.tight_layout(rect=(0, 0.04, 1, 0.95))
fig.text(0.5, 0.008, FOOT, ha="center", fontsize=7.5, color="#444")
save(fig, "gain_vs_t")
