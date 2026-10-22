"""Plots the recorded ratios of reports/<run>/<host>/{inh,gf2}.tsv (written by
summarize.py) for every host of one run: left, inherited F4 / F6-IC CPU ratio
of each switched-off arm over the default per cell; right, GF(2) wall ratio
arm / t4 per case.  Writes reports/<run>/fig_arm64.{svg,png}.

    python3 plot.py reports/<run>
"""
import csv
import glob
import os
import sys

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt

run = sys.argv[1]
hosts = sorted(d for d in glob.glob(f"{run}/*/") if os.path.exists(f"{d}inh.tsv"))
fig, axes = plt.subplots(len(hosts), 2, figsize=(13, 4.2 * len(hosts)), squeeze=False)
for row, host in enumerate(hosts):
    name = os.path.basename(host.rstrip("/"))
    inh = list(csv.DictReader(open(f"{host}inh.tsv"), delimiter="\t"))
    ax = axes[row][0]
    labels = [f"{r['cell']}\n{r['engine']}" for r in inh]
    x = range(len(inh))
    for off, (arm, colour) in zip((-0.2, 0.2), (("noeager", "#2a6fbb"), ("nogate", "#d9822b"))):
        med = [float(r[f"{arm}/on"]) for r in inh]
        lo = [m - float(r[f"{arm}/on_lo"]) for m, r in zip(med, inh)]
        hi = [float(r[f"{arm}/on_hi"]) - m for m, r in zip(med, inh)]
        ax.errorbar([i + off for i in x], med, yerr=[lo, hi], fmt="o", ms=4, color=colour,
                    capsize=2, label=f"{arm} / on (median; bars: worst and best round pairing)")
    ax.axhline(1.0, color="grey", lw=0.8)
    ax.set_xticks(list(x))
    ax.set_xticklabels(labels, fontsize=6.5)
    ax.set_ylabel("CPU time ratio, arm / default")
    ax.set_title(f"{name}: inherited F4 (f4) and F6-IC (f6p), 9 cells", fontsize=10)
    ax.legend(fontsize=7)
    ax = axes[row][1]
    gf2 = list(csv.DictReader(open(f"{host}gf2.tsv"), delimiter="\t")) if os.path.exists(f"{host}gf2.tsv") else []
    groups = sorted({(r["tool"], r["threads"]) for r in gf2})
    colours = {"t2": "#2a6fbb", "t3": "#4caf50", "reuse": "#d9822b"}
    pos = 0
    ticks = []
    for tool, th in groups:
        rows = [r for r in gf2 if r["tool"] == tool and r["threads"] == th]
        for arm, colour in colours.items():
            vals = [float(r["arm/t4"]) for r in rows if r["arm"] == arm]
            ax.scatter([pos] * len(vals), vals, s=12, color=colour, label=arm if pos < 3 else None)
            pos += 1
        ticks.append((pos - 2, f"{tool}\n{th} thr"))
        pos += 1
    ax.axhline(1.0, color="grey", lw=0.8)
    ax.set_xticks([t for t, _ in ticks])
    ax.set_xticklabels([s for _, s in ticks], fontsize=8)
    ax.set_ylabel("wall time ratio, arm / t4 (median of 3 rounds)")
    ax.set_title(f"{name}: GF(2) Gray-code tables, one point per bench case", fontsize=10)
    ax.legend(fontsize=7)
fig.suptitle(f"Arm64 defaults, {os.path.basename(run.rstrip('/'))}: ratio > 1 means the default is faster", fontsize=11)
fig.tight_layout()
fig.savefig(f"{run}/fig_arm64.svg")
fig.savefig(f"{run}/fig_arm64.png", dpi=130)
