#!/usr/bin/env python3
"""Preregistered toy: coverage of m=2 sums for algebraic-membership bases vs size-matched random bases. Stdlib only."""
import json, math, random, sys, time

def inv(a, p): return pow(a, -1, p)
def add(P, Q, a, p):
    if P is None: return Q
    if Q is None: return P
    x1, y1 = P; x2, y2 = Q
    if x1 == x2:
        if (y1 + y2) % p == 0: return None
        l = (3 * x1 * x1 + a) * inv(2 * y1, p) % p
    else:
        l = (y2 - y1) * inv(x2 - x1, p) % p
    x3 = (l * l - x1 - x2) % p
    return (x3, (l * (x1 - x3) - y1) % p)

def is_prime(n):
    if n < 2: return False
    i = 2
    while i * i <= n:
        if n % i == 0: return False
        i += 1
    return True

def make_group(p, rng):
    """Cyclic curve group: return (N, xs) where xs[k] = x(kG) for k=1..N-1."""
    sq = {}
    for y in range(p): sq.setdefault(y * y % p, []).append(y)
    while True:
        a, b = rng.randrange(1, p), rng.randrange(1, p)
        if (4 * a ** 3 + 27 * b * b) % p == 0: continue
        pts = [(x, y) for x in range(p) for y in sq.get((x ** 3 + a * x + b) % p, [])]
        N = len(pts) + 1
        G = rng.choice(pts)
        xs = [None]; Q = G
        while Q is not None:
            xs.append(Q[0]); Q = add(Q, G, a, p)
            if len(xs) > N + 1: break
        if len(xs) - 1 + 1 == N or len(xs) == N:  # order N => cyclic
            if len(xs) == N: return N, xs

def coverage(F, N):
    """fraction of Z_N hit by +-f_i +- f_j (i<=j)."""
    S = set(); L = list(F)
    for i, u in enumerate(L):
        for v in L[i:]:
            S.add((u + v) % N); S.add((u - v) % N); S.add((-u + v) % N); S.add((-u - v) % N)
    return len(S) / N

def families(p, N, xs, B):
    """each family: list of indices k (1..N-1), distinct up to sign, with x(k) in an algebraic locus."""
    qr = lambda x: pow(x, (p - 1) // 2, p) == 1
    g3 = math.gcd(3, p - 1)
    cube = (lambda x: pow(x, (p - 1) // g3, p) == 1) if g3 == 3 else qr
    dick = set((t ** 3 - 3 * t) % p for t in range(p))      # Dickson D_3 image
    sqimg = set(t * t % p for t in range(p))
    quad = set((t * t + t) % p for t in range(p))           # image of t^2+t
    preds = {"x_QR_char_const": qr, "x_cube_or_QR": cube, "x_Dickson3_image": dick.__contains__,
             "x_square_image_t2": sqimg.__contains__, "x_image_t2_plus_t": quad.__contains__,
             "x_interval_lowest": lambda x: x < p // 4}
    xmap = {}
    for k in range(1, N // 2 + 1): xmap.setdefault(xs[k], k)
    out = {}
    for name, f in preds.items():
        sel = []
        for x in sorted(xmap):
            if f(x):
                sel.append(xmap[x])
                if len(sel) == B: break
        out[name] = sel
    # coset-of-index-space-subgroup family (algebraic subgroup of E): multiples of d, d = N//B-ish divisor
    best = max([d for d in range(2, N) if N % d == 0 and N // d <= B] or [None], key=lambda d: -1 if d is None else N // d, default=None)
    return out, best

def one_cell(p, seed, B_factor, R):
    rng = random.Random(seed)
    N, xs = make_group(p, rng)
    B = max(8, int(B_factor * math.sqrt(N)))
    fam, d = families(p, N, xs, B)
    rnd = [coverage(rng.sample(range(1, N // 2 + 1), B), N) for _ in range(R)]
    aa = [coverage(rng.sample(range(1, N // 2 + 1), B), N) for _ in range(R)]
    mu = sum(rnd) / R; sd = math.sqrt(sum((c - mu) ** 2 for c in rnd) / (R - 1)) or 1e-12
    rows = {}
    for name, F in fam.items():
        if len(F) < B: rows[name] = None; continue
        c = coverage(F, N); rows[name] = {"ratio": c / mu, "z": (c - mu) / sd}
    # positive control: planted subgroup (multiples of d) -- known to destroy coverage
    pos = None
    if d and N // d >= 3:
        F = [d * j for j in range(1, N // d // 2 + 1)] or [d]
        # pad with the subgroup only (size-matched random reference uses same size)
        ref = [coverage(rng.sample(range(1, N // 2 + 1), len(F)), N) for _ in range(R)]
        m2 = sum(ref) / R; s2 = math.sqrt(sum((c - m2) ** 2 for c in ref) / (R - 1)) or 1e-12
        c = coverage(F, N); pos = {"size": len(F), "ratio": c / m2, "z": (c - m2) / s2}
    a = coverage(rng.sample(range(1, N // 2 + 1), B), N)
    return {"p": p, "N": N, "B": B, "families": rows, "positive_control": pos,
            "aa": {"ratio": a / mu, "z": (a - mu) / sd}, "rand_mean": mu, "rand_sd": sd}

def main():
    smoke = "--smoke" in sys.argv
    t0 = time.time()
    primes = [509] if smoke else None
    if not smoke:
        primes = []
        for lo in (1 << 11, 1 << 13, 1 << 15, 1 << 17):
            q = lo
            while not is_prime(q): q += 1
            primes.append(q)
    seeds = [20261110] if smoke else [20261110, 20261111, 20261112]
    R = 20 if smoke else 60
    cells = []
    for p in primes:
        for s in seeds:
            if time.time() - t0 > 420: break
            cells.append(one_cell(p, s * 1000003 + p, 1.5, R))
    names = [n for c in cells for n in c["families"]]
    names = sorted(set(names))
    pooled = {}
    for n in names:
        rs = [c["families"][n] for c in cells if c["families"].get(n)]
        if not rs: continue
        pooled[n] = {"cells": len(rs), "mean_ratio": sum(r["ratio"] for r in rs) / len(rs),
                     "stouffer_z": sum(r["z"] for r in rs) / math.sqrt(len(rs))}
    aa = [c["aa"] for c in cells]
    aa_z = sum(r["z"] for r in aa) / math.sqrt(len(aa))
    pos = [c["positive_control"] for c in cells if c["positive_control"]]
    pos_ok = bool(pos) and all(r["ratio"] < 0.8 for r in pos)
    gain = [n for n, v in pooled.items() if v["mean_ratio"] >= 1.02 and v["stouffer_z"] > 5]
    if smoke: verdict = "inconclusive"
    elif abs(aa_z) > 4 or not pos_ok: verdict = "inconclusive"
    elif gain: verdict = "falsified"
    else: verdict = "consistent_with_prediction"
    print(json.dumps({"lead": "LEAD-ICP-algebraic-membership-factor-base", "smoke": smoke,
        "statistic": "mean coverage ratio (|+-F+-F| / N) vs size-matched random base; Stouffer z",
        "thresholds": {"gain": "ratio>=1.02 and z>5", "aa_abs_z_max": 4, "pos_ratio_max": 0.8},
        "seeds": seeds, "primes": primes, "B_factor": 1.5, "random_replicates": R,
        "pooled": pooled, "aa_stouffer_z": aa_z, "positive_control_ok": pos_ok,
        "gain_families": gain, "cells": cells, "elapsed_s": time.time() - t0, "verdict": verdict}))

if __name__ == "__main__":
    main()
