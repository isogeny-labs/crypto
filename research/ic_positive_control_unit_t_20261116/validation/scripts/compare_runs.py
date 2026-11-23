#!/usr/bin/env python3
"""Replay certificate for rows without a dump: every row of REPLAY must equal
the committed row with the same (arm, p, curve_idx, seed) in every
deterministic field (all fields except wall_ns and total_cal; the
validator dumper's extra val_* fields are dropped first).
Usage: compare_runs.py REPLAY_JSON COMMITTED_JSON"""
import json
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import vlib as V

rep = json.load(open(sys.argv[1]))["rows"]
com = json.load(open(sys.argv[2]))["rows"]
key = lambda r: (r["arm"], r["p"], r["curve_idx"], r["seed"])
C = {key(r): r for r in com}
same, diff, missing = 0, [], []
for r in rep:
    c = C.get(key(r))
    if c is None:
        missing.append(key(r))
        continue
    d = V.compare_rows({k: v for k, v in r.items() if not k.startswith("val_")}, c)
    if d:
        diff.append((key(r), d[:4]))
    else:
        same += 1
print(json.dumps({"replay_rows": len(rep), "identical": same, "different": len(diff), "missing_in_committed": len(missing),
                  "first_differences": diff[:5], "first_missing": missing[:5]}, default=str))
