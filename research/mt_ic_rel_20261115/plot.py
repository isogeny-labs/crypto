#!/usr/bin/env python3
"""fig_ic_rel.svg/.png/.pdf from summary.json (the frozen analysis output). Optional args: summary path, output dir."""
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
S1 = [("ic:rel/ic:pf", "batched relation phase", "#d1495b"), ("ic:cb/ic:pf", "batched combinations", "#e3a72f"),
      ("ic:rad/ic:pf", "home-ordered inserts (cache attempt 1)", "#2a6fdb"),
      ("ic:dense/ic:pf", "load-0.6 table (cache attempt 2)", "#3a9d5d")]
S2 = [("ic:pf/best rho", "before: IC (pf) / best rho", "#999999"),
      ("ic:rel+cb/best rho", "after: IC (rel + cb) / best rho", "#222222")]
for ax, series, title in ((a, S1, "Each lever / current IC (pf)"), (b, S2, "Distance remaining to rho")):
    for j, (key, lab, col) in enumerate(series):
        off = (j - (len(series) - 1) / 2) * 0.2
        for x, c in enumerate(cells):
            d = summ["cells"][c][key]
            ax.plot([x + off] * len(d["per_instance"]), d["per_instance"], "o", color=col, alpha=0.4, ms=4)
            ax.plot([x + off], [d["median"]], "_", color=col, ms=14, mew=2.5, label=lab if x == 0 else None)
    ax.axhline(1, color="#888", lw=0.8, ls="--")
    ax.set_xticks(range(len(cells))); ax.set_xticklabels(labels, fontsize=8)
    ax.set_ylabel("whole-solve time ratio (bar median, dots instances)"); ax.set_title(title)
    ax.legend(frameon=False, fontsize=8)
fig.suptitle("IC relation phase, combinations and two cache attempts, local Xeon [pcc:unverified]", fontsize=11)
fig.tight_layout()
for ext in ("svg", "png", "pdf"):
    fig.savefig(os.path.join(out, f"fig_ic_rel.{ext}"), dpi=150 if ext == "png" else None)
