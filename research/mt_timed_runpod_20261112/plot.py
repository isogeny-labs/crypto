#!/usr/bin/env python3
"""Figure + PDF from results/holdout/summary.json and RESULTS.md."""
import json, matplotlib
matplotlib.use("Agg"); import matplotlib.pyplot as plt
from matplotlib.backends.backend_pdf import PdfPages
S = json.load(open("results/holdout/summary.json"))
def fig():
    f, axs = plt.subplots(1, 2, figsize=(13, 4.4))
    for b, col, dx in (("std", "#c0392b", -0.12), ("fast", "#3366cc", 0.12)):
        cs = S[b]["cells"]; xs = range(len(cs))
        axs[0].errorbar([x + dx for x in xs], [c["ic_rho_solve"]["median"] for c in cs],
                        yerr=[[c["ic_rho_solve"]["median"] - c["ic_rho_solve"]["lo"] for c in cs], [c["ic_rho_solve"]["hi"] - c["ic_rho_solve"]["median"] for c in cs]],
                        fmt="o", color=col, ms=4, lw=1, label=f"{b} hasher: IC / rho")
    cs = S["std"]["cells"]
    axs[0].axhline(1, color="k", lw=.8); axs[0].set_xticks(range(len(cs))); axs[0].set_xticklabels([f"{c['bits']}b\nT={c['T']}" for c in cs], fontsize=7)
    axs[0].set_ylabel("solve-time ratio (median, 95% CI)"); axs[0].legend(fontsize=7, frameon=False); axs[0].grid(alpha=.3)
    axs[0].set_title("RunPod cpu5c (EPYC 9575F), one pinned CPU")
    x = S["fast_over_std"]; xs = range(len(x))
    axs[1].plot(xs, [r["ic"]["median"] for r in x], "o-", color="#7a5195", label="fastest IC: fast / std")
    axs[1].plot(xs, [r["rho"]["median"] for r in x], "s-", color="#2a9d55", label="fastest rho: fast / std")
    axs[1].axhline(1, color="k", lw=.8); axs[1].set_xticks(list(xs)); axs[1].set_xticklabels([f"{r['bits']}b\nT={r['T']}" for r in x], fontsize=7)
    axs[1].set_title("Effect of the fast hasher on each side"); axs[1].legend(fontsize=7, frameon=False); axs[1].grid(alpha=.3)
    f.tight_layout(); return f
g = fig(); g.savefig("ratios.svg"); g.savefig("ratios.png", dpi=150)
with PdfPages("REPORT.pdf") as pdf:
    lines = open("RESULTS.md").read().splitlines(); per = 95
    for k in range(0, len(lines), per):
        t = plt.figure(figsize=(8.27, 11.69))
        if k == 0: t.text(0.06, 0.965, "Timed IC vs rho on RunPod, std vs fast hasher (2026-11-12)", fontsize=13, weight="bold")
        t.text(0.04, 0.94, "\n".join(lines[k:k + per]), fontsize=5.6, family="monospace", va="top"); pdf.savefig(t)
    pdf.savefig(fig())
print("ok")
