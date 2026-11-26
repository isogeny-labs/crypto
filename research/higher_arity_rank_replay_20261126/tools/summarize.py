#!/usr/bin/env python3
"""Paired summary of the admission-rule panel: ratios fb2/all with bootstrap intervals."""
import json, glob, math, random, statistics as st, sys
D = sys.argv[1]
d = {}
import os
runs = [json.loads(l) for l in open(f"{D}/runs.jsonl")] if os.path.exists(f"{D}/runs.jsonl") else \
    [dict(json.load(open(p)), file=p.split("/")[-1]) for p in glob.glob(f"{D}/s*-*.json")]
for r in runs:
    salt, rule = r["file"][1:-5].split("-")
    d.setdefault(int(salt), {})[rule] = r
salts = sorted(d)
assert all(d[s][r]["recovered"] == 987654321 and d[s][r]["admit"] == r for s in salts for r in ("fb2", "all"))
gm = lambda v: math.exp(sum(map(math.log, v)) / len(v))
out = {"salts": len(salts), "metrics": {}}
for key in ("queries", "probes", "relations", "useful", "cold_ms", "probe_ms", "elim_ms"):
    a = [d[s]["fb2"][key] for s in salts]; b = [d[s]["all"][key] for s in salts]
    rs = [x / y for x, y in zip(a, b)]
    rng = random.Random(11); bs = sorted(gm([rng.choice(rs) for _ in rs]) for _ in range(20000))
    out["metrics"][key] = {"fb2_median": st.median(a), "all_median": st.median(b),
                           "fb2_over_all": [gm(rs), bs[500], bs[19500]],
                           "all_lower": sum(y < x for x, y in zip(a, b)), "all_higher": sum(y > x for x, y in zip(a, b))}
print(json.dumps(out, indent=1))
