"""Plots raw/probe.txt: per cell, the build-time key collisions of the
Frobenius pair table, the non-trivial rows among them, and the rank the free
rows would need (c - 1).  python3 plot.py"""
import re
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

pat = re.compile(r"^(n\d+) (a\d) c(\d+) .*entries (\d+) attempts (\d+) collisions (\d+) nonzero rows (\d+) rank (\d+) \(need (\d+)\)")
rows = [pat.match(l).groups() for l in open("raw/probe.txt") if pat.match(l)]
labels = [f"{n}{a}\nc={c}" for n, a, c, *_ in rows]
x = range(len(rows))
fig, ax = plt.subplots(figsize=(9, 4.2))
ax.bar([i - 0.27 for i in x], [int(r[5]) for r in rows], 0.27, label="key collisions at build", color="#2a6fbb")
ax.bar([i for i in x], [max(int(r[6]), 0.5) for r in rows], 0.27, label="non-trivial rows (0 everywhere; drawn at 0.5)", color="#d9822b")
ax.bar([i + 0.27 for i in x], [max(int(r[8]), 0.5) for r in rows], 0.27, label="rank needed (c - 1)", color="#888888")
ax.set_yscale("log")
ax.set_xticks(list(x))
ax.set_xticklabels(labels, fontsize=8)
ax.set_ylabel("count (log)")
ax.set_title("Frobenius pair table: every build-time collision is a symmetry, no free relations", fontsize=10)
ax.legend(fontsize=8)
fig.tight_layout()
fig.savefig("fig_free_rows.svg")
fig.savefig("fig_free_rows.png", dpi=120)
