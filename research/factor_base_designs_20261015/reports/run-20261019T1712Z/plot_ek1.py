#!/usr/bin/env python3
"""Plots the recorded E-K1 rows (plotting only; no result is computed here
beyond the means the native analysis also prints: cell = mean of its two
targets, b = log2 of the exact point count recorded in each row).

usage: plot_ek1.py RUNS_DIR OUT_DIR
"""
import glob
import json
import math
import os
import sys
from collections import defaultdict

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt

runs, out = sys.argv[1], sys.argv[2]
os.makedirs(out, exist_ok=True)

rows = []
for f in sorted(glob.glob(os.path.join(runs, "K1-*.jsonl"))):
    if os.path.basename(f).startswith("K1-replay"):
        continue
    for line in open(f):
        line = line.strip()
        if line:
            rows.append(json.loads(line))

cells = defaultdict(lambda: defaultdict(list))  # (arm,n,m) -> ell -> rows
for r in rows:
    if r.get("status") == "no_plant":
        continue
    cells[(r["arm"], r["n"], r["m"])][r["ell"]].append(r)

ARMS = ["S", "X", "XR", "U", "NS", "PC"]
STYLE = {
    "S": ("#0072B2", "o", "-"),
    "X": ("#009E73", "s", "-"),
    "XR": ("#D55E00", "^", "-"),
    "U": ("#CC79A7", "D", "-"),
    "NS": ("#555555", "x", "--"),
    "PC": ("#E69F00", "v", "--"),
}


def cell(rs):
    ok = [r for r in rs if r.get("status") == "ok"]
    done = len(ok) >= 2 and len(ok) == len(rs)
    return ok, done


def panel_data(arm, n, m, field):
    pts, lim = [], []
    for ell in sorted(cells.get((arm, n, m), {})):
        ok, done = cell(cells[(arm, n, m)][ell])
        b = next((r["b"] for r in cells[(arm, n, m)][ell] if r.get("b") is not None), None)
        if b is None:
            continue
        if done:
            if field == "lf":
                y = math.log2(sum(r["field_ops"] for r in ok) / len(ok))
            else:
                y = sum(r["D"] for r in ok) / len(ok)
            pts.append((b, y))
        else:
            lim.append((b, ell))
    return pts, lim


def figure(field, ylabel, name, title):
    fig, axes = plt.subplots(2, 2, figsize=(11, 8), constrained_layout=True)
    for i, n in enumerate([11, 23]):
        for j, m in enumerate([2, 3]):
            ax = axes[i][j]
            first = None
            ymax = None
            for arm in ARMS:
                pts, lim = panel_data(arm, n, m, field)
                color, mk, ls = STYLE[arm]
                if pts:
                    ax.plot(
                        [p[0] for p in pts],
                        [p[1] for p in pts],
                        marker=mk,
                        ls=ls,
                        color=color,
                        label=arm,
                        ms=5,
                        lw=1.3,
                    )
                    if first is None and arm == "S":
                        first = pts[0]
                    ymax = max(ymax or -1e9, max(p[1] for p in pts))
                for b, ell in lim:
                    ax.axvline(b, color=color, alpha=0.18, lw=3)
            if field == "lf" and first is not None:
                xs = [first[0], first[0] + 12]
                ax.plot(
                    xs,
                    [first[1], first[1] + (m - 1) * 12],
                    color="black",
                    ls=":",
                    lw=1,
                    label=f"slope m−1 = {m - 1}",
                )
                if ymax is not None:
                    ax.set_ylim(top=ymax + 2)
            ax.set_title(f"n = {n}, m = {m}")
            ax.set_xlabel("b = log₂ |F| (factor-base points per summand)")
            ax.set_ylabel(ylabel)
            ax.grid(alpha=0.3)
            ax.legend(fontsize=8)
    fig.suptitle(title, fontsize=11)
    for ext, kw in (("svg", {}), ("png", {"dpi": 150})):
        fig.savefig(os.path.join(out, f"{name}.{ext}"), **kw)
    plt.close(fig)


figure(
    "lf",
    "log₂ field_ops (mean of 2 targets)",
    "ek1-field-ops-vs-b",
    "E-K1: log₂ field_ops against b per arm, completed cells; dotted line: break-even slope m−1; shaded verticals: limit cells",
)
figure(
    "D",
    "D = solving degree (mean of 2 targets)",
    "ek1-D-vs-b",
    "E-K1: solving degree D against b per arm, completed cells; shaded verticals: limit cells",
)
print("wrote", out)
