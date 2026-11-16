#!/usr/bin/env python3
"""Torsion levers T1-T3 (PROTOCOL.md) on one family miniature. Exact counts.

Reuses the parent study's frozen instrument (frame_census.build: exact
full-group logs in the cyclic group Z/8r, sign-folded, indexed by Montgomery u).
A point's cofactor coset is its log mod 8 (up to sign) -- the information
[r]P gives without any discrete log.
"""
import json, random, sys, time
import numpy as np
sys.path.insert(0, __import__('os').path.dirname(__file__))
from frame_census import build

def signed(logs, n):
    L = np.array(logs, dtype=np.int64)
    return np.concatenate([L, (-L) % n])

def m2(logs, n):
    """N_c (distinct covered points per coset c, R != O) and J_c (unordered
    signed pairs, i <= j, landing in coset c; P + (-P) excluded from J_0)."""
    N = np.zeros(8, dtype=np.int64); J = np.zeros(8, dtype=np.int64)
    if not logs: return N, J
    S = signed(logs, n)
    sums = (S[:, None] + S[None, :]) % n
    iu = np.triu_indices(len(S))
    s_u = sums[iu]
    nz = s_u != 0
    J += np.bincount(s_u[nz] % 8, minlength=8)
    u = np.unique(sums[sums != 0])
    N += np.bincount(u % 8, minlength=8)
    return N, J

def m3(logs, n):
    """Exact m=3 subgroup coverage and J3 (i <= j <= k signed triples with coset sum 0)."""
    if not logs: return 0, 0
    S = signed(logs, n)
    t3 = (S[:, None, None] + S[None, :, None] + S[None, None, :]) % n
    cov = int(np.unique(t3[(t3 % 8 == 0) & (t3 != 0)]).size)
    c = S % 8
    m = len(S); i, j, k = np.meshgrid(np.arange(m), np.arange(m), np.arange(m), indexing='ij')
    keep = (i <= j) & (j <= k)
    J3 = int((((c[:, None, None] + c[None, :, None] + c[None, None, :]) % 8 == 0) & keep).sum())
    return cov, J3

def main():
    import argparse
    ap = argparse.ArgumentParser()
    ap.add_argument('--k', type=int, required=True)
    ap.add_argument('--p', type=int, required=True)
    ap.add_argument('--A', type=int, required=True)
    ap.add_argument('--n', type=int, required=True)
    ap.add_argument('--K', type=int, default=2000)
    ap.add_argument('--K3', type=int, default=300)
    ap.add_argument('--out', required=True)
    a = ap.parse_args()
    p, A, n = a.p, a.A, a.n
    t0 = time.time(); logu, r = build(p, A, n)
    A3 = A * pow(3, -1, p) % p
    rng = random.Random(20261104 + a.k)
    out = dict(k=a.k, p=p, A=A, n=n, r=r, K=a.K, K3=a.K3, seed=20261104 + a.k, cells=[])
    for t in (4, 5, 6, 7):
        Ls = [rng.randrange(p) for _ in range(a.K)]
        rows = []
        for idx, L in enumerate(Ls):
            # plain translation frame in x
            logs = sorted({int(logu[(L + y - A3) % p]) for y in range(1 << t)} - {0})
            N, J = m2(logs, n)
            # T2: u = w^2, w in [L, L + 2^t)
            ulist = {((L + w) % p) ** 2 % p for w in range(1 << t)}
            logs2 = sorted({int(logu[u]) for u in ulist} - {0})
            N2, J2 = m2(logs2, n)
            odd2 = sum(1 for kk in logs2 if kk % 2)
            row = dict(L=L, cols=len(logs), N=N.tolist(), J=J.tolist(),
                       t2_cols=len(logs2), t2_N0=int(N2[0]), t2_odd=odd2,
                       t2_coset_hist=np.bincount([min(kk % 8, (-kk) % 8) for kk in logs2], minlength=5).tolist())
            if t <= 6 and idx < a.K3:
                cov3, J3 = m3(logs, n)
                row.update(m3_cov=cov3, J3=J3)
            rows.append(row)
        out['cells'].append(dict(t=t, rows=rows))
        print(f'k={a.k} t={t} {time.time()-t0:.0f}s', file=sys.stderr, flush=True)
    json.dump(out, open(a.out, 'w'))

if __name__ == '__main__':
    main()
