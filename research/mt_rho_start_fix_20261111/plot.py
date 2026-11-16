#!/usr/bin/env python3
"""Figure + PDF: v2 IC4/rhobest before and after the rho restart fix (from the two summary.json files) and
RESULTS.md."""
import json, matplotlib
matplotlib.use("Agg"); import matplotlib.pyplot as plt
from matplotlib.backends.backend_pdf import PdfPages
old = json.load(open("../mt_fieldops_v2_20261111/results/holdout/summary.json"))
new = json.load(open("results/summary.json"))
cells = sorted({(s["bits"], s["T"]) for s in new})
def fig():
    f, axs = plt.subplots(1, 3, figsize=(13, 4.2), sharey=True)
    for ax, ci in zip(axs, [3, 10, 30]):
        for S, col, lab, dx in [(old, "#999999", "v2 as published (bounded restarts)", -0.12), (new, "#c0392b", "after the restart fix", 0.12)]:
            for h, mk in zip([0, 0.1, 1], ["o", "s", "^"]):
                ss = [next(s for s in S if s["vs"] == "rhobest" and s["cI"] == ci and (s["bits"], s["T"]) == c and s["h"] == h) for c in cells]
                xs = [i + dx + [0, 0.05, 0.1][[0, 0.1, 1].index(h)] for i in range(len(cells))]
                ax.errorbar(xs, [s["median"] for s in ss], yerr=[[s["median"] - s["lo"] for s in ss], [s["hi"] - s["median"] for s in ss]],
                            fmt=mk, color=col, ms=4, lw=1, label=f"{lab}, h={h}" if ci == 3 else None)
        ax.axhline(1, color="k", lw=0.8); ax.set_xticks(range(len(cells))); ax.set_xticklabels([f"{b}b\nT={t}" for b, t in cells], fontsize=8)
        ax.set_title(f"inversion I = {ci} M"); ax.grid(alpha=.3)
    axs[0].set_ylabel("IC4 / harness rhobest, cost per target"); axs[0].legend(fontsize=6, frameon=False)
    f.tight_layout(); return f
g = fig(); g.savefig("ratios.svg"); g.savefig("ratios.png", dpi=150)
with PdfPages("REPORT.pdf") as pdf:
    lines = open("RESULTS.md").read().splitlines()
    t = plt.figure(figsize=(8.27, 11.69)); t.text(0.06, 0.965, "Rho restart fix: v2 IC4 re-measured (2026-11-11)", fontsize=13, weight="bold")
    t.text(0.04, 0.94, "\n".join(lines), fontsize=5.6, family="monospace", va="top"); pdf.savefig(t); pdf.savefig(fig())
print("ok")
