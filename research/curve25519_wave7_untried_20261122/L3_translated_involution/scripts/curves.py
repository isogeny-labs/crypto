#!/usr/bin/env python3
"""Wave-7 lead L3: curves, exact group tables and coordinate functions.

E  : Montgomery y^2 = x^3 + A x^2 + x over F_p (B = 1), #E = 8r, E(F_p) cyclic Z/8r.
E' : Velu 2-isogenous curve with kernel <T0>, T0 = (0,0):
       y^2 = x^3 + A x^2 - 4x - 4A = (x + A)(x - 2)(x + 2),   phi(x,y) = (x + 1/x, y - y/x^2).
     We work on E' in the shifted coordinate X = x_V + A (Y = y_V):
       Y^2 = X (X - al)(X - be),  al = A + 2, be = A - 2.
     E'(F_p) = Z/2 x Z/4r (checked), all of E'[2] rational.

Group elements are encoded as idx = k + M*eps with k in Z/M, eps in Z/2:
  E : M = 8r, eps = 0 always (generator g of order 8r);
  E': M = 4r, generator g of order 4r and h of order 2 not in <g>.
The order-r subgroup is {eps = 0, k = 0 mod m2}, m2 = M/r (8 on E, 4 on E').
The coset ("2-part") of a point is (k mod m2, eps).

Only stdlib + numpy. All arithmetic is exact (p < 2^24 so products fit int64).
"""
import random
import numpy as np

# ---------------------------------------------------------------- field helpers

def is_prime(n):
    if n < 2:
        return False
    for q in (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37):
        if n % q == 0:
            return n == q
    d, s = n - 1, 0
    while d % 2 == 0:
        d //= 2; s += 1
    for a in (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37):
        x = pow(a, d, n)
        if x in (1, n - 1):
            continue
        for _ in range(s - 1):
            x = x * x % n
            if x == n - 1:
                break
        else:
            return False
    return True


def legendre(a, p):
    a %= p
    if a == 0:
        return 0
    return 1 if pow(a, (p - 1) // 2, p) == 1 else -1


def sqrt_mod(a, p):
    """Square root mod p (Tonelli-Shanks, Atkin shortcut for p = 5 mod 8). None if non-residue."""
    a %= p
    if a == 0:
        return 0
    if pow(a, (p - 1) // 2, p) != 1:
        return None
    if p % 4 == 3:
        return pow(a, (p + 1) // 4, p)
    if p % 8 == 5:
        x = pow(a, (p + 3) // 8, p)
        if x * x % p != a:
            x = x * pow(2, (p - 1) // 4, p) % p
        return x
    q, s = p - 1, 0
    while q % 2 == 0:
        q //= 2; s += 1
    z = 2
    while pow(z, (p - 1) // 2, p) != p - 1:
        z += 1
    m, c, t, r = s, pow(z, q, p), pow(a, q, p), pow(a, (q + 1) // 2, p)
    while t != 1:
        i, tt = 0, t
        while tt != 1:
            tt = tt * tt % p; i += 1
        b = pow(c, 1 << (m - i - 1), p)
        m, c, t, r = i, b * b % p, t * b * b % p, r * b % p
    return r


def vpow(base, e, p):
    """Vectorised modular power on an int64 numpy array (p < 2^24)."""
    base = np.asarray(base, dtype=np.int64) % p
    res = np.ones_like(base)
    while e:
        if e & 1:
            res = res * base % p
        base = base * base % p
        e >>= 1
    return res


def vinv(a, p):
    return vpow(a, p - 2, p)


def count_points(p, a2, a4, a6):
    """#E for y^2 = x^3 + a2 x^2 + a4 x + a6 by an exact character sum."""
    x = np.arange(p, dtype=np.int64)
    x2 = x * x % p
    f = (x2 * x % p + a2 % p * x2 + a4 % p * x + a6) % p
    chi = vpow(f, (p - 1) // 2, p)
    chi = np.where(f == 0, 0, np.where(chi == 1, 1, -1))
    return int(p + 1 + chi.sum())

# ---------------------------------------------------------------- curve arithmetic

class Curve:
    """y^2 = x^3 + a2 x^2 + a4 x + a6, affine points as (x, y), None = O."""

    def __init__(self, p, a2, a4, a6):
        self.p, self.a2, self.a4, self.a6 = p, a2 % p, a4 % p, a6 % p

    def rhs(self, x):
        p = self.p
        return (x * x % p * x + self.a2 * x * x + self.a4 * x + self.a6) % p

    def on(self, P):
        return P is None or (P[1] * P[1] - self.rhs(P[0])) % self.p == 0

    def neg(self, P):
        return None if P is None else (P[0], (-P[1]) % self.p)

    def add(self, P, Q):
        if P is None:
            return Q
        if Q is None:
            return P
        p = self.p
        (x1, y1), (x2, y2) = P, Q
        if x1 == x2:
            if (y1 + y2) % p == 0:
                return None
            lam = (3 * x1 * x1 + 2 * self.a2 * x1 + self.a4) * pow(2 * y1, -1, p) % p
        else:
            lam = (y2 - y1) * pow(x2 - x1, -1, p) % p
        x3 = (lam * lam - self.a2 - x1 - x2) % p
        return (x3, (lam * (x1 - x3) - y1) % p)

    def mul(self, k, P):
        R = None
        if k < 0:
            k, P = -k, self.neg(P)
        while k:
            if k & 1:
                R = self.add(R, P)
            P = self.add(P, P)
            k >>= 1
        return R

    def random_point(self, rng):
        while True:
            x = rng.randrange(self.p)
            y = sqrt_mod(self.rhs(x), self.p)
            if y is not None and y != 0:
                return (x, y if rng.random() < 0.5 else (-y) % self.p)

# ---------------------------------------------------------------- the two curves

FAMILY = {  # family rule curves (frame_search README, erratum E1 applied)
    16: dict(p=65437, A=1058, n=65672),
    18: dict(p=262133, A=78, n=263096),
    20: dict(p=1048573, A=926, n=1049624),
}


def velu_2(p, a2, a4, a6, x0):
    """Velu's formulas for a kernel point (x0, 0) of order 2 on y^2 = x^3 + a2 x^2 + a4 x + a6.
    Returns (a2', a4', a6') and v such that phi(x, y) = (x + v/(x - x0), y - v y/(x - x0)^2)."""
    v = (3 * x0 * x0 + 2 * a2 * x0 + a4) % p      # g^x_Q; g^y_Q = 0 for 2-torsion
    t = v
    w = x0 * v % p                                  # u_Q = (g^y)^2 = 0, w = u + x0 v
    return (a2 % p, (a4 - 5 * t) % p, (a6 - 4 * a2 * t - 7 * w) % p), v


class Group:
    """Exact table of a finite abelian group E(F_p) = Z/M (x Z/2), with coordinate arrays.

    X[idx], Y[idx] (int64, -1 for O at idx 0); xidx[x] = an idx with that x (or -1);
    neg(idx); coset(idx); in_sub(idx)."""

    def __init__(self, curve, n, r, noncyclic, seed=1):
        self.c, self.p, self.n, self.r = curve, curve.p, n, r
        self.noncyclic = noncyclic
        self.M = n // 2 if noncyclic else n
        self.m2 = self.M // r
        rng = random.Random(seed)
        C, M = curve, self.M
        for _ in range(100000):
            g = C.random_point(rng)
            if C.mul(M, g) is None and all(C.mul(M // q, g) is not None for q in prime_factors(M)):
                break
        else:
            raise SystemExit('no point of order M found')
        self.g = g
        h = None
        if noncyclic:
            two_tors = [P for P in self.two_torsion()]
            gM2 = C.mul(M // 2, g)
            for P in two_tors:
                if P != gM2:
                    h = P
                    break
            assert h is not None
        self.h = h
        # enumerate
        X = np.full(n, -1, dtype=np.int64)
        Y = np.full(n, -1, dtype=np.int64)
        P = None
        for k in range(M):
            if P is not None:
                X[k], Y[k] = P
            if noncyclic:
                Q = C.add(P, h)
                if Q is not None:
                    X[k + M], Y[k + M] = Q
            P = C.add(P, g)
        assert P is None, 'generator order mismatch'
        self.X, self.Y = X, Y
        xidx = np.full(self.p, -1, dtype=np.int64)
        ok = X >= 0
        xidx[X[ok]] = np.nonzero(ok)[0]
        self.xidx = xidx
        # every affine point appears exactly once
        pts = X[ok] * self.p + Y[ok]
        assert np.unique(pts).size == pts.size == n - 1, 'enumeration is not a bijection'

    def two_torsion(self):
        """Rational 2-torsion points, from the known factorisation of the cubic (checked on the curve)."""
        for P in self._two_tors:
            assert P[1] == 0 and self.c.rhs(P[0]) == 0
        return list(self._two_tors)

    # helpers on encoded elements ------------------------------------------------
    def split(self, idx):
        idx = np.asarray(idx, dtype=np.int64)
        return idx % self.M, idx // self.M

    def enc(self, k, e):
        return (np.asarray(k, dtype=np.int64) % self.M) + self.M * (np.asarray(e, dtype=np.int64) & 1)

    def neg(self, idx):
        k, e = self.split(idx)
        return self.enc(-k, e)

    def addi(self, a, b):
        ka, ea = self.split(a)
        kb, eb = self.split(b)
        return self.enc(ka + kb, ea ^ eb)

    def subi(self, a, b):
        ka, ea = self.split(a)
        kb, eb = self.split(b)
        return self.enc(ka - kb, ea ^ eb)

    def coset(self, idx):
        k, e = self.split(idx)
        return (k % self.m2) + self.m2 * e

    def in_sub(self, idx):
        k, e = self.split(idx)
        return (e == 0) & (k % self.m2 == 0)

    def point(self, idx):
        idx = int(idx)
        if idx == 0:
            return None
        return (int(self.X[idx]), int(self.Y[idx]))

    def index_of(self, P):
        if P is None:
            return 0
        i = int(self.xidx[P[0]])
        assert i >= 0
        if int(self.Y[i]) == P[1] % self.p:
            return i
        return int(self.neg(i))


def prime_factors(n):
    out, d = [], 2
    while d * d <= n:
        if n % d == 0:
            out.append(d)
            while n % d == 0:
                n //= d
        d += 1
    if n > 1:
        out.append(n)
    return out


def build(k):
    """Build E and E' for family miniature k; return dict with groups and parameters."""
    F = FAMILY[k]
    p, A, n = F['p'], F['A'], F['n']
    r = n // 8
    E = Curve(p, A, 1, 0)
    E._two_tors = None
    # E: rational 2-torsion is only T0 (A^2 - 4 non-square checked in desk_checks)
    GE = Group.__new__(Group)
    _init_group(GE, E, n, r, False, [(0, 0)])
    # E' by Velu with kernel <T0>, then shift X = x_V + A
    (b2, b4, b6), v = velu_2(p, A, 1, 0, 0)
    EV = Curve(p, b2, b4, b6)          # (x + A)(x - 2)(x + 2)
    al, be = (A + 2) % p, (A - 2) % p
    Ep = Curve(p, -(al + be), al * be, 0)   # X (X - al)(X - be)
    nE2 = count_points(p, -(al + be), al * be, 0)
    GEp = Group.__new__(Group)
    _init_group(GEp, Ep, nE2, r, True, [(0, 0), (al, 0), (be, 0)])
    return dict(k=k, p=p, A=A, n=n, r=r, E=E, GE=GE, EV=EV, Ep=Ep, GEp=GEp, al=al, be=be,
                velu=(b2, b4, b6, v))


def _init_group(G, curve, n, r, noncyclic, two_tors):
    G._two_tors = two_tors
    Group.__init__(G, curve, n, r, noncyclic)


def phi(P, p, A):
    """E -> E' (shifted coordinates X = x_V + A): Velu map with kernel <T0>."""
    if P is None:
        return None
    x, y = P
    if x == 0:
        return None
    ix = pow(x, -1, p)
    xv = (x + ix) % p
    yv = (y - y * ix % p * ix) % p
    return ((xv + A) % p, yv)

# ---------------------------------------------------------------- coordinate functions (per point)
# Each returns an int64 array over all idx: value in [0, p) or -1 (pole / O / excluded).

def coord_x(G):
    v = G.X.copy()
    v[0] = -1
    return v


def huff_params(B):
    """Huff model of E': a x (y^2 - 1) = b y (x^2 - 1), from W: v^2 = u (u + a^2)(u + b^2),
    u = X - e0 where e0 is the 2-torsion abscissa lying in 2E'(F_p) (e0 - e_j, e0 - e_k squares)."""
    p, al, be = B['p'], B['al'], B['be']
    es = [0, al, be]
    for i in range(3):
        others = [es[j] for j in range(3) if j != i]
        if all(legendre(es[i] - o, p) == 1 for o in others):
            e0 = es[i]
            ea, eb = sorted(others)
            a = sqrt_mod(e0 - ea, p)
            b = sqrt_mod(e0 - eb, p)
            return dict(e0=e0, ea=ea, eb=eb, a=a, b=b)
    raise SystemExit('no 2-torsion point of E\' lies in 2E\'(F_p): Huff model not over F_p')


def coord_huff(B, which):
    """Huff x (which='x') or y on E', values at 2-torsion by their limits; O -> excluded."""
    G, p = B['GEp'], B['p']
    H = huff_params(B)
    a, b, e0 = H['a'], H['b'], H['e0']
    X, Y = G.X, G.Y
    u = (X - e0) % p
    a2, b2 = a * a % p, b * b % p
    Yi = vinv(np.where(Y == 0, 1, Y), p)
    if which == 'x':
        val = b * ((u + a2) % p) % p * Yi % p
    else:
        val = a * ((u + b2) % p) % p * Yi % p
    val = np.where(Y == 0, -1, val)
    # 2-torsion limits: x: W0 -> inf, Wa (u=-a^2) -> 0, Wb (u=-b^2) -> inf ; y: W0, Wa -> inf, Wb -> 0
    two = np.nonzero((Y == 0) & (X >= 0))[0]
    for i in two:
        ui = int(u[i])
        if which == 'x' and ui == (-a2) % p:
            val[i] = 0
        if which == 'y' and ui == (-b2) % p:
            val[i] = 0
    val[0] = -1
    return val


def coord_ji(B, which):
    """Jacobi-intersection coordinates of E' (al s^2 + c^2 = 1, be s^2 + d^2 = 1):
       s = 2Y/(X^2 - al be), c = (X^2 - 2 al X + al be)/(X^2 - al be), d = (X^2 - 2 be X + al be)/(X^2 - al be)."""
    G, p, al, be = B['GEp'], B['p'], B['al'], B['be']
    X, Y = G.X, G.Y
    den = (X * X % p - al * be) % p
    deni = vinv(np.where(den == 0, 1, den), p)
    if which == 's':
        num = 2 * Y % p
    elif which == 'c':
        num = (X * X % p - 2 * al * X % p + al * be) % p
    else:
        num = (X * X % p - 2 * be * X % p + al * be) % p
    val = num % p * deni % p
    val = np.where((den == 0) | (X < 0), -1, val)
    val[0] = -1
    return val


class FixedFunc:
    """Lookup structure: base(V) = all idx with value in V."""

    def __init__(self, vals):
        ok = np.nonzero(vals >= 0)[0]
        o = np.argsort(vals[ok], kind='stable')
        self.idx = ok[o]
        self.sv = vals[ok][o]

    def base(self, V):
        V = np.asarray(V, dtype=np.int64)
        lo = np.searchsorted(self.sv, V, 'left')
        hi = np.searchsorted(self.sv, V, 'right')
        cnt = hi - lo
        nf = int((cnt > 0).sum())
        if cnt.sum() == 0:
            return np.zeros(0, dtype=np.int64), 0
        starts = np.repeat(lo, cnt)
        offs = np.arange(cnt.sum()) - np.repeat(np.cumsum(cnt) - cnt, cnt)
        return self.idx[starts + offs], nf

# ---------------------------------------------------------------- iota_T functions on the fly

def slope_base(G, T, V):
    """Base of h_T(P) = slope of the line through -T and P (iota_T-invariant, degree 2, poles O and T).
    For T 2-torsion this is Y/(X - x_T). Returns (idx array, n_fibres, list of (idx, lambda))."""
    C, p = G.c, G.p
    xT, yT = T
    a2, a4 = C.a2, C.a4
    out, pairs, nf = [], [], 0
    for lam in V:
        lam = int(lam)
        m = (-lam * xT - yT) % p
        sig = (lam * lam - a2 - xT) % p
        pi = (a4 - 2 * lam * m - xT * sig) % p
        disc = (sig * sig - 4 * pi) % p
        s = sqrt_mod(disc, p)
        if s is None:
            continue
        i2 = (p + 1) // 2
        xs = {(sig + s) * i2 % p, (sig - s) * i2 % p}
        got = False
        for x in xs:
            y = (lam * x + m) % p
            if (y * y - C.rhs(x)) % p:
                continue
            P = (x, y)
            i = G.index_of(P)
            out.append(i); pairs.append((i, lam)); got = True
        nf += got
    return np.array(sorted(set(out)), dtype=np.int64), nf, pairs


def mobius_values(F, t, p):
    a, b, c, d = F
    V = []
    for y in range(1 << t):
        den = (c * y + d) % p
        if den == 0:
            continue
        V.append((a * y + b) * pow(den, -1, p) % p)
    return V


def random_mobius(rng, p):
    while True:
        F = tuple(rng.randrange(p) for _ in range(4))
        if (F[0] * F[3] - F[1] * F[2]) % p:
            return F

# ---------------------------------------------------------------- the census statistic

def census(G, base, shifts=()):
    """Exact counts for a base (array of idx, O removed, deduplicated).
    covered_D    : |{R in sub \\ O : R = P_i - P_j}|
    covered_full : |{R in sub \\ O : R = +-P_i +-P_j}|  (i = j allowed; equals frame_search coverage for symmetric bases)
    covered_shift[T]: |{R in sub \\ O : R in D u (D + T) u (D - T)}| for each shift T (idx)
    J_D, J_full  : log-free selectors from the coset histogram only."""
    base = np.unique(np.asarray(base, dtype=np.int64))
    base = base[base != 0]
    M, m2 = G.M, G.m2
    if base.size == 0:
        return dict(covered_D=0, covered_full=0, J_D=0, J_full=0, n_points=0,
                    covered_shift=[0 for _ in shifts])
    K = base % M
    Ee = base // M
    dk = (K[:, None] - K[None, :]) % M
    de = Ee[:, None] ^ Ee[None, :]
    mD = (de == 0) & (dk % m2 == 0) & (dk != 0)
    setD = np.unique(dk[mD])
    sk = (K[:, None] + K[None, :]) % M
    mP = (de == 0) & (sk % m2 == 0) & (sk != 0)
    setP = np.unique(sk[mP])
    full = np.unique(np.concatenate([setD, setP, (-setP) % M]))
    cs = (K % m2) + m2 * Ee
    ncos = np.bincount(cs, minlength=2 * m2)
    JD = int((ncos * ncos).sum() - base.size)
    # coset negation: (c mod m2, e) -> (-c mod m2, e)
    negc = np.array([((-(c % m2)) % m2) + m2 * (c // m2) for c in range(2 * m2)])
    Jp = int((ncos * ncos[negc]).sum())
    sh = []
    if shifts:
        dkall, deall = dk.ravel(), de.ravel()
        for T in shifts:
            kT, eT = int(T) % M, int(T) // M
            parts = [setD]
            for sgn in (1, -1):
                kk = (dkall + sgn * kT) % M
                ok = ((deall ^ eT) == 0) & (kk % m2 == 0) & (kk != 0)
                parts.append(kk[ok])
            sh.append(int(np.unique(np.concatenate(parts)).size))
    return dict(covered_D=int(setD.size), covered_full=int(full.size), J_D=JD, J_full=JD + 2 * Jp,
                n_points=int(base.size), covered_shift=sh)
