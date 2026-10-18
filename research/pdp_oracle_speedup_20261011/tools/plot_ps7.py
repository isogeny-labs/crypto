#!/usr/bin/env python3
"""Plot recorded PS-7 values only (nothing is computed here; every plotted number is a field of
ps7_cdcl/summary_rows.jsonl, written by `pdp_ps7 summarize --json`).

  plot_ps7.py summary_rows.jsonl leaves.svg bitops.svg

leaves.svg : per rung, per arm, the recorded `leaves_over_pairs` and `conflicts_over_pairs` of the refuted targets.
bitops.svg : per rung, per arm, the recorded mean counted bit operations per refuted target, leaf and shell parts.
"""
import json
import sys

import matplotlib

matplotlib.use("svg")
import matplotlib.pyplot as plt

rows = [json.loads(l) for l in open(sys.argv[1]) if l.strip()]
rows = [r for r in rows if r["class"] == "refuted" and r["leaf"] == "group" and r["set"].startswith("l=")]
rungs = sorted({r["set"] for r in rows}, key=lambda s: int(s[2:]))
arms = ["a1", "a1u", "a2", "a2p", "a2l", "a3"]
colors = dict(zip(arms, plt.rcParams["axes.prop_cycle"].by_key()["color"]))


def get(rung, arm):
    for r in rows:
        if r["set"] == rung and r["arm"] == arm:
            return r
    return None


# ── leaves and conflicts relative to the pair count ──
fig, axes = plt.subplots(1, len(rungs), figsize=(9.2, 3.6), sharey=True)
for ax, rung in zip(axes, rungs):
    for i, arm in enumerate(arms):
        r = get(rung, arm)
        if r is None:
            continue
        ax.bar(i - 0.2, r["leaves_over_pairs"], 0.38, color=colors[arm], label="leaf visits / pairs" if i == 0 else None)
        c = r.get("conflicts_over_pairs")
        if c is not None:
            ax.bar(i + 0.2, c, 0.38, color=colors[arm], alpha=0.45, hatch="//", label="conflicts / pairs" if i == 0 else None)
    ax.set_xticks(range(len(arms)))
    ax.set_xticklabels(arms, rotation=45, fontsize=8)
    ax.set_title(f"{rung}  (n={3 * int(rung[2:])})", fontsize=9)
    ax.grid(axis="y", alpha=0.3)
axes[0].set_ylabel("mean over the refuted targets of the rung")
axes[0].legend(fontsize=7, loc="lower left")
fig.suptitle("PS-7: leaf visits (solid) and conflicts (hatched) per pair of the pair space; refuted targets", fontsize=9)
fig.text(0.01, 0.01, "bars are recorded values; a1/a1u have no conflicts; a2p has none counted by the solver (library level-0 path)", fontsize=7)
fig.tight_layout(rect=(0, 0.04, 1, 0.94))
fig.savefig(sys.argv[2])

# ── counted bit operations ──
fig, axes = plt.subplots(1, len(rungs), figsize=(9.2, 3.6), sharey=True)
for ax, rung in zip(axes, rungs):
    for i, arm in enumerate(arms):
        r = get(rung, arm)
        if r is None:
            continue
        leaf = r.get("mean_leaf_bitops", 0.0)
        shell = r.get("mean_shell_bitops64", 0.0)
        ax.bar(i, leaf, 0.7, color=colors[arm], label="leaf (PS-1 gate model)" if i == 0 else None)
        ax.bar(i, shell, 0.7, bottom=leaf, color=colors[arm], alpha=0.45, hatch="//", label="shell (64 per counted step)" if i == 0 else None)
    ax.set_yscale("log")
    ax.set_xticks(range(len(arms)))
    ax.set_xticklabels(arms, rotation=45, fontsize=8)
    ax.set_title(f"{rung}  (n={3 * int(rung[2:])})", fontsize=9)
    ax.grid(axis="y", alpha=0.3)
axes[0].set_ylabel("counted bit operations per refuted target")
axes[0].legend(fontsize=7, loc="upper left")
fig.suptitle("PS-7: counted bit operations per refuted target, leaf part (solid) and shell part (hatched, stacked)", fontsize=9)
fig.text(0.01, 0.01, "recorded means over the refuted targets of the rung; shell weight is a stated convention (64 bit operations per counted step)", fontsize=7)
fig.tight_layout(rect=(0, 0.04, 1, 0.94))
fig.savefig(sys.argv[3])
