#!/usr/bin/env python3
"""Find a prime-order curve y^2 = x^3 + a x + b over F_p, p ~ 2^bits, with a Hasse-interval BSGS.
Usage: gen_curve.py BITS SEED  -> prints a prime_explicit JSON (deterministic in SEED)."""
import random, sys, json, math
def is_prime(n):
    if n < 2: return False
    for q in (2,3,5,7,11,13,17,19,23,29,31,37):
        if n % q == 0: return n == q
    d, s = n-1, 0
    while d % 2 == 0: d //= 2; s += 1
    for a in (2,3,5,7,11,13,17,19,23,29,31,37):
        x = pow(a, d, n)
        if x in (1, n-1): continue
        for _ in range(s-1):
            x = x*x % n
            if x == n-1: break
        else: return False
    return True
def next_prime(n):
    n |= 1
    while not is_prime(n): n += 2
    return n
def add(P, Q, a, p):
    if P is None: return Q
    if Q is None: return P
    x1, y1 = P; x2, y2 = Q
    if x1 == x2:
        if (y1 + y2) % p == 0: return None
        l = (3*x1*x1 + a) * pow(2*y1, -1, p) % p
    else:
        l = (y2 - y1) * pow(x2 - x1, -1, p) % p
    x3 = (l*l - x1 - x2) % p
    return (x3, (l*(x1 - x3) - y1) % p)
def mul(k, P, a, p):
    R = None
    while k:
        if k & 1: R = add(R, P, a, p)
        P = add(P, P, a, p); k >>= 1
    return R
def sqrt_mod(n, p):  # p = 3 mod 4 or Tonelli-Shanks
    n %= p
    if n == 0: return 0
    if pow(n, (p-1)//2, p) != 1: return None
    if p % 4 == 3: return pow(n, (p+1)//4, p)
    q, s = p-1, 0
    while q % 2 == 0: q //= 2; s += 1
    z = 2
    while pow(z, (p-1)//2, p) != p-1: z += 1
    m, c, t, r = s, pow(z, q, p), pow(n, q, p), pow(n, (q+1)//2, p)
    while t != 1:
        i, t2 = 0, t
        while t2 != 1: t2 = t2*t2 % p; i += 1
        b = pow(c, 1 << (m-i-1), p)
        m, c, t, r = i, b*b % p, t*b*b % p, r*b % p
    return r
def main(bits, seed):
    rng = random.Random(seed)
    p = next_prime((1 << bits) + rng.randrange(1 << (bits-2)))
    while True:
        a, b = rng.randrange(p), rng.randrange(p)
        if (4*a**3 + 27*b*b) % p == 0: continue
        while True:
            x = rng.randrange(p); y = sqrt_mod(x**3 + a*x + b, p)
            if y is not None: break
        P = (x, y)
        w = int(2*math.isqrt(p)) + 2
        lo, hi = p + 1 - w, p + 1 + w
        s = math.isqrt(hi - lo) + 1
        baby = {}; jP = None
        for j in range(s):
            baby.setdefault(jP, j); jP = add(jP, P, a, p)
        R = mul(lo, P, a, p); step = mul(s, P, a, p)
        found = []
        for i in range(s + 1):
            if R in baby:
                m = lo + i*s - baby[R]
                if lo <= m <= hi: found.append(m)
            R = add(R, step, a, p)
        for m in found:
            if is_prime(m) and mul(m, P, a, p) is None:
                print(json.dumps({"kind": "prime_explicit", "p": p, "a": a, "b": b, "group_order": m, "r": m, "gx": P[0], "gy": P[1]}))
                return
main(int(sys.argv[1]), int(sys.argv[2]))
