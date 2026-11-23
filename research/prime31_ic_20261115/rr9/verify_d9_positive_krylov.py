#!/usr/bin/env python3
"""Independent original-polynomial and full-point replay of a B8 RR9 output."""
import hashlib
import json
import sys
from pathlib import Path

P = 2147483647
A_CURVE = 847514846
B_CURVE = 2087444724
BASE_X = (0, 1, 2, 3, 5, 6, 8, 9)
RESIDUAL = (1911579686, 548820096)


def multiply(left, right):
    out = [0] * (len(left) + len(right) - 1)
    for i, x in enumerate(left):
        for j, y in enumerate(right):
            out[i + j] = (out[i + j] + x * y) % P
    return out


def add_points(left, right):
    if left is None:
        return right
    if right is None:
        return left
    x, y = left
    u, v = right
    if x == u and (y + v) % P == 0:
        return None
    if left == right:
        slope = ((3 * x * x + A_CURVE) * pow(2 * y, -1, P)) % P
    else:
        slope = ((v - y) * pow(u - x, -1, P)) % P
    z = (slope * slope - x - u) % P
    return z, (slope * (x - z) - y) % P


def replay(path):
    raw = Path(path).read_bytes()
    result = json.loads(raw)
    if result["public_residual"] != list(RESIDUAL):
        raise ValueError("unexpected public residual")
    if result["base_x"] != list(BASE_X) or result["p"] != P:
        raise ValueError("unexpected curve or factor base")
    coeff = result["recovered_coefficients"]
    if not isinstance(coeff, list) or len(coeff) != 7 or any(
        type(x) is not int or not 0 <= x < P for x in coeff
    ):
        raise ValueError("missing or noncanonical coefficients")
    a = coeff[:4]
    v = coeff[4:] + [1]
    h = [1]
    for x in BASE_X:
        h = multiply(h, [(-x) % P, 1])
    r, s = RESIDUAL
    d = [(r * r + A_CURVE) % P, r, 1]
    dvv = multiply(d, multiply(v, v))
    xaa = multiply([(-r) % P, 1], multiply(a, a))
    av = multiply(a, v)
    errors = [
        (dvv[i] - (xaa[i] if i < len(xaa) else 0)
         - 2 * s * (av[i] if i < len(av) else 0) - h[i]) % P
        for i in range(8)
    ]
    if errors != [0] * 8 or dvv[8] != h[8] or h[8] != 1:
        raise ValueError("original RR9 coefficient identity failed")
    selected = []
    for x in BASE_X:
        rhs = (x * x * x + A_CURVE * x + B_CURVE) % P
        y = pow(rhs, (P + 1) // 4, P)
        if y == 0 or y * y % P != rhs:
            raise ValueError("factor-base ordinate is not a nonzero square")
        ax = sum(c * pow(x, i, P) for i, c in enumerate(a)) % P
        vx = sum(c * pow(x, i, P) for i, c in enumerate(v)) % P
        signs = [yy for yy in (y, (-y) % P)
                 if ((x - r) * ax + (s + yy) * vx) % P == 0]
        if len(signs) != 1:
            raise ValueError("coefficient chart did not choose one point")
        selected.append((x, signs[0]))
    total = None
    for point in selected:
        total = add_points(total, point)
    if total != RESIDUAL:
        raise ValueError("signed full-point sum differs from public residual")
    return {
        "schema": "prime31.rr9.d9-positive-independent-replay.v1",
        "input_sha256": hashlib.sha256(raw).hexdigest(),
        "public_residual": list(RESIDUAL),
        "base_x": list(BASE_X),
        "coefficients": coeff,
        "all_eight_original_equations_zero": True,
        "all_eight_original_points_verified": True,
        "signed_point_sum_matches_residual": True,
        "independent_implementation": True,
        "pcc": "unverified",
    }


if __name__ == "__main__":
    print(json.dumps(replay(sys.argv[1]), indent=2, sort_keys=True))
