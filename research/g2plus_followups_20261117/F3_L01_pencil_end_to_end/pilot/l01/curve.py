from .rng import mix64, M64
from .field import Counter

IDENT = ((1,), ())


class Curve:
    def __init__(self, p, g, f, tables):
        self.p, self.g, self.f = p, g, list(f)
        self.t = tables
        self.ctr = Counter()  # only mul and inv are ticked
        self.ops = {"add": 0, "dbl": 0}

    # ----- polynomial helpers -----
    @staticmethod
    def _trim(a):
        while a and a[-1] == 0:
            a.pop()
        return a

    def k_add(self, a, b):
        p = self.p
        if len(a) < len(b):
            a, b = b, a
        r = list(a)
        for i, y in enumerate(b):
            r[i] = (r[i] + y) % p
        return self._trim(r)

    def k_sub(self, a, b):
        p = self.p
        n = max(len(a), len(b))
        r = [0] * n
        for i, x in enumerate(a):
            r[i] = x
        for i, y in enumerate(b):
            r[i] = (r[i] - y) % p
        return self._trim(r)

    def k_neg(self, a):
        p = self.p
        return [(-x) % p for x in a]

    def k_mul(self, a, b):
        if not a or not b:
            return []
        p = self.p
        self.ctr.mul += len(a) * len(b)
        r = [0] * (len(a) + len(b) - 1)
        for i, x in enumerate(a):
            if x:
                for j, y in enumerate(b):
                    r[i + j] += x * y
        return self._trim([c % p for c in r])

    def k_scale(self, a, s):
        self.ctr.mul += len(a)
        p = self.p
        return self._trim([x * s % p for x in a])

    def k_inv(self, a):
        self.ctr.inv += 1
        return pow(a, self.p - 2, self.p)

    def k_divrem(self, a, b):
        assert b != []
        if len(a) < len(b):
            return [], list(a)
        p = self.p
        monic = b[-1] == 1
        li = 1 if monic else self.k_inv(b[-1])
        db = len(b) - 1
        r = list(a)
        q = [0] * (len(a) - len(b) + 1)
        while r and len(r) >= len(b):
            shift = len(r) - 1 - db
            if monic:
                coef = r[-1]
            else:
                self.ctr.mul += 1
                coef = r[-1] * li % p
            q[shift] = coef
            for j in range(db):
                r[shift + j] = (r[shift + j] - coef * b[j]) % p
            self.ctr.mul += db
            r[shift + db] = 0
            self._trim(r)
        return self._trim(q), r

    def k_rem(self, a, b):
        return self.k_divrem(a, b)[1]

    def k_monic(self, a):
        if not a or a[-1] == 1:
            return a
        return self.k_scale(a, self.k_inv(a[-1]))

    def k_xgcd(self, a, b):
        r0, r1 = a, b
        s0, s1 = [1], []
        t0, t1 = [], [1]
        while r1:
            q, r = self.k_divrem(r0, r1)
            r0, r1 = r1, r
            s = self.k_sub(s0, self.k_mul(q, s1))
            s0, s1 = s1, s
            t = self.k_sub(t0, self.k_mul(q, t1))
            t0, t1 = t1, t
        if not r0 or r0[-1] == 1:
            return r0, s0, t0
        li = self.k_inv(r0[-1])
        return self.k_scale(r0, li), self.k_scale(s0, li), self.k_scale(t0, li)

    # ----- group -----
    def reduce(self, u, v):
        g = self.g
        while len(u) - 1 > g:
            t = self.k_sub(self.f, self.k_mul(v, v))
            q, r = self.k_divrem(t, u)
            assert r == []
            un = self.k_monic(q)
            v = self.k_rem(self.k_neg(v), un)
            u = un
        u = self.k_monic(u)
        return (tuple(u), tuple(v))

    def cantor(self, A, B):
        if A[0] == (1,):
            return B
        if B[0] == (1,):
            return A
        self.ops["dbl" if A == B else "add"] += 1
        u1, v1 = list(A[0]), list(A[1])
        u2, v2 = list(B[0]), list(B[1])
        d1, e1, e2 = self.k_xgcd(u1, u2)
        if len(d1) == 1:
            u = self.k_mul(u1, u2)
            t = self.k_mul(self.k_mul(e1, u1), self.k_sub(v2, v1))
            v = self.k_rem(self.k_add(v1, t), u)
        else:
            d, c1, c2 = self.k_xgcd(d1, self.k_add(v1, v2))
            s1 = self.k_mul(c1, e1)
            s2 = self.k_mul(c1, e2)
            u, ur = self.k_divrem(self.k_mul(u1, u2), self.k_mul(d, d))
            assert ur == []
            t1 = self.k_mul(self.k_mul(s1, u1), v2)
            t2 = self.k_mul(self.k_mul(s2, u2), v1)
            t3 = self.k_mul(c2, self.k_add(self.k_mul(v1, v2), self.f))
            vq, vr = self.k_divrem(self.k_add(self.k_add(t1, t2), t3), d)
            assert vr == []
            v = self.k_rem(vq, u)
        return self.reduce(u, v)

    def negate(self, D):
        return (D[0], tuple(self.k_neg(list(D[1]))))

    def point(self, x, y):
        p = self.p
        assert y * y % p == self.t.fx[x]
        return (((p - x) % p, 1), (y,) if y else ())

    def mul_ltr(self, D, k):
        assert k >= 1
        acc = IDENT
        for i in range(k.bit_length() - 1, -1, -1):
            acc = self.cantor(acc, acc)
            if (k >> i) & 1:
                acc = self.cantor(acc, D)
        return acc

    def random_divisor(self, rng):
        t, p = self.t, self.p
        D = IDENT
        placed = 0
        while placed < self.g:
            h = rng.next()
            x = h % p
            fxv = t.fx[x]
            if t.chi[fxv] == -1:
                continue
            s = t.sq[fxv]
            y = (p - s) % p if (h >> 63) & 1 else s
            D = self.cantor(D, self.point(x, y))
            placed += 1
        return D


def key_of(D):
    u, v = D
    deg = len(u) - 1
    h = 0x243F6A8885A308D3 ^ deg
    for i in range(deg):
        vi = v[i] if i < len(v) else 0
        h = mix64((h ^ ((u[i] << 32) | vi)) & M64)
    return h


def is_identity(D):
    return D[0] == (1,)
