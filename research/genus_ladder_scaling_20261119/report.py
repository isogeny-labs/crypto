#!/usr/bin/env python3
"""Presentation only: fig_scaling.svg/png from analysis.json (post hoc, not frozen)."""
import json, math, os
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
H = os.path.dirname(os.path.abspath(__file__))
c = json.load(open(os.path.join(H, "analysis.json")))["cells"]
f, ax = plt.subplots(figsize=(6.4, 4.2))
for key, lab, col, mk in (("g4_x", "genus 4, x (cc 2.0)", "#9467bd", "o"), ("g4_x*", "genus 4, x* (cc 1.4)", "#1f77b4", "s"),
                          ("g5_x5", "genus 5, x5 (cc 2.0)", "#ff7f0e", "^"), ("g5_x5*", "genus 5, x5* (cc 2.8, SGE 8,2)", "#d62728", "D")):
    g, arm = key.split("_")
    rows = sorted((v["mean_r"], v["neg"]["1"]) for k, v in c.items() if k.startswith(g + "_") and k.endswith("_" + arm))
    xs = [math.log2(r) for r, _ in rows]
    ax.errorbar(xs, [q["ratio"] for _, q in rows], yerr=[[q["ratio"] - q["lo"] for _, q in rows], [q["hi"] - q["ratio"] for _, q in rows]],
                fmt=mk + "-", color=col, capsize=3, label=lab)
ax.axhline(1, color="k", lw=0.8, ls="--")
ax.set(xlabel="log2 of group order r", ylabel="IC cost / ideal negation rho, w = 1", yscale="log",
       title="Hyperelliptic IC vs ideal negation rho (w = 1)  [pcc:unverified]")
ax.legend(fontsize=7)
f.tight_layout()
f.savefig(os.path.join(H, "fig_scaling.svg")); f.savefig(os.path.join(H, "fig_scaling.png"), dpi=80)
