#!/usr/bin/env python3
"""Validator: sanity checks of the independent group law (picarith.py) on study curves.

Usage: python3 -I grouplaw_checks.py RAW_JSONL P [P ...]
For the first curve of each prime: associativity and commutativity on random
point classes, x + 0 = x, [r-1]X + X = 0, and [a]X + [b]X = [a+b]X.
"""
import json
import os
import random
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import picarith as pa  # noqa: E402

rows = [json.loads(l) for l in open(sys.argv[1])]
for P in map(int, sys.argv[2:]):
    r0 = next(r for r in rows if r.get("kind") == "run" and r["p"] == P and "g_class" in r)
    C = pa.Quartic(P, r0["f"], r0["p0"])
    rng = random.Random(P)
    pts = C.random_points(12, rng)
    X = [C.class_of_points(pts[3 * i:3 * i + 3]) for i in range(4)]
    assoc = all(C.eq(C.add(C.add(X[i], X[j], rng), X[k], rng), C.add(X[i], C.add(X[j], X[k], rng), rng))
                for i, j, k in [(0, 1, 2), (1, 2, 3), (0, 0, 1), (3, 3, 3)])
    comm = all(C.eq(C.add(X[i], X[j], rng), C.add(X[j], X[i], rng)) for i, j in [(0, 1), (2, 3)])
    ident = all(C.eq(C.add(x, C.zero(), rng), x) for x in X)
    r = r0["r"]
    inv = C.eq(C.add(C.mul(X[0], r - 1, rng), X[0], rng), C.zero())
    a, b = rng.randrange(r), rng.randrange(r)
    lin = C.eq(C.add(C.mul(X[1], a, rng), C.mul(X[1], b, rng), rng), C.mul(X[1], (a + b) % r, rng))
    print(json.dumps({"p": P, "curve_seed": r0["curve_seed"], "associative": assoc, "commutative": comm,
                      "identity": ident, "r_minus_1_inverse": inv, "linear_in_scalar": lin}))
