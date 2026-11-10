#!/usr/bin/env python3
"""Koblitz volcano descendants toy. stdlib only. Curve y^2+xy=x^3+x^2+1 over F_{2^m}, m odd prime."""
import json, random, sys
from collections import Counter
POLY = {7: (1 << 7) | 3, 11: (1 << 11) | 5, 13: (1 << 13) | 0b11011, 15: (1 << 15) | 3, 17: (1 << 17) | 9}
T = 1  # trace of Frobenius over F_2 for a=1

def mk(m):
    f = POLY[m]; hi = 1 << m
    def mul(a, b):
        r = 0
        while b:
            if b & 1: r ^= a
            b >>= 1; a <<= 1
            if a & hi: a ^= f
        return r
    def sq(a): return mul(a, a)
    def inv(a):
        r = 1; e = (1 << m) - 2; b = a
        while e:
            if e & 1: r = mul(r, b)
            b = sq(b); e >>= 1
        return r
    def tr(a):
        s = 0; x = a
        for _ in range(m): s ^= x; x = sq(x)
        return s
    def half(c):  # z^2+z=c, m odd, Tr(c)=0
        z = 0; x = c
        for _ in range((m + 1) // 2): z ^= x; x = sq(sq(x))
        return z
    return mul, sq, inv, tr, half

def point_x(m, F, x):
    mul, sq, inv, tr, half = F
    if x == 0: return True
    c = x ^ 1 ^ sq(inv(x))
    return tr(c) == 0

def conductor(m):
    A, B = 0, 1  # tau^0 = 1
    for _ in range(m): A, B = A + B, -2 * A
    return abs(A), A, B

def order(m):
    A, B = conductor(m)[1:]
    tr_pi = T * A + 2 * B  # tau^m + conj = A*T + 2B
    return (1 << m) + 1 - tr_pi

def vals(f):
    out = {}
    for l in (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31):
        k = 0
        while f % l == 0: f //= l; k += 1
        if k: out[l] = k
    return out

def brute_count(m, F):
    n = 1
    for x in range(1 << m): n += 1 if x == 0 else (2 if point_x(m, F, x) else 0)
    return n

def sample_labels(m, F, rng, S, frob):
    mul, sq, inv, tr, half = F
    labs = []
    while len(labs) < S:
        x = rng.randrange(1, 1 << m)
        if tr(x ^ 1 ^ sq(inv(x))) != 0: continue
        if frob:
            best = x; y = x
            for _ in range(m - 1):
                y = sq(y); best = min(best, y)
            x = best
        labs.append(x)
    return labs

def coll(labs):
    return sum(c * (c - 1) // 2 for c in Counter(labs).values())

def main():
    smoke = '--smoke' in sys.argv
    ms = [7] if smoke else [11, 13, 15, 17]
    S = {7: 300, 11: 3000, 13: 8000, 15: 20000, 17: 40000}
    seeds = [1] if smoke else [101, 202, 303, 404, 505]
    vol = {str(m): {"f": conductor(m)[0], "N": order(m), "f_small_primes": vals(conductor(m)[0])}
           for m in (7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83)}
    res = {}
    ok_ratio = True; ok_aa = True; ok_count = True
    for m in ms:
        F = mk(m)
        Nb = brute_count(m, F) if m <= 13 else None
        if Nb is not None and Nb != order(m): ok_count = False
        r, aa = [], []
        for sd in seeds:
            rng = random.Random(sd * 1000 + m)
            p1 = coll(sample_labels(m, F, rng, S[m], False))
            p2 = coll(sample_labels(m, F, rng, S[m], False))
            q = coll(sample_labels(m, F, rng, S[m], True))
            r.append(q / max(p1, 1)); aa.append(p2 / max(p1, 1))
        mr, ma = sum(r) / len(r), sum(aa) / len(aa)
        res[str(m)] = {"order_formula": order(m), "order_brute": Nb, "frob_over_plain_collision_ratio": mr,
                       "aa_ratio": ma, "predicted_ratio": m, "S": S[m]}
        if not smoke:
            if not (0.75 * m <= mr <= 1.3 * m): ok_ratio = False
            if not (0.7 <= ma <= 1.4): ok_aa = False
    # descendants exist (f>1) for prime m in table
    desc = [int(k) for k, v in vol.items() if v["f"] == 1]  # m with NO descendants (Z[pi] maximal)
    if smoke: verdict = "inconclusive"
    elif ok_count and ok_ratio and ok_aa: verdict = "consistent_with_prediction"
    elif ok_count and ok_aa and not ok_ratio: verdict = "falsified"
    else: verdict = "inconclusive"
    print(json.dumps({"lead": "LEAD-STR-koblitz-volcano-descendants", "smoke": smoke, "seeds": seeds,
        "statistic": "frobenius-orbit/plain x-label collision-count ratio vs m", "per_m": res,
        "volcano_table": vol, "checks": {"order_matches_brute": ok_count, "ratio_in_band": ok_ratio,
        "aa_in_band": ok_aa, "m_with_no_descendants_f_eq_1": desc},
        "controls": {"positive": "Frobenius orbit labels on the Koblitz curve (ratio ~ m)",
                     "null_and_AA": "plain labels, independent resample (ratio ~ 1); descendants have no tau, so plain is their arm"},
        "verdict": verdict}))
main()
