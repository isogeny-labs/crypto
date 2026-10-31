#!/usr/bin/env python3
"""Layout only: plots the planning estimates recorded in cards.json (no computation of results)."""
import json, sys
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

d = json.load(open("cards.json"))
rented = [c for c in d["cards"] if c["usd_per_h"] > 0]
fig, ax = plt.subplots(figsize=(8, 4.2))
for i, c in enumerate(rented):
    ax.errorbar(c["cost"], i, xerr=[[c["cost"] - c["cost_lo"]], [c["cost_hi"] - c["cost"]]],
                fmt="o", color=plt.cm.viridis(c["p_decision"]), capsize=4, ms=7)
    ax.plot(c["cap"], i, marker="|", color="0.5", ms=12)
    ax.annotate(f'p={c["p_decision"]:.2f}', (c["cost"], i), xytext=(-12, -12), textcoords="offset points", fontsize=8)
ax.set_yticks(range(len(rented)), [f'{c["id"]}: {c["name"]}' for c in rented], fontsize=8)
ax.set_xscale("log"); ax.set_xlabel("expected cost, US$ (log; bar = range, grey tick = lifetime cap)")
ax.invert_yaxis(); ax.grid(axis="x", alpha=0.3)
ax.set_title("Rented-hardware cards, sorted by information per dollar\n(planning estimates; RunPod prices 2026-10-18 21:13Z; P6, P5 run at $0 on owner hosts)", fontsize=9)
fig.tight_layout(); fig.savefig("cards.svg"); fig.savefig("cards.png", dpi=150)
