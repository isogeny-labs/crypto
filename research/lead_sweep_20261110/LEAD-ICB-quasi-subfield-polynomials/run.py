import json, random, sys
SMOKE = "--smoke" in sys.argv

def field(n):
    # find primitive polynomial by brute force; build exp/log tables
    for poly in range((1 << n) | 1, 1 << (n + 1), 2):
        e = [0] * (2 * (1 << n)); x = 1; ok = True; seen = 0
        q = (1 << n) - 1
        for i in range(q):
            e[i] = x
            x <<= 1
            if x >> n: x ^= poly
            if x == 1 and i < q - 1: ok = False; break
        if ok and x == 1:
            for i in range(q, 2 * (1 << n)): e[i] = e[i - q]
            lg = [0] * (1 << n)
            for i in range(q): lg[e[i]] = i
            return e, lg, q
    raise RuntimeError

def roots_qsp(F, k, a, b):
    e, lg, q = F; ex = (1 << k) + 1; la = lg[a] if a else None; c = 0
    for x in range(1 << len(bin(q)) if False else q + 1):
        v = b
        if x:
            lx = lg[x]
            v ^= e[(ex * lx) % q]
            if a: v ^= e[la + lx]
        if v == 0: c += 1
    return c

def roots_dense(F, coefs):
    e, lg, q = F; c = 0
    for x in range(q + 1):
        v = 0
        for co in coefs:  # Horner
            if v and x: v = e[lg[v] + lg[x]]
            else: v = 0
            v ^= co
        if v == 0: c += 1
    return c

def stats(xs):
    m = sum(xs) / len(xs)
    h = {}
    for x in xs: h[x] = h.get(x, 0) + 1
    return {"mean": round(m, 4), "max": max(xs), "hist": dict(sorted(h.items()))}

def main():
    N = 40 if SMOKE else 1500
    ns = [7] if SMOKE else [11, 13]
    seed = 20261110; out = {"protocol": "LEAD-ICB-quasi-subfield-polynomials", "seed": seed,
        "N_per_cell": N, "smoke": SMOKE, "cells": []}
    falsified = False; incons = False
    for n in ns:
        F = field(n); q = F[2]; rng = random.Random(seed + n)
        for k in (2, 3):
            qs = [roots_qsp(F, k, rng.randrange(1, q + 1), rng.randrange(0, q + 1)) for _ in range(N)]
            h = N // 2
            rd = [roots_dense(F, [1] + [rng.randrange(0, q + 1) for _ in range((1 << k) + 1)]) for _ in range(N // 3 or 1)]
            cell = {"n": n, "k": k, "qsp": stats(qs), "aa_half1_mean": sum(qs[:h]) / h,
                    "aa_half2_mean": sum(qs[h:]) / (N - h), "null_dense": stats(rd),
                    "subspace_size_needed": 1 << k}
            out["cells"].append(cell)
            if max(qs) > 3: falsified = True      # Bluher: gcd(k,n)=1 -> <=3 roots
            if abs(cell["aa_half1_mean"] - cell["aa_half2_mean"]) > 0.35: incons = True
    # positive control: composite n, gcd(k,n)=2 should reach 5 roots
    nc = 6 if SMOKE else 12
    F = field(nc); q = F[2]; rng = random.Random(seed + 99)
    pc = [roots_qsp(F, 2, rng.randrange(1, q + 1), rng.randrange(0, q + 1)) for _ in range(N if SMOKE else 3000)]
    out["positive_control"] = {"n": nc, "k": 2, **stats(pc), "expects_max": 5}
    if max(pc) < 5: incons = True   # detector cannot see effect
    out["verdict"] = "falsified" if falsified else ("inconclusive" if incons else "consistent_with_prediction")
    print(json.dumps(out))
main()
