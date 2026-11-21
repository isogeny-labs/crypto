#!/usr/bin/env python3
"""Lays out recorded values of phase5/analysis_{rho,ic3,ic2}.json as SVG; computes nothing.
Usage: plot_phase5.py analysis_rho.json analysis_ic3.json analysis_ic2.json outdir"""
import json, sys
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
A = [json.load(open(f)) for f in sys.argv[1:4]]; out = sys.argv[4]
plt.rcParams.update({"font.size": 9, "svg.fonttype": "none"})
fig, axs = plt.subplots(1, 4, figsize=(7.4, 2.6), gridspec_kw={"width_ratios": [1, 1, 1, 0.9]})
names = {"rho": "rho: ns/GAE", "ic3": "IC m=3: ns/attempt", "ic2": "IC m=2: ns/attempt"}
for ax, a in zip(axs[:3], A):
    ps = [c["friedman"]["p"] for c in a["per_class"] if c["friedman"]["p"] == c["friedman"]["p"]]
    ax.hist(ps, bins=10, range=(0, 1), color="#888", edgecolor="w"); ax.axhline(len(ps) / 10, color="r", lw=0.8)
    ax.set_title(f"{names[a['kind']]}\nKS p = {a['T1_friedman_ks_uniform']['p']:.1e}", fontsize=8)
    ax.set_xlabel("within-class Friedman p")
axs[0].set_ylabel("classes")
probs = [a["S1_iso_vs_base"]["prob_iso_pair_closer"] for a in A]
axs[3].bar(range(3), probs, color="#555"); axs[3].axhline(0.5, color="r", lw=0.8, ls="--"); axs[3].axhline(0.75, color="k", lw=0.6, ls=":")
axs[3].set_xticks(range(3)); axs[3].set_xticklabels(["rho", "m=3", "m=2"]); axs[3].set_ylim(0, 1)
axs[3].set_title("S1: P(iso pair closer)\n0.5 chance, 0.75 bar", fontsize=8)
fig.tight_layout(); fig.savefig(f"{out}/phase5_summary.svg"); plt.close(fig)
print("ok")
