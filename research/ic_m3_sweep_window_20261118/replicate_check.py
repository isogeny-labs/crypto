#!/usr/bin/env python3
"""Prediction 1: the sweep=1 and rho arms of specs/replicate.json (seed 20261128) must equal round 5's
records (research/ic_gpu_rootfind_la_20261115/raw/out_rla.tgz, sessions/rf3la/records.jsonl) exactly.
Also prints the control's f = 1/8 ratio (second sample of the falsifier cell).
    replicate_check.py <round5 records.jsonl> <replicate records.jsonl>
"""
import json
import statistics
import sys


def load(p):
    return [json.loads(l) for l in open(p)]


old = {(r["arm"].replace("rf3-cpu-echelon-", "rf3-f1-"), r["workload"]["workload_id"], r["round"]): r
       for r in load(sys.argv[1])}
new = load(sys.argv[2])
n = ok = 0
for r in new:
    if not r["arm"].startswith(("rf3-f1-", "rho-neg")):
        continue
    o = old[(r["arm"], r["workload"]["workload_id"], r["round"])]
    same = (o["cost"]["s"] == r["cost"]["s"] and o["algorithm_seed"] == r["algorithm_seed"]
            and o["outcome"]["recovered"] == r["outcome"]["recovered"])
    if r["arm"].startswith("rf3-"):
        same = same and o["counters"]["targets_tried"] == r["counters"]["targets_tried"]
    n += 1
    ok += same
    if not same:
        print("DIFF", r["arm"], r["workload"]["workload_id"], r["round"], o["cost"]["s"], r["cost"]["s"])
print(f"replicated {ok}/{n}; verified {sum(r['outcome']['status'] == 'verified' for r in new)}/{len(new)}")
for a in (2048, 4096):
    for big in (False, True):
        def s(arm):
            return [r["cost"]["s"] for r in new
                    if r["arm"] == arm and (r["workload"]["curve_spec"]["p"].bit_length() > 38) == big]
        print(f"f=1/8 ratio, |F_x| {a}, log2 r {'39.3' if big else '37.4'}: "
              f"{statistics.mean(s(f'rf3-f8-F{a}')) / statistics.mean(s(f'rf3-f1-F{a}')):.3f}")
