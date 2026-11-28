"""Validator's own arithmetic for genus_ladder_scaling_20261119 (independent of the solver).

Polynomials over F_p are lists of ints, lowest degree first, trimmed (no trailing zeros; [] is 0).
Divisors on the Jacobian of y^2 = f(x) (deg f = 2g + 1, monic) are Mumford pairs (u, v):
u monic of degree <= g, deg v < deg u, u | v^2 - f.  The identity is ([1], []).
Nothing here imports or calls the solver.  The seed -> f and seed -> (G, t) maps are re-implemented
from their documented specification (src/cryptanalysis/genus_ladder.rs at ca72267899: random_f,
random_divisor, instance_from; residual_walk.rs mix64) so that the validator can rebuild an instance
from its seeds and check it with this module's group law.
"""
from decimal import Decimal, getcontext

M64 = (1 << 64) - 1
GOLD = 0x9E3779B97F4A7C15


# ---------------------------------------------------------------- polynomials over F_p
def trim(a):
    a = list(a)
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
    return trim([(-c) % p for c in a])


def pmul(a, b, p):
    if not a or not b:
        return []
    out = [0] * (len(a) + len(b) - 1)
    for i, x in enumerate(a):
        if x:
            for j, y in enumerate(b):
                out[i + j] += x * y
    return trim([c % p for c in out])


def pscale(a, c, p):
    return trim([x * c % p for x in a])


def pdivmod(a, b, p):
    a = trim(a)
    b = trim(b)
    if not b:
        raise ZeroDivisionError("polynomial division by zero")
    inv = pow(b[-1], p - 2, p)
    q = [0] * max(0, len(a) - len(b) + 1)
    r = list(a)
    db = len(b) - 1
    for k in range(len(a) - len(b), -1, -1):
        c = r[k + db] * inv % p
        q[k] = c
        if c:
            for j in range(len(b)):
                r[k + j] = (r[k + j] - c * b[j]) % p
    return trim(q), trim(r[:db])


def pmod(a, b, p):
    return pdivmod(a, b, p)[1]


def monic(a, p):
    a = trim(a)
    if not a:
        return a
    return pscale(a, pow(a[-1], p - 2, p), p)


def pxgcd(a, b, p):
    """(d, s, t) with d = s a + t b, d monic (or 0 if a = b = 0)."""
    r0, r1 = trim(a), trim(b)
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


def pderiv(a, p):
    return trim([(i * a[i]) % p for i in range(1, len(a))])


def peval(a, x, p):
    acc = 0
    for c in reversed(a):
        acc = (acc * x + c) % p
    return acc


def ppowmod(base, e, m, p):
    result = [1]
    b = pmod(base, m, p)
    while e:
        if e & 1:
            result = pmod(pmul(result, b, p), m, p)
        b = pmod(pmul(b, b, p), m, p)
        e >>= 1
    return result


def prime_factors(n):
    out, d = [], 2
    while d * d <= n:
        while n % d == 0:
            if d not in out:
                out.append(d)
            n //= d
        d += 1
    if n > 1 and n not in out:
        out.append(n)
    return out


def is_irreducible(f, p):
    """Rabin's test for a monic f of degree n >= 1 over F_p."""
    n = len(f) - 1
    x = [0, 1]
    frob = [None, ppowmod(x, p, f, p)]  # frob[k] = x^(p^k) mod f
    for k in range(2, n + 1):
        frob.append(ppowmod(frob[k - 1], p, f, p))
    if trim(psub(frob[n], x, p)):
        return False
    for q in prime_factors(n):
        if len(pgcd(f, psub(frob[n // q], x, p), p)) > 1:
            return False
    return True


def is_squarefree(f, p):
    d = pderiv(f, p)
    return bool(d) and len(pgcd(f, d, p)) == 1


# ---------------------------------------------------------------- integers
def is_prime(n):
    """Deterministic Miller-Rabin for n < 3.3e24 (bases: the first 13 primes)."""
    if n < 2:
        return False
    small = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41]
    for q in small:
        if n % q == 0:
            return n == q
    d, s = n - 1, 0
    while d % 2 == 0:
        d //= 2
        s += 1
    for a in small:
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


def hasse_weil(p, g):
    """Integer bounds [lo, hi] with (sqrt p - 1)^(2g) <= #J <= (sqrt p + 1)^(2g)."""
    getcontext().prec = 80
    s = Decimal(p).sqrt()
    lo = (s - 1) ** (2 * g)
    hi = (s + 1) ** (2 * g)
    lo_i = int(lo) + (0 if lo == int(lo) else 1)
    return lo_i, int(hi)


# ---------------------------------------------------------------- Jacobian (Cantor)
class Jac:
    def __init__(self, p, g, f):
        assert len(f) == 2 * g + 2 and f[-1] == 1
        self.p, self.g, self.f = p, g, trim(f)

    ID = ([1], [])

    def is_valid(self, D):
        u, v = D
        p = self.p
        if not u or u[-1] != 1 or len(u) - 1 > self.g:
            return False
        if len(v) >= len(u):
            return False
        return not pmod(psub(self.f, pmul(v, v, p), p), u, p)

    def reduce(self, u, v):
        p, f, g = self.p, self.f, self.g
        u = monic(u, p)
        v = pmod(v, u, p)
        while len(u) - 1 > g:
            u2, rem = pdivmod(psub(f, pmul(v, v, p), p), u, p)
            assert not rem
            u = monic(u2, p)
            v = pmod(pneg(v, p), u, p)
        return (u, v)

    def add(self, A, B):
        p, f = self.p, self.f
        u1, v1 = A
        u2, v2 = B
        if u1 == [1]:
            return B
        if u2 == [1]:
            return A
        d1, e1, e2 = pxgcd(u1, u2, p)
        d, c1, c2 = pxgcd(d1, padd(v1, v2, p), p)
        if not d:  # cannot happen for reduced divisors (u1, u2 nonzero)
            raise ValueError("degenerate composition")
        s1, s2, s3 = pmul(c1, e1, p), pmul(c1, e2, p), c2
        uu, rem = pdivmod(pmul(u1, u2, p), pmul(d, d, p), p)
        assert not rem
        num = padd(padd(pmul(pmul(s1, u1, p), v2, p), pmul(pmul(s2, u2, p), v1, p), p),
                   pmul(s3, padd(pmul(v1, v2, p), f, p), p), p)
        vv, rem = pdivmod(num, d, p)
        assert not rem
        return self.reduce(uu, vv)

    def neg(self, A):
        return (A[0], pneg(A[1], self.p))

    def mul(self, A, k):
        R = self.ID
        Q = A
        while k:
            if k & 1:
                R = self.add(R, Q)
            Q = self.add(Q, Q)
            k >>= 1
        return R

    def point(self, x, y):
        assert y * y % self.p == peval(self.f, x, self.p)
        return self.reduce([(-x) % self.p, 1], [y % self.p] if y % self.p else [])


# ---------------------------------------------------------------- seed specifications
def mix64(z):
    z = (z + GOLD) & M64
    z = ((z ^ (z >> 30)) * 0xBF58476D1CE4E5B9) & M64
    z = ((z ^ (z >> 27)) * 0x94D049BB133111EB) & M64
    return z ^ (z >> 31)


def random_f(p, g, cs):
    st = ((cs * GOLD) & M64) ^ p ^ (g << 48)
    f = []
    for _ in range(2 * g + 1):
        st = (st + GOLD) & M64
        f.append(mix64(st) % p)
    f.append(1)
    return f


def sqrt_mod_spec(a, p):
    """The square root the specification draws (Tonelli-Shanks as written in the source)."""
    a %= p
    if a == 0:
        return 0
    if pow(a, (p - 1) // 2, p) != 1:
        return None
    if p % 4 == 3:
        return pow(a, (p + 3) // 4, p)  # div_ceil(p, 4) = (p + 3) // 4 for p = 3 mod 4
    q, s = p - 1, 0
    while q % 2 == 0:
        q //= 2
        s += 1
    z = 2
    while pow(z, (p - 1) // 2, p) != p - 1:
        z += 1
    m, c, t, r = s, pow(z, q, p), pow(a, q, p), pow(a, (q + 1) // 2, p)
    while t != 1:
        i, tt = 0, t
        while tt != 1:
            tt = tt * tt % p
            i += 1
        b = pow(c, 1 << (m - i - 1), p)
        m, c, t, r = i, b * b % p, t * b * b % p, r * b % p
    return r


def random_divisor_spec(J, st):
    p = J.p
    D = J.ID
    placed = 0
    while placed < J.g:
        st = (st + GOLD) & M64
        h = mix64(st)
        x = h % p
        s = sqrt_mod_spec(peval(J.f, x, p), p)
        if s is None:
            continue
        y = (p - s) % p if (h >> 63) & 1 else s
        D = J.add(D, J.point(x, y))
        placed += 1
    return D, st


def instance_spec(J, h, r, target_seed):
    """(G, t, T, st) as instance_from builds them, recomputed with this module's group law."""
    st = target_seed ^ 0x123456789ABCDEF0
    G = None
    for _ in range(64):
        d, st = random_divisor_spec(J, st)
        cand = J.mul(d, h)
        if cand != J.ID and J.mul(cand, r) == J.ID:
            G = cand
            break
    if G is None:
        raise ValueError("no generator of order r")
    st = mix64(st)
    t = 1 + mix64(st) % (r - 1)
    return G, t, J.mul(G, t)


def affine_automorphisms(f, p, g):
    """All (a, b) with f(a x + b) = a^(2g+1) f(x); (1, 0) is the identity."""
    n = 2 * g + 1
    inv_n = pow(n, p - 2, p)
    f2g = f[2 * g]
    pts = [1, 2, 3, 5, 7]
    fv = [peval(f, x, p) for x in pts]
    out = []
    for a in range(1, p):
        b = f2g * (a - 1) % p * inv_n % p
        an = pow(a, n, p)
        if all(peval(f, (a * x + b) % p, p) == an * v % p for x, v in zip(pts, fv)):
            # full check: compose f(a x + b)
            comp = []
            lin = [b, a]
            acc = [1]
            for c in f:
                comp = padd(comp, pscale(acc, c, p), p)
                acc = pmul(acc, lin, p)
            if comp == pscale(f, an, p):
                out.append((a, b, pow(an, (p - 1) // 2, p) == 1))
    return out
