#!/usr/bin/env python3
"""d_NS at m=3 (and m=2 control) for Kummer towers. stdlib + numpy. See PROTOCOL.md."""
import sys, json, time, hashlib, random, itertools
import numpy as np

def primroot(p):
    fs = []; n = p - 1; d = 2
    while d * d <= n:
        if n % d == 0:
            fs.append(d)
            while n % d == 0: n //= d
        d += 1
    if n > 1: fs.append(n)
    for h in range(2, p):
        if all(pow(h, (p - 1) // q, p) != 1 for q in fs): return h

def seed_of(p, t, idx):
    return int(hashlib.sha256(f"kummer_m3|{p}|{t}|{idx}".encode()).hexdigest()[:8], 16)

pc = lambda k: bin(k).count("1")

class Ctx:
    def __init__(s, p, t, m):
        s.p, s.t, s.m, s.K = p, t, m, 1 << t
        s.om = pow(primroot(p), (p - 1) // s.K, p)
        s.pw = np.array([pow(s.om, i, p) for i in range(s.K)], dtype=np.int64)
        K = s.K
        s.N = K ** m
        idx = np.indices((K,) * m).reshape(m, -1)          # exponents a_i of grid point
        s.A = idx                                          # z_i = om^{a_i}
        s.Z = s.pw[idx]                                    # z_i values, shape (m,N)
        s.popk = np.array([pc(k) for k in range(K)])
        s.W = np.array([[pow(s.om, (a * k) % K, p) for k in range(K)] for a in range(K)], dtype=np.int64)  # W[a,k]=om^{ak}
        s.Winv = np.array([[pow(s.om, (-a * k) % K, p) for k in range(K)] for a in range(K)], dtype=np.int64)
        s.Kinv = pow(K ** m, p - 2, p)
    def invdft(s, f):
        """f on grid (flat, N) -> coefficient array c[k1..km] with f(a)=sum c_k om^{a.k}"""
        p, K, m = s.p, s.K, s.m
        c = f.reshape((K,) * m) % p
        for ax in range(m):
            c = np.moveaxis(np.tensordot(c, s.Winv, axes=([ax], [0])), -1, ax) % p
        return (c * s.Kinv) % p
    def popdeg_of(s, f):
        c = s.invdft(f)
        pcs = s.popk
        tot = sum(np.meshgrid(*([pcs] * s.m), indexing="ij"))
        nz = c != 0
        return int(tot[nz].max()) if nz.any() else -1
    def monos(s):
        """per popcount-sum: array of flat monomial indices (k1,..,km) and their values at grid"""
        K, m = s.K, s.m
        ks = np.indices((K,) * m).reshape(m, -1)
        tot = sum(s.popk[ks[i]] for i in range(m))
        return ks, tot
    def mono_vals(s, ks, sel):
        """values mu(x)=prod om^{a_i k_i}: shape (nsel,N)"""
        p, K = s.p, s.K
        out = np.ones((len(sel), s.N), dtype=np.int64)
        for i in range(s.m):
            e = (np.outer(ks[i][sel], s.A[i]) % K)
            out = (out * s.pw[e]) % p
        return out

def S3coef(p, a, b, x1, x2):
    """S3(x1,x2,x3) as poly in x3: returns (c2,c1,c0) arrays (mod p)"""
    c2 = (x1 - x2) ** 2 % p
    c1 = (-2 * (((x1 + x2) * (x1 * x2 + a) + 2 * b))) % p
    c0 = ((x1 * x2 - a) ** 2 - 4 * b * (x1 + x2)) % p
    return c2, c1, c0

def build_funcs(C, a, b, g, xR):
    """pointwise u-coefficients of F1,F2 on the grid: dict i -> list over u-power v"""
    p = C.p
    x = (g * C.Z) % p                                   # x_i at grid
    x1, x2, x3 = x[0], x[1], x[2]
    F1 = list(S3coef(p, a, b, x1, x2))[::-1]            # c0,c1,c2 in u
    # F2 = S3(u,x3,xR): poly in u with x1->x3? S3 symmetric: coefficients in 3rd arg with first two (x3,xR)
    F2 = list(S3coef(p, a, b, x3, np.full_like(x3, xR)))[::-1]
    return [np.array(F1) % p, np.array(F2) % p]          # each (3,N)

def gate_det(p, F1, F2):
    """det of 4x4 matrix of (G1,G2 in P_{<=1}) -> P_{<=3}, per point; returns array of det mod p"""
    N = F1.shape[1]
    dets = np.zeros(N, dtype=np.int64)
    for n in range(N):
        M = sylv(F1[:, n], F2[:, n])
        dets[n] = det_mod(M, p)
    return dets

def sylv(f, g):
    """columns: u^j f (j=0,1), u^j g (j=0,1); rows coeff u^0..u^3"""
    M = [[0] * 4 for _ in range(4)]
    for j in range(2):
        for v in range(3):
            M[v + j][j] = int(f[v]); M[v + j][2 + j] = int(g[v])
    return M

def det_mod(M, p):
    M = [r[:] for r in M]; n = len(M); d = 1
    for c in range(n):
        piv = next((r for r in range(c, n) if M[r][c] % p), None)
        if piv is None: return 0
        if piv != c: M[c], M[piv] = M[piv], M[c]; d = -d
        d = d * M[c][c] % p; inv = pow(M[c][c], p - 2, p)
        for r in range(c + 1, n):
            f = M[r][c] * inv % p
            if f:
                for k in range(c, n): M[r][k] = (M[r][k] - f * M[c][k]) % p
    return d % p

def solve_mod(M, rhs, p):
    n = len(M); A = [list(map(int, M[i])) + [rhs[i]] for i in range(n)]
    for c in range(n):
        piv = next(r for r in range(c, n) if A[r][c] % p)
        A[c], A[piv] = A[piv], A[c]
        inv = pow(A[c][c], p - 2, p); A[c] = [v * inv % p for v in A[c]]
        for r in range(n):
            if r != c and A[r][c] % p:
                f = A[r][c]; A[r] = [(v - f * w) % p for v, w in zip(A[r], A[c])]
    return [A[i][n] for i in range(n)]

def unique_cert_E1(C, F):
    """pointwise Bezout with u-deg<=1 cofactors; returns (degree, popdegs) or None if singular"""
    p = C.p; N = C.N
    G = np.zeros((4, N), dtype=np.int64)
    for n in range(N):
        M = sylv(F[0][:, n], F[1][:, n])
        if det_mod(M, p) == 0: return None
        G[:, n] = solve_mod(M, [1, 0, 0, 0], p)
    degF = [max(C.popdeg_of(F[i][v]) + v for v in range(3) if F[i][v].any()) for i in range(2)]
    # G rows: g10,g11 (cofactor of F1 in u^0,u^1), g20,g21
    terms = []
    for r, (i, e) in enumerate([(0, 0), (0, 1), (1, 0), (1, 1)]):
        pd = C.popdeg_of(G[r])
        terms.append((i, e, pd, pd + e + degF[i] if pd >= 0 else -1))
    return max(t[3] for t in terms), terms, degF

# ---------------- linear algebra mod p ----------------
def in_span(rows, b, p):
    """is vector b in the F_p-span of the rows (2D int64 array)? Echelon on rows."""
    R = rows.copy() % p
    nr, nc = R.shape
    piv_rows = []; used = np.zeros(nr, dtype=bool)
    rank = 0
    cur = 0
    # work on active rows only
    active = np.arange(nr)
    for j in range(nc):
        if len(active) == 0: break
        col = R[active, j]
        nzi = np.nonzero(col)[0]
        if len(nzi) == 0: continue
        pi = active[nzi[0]]
        inv = pow(int(R[pi, j]), p - 2, p)
        R[pi] = (R[pi] * inv) % p
        others = active[active != pi]
        f = R[others, j]
        nzf = np.nonzero(f)[0]
        if len(nzf):
            oi = others[nzf]
            R[oi, j:] = (R[oi, j:] - np.outer(f[nzf], R[pi, j:])) % p
        piv_rows.append((j, pi)); rank += 1
        active = others
        # drop zero rows
        if rank % 64 == 0 and len(active):
            keep = R[active].any(axis=1); active = active[keep]
    v = b.copy() % p
    for j, pi in piv_rows:
        if v[j]:
            v = (v - v[j] * R[pi]) % p
    return (not v.any()), rank

def ns_columns(C, F, degF, c, E, ks, tot, nu):
    """rows = candidate vectors (cols of the matrix) over coordinates (w,x), w=0..E+nu-1"""
    N, p = C.N, C.p
    blocks = []
    for i in range(len(F)):
        for e in range(0, E + 1):
            smax = c - degF[i] - e
            if smax < 0: continue
            sel = np.nonzero(tot <= smax)[0]
            if len(sel) == 0: continue
            mu = C.mono_vals(ks, sel)
            blk = np.zeros((len(sel), (E + nu) * N), dtype=np.int64)
            for v in range(nu):
                if not F[i][v].any(): continue
                w = e + v
                blk[:, w * N:(w + 1) * N] = (mu * F[i][v][None, :]) % p
            blocks.append(blk)
    return np.vstack(blocks) if blocks else np.zeros((0, (E + nu) * N), dtype=np.int64)

def d_ns(C, F, E, cmin=2, cmax=None, log=None):
    """least c with 1 in span. F: list of (nu,N) arrays. returns (d or None, info list)"""
    p, N = C.p, C.N
    nu = F[0].shape[0]
    degF = [max(C.popdeg_of(F[i][v]) + v for v in range(nu) if F[i][v].any()) for i in range(len(F))]
    ks, tot = C.monos()
    b = np.zeros((E + nu) * N, dtype=np.int64); b[:N] = 1
    cmax = cmax if cmax is not None else 3 * C.t + 5 + 2
    info = []
    for c in range(cmin, cmax + 1):
        rows = ns_columns(C, F, degF, c, E, ks, tot, nu)
        ok, rank = in_span(rows, b, p) if len(rows) else (False, 0)
        info.append({"c": c, "ncols": int(rows.shape[0]), "nrows": int(rows.shape[1]), "rank": int(rank), "in_span": bool(ok)})
        if log: log(f"   c={c} cols={rows.shape[0]} rows={rows.shape[1]} rank={rank} in_span={ok}")
        if ok: return c, info, degF
    return None, info, degF

# ---------------- instances ----------------
def draw_curve(rng, p):
    while True:
        a, b = rng.randrange(p), rng.randrange(p)
        if (4 * a ** 3 + 27 * b * b) % p: return a, b

def make_instance(p, t, idx, m=3):
    seed = seed_of(p, t, idx) if m == 3 else int(hashlib.sha256(f"kummer_m2|{p}|{t}|{idx}".encode()).hexdigest()[:8], 16)
    rng = random.Random(seed); C = Ctx(p, t, m)
    tries = 0
    while True:
        tries += 1
        a, b = draw_curve(rng, p); g = rng.randrange(1, p); xR = rng.randrange(p)
        gridx = set(int(v) for v in (g * C.pw) % p)
        if xR in gridx: continue
        if m == 3:
            F = build_funcs(C, a, b, g, xR)
            if gate_det(p, F[0], F[1]).all(): break
        else:
            x = (g * C.Z) % p
            c2, c1, c0 = S3coef(p, a, b, x[0], x[1])
            f = (c2 * xR * xR + c1 * xR + c0) % p
            if f.all(): break
    return C, dict(p=p, t=t, idx=idx, seed=seed, a=a, b=b, g=g, x_R=xR, tries=tries), (F if m == 3 else f)

def planted_instance(p, t, seed_tag="plant"):
    """some grid point has a common root u0 of f1,f2"""
    seed = int(hashlib.sha256(f"kummer_m3_plant|{p}|{t}".encode()).hexdigest()[:8], 16)
    rng = random.Random(seed); C = Ctx(p, t, 3)
    allv = np.arange(p, dtype=np.int64)
    while True:
        a, b = draw_curve(rng, p); g = rng.randrange(1, p)
        n0 = rng.randrange(C.N)
        x = [(g * int(C.Z[i, n0])) % p for i in range(3)]
        c2, c1, c0 = S3coef(p, a, b, np.int64(x[0]), np.int64(x[1]))
        vals = (c2 * allv * allv + c1 * allv + c0) % p
        roots = np.nonzero(vals == 0)[0]
        if len(roots) == 0: continue
        u0 = int(roots[0])
        d2, d1, d0 = S3coef(p, a, b, np.int64(x[2]), np.int64(u0))      # S3(x3,u0,xR) in xR
        v2 = (d2 * allv * allv + d1 * allv + d0) % p
        r2 = np.nonzero(v2 == 0)[0]
        r2 = [int(r) for r in r2 if int(r) not in set(int(v) for v in (g * C.pw) % p)]
        if not r2: continue
        xR = r2[0]
        F = build_funcs(C, a, b, g, xR)
        return C, dict(p=p, t=t, seed=seed, a=a, b=b, g=g, x_R=xR, planted_point=int(n0), u0=u0), F
