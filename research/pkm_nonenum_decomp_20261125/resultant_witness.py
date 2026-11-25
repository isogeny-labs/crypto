#!/usr/bin/env python3
"""Staged resultant PKM three-summand witness diagnostic.

No factor-base list is passed to the algebraic extractor. All polynomial
resultants use SymPy's integer expression engine before reduction modulo p;
final point signs are verified by group arithmetic.
"""

import argparse
import itertools
import json
import os
import resource
import signal
import sys
import time

# SymPy's optional python-flint factor sorter fails on modular coefficients in
# the installed version. Keep this backend fixed for reproducible extraction.
os.environ["SYMPY_GROUND_TYPES"] = "python"
import sympy as sp


class TimedOut(Exception):
    pass


class DegenerateGCD(Exception):
    pass


def timeout_handler(_signum, _frame):
    raise TimedOut


def s3(x, y, z, a, b):
    return (x - y) ** 2 * z**2 - 2 * ((x + y) * (x * y + a) + 2 * b) * z + (x * y - a) ** 2 - 4 * b * (x + y)


def add(left, right, a, p):
    if left is None:
        return right
    if right is None:
        return left
    x1, y1 = left
    x2, y2 = right
    if x1 == x2 and (y1 + y2) % p == 0:
        return None
    numerator = 3 * x1 * x1 + a if x1 == x2 else y2 - y1
    denominator = 2 * y1 if x1 == x2 else x2 - x1
    slope = numerator * pow(denominator, -1, p) % p
    x3 = (slope * slope - x1 - x2) % p
    return x3, (slope * (x1 - x3) - y1) % p


def square_root(value, p):
    value %= p
    if value == 0:
        return 0
    if pow(value, (p - 1) // 2, p) != 1:
        return None
    q, s = p - 1, 0
    while q % 2 == 0:
        q //= 2
        s += 1
    if s == 1:
        return pow(value, (p + 1) // 4, p)
    z = 2
    while pow(z, (p - 1) // 2, p) != p - 1:
        z += 1
    c, root, t, m = pow(z, q, p), pow(value, (q + 1) // 2, p), pow(value, q, p), s
    while t != 1:
        i, squared = 0, t
        while squared != 1:
            squared = squared * squared % p
            i += 1
        step = pow(c, 1 << (m - i - 1), p)
        root = root * step % p
        t = t * step * step % p
        c = step * step % p
        m = i
    return root


def points_at_x(x, a, b, p):
    y = square_root(x**3 + a * x + b, p)
    if y is None:
        return ()
    if y == 0:
        return ((x, 0),)
    return ((x, y), (x, (-y) % p))


def linear_roots(poly, variable, p, stats):
    polynomial = sp.Poly(poly, variable, modulus=p)
    if polynomial.is_zero:
        stats["degenerate_gcds"] += 1
        raise DegenerateGCD
    stats["factorizations"] += 1
    _unit, factors = sp.factor_list(polynomial, modulus=p)
    roots = set()
    for factor, _multiplicity in factors:
        if factor.degree() == 1:
            leading, constant = (int(v) % p for v in factor.all_coeffs())
            roots.add((-constant * pow(leading, -1, p)) % p)
    return tuple(sorted(roots))


def primitive_root_power_two(p, height):
    if (p - 1) % (1 << height):
        raise ValueError("tower height does not divide p-1")
    for generator in range(2, p):
        root = pow(generator, (p - 1) >> height, p)
        if pow(root, 1 << (height - 1), p) != 1:
            return root
    raise ValueError("primitive root unavailable")


def extract(p, a, b, height, g, target, max_seconds):
    if not sp.isprime(p) or height < 1 or g % p == 0:
        raise ValueError("invalid prime, height, or coset representative")
    if (4 * a**3 + 27 * b**2) % p == 0:
        raise ValueError("singular curve")
    if (target[1] ** 2 - target[0] ** 3 - a * target[0] - b) % p:
        raise ValueError("target is not on the curve")
    root = primitive_root_power_two(p, height)
    x1, x2, x3, u = sp.symbols("x1 x2 x3 u")
    base_constant = pow(g, 1 << height, p)
    base = lambda x: x ** (1 << height) - base_constant
    stats = {"resultants": 0, "attempted_resultants": 0,
             "gcds": 0, "factorizations": 0,
             "degenerate_gcds": 0, "candidate_coordinate_tuples": 0,
             "exceptional_branch_points": 0,
             "stage_wall_s": {}, "polynomials": {}}
    started = time.monotonic()
    usage_before = resource.getrusage(resource.RUSAGE_SELF)
    signal.signal(signal.SIGALRM, timeout_handler)
    signal.setitimer(signal.ITIMER_REAL, max_seconds)
    witnesses = set()
    try:
        # S3 has no coordinate for the intermediate identity point. When
        # P1+P2=O, P3 must equal R. Extract this branch explicitly.
        if pow(target[0], 1 << height, p) == base_constant:
            for first in linear_roots(sp.Poly(base(x1), x1, modulus=p), x1, p, stats):
                for point in points_at_x(first, a, b, p):
                    stats["exceptional_branch_points"] += 1
                    opposite = (point[0], (-point[1]) % p)
                    if add(add(point, opposite, a, p), target, a, p) == target:
                        witnesses.add((point, opposite, target))
        stage = time.monotonic()
        stats["active_stage"] = "eliminate_x3"
        stats["attempted_resultants"] += 1
        h = sp.Poly(sp.resultant(base(x3), s3(u, x3, target[0], a, b), x3), u, modulus=p)
        stats["resultants"] += 1
        stats["stage_wall_s"]["eliminate_x3"] = time.monotonic() - stage
        stats["polynomials"]["h"] = {"degree": h.degree(), "terms": len(h.terms())}

        stage = time.monotonic()
        stats["active_stage"] = "eliminate_u"
        stats["attempted_resultants"] += 1
        j = sp.Poly(sp.resultant(h.as_expr(), s3(x1, x2, u, a, b), u), x1, x2, modulus=p)
        stats["resultants"] += 1
        stats["stage_wall_s"]["eliminate_u"] = time.monotonic() - stage
        stats["polynomials"]["j"] = {"degree_x1": j.degree(x1),
                                      "degree_x2": j.degree(x2), "terms": len(j.terms())}

        stage = time.monotonic()
        stats["active_stage"] = "eliminate_x2_and_gcd"
        stats["attempted_resultants"] += 1
        reduced = sp.rem(j.as_expr(), base(x2), x2)
        k = sp.Poly(sp.resultant(reduced, base(x2), x2), x1, modulus=p)
        stats["resultants"] += 1
        first_gcd = sp.gcd(k, sp.Poly(base(x1), x1, modulus=p))
        stats["gcds"] += 1
        stats["stage_wall_s"]["eliminate_x2_and_gcd"] = time.monotonic() - stage
        stats["polynomials"]["k"] = {"degree": k.degree(), "terms": len(k.terms())}
        stats["polynomials"]["first_gcd"] = {"degree": first_gcd.degree()}

        for first in linear_roots(first_gcd, x1, p, stats):
            second_gcd = sp.gcd(sp.Poly(j.as_expr().subs(x1, first), x2, modulus=p),
                                sp.Poly(base(x2), x2, modulus=p))
            stats["gcds"] += 1
            for second in linear_roots(second_gcd, x2, p, stats):
                middle_gcd = sp.gcd(h, sp.Poly(s3(first, second, u, a, b), u, modulus=p))
                stats["gcds"] += 1
                for middle in linear_roots(middle_gcd, u, p, stats):
                    third_gcd = sp.gcd(sp.Poly(base(x3), x3, modulus=p),
                                       sp.Poly(s3(middle, x3, target[0], a, b), x3, modulus=p))
                    stats["gcds"] += 1
                    for third in linear_roots(third_gcd, x3, p, stats):
                        stats["candidate_coordinate_tuples"] += 1
                        for points in itertools.product(points_at_x(first, a, b, p),
                                                        points_at_x(second, a, b, p),
                                                        points_at_x(third, a, b, p)):
                            if add(add(points[0], points[1], a, p), points[2], a, p) == target:
                                witnesses.add(points)
        status = "complete"
    except TimedOut:
        status = "censored"
    except DegenerateGCD:
        status = "unresolved_degenerate_gcd"
    finally:
        signal.setitimer(signal.ITIMER_REAL, 0)
    stats["wall_s"] = time.monotonic() - started
    usage_after = resource.getrusage(resource.RUSAGE_SELF)
    stats["process_cpu_s"] = (usage_after.ru_utime + usage_after.ru_stime
                            - usage_before.ru_utime - usage_before.ru_stime)
    rss_divisor = 1024 * 1024 if sys.platform == "darwin" else 1024
    stats["peak_rss_mib"] = usage_after.ru_maxrss / rss_divisor
    return {"status": status, "p": p, "a": a, "b": b, "height": height,
            "g": g, "root": root, "target": target, "stats": stats,
            "witnesses": sorted(witnesses)}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--p", type=int, default=97)
    parser.add_argument("--a", type=int, default=2)
    parser.add_argument("--b", type=int, default=3)
    parser.add_argument("--height", type=int, default=3)
    parser.add_argument("--g", type=int, default=1)
    parser.add_argument("--target-x", type=int, default=92)
    parser.add_argument("--target-y", type=int, default=16)
    parser.add_argument("--max-seconds", type=float, default=60.0)
    args = parser.parse_args()
    result = extract(args.p, args.a, args.b, args.height, args.g,
                     (args.target_x, args.target_y), args.max_seconds)
    print(json.dumps(result, sort_keys=True))


if __name__ == "__main__":
    main()
