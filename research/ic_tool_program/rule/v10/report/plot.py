"""Plots recorded values only: rule v10's matched.json (icprog rule matched),
p1p2.json (p1p2.sh, jq) and rule v9's matched.json for the earlier host's
figures. Computes nothing: every drawn value and error bar is a recorded one.
Usage: plot.py <rule dir holding v10/ and v9/>"""
import json, sys
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

rule = sys.argv[1]
m = json.load(open(f"{rule}/v10/matched.json"))
p = json.load(open(f"{rule}/v10/p1p2.json"))
v9 = json.load(open(f"{rule}/v9/matched.json"))
out = f"{rule}/v10/report"
sz = sorted(m["sizes"], key=lambda s: s["log2_r"])
x = [s["log2_r"] for s in sz]
label = {f'k{s["a"]}n{s["n"]}': s for s in sz}

fig, ax = plt.subplots(figsize=(7.2, 4.6))
for key, col, mk, lab in [("cold_ratio_control", "#f58518", "o", "v10c: main ca1a767b, this session"),
                          ("cold_ratio_matched", "#4c78a8", "D", "v10: single-word selection, this session")]:
    y = [s[key]["value"] for s in sz]
    lo = [s[key]["value"] - s[key]["ci95"][0] for s in sz]
    hi = [s[key]["ci95"][1] - s[key]["value"] for s in sz]
    ax.errorbar(x, y, yerr=[lo, hi], fmt=mk + "-", color=col, label=lab, ms=5, lw=1.2, capsize=2)
v9s = sorted(v9["sizes"], key=lambda s: s["log2_r"])
ax.plot([s["log2_r"] for s in v9s], [s["cold_ratio_matched"]["value"] for s in v9s], "^--", color="#999999",
        ms=5, lw=1, label="rule v9 (Iso Lab EKS, AMD EPYC; quoted, not paired)")
for n, lo_, hi_ in [(47, 3.3, 3.5), (57, 3.9, 4.1)]:
    s = next(s for s in sz if s["n"] == n)
    ax.plot([s["log2_r"]] * 2, [lo_, hi_], color="#54a24b", lw=6, alpha=0.45,
            label="P3 predicted range (from v9's 4.08, 4.73)" if n == 47 else None)
ax.axhline(1, color="k", lw=0.6)
ax.set_yscale("log")
ax.set_xlabel("log2 r (prime subgroup order)")
ax.set_ylabel("cold time, index calculus / matched rho\n(distance remaining; 1 = parity)")
ax.set_title("Rule v10: the cold distance to a matched rho, before and after", fontsize=9)
ax.grid(True, which="both", alpha=0.25)
ax.legend(fontsize=7, loc="upper left")
fig.text(0.01, 0.005, "64 targets a size, 95% bootstrap over targets; local L0, Intel Xeon 2.1 GHz, isolated_bench; "
         "source: rule/v10/matched.json, rule/v9/matched.json", fontsize=6)
fig.tight_layout(rect=(0, 0.02, 1, 1))
fig.savefig(f"{out}/cold-ratio.svg"); fig.savefig(f"{out}/cold-ratio.png", dpi=150)

fig, ax = plt.subplots(figsize=(7.2, 4.0))
xi = list(range(len(sz)))
y = [s["cold_matched_over_control"]["value"] for s in sz]
lo = [s["cold_matched_over_control"]["value"] - s["cold_matched_over_control"]["ci95"][0] for s in sz]
hi = [s["cold_matched_over_control"]["ci95"][1] - s["cold_matched_over_control"]["value"] for s in sz]
ax.bar([i - 0.2 for i in xi], y, 0.38, yerr=[lo, hi], color="#4c78a8", capsize=2,
       label="cold IC/rho, v10 / v10c (95% bootstrap)")
pz = {r["size"]: r["p2"] for r in p["sizes"]}
keys = [f'k{s["a"]}n{s["n"]}' for s in sz]
y2 = [pz[k]["select_ratio_median"] for k in keys]
lo2 = [pz[k]["select_ratio_median"] - pz[k]["select_ratio_q25"] for k in keys]
hi2 = [pz[k]["select_ratio_q75"] - pz[k]["select_ratio_median"] for k in keys]
ax.bar([i + 0.2 for i in xi], y2, 0.38, yerr=[lo2, hi2], color="#e45756", capsize=2,
       label="select + select_projection, v10 / v10c (median, quartiles)")
ax.axhline(1, color="k", lw=0.6)
ax.axhline(0.25, color="#e45756", lw=0.8, ls=":", label="P2: at most 0.25")
ax.axhline(0.90, color="#4c78a8", lw=0.8, ls="--", label="falsified above 0.90 at n47")
for n, a, b in [(47, 0.81, 0.86), (57, 0.82, 0.87)]:
    i = next(i for i, s in enumerate(sz) if s["n"] == n)
    ax.plot([i - 0.2] * 2, [a, b], color="#54a24b", lw=6, alpha=0.6,
            label="P3 predicted paired range" if n == 47 else None)
ax.set_xticks(xi); ax.set_xticklabels([f'n={s["n"]}\nlog2 r {s["log2_r"]}' for s in sz], fontsize=8)
ax.set_ylabel("ratio, paired by target (below 1: v10 cheaper)")
ax.set_ylim(0, 1.12)
ax.legend(fontsize=6.5, loc="lower right", ncol=2)
ax.set_title("Same session, same 68 rows a size: the change against its control", fontsize=9)
fig.text(0.01, 0.005, "source: rule/v10/matched.json (cold), rule/v10/p1p2.json (selection)", fontsize=6)
fig.tight_layout(rect=(0, 0.02, 1, 1)); fig.savefig(f"{out}/paired.svg"); fig.savefig(f"{out}/paired.png", dpi=150)
