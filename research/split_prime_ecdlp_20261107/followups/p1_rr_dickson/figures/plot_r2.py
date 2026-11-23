#!/usr/bin/env python3
"""Plots values recorded in run-20261107T214621Z and run-r2a-20261108T053528Z, merged by analysis/p1r2analyze
(derived_r2/p1_r2_cells.tsv). Hatched = capped job (lower bound). Reads and draws only."""
import csv, os
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
H = os.path.dirname(os.path.abspath(__file__))
rows = list(csv.DictReader(open(os.path.join(H, "..", "analysis", "derived_r2", "p1_r2_cells.tsv")), delimiter="\t"))
cells = ["D24_m3_t3", "D24_m3_t4", "D24_m3_t5", "D31_m3_t3", "D31_m3_t4", "D31_m3_t5", "K0_m3_t3", "K0_m3_t4", "K0_m3_t5", "D24_m5_t3"]
g = {(r["cell"], r["system"]): r for r in rows}
fig, ax = plt.subplots(1, 2, figsize=(15, 5.2))
for k, (col, lab) in enumerate((("D", "F4 solving degree D"), ("widest", "widest matrix (columns)"))):
    for i, (s, name) in enumerate((("rr", "RR (Riemann–Roch)"), ("t", "T (cherry tree)"), ("f", "F (full symmetric, m = 3 only)"))):
        xs, ys, hs = [], [], []
        for j, c in enumerate(cells):
            r = g.get((c, s))
            if r is None: continue
            xs.append(j + (i - 1) * 0.27); ys.append(float(r[col])); hs.append(r["completed"] != "true")
        bars = ax[k].bar(xs, ys, 0.27, label=name, color=f"C{i}")
        for b, h in zip(bars, hs):
            if h: b.set_hatch("//"); b.set_edgecolor("k")
    ax[k].set_xticks(range(len(cells))); ax[k].set_xticklabels(cells, rotation=45, fontsize=8); ax[k].set_ylabel(lab)
    if k == 1: ax[k].set_yscale("log")
    ax[k].legend(fontsize=8)
ax[0].set_title("D per cell (max over 2 seeds); hatched = capped, lower bound")
ax[1].set_title("Widest F4 matrix: RR is one degree lower than F but 2.3–3.0× wider")
fig.suptitle("P1 rounds 1 and 2 merged (run-20261107T214621Z, run-r2a-20261108T053528Z); D24/D31 Dickson torus primes, K0 Kummer p0", fontsize=9)
fig.tight_layout()
for ext, kw in (("svg", {}), ("png", {"dpi": 150})): fig.savefig(os.path.join(H, "p1_r2_cells." + ext), **kw)
print("ok")
