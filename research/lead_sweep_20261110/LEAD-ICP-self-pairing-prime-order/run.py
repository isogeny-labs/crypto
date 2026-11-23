#!/usr/bin/env python3
"""LEAD-ICP-self-pairing-prime-order: stdlib-only toy. See PROTOCOL.md."""
import json, random, sys

def isprime(n):
    if n < 2: return False
    i = 2
    while i * i <= n:
        if n % i == 0: return False
        i += 1
    return True

def primes_in(lo, hi):
    return [n for n in range(lo, hi) if isprime(n)]

def factor(n):
    f = {}; d = 2
    while d * d <= n:
        while n % d == 0: f[d] = f.get(d, 0) + 1; n //= d
        d += 1
    if n > 1: f[n] = f.get(n, 0) + 1
    return f

def mult_order(p, r):
    o = r - 1
    for q in factor(r - 1):
        while o % q == 0 and pow(p, o // q, r) == 1: o //= q
    return o

def count(p, a, b, sq):
    # #E = p+1+sum chi(x^3+ax+b); sq = bytearray of squares
    s = 0
    for x in range(p):
        v = (x * x * x + a * x + b) % p
        if v: s += 1 if sq[v] else -1
    return p + 1 + s

def squares(p):
    sq = bytearray(p)
    for y in range(1, p): sq[y * y % p] = 1
    return sq

def sqrt_mod(n, p):
    n %= p
    if n == 0: return 0
    if pow(n, (p - 1) // 2, p) != 1: return None
    if p % 4 == 3: return pow(n, (p + 1) // 4, p)
    q, s = p - 1, 0
    while q % 2 == 0: q //= 2; s += 1
    z = 2
    while pow(z, (p - 1) // 2, p) != p - 1: z += 1
    m, c, t, R = s, pow(z, q, p), pow(n, q, p), pow(n, (q + 1) // 2, p)
    while t != 1:
        i, t2 = 0, t
        while t2 != 1: t2 = t2 * t2 % p; i += 1
        b = pow(c, 1 << (m - i - 1), p)
        m, c, t, R = i, b * b % p, t * b * b % p, R * b % p
    return R

O = None
def add(P, Q, a, p):
    """returns (R, lam); lam None when vertical/identity"""
    if P is O: return Q, None
    if Q is O: return P, None
    if P[0] == Q[0] and (P[1] + Q[1]) % p == 0: return O, None
    if P == Q: lam = (3 * P[0] * P[0] + a) * pow(2 * P[1], -1, p) % p
    else: lam = (Q[1] - P[1]) * pow(Q[0] - P[0], -1, p) % p
    x = (lam * lam - P[0] - Q[0]) % p
    return (x, (lam * (P[0] - x) - P[1]) % p), lam

def mul(k, P, a, p):
    R = O
    while k:
        if k & 1: R, _ = add(R, P, a, p)
        P, _ = add(P, P, a, p); k >>= 1
    return R

def rand_point(p, a, b, rng):
    while True:
        x = rng.randrange(p); y = sqrt_mod(x ** 3 + a * x + b, p)
        if y is not None: return (x, y)

class Degenerate(Exception): pass

def line(T, lam, X, p):
    if lam is None: return (X[0] - T[0]) % p
    return (X[1] - T[1] - lam * (X[0] - T[0])) % p

def miller(P, r, X, a, p):
    f = 1; T = P
    for bit in bin(r)[3:]:
        R, lam = add(T, T, a, p)
        num = line(T, lam, X, p); den = (X[0] - R[0]) % p if R is not O else 1
        if num == 0 or den == 0: raise Degenerate
        f = f * f % p * num % p * pow(den, -1, p) % p; T = R
        if bit == '1':
            R, lam = add(T, P, a, p)
            num = line(T, lam, X, p); den = (X[0] - R[0]) % p if R is not O else 1
            if num == 0 or den == 0: raise Degenerate
            f = f * num % p * pow(den, -1, p) % p; T = R
    return f

def tate(P, Q, r, a, b, p, rng):
    """reduced Tate pairing t(P,Q) in F_p*/(F_p*)^r, exponent (p-1)/r (needs r | p-1)"""
    for _ in range(50):
        S = rand_point(p, a, b, rng)
        QS, _ = add(Q, S, a, p)
        try:
            v = miller(P, r, QS, a, p) * pow(miller(P, r, S, a, p), -1, p) % p
        except Degenerate:
            continue
        return pow(v, (p - 1) // r, p)
    raise RuntimeError("tate failed")

def neg_control(seed, plist, per_p, sq_cache):
    """prime-order curves: count those with gcd(r,p-1)>1 (Tate target nontrivial) and k"""
    rng = random.Random(seed); n = 0; bad = 0; kmin_ratio = 1.0; small_k = 0; ks = []
    for p in plist:
        sq = sq_cache.setdefault(p, squares(p)); found = 0; tries = 0
        while found < per_p and tries < 400:
            tries += 1
            a, b = rng.randrange(p), rng.randrange(p)
            if (4 * a ** 3 + 27 * b * b) % p == 0: continue
            N = count(p, a, b, sq)
            if not isprime(N) or N == p or N == p + 1: continue  # exclude anomalous / SS-order cases, recorded separately
            found += 1; n += 1
            k = mult_order(p, N)
            if (p - 1) % N == 0: bad += 1
            if k <= 2: small_k += 1
            kmin_ratio = min(kmin_ratio, k / (N - 1)); ks.append(k)
    return {"n_curves": n, "gcd_gt1": bad, "k_le_2": small_k, "min_k_over_rminus1": kmin_ratio}

def pos_control(seed, rlist, per_r, sq_cache):
    """subgroup of prime order r | p-1 inside non-prime-order E(F_p): Tate self-pairing"""
    rng = random.Random(seed); n = 0; nondeg = 0; bil_ok = 0; dl_ok = 0; detect = 0
    for r in rlist:
        got = 0; tries = 0
        while got < per_r and tries < 40:
            tries += 1
            m = rng.randrange(60, 130)
            p = 1 + r * m
            if not isprime(p): continue
            sq = sq_cache.setdefault(p, squares(p))
            for _ in range(6 * r):
                a, b = rng.randrange(p), rng.randrange(p)
                if (4 * a ** 3 + 27 * b * b) % p == 0: continue
                N = count(p, a, b, sq)
                if N % r == 0 and N != r and N % (r * r) != 0: break
            else:
                continue
            h = N // r
            P = O
            for _ in range(200):
                P = mul(h, rand_point(p, a, b, rng), a, p)
                if P is not O: break
            if P is O: continue
            if mul(r, P, a, p) is not O: continue
            got += 1; n += 1
            if (p - 1) % r == 0: detect += 1
            t = tate(P, P, r, a, b, p, rng)
            if t != 1: nondeg += 1
            t2 = tate(mul(2, P, a, p), P, r, a, b, p, rng)
            if t2 == pow(t, 2, p): bil_ok += 1
            # DL transfer through the pairing: t(dP,P)=t(P,P)^d
            d = rng.randrange(1, r); td = tate(mul(d, P, a, p), P, r, a, b, p, rng)
            if t != 1 and any(pow(t, x, p) == td for x in range(r)):
                if pow(t, d, p) == td: dl_ok += 1
    return {"n_curves": n, "gcd_gt1_detected": detect, "nondegenerate": nondeg, "bilinear_ok": bil_ok, "dl_transfer_ok": dl_ok}

def main():
    smoke = "--smoke" in sys.argv
    if smoke:
        plist = primes_in(300, 330)[:3]; per_p = 2; rlist = [7, 11]; per_r = 1
    else:
        plist = primes_in(2000, 2100)[:4] + primes_in(8000, 8100)[:3] + primes_in(30000, 30100)[:2]
        per_p = 12; rlist = [13, 17, 19, 23, 29, 31]; per_r = 3
    cache = {}
    nA = neg_control(101, plist, per_p, cache); nB = neg_control(202, plist, per_p, cache)
    pA = pos_control(303, rlist, per_r, cache); pB = pos_control(404, rlist, per_r, cache)
    bad = nA["gcd_gt1"] + nB["gcd_gt1"]
    smallk = nA["k_le_2"] + nB["k_le_2"]
    pos_rate = (pA["nondegenerate"] + pB["nondegenerate"]) / max(1, pA["n_curves"] + pB["n_curves"])
    pos_ok = pos_rate == 1.0 and pA["bilinear_ok"] == pA["n_curves"] and pB["bilinear_ok"] == pB["n_curves"] \
        and pA["gcd_gt1_detected"] == pA["n_curves"]
    aa = abs(nA["gcd_gt1"] - nB["gcd_gt1"]) == 0
    if not pos_ok or pA["n_curves"] == 0 or nA["n_curves"] == 0: verdict = "inconclusive"
    elif bad == 0 and smallk == 0 and aa: verdict = "consistent_with_prediction"
    else: verdict = "falsified"
    print(json.dumps({"lead": "LEAD-ICP-self-pairing-prime-order", "smoke": smoke, "primes": plist, "rlist": rlist,
        "seeds": {"negA": 101, "negB": 202, "posA": 303, "posB": 404},
        "statistic": {"prime_order_curves_with_gcd_r_pminus1_gt1": bad, "prime_order_curves_with_k_le_2": smallk,
                      "pos_control_nondegenerate_rate": pos_rate},
        "negative_control_A": nA, "negative_control_B_AA": nB, "positive_control_A": pA, "positive_control_B_AA": pB,
        "detector_alive": pos_ok, "verdict": verdict}))
main()
