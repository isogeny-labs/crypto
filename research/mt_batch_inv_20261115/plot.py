#!/usr/bin/env python3
"""fig_batch_inv.svg/.png/.pdf from summary.json (the frozen analysis output)."""
import json, os, sys
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
HERE = os.path.dirname(os.path.abspath(__file__))
summ = json.load(open(sys.argv[1] if len(sys.argv) > 1 else os.path.join(HERE, "summary.json")))
cells = sorted(summ["cells"], key=lambda c: tuple(map(int, c.split(":"))))
labels = [f"2^{c.split(':')[0]}\nT={c.split(':')[1]}" for c in cells]
fig, (a, b) = plt.subplots(1, 2, figsize=(12, 4.4))
S1 = [("ic_bi/ic_plain", "IC batched / IC unbatched", "#d1495b"),
      ("best_batched_rho/best_unbatched_rho", "best batched rho / best unbatched rho", "#2a6fdb")]
S2 = [("ic_plain/best_unbatched_rho", "before: IC unbatched / best unbatched rho", "#999999"),
      ("ic_bi/best_rho", "after: IC batched / best rho (all 16 keys)", "#222222")]
for ax, series, title in ((a, S1, "What batched inversion buys each method"), (b, S2, "Distance remaining to rho")):
    for j, (key, lab, col) in enumerate(series):
        off = (j - 0.5) * 0.3
        for x, c in enumerate(cells):
            d = summ["cells"][c][key]
            ax.plot([x + off] * len(d["per_instance"]), d["per_instance"], "o", color=col, alpha=0.4, ms=4)
            ax.plot([x + off], [d["median"]], "_", color=col, ms=16, mew=2.5, label=lab if x == 0 else None)
    ax.axhline(1, color="#888", lw=0.8, ls="--")
    ax.set_xticks(range(len(cells))); ax.set_xticklabels(labels, fontsize=8)
    ax.set_ylabel("whole-solve time ratio (bar median, dots instances)"); ax.set_title(title)
    ax.legend(frameon=False, fontsize=8)
fig.suptitle("Batched (Montgomery) inversion for multi-target IC and rho, local Xeon [pcc:unverified]", fontsize=11)
fig.tight_layout()
out = os.path.dirname(os.path.abspath(sys.argv[2])) if len(sys.argv) > 2 else HERE
for ext in ("svg", "png", "pdf"):
    fig.savefig(os.path.join(out, f"fig_batch_inv.{ext}"), dpi=150 if ext == "png" else None)
