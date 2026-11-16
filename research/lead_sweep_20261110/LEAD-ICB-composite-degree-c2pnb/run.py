#!/usr/bin/env python3
"""LEAD-ICB-composite-degree-c2pnb toy: subfield-base decomposition coverage, stdlib only."""
import json, random, sys, time, statistics
from math import comb, exp

def build_field(m):
    N = (1 << m) - 1
    for poly in range((1 << m) | 1, 1 << (m + 1), 2):
        e = [0] * (2 * N); x = 1; ok = True
        for i in range(N):
            if i and x == 1: ok = False; break
            e[i] = x; x <<= 1
            if x >> m: x ^= poly
        if ok and x == 1:
            for i in range(N, 2 * N): e[i] = e[i - N]
            lg = [0] * (N + 1)
            for i in range(N): lg[e[i]] = i
            return poly, e, lg
    raise RuntimeError("no primitive poly")

class Curve:
    def __init__(s, m, F, a, b):
        s.m, s.N = m, (1 << m) - 1; s.e, s.lg = F[1], F[2]; s.a, s.b = a, b
        s.hs = [-1] * (1 << m)
        for z in range(1 << m): s.hs[s.mul(z, z) ^ z] = z
    def mul(s, x, y):
        return 0 if not x or not y else s.e[s.lg[x] + s.lg[y]]
    def inv(s, x): return s.e[s.N - s.lg[x]]
    def pts_x(s, x):
        if x == 0:
            r = s.e[(s.lg[s.b] * (1 << (s.m - 1))) % s.N]  # sqrt b
            return [(0, r)]
        x2 = s.mul(x, x); c = s.mul(x2, x) ^ s.mul(s.a, x2) ^ s.b
        z = s.hs[s.mul(c, s.inv(x2))]
        if z < 0: return []
        return [(x, s.mul(x, z)), (x, s.mul(x, z ^ 1))]
    def order(s):
        n = 2
        for x in range(1, 1 << s.m):
            n += len(s.pts_x(x))
        return n
    def add(s, P, Q):
        if P is None: return Q
        if Q is None: return P
        x1, y1 = P; x2, y2 = Q
        if x1 == x2:
            if y1 != y2 or x1 == 0: return None
            lam = x1 ^ s.mul(y1, s.inv(x1)); x3 = s.mul(lam, lam) ^ lam ^ s.a
            return (x3, s.mul(x1, x1) ^ s.mul(lam ^ 1, x3))
        lam = s.mul(y1 ^ y2, s.inv(x1 ^ x2))
        x3 = s.mul(lam, lam) ^ lam ^ x1 ^ x2 ^ s.a
        return (x3, s.mul(lam, x1 ^ x3) ^ x3 ^ y1)

def coverage(C, sub_bits, k, deadline):
    N = C.N; step = N // ((1 << sub_bits) - 1)
    xs = [0] + [C.e[j * step] for j in range(1 << sub_bits) if j * step < N][:(1 << sub_bits) - 1]
    B = [p for x in xs for p in C.pts_x(x)]
    S = {None}
    for _ in range(k):
        T = set()
        for P in S:
            for Q in B:
                T.add(C.add(P, Q))
            if time.time() > deadline: return None
        S = T
    n = C.order()
    h = 1 - exp(-comb(2 ** sub_bits + k, k) / n)
    return dict(base=len(B), order=n, cover=len(S) / n, heuristic=h)

def rand_curve(m, F, rng, sub_bits, subfield):
    N = (1 << m) - 1; step = N // ((1 << sub_bits) - 1)
    subset = {0} | {F[1][j * step] for j in range(N // step)}
    while True:
        if subfield:
            a = rng.choice(sorted(subset)); b = rng.choice(sorted(subset - {0}))
        else:
            a = rng.randrange(1 << m); b = rng.randrange(1, 1 << m)
            if b in subset or a in subset: continue
        return Curve(m, F, a, b)

def main():
    smoke = "--smoke" in sys.argv
    t0 = time.time(); deadline = t0 + (8 if smoke else 520)
    cfgs = [(2, 3)] if smoke else [(2, 3), (2, 5), (2, 7), (3, 3), (3, 4), (3, 5), (4, 3)]
    rows = []; seed = 20261110
    for s, k in cfgs:
        m = s * k; F = build_field(m)
        row = dict(sub_bits=s, k=k, m=m)
        for arm, sub in (("generic_A1", False), ("generic_A2", False), ("subfield_1", True), ("subfield_2", True)):
            rng = random.Random(f"{seed}-{s}-{k}-{arm}")
            r = coverage(rand_curve(m, F, rng, s, sub), s, k, deadline)
            if r is None: row = None; break
            row[arm] = r
        if row is None: break
        rows.append(row)
    gen = [(r[a]["cover"] / r[a]["heuristic"]) for r in rows for a in ("generic_A1", "generic_A2")]
    sub = [(r[a]["cover"] / r[a]["heuristic"]) for r in rows for a in ("subfield_1", "subfield_2")]
    aa = [abs(r["generic_A1"]["cover"] / r["generic_A1"]["heuristic"] - r["generic_A2"]["cover"] / r["generic_A2"]["heuristic"]) for r in rows]
    stat = dict(median_generic_ratio=statistics.median(gen) if gen else None,
                max_subfield_ratio=max(sub) if sub else None,
                median_AA_ratio_diff=statistics.median(aa) if aa else None)
    if smoke or len(rows) < len(cfgs): verdict = "inconclusive"
    elif max(sub) > 0.5: verdict = "falsified"
    elif 0.6 <= stat["median_generic_ratio"] <= 1.3 and max(sub) <= 0.1 and stat["median_AA_ratio_diff"] <= 0.3:
        verdict = "consistent_with_prediction"
    else: verdict = "inconclusive"
    print(json.dumps(dict(lead="LEAD-ICB-composite-degree-c2pnb", smoke=smoke, seed=seed, configs=cfgs,
        statistic=stat, rows=rows, thresholds=dict(generic_ratio=[0.6, 1.3], subfield_ratio_max=0.1,
        subfield_falsify=0.5, AA_median_ratio_diff=0.3), elapsed_s=round(time.time() - t0, 2), verdict=verdict)))
main()
