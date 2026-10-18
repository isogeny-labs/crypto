#!/usr/bin/env python3
"""Plots recorded values only: raw/summary.json (b13 summary) -> graph.svg/png."""
import json, sys
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
D = sys.argv[1]
j = json.load(open(f"{D}/raw/summary.json"))
rules = ["base_ref", "min2", "la", "per8", "per16", "laper16"]
cols = {"min2": "#c0392b", "la": "#2471a3", "per8": "#7d6608", "per16": "#6c3483", "laper16": "#117a65"}
rs = [16, 32, 128, 2048]
fig, axs = plt.subplots(1, 3, figsize=(13, 4.6), sharey=True)
for ax, bits in zip(axs, (40, 44, 48)):
    for k, a in enumerate(r for r in rules if r != "base_ref"):
        xs, ys, lo, hi, cen, bmin = [], [], [], [], [], []
        for c in j["cells"]:
            if c["bits"] == bits and c["theta"] == bits // 4 and c["arm"] == a:
                xs.append(rs.index(c["r"]) + (k - 2) * 0.07)
                g = c["gain_full"]; ys.append(g[0]); lo.append(g[0] - g[1]); hi.append(g[2] - g[0])
                cen.append(c["solved"] < c["targets"]); bmin.append(min(c["band"]))
        o = sorted(range(len(xs)), key=lambda i: xs[i])
        xs = [xs[i] for i in o]; ys = [ys[i] for i in o]; lo = [lo[i] for i in o]; hi = [hi[i] for i in o]
        cen = [cen[i] for i in o]; bmin = [bmin[i] for i in o]
        ax.plot(xs, ys, "-", color=cols[a], lw=1, alpha=.6)
        ax.errorbar(xs, ys, yerr=[lo, hi], fmt="none", ecolor=cols[a], elinewidth=1, capsize=2, alpha=.7)
        for x, y, ce in zip(xs, ys, cen):
            ax.plot(x, y, "o", ms=6, mfc="white" if ce else cols[a], mec=cols[a], label=a if x == xs[0] else None)
        ax.plot(xs, bmin, "v", ms=4, color=cols[a], alpha=.45)
    for y, s, ls in ((1.0, "1.0 (un-quotiented Teske walk)", "-"), (1.35, "1.35 (preregistered reach)", "--"), (1.414, "sqrt 2 (ideal)", ":")):
        ax.axhline(y, color="k", lw=.8, ls=ls)
        if bits == 44: ax.text(-0.45, y + .01, s, ha="left", va="bottom", fontsize=6.5)
    ax.set_xticks(range(4)); ax.set_xticklabels([str(r) for r in rs])
    ax.set_xlabel("jump-table size r"); ax.set_title(f"N ~ 2^{bits}, 64 targets, DP bits {bits//4}", fontsize=10)
    ax.grid(alpha=.25); ax.set_ylim(0.15, 1.95)
axs[0].set_ylabel("gain over plain Teske walk (JAC30, native work 0)")
axs[0].legend(fontsize=7, loc="upper left", title="escape rule", title_fontsize=7)
fig.suptitle("Item 13: fully charged gain of the negation walk, 95% paired bootstrap interval\n"
             "hollow marker = some target unsolved at the budget cap (gain is an upper bound); small triangle = worst of the nine pricing points",
             fontsize=9)
fig.tight_layout(rect=(0, 0, 1, .9))
fig.savefig(f"{D}/graph.svg"); fig.savefig(f"{D}/graph.png", dpi=150)
