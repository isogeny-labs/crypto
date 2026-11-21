#!/usr/bin/env python3
"""Independent genus-3 Jacobian arithmetic (Cantor's algorithm, imaginary model
y^2 = f(x), deg f = 7) over F_p, written by the validator from the textbook
algorithm; shares no code with the producer's hyper64.rs/ic_control.rs.
Polynomials are lists of ints, lowest degree first, no trailing zeros."""
import random


def trim(a):
    while a and a[-1] == 0:
        a.pop()
    return a


def padd(a, b, p):
    n = max(len(a), len(b))
    return trim([((a[i] if i < len(a) else 0) + (b[i] if i < len(b) else 0)) % p for i in range(n)])


def pneg(a, p):
    return [(-x) % p for x in a]


def psub(a, b, p):
    return padd(a, pneg(b, p), p)


def pmul(a, b, p):
    if not a or not b:
        return []
    out = [0] * (len(a) + len(b) - 1)
    for i, x in enumerate(a):
        if x:
            for j, y in enumerate(b):
                out[i + j] = (out[i + j] + x * y) % p
    return trim(out)


def pscale(a, c, p):
    return trim([(x * c) % p for x in a])


def pdivmod(a, b, p):
    a = list(a)
    if not b:
        raise ZeroDivisionError
    inv = pow(b[-1], p - 2, p)
    q = [0] * max(0, len(a) - len(b) + 1)
    while len(a) >= len(b) and a:
        c = (a[-1] * inv) % p
        s = len(a) - len(b)
        q[s] = c
        for i, y in enumerate(b):
            a[s + i] = (a[s + i] - c * y) % p
        trim(a)
    return trim(q), a


def pmod(a, b, p):
    return pdivmod(a, b, p)[1]


def monic(a, p):
    if not a:
        return a
    return pscale(a, pow(a[-1], p - 2, p), p)


def xgcd(a, b, p):
    """(d, s, t) with d = s a + t b monic."""
    r0, r1 = list(a), list(b)
    s0, s1 = [1], []
    t0, t1 = [], [1]
    while r1:
        q, r = pdivmod(r0, r1, p)
        r0, r1 = r1, r
        s0, s1 = s1, psub(s0, pmul(q, s1, p), p)
        t0, t1 = t1, psub(t0, pmul(q, t1, p), p)
    if not r0:
        return [], s0, t0
    inv = pow(r0[-1], p - 2, p)
    return pscale(r0, inv, p), pscale(s0, inv, p), pscale(t0, inv, p)


class Jac:
    def __init__(self, p, f, g=3):
        self.p, self.f, self.g = p, trim(list(f)), g
        assert len(self.f) == 2 * g + 2 and self.f[-1] == 1
    ZERO = ((1,), ())

    def valid(self, D):
        u, v = list(D[0]), list(D[1])
        p = self.p
        if not u or u[-1] != 1 or len(u) - 1 > self.g:
            return False
        if len(v) >= len(u):
            return False
        if any(not (0 <= x < p) for x in u + v):
            return False
        return not pmod(psub(self.f, pmul(v, v, p), p), u, p)

    def norm(self, u, v):
        u = monic(u, self.p)
        v = pmod(v, u, self.p)
        return (tuple(u), tuple(v))

    def add(self, D1, D2):
        p, f = self.p, self.f
        u1, v1 = list(D1[0]), list(D1[1])
        u2, v2 = list(D2[0]), list(D2[1])
        d1, e1, e2 = xgcd(u1, u2, p)
        d, c1, c2 = xgcd(d1, padd(v1, v2, p), p)
        s1, s2, s3 = pmul(c1, e1, p), pmul(c1, e2, p), c2
        u = pdivmod(pmul(u1, u2, p), pmul(d, d, p), p)
        assert not u[1]
        u = u[0]
        num = padd(padd(pmul(pmul(s1, u1, p), v2, p), pmul(pmul(s2, u2, p), v1, p), p),
                   pmul(s3, padd(pmul(v1, v2, p), f, p), p), p)
        vq, vr = pdivmod(num, d, p)
        assert not vr
        v = pmod(vq, u, p)
        while len(u) - 1 > self.g:
            un = pdivmod(psub(f, pmul(v, v, p), p), u, p)
            assert not un[1]
            u = monic(un[0], p)
            v = pmod(pneg(v, p), u, p)
        return self.norm(u, v)

    def neg(self, D):
        return self.norm(list(D[0]), pneg(list(D[1]), self.p))

    def mul(self, D, k):
        R = self.ZERO
        Q = D
        while k > 0:
            if k & 1:
                R = self.add(R, Q)
            k >>= 1
            if k:
                Q = self.add(Q, Q)
        return R

    def sqrt(self, a):
        """Tonelli-Shanks; None if a is a non-residue."""
        p = self.p
        a %= p
        if a == 0:
            return 0
        if pow(a, (p - 1) // 2, p) != 1:
            return None
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

    def random_divisor(self, rnd):
        D = self.ZERO
        for _ in range(self.g):
            while True:
                x = rnd.randrange(self.p)
                fx = 0
                for c in reversed(self.f):
                    fx = (fx * x + c) % self.p
                y = self.sqrt(fx)
                if y is not None:
                    break
            if rnd.random() < 0.5:
                y = (-y) % self.p
            D = self.add(D, (tuple([(-x) % self.p, 1]), tuple(trim([y]))))
        return D


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
    for a in (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37):
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


M64 = (1 << 64) - 1


def mix64(z):
    z = (z + 0x9E3779B97F4A7C15) & M64
    z = ((z ^ (z >> 30)) * 0xBF58476D1CE4E5B9) & M64
    z = ((z ^ (z >> 27)) * 0x94D049BB133111EB) & M64
    return z ^ (z >> 31)


def random_curve_f(p, seed):
    """The documented curve generator (ic_control.rs random_curve), re-implemented."""
    st = ((seed * 0x9E3779B97F4A7C15) & M64) ^ p
    f = []
    for _ in range(7):
        st = (st + 0x9E3779B97F4A7C15) & M64
        f.append(mix64(st) % p)
    f.append(1)
    return f


def squarefree(f, p):
    df = trim([(i * c) % p for i, c in enumerate(f)][1:])
    d, _, _ = xgcd(f, df, p)
    return len(d) == 1


def div_from_packed(a):
    """[deg, u0, u1, u2, v0, v1, v2] -> (u, v) tuples."""
    deg = a[0]
    u = list(a[1:1 + deg]) + [1]
    v = trim(list(a[4:4 + deg]))
    return (tuple(u), tuple(v))


def self_test(seed=1):
    rnd = random.Random(seed)
    p = 131
    f = random_curve_f(p, 1)
    J = Jac(p, f)
    for _ in range(20):
        A, B, C = (J.random_divisor(rnd) for _ in range(3))
        assert J.valid(A) and J.valid(B) and J.valid(C)
        assert J.add(A, B) == J.add(B, A)
        assert J.add(J.add(A, B), C) == J.add(A, J.add(B, C))
        assert J.add(A, J.neg(A)) == J.ZERO
        assert J.add(A, J.ZERO) == A
        assert J.mul(A, 5) == J.add(J.add(J.add(J.add(A, A), A), A), A)
    return True


if __name__ == "__main__":
    print("self-test", self_test())
