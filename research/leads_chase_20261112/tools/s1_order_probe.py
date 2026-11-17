#!/usr/bin/env python3
"""S1 follow-up (post hoc, labelled exploratory): does the k = 6 departure follow small target order in large
characteristic?  Over p = 40961 (2^13 | p - 1): curves with a point R of exact order n in {2,3,4,5,6,8,9,12}, t = x(R);
unit instances only; closure D = 4 at k = 6, 7 against the large-char reference (605, 829; 1 not in V_4)."""
import sys, json, numpy as np
sys.path.insert(0, '/home/user/ml-cryptanalysis/scripts'); sys.path.insert(0, 'tools')
import ic_bounds_growth_lib as G
from unit_test import resultant_unit, s3_coeffs
p = 40961
def gen(a, b, t, k):
    N = 1 << k; g = {}
    for (e1, e2), v in s3_coeffs(p, a, b, t).items():
        key = (e1 % N, e2 % N); g[key] = (g.get(key, 0) + v) % p
    return {e: v for e, v in g.items() if v}
sq = np.zeros(p, dtype=np.int64); sq[(np.arange(p, dtype=np.int64) ** 2) % p] = 1
def count(a, b):
    x = np.arange(p, dtype=np.int64); f = (x * x % p * x + a * x + b) % p
    return int(1 + np.sum(np.where(f == 0, 1, 2 * sq[f])))   # sq[0]=1 overcounts f=0 once: corrected below
def npts(a, b):
    x = np.arange(p, dtype=np.int64); f = (x * x % p * x + a * x + b) % p
    leg = np.array([pow(int(v), (p - 1) // 2, p) for v in f]) if False else None
    s = sq[f].copy(); z = (f == 0)
    return int(1 + np.sum(np.where(z, 1, 2 * s)))
def add(P, Q, a):
    if P is None: return Q
    if Q is None: return P
    (x1, y1), (x2, y2) = P, Q
    if x1 == x2 and (y1 + y2) % p == 0: return None
    l = ((3 * x1 * x1 + a) * pow(2 * y1, p - 2, p) if P == Q else (y2 - y1) * pow(x2 - x1, p - 2, p)) % p
    x3 = (l * l - x1 - x2) % p; return (x3, (l * (x1 - x3) - y1) % p)
def mul(P, n, a):
    R = None
    while n:
        if n & 1: R = add(R, P, a)
        P = add(P, P, a); n >>= 1
    return R
def order(P, a, cap=64):
    Q, n = P, 1
    while Q is not None and n <= cap: Q = add(Q, P, a); n += 1
    return n if Q is None else None
def sqrt_mod(v):
    return next((y for y in range(p) if y * y % p == v), None) if v < 50 else pow(v, (p + 1) // 4, p) if p % 4 == 3 else tonelli(v)
def tonelli(n):
    q, s = p - 1, 0
    while q % 2 == 0: q //= 2; s += 1
    z = next(z for z in range(2, p) if pow(z, (p - 1) // 2, p) == p - 1)
    m, c, t, r = s, pow(z, q, p), pow(n, q, p), pow(n, (q + 1) // 2, p)
    while t != 1:
        i, t2 = 0, t
        while t2 != 1: t2 = t2 * t2 % p; i += 1
        b = pow(c, 1 << (m - i - 1), p); m, c, t, r = i, b * b % p, t * b * b % p, r * b % p
    return r
rng = np.random.default_rng(20261112)
out = open('s1/order_probe.jsonl', 'a')
want = (2, 3, 4, 5, 6, 8, 9, 12); found = {n: 0 for n in want}; per = 2
while min(found.values()) < per:
    a, b = (int(v) for v in rng.integers(1, p, size=2))
    if (4 * a ** 3 + 27 * b ** 2) % p == 0: continue
    N = npts(a, b)
    for n in want:
        if found[n] >= per or N % n: continue
        for _ in range(20):
            x = int(rng.integers(0, p)); f = (x ** 3 + a * x + b) % p
            if f == 0 or pow(f, (p - 1) // 2, p) != 1: continue
            P = (x, tonelli(f)); R = mul(P, N // n, a)
            if R is None or order(R, a) != n: continue
            t = R[0]
            if t == 0: continue
            if not all(resultant_unit(p, a, b, t, k)['unit'] for k in (6, 7)): continue
            rec = dict(p=p, a=a, b=b, t=t, target_order=n, curve_order=N)
            for k in (6, 7):
                L = G.Layout(k, 4); B, piv = G.closure(L, gen(a, b, t, k), p)
                rec[f'dimV4_k{k}'] = len(piv); rec[f'one_k{k}'] = bool(L.idx[(0, 0)] in set(int(c) for c in piv))
            rec['departs'] = (rec['dimV4_k6'] != 605 or rec['dimV4_k7'] != 829)
            out.write(json.dumps(rec) + '\n'); out.flush(); print(json.dumps(rec), flush=True)
            found[n] += 1; break
