#!/usr/bin/env python3
"""LEAD-TRF-cover-catalog-jacobian-transfer toy: even-degree genus-2 covers y^2=g(x^2).
Checks #J(F_p) = #E1(F_p)*#E2(F_p) (Jacobian splits over the degree-2 cover) vs a
no-symmetry sextic null. stdlib only."""
import json, random, sys

def is_prime(n):
    return n > 3 and all(n % d for d in range(2, int(n**.5) + 1))

def padd(a, b, p): return [(x + y) % p for x, y in zip(a, b)]

def sqfree(f, p):
    # f low->high coefficients over F_p; gcd(f,f')==1
    def trim(a):
        while a and a[-1] % p == 0: a.pop()
        return a
    a = trim(list(f)); b = trim([(i * c) % p for i, c in enumerate(f)][1:])
    while b:
        inv = pow(b[-1], p - 2, p)
        while len(a) >= len(b):
            q = a[-1] * inv % p; s = len(a) - len(b)
            for i, c in enumerate(b): a[s + i] = (a[s + i] - q * c) % p
            a = trim(a)
            if not a: break
        a, b = b, a
    return len(a) == 1

def sq_counts_Fp(p):
    c = [0] * p
    for x in range(p): c[x * x % p] += 1
    return c  # number of sqrt of each residue

def count_Fp(f, p, sq, inf):
    n = 0
    for x in range(p):
        v = 0
        for c in reversed(f): v = (v * x + c) % p
        n += sq[v]
    return n + inf

def count_Fp2(f, p, sq2, inf):
    # elements a+b*i, i^2=-1 (p=3 mod 4), encoded a*p+b
    n = 0
    for a in range(p):
        for b in range(p):
            va = vb = 0
            for c in reversed(f):
                va, vb = (va * a - vb * b + c) % p, (va * b + vb * a) % p
            n += sq2[va * p + vb]
    return n + inf

def sq_counts_Fp2(p):
    c = [0] * (p * p)
    for a in range(p):
        for b in range(p):
            c[((a * a - b * b) % p) * p + (2 * a * b) % p] += 1
    return c

def jac_order(f, p, sq, sq2):
    N1 = count_Fp(f, p, sq, 2); N2 = count_Fp2(f, p, sq2, 2)
    t = N1 * N1 + N2
    assert t % 2 == 0
    return t // 2 - p

def trial(p, rng, split):
    sq = sq_counts_Fp(p); sq2 = sq_counts_Fp2(p)
    while True:
        if split:
            a, b, c = rng.randrange(p), rng.randrange(p), rng.randrange(1, p)
            f = [c, 0, b, 0, a, 0, 1]
        else:
            f = [rng.randrange(p) for _ in range(6)] + [1]
            f[1] = f[1] or 1  # break evenness
        if not sqfree(f, p): continue
        J = jac_order(f, p, sq, sq2)
        if split:
            e1 = count_Fp([c, b, a, 1], p, sq, 1)
            e2 = count_Fp([1, a, b, c], p, sq, 1)
            return J == e1 * e2, J
        # null: compare with the same E1,E2 formulas applied to odd coefficients (no theory behind them)
        e1 = count_Fp([f[0], f[2], f[4], 1], p, sq, 1)
        e2 = count_Fp([1, f[4], f[2], f[0]], p, sq, 1)
        return J == e1 * e2, J

def arm(primes, n, seed, split):
    rng = random.Random(seed); hits = tot = 0; logs = []
    for i in range(n):
        p = primes[i % len(primes)]
        ok, J = trial(p, rng, split); hits += ok; tot += 1
    return hits, tot

def main():
    smoke = "--smoke" in sys.argv
    if smoke: primes, n = [7, 11], 6
    else: primes, n = [p for p in range(59, 140) if is_prime(p) and p % 4 == 3], 60
    pos = arm(primes, n, 20261110, True)
    aa = arm(primes, n, 20261111, True)
    null = arm(primes, n, 20261112, False)
    pr, ar, nr = pos[0] / pos[1], aa[0] / aa[1], null[0] / null[1]
    if pr == 1.0 and ar == 1.0 and nr <= 0.2: v = "consistent_with_prediction"
    elif pr < 1.0 or ar < 1.0: v = "falsified"
    else: v = "inconclusive"
    print(json.dumps({"lead": "LEAD-TRF-cover-catalog-jacobian-transfer", "smoke": smoke,
        "statistic": "fraction with #J(F_p)==#E1*#E2", "split_arm": pr, "aa_arm": ar,
        "null_arm_no_symmetry": nr, "threshold": {"split": 1.0, "null_max": 0.2},
        "seeds": [20261110, 20261111, 20261112], "primes": primes, "n_per_arm": n,
        "verdict": v}))
main()
