#!/usr/bin/env python3
"""Lay out recorded values only: reads raw/analysis.json (written by the Rust `analyze`), draws
fig-ratios.{svg,png}. No result is computed here."""
import json, sys, os
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
here = os.path.dirname(os.path.abspath(__file__))
a = json.load(open(os.path.join(here, "raw", "analysis.json")))
roles = {r["role"]: r for r in a["roles"]}
style = {"b": ("b: BW (4,4)", "tab:red", "s"), "c": ("c: block Lanczos w4", "tab:purple", "^"),
         "d": ("d: auto filter (cx, mw)", "tab:green", "o"), "e": ("e: oversample+clique (round 4)", "tab:orange", "D"),
         "aa": ("aa: independent walk (null)", "tab:gray", "x")}
fig, axs = plt.subplots(1, 2, figsize=(11, 4.6))
for ax, sec, ttl in zip(axs, ("la", "whole"), ("LA cost ratio to arm a (c_ell = 1)", "Whole-solve cost ratio to arm a (c_ell = 1)")):
    for k, (lab, col, mk) in style.items():
        rs = roles[k]["sections"][sec]["ratios"]
        x = [r["bits"] for r in rs]; y = [r["ratio"] for r in rs]
        lo = [r["ratio"] - r["ci"][0] for r in rs]; hi = [r["ci"][1] - r["ratio"] for r in rs]
        ax.errorbar(x, y, yerr=[lo, hi], label=lab, color=col, marker=mk, capsize=2, lw=1.2, ms=5)
    ax.axhline(1, color="k", lw=0.8)
    if sec == "whole":
        ax.axhline(1 / 1.2, color="k", ls="--", lw=0.8); ax.text(16, 0.835, "1/1.2 constant threshold", fontsize=7)
    ax.set_xlabel("b  (q ~ 2^b, genus 3)"); ax.set_ylabel("paired geometric-mean ratio"); ax.set_title(ttl, fontsize=10)
    ax.set_xticks([16, 17, 18, 19, 20]); ax.grid(alpha=.3)
h, l = axs[0].get_legend_handles_labels(); fig.legend(h, l, fontsize=7, loc="lower center", ncol=5, bbox_to_anchor=(0.5, 0.03))
fig.suptitle("Item 15, genus-3 IC linear algebra: arms vs incumbent r2:lpsgebw2 (95% bootstrap CI over curves; n = 3-5 curves per size)", fontsize=9)
fig.text(0.5, 0.005, "Source: raw/analysis.json. CPU, one thread, shared host. Arm e uses committed round-4 cells (3 curves at b<=17, 5 at b>=18) and its own paired a.", ha="center", fontsize=7)
fig.tight_layout(rect=(0, 0.09, 1, 0.94))
fig.savefig(os.path.join(here, "fig-ratios.svg")); fig.savefig(os.path.join(here, "fig-ratios.png"), dpi=150)
