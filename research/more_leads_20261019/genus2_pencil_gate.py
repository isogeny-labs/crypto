#!/usr/bin/env python3
"""Exact small-field genus-2 pencil-sieve geometry check.

For D=P+Q-2 infinity on the published Scholten quintic, u and v are its
Mumford coordinates.  For every lambda, divide (v+lambda*u)^2-h by u,
and count three-distinct-root quotients.  This tests relation geometry,
not target-labelled DLP collection or a complete cost comparison.
"""

from hashlib import sha256
import json

P = 257
# 12 Y^2 = 12 + 100t + 299t^2 + 380t^3 + 187t^4 + 30t^5.
H = [c * pow(12, -1, P) % P for c in [12, 100, 299, 380, 187, 30]]


def trim(a):
    while len(a) > 1 and a[-1] == 0:
        a.pop()
    return a


def multiply(a, b):
    out = [0] * (len(a) + len(b) - 1)
    for i, ai in enumerate(a):
        for j, bj in enumerate(b):
            out[i+j] = (out[i+j] + ai*bj) % P
    return trim(out)


def divide(a, b):
    a = a[:]
    b = trim(b[:])
    quotient = [0] * max(1, len(a)-len(b)+1)
    inverse = pow(b[-1], -1, P)
    while len(a) >= len(b) and a != [0]:
        shift = len(a)-len(b)
        coefficient = a[-1]*inverse % P
        quotient[shift] = coefficient
        for i, bi in enumerate(b):
            a[i+shift] = (a[i+shift]-coefficient*bi) % P
        trim(a)
    return trim(quotient), a


def evaluate(a, x):
    acc = 0
    for c in reversed(a):
        acc = (acc*x+c) % P
    return acc


def points():
    squares = {y*y % P: y for y in range(P)}
    out = []
    for x in range(P):
        y = squares.get(evaluate(H, x))
        if y is not None:
            out.append((x, y))
    return out


def smooth_quintic():
    derivative = [(i*H[i]) % P for i in range(1, len(H))]
    a, b = H[:], trim(derivative)
    while b != [0]:
        _, remainder = divide(a, b)
        a, b = b, remainder
    return len(a) == 1


def run():
    assert smooth_quintic()
    pts = points()
    assert len({x for x, _ in pts}) == len(pts)
    # Fixed, visible sampling: first 20 consecutive disjoint pairs after a
    # deterministic hash sort, independent of the number of pencil hits.
    pts.sort(key=lambda z: sha256(f"pencil-v1:{z[0]}:{z[1]}".encode()).digest())
    samples = [pts[2*i:2*i+2] for i in range(20)]
    outputs = []
    for (x1, y1), (x2, y2) in samples:
        slope = (y2-y1)*pow((x2-x1) % P, -1, P) % P
        v = [(y1-slope*x1) % P, slope]
        u = [x1*x2 % P, (-x1-x2) % P, 1]
        v_square = multiply(v, v)
        assert divide([((v_square[i] if i < len(v_square) else 0)-H[i]) % P
                       for i in range(len(H))], u)[1] == [0]
        complete = []
        nonzero_roots = []
        for lam in range(P):
            w = v + [0]
            for i in range(3):
                w[i] = (w[i]+lam*u[i]) % P
            square = multiply(w, w)
            numerator = [(square[i] if i < len(square) else 0) - H[i]
                         for i in range(len(H))]
            numerator = [c % P for c in numerator]
            quotient, remainder = divide(numerator, u)
            assert remainder == [0]
            assert len(quotient) == 4, (quotient, lam)
            roots = tuple(x for x in range(P) if evaluate(quotient, x) == 0)
            nonzero_roots.append(len(roots))
            if len(roots) == 3:
                assert all((evaluate(w, x)**2-evaluate(H, x)) % P == 0 for x in roots)
                complete.append([lam, list(roots)])
        outputs.append({"divisor_points": [[x1, y1], [x2, y2]],
                        "complete_three_root_lambdas": len(complete),
                        "quotient_root_histogram": {str(k): nonzero_roots.count(k)
                                                     for k in range(4)},
                        "complete_sha256": sha256(json.dumps(complete,
                                                  separators=(',', ':')).encode()).hexdigest()})
    return {"schema": "genus2-pencil-geometry/v1", "p": P,
            "quintic_coefficients_low_to_high": H,
            "rational_x_lifts": len(pts), "sampled_divisors": len(outputs),
            "lambdas_per_divisor": P,
            "total_complete_three_root_lambdas": sum(x["complete_three_root_lambdas"]
                                                      for x in outputs),
            "checks": {"all_quotient_remainders_zero": True,
                       "all_complete_intersections_rechecked": True,
                       "quintic_squarefree": True},
            "divisors": outputs}


if __name__ == "__main__":
    print(json.dumps(run(), sort_keys=True))
