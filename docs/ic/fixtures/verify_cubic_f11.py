#!/usr/bin/env python3
"""Independent finite-field and curve check for cubic_f11_public_target.json."""

import hashlib
import json
from collections import Counter
from pathlib import Path


source = Path(__file__).with_name("cubic_f11_public_target.json")
raw = source.read_bytes()
target = json.loads(raw)
p = target["p"]
m0, m1, m2 = target["modulus"]
assert p == 11 and [m0, m1, m2] == [1, 0, 4]
assert all((x**3 + m2 * x**2 + m1 * x + m0) % p for x in range(p))

zero = (0, 0, 0)
one = (1, 0, 0)


def add(x, y):
    return tuple((a + b) % p for a, b in zip(x, y))


def neg(x):
    return tuple(-a % p for a in x)


def sub(x, y):
    return add(x, neg(y))


def mul(x, y):
    coefficients = [0] * 5
    for i in range(3):
        for j in range(3):
            coefficients[i + j] += x[i] * y[j]
    for degree in (4, 3):
        leading = coefficients[degree]
        for j, value in enumerate((m0, m1, m2)):
            coefficients[degree - 3 + j] -= leading * value
    return tuple(value % p for value in coefficients[:3])


def power(x, exponent):
    result = one
    while exponent:
        if exponent & 1:
            result = mul(result, x)
        x = mul(x, x)
        exponent >>= 1
    return result


def inverse(x):
    assert x != zero
    return power(x, p**3 - 2)


a = tuple(target["a"])
b = tuple(target["b"])
g = (tuple(target["gx"]), tuple(target["gy"]))
q = (tuple(target["qx"]), tuple(target["qy"]))
discriminant = add(mul((4, 0, 0), mul(mul(a, a), a)),
                   mul((27 % p, 0, 0), mul(b, b)))
assert discriminant != zero
assert all(target["n"] % divisor for divisor in range(2, int(target["n"] ** 0.5) + 1))


def on_curve(point):
    if point is None:
        return True
    x, y = point
    return mul(y, y) == add(add(mul(mul(x, x), x), mul(a, x)), b)


def point_add(left, right):
    if left is None:
        return right
    if right is None:
        return left
    x1, y1 = left
    x2, y2 = right
    if x1 == x2 and add(y1, y2) == zero:
        return None
    if left == right:
        numerator = add(mul((3, 0, 0), mul(x1, x1)), a)
        denominator = mul((2, 0, 0), y1)
    else:
        numerator = sub(y2, y1)
        denominator = sub(x2, x1)
    slope = mul(numerator, inverse(denominator))
    x3 = sub(sub(mul(slope, slope), x1), x2)
    y3 = sub(mul(slope, sub(x1, x3)), y1)
    result = (x3, y3)
    assert on_curve(result)
    return result


def point_mul(point, scalar):
    result = None
    while scalar:
        if scalar & 1:
            result = point_add(result, point)
        point = point_add(point, point)
        scalar >>= 1
    return result


elements = [
    (i % p, i // p % p, i // (p * p)) for i in range(p**3)
]
squares = Counter(mul(y, y) for y in elements)
curve_order = 1 + sum(
    squares[add(add(mul(mul(x, x), x), mul(a, x)), b)] for x in elements
)
assert curve_order == target["n"] == 1289
assert on_curve(g) and on_curve(q)
assert point_mul(g, 5) == q
assert point_mul(g, target["n"]) is None

print(json.dumps({
    "schema": "crypto.cubic-f11-independent-check/v1",
    "input_sha256": hashlib.sha256(raw).hexdigest(),
    "p": p,
    "modulus": [m0, m1, m2],
    "modulus_roots": [],
    "curve_order": curve_order,
    "generator_order": target["n"],
    "target_scalar": 5,
    "verified": True,
}, indent=2))
