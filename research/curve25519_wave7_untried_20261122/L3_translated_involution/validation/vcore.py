"""Validator's own curve/group code for wave-7 L3 (independent of the study's curves.py).

Weierstrass y^2 = x^3 + a2 x^2 + a4 x + a6 over F_p, affine tuples, None = O.
Group tables: every point gets an integer "label" = k + M*e meaning k*g + e*h, with g, h found by
this module's own deterministic search (smallest-x scan), so labels differ from the study's.
"""
import numpy as np

FAM = {16: (65437, 1058, 65672), 18: (262133, 78, 263096), 20: (1048573, 926, 1049624)}


def isprime(n):
    if n < 2:
        return False
    d = 2
    while d * d <= n:
        if n % d == 0:
            return False
        d += 1
    return True


def leg(a, p):
    a %= p
    if a == 0:
        return 0
    return 1 if pow(a, (p - 1) // 2, p) == 1 else -1


def sqrtp(a, p):
    """Tonelli-Shanks (generic). None for non-residues."""
    a %= p
    if a == 0:
        return 0
    if leg(a, p) != 1:
        return None
    q, s = p - 1, 0
    while q % 2 == 0:
        q //= 2
        s += 1
    z = 2
    while leg(z, p) != -1:
        z += 1
    c = pow(z, q, p)
    x = pow(a, (q + 1) // 2, p)
    t = pow(a, q, p)
    m = s
    while t != 1:
        i, tt = 0, t
        while tt != 1:
            tt = tt * tt % p
            i += 1
        b = pow(c, 1 << (m - i - 1), p)
        x = x * b % p
        c = b * b % p
        t = t * c % p
        m = i
    assert x * x % p == a
    return x


def npow(b, e, p):
    b = np.asarray(b, dtype=np.int64) % p
    r = np.ones_like(b)
    while e:
        if e & 1:
            r = r * b % p
        b = b * b % p
        e >>= 1
    return r


def ninv(a, p):
    return npow(a, p - 2, p)


class Curve:
    def __init__(self, p, a2, a4, a6):
        self.p, self.a2, self.a4, self.a6 = p, a2 % p, a4 % p, a6 % p

    def f(self, x):
        p = self.p
        return (((x + self.a2) * x + self.a4) * x + self.a6) % p

    def on(self, P):
        return P is None or (P[1] * P[1] - self.f(P[0])) % self.p == 0

    def neg(self, P):
        return None if P is None else (P[0], (-P[1]) % self.p)

    def add(self, P, Q):
        if P is None:
            return Q
        if Q is None:
            return P
        p = self.p
        x1, y1 = P
        x2, y2 = Q
        if x1 == x2:
            if (y1 + y2) % p == 0:
                return None
            l = (3 * x1 * x1 + 2 * self.a2 * x1 + self.a4) * pow(2 * y1, -1, p) % p
        else:
            l = (y2 - y1) * pow((x2 - x1) % p, -1, p) % p
        x3 = (l * l - self.a2 - x1 - x2) % p
        return (x3, (l * (x1 - x3) - y1) % p)

    def sub(self, P, Q):
        return self.add(P, self.neg(Q))

    def mul(self, k, P):
        if k < 0:
            k, P = -k, self.neg(P)
        R = None
        while k:
            if k & 1:
                R = self.add(R, P)
            P = self.add(P, P)
            k >>= 1
        return R

    def count(self):
        p = self.p
        x = np.arange(p, dtype=np.int64)
        v = self.f(x)
        c = npow(v, (p - 1) // 2, p)
        chi = np.where(v == 0, 0, np.where(c == 1, 1, -1))
        return int(p + 1 + chi.sum())

    def points(self):
        """All affine points by scanning x (independent of any generator)."""
        p = self.p
        out = []
        for x in range(p):
            v = self.f(x)
            if v == 0:
                out.append((x, 0))
            else:
                y = sqrtp(v, p)
                if y is not None:
                    out.append((x, y))
                    out.append((x, p - y))
        return out


def factor(n):
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


class Group:
    """E(F_p) = <g> (cyclic, M = n) or <g> x <h> (M = n/2, h 2-torsion not in <g>)."""

    def __init__(self, C, n, r, noncyclic, labels=True):
        self.C, self.p, self.n, self.r, self.nc = C, C.p, n, r, noncyclic
        M = n // 2 if noncyclic else n
        self.M, self.m2 = M, M // r
        p = C.p
        g = None
        for x in range(1, p):           # deterministic: smallest x giving a point of order M
            y = sqrtp(C.f(x), p)
            if not y:
                continue
            P = (x, y)
            if C.mul(M, P) is None and all(C.mul(M // q, P) is not None for q in factor(M)):
                g = P
                break
        assert g is not None
        self.g = g
        h = None
        if noncyclic:
            gm2 = C.mul(M // 2, g)
            xs = [x for x in range(p) if C.f(x) == 0]
            assert len(xs) == 3
            for x in sorted(xs, reverse=True):     # a different choice order from the study's
                if (x, 0) != gm2:
                    h = (x, 0)
                    break
        self.h = h
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
        assert P is None
        self.X, self.Y = X, Y
        ok = X >= 0
        pts = X[ok] * C.p + Y[ok]
        assert np.unique(pts).size == n - 1      # bijection onto the affine points
        self.lab = None
        if labels:
            lab = {}
            for i in range(1, n):
                lab[(int(X[i]), int(Y[i]))] = i
            assert len(lab) == n - 1
            self.lab = lab

    # label arithmetic
    def ke(self, L):
        L = np.asarray(L, dtype=np.int64)
        return L % self.M, L // self.M

    def mk(self, k, e):
        return np.asarray(k, dtype=np.int64) % self.M + self.M * (np.asarray(e, dtype=np.int64) & 1)

    def add(self, a, b):
        ka, ea = self.ke(a)
        kb, eb = self.ke(b)
        return self.mk(ka + kb, ea ^ eb)

    def neg(self, a):
        k, e = self.ke(a)
        return self.mk(-k, e)

    def sub(self, a, b):
        return self.add(a, self.neg(b))

    def insub(self, a):
        k, e = self.ke(a)
        return (e == 0) & (k % self.m2 == 0)

    def pt(self, L):
        L = int(L)
        return None if L == 0 else (int(self.X[L]), int(self.Y[L]))

    def label(self, P):
        return 0 if P is None else self.lab[(P[0] % self.p, P[1] % self.p)]


def build(k):
    p, A, n = FAM[k]
    r = n // 8
    E = Curve(p, A, 1, 0)
    return p, A, n, r, E


def velu2(C, x0):
    """Velu (Washington Thm 12.16, a1 = a3 = 0) for the kernel <(x0, 0)>. Returns codomain and map."""
    p = C.p
    gx = (3 * x0 * x0 + 2 * C.a2 * x0 + C.a4) % p
    v = gx                      # 2-torsion: v_Q = g^x_Q, u_Q = (g^y_Q)^2 = 0
    w = x0 * v % p
    C2 = Curve(p, C.a2, C.a4 - 5 * v, C.a6 - 4 * C.a2 * v - 7 * w)

    def phi(P):
        if P is None or P[0] == x0:
            return None
        x, y = P
        d = pow((x - x0) % p, -1, p)
        return ((x + v * d) % p, (y - v * y * d * d) % p)
    return C2, phi


def mobius(F, z, p):
    a, b, c, d = F
    den = (c * z + d) % p
    if den == 0:
        return None
    return (a * z + b) * pow(den, -1, p) % p


def fit_mobius(pairs, p):
    """Fit lam = (a x + b)/(c x + d) from >= 3 pairs with distinct x by linear algebra mod p.
    Solve a x + b - c x lam - d lam = 0 for (a,b,c,d) up to scale (null space)."""
    rows = [[x % p, 1, (-x * l) % p, (-l) % p] for x, l in pairs[:3]]
    # Gaussian elimination for the 1-dim null space of a 3x4 system
    m = [r[:] for r in rows]
    piv = []
    ri = 0
    for col in range(4):
        sel = None
        for i in range(ri, 3):
            if m[i][col] % p:
                sel = i
                break
        if sel is None:
            continue
        m[ri], m[sel] = m[sel], m[ri]
        iv = pow(m[ri][col], -1, p)
        m[ri] = [v * iv % p for v in m[ri]]
        for i in range(3):
            if i != ri and m[i][col]:
                f = m[i][col]
                m[i] = [(a - f * b) % p for a, b in zip(m[i], m[ri])]
        piv.append(col)
        ri += 1
        if ri == 3:
            break
    free = [c for c in range(4) if c not in piv]
    if len(free) != 1:
        return None
    fc = free[0]
    sol = [0] * 4
    sol[fc] = 1
    for i, col in enumerate(piv):
        sol[col] = (-m[i][fc]) % p
    a, b, c, d = sol
    if (a * d - b * c) % p == 0:
        return None
    return (a, b, c, d)


def mobius_inv(F, p):
    a, b, c, d = F
    return (d % p, (-b) % p, (-c) % p, a % p)


def rand_mob(rng, p):
    while True:
        a, b, c, d = (rng.randrange(p) for _ in range(4))
        if c and (a * d - b * c) % p:
            return (a, b, c, d)
