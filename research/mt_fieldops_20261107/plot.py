#!/usr/bin/env python3
"""Figure + 2-page PDF report from results/holdout/summary.json (no other inputs)."""
import json, matplotlib
matplotlib.use("Agg"); import matplotlib.pyplot as plt
from matplotlib.backends.backend_pdf import PdfPages
S = json.load(open("results/holdout/summary.json"))
cells = sorted({(s["bits"], s["T"]) for s in S}); lab = [f"{b}b\nT={t}" for b, t in cells]
col = {"rho": "#3366cc", "rho2": "#e08a00", "bsgs2": "#2a9d55"}
def fig_ratios():
    fig, axs = plt.subplots(1, 3, figsize=(13, 4.2), sharey=True)
    for ax, ci in zip(axs, [3, 10, 30]):
        for k, a in enumerate(["rho", "rho2", "bsgs2"]):
            for h, mk in zip([0, 0.1, 1], ["o", "s", "^"]):
                ss = [next(s for s in S if s["cI"] == ci and s["vs"] == a and (s["bits"], s["T"]) == c and s["h"] == h) for c in cells]
                xs = [i + (k - 1) * 0.25 + (h if h < 1 else 0.2) * 0.3 - 0.03 for i in range(len(cells))]
                ax.errorbar(xs, [s["median"] for s in ss], yerr=[[s["median"] - s["lo"] for s in ss], [s["hi"] - s["median"] for s in ss]],
                            fmt=mk, color=col[a], ms=4, lw=1, label=f"vs {a}, h={h}" if ci == 3 else None)
        ax.axhline(1, color="k", lw=0.8); ax.set_xticks(range(len(cells))); ax.set_xticklabels(lab, fontsize=8)
        ax.set_title(f"inversion I = {ci} M"); ax.grid(alpha=.3)
    axs[0].set_ylabel("IC4 / reference, cost per target (median, 95% CI)")
    fig.legend(loc="lower center", ncol=9, fontsize=7, frameon=False); fig.tight_layout(rect=(0, 0.07, 1, 1)); return fig
f = fig_ratios(); f.savefig("ratios.svg"); f.savefig("ratios.png", dpi=150)
with PdfPages("REPORT.pdf") as pdf:
    t = plt.figure(figsize=(8.27, 11.69)); t.text(0.06, 0.95, "Multi-target IC in field operations (holdout, 2026-11-07)", fontsize=14, weight="bold")
    t.text(0.06, 0.08, open("RESULTS.md").read()[:4200], fontsize=6.3, family="monospace", va="bottom", wrap=True); pdf.savefig(t)
    pdf.savefig(fig_ratios())
print("ok")
