"""Minimal F_q = F_p[z]/(z^n - c) arithmetic and the double-odd curve y^2 = x(x^2+ax+b)."""
import random


class Fq:
    def __init__(self, p, n, c):
        self.p, self.n, self.c = p, n, c
        self.q = p ** n

    def norm(self, a):
        return tuple(x % self.p for x in a)

    def zero(self):
        return (0,) * self.n

    def one(self):
        return (1,) + (0,) * (self.n - 1)

    def const(self, k):
        return (k % self.p,) + (0,) * (self.n - 1)

    def add(self, a, b):
        return tuple((x + y) % self.p for x, y in zip(a, b))

    def sub(self, a, b):
        return tuple((x - y) % self.p for x, y in zip(a, b))

    def neg(self, a):
        return tuple((-x) % self.p for x in a)

    def mul(self, a, b):
        n, p, c = self.n, self.p, self.c
        r = [0] * (2 * n - 1)
        for i, x in enumerate(a):
            if x:
                for j, y in enumerate(b):
                    r[i + j] += x * y
        out = [0] * n
        for k, v in enumerate(r):
            if k < n:
                out[k] += v
            else:
                out[k - n] += c * v
        return tuple(v % p for v in out)

    def pow(self, a, e):
        r = self.one()
        while e:
            if e & 1:
                r = self.mul(r, a)
            a = self.mul(a, a)
            e >>= 1
        return r

    def inv(self, a):
        return self.pow(a, self.q - 2)

    def is_zero(self, a):
        return all(x == 0 for x in a)

    def is_square(self, a):
        return self.is_zero(a) or self.pow(a, (self.q - 1) // 2) == self.one()

    def rand(self, rng):
        return tuple(rng.randrange(self.p) for _ in range(self.n))

    def sqrt(self, a, rng):
        if self.is_zero(a):
            return a
        assert self.is_square(a)
        q = self.q
        s, t = 0, q - 1
        while t % 2 == 0:
            s += 1
            t //= 2
        while True:
            nz = self.rand(rng)
            if not self.is_square(nz):
                break
        m, cc = s, self.pow(nz, t)
        tt, r = self.pow(a, t), self.pow(a, (t + 1) // 2)
        while tt != self.one():
            i, t2 = 0, tt
            while t2 != self.one():
                t2 = self.mul(t2, t2)
                i += 1
            bb = cc
            for _ in range(m - i - 1):
                bb = self.mul(bb, bb)
            m, cc = i, self.mul(bb, bb)
            tt, r = self.mul(tt, cc), self.mul(r, bb)
        assert self.mul(r, r) == self.norm(a)
        return r

    def frob(self, a):
        return self.pow(a, self.p)

    def in_Fp(self, a):
        return all(x == 0 for x in a[1:])


class Curve:
    """y^2 = x^3 + a x^2 + b x ; points as (x, y) tuples or None for O."""

    def __init__(self, F, a, b):
        self.F, self.a, self.b = F, a, b

    def on(self, P):
        if P is None:
            return True
        F = self.F
        x, y = P
        rhs = F.mul(x, F.add(F.add(F.mul(x, x), F.mul(self.a, x)), self.b))
        return F.mul(y, y) == rhs

    def add(self, P, Q):
        F = self.F
        if P is None:
            return Q
        if Q is None:
            return P
        x1, y1 = P
        x2, y2 = Q
        if x1 == x2:
            if F.add(y1, y2) == F.zero():
                return None
            num = F.add(F.add(F.mul(F.const(3), F.mul(x1, x1)), F.mul(F.const(2), F.mul(self.a, x1))), self.b)
            lam = F.mul(num, F.inv(F.mul(F.const(2), y1)))
        else:
            lam = F.mul(F.sub(y2, y1), F.inv(F.sub(x2, x1)))
        x3 = F.sub(F.sub(F.sub(F.mul(lam, lam), self.a), x1), x2)
        y3 = F.sub(F.mul(lam, F.sub(x1, x3)), y1)
        return (x3, y3)

    def neg(self, P):
        return None if P is None else (P[0], self.F.neg(P[1]))

    def rand_point(self, rng):
        F = self.F
        while True:
            x = F.rand(rng)
            rhs = F.mul(x, F.add(F.add(F.mul(x, x), F.mul(self.a, x)), self.b))
            if F.is_square(rhs) and not F.is_zero(rhs):
                return (x, F.sqrt(rhs, rng))

    def u(self, P):
        """Pornin's u = x/y; u(O) = u(N) = 0, u = None (infinity) at the other 2-torsion."""
        F = self.F
        if P is None or F.is_zero(P[0]):
            return F.zero()
        if F.is_zero(P[1]):
            return None
        return F.mul(P[0], F.inv(P[1]))

    def points_with_u(self, u, rng):
        """Points P with u(P) = u: x solves u^2 x^2 + (a u^2 - 1) x + b u^2 = 0, y = x/u."""
        F = self.F
        u2 = F.mul(u, u)
        A, B, C = u2, F.sub(F.mul(self.a, u2), F.one()), F.mul(self.b, u2)
        disc = F.sub(F.mul(B, B), F.mul(F.const(4), F.mul(A, C)))
        if not F.is_square(disc):
            return []
        r = F.sqrt(disc, rng)
        inv2A = F.inv(F.mul(F.const(2), A))
        out = []
        for s in (r, F.neg(r)):
            x = F.mul(F.sub(s, B), inv2A)
            P = (x, F.mul(x, F.inv(u)))
            assert self.on(P)
            out.append(P)
        return out
