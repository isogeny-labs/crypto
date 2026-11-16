"""Plots <run>/table.tsv (summarize_parallel.py): wall-time speed-up of the
parallel split per cell, and the extra counted work it costs.
Writes <run>/fig_parallel.{svg,png}.  python3 plot_parallel.py <run>"""
import csv
import sys

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt

run = sys.argv[1]
rows = list(csv.DictReader(open(f"{run}/table.tsv"), delimiter="\t"))
x = range(len(rows))
w = [float(r["wall_d0/d2"]) for r in rows]
fig, ax = plt.subplots(figsize=(9, 4))
ax.bar([i - 0.2 for i in x], w, 0.4, color="#2a6fbb",
       yerr=[[a - float(r["wall_lo"]) for a, r in zip(w, rows)],
             [float(r["wall_hi"]) - a for a, r in zip(w, rows)]], capsize=2,
       label="wall time per target, sequential / parallel (median of 5; bars: worst/best pairing)")
ax.bar([i + 0.2 for i in x], [float(r["xor_d2/d0"]) for r in rows], 0.4, color="#d9822b",
       label="counted work (word XORs), parallel / sequential")
ax.axhline(1, color="grey", lw=0.8)
ax.set_xticks(list(x))
ax.set_xticklabels([f"{r['cell']}\n{r['hits']} hit" for r in rows], fontsize=8)
ax.set_ylabel("ratio")
ax.set_title(f"Parallel split, depth 2, 4 threads, inherited F4 ({run.rstrip('/').split('/')[-1]})", fontsize=10)
ax.legend(fontsize=7)
fig.tight_layout()
fig.savefig(f"{run}/fig_parallel.svg")
fig.savefig(f"{run}/fig_parallel.png", dpi=120)
