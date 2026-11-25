#!/usr/bin/env python3
"""Experimental PKM three-summand prefix sieve with certified ideal pruning.

The candidate keeps Kummer exponent classes as equations.  A Groebner basis
containing 1 is a sound rejection certificate over the algebraic closure.
Every surviving leaf is checked with elliptic-curve arithmetic.  This small
prototype reports its work; it makes no performance claim by itself.
"""

import argparse
import itertools
import json
import signal
import time

import sympy as sp


def s3(x, y, z, a, b):
    return (x - y) ** 2 * z**2 - 2 * ((x + y) * (x * y + a) + 2 * b) * z + (x * y - a) ** 2 - 4 * b * (x + y)


def add(p1, p2, a, p):
    if p1 is None:
        return p2
    if p2 is None:
        return p1
    x1, y1 = p1
    x2, y2 = p2
    if x1 == x2 and (y1 + y2) % p == 0:
        return None
    if x1 == x2:
        slope = (3 * x1 * x1 + a) * pow(2 * y1, -1, p) % p
    else:
        slope = (y2 - y1) * pow(x2 - x1, -1, p) % p
    x3 = (slope * slope - x1 - x2) % p
    return x3, (slope * (x1 - x3) - y1) % p


def roots_of_curve(x, a, b, p):
    rhs = (x**3 + a * x + b) % p
    return [(x, y) for y in range(p) if y * y % p == rhs]


def primitive_root_power_two(p, t):
    assert (p - 1) % (1 << t) == 0
    for h in range(2, p):
        zeta = pow(h, (p - 1) >> t, p)
        if pow(zeta, 1 << (t - 1), p) != 1:
            return zeta
    raise ValueError("no primitive power-of-two root")


class SolverTimeout(Exception):
    pass


def alarm_handler(_signum, _frame):
    raise SolverTimeout


def solve_prefix(p, a, b, t, g, target, max_seconds):
    zeta = primitive_root_power_two(p, t)
    x1, x2, x3, u = sp.symbols("x1 x2 x3 u")
    chain = [s3(x1, x2, u, a, b), s3(u, x3, target[0], a, b)]
    state = {"nodes": 0, "certificates": 0, "unresolved": 0, "leaves": 0, "verified": 0, "solver_wall_s": 0.0}
    witnesses = []
    deadline = time.monotonic() + max_seconds
    signal.signal(signal.SIGALRM, alarm_handler)

    def visit(depths, residues):
        if time.monotonic() >= deadline:
            raise SolverTimeout
        state["nodes"] += 1
        polynomials = list(chain)
        for variable, k, residue in zip((x1, x2, x3), depths, residues):
            degree = 1 << (t - k)
            fixed = g * pow(zeta, residue, p) % p
            polynomials.append(variable**degree - pow(fixed, degree, p))
        remaining = max(0.001, deadline - time.monotonic())
        signal.setitimer(signal.ITIMER_REAL, remaining)
        started = time.monotonic()
        try:
            basis = sp.groebner(polynomials, x1, x2, x3, u, modulus=p, order="grevlex")
        except SolverTimeout:
            state["unresolved"] += 1
            raise
        finally:
            signal.setitimer(signal.ITIMER_REAL, 0)
            state["solver_wall_s"] += time.monotonic() - started
        if any(poly.is_one for poly in basis.polys):
            state["certificates"] += 1
            return
        if all(k == t for k in depths):
            state["leaves"] += 1
            values = [g * pow(zeta, residue, p) % p for residue in residues]
            for points in itertools.product(*(roots_of_curve(x, a, b, p) for x in values)):
                if add(add(points[0], points[1], a, p), points[2], a, p) == target:
                    state["verified"] += 1
                    witnesses.append(points)
            return
        i = state_depth_index(depths)
        next_depths = list(depths)
        next_depths[i] += 1
        for bit in (0, 1):
            next_residues = list(residues)
            next_residues[i] += bit << depths[i]
            visit(tuple(next_depths), tuple(next_residues))

    try:
        visit((0, 0, 0), (0, 0, 0))
        status = "complete"
    except SolverTimeout:
        status = "censored"
    return {"status": status, "p": p, "a": a, "b": b, "t": t, "g": g, "zeta": zeta,
            "target": target, "work": state, "witnesses": witnesses}


def state_depth_index(depths):
    """Split the least advanced coordinate, cycling ties x1, x2, x3."""
    return min(range(3), key=lambda i: (depths[i], i))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--p", type=int, default=97)
    parser.add_argument("--a", type=int, default=2)
    parser.add_argument("--b", type=int, default=3)
    parser.add_argument("--t", type=int, default=3)
    parser.add_argument("--g", type=int, default=1)
    parser.add_argument("--target-x", type=int, default=92)
    parser.add_argument("--target-y", type=int, default=16)
    parser.add_argument("--max-seconds", type=float, default=10.0)
    args = parser.parse_args()
    result = solve_prefix(args.p, args.a, args.b, args.t, args.g,
                          (args.target_x, args.target_y), args.max_seconds)
    print(json.dumps(result, sort_keys=True))


if __name__ == "__main__":
    main()
