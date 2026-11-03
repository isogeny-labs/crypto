#!/usr/bin/env python3
"""check_points.py <system.ms> <bw.json>: every point bwfinish_u reports satisfies every equation."""
import json, re, sys
lines = open(sys.argv[1]).read().split("\n", 2)
vs = lines[0].strip().split(","); p = int(lines[1])
polys = [q for q in lines[2].replace("\n", "").split(",") if q.strip()]
pts = json.load(open(sys.argv[2]))["points"]
def ev(poly, pt):
    tot = 0
    for sign, body in re.findall(r"([+-]?)([^+-]+)", poly):
        c = 1
        for x in body.split("*"):
            if x.isdigit(): c = c * int(x) % p
            else:
                v, _, k = x.partition("^"); c = c * pow(pt[vs.index(v)], int(k) if k else 1, p) % p
        tot += -c if sign == "-" else c
    return tot % p
res = [all(ev(q, pt) == 0 for q in polys) for pt in pts]
print(json.dumps({"points": len(pts), "all_satisfy": all(res) and len(pts) > 0, "per_point": res}))
sys.exit(0 if all(res) and pts else 1)
