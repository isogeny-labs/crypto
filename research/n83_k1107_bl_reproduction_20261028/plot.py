#!/usr/bin/env python3
"""Lay out recorded values from the Rust analyzer's analysis.json (run 4).

Rendering only: every plotted number is read from <run-dir>/analysis.json;
nothing is computed here beyond placing points.
usage: plot.py <run-dir> <out-dir> <run-label>
"""
import json
import sys
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

ARMS = [
    ("ic", "IC K1107/root11 dynamic", "#7f3c8d"),
    ("rho_plain_fold", "rho, empty table (positive control)", "#80868b"),
    ("bl_a_time_matched", "precomputed rho (a), 650 s", "#11a579"),
    ("bl_a2_wall_equal", "precomputed rho (a'), = IC setup wall", "#e68310"),
    ("bl_b_memory_matched", "precomputed rho (b), = IC max RSS", "#3969ac"),
]
TICKS = {"ic": "IC\nK1107", "rho_plain_fold": "rho\nempty", "bl_a_time_matched": "precomp\n(a) 650 s",
         "bl_a2_wall_equal": "precomp\n(a') IC wall", "bl_b_memory_matched": "precomp\n(b) IC RSS"}
PAIRS = [("ic_over_bl_a", "(a) 650 s", "#11a579"), ("ic_over_bl_a2", "(a') IC wall", "#e68310"),
         ("ic_over_bl_b", "(b) IC RSS", "#3969ac")]


def main():
    run, out, label = Path(sys.argv[1]), Path(sys.argv[2]), sys.argv[3]
    out.mkdir(parents=True, exist_ok=True)
    data = json.loads((run / "analysis.json").read_text())
    fig, (ax, bx) = plt.subplots(1, 2, figsize=(11, 4.6), gridspec_kw={"width_ratios": [3, 2]})
    for k, (key, name, color) in enumerate(ARMS):
        arm = data["arms"][key]
        values = arm["online_ms"]
        xs = [k + (i - (len(values) - 1) / 2) * 0.025 for i in range(len(values))]
        ax.scatter(xs, values, s=14, color=color, alpha=0.75, label=name, zorder=3)
        lo, hi = arm["median_ms_ci95"]
        med = arm["median_ms"]
        ax.errorbar([k + 0.32], [med], yerr=[[med - lo], [hi - med]], fmt="D", color=color, ms=6,
                    capsize=4, zorder=4)
    ax.set_yscale("log")
    ax.set_xticks(range(len(ARMS)))
    ax.set_xticklabels([TICKS[a[0]] for a in ARMS], fontsize=8)
    ax.set_ylabel("online interval per target (ms, log scale)")
    ax.set_title(f"online time per target, n = {data['targets']} points", fontsize=9)
    ax.grid(axis="y", which="both", alpha=0.25)

    for k, (key, name, color) in enumerate(PAIRS):
        p = data["paired"][key]
        ratios = p["per_target_ratio"]
        xs = [k + (i - (len(ratios) - 1) / 2) * 0.025 for i in range(len(ratios))]
        bx.scatter(xs, ratios, s=12, color=color, alpha=0.6, zorder=3)
        med = p["median_of_paired_ratios"]
        lo, hi = p["median_of_paired_ratios_ci95"]
        bx.errorbar([k + 0.3], [med], yerr=[[med - lo], [hi - med]], fmt="D", color=color, ms=6,
                    capsize=4, zorder=4)
        bx.annotate(f"{med:.1f}\n[{lo:.2f}, {hi:.1f}]", (k + 0.36, med), fontsize=7.5, va="center")
    bx.axhline(1.0, color="black", lw=0.8, ls="--")
    bx.set_yscale("log")
    bx.set_xlim(-0.5, len(PAIRS) - 0.2)
    bx.set_xticks(range(len(PAIRS)))
    bx.set_xticklabels([p[1] for p in PAIRS], fontsize=8)
    bx.set_ylabel("paired IC / precomputed rho online (log)")
    bx.set_title("per-target ratio; diamond = median, 95% CI\nabove the dashed line: precomputed rho faster", fontsize=9)
    bx.grid(axis="y", which="both", alpha=0.25)

    handles, labels = ax.get_legend_handles_labels()
    fig.legend(handles, labels, loc="lower center", ncol=3, frameon=False, fontsize=8.5)
    fig.suptitle(f"N83 a=1 Koblitz, r = 8569786107849059, {label}; L0 shared VM", fontsize=10)
    fig.tight_layout(rect=(0, 0.1, 1, 0.95))
    fig.savefig(out / "online-and-ratio.svg")
    fig.savefig(out / "online-and-ratio.png", dpi=150)


if __name__ == "__main__":
    main()
