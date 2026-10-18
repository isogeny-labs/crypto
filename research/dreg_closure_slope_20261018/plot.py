#!/usr/bin/env python3
"""Plot recorded closure degrees against m·ℓ (research-visuals: lays out
recorded values only; the slope is read from `dreg_score slope` output).

    plot.py RUN_DIR OUT_DIR SLOPE [RUN_ID]

RUN_DIR holds the `<kind>-m<M>-<n>-<ℓ>.d<K>.D<d>.jsonl` closure outputs;
the committed degree-4 anchors are listed here as the previous studies
recorded them.  Writes closure_slope.svg and closure_slope.png (150 dpi).
"""
import glob
import json
import os
import sys
from collections import defaultdict

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

run_dir, out_dir, slope = sys.argv[1], sys.argv[2], float(sys.argv[3])
run_id = sys.argv[4] if len(sys.argv) > 4 else os.path.basename(out_dir)

# Committed anchors: (m, n, ℓ) with dense closure 4 on every draw
# (research/dreg_closure_mutantxl_20261016, research/dreg_closure_scale_20261016).
anchors = [(4, 12, 3), (4, 14, 3), (3, 11, 5), (3, 13, 5), (3, 15, 5), (4, 13, 4), (3, 16, 6), (3, 18, 6)]

# Per draw: the first measured refuting degree, or a lower bound.
draws = defaultdict(dict)  # (m,n,ℓ,k) -> {d: record}
for path in sorted(glob.glob(os.path.join(run_dir, "*.D*.jsonl"))):
    name = os.path.basename(path)
    kind, rest = name.split("-m", 1)
    if kind in ("check", "control"):
        continue
    parts = rest.replace(".jsonl", "").split(".")
    m, n, ell = (int(x) for x in parts[0].split("-"))
    k = int(parts[1][1:])
    d = int(parts[2][1:])
    text = open(path).read().strip()
    rec = json.loads(text.splitlines()[0]) if text else {"killed": True}
    draws[(m, n, ell, k)][d] = rec

exact, bounds = [], []  # (mℓ, degree, m)
for (m, n, ell, k), degs in draws.items():
    found, bound = None, 0
    for d in sorted(degs):
        r = degs[d]
        if r.get("killed") or r.get("capped"):
            continue
        if r.get("closure_refuted"):
            found = d
            break
        if r.get("saturated"):
            bound = max(bound, d + 1)
    if found is not None:
        exact.append((m * ell, found, m))
    elif bound:
        bounds.append((m * ell, bound, m))

fig, ax = plt.subplots(figsize=(8, 4.8))
colors = {3: "#1f77b4", 4: "#d62728"}
# Anchors
for m, n, ell in anchors:
    ax.scatter([m * ell], [4], marker="s", s=60, facecolors="none", edgecolors=colors[m], zorder=3)
# Measured here, jittered per draw so coincident points stay visible
import random  # noqa: E402

random.seed(0)
for x, y, m in exact:
    ax.scatter([x + random.uniform(-0.25, 0.25)], [y], marker="o", s=40, color=colors[m], zorder=4)
for x, y, m in bounds:
    ax.scatter([x + random.uniform(-0.25, 0.25)], [y], marker="^", s=50, color=colors[m], zorder=4)
    ax.annotate("≥", (x + 0.35, y), fontsize=8, color=colors[m])
# Fit line and the two registered thresholds, anchored at (18, 4)
xs = [10, 32]
ax.plot(xs, [4 + slope * (x - 18) for x in xs], color="black", lw=1.5, label=f"slope of the least-squares fit, s = {slope:.3f} (line drawn through (18, 4))")
ax.plot(xs, [4 + 0.03 * (x - 18) for x in xs], color="green", ls="--", lw=1, label="s = 0.03 (grows slowly), through (18, 4)")
ax.plot(xs, [4 + 0.10 * (x - 18) for x in xs], color="orange", ls="--", lw=1, label="s = 0.10 (closed at every size), through (18, 4)")
ax.scatter([], [], marker="s", facecolors="none", edgecolors="gray", label="committed anchors, dense closure 4")
ax.scatter([], [], marker="o", color="gray", label="this study, exact degree (one point per draw)")
ax.scatter([], [], marker="^", color="gray", label="this study, lower bound only")
ax.plot([], [], color=colors[3], label="m = 3")
ax.plot([], [], color=colors[4], label="m = 4")
ax.set_xlabel("m·ℓ (summand unknowns of the chained S₃ system)")
ax.set_ylabel("MutantXL closure degree (sparse engine)")
ax.set_xlim(10, 32)
ax.set_ylim(3.5, 7.5)
ax.set_yticks([4, 5, 6, 7])
ax.grid(alpha=0.3)
ax.legend(fontsize=7, loc="upper left")
ax.set_title(f"Closure degree against m·ℓ — {run_id}\n{len(exact)} resolved draws, {len(bounds)} bounded; anchors from the two previous studies", fontsize=9)
fig.tight_layout()
os.makedirs(out_dir, exist_ok=True)
fig.savefig(os.path.join(out_dir, "closure_slope.svg"))
fig.savefig(os.path.join(out_dir, "closure_slope.png"), dpi=150)
print(f"{len(exact)} exact, {len(bounds)} bounded")
