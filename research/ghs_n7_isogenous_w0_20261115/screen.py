#!/usr/bin/env python3
"""Independent product-kernel and elliptic-order census over the exact F128."""
import collections
import hashlib
import json
from pathlib import Path


def mul(a: int, b: int) -> int:
    out = 0
    while b:
        if b & 1:
            out ^= a
        b >>= 1
        a <<= 1
        if a & 0x80:
            a ^= 0x83
    return out


def power(a: int, n: int) -> int:
    out = 1
    while n:
        if n & 1:
            out = mul(out, a)
        a = mul(a, a)
        n >>= 1
    return out


def sigma(a: int) -> int:
    return mul(a, a)


def trace(a: int) -> int:
    out = 0
    for _ in range(7):
        out ^= a
        a = sigma(a)
    assert out in (0, 1)
    return out


def eval_poly_sigma(mask: int, value: int) -> int:
    out = 0
    x = value
    for i in range(mask.bit_length()):
        if (mask >> i) & 1:
            out ^= x
        x = sigma(x)
    return out


assert power(2, 127) == 1
assert all(power(2, k) != 1 for k in range(1, 127))
assert all(power(a, 127) == 1 for a in range(1, 128))

# First counter: the Artin–Schreier solvability criterion for y/x.
orders_trace = {}
for b in range(1, 128):
    affine = 1  # x=0 has its unique square-root solution
    for x in range(1, 128):
        rhs = x ^ mul(b, power(x, 125))  # b/x^2
        if trace(rhs) == 0:
            affine += 2
    orders_trace[b] = affine + 1  # point at infinity

# Second counter: direct enumeration of every (x,y), grouped by b.
affine_by_b = collections.Counter()
for x in range(128):
    x2 = mul(x, x)
    x3 = mul(x2, x)
    for y in range(128):
        b = mul(y, y) ^ mul(x, y) ^ x3
        affine_by_b[b] += 1
orders_direct = {b: 1 + affine_by_b[b] for b in range(1, 128)}
assert orders_trace == orders_direct
assert orders_trace[8] == 120

arms = {}
all_products = set()
for name, poly in (("p1", 0b1011), ("p2", 0b1101)):
    kernel = [x for x in range(128) if eval_poly_sigma(poly, x) == 0]
    assert len(kernel) == 8
    nonzero = kernel[1:]
    witnesses = {}
    for gamma in nonzero:
        for beta in nonzero:
            b = sigma(mul(gamma, beta))
            witnesses.setdefault(b, [gamma, beta])
    arms[name] = {"kernel": kernel, "coefficients": sorted(witnesses),
                  "witnesses": {str(k): v for k, v in sorted(witnesses.items())}}
    all_products.update(witnesses)

hist = collections.Counter(129 - orders_trace[b] for b in all_products)
hits = {str(t): [b for b in sorted(all_products) if 129 - orders_trace[b] == t]
        for t in (-9, 9)}
result = {"schema": "ghs-n7-w0-trace-census/v1", "field_modulus": "0x83",
          "arms": arms, "union_count": len(all_products),
          "trace_histogram": {str(k): v for k, v in sorted(hist.items())},
          "trace_hits": hits,
          "all_127_orders_replayed": orders_trace == orders_direct,
          "chosen_b8_a0_order": orders_trace[8],
          "chosen_b8_a1_order": 258 - orders_trace[8],
          "source_sha256": hashlib.sha256(Path(__file__).read_bytes()).hexdigest()}
Path(__file__).with_name("result.json").write_text(json.dumps(result, indent=2) + "\n")
print(json.dumps({k: result[k] for k in ("union_count", "trace_histogram", "trace_hits",
                                           "all_127_orders_replayed", "chosen_b8_a0_order",
                                           "chosen_b8_a1_order")}, indent=2))
