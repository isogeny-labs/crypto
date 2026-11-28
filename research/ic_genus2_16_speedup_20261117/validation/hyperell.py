#!/usr/bin/env python3
"""Validator's own hyperelliptic arithmetic over F_p (p odd), written from
scratch for the P1/P2 validation; it shares no code with genus_ladder (Rust)
or the producer's analysis scripts.

Curve: y^2 = f(x), f monic, squarefree, deg f = 2g+1 (one point at infinity).
Polynomials are Python lists of ints mod p, lowest degree first, trimmed
(the zero polynomial is []).

Contents
  - F_p[x] arithmetic (add, sub, mul, divmod, xgcd, powmod, irreducibility).
  - Mumford/Cantor group law (composition + reduction), scalar multiples.
  - Random elements of J(F_p) as sums of random prime divisors (random monic
    irreducible u of degree d <= g with f a square mod u; v = sqrt(f) mod u by
    Tonelli-Shanks in F_p[x]/u).
  - Point counts N_k over F_{p^k} (numpy-vectorised enumeration, quadratic
    character by Euler's criterion in F_{p^k}).
  - L-polynomial from N_1..N_K (Newton identities).
  - Cartier-Manin / Hasse-Witt matrix, giving the Frobenius charpoly mod p.
"""
import random

import numpy as np


# ---------------------------------------------------------------- F_p[x]
def trim(a):
    while a and a[-1] == 0:
        a.pop()
    return a


def padd(a, b, p):
    n = max(len(a), len(b))
    return trim([((a[i] if i < len(a) else 0) + (b[i] if i < len(b) else 0)) % p for i in range(n)])


def psub(a, b, p):
    n = max(len(a), len(b))
    return trim([((a[i] if i < len(a) else 0) - (b[i] if i < len(b) else 0)) % p for i in range(n)])


def pneg(a, p):
    return [(-c) % p for c in a]


def pscale(a, c, p):
    c %= p
    if c == 0:
        return []
    return trim([(x * c) % p for x in a])


def pmul(a, b, p):
    if not a or not b:
        return []
    out = [0] * (len(a) + len(b) - 1)
    for i, x in enumerate(a):
        if x == 0:
            continue
        for j, y in enumerate(b):
            out[i + j] += x * y
    return trim([c % p for c in out])


def pdivmod(a, b, p):
    """Quotient and remainder of a by b (b nonzero)."""
    if not b:
        raise ZeroDivisionError
    a = list(a)
    db = len(b) - 1
    inv = pow(b[-1], p - 2, p)
    if len(a) - 1 < db:
        return [], trim(a)
    q = [0] * (len(a) - db)
    for k in range(len(a) - 1 - db, -1, -1):
        c = (a[k + db] * inv) % p
        q[k] = c
        if c:
            for j in range(db + 1):
                a[k + j] = (a[k + j] - c * b[j]) % p
    return trim(q), trim(a[:db])


def pmod(a, b, p):
    return pdivmod(a, b, p)[1]


def monic(a, p):
    if not a:
        return []
    return pscale(a, pow(a[-1], p - 2, p), p)


def pxgcd(a, b, p):
    """(d, s, t) with d = s a + t b, d monic (or [] if a = b = 0)."""
    r0, r1 = trim(list(a)), trim(list(b))
    s0, s1 = [1], []
    t0, t1 = [], [1]
    while r1:
        q, r = pdivmod(r0, r1, p)
        r0, r1 = r1, r
        s0, s1 = s1, psub(s0, pmul(q, s1, p), p)
        t0, t1 = t1, psub(t0, pmul(q, t1, p), p)
    if not r0:
        return [], [], []
    inv = pow(r0[-1], p - 2, p)
    return pscale(r0, inv, p), pscale(s0, inv, p), pscale(t0, inv, p)


def pgcd(a, b, p):
    return pxgcd(a, b, p)[0]


def ppowmod(a, e, m, p):
    res = [1]
    base = pmod(a, m, p)
    while e:
        if e & 1:
            res = pmod(pmul(res, base, p), m, p)
        e >>= 1
        if e:
            base = pmod(pmul(base, base, p), m, p)
    return res


def peval(a, x, p):
    acc = 0
    for c in reversed(a):
        acc = (acc * x + c) % p
    return acc


def prime_factors(n):
    out = []
    d = 2
    while d * d <= n:
        if n % d == 0:
            out.append(d)
            while n % d == 0:
                n //= d
        d += 1
    if n > 1:
        out.append(n)
    return out


def is_irreducible(u, p):
    """Rabin's test for monic u of degree d >= 1."""
    d = len(u) - 1
    if d == 1:
        return True
    x = [0, 1]
    # x^{p^k} mod u for k = 1..d
    frob = [None] * (d + 1)
    cur = pmod(x, u, p)
    for k in range(1, d + 1):
        cur = ppowmod(cur, p, u, p)
        frob[k] = cur
    if pmod(psub(frob[d], x, p), u, p) != []:  # need x^{p^d} = x mod u
        return False
    for r in prime_factors(d):
        h = psub(frob[d // r], x, p)
        if len(pgcd(u, h, p)) != 1:  # gcd must be the constant 1
            return False
    return True


# ---------------------------------------------------------- Cantor group law
class HCurve:
    def __init__(self, f, p):
        self.p = p
        self.f = trim([c % p for c in f])
        assert self.f[-1] == 1, "f must be monic"
        assert (len(self.f) - 1) % 2 == 1, "deg f must be odd"
        self.g = (len(self.f) - 2) // 2

    zero = ([1], [])

    def is_valid(self, D):
        u, v = D
        p = self.p
        if not u or u[-1] != 1 or len(u) - 1 > self.g:
            return False
        if len(v) >= len(u) and len(u) > 1:
            return False
        if len(u) == 1:
            return v == []
        return pmod(psub(pmul(v, v, p), self.f, p), u, p) == []

    def reduce(self, u, v):
        p, f, g = self.p, self.f, self.g
        while len(u) - 1 > g:
            u2 = pdivmod(psub(f, pmul(v, v, p), p), u, p)
            assert u2[1] == [], "non-exact division in reduction"
            u = monic(u2[0], p)
            v = pmod(pneg(v, p), u, p)
        u = monic(u, p)
        v = pmod(v, u, p) if len(u) > 1 else []
        return (u, v)

    def add(self, D1, D2):
        p, f = self.p, self.f
        u1, v1 = D1
        u2, v2 = D2
        d1, e1, e2 = pxgcd(u1, u2, p)
        d, c1, c2 = pxgcd(d1, padd(v1, v2, p), p)
        if not d:  # d1 = 0 impossible (u monic); v1+v2 = 0 and d1 = 0 impossible
            raise ValueError("degenerate composition")
        s1, s2, s3 = pmul(c1, e1, p), pmul(c1, e2, p), c2
        dd = pmul(d, d, p)
        u, rem = pdivmod(pmul(u1, u2, p), dd, p)
        assert rem == []
        num = padd(padd(pmul(pmul(s1, u1, p), v2, p), pmul(pmul(s2, u2, p), v1, p), p),
                   pmul(s3, padd(pmul(v1, v2, p), f, p), p), p)
        v, rem = pdivmod(num, d, p)
        assert rem == []
        v = pmod(v, u, p) if len(u) > 1 else []
        return self.reduce(u, v)

    def neg(self, D):
        u, v = D
        return (u, pmod(pneg(v, self.p), u, self.p) if len(u) > 1 else [])

    def mul(self, n, D):
        if n < 0:
            return self.mul(-n, self.neg(D))
        R = self.zero
        Q = D
        while n:
            if n & 1:
                R = self.add(R, Q)
            n >>= 1
            if n:
                Q = self.add(Q, Q)
        return R

    def is_zero(self, D):
        return D[0] == [1]

    # -- random elements
    def _sqrt_mod_irred(self, a, u, rng):
        """sqrt of a in F_p[x]/u (u irreducible of degree d), or None."""
        p = self.p
        d = len(u) - 1
        q = p ** d
        a = pmod(a, u, p)
        if not a:
            return []
        if ppowmod(a, (q - 1) // 2, u, p) != [1]:
            return None
        s, t = 0, q - 1
        while t % 2 == 0:
            s, t = s + 1, t // 2
        while True:
            z = trim([rng.randrange(p) for _ in range(d)])
            if z and ppowmod(z, (q - 1) // 2, u, p) == [p - 1]:
                break
        m, c = s, ppowmod(z, t, u, p)
        tt = ppowmod(a, t, u, p)
        r = ppowmod(a, (t + 1) // 2, u, p)
        while tt != [1]:
            i, t2 = 0, tt
            while t2 != [1]:
                t2 = pmod(pmul(t2, t2, p), u, p)
                i += 1
            b = c
            for _ in range(m - i - 1):
                b = pmod(pmul(b, b, p), u, p)
            m = i
            c = pmod(pmul(b, b, p), u, p)
            tt = pmod(pmul(tt, c, p), u, p)
            r = pmod(pmul(r, b, p), u, p)
        assert pmod(psub(pmul(r, r, p), a, p), u, p) == []
        return r

    def random_prime_divisor(self, d, rng):
        p = self.p
        while True:
            u = [rng.randrange(p) for _ in range(d)] + [1]
            if not is_irreducible(u, p):
                continue
            v = self._sqrt_mod_irred(self.f, u, rng)
            if v is None:
                continue
            if rng.randrange(2):
                v = pmod(pneg(v, p), u, p)
            D = (u, v)
            assert self.is_valid(D)
            return D

    def random_element(self, rng, parts=3):
        D = self.zero
        for _ in range(parts):
            d = rng.randrange(1, self.g + 1)
            D = self.add(D, self.random_prime_divisor(d, rng))
        return D


def element_order(C, D, N, N_factors):
    """Exact order of D given [N]D = 0 and the prime factorisation of N."""
    assert C.is_zero(C.mul(N, D))
    o = N
    for q in N_factors:
        while o % q == 0 and C.is_zero(C.mul(o // q, D)):
            o //= q
    return o


def factor(n):
    """Full factorisation by trial division + Pollard rho (n < 2^64 here)."""
    out = {}

    def is_prime(m):
        if m < 2:
            return False
        for q in (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37):
            if m % q == 0:
                return m == q
        dd, s = m - 1, 0
        while dd % 2 == 0:
            dd //= 2
            s += 1
        for a in (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37):
            x = pow(a, dd, m)
            if x in (1, m - 1):
                continue
            for _ in range(s - 1):
                x = x * x % m
                if x == m - 1:
                    break
            else:
                return False
        return True

    def rho(m):
        if m % 2 == 0:
            return 2
        for c in range(1, 200):
            x = y = 2
            dd = 1
            while dd == 1:
                x = (x * x + c) % m
                y = (y * y + c) % m
                y = (y * y + c) % m
                dd = __import__("math").gcd(abs(x - y), m)
            if dd != m:
                return dd
        raise RuntimeError("rho failed")

    def rec(m):
        if m == 1:
            return
        if is_prime(m):
            out[m] = out.get(m, 0) + 1
            return
        dd = rho(m)
        rec(dd)
        rec(m // dd)

    for q in range(2, 1000):
        while n % q == 0:
            out[q] = out.get(q, 0) + 1
            n //= q
    rec(n)
    return out


def is_prime(n):
    f = factor(n)
    return len(f) == 1 and list(f.values()) == [1]


# ------------------------------------------------------------ point counts
def _find_irreducible(k, p, rng):
    while True:
        m = [rng.randrange(p) for _ in range(k)] + [1]
        if is_irreducible(m, p):
            return m


def count_points_ext(f, p, k, rng=None, chunk=1 << 18):
    """N_k = #C(F_{p^k}) including the single point at infinity, by
    enumerating all x in F_{p^k} = F_p[t]/m(t) (numpy, vectorised).
    chi(y) = y^{(q-1)/2} in F_{p^k}."""
    rng = rng or random.Random(12345)
    q = p ** k
    if k == 1:
        tot = 0
        for x in range(p):
            y = peval(f, x, p)
            if y == 0:
                tot += 1
            elif pow(y, (p - 1) // 2, p) == 1:
                tot += 2
        return tot + 1
    m = _find_irreducible(k, p, rng)
    # reduction table: t^j mod m for j = k .. 2k-2, as length-k vectors
    red = []
    cur = [0] * k
    # t^k = -(m_0 + ... + m_{k-1} t^{k-1})
    tk = [(-c) % p for c in m[:k]]
    cur = tk[:]
    red.append(cur[:])
    for _ in range(k, 2 * k - 2):
        # multiply cur by t
        top = cur[-1]
        nxt = [0] + cur[:-1]
        nxt = [(nxt[i] + top * tk[i]) % p for i in range(k)]
        cur = nxt
        red.append(cur[:])
    red = np.array(red, dtype=np.int64) if red else np.zeros((0, k), dtype=np.int64)

    def fmul(A, B):
        # A, B: (n, k) arrays; product mod m
        n = A.shape[0]
        prod = np.zeros((n, 2 * k - 1), dtype=np.int64)
        for i in range(k):
            prod[:, i:i + k] += A[:, i:i + 1] * B
        prod %= p
        res = prod[:, :k].copy()
        for j in range(k, 2 * k - 1):
            res += prod[:, j:j + 1] * red[j - k][None, :]
        return res % p

    def fpow(A, e):
        n = A.shape[0]
        R = np.zeros((n, k), dtype=np.int64)
        R[:, 0] = 1
        B = A.copy()
        while e:
            if e & 1:
                R = fmul(R, B)
            e >>= 1
            if e:
                B = fmul(B, B)
        return R

    total = 0
    e = (q - 1) // 2
    for start in range(0, q, chunk):
        idx = np.arange(start, min(q, start + chunk), dtype=np.int64)
        X = np.zeros((idx.size, k), dtype=np.int64)
        t = idx.copy()
        for i in range(k):
            X[:, i] = t % p
            t //= p
        # Horner: Y = f(X)
        Y = np.zeros_like(X)
        for c in reversed(f):
            Y = fmul(Y, X)
            Y[:, 0] = (Y[:, 0] + c) % p
        zero = ~Y.any(axis=1)
        Z = fpow(Y, e)
        one = (Z[:, 0] == 1) & (~Z[:, 1:].any(axis=1))
        total += int(zero.sum()) + 2 * int((one & ~zero).sum())
    return total + 1


def lpoly_from_counts(N, p, g):
    """L-polynomial coefficients a_0..a_K from N_1..N_K (K <= g), and the
    full L(t) (length 2g+1) when K = g (functional equation)."""
    K = len(N)
    S = [None] + [p ** k + 1 - N[k - 1] for k in range(1, K + 1)]
    a = [1] + [0] * K
    for k in range(1, K + 1):
        acc = 0
        for i in range(1, k + 1):
            acc += S[i] * a[k - i]
        assert acc % k == 0
        a[k] = -acc // k
    if K < g:
        return a, None
    full = a[:g + 1] + [p ** j * a[g - j] for j in range(1, g + 1)]
    return a, full


def hasse_witt(f, p, g):
    """Cartier-Manin matrix A[i][j] = c_{i p - j}, i, j = 1..g, where c_k is
    the coefficient of x^k in f^{(p-1)/2}."""
    e = (p - 1) // 2
    F = np.array(f, dtype=np.int64)
    R = np.array([1], dtype=np.int64)
    B = F.copy()
    while e:
        if e & 1:
            R = np.convolve(R, B) % p
        e >>= 1
        if e:
            B = np.convolve(B, B) % p
    c = [int(x) for x in R]
    A = [[(c[i * p - j] if 0 <= i * p - j < len(c) else 0) for j in range(1, g + 1)] for i in range(1, g + 1)]
    return A


def charpoly_mod_p(A, p):
    """Coefficients of det(t I - A) mod p, lowest degree first (Faddeev-
    LeVerrier is unsafe mod small p; use Hessenberg-free interpolation)."""
    g = len(A)
    # det(t I - A) at t = 0..g, then Lagrange interpolation mod p (p > g needed)
    assert p > g

    def det_mod(M):
        M = [row[:] for row in M]
        n = len(M)
        d = 1
        for col in range(n):
            piv = None
            for r in range(col, n):
                if M[r][col] % p:
                    piv = r
                    break
            if piv is None:
                return 0
            if piv != col:
                M[col], M[piv] = M[piv], M[col]
                d = -d
            d = d * M[col][col] % p
            inv = pow(M[col][col], p - 2, p)
            for r in range(col + 1, n):
                fct = M[r][col] * inv % p
                if fct:
                    for cc in range(col, n):
                        M[r][cc] = (M[r][cc] - fct * M[col][cc]) % p
        return d % p

    xs = list(range(g + 1))
    ys = [det_mod([[((t if i == j else 0) - A[i][j]) % p for j in range(g)] for i in range(g)]) for t in xs]
    # Lagrange interpolation
    coeffs = [0] * (g + 1)
    for i, xi in enumerate(xs):
        basis = [1]
        denom = 1
        for j, xj in enumerate(xs):
            if j == i:
                continue
            basis = pmul(basis, [(-xj) % p, 1], p) or [0]
            denom = denom * (xi - xj) % p
        sc = ys[i] * pow(denom, p - 2, p) % p
        for k, c in enumerate(basis):
            coeffs[k] = (coeffs[k] + sc * c) % p
    return coeffs
