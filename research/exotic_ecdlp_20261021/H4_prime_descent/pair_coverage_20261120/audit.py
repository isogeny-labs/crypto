#!/usr/bin/env python3
"""Independent field point count and direct ordered-triple replay."""

from sage.all import *
from collections import Counter
import hashlib
import json
from pathlib import Path
import time

HERE = Path(__file__).resolve().parent
RESULT_PATH = HERE / "result.json"
RESULT = json.loads(RESULT_PATH.read_text())
CONTROL = json.loads((HERE.parent / "pair_index_20261120/result.json").read_text())["cells"][0]
assert RESULT["status"] == "complete"
n = 19
q = 1 << n
poly = CONTROL["field_modulus"]
a2 = CONTROL["curve_a2"]
a6 = CONTROL["curve_a6"]
basis = CONTROL["basis"]

def mul(a, b):
    out = 0
    while b:
        if b & 1:
            out ^= a
        b >>= 1
        a <<= 1
        if a & q:
            a ^= poly
    return out

def square(a):
    return mul(a, a)

def trace_bit(a):
    s = a
    v = a
    for _ in range(1, n):
        v = square(v)
        s ^= v
    assert s in (0, 1)
    return s

t = time.monotonic()
trace_mask = sum(trace_bit(1 << i) << i for i in range(n))
def fast_trace(a):
    return (a & trace_mask).bit_count() & 1

assert all(fast_trace(1 << i) == trace_bit(1 << i) for i in range(n))
exp = [0] * (q - 1)
x = 1
seen = bytearray(q)
for i in range(q - 1):
    assert not seen[x]
    seen[x] = 1
    exp[i] = x
    x <<= 1
    if x & q:
        x ^= poly
assert x == 1 and sum(seen) == q - 1
inv = [0] * q
for i, x in enumerate(exp):
    inv[x] = exp[-i]
assert all(mul(x, inv[x]) == 1 for x in (1, 2, 3, 7, 12345, q - 1))
sqrt_a6 = a6
for _ in range(n - 1):
    sqrt_a6 = square(sqrt_a6)
assert square(sqrt_a6) == a6
good_x = bytearray(q)
for x in range(1, q):
    good_x[x] = 1 - fast_trace(x ^ a2 ^ mul(sqrt_a6, inv[x]))
group_order_by_trace = 2 + 2 * sum(good_x)
base_points_by_trace = 1
for bits in range(1, 1 << len(basis)):
    x = 0
    for j, v in enumerate(basis):
        if (bits >> j) & 1:
            x ^= v
    base_points_by_trace += 2 * good_x[x]
assert group_order_by_trace == RESULT["group_order"]
assert base_points_by_trace == RESULT["base_points"]
trace_seconds = time.monotonic() - t

t = time.monotonic()
F2 = GF(2)
PR = PolynomialRing(F2, "z")
z = PR.gen()
mod = sum(F2((poly >> i) & 1) * z**i for i in range(n + 1))
K = GF(q, "z", modulus=mod)
def elt(v):
    return K([(v >> i) & 1 for i in range(n)])
E = EllipticCurve(K, [1, elt(a2), 0, 0, elt(a6)])
points = []
for bits in range(1 << len(basis)):
    x = 0
    for j, v in enumerate(basis):
        if (bits >> j) & 1:
            x ^= v
    points.extend(E.lift_x(elt(x), all=True))
assert len(points) == base_points_by_trace
target_counts = Counter()
for A in points:
    for B in points:
        AB = A + B
        for C in points:
            target_counts[AB + C] += 1
assert sum(target_counts.values()) == RESULT["ordered_triples"]
assert len(target_counts) == RESULT["distinct_three_sums"]
assert {str(k): v for k, v in sorted(Counter(target_counts.values()).items())} == RESULT["multiplicity_histogram"]
assert E(0) not in target_counts
assert len(target_counts) == RESULT["nonidentity_coverage_numerator"]
triple_seconds = time.monotonic() - t
audit = {"schema": "h4-three-sum-coverage-audit-v1", "status": "passed",
         "result_sha256": hashlib.sha256(RESULT_PATH.read_bytes()).hexdigest(),
         "group_order_by_trace": group_order_by_trace,
         "base_points_by_trace": base_points_by_trace,
         "trace_mask": trace_mask,
         "multiplicative_cycle_length": len(exp),
         "distinct_three_sums_by_direct_triples": len(target_counts),
         "ordered_triples_by_direct_triples": sum(target_counts.values()),
         "trace_seconds": trace_seconds, "direct_triples_seconds": triple_seconds}
(HERE / "audit.json").write_text(json.dumps(audit, indent=2, sort_keys=True) + "\n")
print(json.dumps(audit, indent=2, sort_keys=True))
