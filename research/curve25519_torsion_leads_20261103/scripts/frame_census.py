#!/usr/bin/env python3
"""Exact m=2 subgroup coverage of Mobius-frame factor bases on a
curve25519-family miniature (algebraic track: no cofactor projection).

Curve: short Weierstrass model of Montgomery v^2 = u^3 + A u^2 + u,
x = u + A/3 (the engine's model). A frame F = (a,b,c,d) defines the base
  FB_F(t) = { P : x(P) = (a y + b)/(c y + d), y in [0, 2^t) }.
  bits:lo=L    is (1, L, 0, 1)
  edwards-bits is x = (1+y)/(1-y) + A/3, i.e. u = (1+y)/(1-y)
Coverage = |{R != O in E[r] : R = +-P_i +-P_j}| / r, P_i, P_j in FB (i = j allowed),
the engine's `coverage_exact` (covered points / r).

E(F_p) is cyclic of order 8r, so every point has a full log in Z/8r; a sum
lies in E[r] iff its log is = 0 mod 8. Full log table: one pass over <g>.
"""
import json, random, sys, time
import numpy as np

def sqrt_mod(a, p):
    a %= p
    if a == 0: return 0
    if pow(a, (p - 1) // 2, p) != 1: return None
    if p % 8 == 5:
        x = pow(a, (p + 3) // 8, p)
        if x * x % p != a: x = x * pow(2, (p - 1) // 4, p) % p
        return x
    raise ValueError

def build(p, A, n):
    """Returns logu: array over u in [0,p) of full log (0 = no affine point
    with this u or 2-torsion; else k in [1, n/2]... sign-folded), and g."""
    def add(P, Q):
        if P is None: return Q
        if Q is None: return P
        (u1, v1), (u2, v2) = P, Q
        if u1 == u2:
            if (v1 + v2) % p == 0: return None
            lam = (3 * u1 * u1 + 2 * A * u1 + 1) * pow(2 * v1, -1, p) % p
        else:
            lam = (v2 - v1) * pow(u2 - u1, -1, p) % p
        u3 = (lam * lam - A - u1 - u2) % p
        return (u3, (lam * (u1 - u3) - v1) % p)
    def mul(k, P):
        R = None
        while k:
            if k & 1: R = add(R, P)
            P = add(P, P); k >>= 1
        return R
    r = n // 8
    rng = random.Random(1)
    for _try in range(10000):  # generator of the cyclic group of order n = 8r
        u = rng.randrange(2, p); v = sqrt_mod(u * u % p * u + A * u * u + u, p)
        if v is None: continue
        g = (u, v)
        if mul(n // 2, g) is not None and mul(n // r, g) is not None: break
    else:
        raise SystemExit('E(F_p) is not cyclic of order 8r: curve out of scope')
    logu = np.zeros(p, dtype=np.int64)
    P = None
    for k in range(1, n // 2 + 1):
        P = add(P, g)
        if P is None: break
        logu[P[0]] = k
    return logu, r

def frame_logs(p, A3, logu, a, b, c, d, t):
    """+-folded logs of base points for x = (a y + b)/(c y + d), y in [0, 2^t)."""
    out = []
    for y in range(1 << t):
        den = (c * y + d) % p
        if den == 0: continue
        x = (a * y + b) * pow(den, -1, p) % p
        u = (x - A3) % p
        k = int(logu[u])
        if k: out.append(k)
    return sorted(set(out))

def coverage(logs, n, r):
    """(covered, cols, J). covered: exact distinct subgroup points +-a+-b.
    J: log-free selector = number of signed pairs (unordered, i = j allowed)
    whose cofactor cosets cancel. The coset of P is log(P) mod 8, the same
    information as identifying [r]P among the 8 rational 8-torsion points,
    so J needs no discrete log (cost ~1.5 log2 r group ops per base point)."""
    if not logs: return 0, 0, 0
    L = np.array(logs, dtype=np.int64)
    S = np.concatenate([L, (-L) % n])
    sums = (S[:, None] + S[None, :]) % n
    sub = (sums % 8 == 0) & (sums != 0)
    J = int((np.triu(sub)).sum())
    return int(np.unique(sums[sub]).size), len(logs), J

def main():
    import argparse
    ap = argparse.ArgumentParser()
    ap.add_argument('--p', type=int, required=True)
    ap.add_argument('--A', type=int, required=True)
    ap.add_argument('--n', type=int, required=True)
    ap.add_argument('--t', type=int, nargs='+', default=[4, 5, 6, 7])
    ap.add_argument('--K', type=int, default=2000, help='random frames per family')
    ap.add_argument('--seed', type=int, default=0)
    ap.add_argument('--out', required=True)
    a = ap.parse_args()
    p, A, n = a.p, a.A, a.n
    t0 = time.time(); logu, r = build(p, A, n)
    print(f'log table built in {time.time()-t0:.1f}s, r={r}', file=sys.stderr)
    A3 = A * pow(3, -1, p) % p
    rng = random.Random(a.seed)
    rows = []
    for t in a.t:
        fixed = {'bits_lo0': (1, 0, 0, 1),
                 # u = (1+y)/(1-y) -> x = u + A/3 = ((1+A3) + (1-A3)... ) as Mobius in y:
                 # x = ((1 - A3) * (-1) * y ... ) computed directly below
                 }
        # edwards: x = (1+y)/(1-y) + A3 = ((1 - A3) y + (1 + A3)) / (-y + 1)
        fixed['edwards_bits'] = ((1 - A3) % p, (1 + A3) % p, p - 1, 1)
        for name, F in fixed.items():
            cov, cols, J = coverage(frame_logs(p, A3, logu, *F, t), n, r)
            rows.append(dict(t=t, family=name, frame=F, cols=cols, J=J, covered=cov, cov=cov / r))
        for fam in ('translate', 'mobius'):
            for i in range(a.K):
                if fam == 'translate':
                    F = (1, rng.randrange(p), 0, 1)
                else:
                    while True:
                        F = tuple(rng.randrange(p) for _ in range(4))
                        if (F[0] * F[3] - F[1] * F[2]) % p: break
                cov, cols, J = coverage(frame_logs(p, A3, logu, *F, t), n, r)
                rows.append(dict(t=t, family=fam, frame=F, cols=cols, J=J, covered=cov, cov=cov / r))
        print(f't={t} done {time.time()-t0:.0f}s', file=sys.stderr)
    json.dump(dict(p=p, A=A, n=n, r=r, K=a.K, seed=a.seed, rows=rows), open(a.out, 'w'))

if __name__ == '__main__':
    main()
