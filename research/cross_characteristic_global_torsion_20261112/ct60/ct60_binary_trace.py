#!/usr/bin/env python3
"""Exact minimum extension degrees for every E/F_2 trace (ell <= 2^32)."""
from itertools import product
from math import isqrt

PRIMES = (3, 5, 7, 11, 17, 19, 23, 37, 43, 163, 257, 521,
          65537, 2147483647, 4294967291)
TRACES = (-2, -1, 0, 1, 2)


def require(condition, message):
    if not condition:
        raise RuntimeError(message)


def factors(n):
    distinct = []
    d = 2
    while d * d <= n:
        if n % d == 0:
            distinct.append(d)
            while n % d == 0:
                n //= d
        d += 1 if d == 2 else 2
    if n > 1:
        distinct.append(n)
    return distinct


def prime_trial(n):
    return n > 1 and not any(n % d == 0 for d in range(2, isqrt(n) + 1))


def sqrt_mod(n, p):
    n %= p
    if n == 0:
        return 0
    if pow(n, (p - 1) // 2, p) != 1:
        return None
    if p % 4 == 3:
        return pow(n, (p + 1) // 4, p)
    q = p - 1
    s = 0
    while q % 2 == 0:
        q //= 2
        s += 1
    z = 2
    while pow(z, (p - 1) // 2, p) != p - 1:
        z += 1
    c, x, t, m = pow(z, q, p), pow(n, (q + 1) // 2, p), pow(n, q, p), s
    while t != 1:
        i, w = 1, t * t % p
        while w != 1:
            w = w * w % p
            i += 1
        b = pow(c, 1 << (m - i - 1), p)
        x, t, c, m = x * b % p, t * b * b % p, b * b % p, i
    require(x * x % p == n, "modular sqrt mismatch")
    return x


def mul(a, b, trace, p):
    u, v = a
    r, s = b
    return (u * r - 2 * v * s) % p, (u * s + v * r + trace * v * s) % p


def extension_power(a, n, trace, p):
    out = (1, 0)
    while n:
        if n & 1:
            out = mul(out, a, trace, p)
        a = mul(a, a, trace, p)
        n //= 2
    return out


def multiplicative_order(element, bound, divisors, is_identity):
    for r in sorted(set(divisors)):
        while bound % r == 0 and is_identity(element, bound // r):
            bound //= r
    require(is_identity(element, bound), "order witness failed")
    for r in factors(bound):
        require(not is_identity(element, bound // r), "nonminimal order")
    return bound


def min_single_degree(ell, trace):
    discriminant = (trace * trace - 8) % ell
    root = sqrt_mod(discriminant, ell)
    if root is None:
        # Frob is represented by alpha with alpha^2 = trace*alpha - 2.
        order = multiplicative_order(
            (0, 1), ell * ell - 1, factors(ell - 1) + factors(ell + 1),
            lambda a, k: extension_power(a, k, trace, ell) == (1, 0))
        return order, "irreducible"
    inverse_two = pow(2, -1, ell)
    roots = ((trace + root) * inverse_two % ell,
             (trace - root) * inverse_two % ell)
    orders = [multiplicative_order(
        r, ell - 1, factors(ell - 1), lambda a, k: pow(a, k, ell) == 1)
        for r in roots]
    return min(orders), "repeated" if root == 0 else "split"


def trace_pointcount_mod(ell, trace, degree):
    u, v = 2, trace
    if degree == 1:
        return (3 - trace) % ell
    for _ in range(2, degree + 1):
        u, v = v, (trace * v - 2 * u) % ell
    return (pow(2, degree, ell) + 1 - v) % ell


def witnesses():
    witness = {}
    for c in product((0, 1), repeat=5):
        a1, a2, a3, a4, a6 = c
        if a1 == 0 and a3 != 1:
            continue
        if a1 == 1:
            x, y = a3, a3 ^ a4
            f = y*y + a1*x*y + a3*y - x**3 - a2*x*x - a4*x - a6
            if f % 2 == 0:
                continue
        size = 1 + sum(
            (y*y + a1*x*y + a3*y - x**3 - a2*x*x - a4*x - a6) % 2 == 0
            for x, y in product((0, 1), repeat=2))
        witness.setdefault(3 - size, c)
    require(set(witness) == set(TRACES), "missing actual curve witness")
    return witness


def run():
    for ell in PRIMES:
        require(prime_trial(ell), "input is composite")
    witness = witnesses()
    print("traces", ",".join(map(str, sorted(witness))))
    print("curve_witnesses", ";".join(
        str(t) + ":" + "".join(map(str, witness[t])) for t in TRACES))
    for ell in PRIMES:
        values = [min_single_degree(ell, t)[0] for t in TRACES]
        for t, f in zip(TRACES, values):
            if f <= 128:
                require(trace_pointcount_mod(ell, t, f) == 0,
                        "trace recurrence disagrees with eigenvalue order")
                for j in range(1, f):
                    require(trace_pointcount_mod(ell, t, j) != 0,
                            "smaller fixed point degree")
        best = min(values)
        print(str(ell) + "|" + ",".join(map(str, values)) + "|" + str(best))
    a = [multiplicative_order(r, 36, factors(36),
                              lambda z, k: pow(z, k, 37) == 1)
         for r in ((-1 + sqrt_mod((-7) % 37, 37)) * pow(2, -1, 37) % 37,
                   (-1 - sqrt_mod((-7) % 37, 37)) * pow(2, -1, 37) % 37)]
    require(sorted(a) == [12, 18], "incomparable eigen-order counterexample")
    print("counterexample_ell37_trace-1_eigen_orders=12,18")
    print("PASS exact-order-and-small-degree-recurrence")


if __name__ == "__main__":
    run()
