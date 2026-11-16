#!/usr/bin/env python3
"""Figures from analysis.json (records only): paired cost ratio against the frozen configuration by
size, and the distance of the DL arm to the frozen matched rho."""
import json
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

a = json.load(open("analysis.json"))
R = a["ratios"]
fig, (ax, bx) = plt.subplots(1, 2, figsize=(11, 4.4))
style = {"CHAIN": ("#7f8c8d", "s", "square-root chain"), "CRULE": ("#2471a3", "o", "c_D = 1.55 (k ≥ 18)"),
         "BOTH": ("#c0392b", "D", "both")}
for x, (col, mk, lab) in style.items():
    for y, ls in (("DL", "-"), ("DL1", ":")):
        pts = sorted((r["k"], r["geomean"][0], r["min"][0], r["max"][0]) for r in R.values() if r["x"] == x and r["y"] == y)
        if not pts:
            continue
        ks = [p[0] for p in pts]
        ax.errorbar(ks, [p[1] for p in pts], yerr=[[p[1] - p[2] for p in pts], [p[3] - p[1] for p in pts]],
                    color=col, marker=mk, ls=ls, ms=5, capsize=2, lw=1.4,
                    label=f"{lab}, {y}")
ax.axhline(1, color="#333", lw=0.8)
ax.set_xlabel("k (q ≈ 2^k)")
ax.set_ylabel("cold M-eq / frozen configuration (same instance)")
ax.set_title("Paired cost ratio, w = 0 (geo-mean, min–max over seeds)", fontsize=9)
ax.set_ylim(0.4, 1.05)
ax.grid(alpha=0.3)
ax.legend(fontsize=7, ncol=2, loc="lower left")

D = a["cost_and_rho"]
for arm, col, mk in (("BASE", "#7f8c8d", "o"), ("BOTH", "#c0392b", "D")):
    for ra, ls in (("plain", "-"), ("neg", "--")):
        pts = sorted((int(k.split("/")[2]).bit_length(), v[f"over_rho_{ra}"][0]) for k, v in D.items()
                     if k.startswith(f"{arm}/DL/") and f"over_rho_{ra}" in v)
        if pts:
            bx.plot([p[0] for p in pts], [p[1] for p in pts], color=col, marker=mk, ls=ls, ms=4,
                    label=f"DL {'frozen c_D = 2.0' if arm == 'BASE' else 'both changes'} / rho-{ra}")
bx.set_yscale("log")
bx.axhline(1, color="#333", lw=0.8)
bx.set_xlabel("k (q ≈ 2^k)")
bx.set_ylabel("IC / matched rho (w = 0)")
bx.set_title("Distance to the frozen matched rho (below 1: IC cheaper)", fontsize=9)
bx.grid(alpha=0.3, which="both")
bx.legend(fontsize=7)
fig.tight_layout()
fig.savefig("ratio.svg")
fig.savefig("ratio.png", dpi=200)
print("ok")
