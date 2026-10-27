#!/usr/bin/env python3
"""PROJECTION (not a measurement) of the w1x restricted closure at N ~ 100.

Columns are exact combinatorics (count_columns.py). Basis bytes use
cols^2/16, the truncated-dense formula, which the measured `basis_bytes`
match to within 1% on every local run (checked below). Time is a
log-log least-squares fit of the measured median w1x seconds (3 threads)
against columns, per m, over the d_F = 4 cells measured here; it is a
crude extrapolation over one to two orders of magnitude and is labelled so.
"""
import glob
import json
import math
import statistics as st
import sys
from collections import defaultdict

sys.path.insert(0, __file__.rsplit("/", 1)[0])
from count_columns import count_w  # noqa: E402

E = "research/fall_degree_bounds_20261008/e11"
pts = defaultdict(lambda: defaultdict(list))  # m -> cols -> secs
fmt = []
files = glob.glob(f"{E}/validate/*.jsonl") + glob.glob(f"{E}/scaling/window_*.jsonl")
for p in files:
    for ln in open(p):
        if not ln.startswith("{"):
            continue
        r = json.loads(ln)
        w = r["window"].get("w1x")
        if not w or not w.get("certified"):
            continue
        pts[r["m"]][w["cols"]].append(w["secs"])
        fmt.append(w["basis_bytes"] / (w["cols"] ** 2 / 16))


def fit(m, min_cols=10000):
    xs, ys = [], []
    for c, s in pts[m].items():
        if c >= min_cols:
            xs.append(math.log(c))
            ys.append(math.log(max(st.median(s), 0.05)))
    if len(xs) < 2:
        return None
    mx, my = st.mean(xs), st.mean(ys)
    b = sum((x - mx) * (y - my) for x, y in zip(xs, ys)) / sum((x - mx) ** 2 for x in xs)
    a = my - b * mx
    return a, b, len(xs), max(math.exp(x) for x in xs)


print(f"basis_bytes / (cols^2/16): min {min(fmt):.3f} max {max(fmt):.3f} over {len(fmt)} certified runs", file=sys.stderr)
out = []
for (m, n, ell) in [(5, 25, 5), (4, 32, 8), (3, 48, 16)]:
    N = m * ell + (m - 2) * n
    low = sum(math.comb(N, d) for d in range(4))
    dense = low + math.comb(N, 4)
    w1 = low + count_w(m, n, ell, 1)
    f = fit(m)
    row = {"cell": f"{m}:{n}:{ell}", "N": N, "dense_cols": dense, "w1_cols": w1,
           "dense_basis_GB": round(dense ** 2 / 16 / 1e9, 1), "w1_basis_GB": round(w1 ** 2 / 16 / 1e9, 1),
           "deg3_cols": low, "deg3_dense_GB": round(low ** 2 / 16 / 1e9, 2)}
    if f:
        a, b, k, cmax = f
        row["time_fit"] = {"exponent": round(b, 2), "cells": k, "largest_measured_cols": round(cmax),
                           "projected_hours_3_threads": round(math.exp(a + b * math.log(w1)) / 3600, 1)}
    out.append(row)
print(json.dumps(out, indent=1))
