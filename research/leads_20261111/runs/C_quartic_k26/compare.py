#!/usr/bin/env python3
"""Compare v2_orig and v2w unit records field by field, ignoring timing-only fields.

usage: compare.py <controls dir>   -> prints one line per unit pair and a final verdict.
Timing-only fields (wall clock, CPU, measured ratios, load, search time) differ between any two
executions; every count, statistic, ell, order, planted k and recovered k must be identical.
"""
import glob
import json
import os
import sys

TIMING_SUFFIX = (".wall", ".cpu")
TIMING_KEYS = {"r_inv", "r_lmul", "load1", "search_s", "s.controls_wall", "s.controls_cpu", "attempt_cpu"}


def strip(rec):
    return {k: v for k, v in rec.items() if k not in TIMING_KEYS and not k.endswith(TIMING_SUFFIX)}


def load(p):
    with open(p) as f:
        return [json.loads(l, parse_int=int) for l in f if l.strip().startswith("{")]


d = sys.argv[1]
pairs = 0
bad = 0
nfields = 0
for po in sorted(glob.glob(os.path.join(d, "v2_orig_ic_*.jsonl"))):
    pw = po.replace("v2_orig_", "v2w_")
    if not os.path.exists(pw) or not os.path.exists(pw.replace(".jsonl", ".res.json")):
        continue
    a, b = load(po), load(pw)
    unit = os.path.basename(po)[len("v2_orig_"):-len(".jsonl")]
    if len(a) != len(b) or len(a) == 0:
        print(f"{unit}: DIFFER (record count {len(a)} vs {len(b)})")
        bad += 1
        pairs += 1
        continue
    diffs = []
    for ra, rb in zip(a, b):
        sa, sb = strip(ra), strip(rb)
        nfields += len(sa)
        if sa != sb:
            ks = sorted(set(sa) | set(sb))
            diffs += [(k, sa.get(k), sb.get(k)) for k in ks if sa.get(k) != sb.get(k)]
    pairs += 1
    oks = [r.get("ok") for r in a if r.get("rec") == "ic"]
    if diffs:
        bad += 1
        print(f"{unit}: DIFFER {diffs[:6]}")
    else:
        print(f"{unit}: IDENTICAL ({len(a)} records, arms ok={oks})")
print(f"pairs={pairs} differing={bad} fields_compared={nfields}")
print("ALL IDENTICAL" if bad == 0 and pairs > 0 else "NOT ALL IDENTICAL")
