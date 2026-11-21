"""Finite-field evaluation of the chi_18 formula (derive.py output) for J/L.

Tower: Kb = F_p[z]/(fb) (deg fb = 1 or 7), K2 = Kb[s]/(s^2 - n) with n a nonsquare
of F_p, K4 = K2[w]/(w^2 - m) with m a nonsquare of K2. Elements: Kb tuples of ints,
K2 = (Kb, Kb), K4 = (K2, K2).
"""
import itertools
import json
import random


class Kb:
    def __init__(self, p, fb):
        self.p, self.fb, self.d = p, fb, len(fb) - 1
        self.order = p ** self.d
        self.zero = (0,) * self.d
        self.one = (1,) + (0,) * (self.d - 1)

    def c(self, x):
        return (x % self.p,) + (0,) * (self.d - 1)

    def add(self, a, b):
        p = self.p
        return tuple((x + y) % p for x, y in zip(a, b))

    def sub(self, a, b):
        p = self.p
        return tuple((x - y) % p for x, y in zip(a, b))

    def neg(self, a):
        return tuple((-x) % self.p for x in a)

    def mul(self, a, b):
        p, d, fb = self.p, self.d, self.fb
        if d == 1:
            return (a[0] * b[0] % p,)
        r = [0] * (2 * d - 1)
        for i, x in enumerate(a):
            if x:
                for j, y in enumerate(b):
                    r[i + j] += x * y
        for k in range(2 * d - 2, d - 1, -1):
            t = r[k] % p
            if t:
                for j in range(d):
                    r[k - d + j] -= t * fb[j]
        return tuple(x % p for x in r[:d])

    def pow(self, a, e):
        r = self.one
        while e:
            if e & 1:
                r = self.mul(r, a)
            a = self.mul(a, a)
            e >>= 1
        return r

    def inv(self, a):
        return self.pow(a, self.order - 2)

    def is_zero(self, a):
        return not any(a)


class Quad:
    """F[s]/(s^2 - n) over base field object F."""

    def __init__(self, F, n):
        self.F, self.n = F, n
        self.order = F.order ** 2
        self.zero = (F.zero, F.zero)
        self.one = (F.one, F.zero)

    def add(self, a, b):
        F = self.F
        return (F.add(a[0], b[0]), F.add(a[1], b[1]))

    def sub(self, a, b):
        F = self.F
        return (F.sub(a[0], b[0]), F.sub(a[1], b[1]))

    def neg(self, a):
        return (self.F.neg(a[0]), self.F.neg(a[1]))

    def mul(self, a, b):
        F = self.F
        x0y0 = F.mul(a[0], b[0])
        x1y1 = F.mul(a[1], b[1])
        cross = F.sub(F.sub(F.mul(F.add(a[0], a[1]), F.add(b[0], b[1])), x0y0), x1y1)
        return (F.add(x0y0, F.mul(self.n, x1y1)), cross)

    def pow(self, a, e):
        r = self.one
        while e:
            if e & 1:
                r = self.mul(r, a)
            a = self.mul(a, a)
            e >>= 1
        return r

    def inv(self, a):
        return self.pow(a, self.order - 2)

    def is_zero(self, a):
        return self.F.is_zero(a[0]) and self.F.is_zero(a[1])

    def emb(self, x):
        return (x, self.F.zero)


def is_square(F, a):
    if F.is_zero(a):
        return True
    return F.pow(a, (F.order - 1) // 2) == F.one


def tonelli(F, a, nonsq):
    """sqrt of a square a in F (F.order odd)."""
    if F.is_zero(a):
        return F.zero
    q = F.order
    Q, S = q - 1, 0
    while Q % 2 == 0:
        Q //= 2
        S += 1
    z = F.pow(nonsq, Q)
    x = F.pow(a, (Q + 1) // 2)
    t = F.pow(a, Q)
    M = S
    while t != F.one:
        i, tt = 0, t
        while tt != F.one:
            tt = F.mul(tt, tt)
            i += 1
            assert i < M, "not a square"
        b = z
        for _ in range(M - i - 1):
            b = F.mul(b, b)
        x = F.mul(x, b)
        z = F.mul(b, b)
        t = F.mul(t, z)
        M = i
    assert F.mul(x, x) == a
    return x


class Tower:
    def __init__(self, p, fb, rng):
        self.p = p
        self.K = Kb(p, fb)
        n = 2
        while pow(n, (p - 1) // 2, p) == 1:
            n += 1
        self.nF = n
        self.K2 = Quad(self.K, self.K.c(n))
        while True:
            m = (tuple(rng.randrange(p) for _ in range(self.K.d)), tuple(rng.randrange(p) for _ in range(self.K.d)))
            if not self.K2.is_zero(m) and not is_square(self.K2, m):
                break
        self.m = m
        self.K4 = Quad(self.K2, m)
        self.inv_n = self.K.inv(self.K.c(n))
        self.inv_m = self.K2.inv(m)
        self.i4 = self.fourth_root_any(self.K.c(-1), rng=None, sq_only=True)  # sqrt(-1) in K4
        self.z8 = self.fourth_root_any(self.K.c(-1), rng=None)  # primitive 8th root

    def sqrt_K_to_K2(self, d):
        K, K2 = self.K, self.K2
        if is_square(K, d):
            return (tonelli(K, d, K.c(self.nF)), K.zero)
        dn = K.mul(d, K.c(self.nF))
        r = tonelli(K, dn, K.c(self.nF))
        return (K.zero, K.mul(r, self.inv_n))  # sqrt(dn) * s / n

    def sqrt_K2_to_K4(self, h):
        K2 = self.K2
        if is_square(K2, h):
            return (tonelli(K2, h, self.m), K2.zero)
        hm = K2.mul(h, self.m)
        r = tonelli(K2, hm, self.m)
        return (K2.zero, K2.mul(r, self.inv_m))

    def fourth_root_any(self, d, rng=None, sq_only=False):
        h = self.sqrt_K_to_K2(d)
        if sq_only:
            return self.K4.emb(h)
        r = self.sqrt_K2_to_K4(h)
        return r

    def check_fourth(self, r, d):
        K4 = self.K4
        r2 = K4.mul(r, r)
        return K4.mul(r2, r2) == K4.emb(self.K2.emb(d))


def load_formula(path):
    j = json.load(open(path))
    return j["formula"], [tuple(x) for x in j["pairs"]], j["report"]


def eval_formula(T, formula, PAIRS, es, rng, z8pow=1):
    """S(rho) with rho_ij a random fourth root of e_j - e_i (es: list of 8 Kb elements)."""
    K, K4 = T.K, T.K4
    rho = []
    ipows = [K4.one]
    for _ in range(3):
        ipows.append(K4.mul(ipows[-1], T.i4))
    for (i, j) in PAIRS:
        d = K.sub(es[j], es[i])
        r = T.fourth_root_any(d)
        assert T.check_fourth(r, d)
        if rng is not None:
            r = K4.mul(r, ipows[rng.randrange(4)])
        rho.append(r)
    z = K4.pow(T.z8, z8pow)
    zp = [K4.one]
    for _ in range(7):
        zp.append(K4.mul(zp[-1], z))
    sq = [K4.mul(r, r) for r in rho]
    tot = K4.one
    for fp in formula:
        acc = K4.zero
        for k, v in fp["terms"]:
            t = zp[k]
            for idx, e in enumerate(v):
                if e == 1:
                    t = K4.mul(t, rho[idx])
                elif e == 2:
                    t = K4.mul(t, sq[idx])
            acc = K4.add(acc, t)
        tot = K4.mul(tot, acc)
    return tot


def in_Fp(T, v):
    """If v (K4) lies in F_p return the int, else None."""
    K = T.K
    (a0, a1), (b0, b1) = v
    if K.is_zero(a1) and K.is_zero(b0) and K.is_zero(b1) and all(x == 0 for x in a0[1:]):
        return a0[0]
    return None


def legendre(x, p):
    x %= p
    if x == 0:
        return 0
    return 1 if pow(x, (p - 1) // 2, p) == 1 else -1


# ---------------------------------------------------------------- relabelling


def normset(Sset, labels=8):
    Sset = frozenset(Sset)
    comp = frozenset(range(labels)) - Sset
    return min(Sset, comp, key=lambda s: (len(s), sorted(s)))


def lag_key(subsets):
    return frozenset(normset(s) for s in subsets)


def sigma_table(L_subsets):
    """map: lag_key(sigma(L*)) -> list of sigma (tuple: real label -> target label)."""
    tab = {}
    for sig in itertools.permutations(range(8)):
        key = lag_key([[sig[i] for i in s] for s in L_subsets])
        tab.setdefault(key, []).append(sig)
    return tab


def all_lagrangians_of_type(tab):
    return list(tab.keys())
