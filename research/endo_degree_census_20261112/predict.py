#!/usr/bin/env python3
"""Pre-data prediction for the endo-degree census (no curve is generated or counted).

For a prime p>3 and a uniformly random (a,b) with 4a^3+27b^2 != 0, the number of
(a,b) with Frobenius trace t is (p-1)*H(4p-t^2)/2 (Deuring, Lenstra), H the
Hurwitz-Kronecker class number.  Hence
    P(d_K = d | p) = (1/2p) * sum_{f>=1, 4p-|d|f^2 = t^2, t>0} 2*H(|d|f^2),
    H(|d|f^2) = sum_{c|f} h(d) c prod_{q|c}(1-(d/q)/q) / (w(d)/2).
Summing over the fundamental discriminants d, |d|<=44, gives P(|d_K|<=44 | p).
"""
import math, random, sys, json

def fund_discs(maxabs):
    out = []
    for n in range(3, maxabs + 1):
        d = -n
        if d % 4 == 1 and sqfree(n):
            out.append(d)
        elif d % 4 == 0:
            m = d // 4
            if m % 4 in (2, 3) and sqfree(-m):
                out.append(d)
    return out

def sqfree(n):
    q = 2
    while q * q <= n:
        if n % (q * q) == 0:
            return False
        q += 1
    return True

def h(d):
    cnt = 0
    a = 1
    while 3 * a * a <= -d:
        for b in range(-a + 1, a + 1):
            if (b - d) % 2:
                continue
            num = b * b - d
            if num % (4 * a):
                continue
            c = num // (4 * a)
            if c < a:
                continue
            if a == c and b < 0:
                continue
            cnt += 1
        a += 1
    return cnt

def kron(d, q):
    if q == 2:
        if d % 2 == 0:
            return 0
        return 1 if d % 8 in (1, 7) else -1
    r = pow(d % q, (q - 1) // 2, q)
    return 0 if r == 0 else (1 if r == 1 else -1)

def primefacs(n):
    fs, q = [], 2
    while q * q <= n:
        if n % q == 0:
            fs.append(q)
            while n % q == 0:
                n //= q
        q += 1
    if n > 1:
        fs.append(n)
    return fs

def divisors_with_rad(f):
    # returns list of (c, prod over q|c of (1-(d/q)/q) handled by caller)
    ds = [d for d in range(1, f + 1) if f % d == 0] if f < 5000 else None
    return ds

def Hd(d, f, hd, w2):
    # sum over c|f of hd*c*prod_{q|c}(1-(d/q)/q)/w2, using multiplicativity via factorization of f
    fac = {}
    n = f; q = 2
    while q * q <= n:
        while n % q == 0:
            fac[q] = fac.get(q, 0) + 1; n //= q
        q += 1
    if n > 1: fac[n] = fac.get(n, 0) + 1
    tot = 1.0
    for q, e in fac.items():
        s = 1.0
        for k in range(1, e + 1):
            s += q ** k * (1 - kron(d, q) / q)
        tot *= s
    return hd * tot / w2

def P(p, dmax=44):
    tot = 0.0
    for d in DISCS:
        hd = h(d); w2 = 3 if d == -3 else (2 if d == -4 else 1)
        f = 1
        while -d * f * f < 4 * p:
            s = 4 * p + d * f * f
            t = math.isqrt(s)
            if t * t == s and t > 0:
                tot += 2 * Hd(d, f, hd, w2)
            f += 1
    return tot / (2 * p)

def is_prime(n):
    if n < 2: return False
    for q in (2,3,5,7,11,13,17,19,23,29,31,37):
        if n % q == 0: return n == q
    d, s = n - 1, 0
    while d % 2 == 0: d //= 2; s += 1
    for a in (2,3,5,7,11,13,17,19,23,29,31,37):
        x = pow(a, d, n)
        if x in (1, n - 1): continue
        for _ in range(s - 1):
            x = x * x % n
            if x == n - 1: break
        else: return False
    return True

DISCS = fund_discs(44)
if __name__ == "__main__":
    print("fundamental discriminants |d|<=44:", len(DISCS), DISCS)
    rng = random.Random(20261112)
    res = {}
    for bits, nprimes in ((12, 400), (16, 400), (20, 300), (24, 200), (28, 100), (32, 60), (40, 30)):
        vals = []
        while len(vals) < nprimes:
            p = rng.randrange(1 << (bits - 1), 1 << bits) | 1
            if is_prime(p): vals.append(P(p))
        m = sum(vals) / len(vals)
        res[bits] = m
        print(bits, "mean P(|d_K|<=44) = %.6g   sqrt(p)*P = %.4f" % (m, m * math.sqrt(1.5 * 2 ** (bits - 1))))
    json.dump({str(k): v for k, v in res.items()}, open("predict.json", "w"), indent=1)
