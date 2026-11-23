#!/usr/bin/env python3
"""Fit the cumulative mean round trip printed by FES_GPU_TIMING every 500 calls to rt_mean(n) = b + F/n.

    fit_startup.py LOG... [OUT_JSON]

b is the steady-state per-call round trip and F a one-time cost paid once per process (here: the first call of a
CUDA serve worker, which includes its CUDA initialisation).  Least squares on (1/n, mean) points; reads only the
`fes-timing:` lines the runs printed.
"""
import json, re, statistics, sys
P = re.compile(r"fes-timing: calls=(\d+) .*?round_trip=([\d.]+)")
args = sys.argv[1:]
out = args.pop() if args and args[-1].endswith(".json") else None
res = {}
for f in args:
    pts = [(int(a), float(b)) for a, b in P.findall(open(f, errors="replace").read())]
    if len(pts) < 3:
        continue
    xs = [1 / n for n, _ in pts]; ys = [r for _, r in pts]
    mx, my = statistics.fmean(xs), statistics.fmean(ys)
    F = sum((x - mx) * (y - my) for x, y in zip(xs, ys)) / sum((x - mx) ** 2 for x in xs)
    b = my - F * mx
    resid = max(abs(b + F * x - y) for x, y in zip(xs, ys))
    res[f] = {"points_calls_ms": pts, "b_us_per_call": b * 1e3, "F_ms_one_time": F, "max_abs_residual_us": resid * 1e3}
    print("%s: b=%.1f us/call  F=%.0f ms  max residual %.1f us  (%d points)" % (f.split("/")[-1], b * 1e3, F, resid * 1e3, len(pts)))
if out:
    json.dump(res, open(out, "w"), indent=1, sort_keys=True)
