"""Plots recorded values only: rule v9's matched.json (icprog rule matched) and
rule v8's matched.json for the earlier host's figures. Computes nothing new
except the drawn ratios' error bars, which are the recorded intervals.
Usage: plot.py <rule dir holding v9/ and v8/>"""
import json, sys
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

rule = sys.argv[1]
m = json.load(open(f"{rule}/v9/matched.json"))
v8 = json.load(open(f"{rule}/v8/matched.json"))
out = f"{rule}/v9/report"
sz = sorted(m["sizes"], key=lambda s: s["log2_r"])
x = [s["log2_r"] for s in sz]

fig, ax = plt.subplots(figsize=(7.2, 4.6))
for key, col, mk, lab in [("cold_ratio_control", "#f58518", "o", "v9c: rule v8's binary, this session"),
                          ("cold_ratio_matched", "#e45756", "D", "v9: main f465f2fb, this session")]:
    y = [s[key]["value"] for s in sz]
    lo = [s[key]["value"] - s[key]["ci95"][0] for s in sz]
    hi = [s[key]["ci95"][1] - s[key]["value"] for s in sz]
    ax.errorbar(x, y, yerr=[lo, hi], fmt=mk + "-", color=col, label=lab, ms=5, lw=1.2, capsize=2)
v8s = sorted(v8["sizes"], key=lambda s: s["log2_r"])
ax.plot([s["log2_r"] for s in v8s], [s["cold_ratio_matched"]["value"] for s in v8s], "^--", color="#999999",
        ms=5, lw=1, label="rule v8 (2026-10-19, another host; quoted, not paired)")
ax.set_yscale("log")
ax.set_xlabel("log2 r (prime subgroup order)")
ax.set_ylabel("cold time, index calculus / matched rho  (above 1: rho faster)")
ax.set_title("Rule v9: the distance to a matched rho at today's main", fontsize=9)
ax.grid(True, which="both", alpha=0.25)
ax.legend(fontsize=7, loc="upper left")
fig.text(0.01, 0.005, "64 targets a size, 95% bootstrap over targets; Iso Lab EKS, AMD EPYC 9R45; source: rule/v9/matched.json, rule/v8/matched.json", fontsize=6)
fig.tight_layout(rect=(0, 0.02, 1, 1))
fig.savefig(f"{out}/cold-ratio.svg"); fig.savefig(f"{out}/cold-ratio.png", dpi=150)

fig, ax = plt.subplots(figsize=(7.2, 3.8))
xi = list(range(len(sz)))
for off, key, col, lab in [(-0.27, "cold_matched_over_control", "#e45756", "cold IC/rho, v9 / v9c"),
                            (0.0, "ic_online_matched_over_control", "#4c78a8", "IC online time, v9 / v9c"),
                            (0.27, "rho_online_control_over_matched", "#54a24b", "rho online time, v9c / v9")]:
    y = [s[key]["value"] for s in sz]
    lo = [s[key]["value"] - s[key]["ci95"][0] for s in sz]
    hi = [s[key]["ci95"][1] - s[key]["value"] for s in sz]
    ax.bar([i + off for i in xi], y, 0.26, yerr=[lo, hi], color=col, label=lab, capsize=2)
ax.axhline(1, color="k", lw=0.6)
ax.set_xticks(xi); ax.set_xticklabels([f'{s["log2_r"]}\nn={s["n"]}' for s in sz], fontsize=8)
ax.set_ylabel("ratio, paired by target"); ax.set_ylim(0.85, 1.12)
ax.legend(fontsize=7, loc="upper right", ncol=3)
ax.set_title("Same session, same 64 targets: main against rule v8's code (in the run's time unit)", fontsize=9)
fig.tight_layout(); fig.savefig(f"{out}/paired.svg"); fig.savefig(f"{out}/paired.png", dpi=150)
