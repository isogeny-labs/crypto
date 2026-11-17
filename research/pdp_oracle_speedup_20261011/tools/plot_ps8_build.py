#!/usr/bin/env python3
"""Plot recorded PS-8 table rows: build seconds per run and table size per rung.

  plot_ps8_build.py times.jsonl out.svg
Reads the `mitm` rows (setup_secs, table_keys, table_bytes) as recorded; draws every run and the median.
"""
import json, statistics, sys
import matplotlib
matplotlib.use("svg")
import matplotlib.pyplot as plt

times, out = sys.argv[1:3]
rows = [json.loads(l) for l in open(times) if l.strip() and json.loads(l)["arm"] == "mitm"]
ls = sorted({r["l"] for r in rows})
fig, (a, b) = plt.subplots(1, 2, figsize=(7.6, 3.4))
for i, l in enumerate(ls):
    v = [r["setup_secs"] for r in rows if r["l"] == l]
    a.scatter([i] * len(v), v, s=16, color="C0", zorder=3)
    a.hlines(statistics.median(v), i - 0.25, i + 0.25, color="black", lw=1.2, zorder=4)
    r0 = next(r for r in rows if r["l"] == l)
    b.bar(i, r0["table_bytes"] / 2**20, color="C1")
    b.text(i, r0["table_bytes"] / 2**20, f'{r0["table_keys"]/1e6:.2f}M keys', ha="center", va="bottom", fontsize=6)
for ax in (a, b):
    ax.set_xticks(range(len(ls))); ax.set_xticklabels([f"l={l}" for l in ls], fontsize=7); ax.set_yscale("log"); ax.grid(axis="y", alpha=0.3)
a.set_ylabel("table build, seconds per run"); b.set_ylabel("table size, MiB")
a.set_title("build time (5 runs per rung, black bar = median)", fontsize=8); b.set_title("table size (keys + payloads)", fontsize=8)
fig.text(0.01, 0.01, "source: ps8_mitm/raw/times.jsonl (setup_secs, table_keys, table_bytes); single thread", fontsize=7)
fig.tight_layout(rect=(0, 0.04, 1, 1)); fig.savefig(out)
