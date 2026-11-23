#!/usr/bin/env python3
"""Plot recorded values from analyze.py output (no new computation beyond ratios already in the JSON).

plot.py RESULTS_AB.json RESULTS_AA.json OUT_PREFIX
Left: paired B/A ratio of session-total solve CPU time per round (run 2, distinct binaries).
Grey band: the same statistic between identical binaries (run 1, the A/A control), per row.
"""
import json, sys
import matplotlib; matplotlib.use("Agg")
import matplotlib.pyplot as plt

ab = json.load(open(sys.argv[1]))["rows"]; aa = {(r["pr"], r["spec"]): r for r in json.load(open(sys.argv[2]))["rows"]}
out = sys.argv[3]
fig, ax = plt.subplots(figsize=(8, 4.2))
for i, r in enumerate(ab):
    c = aa[(r["pr"], r["spec"])]["solve_cpu"]
    ax.add_patch(plt.Rectangle((i - 0.35, c["ba_min"]), 0.7, c["ba_max"] - c["ba_min"], color="0.85", zorder=0,
                               label="A/A control range (run 1, identical binaries)" if i == 0 else None))
    ys = r["solve_cpu"]["ba_per_round"]
    ax.scatter([i - 0.15 + 0.15 * k for k in range(len(ys))], ys, color="#2a6fdb", zorder=3, s=22,
               label="B/A per round (run 2)" if i == 0 else None)
    ax.hlines(r["solve_cpu"]["ba_geomean"], i - 0.3, i + 0.3, color="#c0392b", zorder=4,
              label="geometric mean over 3 rounds" if i == 0 else None)
ax.axhline(1.0, color="k", lw=0.6)
ax.set_xticks(range(len(ab))); ax.set_xticklabels([f"#{r['pr']}\n{r['spec']}" for r in ab])
ax.set_ylabel("B/A session-total solve CPU time\n(<1 = merge faster)")
ax.set_title("wallfix PRs, RunPod CPU pod bdr-wallfix, 3 counterbalanced rounds, n=20 measured runs/arm/session", fontsize=9)
ax.legend(fontsize=7, loc="lower left"); ax.grid(axis="y", alpha=0.3)
fig.tight_layout()
for ext in ("svg", "png"):
    fig.savefig(f"{out}.{ext}", dpi=150)
