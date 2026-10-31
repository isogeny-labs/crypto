#!/usr/bin/env python3
"""Rule v4's figures, drawn from analysis.json only (the measurement and the
analysis are native: src/bin/ic_rule_v4.rs).  Same practice as
rule/v8/report/plot.py.  Usage: plot.py <analysis.json> <out dir>"""
import json
import sys

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt

A = json.load(open(sys.argv[1]))
OUT = sys.argv[2]
S = sorted(A["sizes"], key=lambda s: s["log2_r"])
labels = [f"n={s['n']}\n2^{s['log2_r']:.1f}" for s in S]
x = list(range(len(S)))

C_V4, C_V3, C_AA, C_POS = "#1b6ca8", "#8a8a8a", "#c0762b", "#9b3d6e"

# 1. The paired cold ratio, with its controls.
fig, ax = plt.subplots(figsize=(7.2, 3.9))
for i, s in enumerate(S):
    r = s["paired_v4_over_v3"]["cold"]
    ax.errorbar(i, r["value"], yerr=[[r["value"] - r["ci95"][0]], [r["ci95"][1] - r["value"]]],
                fmt="o", color=C_V4, capsize=4, ms=7, label="v4 / v3, cold [95% bootstrap]" if i == 0 else None)
    aa = s.get("aa_v3b_over_v3")
    if aa:
        ax.errorbar(i + 0.18, aa["cold_ratio"], yerr=[[aa["cold_ratio"] - aa["ci95"][0]], [aa["ci95"][1] - aa["cold_ratio"]]],
                    fmt="s", color=C_AA, capsize=3, ms=5, label="A/A: v3 again / v3" if i == 0 else None)
    pc = s.get("lever_control_over_v3")
    if pc:
        ax.errorbar(i - 0.18, pc["cold_ratio"], yerr=[[pc["cold_ratio"] - pc["ci95"][0]], [pc["ci95"][1] - pc["cold_ratio"]]],
                    fmt="^", color=C_POS, capsize=3, ms=5, label="positive control: v3 + free relations / v3" if i == 0 else None)
ax.axhline(1.0, color="black", lw=0.8)
ax.set_xticks(x, labels)
ax.set_ylabel("cold cost, ratio to the v3 recipe")
ax.set_title("Rule v4: cold cost of one solve, v4 recipe over v3 recipe (paired targets)", fontsize=9.5)
ax.set_ylim(0.78, 1.36)
ax.legend(fontsize=7.5, loc="upper center", ncol=3, frameon=False)
ax.grid(axis="y", alpha=0.25)
fig.tight_layout()
fig.savefig(f"{OUT}/paired-cold.svg")
fig.savefig(f"{OUT}/paired-cold.png", dpi=150)
plt.close(fig)

# 2. The distance remaining to rho, both arms, both pricings.
fig, ax = plt.subplots(figsize=(7.2, 3.9))
w = 0.2
series = [
    ("v3 recipe, canonical step", "v3_canonical", C_V3, None),
    ("v4 recipe, canonical step", "v4_canonical", C_V4, None),
    ("v3 recipe, rho as built", "v3_as_built", C_V3, "//"),
    ("v4 recipe, rho as built", "v4_as_built", C_V4, "//"),
]
for k, (name, key, col, hatch) in enumerate(series):
    vals = [s["gap_ic_cold_over_rho_cold"][key]["value"] for s in S]
    lo = [v - s["gap_ic_cold_over_rho_cold"][key]["ci95"][0] for v, s in zip(vals, S)]
    hi = [s["gap_ic_cold_over_rho_cold"][key]["ci95"][1] - v for v, s in zip(vals, S)]
    ax.bar([i + (k - 1.5) * w for i in x], vals, w, yerr=[lo, hi], color=col, hatch=hatch, alpha=0.9 if hatch is None else 0.55,
           edgecolor="white", capsize=2, label=name)
ax.axhline(1.0, color="black", lw=0.8)
ax.set_yscale("log")
ax.set_xticks(x, labels)
ax.set_ylabel("cold cost, index calculus / rho (1 = parity)")
ax.set_title("Distance remaining to rho, both recipes, rho priced in the same process", fontsize=9.5)
ax.legend(fontsize=7.5, ncol=2, loc="upper left")
ax.grid(axis="y", alpha=0.25, which="both")
fig.tight_layout()
fig.savefig(f"{OUT}/gap-to-rho.svg")
fig.savefig(f"{OUT}/gap-to-rho.png", dpi=150)
plt.close(fig)
