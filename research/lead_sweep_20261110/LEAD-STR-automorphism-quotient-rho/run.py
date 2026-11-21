#!/usr/bin/env python3
"""Toy rho on the quotient of Z_N by an automorphism subgroup <lambda> of order k.
Stdlib only, deterministic. Steps until first repeated class (birthday statistic)."""
import argparse, json, math, random

def is_prime(n):
    if n < 2: return False
    i = 2
    while i * i <= n:
        if n % i == 0: return False
        i += 1
    return True

def find_prime(lo, mod=12):
    n = lo - lo % mod + 1
    while not is_prime(n): n += mod
    return n

def lam_of_order(N, k):
    for g in range(2, N):
        l = pow(g, (N - 1) // k, N)
        if k == 1: return 1
        if all(pow(l, k // q, N) != 1 for q in range(2, k + 1) if k % q == 0 and is_prime(q)):
            return l

def walk_steps(N, orbit_mults, r, rng, mode):
    """mode 'real': adding walk on canonical reps; 'ideal': random mapping on classes (hash)."""
    seen = {}
    if mode == 'real':
        M = [rng.randrange(1, N) for _ in range(r)]
        salt = rng.getrandbits(30)
    x = rng.randrange(N); steps = 0
    while True:
        c = min((x * m) % N for m in orbit_mults)
        if c in seen:
            if steps - seen[c] > 8: return steps          # genuine collision
            x = rng.randrange(N); continue                # fruitless short cycle: restart walk
        seen[c] = steps; steps += 1
        if mode == 'real':
            x = (c + M[((c ^ salt) * 2654435761 >> 7) % r]) % N
        else:
            x = (hash((c, salt2[0])) % N) if False else ((c * 6364136223846793005 + rng.getrandbits(62)) % N)

def stats(v):
    n = len(v); m = sum(v) / n
    sd = math.sqrt(sum((a - m) ** 2 for a in v) / (n - 1))
    return m, sd / math.sqrt(n)

def run(bits, trials, ks, r, seed):
    N = find_prime(1 << bits)
    out = {"N": N, "bits": bits, "cells": {}}
    base = {}
    for k in ks:
        mult = sorted({pow(lam_of_order(N, k), i, N) for i in range(k)}) if k > 1 else [1]
        assert len(mult) == k
        for mode in ('real', 'ideal'):
            rng = random.Random(seed * 1000003 + bits * 101 + k * 7 + (mode == 'ideal'))
            v = [walk_steps(N, mult, r, rng, mode) for _ in range(trials)]
            out["cells"][f"k{k}_{mode}"] = stats(v)
    # A/A: k=1 real with a different seed
    rng = random.Random(seed * 999983 + bits)
    out["aa_k1"] = stats([walk_steps(N, [1], r, rng, 'real') for _ in range(trials)])
    return out

def main():
    ap = argparse.ArgumentParser(); ap.add_argument("--smoke", action="store_true")
    a = ap.parse_args()
    ks = [1, 2, 4, 6]; seed = 20261110; r = 32
    sizes, trials = ([12], 20) if a.smoke else ([16, 18, 20], 400)
    res = []; falsified = False; consistent = True; ctrl_ok = True
    for b in sizes:
        o = run(b, trials, ks, r, seed); res.append(o)
        c = o["cells"]; m1, e1 = c["k1_real"]
        aa_m, aa_e = o["aa_k1"]
        if abs(m1 - aa_m) > 3 * math.hypot(e1, aa_e): ctrl_ok = False
        for k in ks[1:]:
            ideal = 1 / math.sqrt(k)
            mi, ei = c[f"k{k}_ideal"]
            if abs(mi / m1 - ideal) > 0.12 * ideal: ctrl_ok = False   # positive control
            m, e = c[f"k{k}_real"]
            R = m / m1; se = R * math.hypot(e / m, e1 / m1)
            o.setdefault("R", {})[k] = [R, se, ideal]
            if R + 1.96 * se < 0.9 * ideal: falsified = True
            if R - 1.96 * se < 0.9 * ideal or R > 1.4 * ideal: consistent = False
    if falsified and ctrl_ok: v = "falsified"
    elif consistent and ctrl_ok and not a.smoke: v = "consistent_with_prediction"
    else: v = "inconclusive"
    print(json.dumps({"lead": "LEAD-STR-automorphism-quotient-rho", "smoke": a.smoke, "seed": seed,
        "params": {"sizes_bits": sizes, "trials": trials, "ks": ks, "walk_r": r},
        "controls_ok": ctrl_ok, "results": res, "verdict": v}))
main()
