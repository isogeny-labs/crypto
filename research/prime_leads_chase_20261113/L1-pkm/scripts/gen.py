#!/usr/bin/env python3
"""PKM Kummer chain generator in tower normal form (group algebra F_p[Z/2^t] per block).

Block i: y_{i,0} in mu_{2^t}; y_{i,j} = y_{i,0}^{2^j}; a normal-form block monomial is y_{i,0}^e,
0 <= e < 2^t, whose PC degree is popcount(e) (it equals prod_{j: bit j of e} y_{i,j}).
x_i = g * y_{i,0}. Aux unknowns u_0..u_{m-3}. Chain:
  S3(x0,x1,u0)=0, S3(u_{k-1},x_{k+1},u_k)=0, ..., S3(u_{m-3},x_{m-1},xR)=0   (m>=3)
  S3(x0,x1,xR)=0                                                            (m=2)
Output: JSON with p, m, t, curve, g, zeta, xR, polys: list of [[block_exps..., u_exps...], coef].
Also exhaustive consistency check over E(F_{p^2}) (all sign choices, multisets of V).
"""
import json, sys, random, itertools

def inv(a, p): return pow(a, p - 2, p)

def legendre(a, p):
    a %= p
    if a == 0: return 0
    return 1 if pow(a, (p - 1) // 2, p) == 1 else -1

def curve_order(a, b, p):
    if p > 100000:
        import numpy as np
        x = np.arange(p, dtype=np.int64)
        r = (((x * x) % p * x) % p + a * x + b) % p
        e = (p - 1) // 2; acc = np.ones(p, dtype=np.int64); base = r.copy()
        while e:
            if e & 1: acc = (acc * base) % p
            base = (base * base) % p; e >>= 1
        leg = np.where(r == 0, 0, np.where(acc == 1, 1, -1))
        return int(p + 1 + leg.sum())
    return p + 1 + sum(legendre(x * x * x + a * x + b, p) for x in range(p))

def is_prime(n):
    if n < 2: return False
    i = 2
    while i * i <= n:
        if n % i == 0: return False
        i += 1
    return True

# --- sparse polys: dict key=(tuple block exps, tuple u exps) ---
class Ring:
    def __init__(s, p, m, t):
        s.p, s.m, s.t, s.k = p, m, t, max(m - 2, 0)
        s.mask = (1 << t) - 1
    def const(s, c):
        return {((0,) * s.m, (0,) * s.k): c % s.p} if c % s.p else {}
    def x(s, i, g):
        b = [0] * s.m; b[i] = 1
        return {(tuple(b), (0,) * s.k): g % s.p}
    def u(s, j):
        e = [0] * s.k; e[j] = 1
        return {((0,) * s.m, tuple(e)): 1}
    def add(s, A, B, cb=1):
        C = dict(A)
        for kk, v in B.items():
            C[kk] = (C.get(kk, 0) + cb * v) % s.p
            if C[kk] == 0: del C[kk]
        return C
    def mul(s, A, B):
        C = {}
        for (ba, ua), va in A.items():
            for (bb, ub), vb in B.items():
                kk = (tuple((x + y) & s.mask for x, y in zip(ba, bb)), tuple(x + y for x, y in zip(ua, ub)))
                C[kk] = (C.get(kk, 0) + va * vb) % s.p
        return {kk: v for kk, v in C.items() if v}

def S3(R, x0, x1, x2, a, b):
    d = R.add(x0, x1, -1)
    s = R.add(x0, x1)
    pr = R.mul(x0, x1)
    t1 = R.mul(R.mul(d, d), R.mul(x2, x2))
    t2 = R.mul(R.mul(s, R.add(pr, R.const(a))), R.const(1))
    t2 = R.add(t2, R.const(2 * b))
    t2 = R.mul(R.mul(t2, x2), R.const(-2))
    q = R.add(pr, R.const(-a))
    t3 = R.add(R.mul(q, q), R.mul(s, R.const(-4 * b)))
    return R.add(R.add(t1, t2), t3)

# --- F_{p^2} = F_p[s]/(s^2 - nr) and E(F_{p^2}) affine arithmetic for the consistency check ---
class F2:
    def __init__(s, p):
        s.p = p
        s.nr = next(c for c in range(2, p) if legendre(c, p) == -1)
    def mul(s, A, B):
        p = s.p
        return ((A[0] * B[0] + s.nr * A[1] * B[1]) % p, (A[0] * B[1] + A[1] * B[0]) % p)
    def add(s, A, B): return ((A[0] + B[0]) % s.p, (A[1] + B[1]) % s.p)
    def sub(s, A, B): return ((A[0] - B[0]) % s.p, (A[1] - B[1]) % s.p)
    def inv(s, A):
        p = s.p
        n = (A[0] * A[0] - s.nr * A[1] * A[1]) % p
        ni = inv(n, p)
        return (A[0] * ni % p, (-A[1]) * ni % p)

def sqrt_fp(a, p):
    a %= p
    if a == 0: return 0
    for r in range(p):  # p small
        if r * r % p == a: return r
    return None

def ec_add(F, P, Q, a):
    if P is None: return Q
    if Q is None: return P
    if P[0] == Q[0]:
        if F.add(P[1], Q[1]) == (0, 0): return None
        num = F.add(F.mul((3, 0), F.mul(P[0], P[0])), (a % F.p, 0))
        lam = F.mul(num, F.inv(F.mul((2, 0), P[1])))
    else:
        lam = F.mul(F.sub(Q[1], P[1]), F.inv(F.sub(Q[0], P[0])))
    x3 = F.sub(F.sub(F.mul(lam, lam), P[0]), Q[0])
    y3 = F.sub(F.mul(lam, F.sub(P[0], x3)), P[1])
    return (x3, y3)

def lift(F, x, a, b):
    p = F.p
    r = (x * x * x + a * x + b) % p
    if legendre(r, p) >= 0:
        return ((x, 0), (sqrt_fp(r, p), 0))
    c = sqrt_fp(r * inv(F.nr, p) % p, p)  # r = nr * c^2, y = c*s
    return ((x, 0), (0, c))

def decomposable(F, V, m, xR, a, b):
    pts = [lift(F, x, a, b) for x in V]
    hits = []
    for ms in itertools.combinations_with_replacement(range(len(V)), m):
        for signs in itertools.product((1, -1), repeat=m - 1):
            S = pts[ms[0]]
            for idx, sg in zip(ms[1:], signs):
                P = pts[idx]
                if sg < 0: P = (P[0], ((-P[1][0]) % F.p, (-P[1][1]) % F.p))
                S = ec_add(F, S, P, a)
            if S is not None and S[0] == (xR % F.p, 0):
                hits.append(ms)
                break
    return hits

def main():
    import argparse
    ap = argparse.ArgumentParser()
    ap.add_argument('--p', type=int, required=True)
    ap.add_argument('--m', type=int, required=True)
    ap.add_argument('--t', type=int, required=True)
    ap.add_argument('--seed', type=int, default=1)
    ap.add_argument('--target', choices=['random', 'planted'], default='random')
    ap.add_argument('--curve', default=None, help='a,b,g,xR override (for repo cross-check)')
    ap.add_argument('--nocheck', action='store_true')
    ap.add_argument('-o', required=True)
    A = ap.parse_args()
    p, m, t = A.p, A.m, A.t
    assert (p - 1) % (1 << t) == 0
    rng = random.Random(f"{p}-{m}-{t}-{A.seed}-{A.target}")
    if A.curve:
        a, b, g, xR = map(int, A.curve.split(','))
        order = None
    else:
        while True:
            a, b = rng.randrange(1, p), rng.randrange(1, p)
            if (4 * a ** 3 + 27 * b * b) % p == 0: continue
            order = curve_order(a, b, p)
            if is_prime(order): break
        g = rng.randrange(2, p)
    zeta = next(z for z in (pow(h, (p - 1) >> t, p) for h in range(2, p)) if pow(z, 1 << (t - 1), p) != 1)
    V = [g * pow(zeta, k, p) % p for k in range(1 << t)]
    F = F2(p)
    hits = None
    if not A.curve:
        if A.target == 'planted':
            # x of a sum of m random points with x in V (signs random), must lie in F_p
            while True:
                pts = [lift(F, rng.choice(V), a, b) for _ in range(m)]
                S = None
                for P in pts:
                    if rng.random() < .5: P = (P[0], ((-P[1][0]) % p, (-P[1][1]) % p))
                    S = ec_add(F, S, P, a)
                if S is not None and S[0][1] == 0 and legendre(S[0][0] ** 3 + a * S[0][0] + b, p) == 1:
                    xR = S[0][0]; break
        else:
            while True:
                xR = rng.randrange(1, p)
                if legendre(xR ** 3 + a * xR + b, p) != 1: continue
                if A.nocheck: break
                hits = decomposable(F, V, m, xR, a, b)
                if not hits: break
    R = Ring(p, m, t)
    xs = [R.x(i, g) for i in range(m)]
    if m == 2:
        polys = [S3(R, xs[0], xs[1], R.const(xR), a, b)]
    else:
        us = [R.u(j) for j in range(m - 2)]
        polys = [S3(R, xs[0], xs[1], us[0], a, b)]
        for k in range(1, m - 2):
            polys.append(S3(R, us[k - 1], xs[k + 1], us[k], a, b))
        polys.append(S3(R, us[m - 3], xs[m - 1], R.const(xR), a, b))
    out = dict(p=p, m=m, t=t, a=a, b=b, order=order, g=g, zeta=zeta, xR=xR, target=A.target, seed=A.seed,
               consistent_by_enum=(None if hits is None else bool(hits)),
               polys=[[[list(k[0]), list(k[1])], v] for P in polys for k, v in [(None, None)] if False] )
    out['polys'] = [[[list(k[0]) + list(k[1]), v] for k, v in sorted(P.items())] for P in polys]
    json.dump(out, open(A.o, 'w'))
    print(json.dumps({kk: out[kk] for kk in ['p', 'm', 't', 'a', 'b', 'order', 'g', 'xR', 'target', 'consistent_by_enum']}))

if __name__ == '__main__':
    main()
