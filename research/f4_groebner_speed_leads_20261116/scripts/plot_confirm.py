"""Lay out the recorded rows of the confirmation run as a forest plot.

Reads data/summary.json written by examples/f4_ab_confirm.rs (median old/new
ratios with bootstrap intervals) and draws them; it computes nothing new.

    python3 scripts/plot_confirm.py reports/run-20261117T0022Z/data/summary.json reports/run-20261117T0022Z/ratios.svg
"""
import json
import sys

import matplotlib

matplotlib.use("svg")
matplotlib.rcParams["svg.fonttype"] = "path"
matplotlib.rcParams["font.size"] = 11
import matplotlib.pyplot as plt  # noqa: E402
from matplotlib.lines import Line2D  # noqa: E402

s = json.load(open(sys.argv[1]))
rows = s["rows"]
CASES = ["quad_n6_p65521", "quad_n7_p65521", "quad_n8_p65521", "quad_n9_p65521", "cubic_n4_p31",
         "cubic_n5_p31", "quart_n3_p31", "quad_n5_p31", "overdet_n6_m9_p101", "overdet_n10_m15_p65521",
         "overdet_n12_m24_p65521", "SUM_f4_ms", "kernel_x"]
COLORS = {1: "#3b6ea8", 4: "#c8553d"}


def find(pair, case, t):
    for r in rows:
        if r["pair"] == pair and r["case"] == case and r["threads"] == t:
            return r
    return None


fig, axes = plt.subplots(1, 2, figsize=(13, 7.5), gridspec_kw={"width_ratios": [3, 1]})
for ax, pair, title in ((axes[0], "DEFAULT", "shipped default vs old paths\n(f4_fp_bench 3 large; kernel_x: M4RI)"),
                        (axes[1], "AA", "A/A control\n(same binary, same settings)")):
    for i, case in enumerate(CASES):
        for dy, t in ((-0.17, 1), (0.17, 4)):
            p = "M4RI" if (case == "kernel_x" and pair == "DEFAULT") else pair
            r = find(p, case, t)
            if not r or r.get("ci_low") is None:
                continue
            y = i + dy
            ax.plot([r["ci_low"], r["ci_high"]], [y, y], color=COLORS[t], lw=1.6)
            ax.plot(r["ratio_old_over_new"], y, "o", color=COLORS[t],
                    mfc=COLORS[t] if r["significant"] else "white", ms=6)
    ax.axvline(1.0, color="#555", lw=0.8, ls="--")
    ax.set_yticks(range(len(CASES)))
    ax.set_yticklabels(CASES if ax is axes[0] else [], fontsize=10)
    ax.set_ylim(-0.7, len(CASES) - 0.3)
    ax.invert_yaxis()
    ax.set_title(title, fontsize=11)
    ax.set_xlabel("median old / median new (>1: new faster)", fontsize=10)
handles = [Line2D([], [], color=COLORS[1], marker="o", lw=1.6, label="1 thread"),
           Line2D([], [], color=COLORS[4], marker="o", lw=1.6, label="4 threads"),
           Line2D([], [], color="#333", marker="o", mfc="#333", lw=0, label="significant"),
           Line2D([], [], color="#333", marker="o", mfc="white", lw=0, label="not significant")]
fig.legend(handles=handles, loc="lower center", ncol=4, fontsize=10, frameon=False)
fig.suptitle(f"run-20261117T0022Z: native same-binary ABBA confirmation, {s['blocks']} blocks, "
             f"{s['runs']} runs, 24 pairs per cell, bootstrap 95% CI [pcc:unverified]", fontsize=11)
fig.tight_layout(rect=(0, 0.06, 1, 0.94))
fig.savefig(sys.argv[2])
print("wrote", sys.argv[2])
