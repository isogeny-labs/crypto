#!/usr/bin/env python3
"""Smoke table of session-smoke: per (arm, p) mean S, verified count, ratios.

Usage: summarize.py session-smoke [audit.json]
"""
import json, sys, collections, statistics as st

d = sys.argv[1]
recs = [json.loads(l) for l in open(f"{d}/records.jsonl")]
rows = collections.defaultdict(list)
for r in recs:
    slug = r["workload"]["curve"]["slug"]
    p = int(slug.split("fp3p")[1].split("-")[0])
    rows[(r["arm"], p)].append((slug, r["cost"]["s"], r["outcome"]["status"]))
ps = sorted({p for _, p in rows})
arms = ["rho-neg", "rho-plain", "k3-eliminator", "k3-cycles"]
print("arm              p     curves verified  mean S     S/rho-neg  S/rho-plain")
mean = {k: st.mean(x[1] for x in v) for k, v in rows.items()}
for a in arms:
    for p in ps:
        v = rows[(a, p)]
        ok = sum(x[2] == "verified" for x in v)
        print(f"{a:<16} {p:<5} {len(v):<6} {ok}/{len(v):<7} {mean[(a,p)]:<10.3f} "
              f"{mean[(a,p)]/mean[('rho-neg',p)]:<10.1f} {mean[(a,p)]/mean[('rho-plain',p)]:.1f}")
print()
print("paired per curve, cycles/eliminator:")
for p in ps:
    e = {x[0]: x[1] for x in rows[("k3-eliminator", p)]}
    c = {x[0]: x[1] for x in rows[("k3-cycles", p)]}
    rat = [c[k] / e[k] for k in e]
    print(f"p={p}: per-curve ratios {[round(x,3) for x in rat]}  mean {st.mean(rat):.3f}  "
          f"mean S cycles/eliminator {mean[('k3-cycles',p)]/mean[('k3-eliminator',p)]:.3f}")
