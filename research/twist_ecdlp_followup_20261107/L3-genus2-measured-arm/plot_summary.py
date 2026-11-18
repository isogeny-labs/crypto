#!/usr/bin/env python3
"""plot_summary.py -- reads results.json ONLY and draws (1) cost vs q (log-log) for R_E, R_J, IC-G, IC-S, IC-D with the 5a band
(w = 0 .. 1 GAE per native operation) as shaded region, and (2) the ratio IC/R_E vs q with the same band and the fitted slope.
Usage: python -I plot_summary.py results.json out_prefix"""
import json, sys, math
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

res = json.load(open(sys.argv[1]))
prefix = sys.argv[2]
ks = [k for k in res["sizes"]]
qs = [res["per_size"]["k%d" % k]["q"] for k in ks]
arms = ["R_E", "R_J", "IC-G", "IC-S", "IC-D"]
colors = {"R_E": "#1b1b1b", "R_J": "#6a6a6a", "IC-G": "#d95f02", "IC-S": "#7570b3", "IC-D": "#1b9e77"}
fig, ax = plt.subplots(1, 2, figsize=(11, 4.4))
for a in arms:
    lo = [res["per_size"]["k%d" % k]["arms"][a]["cost_w0_gm"] for k in ks]
    mid = [res["per_size"]["k%d" % k]["arms"][a]["cost_w01_gm"] for k in ks]
    hi = [res["per_size"]["k%d" % k]["arms"][a]["cost_w1_gm"] for k in ks]
    ax[0].plot(qs, lo, "-o", ms=3, color=colors[a], label=a)
    ax[0].fill_between(qs, lo, hi, color=colors[a], alpha=0.15, linewidth=0)
ax[0].set_xscale("log", base=2); ax[0].set_yscale("log")
ax[0].set_xlabel("q"); ax[0].set_ylabel("operations per solve (geometric mean)")
ax[0].set_title("Cost per verified solve; shaded: unpriced native work at w = 0 .. 1 GAE")
ax[0].legend(fontsize=8)
for a, p in (("IC-G", "IC-G/R_E"), ("IC-S", "IC-S/R_E"), ("IC-D", "IC-D/R_E")):
    r0 = [res["per_size"]["k%d" % k]["ratios"][p]["w0"]["gm"] for k in ks]
    r1 = [res["per_size"]["k%d" % k]["ratios"][p]["w1"]["gm"] for k in ks]
    lo = [res["per_size"]["k%d" % k]["ratios"][p]["w0"]["lo"] for k in ks]
    hi = [res["per_size"]["k%d" % k]["ratios"][p]["w0"]["hi"] for k in ks]
    ax[1].plot(qs, r0, "-o", ms=3, color=colors[a], label=p + " (w=0)")
    ax[1].errorbar(qs, r0, yerr=[[r - l for r, l in zip(r0, lo)], [h - r for r, h in zip(r0, hi)]], fmt="none", ecolor=colors[a], elinewidth=0.8, capsize=2)
    ax[1].fill_between(qs, r0, r1, color=colors[a], alpha=0.12, linewidth=0)
ax[1].axhline(1.0, color="k", lw=0.8, ls="--")
ax[1].set_xscale("log", base=2); ax[1].set_yscale("log")
ax[1].set_xlabel("q"); ax[1].set_ylabel("cost ratio to matched rho R_E")
slope = None
for key, v in res["fits"].items():
    if key.startswith("ratio IC-D/R_E (w=0, kappa_J=1) k>=10"):
        slope = v
if slope:
    ax[1].set_title("Distance remaining; fitted slope of IC-D/R_E (k>=10): %.2f [%.2f, %.2f]" % (slope["slope"], slope["lo"], slope["hi"]), fontsize=9)
else:
    ax[1].set_title("Distance remaining (ratio < 1 would be a win)", fontsize=9)
ax[1].legend(fontsize=8)
fig.tight_layout()
fig.savefig(prefix + ".png", dpi=130)
fig.savefig(prefix + ".svg")
print("wrote", prefix + ".png", prefix + ".svg")
