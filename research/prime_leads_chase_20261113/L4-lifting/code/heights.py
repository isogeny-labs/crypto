"""Canonical heights on integral Weierstrass curves y^2 = x^3 + a2 x^2 + a4 x + a6 (exact ints).

hhat(P) := (1/2) lim_n h(x(2^n P)) / 4^n,   h(X/Z) = log max(|X|,|Z|) (gcd-reduced).
Returned with an error estimate |e_n - e_{n-1}| (geometric tail, e_n - hhat = O(4^-n)).
Convention: Silverman-book normalisation. rho is scale-invariant, so the factor 1/2 is irrelevant.
"""
from fractions import Fraction as Fr
from math import gcd, log
import numpy as np


def logabs(n):
    n = abs(n)
    if n == 0:
        return float('-inf')
    b = n.bit_length()
    if b < 1000:
        return log(n)
    s = b - 900
    return log(n >> s) + s * log(2)


class Curve:
    def __init__(self, a2, a4, a6):
        self.a2, self.a4, self.a6 = int(a2), int(a4), int(a6)

    def on(self, P):
        if P is None:
            return True
        x, y = P
        return y * y == x ** 3 + self.a2 * x * x + self.a4 * x + self.a6

    def add(self, P, Q):
        if P is None:
            return Q
        if Q is None:
            return P
        x1, y1 = P
        x2, y2 = Q
        if x1 == x2:
            if y1 + y2 == 0:
                return None
            lam = (3 * x1 * x1 + 2 * self.a2 * x1 + self.a4) / (2 * y1)
        else:
            lam = (y2 - y1) / (x2 - x1)
        x3 = lam * lam - self.a2 - x1 - x2
        y3 = -(lam * (x3 - x1) + y1)
        return (x3, y3)

    def neg(self, P):
        return None if P is None else (P[0], -P[1])

    def mul(self, n, P):
        R = None
        if n < 0:
            n, P = -n, self.neg(P)
        while n:
            if n & 1:
                R = self.add(R, P)
            P = self.add(P, P)
            n >>= 1
        return R

    def xdouble(self, X, Z):
        a2, a4, a6 = self.a2, self.a4, self.a6
        X2, Z2 = X * X, Z * Z
        Xn = X2 * X2 - 2 * a4 * X2 * Z2 - 8 * a6 * X * Z2 * Z + (a4 * a4 - 4 * a2 * a6) * Z2 * Z2
        Zn = 4 * Z * (X2 * X + a2 * X2 * Z + a4 * X * Z2 + a6 * Z2 * Z)
        g = gcd(Xn, Zn)
        if g > 1:
            Xn //= g
            Zn //= g
        return Xn, Zn

    def hhat(self, P, n=6):
        """Return (estimate, err_estimate, seq)."""
        if P is None:
            return 0.0, 0.0, []
        x = Fr(P[0])
        X, Z = x.numerator, x.denominator
        seq = []
        for i in range(n + 1):
            if Z == 0:  # hit torsion (2^i P = O)
                return 0.0, 0.0, seq
            h = max(logabs(X), logabs(Z))
            seq.append(0.5 * h / 4 ** i)
            if i < n:
                X, Z = self.xdouble(X, Z)
        err = abs(seq[-1] - seq[-2]) if len(seq) > 1 else float('inf')
        return seq[-1], err, seq


def gram(E, pts, n=6, cache=None):
    k = len(pts)
    h = [E.hhat(P, n) for P in pts]
    M = np.zeros((k, k))
    err = np.zeros((k, k))
    for i in range(k):
        M[i, i] = h[i][0]
        err[i, i] = h[i][1]
    for i in range(k):
        for j in range(i + 1, k):
            S = E.add(pts[i], pts[j])
            hs, es, _ = E.hhat(S, n)
            M[i, j] = M[j, i] = 0.5 * (hs - h[i][0] - h[j][0])
            err[i, j] = err[j, i] = 0.5 * (es + h[i][1] + h[j][1])
    return M, err


def svp_gram(M, scale=1e7):
    """lambda = min nonzero v^T M v over Z^k (M positive definite), LLL + Fincke-Pohst."""
    from fpylll import IntegerMatrix, LLL
    k = M.shape[0]
    L = np.linalg.cholesky(M)
    s = scale / np.sqrt(np.max(np.diag(M)))
    B = IntegerMatrix.from_matrix([[int(round(L[i, j] * s)) for j in range(k)] for i in range(k)])
    U = IntegerMatrix.identity(k)
    LLL.reduction(B, U)
    Um = np.array([[U[i, j] for j in range(k)] for i in range(k)], dtype=np.int64)
    G = Um @ M @ Um.T
    # Cholesky-type decomposition G = R^T R (upper R) for enumeration
    R = np.linalg.cholesky(G).T
    best = [min(np.diag(G)), None]
    bi = int(np.argmin(np.diag(G)))
    e = np.zeros(k, dtype=np.int64); e[bi] = 1
    best[1] = e.copy()
    x = np.zeros(k, dtype=np.int64)

    def rec(i, partial):
        # choose x_i given x_{i+1..k-1}; contribution (R[i,i] x_i + sum_{j>i} R[i,j] x_j)^2
        c = -np.dot(R[i, i + 1:], x[i + 1:]) / R[i, i]
        rad = np.sqrt(max(best[0] * (1 + 1e-12) - partial, 0)) / abs(R[i, i])
        lo, hi = int(np.ceil(c - rad)), int(np.floor(c + rad))
        for xi in range(lo, hi + 1):
            x[i] = xi
            t = (R[i, i] * xi + np.dot(R[i, i + 1:], x[i + 1:])) ** 2
            if partial + t > best[0] * (1 + 1e-12):
                continue
            if i == 0:
                if np.any(x) and partial + t < best[0]:
                    best[0] = partial + t; best[1] = x.copy()
            else:
                rec(i - 1, partial + t)
        x[i] = 0
    rec(k - 1, 0.0)
    coeff = best[1] @ Um
    lam = float(coeff @ M @ coeff)
    return lam, coeff
