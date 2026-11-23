"""Lay out the recorded A/B ratios of run-20261116T2306Z as a forest plot.

Reads data/rows.json (one row per bench, pair, case and thread count: median
old/new ratio with its paired-bootstrap 95% interval, as written by the frozen
analysis) and draws them. It computes nothing new. Usage:

    python3 scripts/plot_ratios.py data/rows.json reports/run-20261116T2306Z/ratios.svg
"""
import json
import sys

import matplotlib

matplotlib.use("svg")
matplotlib.rcParams["svg.fonttype"] = "path"  # text as outlines: no font substitution in the PDF
matplotlib.rcParams["font.size"] = 11
import matplotlib.pyplot as plt  # noqa: E402

rows = json.load(open(sys.argv[1]))
out = sys.argv[2]

F4_CASES = [
    "quad_n6_p65521", "quad_n7_p65521", "quad_n8_p65521", "quad_n9_p65521",
    "cubic_n4_p31", "cubic_n5_p31", "quart_n3_p31",
    "overdet_n10_m15_p65521", "overdet_n12_m24_p65521", "SUM_f4_ms",
]
PANELS = [
    ("f4_fp_bench", "TOTAL", "integrated binary\nvs baseline binary"),
    ("f4_fp_bench", "PAR", "f4-parallel marginal\n(old = F4_FP_PAR_OLD=1)"),
    ("f4_fp_bench", "MEM", "f4-memory marginal\n(old = F4_FP_MEM_OLD=1)"),
]
COLORS = {1: "#3b6ea8", 4: "#c8553d"}


def find(bench, pair, case, threads):
    for r in rows:
        if r["bench"] == bench and r["pair"] == pair and r["case"] == case and r["threads"] == threads:
            return r
    return None


fig, axes = plt.subplots(1, 4, figsize=(15, 8), sharex=False)
for ax, (bench, pair, title) in zip(axes[:3], PANELS):
    for i, case in enumerate(F4_CASES):
        for dy, t in ((-0.17, 1), (0.17, 4)):
            r = find(bench, pair, case, t)
            if not r:
                continue
            y = i + dy
            lo, hi = r.get("ci_low"), r.get("ci_high")
            ax.plot([lo, hi], [y, y], color=COLORS[t], lw=1.6)
            ax.plot(r["ratio_old_over_new"], y, "o", color=COLORS[t],
                    mfc=COLORS[t] if r["significant"] else "white", ms=5.5)
    ax.axvline(1.0, color="#555", lw=0.8, ls="--")
    ax.set_yticks(range(len(F4_CASES)))
    ax.set_yticklabels(F4_CASES if ax is axes[0] else [], fontsize=10)
    ax.set_ylim(-0.7, len(F4_CASES) - 0.3)
    ax.invert_yaxis()
    ax.set_title(f"{pair}\n{title}", fontsize=10)
    ax.set_xlabel("median old / median new (>1: new faster)", fontsize=10)
    ax.tick_params(axis="x", labelsize=10)

ax = axes[3]
kcases = [("koblitz_f4_kernel_bench", "kernel_x"), ("koblitz_f4_kernel_bench", "kernel_sym")]
labels = []
for i, (bench, case) in enumerate(kcases):
    labels.append(case)
    for dy, t in ((-0.17, 1), (0.17, 4)):
        r = None
        for rr in rows:
            if rr["bench"] == bench and rr["pair"] == "M4RI" and rr["threads"] == t and case in rr["case"]:
                r = rr
                break
        if not r:
            continue
        y = i + dy
        ax.plot([r["ci_low"], r["ci_high"]], [y, y], color=COLORS[t], lw=1.6)
        ax.plot(r["ratio_old_over_new"], y, "o", color=COLORS[t],
                mfc=COLORS[t] if r["significant"] else "white", ms=5.5)
ax.axvline(1.0, color="#555", lw=0.8, ls="--")
ax.set_yticks(range(len(kcases)))
ax.set_yticklabels(labels, fontsize=10)
ax.set_ylim(-0.7, len(kcases) - 0.3)
ax.invert_yaxis()
ax.set_title("M4RI\ngf2-m4ri marginal\n(old = KIC_M4RI_OLD=1)", fontsize=10)
ax.set_xlabel("median old / median new (>1: new faster)", fontsize=10)
ax.tick_params(axis="x", labelsize=10)

from matplotlib.lines import Line2D  # noqa: E402

handles = [
    Line2D([], [], color=COLORS[1], marker="o", lw=1.6, label="1 thread"),
    Line2D([], [], color=COLORS[4], marker="o", lw=1.6, label="4 threads"),
    Line2D([], [], color="#333", marker="o", mfc="#333", lw=0, label="significant (CI excludes 1, beyond A/A)"),
    Line2D([], [], color="#333", marker="o", mfc="white", lw=0, label="not significant"),
]
fig.legend(handles=handles, loc="lower center", ncol=4, fontsize=10, frameon=False)
fig.suptitle(
    "run-20261116T2306Z: serial ABBA wall-clock A/B, 4-vCPU AVX-512 VM, 16 trials per arm "
    "(f4_fp_bench '3 large'), paired bootstrap 95% CI [pcc:unverified]",
    fontsize=11,
)
fig.tight_layout(rect=(0, 0.06, 1, 0.93))
fig.savefig(out)
print("wrote", out)
