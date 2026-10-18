#!/usr/bin/env python3
"""Plot recorded thread-scaling rows: ns per refuted target against reserved cores, per rung.

  plot_threads.py times.jsonl diag_times.jsonl out.svg
Reads the isolated rows labelled bs512-1c / -2c / -3c and the non-isolated 4-thread diagnostic rows;
computes only the per-count median drawn as a bar.  Diagnostic points are hollow and labelled.
"""
import json, statistics, sys
import matplotlib
matplotlib.use("svg")
import matplotlib.pyplot as plt

times, diag, out = sys.argv[1:4]
rows = [json.loads(l) for l in open(times) if l.strip()]
drows = [json.loads(l) for l in open(diag) if l.strip()]
fig, axes = plt.subplots(1, 2, figsize=(7.6, 3.6))
for ax, l in zip(axes, (11, 12)):
    for k, tag in ((1, "bs512-1c"), (2, "bs512-2c"), (3, "bs512-3c")):
        v = [r["ns_per_target"] / 1e6 for r in rows if r["label"].startswith(f"{tag}-l{l}-")]
        ax.scatter([k] * len(v), v, s=16, color="C0", zorder=3, label="isolated_bench, reserved cores" if k == 1 else None)
        if v:
            m = statistics.median(v); ax.hlines(m, k - 0.2, k + 0.2, color="black", lw=1.2, zorder=4)
    v = [r["ns_per_target"] / 1e6 for r in drows if f"-l{l}-" in r["label"]]
    ax.scatter([4] * len(v), v, s=16, facecolors="none", edgecolors="C3", zorder=3, label="4 threads, NOT isolated (diagnostic)")
    if v:
        m = statistics.median(v); ax.hlines(m, 3.8, 4.2, color="black", lw=1.2, zorder=4)
    ax.set_xticks([1, 2, 3, 4]); ax.set_xlabel("cores / threads"); ax.set_title(f"l = {l} (n = {3*l})", fontsize=9)
    ax.set_yscale("log"); ax.grid(axis="y", alpha=0.3)
axes[0].set_ylabel("ms per refuted target"); axes[0].legend(fontsize=6)
fig.text(0.01, 0.01, "bs512, five rounds per point; black bars = medians; hollow = not an isolated run, never pooled", fontsize=7)
fig.tight_layout(rect=(0, 0.04, 1, 1)); fig.savefig(out)
