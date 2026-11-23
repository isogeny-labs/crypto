#!/usr/bin/env python3
"""Fixtures for EXP-PKM-IC-001: for each size, the prime p = c·2^k + 1 just below 2^bits with 2^k | p − 1 (k ≥ 12),
and the first (a, b) (a = p − 3, b = 1, 2, …) with prime #E(F_p). The order is found by the Hasse-interval scan of
ic_boundary::PrimeCurve::prime_order (a single prime multiple of a point inside [p+1−2√p, p+1+2√p]); the generator is
the first affine point. Deterministic; writes curves.json."""
import json
import math
import os


def is_prime(n):
    if n < 2:
        return False
    for q in (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37):
        if n % q == 0:
            return n == q
    d, s = n - 1, 0
    while d % 2 == 0:
        d //= 2
        s += 1
    for a in (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37):
        x = pow(a, d, n)
        if x in (1, n - 1):
            continue
        for _ in range(s - 1):
            x = x * x % n
            if x == n - 1:
                break
        else:
            return False
    return True


def sqrt_mod(v, p):
    v %= p
    if v == 0:
        return 0
    if pow(v, (p - 1) // 2, p) != 1:
        return None
    # Tonelli-Shanks
    q, s = p - 1, 0
    while q % 2 == 0:
        q //= 2
        s += 1
    z = 2
    while pow(z, (p - 1) // 2, p) != p - 1:
        z += 1
    m, c, t, r = s, pow(z, q, p), pow(v, q, p), pow(v, (q + 1) // 2, p)
    while t != 1:
        i, t2 = 0, t
        while t2 != 1:
            t2 = t2 * t2 % p
            i += 1
        b = pow(c, 1 << (m - i - 1), p)
        m, c, t, r = i, b * b % p, t * b * b % p, r * b % p
    return r


def add(P, Q, a, p):
    if P is None:
        return Q
    if Q is None:
        return P
    (x1, y1), (x2, y2) = P, Q
    if x1 == x2 and (y1 + y2) % p == 0:
        return None
    lam = ((3 * x1 * x1 + a) * pow(2 * y1, -1, p) if P == Q else (y2 - y1) * pow(x2 - x1, -1, p)) % p
    x3 = (lam * lam - x1 - x2) % p
    return (x3, (lam * (x1 - x3) - y1) % p)


def mul(P, k, a, p):
    R = None
    while k:
        if k & 1:
            R = add(R, P, a, p)
        P = add(P, P, a, p)
        k >>= 1
    return R


def prime_order(a, b, p):
    pt = None
    for x in range(p):
        y = sqrt_mod(x ** 3 + a * x + b, p)
        if y is not None:
            pt = (x, y)
            break
    t = math.isqrt(4 * p)
    lo = p + 1 - t
    acc = mul(pt, lo, a, p)
    hits = []
    for j in range(2 * t + 1):
        if acc is None:
            hits.append(lo + j)
            if len(hits) > 1:
                return None, pt
        acc = add(acc, pt, a, p)
    return (hits[0] if len(hits) == 1 and is_prime(hits[0]) else None), pt


def main():
    out = []
    for bits, k in ((20, 12), (24, 14), (28, 16)):
        c = (1 << bits) // (1 << k)
        while True:
            p = c * (1 << k) + 1
            if p < (1 << bits) and is_prime(p):
                break
            c -= 1
        two_adic = ((p - 1) & -(p - 1)).bit_length() - 1
        a = p - 3
        b = 1
        while True:
            if (4 * a ** 3 + 27 * b * b) % p:
                n, g = prime_order(a, b, p)
                if n:
                    break
            b += 1
        assert mul(g, n, a, p) is None
        out.append({"bits": bits, "name": f"pkm-ic-{bits}bit", "p": p, "two_adic_valuation_p_minus_1": two_adic,
                    "a": a, "b": b, "gx": g[0], "gy": g[1], "n": n})
        print(out[-1], flush=True)
    json.dump(out, open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "curves.json"), "w"), indent=1)


if __name__ == "__main__":
    main()
