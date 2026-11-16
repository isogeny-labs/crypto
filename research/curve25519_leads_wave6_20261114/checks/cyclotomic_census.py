#!/usr/bin/env python3
"""Cyclotomic-torus census for Curve25519: for k = 1..12, factor Phi_k(p) by trial division (to 2e6) and
Pollard-Brent rho (time-capped), then report the largest subgroup order |V| that divides Phi_k(p) and uses
only prime factors <= B, for B in {2^8, 2^12, 2^16, 2^20}. Under the solving-degree law SD >= e_max + 1, a base
mu_d inside the degree-k torus with e_max <= B has |V| <= that number. stdlib only."""
import math, random, sys, time
p = 2**255 - 19

def cyclotomic(k, x):
    # Phi_k(x) via Moebius product over divisors
    def mobius(n):
        r, d = 1, 2
        while d * d <= n:
            if n % d == 0:
                n //= d
                if n % d == 0: return 0
                r = -r
            d += 1
        return -r if n > 1 else r
    num, den = 1, 1
    for d in range(1, k + 1):
        if k % d == 0:
            mu = mobius(k // d)
            if mu == 1: num *= x**d - 1
            elif mu == -1: den *= x**d - 1
    return num // den

def is_prp(n):
    if n < 2: return False
    for q in (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37):
        if n % q == 0: return n == q
    d, s = n - 1, 0
    while d % 2 == 0: d //= 2; s += 1
    for a in (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37):
        x = pow(a, d, n)
        if x in (1, n - 1): continue
        for _ in range(s - 1):
            x = x * x % n
            if x == n - 1: break
        else: return False
    return True

def brent(n, limit_s):
    if n % 2 == 0: return 2
    t0 = time.time()
    while time.time() - t0 < limit_s:
        y, c, m = random.randrange(1, n), random.randrange(1, n), 128
        g = r = q = 1
        while g == 1:
            x = y
            for _ in range(r): y = (y * y + c) % n
            k = 0
            while k < r and g == 1:
                ys = y
                for _ in range(min(m, r - k)):
                    y = (y * y + c) % n
                    q = q * abs(x - y) % n
                g = math.gcd(q, n); k += m
            r *= 2
            if time.time() - t0 > limit_s: return None
        if g == n:
            while True:
                ys = (ys * ys + c) % n
                g = math.gcd(abs(x - ys), n)
                if g > 1: break
        if g != n: return g
    return None

def factor(n, tdiv=2_000_000, rho_s=20):
    small, d = {}, 2
    while d <= tdiv and d * d <= n:
        while n % d == 0:
            small[d] = small.get(d, 0) + 1; n //= d
        d += 1 if d == 2 else 2
    rest = []
    stack = [n] if n > 1 else []
    while stack:
        m = stack.pop()
        if m == 1: continue
        if is_prp(m):
            small[m] = small.get(m, 0) + 1; continue
        f = brent(m, rho_s)
        if f is None or f == m:
            rest.append(m); continue
        stack += [f, m // f]
    return small, rest

random.seed(20261114)
print("k | Phi_k(p) bits | known prime factors (exp) | unfactored composite bits | max |V| with e_max<=2^8 / 2^12 / 2^16 / 2^20")
for k in range(1, 13):
    n = cyclotomic(k, p)
    fac, rest = factor(n)
    out = []
    for B in (2**8, 2**12, 2**16, 2**20):
        v = 1
        for q, e in fac.items():
            if q <= B: v *= q**e
        out.append(v)
    known = sorted((q, e) for q, e in fac.items())
    shown = ", ".join(f"{q}^{e}" if e > 1 else str(q) for q, e in known if q < 10**12)
    big = [q.bit_length() for q, e in known if q >= 10**12]
    print(f"{k:2d} | {n.bit_length():4d} | {shown} ; big prp bits {big} | {[r.bit_length() for r in rest]} | {out}")
    sys.stdout.flush()
