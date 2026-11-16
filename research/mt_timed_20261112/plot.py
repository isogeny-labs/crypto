#!/usr/bin/env python3
"""Figure + PDF from results/holdout/summary.json and RESULTS.md."""
import json, matplotlib
matplotlib.use("Agg"); import matplotlib.pyplot as plt
from matplotlib.backends.backend_pdf import PdfPages
S = json.load(open("results/holdout/summary.json"))
def fig():
    f, axs = plt.subplots(1, 2, figsize=(13, 4.4))
    cs = S["cells"]; lab = [f"{c['bits']}b\nT={c['T']}" for c in cs]; xs = range(len(cs))
    for key, col, name, dx in (("ic_rho_solve", "#c0392b", "IC / rho (best of each side)", -0.15),
                               ("fixed_ic_rho_solve", "#3366cc", "IC / rho (keys fixed on repeat 0)", 0.0),
                               ("aa_rho", "#888888", "A/A: rho repeat 0 / repeat 1", 0.15)):
        m = [c[key]["median"] for c in cs]
        axs[0].errorbar([x + dx for x in xs], m, yerr=[[c[key]["median"] - c[key]["lo"] for c in cs], [c[key]["hi"] - c[key]["median"] for c in cs]],
                        fmt="o", color=col, ms=4, lw=1, label=name)
    axs[0].axhline(1, color="k", lw=.8); axs[0].set_xticks(list(xs)); axs[0].set_xticklabels(lab, fontsize=7)
    axs[0].set_ylabel("wall-time ratio, solve phase (median, 95% CI)"); axs[0].legend(fontsize=7, frameon=False); axs[0].grid(alpha=.3)
    axs[0].set_title("Timed head-to-head, one core of the session VM")
    for o in S["opcost"]:
        if o["when"] != "before": continue
        ks = sorted(o["std_lookup_gae"], key=int)
        axs[1].plot([int(k) for k in ks], [o["std_lookup_gae"][k] for k in ks], "o-", label=f"std HashMap lookup, {o['pb']} b")
        axs[1].plot([int(k) for k in ks], [o["fast_lookup_gae"][k] for k in ks], "x--", label=f"fast hasher, {o['pb']} b")
        axs[1].axhline(o["native_gae"], color="gray", lw=.6)
    axs[1].set_xlabel("log2 table entries"); axs[1].set_ylabel("price in GAE (one affine addition)")
    axs[1].set_title("Measured operation prices (grey: native op)"); axs[1].legend(fontsize=6, frameon=False); axs[1].grid(alpha=.3)
    f.tight_layout(); return f
g = fig(); g.savefig("ratios.svg"); g.savefig("ratios.png", dpi=150)
with PdfPages("REPORT.pdf") as pdf:
    lines = open("RESULTS.md").read().splitlines(); per = 95
    for k in range(0, len(lines), per):
        t = plt.figure(figsize=(8.27, 11.69))
        if k == 0: t.text(0.06, 0.965, "Multi-target IC vs rho in wall time (2026-11-12)", fontsize=13, weight="bold")
        t.text(0.04, 0.94, "\n".join(lines[k:k + per]), fontsize=5.6, family="monospace", va="top"); pdf.savefig(t)
    pdf.savefig(fig())
print("ok")
