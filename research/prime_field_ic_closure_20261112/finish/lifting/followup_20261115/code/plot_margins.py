#!/usr/bin/env python3
"""Margin at 256 bits vs r: R5 box cutoff (T=16) against the Hermite circuit cutoff.  [pcc:unverified]"""
import sys, json
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
d = json.load(open(sys.argv[1]))
rows = d["rows"]
fig, ax = plt.subplots(figsize=(7, 4.2))
for rho, col in (("1.065", "#2a6fdb"), ("4", "#d9822b"), ("10", "#7a4fbf")):
    rs = [r["r"] for r in rows]
    ax.plot(rs, [r[f"herm_{rho}"] for r in rows], "-o", color=col, ms=3, label=f"Hermite cutoff, rho={rho}")
    rb = [r["r"] for r in rows if r.get(f"box_T16_{rho}") is not None]
    ax.plot(rb, [r[f"box_T16_{rho}"] for r in rows if r.get(f"box_T16_{rho}") is not None], "--", color=col, lw=1, label=f"R5 box cutoff (T=16), rho={rho}")
ax.axhline(0, color="k", lw=0.8)
ax.set_xlabel("r (lifted points per trial)")
ax.set_ylabel("margin at 256 bits (bits above sqrt q)")
ax.set_ylim(-250, 140)
ax.set_title("Conditional lifting bound at 256 bits [pcc:unverified]")
ax.legend(fontsize=7, loc="lower left")
fig.tight_layout()
fig.savefig(sys.argv[2])
