#!/usr/bin/env python3
"""Part C step 1 of PROTOCOL.md: on-curve count c per candidate (t=5 seeds 61..3060), choose the largest. Run: python3 -I select_deep.py > deep_groups.txt"""
import json, os, collections
here = os.path.dirname(os.path.abspath(__file__))
p, A = 1048573, 926
leg = lambda v: 0 if v % p == 0 else (1 if pow(v % p, (p-1)//2, p) == 1 else -1)
shift = A * pow(3, p-2, p) % p
rhs = lambda x: (x*x*x + A*x*x + x) % p
rows = []
for l in open(os.path.join(here, "deep", "candidates_t5.jsonl")):
    j = json.loads(l); on = sum(1 for x in j["v"] if leg(rhs((x - shift) % p)) >= 0)
    rows.append((j["seed"], on, j["search_gae"] + j["count_gae"] + j["build_gae"]))
best = sorted(rows, key=lambda r: (-r[1], r[0]))[0]
hist = collections.Counter(r[1] for r in rows)
print("candidates", len(rows), "total tower ops", sum(r[2] for r in rows))
print("histogram c:", sorted(hist.items()))
print("CHOSEN seed %d c=%d" % best[:2], "top5", sorted(rows, key=lambda r: (-r[1], r[0]))[:5])
json.dump({"chosen": best[0], "c": best[1], "total_tower_ops": sum(r[2] for r in rows), "n": len(rows)}, open(os.path.join(here, "deep_groups.json"), "w"))
