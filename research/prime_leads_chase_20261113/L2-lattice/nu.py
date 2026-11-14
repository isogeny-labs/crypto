#!/usr/bin/env python3
"""nu_s(T): least weight of a nonzero element of (I mod q)^s in a truncated Macaulay matrix.
Desk instrument for L2-lattice (see PROTOCOL.md). Single-threaded."""
import sys, json, itertools, random, time
from fractions import Fraction as Fr
import numpy as np
import sympy as sp

q = 1000003
rng = random.Random(20261113)

def sqrt_mod(a):
    return sp.sqrt_mod(a % q, q)

def random_curve_point():
    while True:
        A, Bc = rng.randrange(q), rng.randrange(q)
        if (4*A**3 + 27*Bc**2) % q == 0: continue
        for _ in range(100):
            x = rng.randrange(q); r = sqrt_mod(x**3 + A*x + Bc)
            if r is not None and r != 0:
                return A, Bc, x, r

def to_dict(expr, gens):
    P = sp.Poly(sp.expand(expr), *gens)
    d = {}
    for mon, c in P.terms():
        c = int(c) % q
        if c: d[mon] = (d.get(mon, 0) + c) % q
    return {k: v for k, v in d.items() if v}

def pmul(a, b):
    out = {}
    for ka, va in a.items():
        for kb, vb in b.items():
            k = tuple(i + j for i, j in zip(ka, kb))
            out[k] = (out.get(k, 0) + va * vb) % q
    return {k: v for k, v in out.items() if v}

def monomials(n, D):
    res = []
    for d in range(D + 1):
        for c in itertools.combinations_with_replacement(range(n), d):
            e = [0]*n
            for i in c: e[i] += 1
            res.append(tuple(e))
    return res

def rank_last_pivot(M):
    """Row echelon mod q; columns already ordered by decreasing weight.
    Returns list of pivot columns."""
    M = M % q
    r, c = M.shape
    piv = []
    row = 0
    for col in range(c):
        if row >= r: break
        nz = np.nonzero(M[row:, col])[0]
        if len(nz) == 0: continue
        k = row + nz[0]
        if k != row: M[[row, k]] = M[[k, row]]
        inv = pow(int(M[row, col]), q - 2, q)
        M[row] = (M[row] * inv) % q
        below = np.nonzero(M[row+1:, col])[0] + row + 1
        if len(below):
            f = M[below, col].reshape(-1, 1)
            M[below] = (M[below] - f * M[row]) % q
        piv.append(col); row += 1
    return piv

def nu(gens_d, nvars, weights, s, T):
    degs = [max(sum(k) for k in g) for g in gens_d]
    prods = []
    for alpha in itertools.combinations_with_replacement(range(len(gens_d)), s):
        p = {tuple([0]*nvars): 1}
        for i in alpha: p = pmul(p, gens_d[i])
        prods.append(p)
    cols = monomials(nvars, T)
    wt = lambda m: sum(Fr(w) * e for w, e in zip(weights, m))
    cols.sort(key=lambda m: (-wt(m), -sum(m), tuple(-e for e in m)))
    idx = {m: i for i, m in enumerate(cols)}
    rows = []
    for p in prods:
        dp = max(sum(k) for k in p)
        for a in monomials(nvars, T - dp):
            v = np.zeros(len(cols), dtype=np.int64)
            for k, c in p.items():
                v[idx[tuple(i + j for i, j in zip(k, a))]] = c
            rows.append(v)
    if not rows: return None, 0, len(cols)
    M = np.array(rows, dtype=np.int64)
    piv = rank_last_pivot(M)
    if not piv: return None, len(rows), len(cols)
    return wt(cols[piv[-1]]), len(rows), len(cols)

# ---------------- systems ----------------
def S3(x1, x2, x3, A, Bc):
    return ((x1 - x2)**2 * x3**2 - 2*((x1 + x2)*(x1*x2 + A) + 2*Bc)*x3
            + (x1*x2 - A)**2 - 4*Bc*(x1 + x2))

def build(name, delta, gamma=1):
    A, Bc, xR, yR = CURVE
    d = Fr(delta)
    if name == 'NEG_S3x':
        x1, x2 = sp.symbols('x1 x2'); g = [x1, x2]
        return [to_dict(S3(x1, x2, xR, A, Bc), g)], g, [d, d], 4*d
    if name == 'NEG_S3e':
        e1, e2 = sp.symbols('e1 e2'); g = [e1, e2]
        S = (e2 - xR*e1 - A)**2 - 4*(xR**2*e2 + (A*xR + Bc)*e1 + Bc*xR)
        return [to_dict(S, g)], g, [d, 2*d], 4*d
    if name == 'POS':
        t, e1, e2 = sp.symbols('t e1 e2'); g = [t, e1, e2]
        a, b = rng.randrange(1, q), rng.randrange(1, q)
        return [to_dict(t - a*e1, g), to_dict(t - b*e1 - e2, g)], g, [Fr(1), d, 2*d], None
    if name == 'A1':
        x1, x2, y1, y2 = sp.symbols('x1 x2 y1 y2'); g = [x1, x2, y1, y2]
        c1 = y1**2 - (x1**3 + A*x1 + Bc); c2 = y2**2 - (x2**3 + A*x2 + Bc)
        col = sp.Matrix([[x1, y1, 1], [x2, y2, 1], [xR, -yR, 1]]).det()
        return [to_dict(c1, g), to_dict(c2, g), to_dict(col, g)], g, [d, d, Fr(1), Fr(1)], 4*d
    if name in ('A2', 'A3'):
        lam, x = sp.symbols('lam x')
        nu_ = yR - lam*xR
        N = sp.expand(x**3 + A*x + Bc - (lam*x + nu_)**2)
        Qt, R = sp.div(sp.Poly(N, x), sp.Poly(x - xR, x))  # x^2 - e1 x + e2 (mod remainder)
        cf = Qt.all_coeffs()  # [1, -e1, e2] symbolic in lam
        E1, E2 = -cf[1], cf[2]
        if name == 'A2':
            e1, e2 = sp.symbols('e1 e2'); g = [lam, e1, e2]
            return [to_dict(e1 - E1, g), to_dict(e2 - E2, g)], g, [Fr(1), d, 2*d], 4*d
        x1, x2 = sp.symbols('x1 x2'); g = [lam, x1, x2]
        return [to_dict(x1 + x2 - E1, g), to_dict(x1*x2 - E2, g)], g, [Fr(1), d, d], 4*d
    if name == 'A4':
        a1, c, x, y = sp.symbols('a1 c x y')
        a0 = -(xR**2 + a1*xR + c*yR)
        N = sp.expand((x**2 + a1*x + a0)**2 - c**2*(x**3 + A*x + Bc))
        Qt, R = sp.div(sp.Poly(N, x), sp.Poly(x - xR, x))
        cf = Qt.all_coeffs()  # x^3 - e1 x^2 + e2 x - e3
        E = [-cf[1], cf[2], -cf[3]]
        e = sp.symbols('e1 e2 e3'); g = [a1, c, *e]
        return [to_dict(e[i] - E[i], g) for i in range(3)], g, [Fr(1), Fr(1), d, 2*d, 3*d], 12*d
    if name == 'A5':
        a2, a1, c, x = sp.symbols('a2 a1 c x')
        a0 = -(xR*yR + a2*xR**2 + a1*xR + c*yR)
        N = sp.expand(-((a2*x**2 + a1*x + a0)**2 - (x + c)**2*(x**3 + A*x + Bc)))
        Qt, R = sp.div(sp.Poly(N, x), sp.Poly(x - xR, x))
        cf = Qt.all_coeffs()
        E = [-cf[1], cf[2], -cf[3], cf[4]]
        e = sp.symbols('e1 e2 e3 e4'); g = [a2, a1, c, *e]
        return [to_dict(e[i] - E[i], g) for i in range(4)], g, [Fr(1)]*3 + [d, 2*d, 3*d, 4*d], 32*d
    if name == 'B':
        x1, x2, x3, x4 = sp.symbols('x1 x2 x3 x4'); g = [x1, x2, x3, x4]
        z2 = rng.randrange(q)
        return [to_dict(S3(x1, x2, xR, A, Bc), g), to_dict(S3(x3, x4, z2, A, Bc), g)], g, [d]*4, 4*d
    if name == 'F':
        x1, x2, x3, u = sp.symbols('x1 x2 x3 u'); g = [x1, x2, x3, u]
        return ([to_dict(S3(x1, x2, u, A, Bc), g), to_dict(S3(u, x3, xR, A, Bc), g)], g,
                [d, d, d, Fr(gamma)], None)
    if name.startswith('RAND_'):
        base = name[5:]
        gens, g, w, h = build(base, delta)
        rg = [{k: rng.randrange(1, q) for k in gd} for gd in gens]
        # keep the 'e_k - ...' leading linear term as in the original (coefficient 1 kept random)
        return rg, g, w, h
    raise ValueError(name)

if __name__ == '__main__':
    CURVE = random_curve_point()
    name, delta, s, T = sys.argv[1], Fr(sys.argv[2]), int(sys.argv[3]), int(sys.argv[4])
    gamma = Fr(sys.argv[5]) if len(sys.argv) > 5 else Fr(1)
    gens, g, w, h = build(name, delta, gamma)
    t0 = time.time()
    v, nr, nc = nu(gens, len(g), w, s, T)
    out = dict(system=name, delta=str(delta), delta_f=float(delta), s=s, T=T, gamma=str(gamma),
               nu=None if v is None else float(v), nu_over_s=None if v is None else float(v)/s,
               h_S=None if h is None else float(h), bound=None if h is None else min(1.0, float(h)),
               rows=nr, cols=nc, secs=round(time.time() - t0, 2), curve=list(CURVE), q=q)
    print(json.dumps(out))
