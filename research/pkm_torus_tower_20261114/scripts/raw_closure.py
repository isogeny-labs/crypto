#!/usr/bin/env python3
"""Raw-presentation polynomial-calculus closure degree of the PKM chain for three tower
presentations of the factor base V (PROTOCOL.md): arm K (Kummer, y_{j+1} = y_j^2), arm S (G_m in the
Möbius coordinate, 2 y_j y_{j+1} = y_j^2 + 1), arm T (norm-one torus T2, 2 y_j y_{j+1} = y_j^2 - 1).

Independent of the L1 instruments: dense numpy Gaussian elimination mod p over the raw monomials of
total degree <= D (no tower normal form).  Tiny cells only.

Usage: raw_closure.py --arm K|S|T --p P --m M --t T [--seed 1] [--target random|planted]
                      [--dmax 8] [--out FILE.jsonl]
Each finished degree prints one JSON line {D, rank, cols, one_in_W, ...}; stops at the first D with
1 in W_D (the certified degree) or at --dmax (censored: D >= dmax+1).
"""
import argparse, itertools, json, random, sys, time
import numpy as np

# ----------------------------------------------------------------------------- field / curve helpers
def inv(a, p): return pow(a % p, p - 2, p)
def legendre(a, p):
    a %= p
    if a == 0: return 0
    return 1 if pow(a, (p - 1) // 2, p) == 1 else -1
def is_prime(n):
    if n < 2: return False
    i = 2
    while i * i <= n:
        if n % i == 0: return False
        i += 1
    return True
def curve_order(a, b, p):
    return p + 1 + sum(legendre(x * x * x + a * x + b, p) for x in range(p))

class F2:
    """F_{p^2} = F_p[s]/(s^2 - nr)."""
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
    if p % 4 == 3:
        r = pow(a, (p + 1) // 4, p)
        return r if r * r % p == a else None
    for r in range(p):
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
    c = sqrt_fp(r * inv(F.nr, p) % p, p)
    return ((x, 0), (0, c))

def decomposable(F, V, m, xR, a, b):
    pts = [lift(F, x, a, b) for x in V]
    for ms in itertools.combinations_with_replacement(range(len(V)), m):
        for signs in itertools.product((1, -1), repeat=m - 1):
            S = pts[ms[0]]
            for idx, sg in zip(ms[1:], signs):
                P = pts[idx]
                if sg < 0: P = (P[0], ((-P[1][0]) % F.p, (-P[1][1]) % F.p))
                S = ec_add(F, S, P, a)
            if S is not None and S[0] == (xR % F.p, 0):
                return True
    return False

# ----------------------------------------------------------------------------- factor bases
def base_K(p, t, rng):
    """V = g * mu_{2^t} in F_p^*; tower y_{j+1} = y_j^2, y_{t-1}^2 = 1, x = g*y_0."""
    assert (p - 1) % (1 << t) == 0
    g = rng.randrange(2, p)
    zeta = next(z for z in (pow(h, (p - 1) >> t, p) for h in range(2, p)) if pow(z, 1 << (t - 1), p) != 1)
    V = [g * pow(zeta, k, p) % p for k in range(1 << t)]
    return dict(V=V, g=g, c=1, zeta=zeta)

def base_S(p, t, rng):
    """V = {(z+1)/(z-1) : z in z0*mu_{2^t}} subset F_p; tower 2 y_j y_{j+1} = y_j^2 + 1, top 2 c y = y^2 + 1."""
    assert (p - 1) % (1 << t) == 0
    zeta = next(z for z in (pow(h, (p - 1) >> t, p) for h in range(2, p)) if pow(z, 1 << (t - 1), p) != 1)
    while True:
        z0 = rng.randrange(2, p)
        if pow(z0, 1 << (t + 1), p) != 1: break
    a_of = lambda z: (z + 1) * inv(z - 1, p) % p
    V = [a_of(z0 * pow(zeta, k, p) % p) for k in range(1 << t)]
    c = a_of(pow(z0, 1 << t, p))
    return dict(V=V, g=1, c=c, z0=z0, zeta=zeta)

def base_T(p, t, rng):
    """V = {i(z+1)/(z-1) : z in z0*mu_{2^t} subset T2} subset F_p; tower 2 y_j y_{j+1} = y_j^2 - 1."""
    assert p % 4 == 3 and (p + 1) % (1 << t) == 0
    def mul(A, B): return ((A[0] * B[0] - A[1] * B[1]) % p, (A[0] * B[1] + A[1] * B[0]) % p)
    def finv(A):
        n = inv((A[0] * A[0] + A[1] * A[1]) % p, p)
        return (A[0] * n % p, (-A[1]) * n % p)
    def fpow(A, e):
        R = (1, 0)
        while e:
            if e & 1: R = mul(R, A)
            A = mul(A, A); e >>= 1
        return R
    def z_of_a(a): return mul((a % p, 1), finv((a % p, p - 1)))
    def a_of_z(z):
        r = mul(mul((0, 1), ((z[0] + 1) % p, z[1])), finv(((z[0] - 1) % p, z[1])))
        assert r[1] == 0
        return r[0]
    while True:
        zeta = fpow(z_of_a(rng.randrange(1, p)), (p + 1) >> t)
        if fpow(zeta, 1 << (t - 1)) != (1, 0): break
    while True:
        z0 = z_of_a(rng.randrange(1, p))
        if fpow(z0, 1 << (t + 1)) != (1, 0): break
    V = []
    z = z0
    for _ in range(1 << t):
        V.append(a_of_z(z)); z = mul(z, zeta)
    c = a_of_z(fpow(z0, 1 << t))
    return dict(V=V, g=1, c=c, z0=list(z0), zeta=list(zeta))

def base_M(p, t, rng):
    """Arm M: Kummer tower in y (polynomial doubling) with the Möbius link x = (y_0+1)/(y_0-1).
    V = {(z+1)/(z-1) : z in z0*mu_{2^t}} (the same set as arm S); tower y_{j+1} = y_j^2, top y_{t-1}^2 = c
    with c = z0^(2^t) (not 1: z0 is a coset representative), link x (y_0 - 1) - (y_0 + 1) = 0."""
    assert (p - 1) % (1 << t) == 0
    zeta = next(z for z in (pow(h, (p - 1) >> t, p) for h in range(2, p)) if pow(z, 1 << (t - 1), p) != 1)
    while True:
        z0 = rng.randrange(2, p)
        if pow(z0, 1 << (t + 1), p) != 1: break
    Z = [z0 * pow(zeta, k, p) % p for k in range(1 << t)]
    V = [(z + 1) * inv(z - 1, p) % p for z in Z]
    return dict(V=V, Z=Z, g=1, c=pow(z0, 1 << t, p), z0=z0, zeta=zeta)

def base_C(p, t, rng):
    """Arm C: T2 as the circle u^2 + v^2 = 1 with polynomial doubling (u,v) -> (u^2 - v^2, 2uv) and the
    stereographic link x (1 - u_0) = v_0 (so x = i(z+1)/(z-1) with z = u + iv: the same V as arm T)."""
    b = base_T(p, t, rng)
    def mul(A, B): return ((A[0] * B[0] - A[1] * B[1]) % p, (A[0] * B[1] + A[1] * B[0]) % p)
    def fpow(A, e):
        R = (1, 0)
        while e:
            if e & 1: R = mul(R, A)
            A = mul(A, A); e >>= 1
        return R
    z0, zeta = tuple(b['z0']), tuple(b['zeta'])
    Z = []; z = z0
    for _ in range(1 << t):
        Z.append(z); z = mul(z, zeta)
    ctop = fpow(z0, 1 << t)
    # x = v/(1-u) must reproduce V
    for (u, v), a in zip(Z, b['V']):
        assert v * inv((1 - u) % p, p) % p == a
    return dict(V=b['V'], Z=Z, g=1, c=b['c'], ctop=list(ctop), z0=list(z0), zeta=list(zeta))

def tower_step(arm, p):
    """Return (f_mid(yj, yj1), f_top(y)) as coefficient maps on exponent pairs / singles."""
    if arm == 'K':
        return 'K'
    return arm

# ----------------------------------------------------------------------------- raw polynomials
class Poly:
    """Sparse polynomial over F_p in n variables: dict exponent-tuple -> coeff."""
    def __init__(s, n, p, d=None):
        s.n, s.p, s.d = n, p, (d or {})
    @staticmethod
    def const(n, p, c):
        return Poly(n, p, {(0,) * n: c % p} if c % p else {})
    @staticmethod
    def var(n, p, i, c=1):
        e = [0] * n; e[i] = 1
        return Poly(n, p, {tuple(e): c % p})
    def add(s, o, cb=1):
        d = dict(s.d)
        for k, v in o.d.items():
            d[k] = (d.get(k, 0) + cb * v) % s.p
            if d[k] == 0: del d[k]
        return Poly(s.n, s.p, d)
    def mul(s, o):
        d = {}
        for ka, va in s.d.items():
            for kb, vb in o.d.items():
                k = tuple(x + y for x, y in zip(ka, kb))
                d[k] = (d.get(k, 0) + va * vb) % s.p
        return Poly(s.n, s.p, {k: v for k, v in d.items() if v})
    def scale(s, c): return Poly(s.n, s.p, {k: v * c % s.p for k, v in s.d.items() if v * c % s.p})
    def deg(s): return max((sum(k) for k in s.d), default=-1)

def S3(x0, x1, x2, a, b, n, p):
    C = lambda c: Poly.const(n, p, c)
    d = x0.add(x1, -1); s = x0.add(x1); pr = x0.mul(x1)
    t1 = d.mul(d).mul(x2.mul(x2))
    t2 = s.mul(pr.add(C(a))).add(C(2 * b)).mul(x2).scale(-2)
    q = pr.add(C(-a))
    t3 = q.mul(q).add(s.scale(-4 * b))
    return t1.add(t2).add(t3)

def build_system(arm, p, m, t, base, a, b, xR):
    if arm in ('M', 'C'): return build_system_linked(arm, p, m, t, base, a, b, xR)
    k = max(m - 2, 0); n = m * t + k
    Y = lambda i, j: Poly.var(n, p, i * t + j)
    U = lambda j: Poly.var(n, p, m * t + j)
    C = lambda c: Poly.const(n, p, c)
    polys = []
    c = base['c']
    for i in range(m):
        for j in range(t - 1):
            yj, yj1 = Y(i, j), Y(i, j + 1)
            if arm == 'K':   polys.append(yj1.add(yj.mul(yj), -1))                       # y_{j+1} - y_j^2
            elif arm == 'S': polys.append(yj.mul(yj1).scale(2).add(yj.mul(yj), -1).add(C(-1)))  # 2 y y' - y^2 - 1
            else:            polys.append(yj.mul(yj1).scale(2).add(yj.mul(yj), -1).add(C(1)))   # 2 y y' - y^2 + 1
        yt = Y(i, t - 1)
        if arm == 'K':   polys.append(C(1).add(yt.mul(yt), -1))                           # 1 - y^2  (c = 1)
        elif arm == 'S': polys.append(yt.scale(2 * c).add(yt.mul(yt), -1).add(C(-1)))     # 2 c y - y^2 - 1
        else:            polys.append(yt.scale(2 * c).add(yt.mul(yt), -1).add(C(1)))      # 2 c y - y^2 + 1
    xs = [Y(i, 0).scale(base['g']) for i in range(m)]
    if m == 2:
        polys.append(S3(xs[0], xs[1], C(xR), a, b, n, p))
    else:
        us = [U(j) for j in range(k)]
        polys.append(S3(xs[0], xs[1], us[0], a, b, n, p))
        for q in range(1, m - 2):
            polys.append(S3(us[q - 1], xs[q + 1], us[q], a, b, n, p))
        polys.append(S3(us[m - 3], xs[m - 1], C(xR), a, b, n, p))
    return n, polys

def build_system_linked(arm, p, m, t, base, a, b, xR):
    """Arm M: per block y_0..y_{t-1}, x; arm C: per block u_0..u_{t-1}, v_0..v_{t-1}, x.  Then u_0..u_{m-3}."""
    per = (t + 1) if arm == 'M' else (2 * t + 1)
    k = max(m - 2, 0); n = m * per + k
    C = lambda c: Poly.const(n, p, c)
    X = lambda i: Poly.var(n, p, i * per + per - 1)
    U = lambda j: Poly.var(n, p, m * per + j)
    polys = []
    for i in range(m):
        if arm == 'M':
            Y = lambda j: Poly.var(n, p, i * per + j)
            for j in range(t - 1):
                polys.append(Y(j + 1).add(Y(j).mul(Y(j)), -1))                 # y_{j+1} - y_j^2
            polys.append(Y(t - 1).mul(Y(t - 1)).add(C(-base['c'])))           # y_{t-1}^2 - c
            polys.append(X(i).mul(Y(0).add(C(-1))).add(Y(0).add(C(1)), -1))   # x (y_0 - 1) - (y_0 + 1)
        else:
            Uu = lambda j: Poly.var(n, p, i * per + j)
            Vv = lambda j: Poly.var(n, p, i * per + t + j)
            polys.append(Uu(0).mul(Uu(0)).add(Vv(0).mul(Vv(0))).add(C(-1)))   # u_0^2 + v_0^2 - 1
            for j in range(t - 1):
                polys.append(Uu(j + 1).add(Uu(j).mul(Uu(j)), -1).add(Vv(j).mul(Vv(j))))      # u' - (u^2 - v^2)
                polys.append(Vv(j + 1).add(Uu(j).mul(Vv(j)).scale(2), -1))                   # v' - 2uv
            cu, cv = base['ctop']
            polys.append(Uu(t - 1).mul(Uu(t - 1)).add(Vv(t - 1).mul(Vv(t - 1)), -1).add(C(-cu)))
            polys.append(Uu(t - 1).mul(Vv(t - 1)).scale(2).add(C(-cv)))
            polys.append(X(i).mul(C(1).add(Uu(0), -1)).add(Vv(0), -1))        # x (1 - u_0) - v_0
    xs = [X(i) for i in range(m)]
    if m == 2:
        polys.append(S3(xs[0], xs[1], C(xR), a, b, n, p))
    else:
        us = [U(j) for j in range(k)]
        polys.append(S3(xs[0], xs[1], us[0], a, b, n, p))
        for q in range(1, m - 2):
            polys.append(S3(us[q - 1], xs[q + 1], us[q], a, b, n, p))
        polys.append(S3(us[m - 3], xs[m - 1], C(xR), a, b, n, p))
    return n, polys

def verify_tower_on_V(arm, p, t, base):
    if arm == 'M':
        ok = all(pow(z, 1 << t, p) == base['c'] for z in base['Z'])
        return ok and len(set(base['V'])) == (1 << t)
    if arm == 'C':
        return len(set(base['V'])) == (1 << t)
    """Every v in V must satisfy the tower axioms with y_0 = v/g (K) or y_0 = v (S, T)."""
    for v in base['V']:
        y = v * inv(base['g'], p) % p if arm == 'K' else v
        for _ in range(t - 1):
            if arm == 'K': y = y * y % p
            elif arm == 'S': y = (y * y + 1) * inv(2 * y, p) % p
            else: y = (y * y - 1) * inv(2 * y, p) % p
        if arm == 'K': ok = y * y % p == 1
        elif arm == 'S': ok = (2 * base['c'] * y - y * y - 1) % p == 0
        else: ok = (2 * base['c'] * y - y * y + 1) % p == 0
        if not ok: return False
    return len(set(base['V'])) == len(base['V']) == (1 << t)

# ----------------------------------------------------------------------------- closure
def monomials(n, D):
    out = []
    for d in range(D, -1, -1):           # degree-descending column order; constant last
        for comb in itertools.combinations_with_replacement(range(n), d):
            e = [0] * n
            for i in comb: e[i] += 1
            out.append(tuple(e))
    return out

def rref_small(M, p):
    """Full RREF of a small dense matrix (rows x cols) mod p; returns (basis rows, pivot cols)."""
    M = M.copy() % p
    r = 0; piv = []
    rows, cols = M.shape
    for c in range(cols):
        if r == rows: break
        nz = np.nonzero(M[r:, c])[0]
        if len(nz) == 0: continue
        i = r + nz[0]
        if i != r: M[[r, i]] = M[[i, r]]
        M[r] = M[r] * inv(int(M[r, c]), p) % p
        others = np.nonzero(M[:, c])[0]
        others = others[others != r]
        if len(others):
            M[others] = (M[others] - np.outer(M[others, c], M[r])) % p
        piv.append(c); r += 1
    return M[:r], piv

def matmul_mod(A, B, p):
    """Exact (A @ B) mod p via float64 BLAS; valid while inner_dim * p^2 < 2^53."""
    assert A.shape[1] * p * p < 2 ** 53, "inner dimension too large for exact float64 matmul"
    return np.fmod(A.astype(np.float64) @ B.astype(np.float64), p).astype(np.int64)

def closure_degree(n, polys, p, D, log):
    """1 in W_D ?  Returns (one_in_W, rank, cols, expansions)."""
    monos = monomials(n, D)
    col = {mo: i for i, mo in enumerate(monos)}
    deg_of_col = np.array([sum(mo) for mo in monos])
    cols = len(monos)
    gens = [P for P in polys if P.deg() <= D]
    if not gens: return False, 0, cols, 0
    def to_rows(ps):
        M = np.zeros((len(ps), cols), dtype=np.int64)
        for r, P in enumerate(ps):
            for mo, c in P.d.items(): M[r, col[mo]] = c
        return M
    B, piv = rref_small(to_rows(gens), p)
    piv = list(piv)
    expanded = {}          # pivot col -> bytes of the row when last expanded
    n_exp = 0
    const_col = cols - 1
    while True:
        if const_col in piv: return True, len(piv), cols, n_exp
        # low-degree rows: pivot column of degree <= D-1
        todo = []
        for ridx, pc in enumerate(piv):
            if deg_of_col[pc] <= D - 1:
                key = B[ridx].tobytes()
                if expanded.get(pc) != key:
                    todo.append(ridx); expanded[pc] = key
        if not todo: return False, len(piv), cols, n_exp
        # shifted rows: multiply by each variable (column permutation per variable)
        new_rows = []
        for ridx in todo:
            row = B[ridx]
            nzc = np.nonzero(row)[0]
            for v in range(n):
                shifted = np.zeros(cols, dtype=np.int64)
                for c0 in nzc:
                    e = list(monos[c0]); e[v] += 1
                    shifted[col[tuple(e)]] = row[c0]
                new_rows.append(shifted)
        n_exp += len(new_rows)
        R = np.array(new_rows, dtype=np.int64)
        # reduce against the current reduced basis in chunks, then RREF the residual and merge
        CH = 256
        for s0 in range(0, R.shape[0], CH):
            Rc = R[s0:s0 + CH]
            if piv:
                Rc = (Rc - matmul_mod(Rc[:, piv], B, p)) % p
            Rc = Rc[np.any(Rc, axis=1)]
            if Rc.shape[0] == 0: continue
            Bn, pn = rref_small(Rc, p)
            if not pn: continue
            # eliminate the new pivots from the old basis
            B = (B - matmul_mod(B[:, pn], Bn, p)) % p
            B = np.vstack([B, Bn]); piv = piv + list(pn)
            log(f"    merged {len(pn)} new pivots (rank {len(piv)}/{cols})")
    # unreachable

# ----------------------------------------------------------------------------- main
def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--arm', choices=['K', 'S', 'T', 'M', 'C'], required=True)
    ap.add_argument('--p', type=int, required=True)
    ap.add_argument('--m', type=int, required=True)
    ap.add_argument('--t', type=int, required=True)
    ap.add_argument('--seed', type=int, default=1)
    ap.add_argument('--target', choices=['random', 'planted'], default='random')
    ap.add_argument('--dmax', type=int, default=8)
    ap.add_argument('--dmin', type=int, default=1)
    ap.add_argument('--curve', default=None, help='a,b,xR override (skips the consistency check)')
    ap.add_argument('--out', default=None)
    ap.add_argument('--quiet', action='store_true')
    ap.add_argument('--dump', default=None, help='write the raw system as JSON {p,n,polys} (input of closure/pc_closure)')
    ap.add_argument('--dump-only', action='store_true')
    A = ap.parse_args()
    p, m, t = A.p, A.m, A.t
    log = (lambda *a: None) if A.quiet else (lambda *a: print(*a, file=sys.stderr, flush=True))
    # curve and target are drawn from the seed only (so arms K and S share them at the same p)
    rng = random.Random(f"{p}-{m}-{t}-{A.seed}-{A.target}")
    while True:
        a, b = rng.randrange(1, p), rng.randrange(1, p)
        if (4 * a ** 3 + 27 * b * b) % p == 0: continue
        order = curve_order(a, b, p)
        if is_prime(order): break
    brng = random.Random(f"base-{A.arm}-{p}-{t}-{A.seed}")
    base = {'K': base_K, 'S': base_S, 'T': base_T, 'M': base_M, 'C': base_C}[A.arm](p, t, brng)
    assert verify_tower_on_V(A.arm, p, t, base), "tower axioms fail on V"
    F = F2(p)
    trng = random.Random(f"target-{p}-{m}-{t}-{A.seed}-{A.target}")
    if A.curve:
        a, b, xR = map(int, A.curve.split(','))
        consistent = None
    elif A.target == 'planted':
        while True:
            pts = [lift(F, trng.choice(base['V']), a, b) for _ in range(m)]
            S = None
            for P in pts:
                if trng.random() < .5: P = (P[0], ((-P[1][0]) % p, (-P[1][1]) % p))
                S = ec_add(F, S, P, a)
            if S is not None and S[0][1] == 0 and legendre(S[0][0] ** 3 + a * S[0][0] + b, p) == 1:
                xR = S[0][0]; break
        consistent = True
    else:
        while True:
            xR = trng.randrange(1, p)
            if legendre(xR ** 3 + a * xR + b, p) != 1: continue
            if not decomposable(F, base['V'], m, xR, a, b): break
        consistent = False
    n, polys = build_system(A.arm, p, m, t, base, a, b, xR)
    hdr = dict(arm=A.arm, p=p, m=m, t=t, N=m * t, n_vars=n, seed=A.seed, target=A.target, a=a, b=b,
               order=order, xR=xR, consistent_by_enum=consistent, c=base['c'], g=base['g'],
               n_polys=len(polys), max_gen_deg=max(P.deg() for P in polys))
    log(json.dumps(hdr))
    outf = open(A.out, 'a') if A.out else None
    def emit(rec):
        s = json.dumps(rec); print(s, flush=True)
        if outf: outf.write(s + '\n'); outf.flush()
    emit({'header': hdr})
    if A.dump:
        json.dump({'p': p, 'n': n, 'header': hdr,
                   'polys': [[[list(k), v] for k, v in sorted(P.d.items())] for P in polys]}, open(A.dump, 'w'))
        if A.dump_only: return
    for D in range(A.dmin, A.dmax + 1):
        t0 = time.time()
        one, rank, cols, nexp = closure_degree(n, polys, p, D, log)
        emit(dict(D=D, rank=rank, cols=cols, one_in_W=one, expansions=nexp, seconds=round(time.time() - t0, 2)))
        if one:
            emit({'certified_D': D, 'arm': A.arm, 'm': m, 't': t, 'seed': A.seed, 'target': A.target})
            break
    else:
        emit({'censored_D_ge': A.dmax + 1, 'arm': A.arm, 'm': m, 't': t, 'seed': A.seed, 'target': A.target})

if __name__ == '__main__':
    main()
