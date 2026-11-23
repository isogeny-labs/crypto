#!/usr/bin/env python3
"""Step 1 of PROTOCOL.md: per t, count on-curve V x-values for seeds 201..240, pick top 4 and bottom 4. Run: python3 -I select_groups.py > groups.txt"""
import json, os
here = os.path.dirname(os.path.abspath(__file__))
p, A = 1048573, 926
leg = lambda v: 0 if v % p == 0 else (1 if pow(v % p, (p-1)//2, p) == 1 else -1)
shift = A * pow(3, p-2, p) % p
rhs = lambda x: (x*x*x + A*x*x + x) % p
out = {}
for t in (4, 3):
    rows = []
    for l in open(os.path.join(here, "screen", "candidates_t%d.jsonl" % t)):
        j = json.loads(l); on = sum(1 for x in j["v"] if leg(rhs((x - shift) % p)) >= 0)
        rows.append((j["seed"], on, j["search_gae"] + j["count_gae"] + j["build_gae"], len(j["v"])))
    top = sorted(rows, key=lambda r: (-r[1], r[0]))[:4]; bot = sorted(rows, key=lambda r: (r[1], r[0]))[:4]
    print("t=%d |V|=%d candidates %d total tower ops %d" % (t, rows[0][3], len(rows), sum(r[2] for r in rows)))
    print(" TOP", [(r[0], r[1]) for r in top]); print(" BOTTOM", [(r[0], r[1]) for r in bot])
    out[str(t)] = {"top": [r[0] for r in top], "bottom": [r[0] for r in bot], "on_curve": {str(r[0]): r[1] for r in rows}, "tower_ops": {str(r[0]): r[2] for r in rows}}
json.dump(out, open(os.path.join(here, "groups.json"), "w"))
