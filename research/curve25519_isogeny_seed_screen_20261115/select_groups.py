#!/usr/bin/env python3
"""Step 1 of PROTOCOL.md: count on-curve V x-values per candidate seed, pick top/bottom 8. Run: python3 -I select_groups.py > groups.txt"""
import json, os
here = os.path.dirname(os.path.abspath(__file__))
p, A = 1048573, 926
leg = lambda v: 0 if v % p == 0 else (1 if pow(v % p, (p-1)//2, p) == 1 else -1)
shift = A * pow(3, p-2, p) % p
rhs = lambda x: (x*x*x + A*x*x + x) % p
rows = []
for l in open(os.path.join(here, "screen", "candidates.jsonl")):
    j = json.loads(l); on = sum(1 for x in j["v"] if leg(rhs((x - shift) % p)) >= 0)
    rows.append((j["seed"], on, j["search_gae"] + j["count_gae"] + j["build_gae"]))
order = sorted(rows, key=lambda r: (-r[1], r[0]))
top = order[:8]; bot = sorted(rows, key=lambda r: (r[1], r[0]))[:8]
print("seed on_curve tower_ops")
for r in sorted(rows): print(*r)
print("TOP", [r[0] for r in top], [r[1] for r in top])
print("BOTTOM", [r[0] for r in bot], [r[1] for r in bot])
print("TOTAL_TOWER_OPS", sum(r[2] for r in rows))
json.dump({"top": [r[0] for r in top], "bottom": [r[0] for r in bot], "on_curve": {r[0]: r[1] for r in rows}, "total_tower_ops": sum(r[2] for r in rows)}, open(os.path.join(here, "groups.json"), "w"))
