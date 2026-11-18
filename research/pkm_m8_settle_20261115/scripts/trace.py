#!/usr/bin/env python3
"""Summarize msolve -v 2 F4 traces: D, first round of each degree, cumulative time at first entry."""
import json, re, sys
ROUND = re.compile(r"^\s*(\d+)\s+(\d+)\s+(\d+)\s+(\d+) x (\d+)\s+([\d.]+)%\s+(\d+) new\s+(\d+) zero\s+([\d.]+) \| ([\d.]+)")
for f in sys.argv[1:]:
    rs = []
    for ln in open(f, errors="replace"):
        m = ROUND.match(ln)
        if m:
            rs.append(dict(deg=int(m[1]), pairs=int(m[2]), rows=int(m[4]), cols=int(m[5]), t=float(m[10])))
    first = {}
    cum = 0.0
    for i, r in enumerate(rs):
        cum += r["t"]
        first.setdefault(r["deg"], (i, round(cum, 2)))
    seq = []
    for r in rs:
        if not seq or seq[-1][0] != r["deg"]:
            seq.append([r["deg"], 1])
        else:
            seq[-1][1] += 1
    D = max((r["deg"] for r in rs), default=None)
    # monotone-entry check: before first round of degree d, all rounds have degree < d
    ok = all(all(rs[j]["deg"] < d for j in range(i)) for d, (i, _) in first.items())
    print(json.dumps(dict(log=f.split("/")[-1], rounds=len(rs), D=D, first_entry={d: v for d, v in sorted(first.items())},
                          entry_monotone=ok, deg_runs="".join(f"{d}x{n} " for d, n in seq)[:300])))
