#!/usr/bin/env python3
"""fig_ic_tb.svg/.png/.pdf from summary.json (the frozen analysis output). Optional args: summary path, output dir."""
import json, os, sys
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
HERE = os.path.dirname(os.path.abspath(__file__))
summ = json.load(open(sys.argv[1] if len(sys.argv) > 1 else os.path.join(HERE, "summary.json")))
out = sys.argv[2] if len(sys.argv) > 2 else HERE
cells = sorted(summ["cells"], key=lambda c: tuple(map(int, c.split(":"))))
labels = [f"2^{c.split(':')[0]}\nT={c.split(':')[1]}" for c in cells]
fig, (a, b) = plt.subplots(1, 2, figsize=(12, 4.4))
S1 = [("ic:ib/ic:pf", "IC: batched inserts / current", "#e3a72f"), ("ic:z/ic:pf", "IC: zero-encoded slots / current", "#3a9d5d"),
      ("ic:z+ib/ic:pf", "IC: both / current (primary)", "#d1495b"), ("rho z1/rho z0", "best rho: zero-encoded / classic", "#2a6fdb")]
S2 = [("ic:pf/rho z0", "before: current IC / best rho", "#999999"),
      ("ic:z+ib/rho", "after: IC (both) / best rho (either encoding)", "#222222")]
for ax, series, title in ((a, S1, "What each table-build lever buys"), (b, S2, "Distance remaining to rho")):
    for j, (key, lab, col) in enumerate(series):
        off = (j - (len(series) - 1) / 2) * 0.18
        for x, c in enumerate(cells):
            d = summ["cells"][c][key]
            ax.plot([x + off] * len(d["per_instance"]), d["per_instance"], "o", color=col, alpha=0.4, ms=4)
            ax.plot([x + off], [d["median"]], "_", color=col, ms=14, mew=2.5, label=lab if x == 0 else None)
    ax.axhline(1, color="#888", lw=0.8, ls="--")
    ax.set_xticks(range(len(cells))); ax.set_xticklabels(labels, fontsize=8)
    ax.set_ylabel("whole-solve time ratio (bar median, dots instances)"); ax.set_title(title)
    ax.legend(frameon=False, fontsize=8)
fig.suptitle("IC table build: batched inserts and zero-encoded slots, local Xeon [pcc:unverified]", fontsize=11)
fig.tight_layout()
for ext in ("svg", "png", "pdf"):
    fig.savefig(os.path.join(out, f"fig_ic_tb.{ext}"), dpi=150 if ext == "png" else None)
