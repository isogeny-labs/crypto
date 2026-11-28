#!/usr/bin/env python3
"""summary.json -> ratios.svg, gap.svg (plots recorded values only). cwd = study dir."""
import json
import matplotlib
matplotlib.use("Agg")
matplotlib.rcParams["svg.fonttype"] = "none"
import matplotlib.pyplot as plt
S = json.load(open("summary.json")); rows = list(S["rows"]); W = ["0", "0.1", "1"]
fig, axes = plt.subplots(1, 3, figsize=(12, 4.2), sharey=True)
for ax, w in zip(axes, W):
    for i, (key, lab, c) in enumerate([("L1", "L1 prefix / inc", "#3b6fb6"), ("L2", "L2 joint / inc", "#d07a1f"), ("both", "both / inc", "#4a9a5b")]):
        ys = [S["rows"][r]["weights"][w][key]["ratio"] for r in rows]
        lo = [y - S["rows"][r]["weights"][w][key]["ci95"][0] for r, y in zip(rows, ys)]
        hi = [S["rows"][r]["weights"][w][key]["ci95"][1] - y for r, y in zip(rows, ys)]
        ax.errorbar([j + (i - 1) * 0.22 for j in range(len(rows))], ys, yerr=[lo, hi], fmt="o", ms=4, color=c, label=lab, capsize=0)
    ax.axhline(1, color="#888", lw=0.8)
    ax.set_xticks(range(len(rows))); ax.set_xticklabels(rows); ax.set_title(f"w = {w} GAE per native op")
axes[0].set_ylabel("cost ratio (pooled, 95 % CI)"); axes[0].legend(fontsize=8, loc="lower left")
fig.suptitle("ic_ksum_prefix_joint_20261118: ratio to the -psr incumbent, 24 sessions, 1,152 pairs a row [pcc:unverified]", fontsize=10)
fig.tight_layout(); fig.savefig("ratios.svg"); fig.savefig("/tmp/claude-0/-home-user/cba6917f-779a-544b-9ecf-84e512b45b33/scratchpad/kp_ratios.png", dpi=110)
fig, ax = plt.subplots(figsize=(7, 4))
for i, w in enumerate(W):
    inc = [S["rows"][r]["weights"][w]["gap"]["inc"] for r in rows]
    best = [S["rows"][r]["weights"][w]["gap"][S["rows"][r]["registered_best"]] for r in rows]
    xs = [j + (i - 1) * 0.25 for j in range(len(rows))]
    ax.bar(xs, inc, width=0.24, color=["#c9c9c9", "#a9a9a9", "#898989"][i], label=f"incumbent, w = {w}")
    ax.scatter(xs, best, color="#b03030", s=14, zorder=3, label="registered best" if i == 0 else None)
ax.axhline(1, color="#333", lw=0.8); ax.set_xticks(range(len(rows))); ax.set_xticklabels(rows)
ax.set_ylabel("S(arm) / S(rho-neg)"); ax.set_title("Distance remaining to matched rho-neg [pcc:unverified]", fontsize=10); ax.legend(fontsize=7)
fig.tight_layout(); fig.savefig("gap.svg")
