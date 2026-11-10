#!/usr/bin/env python3
"""Plots values recorded in run-20261107T214621Z, tabulated by analysis/p1analyze (derived/p1_cells.tsv).
Capped jobs (1.5 GiB) are drawn hatched: their values are lower bounds. No computation here."""
import csv, os
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
H = os.path.dirname(os.path.abspath(__file__))
rows = list(csv.DictReader(open(os.path.join(H, "..", "analysis", "derived", "p1_cells.tsv")), delimiter="\t"))
cells = []
for r in rows:
    if r["cell"] not in cells: cells.append(r["cell"])
get = lambda c, s: next(r for r in rows if r["cell"] == c and r["system"] == s)
fig, ax = plt.subplots(1, 2, figsize=(14, 5))
for k, (col, lab) in enumerate((("D_max", "F4 solving degree D"), ("widest_cols_max", "widest matrix (columns)"))):
    for i, sys in enumerate(("rr", "t")):
        xs, ys, hatch = [], [], []
        for j, c in enumerate(cells):
            r = get(c, sys); xs.append(j + (i - 0.5) * 0.38); ys.append(float(r[col])); hatch.append(r["completed"] != "true")
        bars = ax[k].bar(xs, ys, 0.38, label={"rr": "RR (Riemann–Roch, separator-free)", "t": "T (cherry-tree chain)"}[sys], color=["C0", "C1"][i])
        for b, h in zip(bars, hatch):
            if h: b.set_hatch("//"); b.set_edgecolor("k")
    ax[k].set_xticks(range(len(cells))); ax[k].set_xticklabels(cells, rotation=45, fontsize=8)
    ax[k].set_ylabel(lab)
    if k == 1: ax[k].set_yscale("log")
    ax[k].legend(fontsize=8)
ax[0].set_title("D per cell (max over 2 seeds); hatched = capped at 1.5 GiB, value is a lower bound")
ax[1].set_title("Widest F4 matrix; hatched = lower bound from a capped run")
fig.suptitle("P1, run-20261107T214621Z (CARD-p1-run-01): D24 = T24 = 14,680,063, D31 = T31 = 2,046,820,351 (Dickson torus); K0 = p0 = 786,433 (Kummer, prime-order curve). m5 t3 RR has no tower (t = 3): see RESULTS.", fontsize=9)
fig.tight_layout()
for ext, kw in (("svg", {}), ("png", {"dpi": 150})): fig.savefig(os.path.join(H, "p1_cells." + ext), **kw)
print("ok")
