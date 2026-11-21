#!/usr/bin/env python3
"""Validator: FPM-mid components of the PRIMARY arm (and OLD) per prime, and
the OLS exponent of each component (curve-level geometric means, as the
protocol's fit).  Shows which part of the total sets the sub-1 slope.
Usage: python3 -I components.py STUDY_DIR > components.json
"""
import json
import math
import sys
from collections import defaultdict

STUDY = sys.argv[1]
rows = []
for f in ["raw/run.jsonl", "raw/run65537_odd.jsonl", "raw/run65537_even.jsonl"]:
    for l in open(f"{STUDY}/{f}"):
        if l.strip():
            r = json.loads(l)
            if r["kind"] == "run" and r["family"] == "quartic":
                rows.append(r)


def comp(r):
    enum = r["rel_enum_mul"]
    dec = r["rel_decomp_mul"]
    return {
        "group": r["cantor_mul"] + 10 * r["cantor_inv"],
        "rel_enum": enum,
        "rel_decomp": dec,
        "rel_lines_taylor": r["rel_mul"] - enum - dec,
        "la9": 9 * r["la_mul"],
        "fpm_mid": r["cantor_mul"] + 10 * r["cantor_inv"] + r["rel_mul"] + 9 * r["la_mul"],
        "fpm_mid_minus_enum": r["cantor_mul"] + 10 * r["cantor_inv"] + r["rel_mul"] - enum + 9 * r["la_mul"],
        "group_ops": r["group_ops"],
        "decomp_trials": r["decomp_trials"],
        "pic_mul_per_op": r["cantor_mul"] / r["group_ops"],
        "lines_tried": r["lines_tried"],
    }


def gm(v):
    return math.exp(sum(math.log(x) for x in v) / len(v))


def ols(xs, ys):
    mx, my = sum(xs) / len(xs), sum(ys) / len(ys)
    return sum((x - mx) * (y - my) for x, y in zip(xs, ys)) / sum((x - mx) ** 2 for x in xs)


out = {}
for arm in ["Q_diem_2lp", "Q_diem_2lp_new"]:
    cur = defaultdict(list)
    for r in rows:
        if r["arm"] == arm:
            cur[(r["p"], r["curve_seed"])].append(comp(r))
    keys = list(next(iter(cur.values()))[0])
    curves = {k: {c: gm([m[c] for m in ms]) for c in keys} for k, ms in cur.items()}
    primes = sorted({k[0] for k in curves})
    per_p = {}
    for p in primes:
        cs = [v for k, v in curves.items() if k[0] == p]
        g = {c: gm([v[c] for v in cs]) for c in keys}
        tot = g["fpm_mid"]
        per_p[p] = {"fpm_mid": tot,
                    "share_group": g["group"] / tot, "share_rel_enum": g["rel_enum"] / tot,
                    "share_rel_decomp": g["rel_decomp"] / tot,
                    "share_rel_lines": g["rel_lines_taylor"] / tot, "share_la": g["la9"] / tot,
                    "group_ops": g["group_ops"], "decomp_trials": g["decomp_trials"],
                    "pic_mul_per_op": g["pic_mul_per_op"], "lines_tried": g["lines_tried"]}
    expo = {}
    for c in keys:
        xs = [math.log(k[0]) for k in curves]
        ys = [math.log(v[c]) for v in curves.values()]
        expo[c] = ols(xs, ys)
    out[arm] = {"per_prime": per_p, "exponents": expo}
json.dump(out, sys.stdout, indent=1)
print()
