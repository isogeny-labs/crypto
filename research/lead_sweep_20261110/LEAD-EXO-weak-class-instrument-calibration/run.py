#!/usr/bin/env python3
"""Weak-class instrument calibration toy (stdlib only). See PROTOCOL.md."""
import sys, json, math, cmath, random, time

def is_prime(n):
    if n < 2: return False
    i = 2
    while i * i <= n:
        if n % i == 0: return False
        i += 1
    return True

def next_prime(n):
    while not is_prime(n): n += 1
    return n

def count_points(p, a, b, sq):
    # sq: Legendre table, sq[v] in {-1,0,1}
    s = 0
    for x in range(p):
        s += sq[(x * x * x + a * x + b) % p]
    return p + 1 + s

def add(P, Q, p, a):
    if P is None: return Q
    if Q is None: return P
    x1, y1 = P; x2, y2 = Q
    if x1 == x2:
        if (y1 + y2) % p == 0: return None
        l = (3 * x1 * x1 + a) * pow(2 * y1, -1, p) % p
    else:
        l = (y2 - y1) * pow(x2 - x1, -1, p) % p
    x3 = (l * l - x1 - x2) % p
    return (x3, (l * (x1 - x3) - y1) % p)

def find_point(p, a, b, rng):
    while True:
        x = rng.randrange(p)
        r = (x ** 3 + a * x + b) % p
        if pow(r, (p - 1) // 2, p) == 1 and r:
            # p = 3 mod 4 not assumed; use Tonelli-lite brute for small p
            for y in range(p):
                if y * y % p == r: return (x, y)

def labels_small_x(p, a, b, N, P, T):
    """DL (w.r.t. P) of every point with x < T."""
    out = []
    Q = None
    for k in range(1, N):
        Q = add(Q, P, p, a)
        if Q is not None and Q[0] < T: out.append(k)
    return out

def stat(dls, N, K):
    n = len(dls)
    if n == 0: return 0.0
    best = 0.0
    for k in range(1, K + 1):
        w = 2j * math.pi * k / N
        s = sum(cmath.exp(w * d) for d in dls)
        best = max(best, abs(s))
    return best / math.sqrt(n)

def curve_stat(p, a, b, N, rng, K, plant=0.0):
    P = find_point(p, a, b, rng)
    T = max(4, p // 8)
    dls = labels_small_x(p, a, b, N, P, T)
    if plant > 0:
        step = max(1, N // 7)
        dls = [(step * rng.randrange(7)) % N if rng.random() < plant else d for d in dls]
    return stat(dls, N, K)

def gen_class(p, sq, rng, kind, count, K, plant=0.0, maxtry=60000):
    vals, got, tries = [], 0, 0
    while got < count and tries < maxtry:
        tries += 1
        a, b = rng.randrange(p), rng.randrange(p)
        if (4 * a ** 3 + 27 * b * b) % p == 0: continue
        N = count_points(p, a, b, sq)
        ok = (N == p) if kind == "anomalous" else (is_prime(N) and N != p and N > 50)
        if ok:
            vals.append(curve_stat(p, a, b, N, rng, K, plant)); got += 1
    return vals, tries

def auc(x, y):
    # P(x > y) + 0.5 P(tie)
    c = 0.0
    for u in x:
        for v in y:
            c += 1.0 if u > v else (0.5 if u == v else 0.0)
    return c / (len(x) * len(y))

def perm_p(x, y, rng, reps):
    obs = abs(auc(x, y) - 0.5)
    z = x + y; n = len(x); hit = 0
    for _ in range(reps):
        rng.shuffle(z)
        if abs(auc(z[:n], z[n:]) - 0.5) >= obs: hit += 1
    return (hit + 1) / (reps + 1)

def main():
    smoke = "--smoke" in sys.argv
    t0 = time.time()
    if smoke: primes, per, K, reps, plant = [next_prime(300)], 4, 8, 50, 0.3
    else: primes, per, K, reps, plant = [next_prime(3000), next_prime(6000)], 36, 48, 1000, 0.3
    seed = 20261110
    res = {"lead": "LEAD-EXO-weak-class-instrument-calibration", "seed": seed, "smoke": smoke,
           "params": {"primes": primes, "curves_per_class": per, "K": K, "perm_reps": reps,
                      "plant_w": plant, "smallx_T": "p//8"}, "cells": []}
    for p in primes:
        rng = random.Random(seed + p)
        sq = [0] * p
        for v in range(1, p): sq[v * v % p] = 1
        for v in range(1, p):
            if sq[v] == 0: sq[v] = -1
        anom, ta = gen_class(p, sq, rng, "anomalous", per, K)
        rand, tr = gen_class(p, sq, rng, "random", 2 * per, K)
        ctrl, _ = gen_class(p, sq, rng, "random", per, K, plant=plant)
        rA, rB = rand[:per], rand[per:]
        c = {"p": p, "n_anom": len(anom), "n_rand": len(rand), "tries": [ta, tr],
             "mean_anom": sum(anom) / max(1, len(anom)), "mean_rand": sum(rand) / len(rand)}
        if len(anom) >= 3:
            c["auc_weak"] = auc(anom, rB); c["p_weak"] = perm_p(anom, rB, rng, reps)
        c["auc_AA"] = auc(rA, rB); c["p_AA"] = perm_p(rA, rB, rng, reps)
        c["auc_pos"] = auc(ctrl, rB); c["p_pos"] = perm_p(ctrl, rB, rng, reps)
        res["cells"].append(c)
    cs = res["cells"]
    pos_ok = all(c["auc_pos"] >= 0.9 for c in cs)
    aa_ok = all(0.25 <= c["auc_AA"] <= 0.75 for c in cs)
    have = all("auc_weak" in c for c in cs)
    blind = have and all(abs(c["auc_weak"] - 0.5) < 0.2 and c["p_weak"] > 0.01 for c in cs)
    seen = have and any(abs(c["auc_weak"] - 0.5) >= 0.3 and c["p_weak"] < 0.01 for c in cs)
    if smoke: v = "inconclusive"
    elif not (pos_ok and aa_ok and have): v = "inconclusive"
    elif seen: v = "falsified"
    elif blind: v = "consistent_with_prediction"
    else: v = "inconclusive"
    res.update(controls={"positive_ok": pos_ok, "AA_ok": aa_ok}, verdict=v,
               elapsed_s=round(time.time() - t0, 2))
    print(json.dumps(res))
    return 0

if __name__ == "__main__":
    sys.exit(main())
