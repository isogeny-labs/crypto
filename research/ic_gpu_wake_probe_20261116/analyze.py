#!/usr/bin/env python3
"""Summarise run_cells.sh output (cells.out): round trip per call by worker, reply wait, worker spin and idle gap.

    analyze.py CELLS_OUT [OUT_JSON]

Per cell (mean over repeats, with the range of the repeat means): client round trip mean and p50 in us; for the
CUDA worker its busy time per call.  Derived: unlocated = round trip mean - busy - null-worker round trip mean in the
same (reply, wspin, gap) cell.  Identity check: every reply checksum is equal within a worker.  It parses what the
runs printed and computes nothing else.
"""
import json, re, statistics, sys
L = re.compile(r"rep(\d+) worker=(\w+) reply=(\w+) wspin=(\d+) gap=(\d+) \| n=(\d+) mean=([\d.]+) p50=([\d.]+) p90=([\d.]+) sum=(\w+) \|(.*)")
B = re.compile(r"busy_us_per_call=([\d.]+)(?: kernel_gpu_us_per_call=([\d.]+))?")
cells, sums = {}, {}
for line in open(sys.argv[1]):
    m = L.match(line.strip())
    if not m:
        continue
    rep, w, r, ws, g, n, mean, p50, p90, s, rest = m.groups()
    key = (w, r, int(ws), int(g))
    rec = cells.setdefault(key, {"mean": [], "p50": [], "p90": [], "busy": [], "kernel": []})
    rec["mean"].append(float(mean)); rec["p50"].append(float(p50)); rec["p90"].append(float(p90))
    b = B.search(rest)
    if b:
        rec["busy"].append(float(b.group(1)))
        if b.group(2):
            rec["kernel"].append(float(b.group(2)))
    sums.setdefault(w, set()).add(s)

def ag(v):
    return None if not v else {"mean": statistics.fmean(v), "min": min(v), "max": max(v), "n": len(v)}

out = {"cells": {}, "identity": {w: len(s) for w, s in sums.items()}}
for (w, r, ws, g), rec in sorted(cells.items()):
    out["cells"]["%s|reply=%s|wspin=%d|gap=%d" % (w, r, ws, g)] = {k: ag(v) for k, v in rec.items() if v}
rows = []
for (w, r, ws, g), rec in sorted(cells.items()):
    if w != "cuda" or not rec["busy"]:
        continue
    floor = cells.get(("null", r, ws, g))
    rt, busy = statistics.fmean(rec["mean"]), statistics.fmean(rec["busy"])
    fl = statistics.fmean(floor["mean"]) if floor else None
    rows.append((r, ws, g, rt, statistics.fmean(rec["p50"]), busy, fl, None if fl is None else rt - busy - fl))
    out["cells"]["cuda|reply=%s|wspin=%d|gap=%d" % (r, ws, g)]["unlocated_us"] = rows[-1][-1]
print("distinct reply checksums per worker (1 = identical in every cell):", out["identity"])
print("%-6s %-6s %-5s | %9s %9s | %8s %9s | %11s" % ("reply", "wspin", "gap", "RT mean", "RT p50", "busy", "null RT", "unlocated"))
for r, ws, g, rt, p50, busy, fl, un in rows:
    print("%-6s %-6d %-5d | %9.1f %9.1f | %8.1f %9s | %11s" % (r, ws, g, rt, p50, busy, "-" if fl is None else "%.1f" % fl, "-" if un is None else "%.1f" % un))
if len(sys.argv) > 2:
    json.dump(out, open(sys.argv[2], "w"), indent=1, sort_keys=True)
