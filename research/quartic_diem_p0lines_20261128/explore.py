#!/usr/bin/env python3
"""Exploratory, descriptive (not the frozen decision): cost of p0 vs nofb by attempt pattern."""
import json, math, sys
from collections import defaultdict
R = {}
for f in ("raw/run.jsonl", "raw/run131101.jsonl"):
    for l in open(f):
        r = json.loads(l)
        if r.get("kind") == "run":
            R[(r["arm"][17:], r["p"], r["curve_seed"], r["seed"])] = r
na = lambda r: r["rel_mul"] + r["rel_add"] + r["la_mul"] + r["la_add"] + r["idx_lookups"] + r["idx_inserts"]
g1 = lambda r: r["group_ops"] + na(r)
gm = lambda v: math.exp(sum(map(math.log, v)) / len(v)) if v else float("nan")
out = {}
for p in (4099, 16411, 65537, 131101):
    cls = defaultdict(list)
    for (a, pp, cs, s), r in R.items():
        if a != "e_nofb" or pp != p:
            continue
        q = R[("e_p0", p, cs, s)]
        key = f"nofb{min(r['attempts'],2)}_p0{min(q['attempts'],2)}"
        cls[key].append(g1(q) / g1(r))
    p0rel = [R[k]["p0_line_relations"] for k in R if k[0] == "e_p0" and k[1] == p]
    p0lin = [R[k]["p0_lines"] for k in R if k[0] == "e_p0" and k[1] == p]
    out[p] = {"p0_lines": sum(p0lin) / len(p0lin), "p0_line_relations": sum(p0rel) / len(p0rel),
              **{k: {"n": len(v), "gm_gae1_ratio": round(gm(v), 3)} for k, v in sorted(cls.items())}}
json.dump(out, sys.stdout, indent=1)
print()
