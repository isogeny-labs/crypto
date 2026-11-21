#!/usr/bin/env python3
"""Validator: the Wiedemann-retry diagnosis, checked on grid rows.

Inputs: the study dir; the validator's diag replay of the OLD and ALG arms on
the grid curves at p <= 8209 (binary --quartic-diag, uncounted rank); and,
optionally, the producer's scratch development directory (not committed).
Checks: the diag replay rows equal raw/run.jsonl on every raw field
(wall_ns excepted); per-attempt rank pattern of failures; first-attempt
outcomes per prime (the protocol says the frozen first attempt is singular in
t "by construction"); the committed post-hoc NEW diagnosis; the development
claims of PROTOCOL.md section 2.
Usage: python3 -I diag_check.py STUDY_DIR DIAG_JSONL [DEV_DIR] > diag_check.json
"""
import json
import sys
from collections import Counter, defaultdict

STUDY, DIAG = sys.argv[1], sys.argv[2]
DEV = sys.argv[3] if len(sys.argv) > 3 else None


def runs(path):
    out = []
    for l in open(path):
        if l.strip():
            r = json.loads(l)
            if r["kind"] == "run" and r["family"] == "quartic":
                out.append(r)
    return out


raw = {}
for f in ["raw/run.jsonl", "raw/run65537_odd.jsonl", "raw/run65537_even.jsonl"]:
    for r in runs(f"{STUDY}/{f}"):
        raw[(r["p"], r["curve_seed"], r["seed"], r["arm"])] = r


def pattern(rs, arm):
    first = defaultdict(lambda: [0, 0])
    fails = Counter()
    tw = []
    for r in rs:
        if r["arm"] != arm:
            continue
        d0 = r["diag"][0]
        first[r["p"]][0] += 1
        first[r["p"]][1] += d0["outcome"] == 3
        for d in r["diag"]:
            if d["outcome"] != 3:
                fails[f"outcome={d['outcome']},rank==rank_no_t={d['rank'] == d['rank_no_t']}"] += 1
                tw.append(d["t_weight"])
    return {"first_attempt_ok_by_p": {p: f"{v[1]}/{v[0]}" for p, v in sorted(first.items())},
            "failed_attempts": dict(fails), "t_weight_of_failed": sorted(tw)}


out = {}
d = runs(DIAG)
same = sum(1 for r in d
           if all(r[k] == v for k, v in raw[(r["p"], r["curve_seed"], r["seed"], r["arm"])].items()
                  if k != "wall_ns"))
out["grid_diag_replay"] = {"rows": len(d), "identical_on_raw_fields": same,
                           "OLD": pattern(d, "Q_diem_2lp"), "ALG": pattern(d, "Q_diem_2lp_alg")}
out["grid_first_attempt_ok_all_primes"] = {}
for arm in ["Q_diem_2lp", "Q_diem_2lp_kick", "Q_diem_2lp_new"]:
    c = defaultdict(lambda: [0, 0])
    for k, r in raw.items():
        if k[3] == arm:
            c[k[0]][0] += 1
            c[k[0]][1] += r["attempts"] == 1
    out["grid_first_attempt_ok_all_primes"][arm] = {p: f"{v[1]}/{v[0]}" for p, v in sorted(c.items())}
out["explore_new_post_hoc"] = pattern(runs(f"{STUDY}/explore/diag_new_post_hoc.jsonl"), "Q_diem_2lp_new")
if DEV:
    out["dev_diag1_alg_frozen_order"] = pattern(runs(f"{DEV}/diag1.jsonl"), "Q_diem_2lp_alg")
    for k in (3, 4):
        rs = runs(f"{DEV}/dev3_k{k}.jsonl")
        out[f"dev3_k{k}"] = {arm: f"{sum(r['attempts'] > 1 for r in rs if r['arm'] == arm)}/"
                                  f"{sum(1 for r in rs if r['arm'] == arm)} runs with attempts > 1"
                             for arm in sorted({r['arm'] for r in rs})}
json.dump(out, sys.stdout, indent=1)
print()
