#!/usr/bin/env python3
"""Prime-order curves y^2 = x^3 - 3x + b over F_p for 34-40 bits, as ecbench `prime_explicit` specs
(ecbench's `prime_search` stops at 32 bits).  Deterministic in the seed.  The group order N is found
by baby-step giant-step on a random point P over the Hasse interval [p+1-2√p, p+1+2√p]: a curve is
kept only when exactly one m in that interval has [m]P = O and that m is prime (then ord P = m >
4√p, so N = m).  The generator is then checked: [N]G = O.
    make_curves.py BITS[,BITS...] SEED > curves.json
"""
import json, math, random, sys

def is_prime(n):
    if n < 2:
        return False
    for q in (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37):
        if n % q == 0:
            return n == q
    d, s = n - 1, 0
    while d % 2 == 0:
        d //= 2; s += 1
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

def add(P, Q, a, p):
    if P is None: return Q
    if Q is None: return P
    (x1, y1), (x2, y2) = P, Q
    if x1 == x2:
        if (y1 + y2) % p == 0: return None
        l = (3 * x1 * x1 + a) * pow(2 * y1, -1, p) % p
    else:
        l = (y2 - y1) * pow(x2 - x1, -1, p) % p
    x3 = (l * l - x1 - x2) % p
    return (x3, (l * (x1 - x3) - y1) % p)

def mul(P, k, a, p):
    R = None
    while k:
        if k & 1: R = add(R, P, a, p)
        P = add(P, P, a, p); k >>= 1
    return R

def neg(P, p):
    return None if P is None else (P[0], (-P[1]) % p)

def sqrt_mod(n, p):
    n %= p
    if n == 0: return 0
    if pow(n, (p - 1) // 2, p) != 1: return None
    if p % 4 == 3: return pow(n, (p + 1) // 4, p)
    q, s = p - 1, 0
    while q % 2 == 0: q //= 2; s += 1
    z = 2
    while pow(z, (p - 1) // 2, p) != p - 1: z += 1
    m, c, t, r = s, pow(z, q, p), pow(n, q, p), pow(n, (q + 1) // 2, p)
    while t != 1:
        i, tt = 0, t
        while tt != 1: tt = tt * tt % p; i += 1
        b = pow(c, 1 << (m - i - 1), p)
        m, c, t, r = i, b * b % p, t * b * b % p, r * b % p
    return r

def rand_point(a, b, p, rng):
    while True:
        x = rng.randrange(p)
        y = sqrt_mod(x ** 3 + a * x + b, p)
        if y is not None and y != 0:
            return (x, y)

def hasse_multiples(P, a, p):
    """Every m in the Hasse interval with [m]P = O (BSGS)."""
    lo = p + 1 - 2 * math.isqrt(p) - 2
    hi = p + 1 + 2 * math.isqrt(p) + 2
    w = hi - lo + 1
    k = math.isqrt(w) + 1
    baby, Q = {}, None
    for j in range(k):                       # Q = [j]P
        baby.setdefault(Q, []).append(j)
        Q = add(Q, P, a, p)
    step = neg(mul(P, k, a, p), p)           # -[k]P
    out = []
    R = mul(P, lo, a, p)                      # [lo + i k]P
    for i in range(k + 1):
        # [lo + i k]P + ... want [lo + i k + j]P = O  <=>  [j]P = -[lo + i k]P
        for j in baby.get(neg(R, p), []):
            m = lo + i * k + j
            if lo <= m <= hi:
                out.append(m)
        R = add(R, mul(P, k, a, p), a, p) if False else add(R, neg(step, p), a, p)
    return sorted(set(out))

def curve(bits, seed):
    rng = random.Random(f"{bits}:{seed}")
    while True:
        p = rng.randrange(1 << (bits - 1), 1 << bits) | 1
        if not is_prime(p): continue
        a = p - 3
        b = rng.randrange(1, p)
        if (4 * a ** 3 + 27 * b * b) % p == 0: continue
        P = rand_point(a, b, p, rng)
        ms = hasse_multiples(P, a, p)
        if len(ms) != 1 or not is_prime(ms[0]): continue
        n = ms[0]
        G = rand_point(a, b, p, rng)
        assert mul(G, n, a, p) is None
        return {"kind": "prime_explicit", "p": p, "a": a, "b": b, "group_order": n, "r": n, "gx": G[0], "gy": G[1]}

if __name__ == "__main__":
    bits = [int(b) for b in sys.argv[1].split(",")]
    seed = int(sys.argv[2])
    print(json.dumps([curve(b, seed) for b in bits], indent=1))
