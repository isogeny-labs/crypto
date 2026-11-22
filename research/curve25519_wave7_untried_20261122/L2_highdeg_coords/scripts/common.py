#!/usr/bin/env python3
"""L2 (wave 7): shared curve, point-table and coordinate-function code.

Curve: Montgomery v^2 = u^3 + A u^2 + u over F_p (B = 1), the Curve25519
family miniatures. Short Weierstrass model x = u + A/3, y = v (the engine's).
E(F_p) is cyclic of order n = 8r; every point carries its full log in Z/n.
"""
import hashlib
import random
import numpy as np

# Family-rule curves (research/curve25519_frame_search_20261103, erratum E1).
CURVES = {
    16: dict(p=65437, A=1058, n=65672),
    18: dict(p=262133, A=78, n=263096),
    20: dict(p=1048573, A=926, n=1049624),
}


def dseed(*parts):
    """Derived seed: first 12 hex digits of sha256('wave7-L2|' + '|'.join(parts))."""
    s = 'wave7-L2|' + '|'.join(str(x) for x in parts)
    return int(hashlib.sha256(s.encode()).hexdigest()[:12], 16)


def is_prime(n):
    if n < 2:
        return False
    i = 2
    while i * i <= n:
        if n % i == 0:
            return False
        i += 1
    return True


def vpow(base, e, p):
    """Vectorised modular power, p < 2^31 (products < 2^62)."""
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


def sqrt_mod(a, p):
    a %= p
    if a == 0:
        return 0
    if pow(a, (p - 1) // 2, p) != 1:
        return None
    assert p % 8 == 5
    x = pow(a, (p + 3) // 8, p)
    if x * x % p != a:
        x = x * pow(2, (p - 1) // 4, p) % p
    return x


class Curve:
    """Point table of the cyclic group E(F_p) = <g>, with exact facts verified."""

    def __init__(self, k):
        c = CURVES[k]
        self.k, self.p, self.A, self.n = k, c['p'], c['A'], c['n']
        p, A, n = self.p, self.A, self.n
        self.r = n // 8
        self.A3 = A * pow(3, -1, p) % p
        self.facts = self._verify()
        self._build()

    # --- curve arithmetic (Montgomery affine, None = O)
    def add(self, P, Q):
        p, A = self.p, self.A
        if P is None:
            return Q
        if Q is None:
            return P
        (u1, v1), (u2, v2) = P, Q
        if u1 == u2:
            if (v1 + v2) % p == 0:
                return None
            lam = (3 * u1 * u1 + 2 * A * u1 + 1) * pow(2 * v1, -1, p) % p
        else:
            lam = (v2 - v1) * pow(u2 - u1, -1, p) % p
        u3 = (lam * lam - A - u1 - u2) % p
        return (u3, (lam * (u1 - u3) - v1) % p)

    def mul(self, k, P):
        R = None
        while k:
            if k & 1:
                R = self.add(R, P)
            P = self.add(P, P)
            k >>= 1
        return R

    def _verify(self):
        p, A, n, r = self.p, self.A, self.n, self.r
        f = {}
        f['p_prime'] = is_prime(p)
        f['p_mod_8'] = p % 8
        f['A2m4_nonsquare'] = pow((A * A - 4) % p, (p - 1) // 2, p) == p - 1
        # exact point count by Euler's criterion over every u
        u = np.arange(p, dtype=np.int64)
        rhs = (u * u % p * u + A * u % p * u + u) % p
        leg = vpow(rhs, (p - 1) // 2, p)
        cnt = 1 + int((rhs == 0).sum()) + 2 * int((leg == 1).sum())
        f['count_E'] = cnt
        f['count_matches_n'] = cnt == n
        f['n_eq_8r'] = n == 8 * r
        f['r_prime'] = is_prime(r)
        nt = 2 * p + 2 - n
        f['twist_order'] = nt
        f['twist_4rprime'] = nt % 4 == 0 and is_prime(nt // 4)
        f['A_plus_2_square'] = pow((A + 2) % p, (p - 1) // 2, p) == 1
        assert all(f[x] for x in ('p_prime', 'A2m4_nonsquare', 'count_matches_n', 'n_eq_8r', 'r_prime', 'twist_4rprime')), f
        return f

    def _build(self):
        p, A, n, r = self.p, self.A, self.n, self.r
        rng = random.Random(1)
        while True:
            u = rng.randrange(2, p)
            v = sqrt_mod(u * u % p * u + A * u * u + u, p)
            if v is None:
                continue
            g = (u, v)
            if self.mul(n // 2, g) is not None and self.mul(8, g) is not None and self.mul(n, g) is None:
                break
        self.g = g
        U = np.full(n, -1, dtype=np.int64)
        V = np.full(n, -1, dtype=np.int64)
        P = None
        for l in range(1, n // 2 + 1):
            P = self.add(P, g)
            U[l], V[l] = P
            if l < n // 2:
                U[n - l], V[n - l] = P[0], (-P[1]) % p
        assert P == (0, 0)  # [n/2] g = T0
        self.U, self.V = U, V          # index = log; index 0 = O (U = -1)
        self.logs = np.arange(1, n, dtype=np.int64)
        self.facts['generator'] = list(g)
        self.facts['distinct_u_pairs'] = int(np.unique(U[1:] * p + V[1:]).size) == n - 1

    # --- coordinate functions: value array and validity mask over all logs 1..n-1
    def fvals(self, name):
        p, A3 = self.p, self.A3
        u = self.U[1:]
        v = self.V[1:]
        x = (u + A3) % p
        ok = np.ones(u.size, dtype=bool)
        if name == 'x':        # Weierstrass x = X/Z (proj.) = X/Z^2 (Jacobian); Montgomery u up to +A/3
            val = x
        elif name == 'v':      # y = Y/Z (proj.), Y/Z^3 (Jacobian), Montgomery v
            val = v
        elif name == 'q':      # Weierstrass x/y = X/Y (proj.), XZ/Y (Jacobian)
            ok = v != 0
            val = x * vinv(np.where(ok, v, 1), p) % p
        elif name == 'uv':     # Montgomery XY/Z^2
            val = u * v % p
        elif name == 'xeye':   # extended Edwards T/Z = x_E y_E, x_E = u/v, y_E = (u-1)/(u+1)
            den = v * ((u + 1) % p) % p
            ok = den != 0
            val = u * ((u - 1) % p) % p * vinv(np.where(ok, den, 1), p) % p
        elif name == 'jac6':   # Jacobian Y^2/X^3 = y^2/x^3
            ok = x != 0
            val = v * v % p * vinv(np.where(ok, x, 1) ** 1, p) % p
            val = val * vinv(np.where(ok, x, 1), p) % p * vinv(np.where(ok, x, 1), p) % p
        elif name == 'ye':     # Edwards y = Y/Z = (u-1)/(u+1)  (degree-2 control)
            ok = (u + 1) % p != 0
            val = (u - 1) % p * vinv(np.where(ok, (u + 1) % p, 1), p) % p
        elif name == 'xe':     # Montgomery X/Y = u/v = Edwards x up to scale (degree-2 control)
            ok = v != 0
            val = u * vinv(np.where(ok, v, 1), p) % p
        else:
            raise KeyError(name)
        return np.where(ok, val, -1), ok


FUNCS = ['x', 'v', 'q', 'uv', 'xeye', 'jac6', 'ye', 'xe']
# Desk values (PROTOCOL.md section 2): degree of f: E -> P^1, parity under P -> -P,
# predicted degree of S^f in each variable.
DESK = {
    'x': dict(deg=2, parity='even', pervar=2),
    'v': dict(deg=3, parity='odd', pervar=9),
    'q': dict(deg=3, parity='odd', pervar=9),
    'uv': dict(deg=5, parity='odd', pervar=25),
    'xeye': dict(deg=4, parity='odd', pervar=4),
    'jac6': dict(deg=6, parity='even', pervar=18),
    'ye': dict(deg=2, parity='even', pervar=2),
    'xe': dict(deg=2, parity='odd', pervar=2),
}


class FTable:
    """Points sorted by f-value, for interval queries V = [L, L+M) mod p."""

    def __init__(self, C, name):
        val, ok = C.fvals(name)
        logs = C.logs[ok]
        val = val[ok]
        o = np.argsort(val, kind='stable')
        self.val = val[o]
        self.log = logs[o]
        self.p, self.n = C.p, C.n
        self.uvals = np.unique(self.val)

    def interval(self, L, M):
        """logs of points with f in [L, L+M) mod p."""
        p = self.p
        L %= p
        if L + M <= p:
            a, b = np.searchsorted(self.val, [L, L + M])
            return self.log[a:b]
        a = np.searchsorted(self.val, L)
        b = np.searchsorted(self.val, L + M - p)
        return np.concatenate([self.log[a:], self.log[:b]])

    def first_values(self, L, c):
        """The first c distinct f-values >= L (cyclically)."""
        i = np.searchsorted(self.uvals, L % self.p)
        idx = (i + np.arange(c)) % self.uvals.size
        return self.uvals[idx]


def fold(logs, n):
    logs = np.asarray(logs, dtype=np.int64) % n
    return np.unique(np.minimum(logs, n - logs))


def coverage(classes, n):
    """(covered, cols, J) exactly as research/curve25519_frame_search_20261103/scripts/frame_census.py."""
    if len(classes) == 0:
        return 0, 0, 0
    L = np.asarray(classes, dtype=np.int64)
    S = np.concatenate([L, (-L) % n])
    sums = (S[:, None] + S[None, :]) % n
    sub = (sums % 8 == 0) & (sums != 0)
    J = int(np.triu(sub).sum())
    return int(np.unique(sums[sub]).size), len(L), J
