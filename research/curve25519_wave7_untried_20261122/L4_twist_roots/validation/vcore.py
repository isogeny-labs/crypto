#!/usr/bin/env python3
"""Validator's own primitives for the L4 check (independent of l4core/run_l4/controls/analyze_l4).

Field tables are built by Euler's criterion and Atkin's square root (p = 5 mod 8), inverses by
Fermat, all vectorised in numpy int64 (p < 2^25, so every product fits).  Point arithmetic is done
on the long-Weierstrass model y^2 = x^3 + a2 x^2 + a4 x; the twist is mapped to
Y^2 = X^3 + (A n) X^2 + n^2 X by X = n x (n a non-residue), not via a B-scaled chord.
"""
import math
import numpy as np

CURVES = {16: (65437, 1058), 20: (1048573, 926), 24: (16777141, 7842)}


def vpow(base, e, p):
    """Elementwise base^e mod p for an int64 array (p < 2^25)."""
    r = np.ones_like(base)
    b = base % p
    while e:
        if e & 1:
            r = r * b % p
        b = b * b % p
        e >>= 1
    return r


def isprime_td(n):
    if n < 2:
        return False
    if n % 2 == 0:
        return n == 2
    q = 3
    while q * q <= n:
        if n % q == 0:
            return False
        q += 2
    return True


def factor_td(n):
    f = {}
    q = 2
    while q * q <= n:
        while n % q == 0:
            f[q] = f.get(q, 0) + 1
            n //= q
        q += 1 if q == 2 else 2
    if n > 1:
        f[n] = f.get(n, 0) + 1
    return f


class Tables:
    """cls: 0 = Z (f = 0), 1 = E (f nonzero square), 2 = T (f non-square)."""

    def __init__(self, p, A):
        assert p % 8 == 5, "Atkin sqrt needs p = 5 mod 8"
        self.p, self.A = p, A
        x = np.arange(p, dtype=np.int64)
        e = vpow(x, (p - 1) // 2, p)                     # Euler's criterion
        qr = np.zeros(p, dtype=np.int8)                  # 1 square, 2 non-square, 0 zero
        qr[e == 1] = 1
        qr[e == p - 1] = 2
        assert int((qr == 0).sum()) == 1
        self.qr = qr
        # Atkin: b = (2a)^((p-5)/8), i = 2 a b^2, s = a b (i - 1)
        b = vpow(2 * x % p, (p - 5) // 8, p)
        i = 2 * x % p * (b * b % p) % p
        s = x * b % p * ((i - 1) % p) % p
        ok = (s * s % p) == x
        assert bool(np.all(ok[qr == 1])) and bool(ok[0])
        s[qr == 2] = 0
        self.sqrt = s
        del b, i, e
        self.inv = vpow(x, p - 2, p)                     # inv[0] = 0
        f = x * ((x * x % p + A * x + 1) % p) % p
        self.f = f
        cls = qr[f].copy()                               # same coding: 0 Z, 1 E, 2 T
        self.cls = cls
        self.Ex = np.flatnonzero(cls == 1).astype(np.int64)
        self.Tx = np.flatnonzero(cls == 2).astype(np.int64)
        self.Zx = np.flatnonzero(cls == 0).astype(np.int64)


# weight of a root of each class in the base-3 root code (nZ*1 + nT*3 + nE*9)
WROOT = np.array([1, 9, 3], dtype=np.int64)  # index by cls: Z -> 1, E -> 9, T -> 3


def s3_coeffs(x1, x2, A, p, variant="std"):
    m = x1 * x2 % p
    B2 = ((x1 + x2) * ((m + 1) % p) + 2 * A * m) % p
    u = (x1 - x2) % p
    w = (m - 1) % p
    if variant == "std":
        a, c = u * u % p, w * w % p
    else:
        a, c = w * w % p, u * u % p
    return a, B2, c


def classify(T, x1, x2, variant="std"):
    """Return (pc, deg, code) arrays: pc = lo*3+hi pair class, deg = leading coeff zero,
    code = nZ + 3 nT + 9 nE over the DISTINCT finite F_p roots of a X^2 - 2 B2 X + c."""
    p, A = T.p, T.A
    x1 = np.broadcast_to(np.asarray(x1, dtype=np.int64), np.shape(x2)).astype(np.int64)
    x2 = np.asarray(x2, dtype=np.int64)
    a, B2, c = s3_coeffs(x1, x2, A, p, variant)
    d = (B2 * B2 - a * c) % p                           # disc/4, computed directly
    q = T.qr[d]                                         # 0 zero, 1 square, 2 non-square
    s = T.sqrt[d]
    ia = T.inv[a]
    X1 = (B2 + s) * ia % p
    X2 = ((B2 - s) % p) * ia % p
    has1 = (q != 2) & (a != 0)
    has2 = (q == 1) & (a != 0)
    code = np.where(has1, WROOT[T.cls[X1]], 0) + np.where(has2, WROOT[T.cls[X2]], 0)
    deg = a == 0
    if deg.any():
        idx = np.flatnonzero(deg)
        for j in idx:                                    # linear: -2 B2 X + c = 0
            b2, cc = int(B2[j]), int(c[j])
            if b2 != 0:
                X = cc * pow(2 * b2, -1, p) % p
                code[j] = int(WROOT[T.cls[X]])
            else:
                code[j] = 0 if cc != 0 else -999         # identically zero would be flagged
    k1 = T.cls[x1].astype(np.int64)
    k2 = T.cls[x2].astype(np.int64)
    pc = np.minimum(k1, k2) * 3 + np.maximum(k1, k2)
    return pc, deg, code


def roots_list(T, x1, x2, variant="std"):
    """Scalar: sorted list of distinct finite F_p roots (validator's quadratic formula)."""
    p = T.p
    a, B2, c = s3_coeffs(x1, x2, T.A, p, variant)
    a, B2, c = int(a), int(B2), int(c)
    if a == 0:
        if B2 == 0:
            return [] if c else None
        return [c * pow(2 * B2, -1, p) % p]
    d = (B2 * B2 - a * c) % p
    if d == 0:
        return [B2 * pow(a, -1, p) % p]
    if T.qr[d] == 2:
        return []
    s = int(T.sqrt[d])
    ia = pow(a, -1, p)
    return sorted({(B2 + s) * ia % p, (B2 - s) * ia % p})


# ---------------------------------------------------------------- x-only Montgomery ladder
def ladder(x, n, A, p):
    """[n] applied to the x-coordinate x on B y^2 = x^3 + A x^2 + x (any B): returns (X, Z)."""
    a24 = (A + 2) * pow(4, -1, p) % p
    X0, Z0 = 1, 0
    X1, Z1 = x % p, 1
    for bit in bin(n)[2:]:
        # diff add R0 + R1 (difference x), double
        if bit == "1":
            X0, Z0, X1, Z1 = *xadd(X0, Z0, X1, Z1, x, p), *xdbl(X1, Z1, a24, p)
        else:
            X1, Z1, X0, Z0 = *xadd(X0, Z0, X1, Z1, x, p), *xdbl(X0, Z0, a24, p)
    return X0, Z0


def xdbl(X, Z, a24, p):
    s = (X + Z) * (X + Z) % p
    d = (X - Z) * (X - Z) % p
    t = (s - d) % p
    return s * d % p, t * ((d + a24 * t) % p) % p


def xadd(XP, ZP, XQ, ZQ, xd, p):
    u = (XP - ZP) * (XQ + ZQ) % p
    v = (XP + ZP) * (XQ - ZQ) % p
    return (u + v) * (u + v) % p, xd * ((u - v) * (u - v) % p) % p


def x_order(x, N, A, p):
    """Exact order of the x-point x, given that [N]x = O."""
    assert ladder(x, N, A, p)[1] == 0
    o = N
    for q, e in factor_td(N).items():
        for _ in range(e):
            if o % q == 0 and ladder(x, o // q, A, p)[1] == 0:
                o //= q
            else:
                break
    return o


# ---------------------------------------------------------------- long-Weierstrass affine arithmetic
def lw_add(P, Q, a2, a4, p):
    """y^2 = x^3 + a2 x^2 + a4 x; None is the point at infinity."""
    if P is None:
        return Q
    if Q is None:
        return P
    (x1, y1), (x2, y2) = P, Q
    if x1 == x2:
        if (y1 + y2) % p == 0:
            return None
        lam = (3 * x1 * x1 + 2 * a2 * x1 + a4) * pow(2 * y1, -1, p) % p
    else:
        lam = (y2 - y1) * pow(x2 - x1, -1, p) % p
    x3 = (lam * lam - a2 - x1 - x2) % p
    y3 = (lam * (x1 - x3) - y1) % p
    return (x3, y3)


def lw_neg(P, p):
    return None if P is None else (P[0], (-P[1]) % p)
