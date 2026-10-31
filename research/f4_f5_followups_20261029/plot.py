"""Plots the recorded values of runs/ (no computation beyond medians and
ratios of recorded times): item 1 per-site wall ratios, item 2 reduce-part
times at 1 and 4 threads, item 3 direct-unpack confirmation ratios.
Writes fig_followups.{svg,png}.  python3 plot.py"""
import collections
import glob
import json
import os
import re
import statistics

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt

here = os.path.dirname(os.path.abspath(__file__))
R = os.path.join(here, "runs")
fig, ax = plt.subplots(1, 3, figsize=(15, 4.6))


def short(case):
    return case.split("_p")[0].replace("overdet_n14_m28", "overdet14")

# item 1: none/site wall ratio per case and threads (ab_sites.txt)
d = collections.defaultdict(list)
for line in open(f"{R}/item1/ab_sites.txt"):
    r, t, f, c, ms = line.split()
    d[(t, c, f)].append(float(ms))
cases = sorted({k[1] for k in d})
x = 0
ticks = []
colours = {"pairs": "#2a6fbb", "bprime": "#4caf50", "rref": "#d9822b"}
for t in ("t1", "t4"):
    for c in cases:
        base = statistics.median(d[(t, c, "none")])
        for i, s in enumerate(("pairs", "bprime", "rref")):
            ax[0].bar(x + i * 0.25, base / statistics.median(d[(t, c, s)]), 0.25, color=colours[s],
                      label=s if x == 0 else None)
        ticks.append((x + 0.25, f"{short(c)}\n{t[1]} thr"))
        x += 1
ax[0].axhline(1, color="grey", lw=0.8)
ax[0].set_ylim(0.8, 1.15)
ax[0].set_xticks([a for a, _ in ticks])
ax[0].set_xticklabels([b for _, b in ticks], fontsize=6.5)
ax[0].set_ylabel("wall ratio control / four-row pass (>1: faster)")
ax[0].set_title("Item 1: four-row pass per site (3 rounds, median)", fontsize=9)
ax[0].legend(fontsize=7)

# item 2: reduce parts at 1 and 4 threads
rows = [l.split("\t") for l in open(f"{R}/item2/reduce_parts_ms.tsv") if not l.startswith(("#", "case"))]
x = 0
ticks = []
for case in dict.fromkeys(r[0] for r in rows):
    for r in [r for r in rows if r[0] == case]:
        b, s, p, q = (float(v) for v in (r[2], r[3], r[4], r[5]))
        ax[1].bar(x, b - s, color="#4caf50", label="B' build (parallel part)" if x == 0 else None)
        ax[1].bar(x, s, bottom=b - s, color="#2e7d32", label="B' in-block (serial)" if x == 0 else None)
        ax[1].bar(x, p, bottom=b, color="#2a6fbb", label="pair product" if x == 0 else None)
        ax[1].bar(x, q, bottom=b + p, color="#d9822b", label="echelon of the rest" if x == 0 else None)
        ticks.append((x, f"{short(case)}\n{r[1]} thr"))
        x += 1
    x += 0.5
ax[1].set_xticks([a for a, _ in ticks])
ax[1].set_xticklabels([b for _, b in ticks], fontsize=7)
ax[1].set_ylabel("ms (one run, F4 reduce phase)")
ax[1].set_title("Item 2: where the reduce phase spends its time", fontsize=9)
ax[1].legend(fontsize=7)

# item 3: off/on wall ratio, form 2 and form 0, 1 thread, 7 rounds
for k, (sub, lab, col) in enumerate((("confirm", "SelectiveEchelon", "#2a6fbb"), ("confirm0", "Reduced", "#d9822b"))):
    t = collections.defaultdict(list)
    for f in glob.glob(f"{R}/item3/{sub}/*.jsonl"):
        u = re.search(r"-u(\d)", f).group(1)
        for line in open(f):
            j = json.loads(line)
            t[(j["case"], u)].append(j["wall_ms"])
    cs = sorted({c for c, _ in t})
    vals = [statistics.median(t[(c, "0")]) / statistics.median(t[(c, "1")]) for c in cs]
    ax[2].bar([i + k * 0.4 for i in range(len(cs))], vals, 0.4, color=col, label=f"{lab} output")
ax[2].axhline(1, color="grey", lw=0.8)
ax[2].set_ylim(0.85, 1.2)
ax[2].set_xticks([i + 0.2 for i in range(len(cs))])
ax[2].set_xticklabels([c.replace("f5_", "") for c in cs], fontsize=7, rotation=20)
ax[2].set_ylabel("wall ratio off / direct unpack (>1: faster)")
ax[2].set_title("Item 3: direct unpack, 1 thread, 7 rounds\n(d3 cases run the same code in both forms)", fontsize=9)
ax[2].legend(fontsize=7)
fig.tight_layout()
fig.savefig(os.path.join(here, "fig_followups.svg"))
fig.savefig(os.path.join(here, "fig_followups.png"), dpi=120)
