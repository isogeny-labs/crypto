#!/usr/bin/env python3
"""Figures for research/hec_bitop_20261020. Plots recorded values only.

Reads analysis/analysis.json (examples/hec_bitop_analyse.rs), the per-phase
totals recorded in raw/g4.jsonl, and round seven's committed rows
(experiments/hyperelliptic_sparse_solve_r7/fbwalk_rows.json). Computes nothing
beyond selecting and plotting those values.

    /tmp/report-venv/bin/python research/hec_bitop_20261020/figures/plots.py
"""
import json
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

HERE = Path(__file__).resolve().parent
STUDY = HERE.parent
ROOT = STUDY.parent.parent
A = json.loads((STUDY / "analysis" / "analysis.json").read_text())
R7 = json.loads((ROOT / "experiments/hyperelliptic_sparse_solve_r7/fbwalk_rows.json").read_text())
COLOR = {2: "#2a78d6", 3: "#1baf7a", 4: "#eb6834"}
plt.rcParams.update({"font.size": 9, "axes.spines.top": False, "axes.spines.right": False,
                     "axes.grid": True, "grid.color": "#e4e4e4", "grid.linewidth": 0.6})


def save(fig, name):
    fig.tight_layout()
    fig.savefig(HERE / f"{name}.svg")
    fig.savefig(HERE / f"{name}.png", dpi=150)
    plt.close(fig)


# 1. IC/rho per row against log2 N, bitop/v1 with intervals; round seven's
#    in-situ ratio (8-bit rho walk) as the "was" marks.
fig, ax = plt.subplots(figsize=(7.2, 4.4))
for g in (2, 3, 4):
    rows = [r for r in A["rows"] if r["genus"] == g]
    xs = [r["log2_n"] for r in rows]
    ys = [r["ic_over_rho"] for r in rows]
    lo = [r["ic_over_rho"] - r["ic_over_rho_ci95"][0] for r in rows]
    hi = [r["ic_over_rho_ci95"][1] - r["ic_over_rho"] for r in rows]
    f = A["fits"][str(g)]
    ax.errorbar(xs, ys, yerr=[lo, hi], fmt="-o", ms=4, lw=1.6, capsize=2, color=COLOR[g],
                label=f"genus {g}, bitop/v1 (slope {f['slope']:+.3f} [{f['slope_ci95'][0]:+.3f}, {f['slope_ci95'][1]:+.3f}])")
    old = sorted((r["N"], sorted(r["ratio_B"])[len(r["ratio_B"]) // 2]) for r in R7 if r["genus"] == g)
    import math
    ax.plot([math.log2(n) for n, _ in old], [v for _, v in old], "x", ms=6, color=COLOR[g], alpha=0.6)
ax.plot([], [], "x", color="#555", label="round seven, in-situ unit, 8-bit rho walk (was)")
ax.axhline(1, color="#555", lw=1, ls="--")
ax.text(10.2, 1.05, "rho", color="#555", fontsize=8)
ax.set_yscale("log")
ax.set_xlabel("log₂ N (subgroup order)")
ax.set_ylabel("IC / rho, mean per solve")
ax.set_title("Hyperelliptic IC against full-width rho, both priced in bitop/v1 (5 IC × 40 rho runs per row)", fontsize=9)
ax.legend(fontsize=7, frameon=False, loc="upper right")
save(fig, "ratio_vs_n")

# 2. Where the work goes on the genus-4, p = 61 row: per-phase bitop/v1,
#    summed over runs and divided by the run count (recorded totals).
row = next(json.loads(line) for line in (STUDY / "raw" / "g4.jsonl").read_text().splitlines()
           if json.loads(line)["p"] == 61)
phases = ["precompute", "walk", "oracle", "linear_algebra", "verification"]
fig, ax = plt.subplots(figsize=(7.2, 2.8))
for i, arm in enumerate(("ic", "rho")):
    runs = row[arm]["runs"]
    left = 0.0
    for j, ph in enumerate(phases):
        v = float(row[arm]["phases_summed_over_runs"][ph]["bitops"]) / runs
        ax.barh(i, v, left=left, color=plt.cm.tab10(j), edgecolor="white",
                label=ph.replace("_", " ") if i == 0 else None)
        left += v
ax.set_yticks([0, 1], ["index calculus", "rho (full width)"])
ax.set_xlabel("bitop/v1 per solve (mean over runs)")
ax.set_title("Genus 4, p = 61, N = 16,790,591: phases of one solve", fontsize=9)
ax.legend(fontsize=7, frameon=False, ncol=5, loc="upper center", bbox_to_anchor=(0.5, -0.35))
save(fig, "phases_g4_p61")

# 3. The p = 61 row under the primary measure and its reported variants.
r61 = next(r for r in A["rows"] if r["genus"] == 4 and r["p"] == 61)
labels = ["primary\nbitop/v1", "excluding\nvalidation", "post hoc: no rho\nprecompute",
          "post hoc: IC\noracle ×2", "in-situ unit,\nfull-width rho", "round seven,\nin-situ raw", "scoreboard: round\nseven, rho renormalised"]
vals = [r61["ic_over_rho"], r61["ic_over_rho_excluding_validation"],
        r61["post_hoc_ic_over_rho_without_rho_precompute"], r61["post_hoc_ic_over_rho_oracle_doubled"],
        r61["in_situ_s_ic_over_s_rho"], 0.15, 0.24]
fig, ax = plt.subplots(figsize=(7.2, 3.2))
bars = ax.bar(range(len(vals)), vals, color=["#eb6834"] + ["#f3a47f"] * 4 + ["#aaaaaa"] * 2)
ax.errorbar([0], [vals[0]], yerr=[[vals[0] - r61["ic_over_rho_ci95"][0]], [r61["ic_over_rho_ci95"][1] - vals[0]]],
            fmt="none", color="#333", capsize=4)
for b, v in zip(bars, vals):
    top = r61["ic_over_rho_ci95"][1] if b is bars[0] else v
    ax.text(b.get_x() + b.get_width() / 2, top + 0.006, f"{v:.3f}", ha="center", fontsize=8)
ax.set_xticks(range(len(vals)), labels, fontsize=7)
ax.set_ylabel("IC / rho")
ax.set_ylim(0, 0.3)
ax.set_title("Genus 4, p = 61: the ratio under every reported variant (rho = 1 is off the top)", fontsize=9)
save(fig, "p61_variants")
print("figures written")
