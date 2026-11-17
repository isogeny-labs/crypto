"""Counted Wiedemann / Berlekamp-Massey over Z/r (SPEC.md section 8.3)."""
import random
from operator import mul as _mul


def bm(s, r, ops):
    Cp = [1]
    Bp = [1]
    L = 0
    m = 1
    binv = 1
    for n in range(len(s)):
        d = s[n]
        if L:
            d += sum(map(_mul, Cp[1:L + 1], s[n - 1::-1][:L]))
        ops.mul += L
        d %= r
        if d == 0:
            m += 1
            continue
        coef = d * binv % r
        ops.mul += 1
        T = Cp[:]
        need = len(Bp) + m
        if len(Cp) < need:
            Cp.extend([0] * (need - len(Cp)))
        for i, bi in enumerate(Bp):
            Cp[i + m] = (Cp[i + m] - coef * bi) % r
        ops.mul += len(Bp)
        if 2 * L <= n:
            L = n + 1 - L
            Bp = T
            binv = pow(d, r - 2, r)
            ops.inv += 1
            m = 1
            if len(Cp) < L + 1:
                Cp.extend([0] * (L + 1 - len(Cp)))
        else:
            m += 1
    if len(Cp) < L + 1:
        Cp.extend([0] * (L + 1 - len(Cp)))
    return Cp[:L + 1], L


def _build(rows, j0, r):
    ncols = None
    A = []
    b = []
    W = 0
    for row in rows:
        ent = []
        bi = 0
        for col, coef in row:
            if col == j0:
                bi = (-coef) % r
            else:
                nc = col if col < j0 else col - 1
                ent.append((nc, coef % r))
        A.append(ent)
        b.append(bi)
        W += len(ent)
    return A, b, W


def attempt(rows, ncols, j0, r, ss, a, ops):
    """One Wiedemann attempt. Returns solution list x (length ncols-1) or None."""
    N = ncols - 1
    A, b, W = _build(rows, j0, r)
    R = len(rows)
    rowsA = [(tuple(c for c, _ in ent), tuple(v for _, v in ent)) for ent in A]
    cols_t = [[] for _ in range(N)]
    for i, ent in enumerate(A):
        for c, v in ent:
            cols_t[c].append((i, v))
    rowsAT = [(tuple(i for i, _ in lst), tuple(v for _, v in lst)) for lst in cols_t]
    la_rng = random.Random(ss + 50000 + 1000 * a)
    D = [la_rng.randrange(1, r) for _ in range(R)]
    u = [la_rng.randrange(r) for _ in range(N)]

    def Av(v):
        ops.mul += W
        return [sum(map(_mul, cf, [v[k] for k in cl])) % r for cl, cf in rowsA]

    def ATt(t):
        ops.mul += W
        return [sum(map(_mul, cf, [t[k] for k in cl])) % r for cl, cf in rowsAT]

    def Bv(v):
        av = Av(v)
        ops.mul += R
        t = [D[i] * av[i] % r for i in range(R)]
        return ATt(t)

    def dot(x, y):
        ops.mul += N
        return sum(map(_mul, x, y)) % r

    ops.mul += R
    tb = [D[i] * b[i] % r for i in range(R)]
    y = ATt(tb)
    s = []
    v = y
    for k in range(2 * N):
        s.append(dot(u, v))
        if k < 2 * N - 1:
            v = Bv(v)
    Cp, L = bm(s, r, ops)
    if L == 0:
        return None
    g = [Cp[L - k] for k in range(L + 1)]
    if g[0] == 0:
        return None
    ops.mul += N
    z = [g[L] * yk % r for yk in y]
    for k in range(L - 1, 0, -1):
        z = Bv(z)
        ops.mul += N
        gk = g[k]
        z = [(zi + gk * yi) % r for zi, yi in zip(z, y)]
    ig = pow(g[0], r - 2, r)
    ops.inv += 1
    ops.mul += N
    x = [(-ig * zi) % r for zi in z]
    chk = Av(x)
    if chk == b:
        return x
    return None


def solve_fixed(rows, ncols, j0, r, ss, ops, max_attempts=12):
    """Retry the same rows (used by unit tests). Returns (x, attempts)."""
    for a in range(1, max_attempts + 1):
        x = attempt(rows, ncols, j0, r, ss, a, ops)
        if x is not None:
            return x, a
    return None, max_attempts
