"""Plots <run>/table.tsv (summarize.py): per cell, the CPU and word-XOR ratio
off / on for F6-IC with the pair index, and the f4 control.
Writes <run>/fig_few_candidates.{svg,png}.  python3 plot.py <run>"""
import csv
import sys

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt

run = sys.argv[1]
rows = list(csv.DictReader(open(f"{run}/table.tsv"), delimiter="\t"))
f6 = [r for r in rows if r["engine"] == "f6p"]
f4 = [r for r in rows if r["engine"] == "f4"]
fig, ax = plt.subplots(1, 2, figsize=(12, 4.2))
x = range(len(f6))
cpu = [float(r["cpu_off/on"]) for r in f6]
lo = [c - float(r["cpu_lo"]) for c, r in zip(cpu, f6)]
hi = [float(r["cpu_hi"]) - c for c, r in zip(cpu, f6)]
ax[0].bar([i - 0.2 for i in x], cpu, 0.4, yerr=[lo, hi], capsize=2, color="#2a6fbb",
          label="CPU (median of 5; bars: worst/best round pairing)")
ax[0].bar([i + 0.2 for i in x], [float(r["xor_off/on"]) for r in f6], 0.4, color="#d9822b",
          label="word XORs")
ax[0].axhline(1, color="grey", lw=0.8)
ax[0].set_xticks(list(x))
ax[0].set_xticklabels([r["cell"] for r in f6], fontsize=8)
ax[0].set_ylabel("off / on (>1: closure saves)")
ax[0].set_title("F6-IC with pair index: few-candidate closure", fontsize=10)
ax[0].legend(fontsize=7)
ax[1].bar(list(range(len(f4))), [float(r["cpu_off/on"]) for r in f4], 0.6, color="#888888")
ax[1].axhline(1, color="grey", lw=0.8)
ax[1].set_xticks(list(range(len(f4))))
ax[1].set_xticklabels([r["cell"] for r in f4], fontsize=8)
ax[1].set_ylim(0.8, 1.2)
ax[1].set_ylabel("CPU off / on")
ax[1].set_title("Control: inherited F4 (no gate), same binary", fontsize=10)
fig.suptitle(f"{run.rstrip('/').split('/')[-1]}: 9 Koblitz cells, 5 rounds × 3 reps", fontsize=10)
fig.tight_layout()
fig.savefig(f"{run}/fig_few_candidates.svg")
fig.savefig(f"{run}/fig_few_candidates.png", dpi=120)
