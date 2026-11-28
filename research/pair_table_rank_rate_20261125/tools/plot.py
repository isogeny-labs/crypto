#!/usr/bin/env python3
"""Useful rank/s and cold-time breakdown per arm (panel summary)."""
import json, sys, matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
d = json.load(open(sys.argv[1]))["arms"]; out = sys.argv[2]
arms = [("default", "library default\ndecompose_four"), ("ref", "#5184 all-witness\nsource"),
        ("pt1", "pair table\n1 thread"), ("pt4", "pair table\n4 threads"), ("rho", "equal-eng. rho\n1 thread")]
fig, (a1, a2) = plt.subplots(1, 2, figsize=(12.5, 4.4))
vals = [d[a]["rank_per_s"] for a, _ in arms[:4]]
a1.bar(range(4), vals, color=["#adb5bd", "#868e96", "#4c6ef5", "#364fc7"])
a1.set_xticks(range(4)); a1.set_xticklabels([l for _, l in arms[:4]], fontsize=8)
a1.set_title("useful rank per second, cold to verified recovery (median)", fontsize=10); a1.grid(axis="y", alpha=.3)
for i, v in enumerate(vals):
    a1.text(i, v * 1.02, f"{v:,.0f}", ha="center", fontsize=8)
parts = [("build_ms", "table / projection build", "#748ffc"), ("source_ms", "relation source (probes, oracle)", "#4c6ef5"),
         ("consumer_ms", "replay + elimination", "#adb5bd")]
bottom = [0.0] * len(arms)
for key, lab, col in parts:
    v = [d[a][key] for a, _ in arms]
    a2.bar(range(len(arms)), v, bottom=bottom, color=col, label=lab)
    bottom = [b + x for b, x in zip(bottom, v)]
for i, (a, _) in enumerate(arms):
    a2.text(i, d[a]["cold_ms"] * 1.03, f"{d[a]['cold_ms']:.0f} ms" if d[a]["cold_ms"] >= 10 else f"{d[a]['cold_ms']:.1f} ms", ha="center", fontsize=8)
a2.set_yscale("log"); a2.set_ylim(1, 1500)
a2.set_xticks(range(len(arms))); a2.set_xticklabels([l for _, l in arms], fontsize=8)
a2.set_title("cold wall time to verified recovery, by phase (median, log)", fontsize=10); a2.legend(fontsize=7, loc="upper right")
fig.suptitle("Frozen B677 M31 workload (677 points, scalar 987654321 every run), one host", fontsize=10)
fig.tight_layout()
for ext in ("svg", "pdf", "png"):
    fig.savefig(f"{out}.{ext}", dpi=150)
