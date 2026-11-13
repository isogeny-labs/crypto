#!/usr/bin/env python3
"""Lays out recorded values of phase3/analysis_m{3,2}.json as SVG; computes no result.
Usage: plot_phase3.py analysis_m3.json analysis_m2.json outdir"""
import json, sys, math
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
A = {3: json.load(open(sys.argv[1])), 2: json.load(open(sys.argv[2]))}; out = sys.argv[3]
plt.rcParams.update({"font.size": 9, "svg.fonttype": "none"})
cols = {"S1": "#1f77b4", "S2": "#ff7f0e", "S3": "#2ca02c"}

# 1. per-class pooled yield / predicted, both m, with binomial SE whiskers sqrt(rel)/rel
fig, axs = plt.subplots(2, 1, figsize=(7.2, 4.6), sharex=True)
for ax, m in zip(axs, (3, 2)):
    for i, c in enumerate(A[m]["classes"]):
        y = c["yield_over_predicted"]; se = y / math.sqrt(c["relations"])
        ax.errorbar(i, y, yerr=2 * se, fmt="o", ms=3, color=cols[c["stratum"]], elinewidth=0.8)
    ax.axhline(A[m]["class_yield_over_predicted"]["mean"], color="k", lw=0.6, ls="--")
    t1 = A[m]["primary_tests"][0]["p"]
    ax.set_ylabel(f"m = {m}: yield / predicted")
    ax.set_title(f"m = {m}: T1 Kruskal-Wallis p = {t1:.2f}", fontsize=8)
for st, col in cols.items():
    axs[0].plot([], [], "o", color=col, label=st)
axs[0].legend(ncol=3, fontsize=7, loc="lower right")
axs[1].set_xlabel("class (classes.json order; S1, S2, S3 = 1, 2, 3+ endomorphism rings), +-2 binomial SE")
fig.suptitle("IC relation yield per isogeny class, 48 prime-order classes of F_p, p = 2^31 - 1", fontsize=9)
fig.tight_layout(); fig.savefig(f"{out}/class_yield.svg"); plt.close(fig)

# 2. class yield against class size (T2), both m
fig, axs = plt.subplots(1, 2, figsize=(7.2, 2.9), sharey=True)
for ax, m in zip(axs, (3, 2)):
    c = A[m]["classes"]
    hs = [x for x in c]
    ax.scatter([math.log2(next(cc for cc in json.load(open(sys.argv[4]))["classes"] if cc["id"] == x["id"])["h"]) for x in hs] if len(sys.argv) > 4 else range(len(hs)),
               [x["yield_over_predicted"] for x in hs], c=[cols[x["stratum"]] for x in hs], s=12)
    t = A[m]["primary_tests"][1]
    ax.set_title(f"m = {m}: T2 Spearman {t['rho']:+.3f}, p {t['p_perm']:.2f}", fontsize=8)
    ax.set_xlabel("log2 H (curves in class)")
axs[0].set_ylabel("class yield / predicted")
fig.tight_layout(); fig.savefig(f"{out}/yield_vs_H.svg"); plt.close(fig)

# 3. positive control and within-class p-values
fig, axs = plt.subplots(1, 3, figsize=(7.2, 2.7))
for m, col in ((3, "#555"), (2, "#bbb")):
    r = [c["planted_pc_over_main"] for c in A[m]["classes"] if c["planted_pc_over_main"] is not None]
    axs[0].hist(r, bins=12, range=(1.0, 1.4), color=col, alpha=0.8, label=f"m = {m}")
axs[0].axvline(1.2, color="r", lw=0.8); axs[0].axvspan(1.12, 1.28, color="r", alpha=0.08)
axs[0].set_xlabel("planted +20% arm: pc / main"); axs[0].set_ylabel("classes"); axs[0].legend(fontsize=7)
for ax, m in zip(axs[1:], (3, 2)):
    ps = [c["icc_perm_p"] for c in A[m]["classes"] if c["icc_perm_p"] is not None and c["curves"] >= 2]
    ax.hist(ps, bins=10, range=(0, 1), color="#888", edgecolor="w"); ax.axhline(len(ps) / 10, color="r", lw=0.8)
    t = A[m]["primary_tests"][5]
    ax.set_xlabel(f"m = {m}: within-class ICC p"); ax.set_title(f"KS p {t['p']:.2f}, n {len(ps)}", fontsize=8)
fig.tight_layout(); fig.savefig(f"{out}/controls_within.svg"); plt.close(fig)
print("ok")
