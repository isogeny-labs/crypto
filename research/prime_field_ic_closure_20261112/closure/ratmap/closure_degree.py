"""Ratmap route, part B: solving degree of the decomposition system for the power-map image base
B = {P : x(P) in mu_n}, mu_n = image of t -> t^((p-1)/n), n = 2^k, at m = 3 and 4.
Presentations (both are Groebner bases for a degree order, so the degree-D Macaulay space equals
tower_<=D + span{NF(M*S) : deg M <= D - deg S}):
  tower: u_0=x, u_i^2=u_{i+1}, u_{k-1}^2=1  -> NF basis x^j, deg = popcount(j)   (low-degree PKM-type circuit)
  plain: x^n = 1                              -> NF basis x^j, deg = j           (simple univariate locus)
Solving degree D* (unsat targets) = min D with 1 in span. Columns capped; censored values reported as '>D'.
"""
import random, json, sys, time, itertools
import flint
from sympy import isprime

def semaev(p, a, b, m, xR):
    names = tuple(f'x{i}' for i in range(1, m+1)) + ('X', 'Y')
    ctx = flint.nmod_mpoly_ctx.get(names, modulus=p)
    g = ctx.gens(); xs = g[:m]; X, Y = g[m], g[m+1]
    def S3(u, v, w):
        return (u-v)**2*w**2 - 2*((u+v)*(u*v+a)+2*b)*w + (u*v-a)**2 - 4*b*(u+v)
    xr = ctx.constant(xR)
    if m == 2: S = S3(xs[0], xs[1], xr)
    elif m == 3: S = S3(xs[0], xs[1], X).resultant(S3(xs[2], xr, X), 'X')
    elif m == 4:
        S4 = S3(xs[2], xs[3], Y).resultant(S3(xr, X, Y), 'Y')    # S4(x3,x4,xR,X)
        S = S3(xs[0], xs[1], X).resultant(S4, 'X')
    return {tuple(int(x) for x in e[:m]): int(c) for e, c in S.terms()}

def solving_degree(S, m, n, p, wt, colcap, Dmax):
    # reduce S into R = F_p[x]/(x_i^n - 1)
    def red(poly):
        out = {}
        for e, c in poly.items():
            k = tuple(ei % n for ei in e); out[k] = (out.get(k, 0) + c) % p
        return {k: c for k, c in out.items() if c}
    Sr = red(S)
    W = lambda e: sum(wt(ei) for ei in e)
    WS = max(W(e) for e in Sr)
    allmon = list(itertools.product(range(n), repeat=m))
    for D in range(WS, Dmax+1):
        cols = sorted((e for e in allmon if W(e) <= D), key=lambda e: (W(e) == 0, -W(e), e))
        if len(cols) > colcap: return f'>{D-1}', WS, len(cols)
        cidx = {e: i for i, e in enumerate(cols)}
        rows = [M for M in allmon if W(M) <= D - WS]
        mat = flint.nmod_mat(len(rows), len(cols), p)
        for r, M in enumerate(rows):
            for e, c in Sr.items():
                k = tuple((ei + mi) % n for ei, mi in zip(e, M))
                j = cidx[k]; mat[r, j] = (int(mat[r, j]) + c) % p
        R, rk = mat.rref()
        last = len(cols) - 1   # constant monomial is the last column
        for r in range(rk):
            if int(R[r, last]) and all(int(R[r, j]) == 0 for j in range(last)):
                return D, WS, len(cols)
    return f'>{Dmax}', WS, None

import numpy as np
def inverse_support(S, m, n, p, g):
    """R = F_p[x]/(x_i^n-1) ~ F_p^(n^m) by evaluation on mu_n^m (a DFT). For an unsat target S is a unit
    and its inverse is unique; return (max popcount weight, max plain degree, support fraction) of NF(S^-1)."""
    w = pow(g, (p-1)//n, p)
    pts = [pow(w, i, p) for i in range(n)]
    V = np.zeros((n,)*m, dtype=object)
    for idx in np.ndindex(*V.shape):
        v = 0
        for e, c in S.items():
            t = c
            for i, ei in zip(idx, e): t = t * pow(pts[i], ei, p) % p
            v = (v + t) % p
        if v == 0: return None
        V[idx] = pow(v, p-2, p)
    F = np.array([[pow(w, (-i*j) % n, p) for i in range(n)] for j in range(n)], dtype=object)  # coeff_j = n^-1 sum_i v_i w^{-ij}
    C = V
    for ax in range(m):
        C = np.moveaxis(np.tensordot(F, np.moveaxis(C, ax, 0), axes=(1, 0)) % p, 0, ax)
    ninv = pow(n, -m, p)
    C = (C * ninv) % p
    nz = [idx for idx in np.ndindex(*C.shape) if C[idx] != 0]
    return (max(sum(bin(i).count('1') for i in idx) for idx in nz), max(sum(idx) for idx in nz), round(len(nz)/n**m, 4))

def main():
    rnd = random.Random(int(sys.argv[1]) if len(sys.argv) > 1 else 7)
    cfgs = [(3, k, bits) for bits in (12, 15, 18) for k in (2, 3, 4, 5, 6)] + [(4, k, bits) for bits in (12, 16) for k in (2, 3, 4)]
    for m, k, bits in cfgs:
        n = 2**k
        p = next(q for q in range(rnd.randrange(2**(bits-1), 2**bits) | 1, 2**bits, 1) if (q - 1) % n == 0 and isprime(q))
        a, b = rnd.randrange(p), rnd.randrange(p)
        f = lambda x: (x**3 + a*x + b) % p
        g = next(g for g in range(2, p) if pow(g, (p-1)//2, p) != 1)
        mu = {pow(g, (p-1)//n*i, p) for i in range(n)}
        for trial in range(2):
            xR = next(x for x in (rnd.randrange(p) for _ in range(10**4)) if pow(f(x), (p-1)//2, p) == 1 and x not in mu)
            S = semaev(p, a, b, m, xR)
            inv = inverse_support(S, m, n, p, g)
            res = dict(m=m, k=k, n=n, bits=bits, p=p, trial=trial, unsat=inv is not None)
            if inv is not None:
                Wt = max(sum(bin(e).count('1') for e in mon) for mon in ({tuple(ei % n for ei in e) for e in S}))
                Wp = max(sum(ei % n for ei in e) for e in S)
                res.update(inv_wt=inv[0], inv_plain=inv[1], inv_support=inv[2], pred_tower=Wt+inv[0], pred_plain=Wp+inv[1], max_tower=m*k, max_plain=m*(n-1))
            macaulay = (m == 3 and k <= 3) or (m == 3 and k == 4 and trial == 0 and bits == 15) or (m == 4 and k <= 2) or (m == 4 and k == 3 and trial == 0 and bits == 12)
            if macaulay and inv is not None:
                for name, wt in (('tower', lambda j: bin(j).count('1')), ('plain', lambda j: j)):
                    if name == 'plain' and n**m > 600: continue
                    t0 = time.time()
                    D, WS, ncol = solving_degree(S, m, n, p, wt, 5000, 80)
                    res['mac_'+name] = dict(D=D, degS=WS, cols=ncol, sec=round(time.time()-t0, 1))
            print(json.dumps(res), flush=True)

main()
