# Validator copy (genus4_prime_order_20261118) of research/genus4_followup_20261118/validation/scripts/jac.py,
# byte-identical below this header. Re-reviewed 2026-11-18 against ca72267899: between ae36cdc6e4 and
# ca72267899 genus_ladder.rs only adds f_irreducible (838-915) and one test; mix64, random_f, random_divisor,
# point, sqrt_mod and instance_from are unchanged. p = 16411 and p = 32771 are both 3 mod 4.
# Validator copy (genus4_followup_20261118) of research/genus4_positive_control_20261116/validation/scripts/jac.py,
# byte-identical below this header. Reviewed 2026-11-18 against src/cryptanalysis/genus_ladder.rs and
# residual_walk.rs at ae36cdc6e4: mix64 (residual_walk.rs 336), random_f (842), sqrt_mod (303; p = 16411 is
# 3 mod 4, so the a^((p+1)/4) branch), random_divisor (516), point (501), instance_from (867). Cantor
# composition/reduction follows Cantor 1987 for y^2 = f (h = 0); the reduced Mumford form is unique.
#!/usr/bin/env python3
"""Validator's own Jacobian arithmetic for imaginary hyperelliptic curves
y^2 = f(x) over F_p (odd p), written from Cantor (1987) / Koblitz, not from the
producer's Rust.  Divisors are reduced Mumford pairs (u, v): u monic, deg v < deg u,
u | f - v^2; polynomials are tuples of ints, lowest degree first, no trailing zeros.
The reduced representation is unique, so equality of pairs is equality in J(F_p).

Also: the producer's seeding (SplitMix-style mix64, random_f, random_divisor,
instance_from) re-implemented from its documented behaviour so the planted
instance can be re-derived and checked independently.
"""
M64 = (1 << 64) - 1


# ---------- polynomials over F_p ----------
def trim(a):
    a = list(a)
    while a and a[-1] == 0:
        a.pop()
    return tuple(a)


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
        return ()
    out = [0] * (len(a) + len(b) - 1)
    for i, x in enumerate(a):
        if x:
            for j, y in enumerate(b):
                out[i + j] += x * y
    return trim([c % p for c in out])


def pscale(a, s, p):
    return trim([c * s % p for c in a])


def pdivmod(a, b, p):
    if not b:
        raise ZeroDivisionError("poly division by zero")
    a = list(a)
    db = len(b) - 1
    inv = pow(b[-1], p - 2, p)
    if len(a) < len(b):
        return (), trim(a)
    q = [0] * (len(a) - db)
    for k in range(len(a) - 1, db - 1, -1):
        c = a[k] * inv % p
        q[k - db] = c
        if c:
            for j in range(db + 1):
                a[k - db + j] = (a[k - db + j] - c * b[j]) % p
    return trim(q), trim(a[:db])


def pmonic(a, p):
    if not a:
        return a
    return pscale(a, pow(a[-1], p - 2, p), p)


def pxgcd(a, b, p):
    """(g, s, t) with s a + t b = g, g monic (or zero)."""
    r0, r1 = a, b
    s0, s1 = (1,), ()
    t0, t1 = (), (1,)
    while r1:
        q, r = pdivmod(r0, r1, p)
        r0, r1 = r1, r
        s0, s1 = s1, psub(s0, pmul(q, s1, p), p)
        t0, t1 = t1, psub(t0, pmul(q, t1, p), p)
    if not r0:
        return r0, s0, t0
    li = pow(r0[-1], p - 2, p)
    return pscale(r0, li, p), pscale(s0, li, p), pscale(t0, li, p)


def peval(a, x, p):
    acc = 0
    for c in reversed(a):
        acc = (acc * x + c) % p
    return acc


def pderiv(a, p):
    return trim([(i * a[i]) % p for i in range(1, len(a))])


# ---------- the Jacobian ----------
class Jac:
    def __init__(self, p, f):
        self.p = p
        self.f = trim(f)
        self.g = (len(self.f) - 2) // 2
        assert len(self.f) == 2 * self.g + 2 and self.f[-1] == 1, "monic deg 2g+1"
        gg, _, _ = pxgcd(self.f, pderiv(self.f, p), p)
        self.squarefree = gg == (1,)
        self.zero = ((1,), ())

    def valid(self, D):
        u, v = D
        p = self.p
        if not u or u[-1] != 1 or len(u) - 1 > self.g:
            return False
        if v and len(v) >= len(u):
            return False
        return not pdivmod(psub(self.f, pmul(v, v, p), p), u, p)[1]

    def add(self, D1, D2):
        p, f = self.p, self.f
        u1, v1 = D1
        u2, v2 = D2
        d1, e1, e2 = pxgcd(u1, u2, p)
        d, c1, c2 = pxgcd(d1, padd(v1, v2, p), p)
        s1, s2, s3 = pmul(c1, e1, p), pmul(c1, e2, p), c2
        dd = pmul(d, d, p)
        u, rem = pdivmod(pmul(u1, u2, p), dd, p)
        assert not rem
        num = padd(padd(pmul(pmul(s1, u1, p), v2, p), pmul(pmul(s2, u2, p), v1, p), p),
                   pmul(s3, padd(pmul(v1, v2, p), f, p), p), p)
        vq, rem = pdivmod(num, d, p)
        assert not rem
        v = pdivmod(vq, u, p)[1]
        # reduction
        while len(u) - 1 > self.g:
            un, rem = pdivmod(psub(f, pmul(v, v, p), p), u, p)
            assert not rem
            un = pmonic(un, p)
            v = pdivmod(pneg(v, p), un, p)[1]
            u = un
        u = pmonic(u, p)
        return (u, v)

    def neg(self, D):
        u, v = D
        return (u, pneg(v, self.p))

    def is_zero(self, D):
        return D[0] == (1,)

    def mul(self, D, k):
        """[k]D by a signed-digit (NAF) chain: a different addition chain from
        the producer's left-to-right binary ladder and right-to-left ladder."""
        if k < 0:
            return self.mul(self.neg(D), -k)
        naf = []
        while k:
            if k & 1:
                z = 2 - (k % 4)
                naf.append(z)
                k -= z
            else:
                naf.append(0)
            k //= 2
        acc = self.zero
        nD = self.neg(D)
        for z in reversed(naf):
            acc = self.add(acc, acc)
            if z == 1:
                acc = self.add(acc, D)
            elif z == -1:
                acc = self.add(acc, nD)
        return acc

    def point(self, x, y):
        p = self.p
        assert y * y % p == peval(self.f, x, p)
        return (((-x) % p, 1), (y,) if y else ())


# ---------- producer's seeding, re-implemented ----------
def mix64(z):
    z = (z + 0x9E3779B97F4A7C15) & M64
    z = ((z ^ (z >> 30)) * 0xBF58476D1CE4E5B9) & M64
    z = ((z ^ (z >> 27)) * 0x94D049BB133111EB) & M64
    return z ^ (z >> 31)


def random_f(p, genus, curve_seed):
    st = ((curve_seed * 0x9E3779B97F4A7C15) & M64) ^ p ^ (genus << 48)
    f = []
    for _ in range(2 * genus + 1):
        st = (st + 0x9E3779B97F4A7C15) & M64
        f.append(mix64(st) % p)
    f.append(1)
    return f


def sqrt_ts(a, p):
    """Tonelli-Shanks returning the same root as the producer's algorithm
    (p = 3 mod 4: a^((p+1)/4); else z = least non-residue >= 2).  The root
    choice matters only because random_divisor's sign bit selects s or p - s."""
    if a == 0:
        return 0
    if pow(a, (p - 1) // 2, p) != 1:
        return None
    if p % 4 == 3:
        return pow(a, (p + 1) // 4, p)
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


def random_divisor(J, state):
    p = J.p
    D = J.zero
    placed = 0
    while placed < J.g:
        state = (state + 0x9E3779B97F4A7C15) & M64
        h = mix64(state)
        x = h % p
        s = sqrt_ts(peval(J.f, x, p), p)
        if s is None:
            continue
        y = (p - s) % p if (h >> 63) & 1 else s
        D = J.add(D, J.point(x, y))
        placed += 1
    return D, state


def instance_from(J, h, r, target_seed):
    st = target_seed ^ 0x123456789ABCDEF0
    G = None
    for _ in range(64):
        d, st = random_divisor(J, st)
        cand = J.mul(d, h)
        if not J.is_zero(cand) and J.is_zero(J.mul(cand, r)):
            G = cand
            break
    if G is None:
        raise ValueError("no generator of order r")
    st = mix64(st)
    t = 1 + mix64(st) % (r - 1)
    return G, t


# ---------- number theory ----------
def is_prime(n):
    if n < 2:
        return False
    for q in (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37):
        if n % q == 0:
            return n == q
    d, s = n - 1, 0
    while d % 2 == 0:
        d //= 2
        s += 1
    for a in (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37):  # deterministic below 3.3e24
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


def isqrt_floor(n):
    import math
    return math.isqrt(n)


def hasse_weil_bounds(p, g):
    """Integer bounds lo <= #J <= hi from (sqrt p - 1)^{2g} <= #J <= (sqrt p + 1)^{2g},
    computed exactly: (sqrt p +- 1)^{2g} = A +- B sqrt p with integers A, B."""
    # (x + y sqrt p)^n by repeated multiplication in Z[sqrt p]
    def power(sign):
        a, b = 1, 0
        for _ in range(2 * g):
            a, b = a * sign + b * p, a + b * sign
        return a, b  # value a + b sqrt(p)
    import math
    out = []
    for sign in (-1, 1):
        a, b = power(sign)
        # value = a + b*sqrt(p); bracket with integer sqrt of b^2 p
        bs = b * b * p
        root = math.isqrt(bs)  # floor(|b| sqrt p)
        if b >= 0:
            lo_v, hi_v = a + root, a + root + 1
        else:
            lo_v, hi_v = a - root - 1, a - root
        out.append((lo_v, hi_v))
    lo = out[0][0]   # floor-ish lower bound of (sqrt p - 1)^{2g}
    hi = out[1][1]   # ceil-ish upper bound of (sqrt p + 1)^{2g}
    return lo, hi
