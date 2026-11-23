"""Core library for the restricted-model (XL) index-calculus theorem checks.  [pcc:unverified]

Objects
  curve      E: y^2 = x^3 + a x + b over F_p (toy p), points by enumeration.
  semaev     S_3 explicit, S_4 = Res_u(S_3(x1,x2,u), S_3(x3,x4,u)) over Z[a,b,x1..x4] (python-flint),
             specialised to F_p and a target abscissa x_R.
  base       one-coordinate presentation (x, z_1..z_s) with generators and its point set W.
               uni   : f(x) = prod (x - r)                     (no auxiliary variable)
               tower : x^2 = t1, t1^2 = t2, ..., t_{k-1}^2 = c  (W = g*mu_{2^k}; PKM-style chain)
               graph : x*den(t) = num(t), f_T(t) = 0            (rational-map image, aux t)
  system     F_R = {S_R(x_1..x_m)} + {base generators in each block}; N = m*(1+s) variables.
  XL         Mac_D(F) = span{u*f : u monomial, deg(u f) <= D}; membership by rank over F_p (flint).
  interp     delta(h; V) = least t such that some polynomial of total degree <= t (all N variables)
             agrees with h on W^m minus V  (exact definition, evaluation-matrix rank).
"""
import itertools, math, random, time
from math import comb
import flint

# ---------------------------------------------------------------- curve
def curve_points(p, a, b):
    sq = {}
    for y in range(p):
        sq.setdefault(y * y % p, []).append(y)
    pts = []
    for x in range(p):
        for y in sq.get((x * x * x + a * x + b) % p, []):
            pts.append((x, y))
    return pts

def ec_add(P, Q, p, a):
    if P is None: return Q
    if Q is None: return P
    (x1, y1), (x2, y2) = P, Q
    if x1 == x2:
        if (y1 + y2) % p == 0: return None
        l = (3 * x1 * x1 + a) * pow(2 * y1, -1, p) % p
    else:
        l = (y2 - y1) * pow(x2 - x1, -1, p) % p
    x3 = (l * l - x1 - x2) % p
    return (x3, (l * (x1 - x3) - y1) % p)

def ec_neg(P, p):
    return None if P is None else (P[0], (-P[1]) % p)

def rand_curve(p, rng, prime_order=False, tries=2000):
    for _ in range(tries):
        a = rng.randrange(p); b = rng.randrange(1, p)
        if (4 * a ** 3 + 27 * b ** 2) % p == 0: continue
        pts = curve_points(p, a, b)
        n = len(pts) + 1
        if prime_order:
            from sympy import isprime
            if not isprime(n): continue
        return a, b, pts, n
    raise RuntimeError("no curve found")

# ---------------------------------------------------------------- Semaev
_ctx = flint.fmpz_mpoly_ctx.get(('a', 'b', 'x1', 'x2', 'x3', 'x4', 'u'), 'lex')
_A, _B, _X1, _X2, _X3, _X4, _U = _ctx.gens()
def _S3(p_, q_, r_):
    return (p_ - q_) ** 2 * r_ ** 2 - 2 * ((p_ + q_) * (p_ * q_ + _A) + 2 * _B) * r_ \
        + (p_ * q_ - _A) ** 2 - 4 * _B * (p_ + q_)
_GEN = {}
def semaev_generic(m):
    """S_{m+1} over Z[a,b,x1..x_{m+1}] as dict {(ea,eb,e1..e_{m+1}): int}."""
    if m in _GEN: return _GEN[m]
    if m == 2:
        P = _S3(_X1, _X2, _X3)
        d = {}
        for e, c in P.to_dict().items():
            d[tuple(int(t) for t in e[:5])] = int(c)
    elif m == 3:
        P = _S3(_X1, _X2, _U).resultant(_S3(_X3, _X4, _U), 'u')
        d = {}
        for e, c in P.to_dict().items():
            assert int(e[6]) == 0
            d[tuple(int(t) for t in e[:6])] = int(c)
    else:
        raise ValueError("m in {2,3} only")
    _GEN[m] = d
    return d

def semaev_special(m, p, a, b, xR):
    """S_{m+1}(x_1..x_m, xR) over F_p as dict {(e1..em): c}."""
    out = {}
    for e, c in semaev_generic(m).items():
        v = c * pow(a, e[0], p) * pow(b, e[1], p) * pow(xR, e[2 + m], p) % p
        if v:
            k = tuple(e[2:2 + m])
            out[k] = (out.get(k, 0) + v) % p
    return {k: v for k, v in out.items() if v}

def eval_poly(P, pt, p):
    s = 0
    for e, c in P.items():
        t = c
        for v, k in zip(pt, e):
            if k: t = t * pow(v, k, p) % p
        s += t
    return s % p

def tdeg(P):
    return max(sum(e) for e in P) if P else -1

# ---------------------------------------------------------------- base presentations
class Base:
    """One-coordinate presentation: nloc local variables (index 0 is x), generators (dicts over local
    exponent tuples), point set W (list of local tuples, all F_p-rational in these toys)."""
    def __init__(self, kind, p, nloc, gens, pts, info):
        self.kind, self.p, self.nloc, self.gens, self.pts, self.info = kind, p, nloc, gens, pts, info
    @property
    def xvals(self):
        return sorted({w[0] for w in self.pts})

def polmul1(A, B, p):
    C = [0] * (len(A) + len(B) - 1)
    for i, u in enumerate(A):
        for j, v in enumerate(B):
            C[i + j] = (C[i + j] + u * v) % p
    return C

def uni_base(p, roots):
    f = [1]
    for r in roots: f = polmul1(f, [(-r) % p, 1], p)
    gen = {(k,): c for k, c in enumerate(f) if c}
    return Base("uni", p, 1, [gen], [(r % p,) for r in roots], {"d": len(roots)})

def kummer_base(p, d, g):
    assert (p - 1) % d == 0
    z = None
    for h in range(2, p):
        z = pow(h, (p - 1) // d, p)
        if all(pow(z, d // q, p) != 1 for q in _prime_factors(d)): break
    roots = sorted({g * pow(z, j, p) % p for j in range(d)})
    gen = {(d,): 1, (0,): (-pow(g, d, p)) % p}
    return Base("kummer", p, 1, [gen], [(r,) for r in roots], {"d": d, "g": g})

def _prime_factors(n):
    out, q = [], 2
    while q * q <= n:
        if n % q == 0:
            out.append(q)
            while n % q == 0: n //= q
        q += 1
    if n > 1: out.append(n)
    return out

def tower_base(p, k, g):
    """W = g*mu_{2^k}; variables (x, t_1..t_{k-1}); x^2=t1, t_j^2=t_{j+1}, t_{k-1}^2 = c, c = g^{2^k}."""
    n = 2 ** k
    assert (p - 1) % n == 0
    z = None
    for h in range(2, p):
        z = pow(h, (p - 1) // n, p)
        if pow(z, n // 2, p) != 1: break
    c = pow(g, n, p)
    nloc = k
    gens = []
    def mono(idx, e):
        v = [0] * nloc; v[idx] = e; return tuple(v)
    if k == 1:
        gens.append({mono(0, 2): 1, tuple([0] * nloc): (-c) % p})
    else:
        gens.append({mono(0, 2): 1, mono(1, 1): p - 1})
        for j in range(1, k - 1):
            gens.append({mono(j, 2): 1, mono(j + 1, 1): p - 1})
        gens.append({mono(k - 1, 2): 1, tuple([0] * nloc): (-c) % p})
    pts = []
    for j in range(n):
        x = g * pow(z, j, p) % p
        pts.append(tuple([x] + [pow(x, 2 ** i, p) for i in range(1, k)]))
    return Base("tower", p, nloc, gens, pts, {"k": k, "g": g, "c": c})

def graph_base(p, num, den, troots):
    """x = num(t)/den(t), t in troots: variables (x, t); x*den(t) - num(t), f_T(t)."""
    fT = [1]
    for r in troots: fT = polmul1(fT, [(-r) % p, 1], p)
    g1 = {}
    for k, c in enumerate(den):
        if c: g1[(1, k)] = (g1.get((1, k), 0) + c) % p
    for k, c in enumerate(num):
        if c: g1[(0, k)] = (g1.get((0, k), 0) - c) % p
    g1 = {e: c for e, c in g1.items() if c}
    g2 = {(0, k): c for k, c in enumerate(fT) if c}
    pts = []
    for r in troots:
        dv = sum(c * pow(r, k, p) for k, c in enumerate(den)) % p
        nv = sum(c * pow(r, k, p) for k, c in enumerate(num)) % p
        assert dv
        pts.append((nv * pow(dv, -1, p) % p, r % p))
    return Base("graph", p, 2, [g1, g2], pts, {"dT": len(troots)})

# ---------------------------------------------------------------- m-fold system
def lift_block(P, i, nloc, N, local_to_x_only=False):
    """local dict -> global dict, block i."""
    out = {}
    for e, c in P.items():
        v = [0] * N
        for j, k in enumerate(e): v[i * nloc + j] = k
        out[tuple(v)] = c
    return out

def S_global(SR, m, nloc):
    N = m * nloc
    out = {}
    for e, c in SR.items():
        v = [0] * N
        for i, k in enumerate(e): v[i * nloc] = k
        out[tuple(v)] = c
    return out

def system(base, SR, m):
    N = m * base.nloc
    G = [S_global(SR, m, base.nloc)]
    for i in range(m):
        for g in base.gens:
            G.append(lift_block(g, i, base.nloc, N))
    return N, G

def grid(base, m):
    return [tuple(itertools.chain.from_iterable(ws)) for ws in itertools.product(base.pts, repeat=m)]

# ---------------------------------------------------------------- monomials
_MON = {}
def monomials_upto(N, D):
    key = (N, D)
    if key in _MON: return _MON[key]
    out = []
    for tot in range(D + 1):
        for c in itertools.combinations_with_replacement(range(N), tot):
            e = [0] * N
            for i in c: e[i] += 1
            out.append(tuple(e))
    _MON[key] = out
    return out

# ---------------------------------------------------------------- XL / Macaulay
def mac_matrix(G, N, D, p, extra_row=None):
    cols = monomials_upto(N, D)
    idx = {e: j for j, e in enumerate(cols)}
    rows = []
    for g in G:
        dg = tdeg(g)
        if dg > D: continue
        for u in monomials_upto(N, D - dg):
            rows.append([(idx[tuple(a + b for a, b in zip(e, u))], c) for e, c in g.items()])
    nr = len(rows) + (1 if extra_row is not None else 0)
    M = flint.nmod_mat(max(nr, 1), len(cols), p)
    for i, r in enumerate(rows):
        for j, c in r: M[i, j] = c
    if extra_row is not None:
        for e, c in extra_row.items():
            M[len(rows), idx[e]] = c % p
    return M, len(rows), len(cols)

def in_mac(G, N, D, p, target):
    if tdeg(target) > D: return False, None, None, None
    M0, nr, nc = mac_matrix(G, N, D, p)
    r0 = M0.rank() if nr else 0
    M1, _, _ = mac_matrix(G, N, D, p, extra_row=target)
    r1 = M1.rank()
    return r1 == r0, r0, nr, nc

# ---------------------------------------------------------------- interpolation degree
def eval_monomials(pts, mons, p):
    """rows = points, cols = monomials."""
    M = flint.nmod_mat(len(pts), len(mons), p)
    for i, pt in enumerate(pts):
        for j, e in enumerate(mons):
            v = 1
            for x, k in zip(pt, e):
                if k: v = v * pow(x, k, p) % p
            M[i, j] = v
    return M

def interp_degree(N, X, hvals, p, tmax):
    """least t <= tmax with h in span of degree-<=t monomials on X; None if none.
    Convention: the zero function has degree -1 (deg of the zero polynomial = -infinity)."""
    if all(v % p == 0 for v in hvals):
        return -1
    for t in range(tmax + 1):
        mons = monomials_upto(N, t)
        E = flint.nmod_mat(len(X), len(mons) + 1, p)
        for i, pt in enumerate(X):
            for j, e in enumerate(mons):
                v = 1
                for x, k in zip(pt, e):
                    if k: v = v * pow(x, k, p) % p
                E[i, j] = v
            E[i, len(mons)] = hvals[i] % p
        r_aug = E.rank()
        # rank without last column: rebuild cheaply
        E0 = flint.nmod_mat(len(X), len(mons), p)
        for i in range(len(X)):
            for j in range(len(mons)):
                E0[i, j] = E[i, j]
        if E0.rank() == r_aug:
            return t
    return None

def hilbert_function(N, pts, p, tmax):
    """H(t) = dim of span of degree-<=t monomials restricted to pts, t = 0..tmax."""
    out = []
    for t in range(tmax + 1):
        out.append(eval_monomials(pts, monomials_upto(N, t), p).rank())
    return out

def poly_series_cdf(hvec, m, tmax):
    """coefficients of (sum h_j z^j)^m, cumulative up to tmax."""
    c = [1]
    for _ in range(m):
        new = [0] * (len(c) + len(hvec) - 1)
        for i, u in enumerate(c):
            for j, v in enumerate(hvec):
                new[i + j] += u * v
        c = new
    out, s = [], 0
    for t in range(tmax + 1):
        s += c[t] if t < len(c) else 0
        out.append(s)
    return out, c

def bits_base(p, s):
    """Interval {0..2^s-1} in Boolean-bit presentation: variables (x, b_0..b_{s-1});
    x - sum 2^j b_j, b_j^2 - b_j  (complete intersection, Groebner basis for degree orders with x first)."""
    nloc = 1 + s
    def mono(idx, e):
        v = [0] * nloc; v[idx] = e; return tuple(v)
    g0 = {mono(0, 1): 1}
    for j in range(s):
        g0[mono(1 + j, 1)] = (-(2 ** j)) % p
    gens = [g0] + [{mono(1 + j, 2): 1, mono(1 + j, 1): p - 1} for j in range(s)]
    pts = [tuple([v] + [(v >> j) & 1 for j in range(s)]) for v in range(2 ** s)]
    return Base("bits", p, nloc, gens, pts, {"s": s})
