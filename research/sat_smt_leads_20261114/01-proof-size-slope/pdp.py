"""Self-contained F_{2^n} arithmetic, binary curve, chained-S4 PDP CNF generator.

Pilot code for research/sat_smt_leads_20261114/01-proof-size-slope.
"""
import random


class Field:
    def __init__(self, n):
        self.n = n
        self.f = self._find_irreducible(n)
        self.mask = (1 << n) - 1

    @staticmethod
    def _mulmod(a, b, f, n):
        r = 0
        while b:
            if b & 1:
                r ^= a
            b >>= 1
            a <<= 1
            if a >> n:
                a ^= f
        return r

    def _find_irreducible(self, n):
        # n odd prime: f irreducible iff f(0)=f(1)=1 and t^(2^n) == t mod f.
        for low in range(1, 1 << n, 2):
            f = (1 << n) | low
            if bin(f).count("1") % 2 == 0:
                continue
            x = 2
            for _ in range(n):
                x = self._mulmod(x, x, f, n)
            if x == 2:
                return f
        raise RuntimeError

    def mul(self, a, b):
        return self._mulmod(a, b, self.f, self.n)

    def sq(self, a):
        return self._mulmod(a, a, self.f, self.n)

    def inv(self, a):
        assert a
        # a^(2^n - 2)
        r = 1
        e = (1 << self.n) - 2
        x = a
        while e:
            if e & 1:
                r = self.mul(r, x)
            x = self.sq(x)
            e >>= 1
        return r

    def sqrt(self, a):
        for _ in range(self.n - 1):
            a = self.sq(a)
        return a

    def trace(self, a):
        t = 0
        for _ in range(self.n):
            t ^= a
            a = self.sq(a)
        assert t in (0, 1)
        return t

    def halftrace(self, c):
        # n odd; z = sum c^(4^i), i = 0..(n-1)/2 solves z^2+z=c when Tr(c)=0
        z = 0
        x = c
        for _ in range((self.n - 1) // 2 + 1):
            z ^= x
            x = self.sq(self.sq(x))
        return z

    def quad_roots(self, A, B, C):
        """roots of A X^2 + B X + C; returns list, or None if every X is a root"""
        if A == 0:
            if B == 0:
                return None if C == 0 else []
            return [self.mul(C, self.inv(B))]
        if B == 0:
            return [self.sqrt(self.mul(C, self.inv(A)))]
        Bi = self.inv(B)
        c = self.mul(self.mul(A, C), self.sq(Bi))
        if self.trace(c):
            return []
        z = self.halftrace(c)
        assert self.sq(z) ^ z == c
        k = self.mul(B, self.inv(A))
        return [self.mul(k, z), self.mul(k, z ^ 1)]


class Curve:
    """y^2 + xy = x^3 + a x^2 + b over F_{2^n}; points are (x,y) or None."""

    def __init__(self, F, a, b):
        self.F, self.a, self.b = F, a, b

    def lift(self, x):
        F = self.F
        if x == 0:
            return (0, F.sqrt(self.b))
        rhs = F.mul(F.sq(x), x) ^ F.mul(self.a, F.sq(x)) ^ self.b
        c = F.mul(rhs, F.inv(F.sq(x)))
        if F.trace(c):
            return None
        z = F.halftrace(c)
        return (x, F.mul(x, z))

    def on_curve(self, P):
        if P is None:
            return True
        F = self.F
        x, y = P
        return F.sq(y) ^ F.mul(x, y) == F.mul(F.sq(x), x) ^ F.mul(self.a, F.sq(x)) ^ self.b

    def neg(self, P):
        return None if P is None else (P[0], P[0] ^ P[1])

    def add(self, P, Q):
        F = self.F
        if P is None:
            return Q
        if Q is None:
            return P
        x1, y1 = P
        x2, y2 = Q
        if x1 == x2:
            if y1 != y2 or x1 == 0:
                return None
            lam = x1 ^ F.mul(y1, F.inv(x1))
            x3 = F.sq(lam) ^ lam ^ self.a
            y3 = F.sq(x1) ^ F.mul(lam ^ 1, x3)
            return (x3, y3)
        lam = F.mul(y1 ^ y2, F.inv(x1 ^ x2))
        x3 = F.sq(lam) ^ lam ^ x1 ^ x2 ^ self.a
        y3 = F.mul(lam, x1 ^ x3) ^ x3 ^ y1
        return (x3, y3)

    def S3(self, u, v, w):
        F = self.F
        e2 = F.mul(u, v) ^ F.mul(u, w) ^ F.mul(v, w)
        return F.sq(e2) ^ F.mul(F.mul(u, v), w) ^ self.b


def brute_table(curve, l):
    """dict X -> list of (x1,x2) in V^2 with S3(x1,x2,X)=0 (V = low l bits)."""
    F = curve.F
    tab = {}
    for x1 in range(1 << l):
        for x2 in range(1 << l):
            s, p = x1 ^ x2, F.mul(x1, x2)
            roots = F.quad_roots(F.sq(s), p, F.sq(p) ^ curve.b)
            assert roots is not None
            for X in set(roots):
                tab.setdefault(X, []).append((x1, x2))
    return tab


def brute_solutions(curve, l, xr, tab):
    F = curve.F
    sols = set()
    for x3 in range(1 << l):
        roots = F.quad_roots(F.sq(x3 ^ xr), F.mul(xr, x3), F.sq(F.mul(x3, xr)) ^ curve.b)
        if roots is None:
            raise RuntimeError("degenerate second equation")
        for X in set(roots):
            for (x1, x2) in tab.get(X, ()):
                sols.add((x1, x2, x3))
    return sols


class CNF:
    def __init__(self):
        self.nv = 0
        self.clauses = []

    def new(self):
        self.nv += 1
        return self.nv

    def AND(self, x, y):
        z = self.new()
        self.clauses += [[-z, x], [-z, y], [z, -x, -y]]
        return z

    def _xor_direct(self, vs, rhs):
        k = len(vs)
        if k == 0:
            if rhs:
                d = self.new()
                self.clauses += [[d], [-d]]
            return
        for m in range(1 << k):
            # forbid assignments with wrong parity: assignment bits = m
            if bin(m).count("1") % 2 != rhs:
                self.clauses.append([(-v if (m >> i) & 1 else v) for i, v in enumerate(vs)])

    def XOR(self, vs, rhs):
        """XOR of variables vs equals rhs (0/1); cut to width <= 4."""
        # cancel duplicates
        cnt = {}
        for v in vs:
            cnt[v] = cnt.get(v, 0) ^ 1
        vs = [v for v in vs if cnt.get(v)]
        seen = set()
        vs = [v for v in vs if not (v in seen or seen.add(v))]
        while len(vs) > 4:
            t = self.new()
            self._xor_direct(vs[:3] + [t], 0)
            vs = [t] + vs[3:]
        self._xor_direct(vs, rhs)

    def write(self, path):
        with open(path, "w") as f:
            f.write("p cnf %d %d\n" % (self.nv, len(self.clauses)))
            for c in self.clauses:
                f.write(" ".join(map(str, c)) + " 0\n")


def encode(curve, l, xr):
    """Chained S4: S3(x1,x2,X)=0 and S3(x3,xr,X)=0, x_i in V=span(1..t^(l-1))."""
    F = curve.F
    n = F.n
    cnf = CNF()
    a = [cnf.new() for _ in range(l)]
    b = [cnf.new() for _ in range(l)]
    c = [cnf.new() for _ in range(l)]
    X = [cnf.new() for _ in range(n)]
    tpow = [1]
    for _ in range(4 * n):
        tpow.append(F.mul(tpow[-1], 2))
    # s = x1 + x2
    s = []
    for i in range(l):
        v = cnf.new()
        cnf.XOR([v, a[i], b[i]], 0)
        s.append(v)
    # p = x1*x2 as polynomial of degree <= 2l-2 (no reduction needed if 2l-2 < n)
    assert 2 * l - 2 < n
    pk = []
    for k in range(2 * l - 1):
        ms = [cnf.AND(a[i], b[k - i]) for i in range(l) if 0 <= k - i < l]
        v = cnf.new()
        cnf.XOR([v] + ms, 0)
        pk.append(v)
    rows1 = [[] for _ in range(n)]
    rows2 = [[] for _ in range(n)]
    const1 = curve.b
    const2 = curve.b

    def addterm(rows, var, coef):
        for j in range(n):
            if (coef >> j) & 1:
                rows[j].append(var)

    # eq1 = p^2 + (sX)^2 + pX + b
    for k in range(2 * l - 1):
        addterm(rows1, pk[k], tpow[2 * k])
    for i in range(l):
        for k in range(n):
            q = cnf.AND(s[i], X[k])
            addterm(rows1, q, tpow[2 * (i + k)])
    for k in range(2 * l - 1):
        for j in range(n):
            u = cnf.AND(pk[k], X[j])
            addterm(rows1, u, tpow[k + j])
    # eq2 = x3^2 r^2 + (x3 X)^2 + r^2 X^2 + r x3 X + b
    r2 = F.sq(xr)
    for i in range(l):
        addterm(rows2, c[i], F.mul(r2, tpow[2 * i]))
    for k in range(n):
        addterm(rows2, X[k], F.mul(r2, tpow[2 * k]))
    for i in range(l):
        for k in range(n):
            w = cnf.AND(c[i], X[k])
            addterm(rows2, w, tpow[2 * (i + k)] ^ F.mul(xr, tpow[i + k]))
    for j in range(n):
        cnf.XOR(rows1[j], (const1 >> j) & 1)
        cnf.XOR(rows2[j], (const2 >> j) & 1)
    return cnf, {"a": a, "b": b, "c": c, "X": X}


def bits_to_int(model_set, vs):
    return sum(1 << i for i, v in enumerate(vs) if v in model_set)


def random_point(curve, rng):
    while True:
        x = rng.randrange(1, 1 << curve.F.n)
        P = curve.lift(x)
        if P is not None:
            if rng.random() < 0.5:
                P = curve.neg(P)
            return P


def planted_target(curve, l, rng):
    """R = +-P1 +-P2 +-P3 with x(Pi) in V rational; returns (xR, (x1,x2,x3))."""
    base = [x for x in range(1, 1 << l) if curve.lift(x) is not None]
    if not base:
        return None
    for _ in range(500):
        if len(base) >= 3:
            xs = rng.sample(base, 3)  # distinct x: avoids P1 = -P2 (X at infinity), which the chain cannot express
        else:
            xs = [rng.choice(base) for _ in range(3)]
        R = None
        ok = True
        for x in xs:
            P = curve.lift(x)
            if rng.random() < 0.5:
                P = curve.neg(P)
            R = curve.add(R, P)
            ok = ok and R is not None
        if ok and R[0] != 0:
            return R[0], tuple(xs)
    return None


def lifts_to_sum(curve, xs, xr):
    Ps = [curve.lift(x) for x in xs]
    if any(P is None for P in Ps):
        return False
    for m in range(8):
        R = None
        for i, P in enumerate(Ps):
            R = curve.add(R, curve.neg(P) if (m >> i) & 1 else P)
        if R is not None and R[0] == xr:
            return True
    return False


def make_curve(n, seed):
    F = Field(n)
    rng = random.Random(seed)
    b = rng.randrange(1, 1 << n)
    return Curve(F, 1, b), rng
