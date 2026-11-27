#!/usr/bin/env python3
"""Useful rank per second by arm (as merged and patched) and query speed, log scale."""
import json, matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
p1 = json.load(open("evidence/panel-01/summary.json"))["arms"]
p2 = json.load(open("evidence/panel-02/summary.json"))["arms"]
rows = [("generic\n(as merged)", p1["generic"], "#adb5bd"), ("fast_path\n(full scan)", p2["fast"], "#adb5bd"),
        ("first\n(as merged)", p1["first"], "#adb5bd"), ("first\n(patched)", p2["first"], "#4c6ef5"),
        ("decompose_four\n(default)", p2["default"], "#4c6ef5"), ("all-witness ref\n(#5184)", p2["ref"], "#212529")]
fig, axes = plt.subplots(1, 2, figsize=(12, 4.3))
for ax, key, title in [(axes[0], "rank_per_s", "useful rank per second (cold, to verified recovery)"),
                       (axes[1], "query_ms_mean", "mean oracle time per query (ms)")]:
    labels, vals, cols = [], [], []
    for lab, d, c in rows:
        if key == "query_ms_mean" and "query_ms_mean" not in d:
            v = d["query_ms"] / d["queries"]
        else:
            v = d[key]
        labels.append(lab); vals.append(v); cols.append(c)
    ax.bar(range(len(vals)), vals, color=cols); ax.set_yscale("log"); ax.set_title(title, fontsize=10)
    ax.set_xticks(range(len(vals))); ax.set_xticklabels(labels, fontsize=7); ax.grid(axis="y", alpha=.3)
    for i, v in enumerate(vals):
        ax.text(i, v * 1.15, f"{v:,.1f}" if v < 100 else f"{v:,.0f}", ha="center", fontsize=7)
fig.suptitle("Semaev PairProjection on frozen B677 M31 (458,234 finite pair roots); same queries, scalar 987654321 every run", fontsize=10)
fig.tight_layout()
for ext in ("svg", "pdf", "png"):
    fig.savefig(f"figures/rank_rate.{ext}", dpi=150)
