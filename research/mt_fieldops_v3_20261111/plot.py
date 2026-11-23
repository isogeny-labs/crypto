#!/usr/bin/env python3
"""Figure + PDF report from results/holdout/summary.json and RESULTS.md (no other inputs)."""
import json, matplotlib
matplotlib.use("Agg"); import matplotlib.pyplot as plt
from matplotlib.backends.backend_pdf import PdfPages
S = json.load(open("results/holdout/summary.json"))
cells = sorted({(s["bits"], s["T"]) for s in S}); lab = [f"{b}b\nT={t}" for b, t in cells]
col = {"rhobest": "#c0392b", "bsgs2": "#2a9d55", "ic4": "#7a5195"}
def fig_ratios():
    fig, axs = plt.subplots(2, 3, figsize=(15, 8), sharey=True)
    for row, cand in enumerate(["ic5", "ic4"]):
        refs = [a for a in ["rhobest", "bsgs2", "ic4"] if a != cand]
        for ax, ci in zip(axs[row], [3, 10, 30]):
            for k, a in enumerate(refs):
                for h, mk in zip([0, 0.1, 1], ["o", "s", "^"]):
                    ss = [next(s for s in S if s["cand"] == cand and s["cI"] == ci and s["vs"] == a
                               and (s["bits"], s["T"]) == c and s["h"] == h) for c in cells]
                    xs = [i + (k - 1) * 0.25 + [0, 0.07, 0.14][[0, 0.1, 1].index(h)] for i in range(len(cells))]
                    ax.errorbar(xs, [s["median"] for s in ss],
                                yerr=[[s["median"] - s["lo"] for s in ss], [s["hi"] - s["median"] for s in ss]],
                                fmt=mk, color=col[a], ms=3.5, lw=1, label=f"{cand} vs {a}, h={h}" if ci == 3 else None)
            ax.axhline(1, color="k", lw=0.8); ax.set_xticks(range(len(cells))); ax.set_xticklabels(lab, fontsize=7)
            ax.set_title(f"{cand}, inversion I = {ci} M", fontsize=10); ax.grid(alpha=.3)
        axs[row][0].set_ylabel(f"{cand} / reference, cost per target")
        axs[row][0].legend(fontsize=6, ncol=2, frameon=False)
    fig.tight_layout(); return fig
f = fig_ratios(); f.savefig("ratios.svg"); f.savefig("ratios.png", dpi=150)
with PdfPages("REPORT.pdf") as pdf:
    lines = open("RESULTS.md").read().splitlines(); per = 95
    for k in range(0, len(lines), per):
        t = plt.figure(figsize=(8.27, 11.69))
        if k == 0: t.text(0.06, 0.965, "Multi-target IC v3: k-term table (ic5), 22-32 bit groups", fontsize=13, weight="bold")
        t.text(0.04, 0.94, "\n".join(lines[k:k + per]), fontsize=5.6, family="monospace", va="top"); pdf.savefig(t)
    pdf.savefig(fig_ratios())
print("ok")
