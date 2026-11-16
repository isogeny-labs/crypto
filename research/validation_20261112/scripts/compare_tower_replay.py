#!/usr/bin/env python3
"""Compare a partial tower_large replay (subset of primes) with the committed tower_large.json:
every non-timing field of every replayed instance and prime row, and the HCurve cross-check
section.  Validator tool.  Usage: compare_tower_replay.py committed.json replay.json"""
import json, sys, re
A = json.load(open(sys.argv[1])); B = json.load(open(sys.argv[2]))
TIMING = re.compile(r"(wall|bookkeeping)")
def strip(x):
    if isinstance(x, dict): return {k: strip(v) for k, v in x.items() if not TIMING.search(k)}
    if isinstance(x, list): return [strip(v) for v in x]
    return x
ia = {(i["p"], i["instance"]): strip(i) for i in A["instances"]}
ib = {(i["p"], i["instance"]): strip(i) for i in B["instances"]}
ra = {r["p"]: strip(r) for r in A["rows"]}; rb = {r["p"]: strip(r) for r in B["rows"]}
diff = []
for k, v in ib.items():
    if k not in ia: diff.append(("instance not in committed", k)); continue
    if v != ia[k]: diff.append(("instance differs", k, [f for f in v if v[f] != ia[k].get(f)]))
for p, v in rb.items():
    if p not in ra: diff.append(("row not in committed", p)); continue
    if v != ra[p]: diff.append(("row differs", p, [f for f in v if v[f] != ra[p].get(f)]))
cx = strip(A["crosscheck"]) == strip(B["crosscheck"]) and A["crosscheck_pass"] == B["crosscheck_pass"]
if not cx: diff.append(("crosscheck differs",))
print(f"replayed instances {len(ib)} (primes {sorted(rb)}), rows {len(rb)}; crosscheck identical {cx}; differences {len(diff)}")
for d in diff[:20]: print("  ", d)
sys.exit(1 if diff else 0)
