"""Exact affine EC arithmetic over F_p, curve search with rational 2-torsion, fibre bases.
Independent of the crate. All arithmetic exact (Python ints); numpy only for Legendre counting."""
import random
import numpy as np
from sympy import isprime, sqrt_mod

INF = None


class Curve:
    def __init__(self, p, a, b):
        self.p, self.a, self.b = p, a % p, b % p

    def on(self, P):
        if P is INF:
            return True
        x, y = P
        return (y * y - (x * x * x + self.a * x + self.b)) % self.p == 0

    def neg(self, P):
        return INF if P is INF else (P[0], (-P[1]) % self.p)

    def add(self, P, Q):
        p = self.p
        if P is INF:
            return Q
        if Q is INF:
            return P
        x1, y1 = P
        x2, y2 = Q
        if x1 == x2:
            if (y1 + y2) % p == 0:
                return INF
            lam = (3 * x1 * x1 + self.a) * pow(2 * y1, -1, p) % p
        else:
            lam = (y2 - y1) * pow(x2 - x1, -1, p) % p
        x3 = (lam * lam - x1 - x2) % p
        return (x3, (lam * (x1 - x3) - y1) % p)

    def mul(self, k, P):
        R = INF
        if k < 0:
            k, P = -k, self.neg(P)
        while k:
            if k & 1:
                R = self.add(R, P)
            P = self.add(P, P)
            k >>= 1
        return R

    def lift(self, x):
        """Both points with abscissa x (list, possibly empty)."""
        p = self.p
        f = (x * x * x + self.a * x + self.b) % p
        if f == 0:
            return [(x, 0)]
        if pow(f, (p - 1) // 2, p) != 1:
            return []
        y = sqrt_mod(f, p)
        return [(x, y), (x, (p - y) % p)]


def _modmul(u, v, p):
    return (u * v) % p  # int64 safe for p < 2**31


def count_points(p, a, b, chunk=1 << 22):
    s = 0
    e = (p - 1) // 2
    for lo in range(0, p, chunk):
        x = np.arange(lo, min(p, lo + chunk), dtype=np.int64)
        f = (_modmul(_modmul(x, x, p), x, p) + a * x + b) % p
        # Euler criterion
        r = np.ones_like(f)
        base = f.copy()
        k = e
        while k:
            if k & 1:
                r = _modmul(r, base, p)
            base = _modmul(base, base, p)
            k >>= 1
        r = np.where(f == 0, 0, np.where(r == 1, 1, -1))
        s += int(r.sum())
    return p + 1 + s


def find_curve(bits, seed, cofactors=(2, 4)):
    """Prime p≡3 mod 4 of `bits` bits, curve with rational 2-torsion (t,0), #E = h*r, r prime."""
    rng = random.Random(seed)
    while True:
        p = rng.randrange(1 << (bits - 1), 1 << bits) | 3
        if not isprime(p):
            continue
        for _ in range(60):
            t = rng.randrange(p)
            a = rng.randrange(p)
            b = (-(t ** 3 + a * t)) % p
            if (4 * a ** 3 + 27 * b * b) % p == 0:
                continue
            n = count_points(p, a, b)
            for h in cofactors:
                if n % h == 0 and isprime(n // h) and n // h > 2:
                    E = Curve(p, a, b)
                    r = n // h
                    while True:
                        x = rng.randrange(p)
                        L = E.lift(x)
                        if not L:
                            continue
                        g = E.mul(h, L[0])
                        if g is not INF:
                            assert E.mul(r, g) is INF
                            return dict(E=E, p=p, a=a, b=b, t=t, n=n, h=h, r=r, g=g)


def in_G(E, r, P):
    return P is not INF and E.mul(r, P) is INF


def crate_image_x(E, t, x):
    """Crate formula: u=x-t, image abscissa u+3t+(3t^2+a)/u (None on kernel)."""
    p = E.p
    u = (x - t) % p
    if u == 0:
        return None
    return (u + 3 * t + (3 * t * t + E.a) * pow(u, -1, p)) % p


def fibre_points(C, xprime):
    """All P in G with crate image abscissa == xprime (solving the crate's quadratic)."""
    E, p, t, r = C['E'], C['p'], C['t'], C['r']
    lin = (3 * t - xprime) % p
    cst = (3 * t * t + E.a) % p
    disc = (lin * lin - 4 * cst) % p
    out = []
    if disc != 0 and pow(disc, (p - 1) // 2, p) != 1:
        return out
    s = sqrt_mod(disc, p) if disc else 0
    inv2 = pow(2, -1, p)
    us = {((-lin + s) * inv2) % p, ((-lin - s) * inv2) % p}
    for u in us:
        if u == 0:
            continue
        x = (u + t) % p
        for P in E.lift(x):
            if in_G(E, r, P):
                out.append(P)
    return out


def velu2(C):
    """Velu codomain for kernel (t,0): returns (E', phi) with phi(x,y)=(x+v/(x-t), y(1-v/(x-t)^2))."""
    E, p, t = C['E'], C['p'], C['t']
    v = (3 * t * t + E.a) % p
    w = t * v % p
    E2 = Curve(p, E.a - 5 * v, E.b - 7 * w)

    def phi(P):
        if P is INF:
            return INF
        x, y = P
        if (x - t) % p == 0:
            return INF
        d = pow(x - t, -1, p)
        return ((x + v * d) % p, y * (1 - v * d * d) % p)

    return E2, phi


def bsgs_log(E, g, r, M=None):
    """Return a function P -> log_g P in Z_r via baby-step table (exact)."""
    import math
    M = M or int(math.isqrt(r)) + 1
    table = {}
    Q = INF
    for j in range(M):
        table[Q] = j
        Q = E.add(Q, g)
    giant = E.neg(E.mul(M, g))

    def log(P):
        Q = P
        for i in range(r // M + 2):
            j = table.get(Q)
            if j is not None:
                k = (i * M + j) % r
                return k
            Q = E.add(Q, giant)
        raise ValueError('no log')

    return log
