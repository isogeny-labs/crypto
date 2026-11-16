"""Plot recorded pooled yield ratios from binary_edwards.json (no computation
beyond reading the recorded ratio and its bootstrap interval).
Usage: python plot_edwards.py binary_edwards.json fig_edwards_yield.svg"""
import json
import sys

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt

src, out = sys.argv[1], sys.argv[2]
rows = json.load(open(src))["pooled_ratios"]
NS = [9, 10, 11, 12, 13, 14]
TARGET = 0.25
COL = {"s2": "#1b6ca8", "s4": "#c0562a"}
plt.rcParams.update({"font.size": 9, "svg.fonttype": "none"})
fig, ax = plt.subplots(1, 2, figsize=(9.2, 3.7), sharex=True)


def series(metric, arm):
    xs, ys, lo, hi = [], [], [], []
    for n in NS:
        r = [x for x in rows if x["metric"] == metric and x["group"] == f"n={n}"
             and x["arm"] == arm and abs(x["target_H_plain"] - TARGET) < 1e-9]
        if r:
            xs.append(n); ys.append(r[0]["ratio"]); lo.append(r[0]["ci_lo"]); hi.append(r[0]["ci_hi"])
    return xs, ys, lo, hi


def draw(a, metric, arm, ls, marker, dx):
    xs, ys, lo, hi = series(metric, arm)
    key = arm[:2]
    a.errorbar([x + dx for x in xs], ys, yerr=[[y - l for y, l in zip(ys, lo)], [h - y for y, h in zip(ys, hi)]],
               color=COL[key], ls=ls, marker=marker, ms=4, lw=1.2, capsize=2,
               label=f"{key} ({'U = c' if metric.endswith('/c') else 'U = c+1'})")


for arm in ("s2_quot", "s4_quot"):
    draw(ax[0], "lambda/c", arm, "-", "o", -0.06 if arm == "s2_quot" else 0.06)
    draw(ax[0], "lambda/c+1", arm, "--", "s", -0.06 if arm == "s2_quot" else 0.06)
ax[0].axhline(1.0, color="#444", lw=0.8)
ax[0].set_title("Quotient sweep (one point per M-coset)")
ax[0].set_ylabel("yield ratio to plain base (relations per lookup per column)")
ax[0].set_ylim(0.6, 1.8)
for arm in ("s2_full", "s4_full"):
    draw(ax[1], "lambda/c", arm, "-", "o", -0.06 if arm == "s2_full" else 0.06)
ax[1].axhline(0.5, color=COL["s2"], lw=0.8, ls=":")
ax[1].axhline(0.25, color=COL["s4"], lw=0.8, ls=":")
ax[1].text(9.05, 0.515, "law 1/2", color=COL["s2"], fontsize=8)
ax[1].text(9.05, 0.265, "law 1/4", color=COL["s4"], fontsize=8)
ax[1].set_title("Full sweep of F (U = c)")
ax[1].set_ylim(0.1, 0.8)
for a in ax:
    a.set_xlabel("n  (curves over F_{2^n}; n = 13, 14: scale check, 2-3 curves)")
    a.set_xticks(NS)
    a.grid(alpha=0.25)
    a.legend(frameon=False, fontsize=8, loc="upper right" if a is ax[0] else "upper left")
fig.suptitle("Binary Edwards, pair-table oracle, plain H = 0.25 target: pooled ratio and 95% bootstrap CI over bases", fontsize=9)
fig.tight_layout()
fig.savefig(out)
