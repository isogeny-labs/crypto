#!/usr/bin/env python3
"""Self-contained F_{2^n} arithmetic, binary Koblitz curve, S3-chain SAT encoding
of the m=3 point decomposition problem, and MITM ground truth. Pilot code."""
import random
from itertools import combinations

MODS = {7: (1 << 7) | 0b11, 11: (1 << 11) | 0b101, 13: (1 << 13) | 0b11011, 17: (1 << 17) | 0b1001, 19: (1 << 19) | 0b100111}


class Field:
    def __init__(self, n):
        self.n = n
        self.f = MODS[n]
        self.q = 1 << n
        # irreducibility (n prime): x^(2^n) == x mod f, and no root in F2
        x = 2
        for _ in range(n):
            x = self._slowmul(x, x)
        assert x == 2, "modulus not irreducible"
        assert self.f & 1 and bin(self.f).count("1") % 2 == 1
        # 2^n - 1 is a Mersenne prime for n in {13,17,19}: z is primitive
        self.exp = [0] * (2 * self.q)
        self.log = [0] * self.q
        v = 1
        for i in range(self.q - 1):
            self.exp[i] = v
            self.log[v] = i
            v <<= 1
            if v >> n:
                v ^= self.f
        assert v == 1 and len(set(self.exp[: self.q - 1])) == self.q - 1, 'z not primitive'
        for i in range(self.q - 1, 2 * self.q):
            self.exp[i] = self.exp[i - (self.q - 1)]

    def _slowmul(self, a, b):
        r = 0
        while b:
            if b & 1:
                r ^= a
            b >>= 1
            a <<= 1
            if a >> self.n:
                a ^= self.f
        return r

    def mul(self, a, b):
        if a == 0 or b == 0:
            return 0
        return self.exp[self.log[a] + self.log[b]]

    def inv(self, a):
        return self.exp[(self.q - 1) - self.log[a]]

    def sq(self, a):
        return self.mul(a, a)

    def sqrt(self, a):
        for _ in range(self.n - 1):
            a = self.mul(a, a)
        return a

    def trace(self, a):
        t = 0
        for _ in range(self.n):
            t ^= a
            a = self.mul(a, a)
        return t

    def halftrace(self, a):
        h = 0
        for i in range((self.n - 1) // 2 + 1):
            h ^= a
            a = self.sq(self.sq(a))
        return h


class Curve:
    """y^2 + xy = x^3 + a x^2 + b ; point = (x,y) or None for infinity."""

    def __init__(self, F, a=1, b=1):
        self.F, self.a, self.b = F, a, b

    def on(self, P):
        if P is None:
            return True
        F = self.F
        x, y = P
        return F.sq(y) ^ F.mul(x, y) == F.mul(F.sq(x), x) ^ F.mul(self.a, F.sq(x)) ^ self.b

    def lift(self, x):
        """one rational point with this x, or None."""
        F = self.F
        if x == 0:
            return (0, F.sqrt(self.b))
        c = x ^ self.a ^ F.mul(self.b, F.inv(F.sq(x)))
        if F.trace(c) != 0:
            return None
        w = F.halftrace(c)
        P = (x, F.mul(x, w))
        assert self.on(P)
        return P

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

    def random_point(self, rng):
        while True:
            x = rng.randrange(1, self.F.q)
            P = self.lift(x)
            if P is not None:
                return P if rng.random() < 0.5 else self.neg(P)


# ---------------------------------------------------------------- ground truth
class Truth:
    """MITM over curve points. Chain semantics: exists t = x(P1 +- P2) (finite)
    with t = x(R +- P3) (finite), xi in V = [0, 2^l)."""

    def __init__(self, E, l):
        self.E, self.l = E, l
        self.pts = {}
        for x in range(1 << l):
            P = E.lift(x)
            if P is not None:
                self.pts[x] = P
        xs = sorted(self.pts)
        self.pair = {}  # t -> set of unordered (x1<=x2)
        for i, x1 in enumerate(xs):
            for x2 in xs[i:]:
                P, Q = self.pts[x1], self.pts[x2]
                for T in (E.add(P, Q), E.add(P, E.neg(Q))):
                    if T is not None:
                        self.pair.setdefault(T[0], set()).add((x1, x2))

    def query(self, R):
        E = self.E
        k3 = 0
        nsol = 0
        pairs = set()
        for x3, P3 in self.pts.items():
            ts = set()
            for T in (E.add(R, P3), E.add(R, E.neg(P3))):
                if T is not None:
                    ts.add(T[0])
            hit = set()
            for t in ts:
                hit |= self.pair.get(t, set())
            if hit:
                k3 += 1
                pairs |= hit
                nsol += sum(1 if a == b else 2 for a, b in hit)
        k2 = sum(1 if a == b else 2 for a, b in pairs)
        return dict(nsol=nsol, k3=k3, k2=k2)


# ---------------------------------------------------------------- SAT encoding
class Enc:
    """bit expressions: dict-free sets of monomials; monomial = tuple of var ids
    (sorted), () is the constant 1. Symbolic element = list of n sets."""

    def __init__(self, F, l, xR, b=1, chunk=4):
        self.F, self.l, self.n = F, l, F.n
        self.nv = 0
        self.clauses = []
        self.andv = {}
        self.chunk = chunk
        n = self.n
        self.zp = [F.exp[k] if k < F.q - 1 else None for k in range(2 * n)]  # z^k reduced
        self.c = [[self.new() for _ in range(l)] for _ in range(3)]
        self.t = [self.new() for _ in range(n)]
        X = [self.lin(self.c[i]) for i in range(3)]
        T = self.lin(self.t)
        # p = x1*x2 with fresh variables
        pe = self.prod(X[0], X[1])
        pv = []
        P = []
        for k in range(n):
            if not pe[k]:
                P.append(set())
                continue
            v = self.new()
            pv.append(v)
            P.append({(v,)})
            self.xor_constraint(pe[k] ^ {(v,)})
        S = [X[0][k] ^ X[1][k] for k in range(n)]
        # eq1: p^2 + (s t)^2 + p t + b = 0
        e1 = self.xadd(self.xadd(self.sqr(P), self.sqr(self.prod(S, T))), self.xadd(self.prod(P, T), self.const(b)))
        # eq2: (x3 xR)^2 + ((x3+xR) t)^2 + (x3 xR) t + b = 0
        A = self.cmul(X[2], xR)
        Bv = self.xadd(X[2], self.const(xR))
        e2 = self.xadd(self.xadd(self.sqr(A), self.sqr(self.prod(Bv, T))), self.xadd(self.prod(A, T), self.const(b)))
        for e in (e1, e2):
            for k in range(n):
                self.xor_constraint(e[k])
        self.base_vars = self.nv

    def new(self):
        self.nv += 1
        return self.nv

    def lin(self, vs):
        return [{(vs[k],)} if k < len(vs) else set() for k in range(self.n)]

    def const(self, c):
        return [{()} if (c >> k) & 1 else set() for k in range(self.n)]

    def xadd(self, A, B):
        return [A[k] ^ B[k] for k in range(self.n)]

    def cmul(self, A, c):
        out = [set() for _ in range(self.n)]
        for j in range(self.n):
            if A[j]:
                w = self.F.mul(c, 1 << j)
                for k in range(self.n):
                    if (w >> k) & 1:
                        out[k] ^= A[j]
        return out

    def sqr(self, A):
        out = [set() for _ in range(self.n)]
        for j in range(self.n):
            if A[j]:
                w = self.F.sq(1 << j)
                for k in range(self.n):
                    if (w >> k) & 1:
                        out[k] ^= A[j]
        return out

    def prod(self, A, B):
        out = [set() for _ in range(self.n)]
        for i in range(self.n):
            if not A[i]:
                continue
            for j in range(self.n):
                if not B[j]:
                    continue
                w = self.F.mul(1 << i, 1 << j)
                ms = set()
                for ma in A[i]:
                    for mb in B[j]:
                        m = tuple(sorted(set(ma) | set(mb)))
                        assert len(m) <= 2
                        ms ^= {m}
                for k in range(self.n):
                    if (w >> k) & 1:
                        out[k] ^= ms
        return out

    def mono_var(self, m):
        if len(m) == 1:
            return m[0]
        v = self.andv.get(m)
        if v is None:
            v = self.new()
            self.andv[m] = v
            a, b = m
            self.clauses += [[-v, a], [-v, b], [v, -a, -b]]
        return v

    def xor_constraint(self, expr):
        """XOR of monomials == 0."""
        rhs = 0
        lits = []
        for m in sorted(expr):
            if m == ():
                rhs ^= 1
            else:
                lits.append(self.mono_var(m))
        self.add_xor(lits, rhs)

    def add_xor(self, lits, rhs):
        """XOR(lits) == rhs, chunked."""
        lits = list(lits)
        if not lits:
            if rhs:
                self.clauses.append([])
            return
        while len(lits) > self.chunk:
            head, lits = lits[: self.chunk - 1], lits[self.chunk - 1:]
            v = self.new()
            self._xor_cnf(head + [v], 0)
            lits = [v] + lits
        self._xor_cnf(lits, rhs)

    def _xor_cnf(self, lits, rhs):
        k = len(lits)
        for mask in range(1 << k):
            # forbid assignments with wrong parity: assignment bits = mask
            if bin(mask).count("1") % 2 != rhs:
                self.clauses.append([(-lits[i] if (mask >> i) & 1 else lits[i]) for i in range(k)])

    def streamline(self, k, rng):
        """return extra clauses for k random parities over the 3l coordinate bits."""
        saved, savednv = self.clauses, self.nv
        self.clauses = []
        cb = [v for row in self.c for v in row]
        for _ in range(k):
            while True:
                sel = [v for v in cb if rng.random() < 0.5]
                if sel:
                    break
            self.add_xor(sel, rng.randrange(2))
        extra = self.clauses
        self.clauses, self.nv = saved, savednv  # aux vars above base are per-call
        return extra

    def xs_from_model(self, model):
        s = set(l for l in model if l > 0)
        return [sum(1 << j for j in range(self.l) if self.c[i][j] in s) for i in range(3)]


def lift_ok(E, xs, R):
    Ps = [E.lift(x) for x in xs]
    if any(P is None for P in Ps):
        return False
    for s in range(8):
        Q = None
        for i in range(3):
            Q = E.add(Q, E.neg(Ps[i]) if (s >> i) & 1 else Ps[i])
        if Q is not None and Q[0] == R[0]:
            return True
    return False


def S3(F, a, b, c, bb=1):
    u = F.mul(a, b) ^ F.mul(a, c) ^ F.mul(b, c)
    return F.sq(u) ^ F.mul(F.mul(a, b), c) ^ bb
