#!/usr/bin/env python3
"""Descriptive (non-gating) validator checks: per-curve and per-phase candidate/incumbent
wall ratios, memory per arm, and the counted l2-gate below-L2 arms.
usage: descriptive.py OUT_JSON WALL_NAME=SESSION_DIR ... COUNTED_L2GATE=PATH COUNTED_SESSION=DIR"""
import json
import math
import sys
from collections import defaultdict


def load(p):
    with open(p) as f:
        return [json.loads(l) for l in f if l.strip()]


def measured(recs):
    d = defaultdict(dict)
    for r in recs:
        if not r["warmup"]:
            d[r["arm"]][(r["workload"]["workload_id"], r["round"])] = r
    return d


def median(xs):
    s = sorted(xs)
    n = len(s)
    return s[n // 2] if n % 2 else 0.5 * (s[n // 2 - 1] + s[n // 2])


out = {"wall": {}}
args = sys.argv[2:]
for a in args:
    k, v = a.split("=", 1)
    if k == "COUNTED_L2GATE":
        with open(v) as f:
            g = json.load(f)
        arms = defaultdict(int)
        lvls = defaultdict(int)
        for e in g.get("below_l2", []):
            arms[e.get("arm", "?")] += 1
            lvls[(e.get("isolation") or {}).get("level", e.get("level", "?"))] += 1
        out["counted_below_l2"] = {"n": len(g.get("below_l2", [])), "by_arm": dict(arms), "by_level": dict(lvls),
                                   "invalid_outcomes": g.get("invalid_outcomes"), "accepted": g.get("accepted"),
                                   "first_entry_keys": sorted(g["below_l2"][0].keys()) if g.get("below_l2") else []}
        continue
    if k == "COUNTED_SESSION":
        continue
    recs = load(f"{v}/records.jsonl")
    m = measured(recs)
    inc, cand = m["incumbent"], m["candidate"]
    keys = sorted(set(inc) & set(cand))
    per_curve = defaultdict(lambda: [0, 0])
    for kk in keys:
        slug = inc[kk]["workload"]["curve"]["slug"]
        per_curve[slug][0] += inc[kk]["time"]["solve_wall_ns"]
        per_curve[slug][1] += cand[kk]["time"]["solve_wall_ns"]
    phase = defaultdict(lambda: [0, 0])
    for kk in keys:
        for p, q in zip(inc[kk]["phases"], cand[kk]["phases"]):
            assert p["name"] == q["name"]
            phase[p["name"]][0] += p.get("wall_ns", 0)
            phase[p["name"]][1] += q.get("wall_ns", 0)
    tot_inc = sum(v[0] for v in phase.values())
    tot_cand = sum(v[1] for v in phase.values())
    mem = {arm: {"median_max_rss_kib": median([r["time"]["max_rss_kib"] for r in d.values()]),
                 "max_max_rss_kib": max(r["time"]["max_rss_kib"] for r in d.values()),
                 "median_solve_wall_ms": median([r["time"]["solve_wall_ns"] for r in d.values()]) / 1e6}
           for arm, d in sorted(m.items())}
    out["wall"][k] = {
        "per_curve_total_wall_ratio": {s: v[1] / v[0] for s, v in sorted(per_curve.items())},
        "per_phase_wall_ratio": {n: (v[1] / v[0] if v[0] else None) for n, v in phase.items()},
        "per_phase_share_of_extra_ns": {n: ((v[1] - v[0]) / (tot_cand - tot_inc)) for n, v in phase.items()},
        "memory_and_time_by_arm": mem,
    }
with open(sys.argv[1], "w") as f:
    json.dump(out, f, indent=1, sort_keys=True)
print(json.dumps(out, sort_keys=True)[:4000])
