"""Self-contained F_{2^n} + binary curve + m=3 PDP CNF generator (pilot)."""
import random, itertools


def _mulmod(a, b, n, mod):
    r = 0
    while b:
        if b & 1:
            r ^= a
        b >>= 1
        a <<= 1
        if a >> n:
            a ^= mod
    return r


def find_irred(n):
    def ok(mod):
        x = 2
        for _ in range(n):
            x = _mulmod(x, x, n, mod)
        return x == 2
    for k in range(1, n):
        mod = (1 << n) | (1 << k) | 1
        if ok(mod):
            return mod
    for k3, k2, k1 in itertools.combinations(range(n - 1, 0, -1), 3):
        mod = (1 << n) | (1 << k3) | (1 << k2) | (1 << k1) | 1
        if ok(mod):
            return mod
    raise ValueError


class GF:
    def __init__(self, n):
        assert n % 2 == 1
        self.n = n
        self.mod = find_irred(n)

    def mul(self, a, b):
        return _mulmod(a, b, self.n, self.mod)

    def sq(self, a):
        return _mulmod(a, a, self.n, self.mod)

    def inv(self, a):
        assert a
        r, e, b = 1, (1 << self.n) - 2, a
        while e:
            if e & 1:
                r = self.mul(r, b)
            b = self.sq(b)
            e >>= 1
        return r

    def tr(self, a):
        t = 0
        for _ in range(self.n):
            t ^= a
            a = self.sq(a)
        return t & 1 if t in (0, 1) else None

    def halftrace(self, c):
        h = 0
        for i in range((self.n + 1) // 2):
            h ^= c
            c = self.sq(self.sq(c))
        return h


class Curve:
    """y^2 + xy = x^3 + a x^2 + b; points are (x,y) or None."""

    def __init__(self, F, a, b):
        self.F, self.a, self.b = F, a, b

    def lift(self, x):
        F = self.F
        if x == 0:
            return None
        rhs = F.mul(F.sq(x), x) ^ F.mul(self.a, F.sq(x)) ^ self.b
        c = F.mul(rhs, F.inv(F.sq(x)))
        if F.tr(c) != 0:
            return None
        z = F.halftrace(c)
        assert F.sq(z) ^ z == c
        return (x, F.mul(z, x))

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

    def on(self, P):
        F = self.F
        x, y = P
        return F.sq(y) ^ F.mul(x, y) == F.mul(F.sq(x), x) ^ F.mul(self.a, F.sq(x)) ^ self.b


def span(basis):
    s = [0]
    for v in basis:
        s += [u ^ v for u in s]
    return s


def rank(vs):
    vs = list(vs)
    r = 0
    piv = []
    for v in vs:
        for p in piv:
            v = min(v, v ^ p)
        if v:
            piv.append(v)
            piv.sort(reverse=True)
    return len(piv)


class Setup:
    def __init__(self, n, l, seed, a=0, b=1):
        self.n, self.l, self.seed = n, l, seed
        self.F = GF(n)
        self.E = Curve(self.F, a, b)
        rng = random.Random(seed)
        while True:
            V = [rng.randrange(1, 1 << n) for _ in range(l)]
            if rank(V) == l:
                break
        self.V = V
        self.Vset = set(span(V))
        self.base = []  # signed points
        for x in sorted(self.Vset):
            P = self.E.lift(x)
            if P is not None:
                self.base += [P, self.E.neg(P)]
        E = self.E
        self.sum2x = set()
        for i, P in enumerate(self.base):
            for Q in self.base[i:]:
                S = E.add(P, Q)
                if S is not None:
                    self.sum2x.add(S[0])

    def decomposable(self, R):
        """real-point 2+1 MITM ground truth: exists P1,P2,P3 in signed base with P1+P2+P3=R."""
        E = self.E
        for P in self.base:
            D = E.add(R, P)  # base is closed under negation
            if D is not None and D[0] in self.sum2x:
                return True
        return False

    def random_point(self, rng):
        while True:
            x = rng.randrange(1, 1 << self.n)
            P = self.E.lift(x)
            if P is not None:
                return P if rng.random() < 0.5 else self.E.neg(P)

    def planted(self, rng):
        E = self.E
        P = [rng.choice(self.base) for _ in range(3)]
        return E.add(E.add(P[0], P[1]), P[2])


# ---------------- CNF encoder ----------------
class Enc:
    def __init__(self, F):
        self.F = F
        self.nv = 0
        self.clauses = []
        self.andc = {}
        self.and_inputs = {}
        self.nxor = 0
        self.unsat_const = False

    def new(self):
        self.nv += 1
        return self.nv

    def AND(self, u, v):
        if u == v:
            return u
        k = (u, v) if u < v else (v, u)
        w = self.andc.get(k)
        if w is None:
            w = self.new()
            self.andc[k] = w
            self.and_inputs[w] = k
            self.clauses += [[-w, u], [-w, v], [w, -u, -v]]
        return w

    @staticmethod
    def add(*els):
        d, c0 = {}, 0
        for (dd, cc) in els:
            c0 ^= cc
            for v, c in dd.items():
                nc = d.get(v, 0) ^ c
                if nc:
                    d[v] = nc
                else:
                    d.pop(v, None)
        return (d, c0)

    def mul(self, A, B):
        F = self.F
        d = {}
        (da, ca), (db, cb) = A, B

        def acc(v, c):
            if c:
                nc = d.get(v, 0) ^ c
                if nc:
                    d[v] = nc
                else:
                    d.pop(v, None)
        for u, cu in da.items():
            for v, cv in db.items():
                acc(self.AND(u, v), F.mul(cu, cv))
            if cb:
                acc(u, F.mul(cu, cb))
        if ca:
            for v, cv in db.items():
                acc(v, F.mul(cv, ca))
        return (d, F.mul(ca, cb))

    def sq(self, A):
        F = self.F
        d = {}
        for v, c in A[0].items():
            d[v] = F.sq(c)
        return (d, F.sq(A[1]))

    def xor(self, vs, rhs):
        self.nxor += 1
        vs = list(vs)
        if not vs:
            if rhs:
                self.clauses.append([])
                self.unsat_const = True
            return
        i = 0
        while len(vs) - i > 3:
            a, b = vs[i], vs[i + 1]
            i += 2
            z = self.new()  # z = a ^ b
            self.clauses += [[-z, a, b], [-z, -a, -b], [z, -a, b], [z, a, -b]]
            vs.append(z)
        vs = vs[i:]
        k = len(vs)
        for signs in itertools.product((0, 1), repeat=k):
            if (sum(signs) & 1) != (rhs & 1):
                # forbid assignment 'signs'
                self.clauses.append([-v if s else v for v, s in zip(vs, signs)])

    def eqzero(self, A):
        d, c0 = A
        for i in range(self.F.n):
            self.xor([v for v, c in d.items() if (c >> i) & 1], (c0 >> i) & 1)

    def materialize(self, A):
        n = self.F.n
        if len(A[0]) <= n:
            return A
        z = [self.new() for _ in range(n)]
        Z = ({z[i]: 1 << i for i in range(n)}, 0)
        self.eqzero(self.add(A, Z))
        return Z


def build(S, R_x=None):
    """m=3 PDP CNF. R_x None -> parametric (target bits are variables r)."""
    F, n, l = S.F, S.n, S.l
    e = Enc(F)
    c = [[e.new() for _ in range(l)] for _ in range(3)]
    X = [({c[i][j]: S.V[j] for j in range(l)}, 0) for i in range(3)]
    tv = [e.new() for _ in range(n)]
    T = ({tv[i]: 1 << i for i in range(n)}, 0)
    if R_x is None:
        r = [e.new() for _ in range(n)]
        XR = ({r[i]: 1 << i for i in range(n)}, 0)
    else:
        r = []
        XR = ({}, R_x)
    B = ({}, S.E.b)
    # S3(x1,x2,t) = p^2 + (s t)^2 + p t + b
    p = e.materialize(e.mul(X[0], X[1]))
    s = e.add(X[0], X[1])
    st = e.mul(s, T)
    e.eqzero(e.add(e.sq(p), e.sq(st), e.mul(p, T), B))
    # S3(x3,xR,t) = (xR (x3+t) + q)^2 + xR q + b,  q = x3 t
    q = e.materialize(e.mul(X[2], T))
    w = e.add(X[2], T)
    rw = e.mul(XR, w)
    e.eqzero(e.add(e.sq(e.add(rw, q)), e.mul(XR, q), B))
    rset = set(r)
    cone = set(r) | {w_ for w_, (u, v) in e.and_inputs.items() if u in rset or v in rset}
    return dict(nv=e.nv, clauses=e.clauses, c=c, t=tv, r=r, cone=cone,
                nand=len(e.andc), nxor=e.nxor)


def assumptions(inst, R_x):
    return [v if (R_x >> i) & 1 else -v for i, v in enumerate(inst['r'])]


def lift_model(S, inst, model, R):
    """check that the model's x1,x2,x3 lift to rational points with +-P1+-P2+-P3 = +-R."""
    E = S.E
    ms = set(l_ for l_ in model if l_ > 0)
    xs = []
    for i in range(3):
        x = 0
        for j in range(S.l):
            if inst['c'][i][j] in ms:
                x ^= S.V[j]
        xs.append(x)
    P = [E.lift(x) for x in xs]
    if any(p is None for p in P):
        return False, xs
    for s1 in (0, 1):
        for s2 in (0, 1):
            Q = E.add(E.add(P[0], E.neg(P[1]) if s1 else P[1]), E.neg(P[2]) if s2 else P[2])
            if Q is not None and Q[0] == R[0]:
                return True, xs
    return False, xs


def _lex_le(e, a, b):
    """a <= b lexicographically (index 0 most significant)."""
    eq = None
    for j in range(len(a)):
        pre = [] if eq is None else [-eq]
        e.clauses.append(pre + [-a[j], b[j]])
        if j + 1 < len(a):
            nq = e.new()
            e.clauses.append(pre + [a[j], b[j], nq])
            e.clauses.append(pre + [-a[j], -b[j], nq])
            eq = nq


def build2(S, R_x=None, symbreak=True):
    """m=3 PDP CNF, t eliminated by the resultant of the two quadratics in t
    (S4 = (A C' + A' C)^2 + (A B' + A' B)(B C' + B' C)); every auxiliary is
    forward-determined by unit propagation once c is assigned."""
    F, n, l = S.F, S.n, S.l
    e = Enc(F)
    c = [[e.new() for _ in range(l)] for _ in range(3)]
    X = [({c[i][j]: S.V[j] for j in range(l)}, 0) for i in range(3)]
    if R_x is None:
        r = [e.new() for _ in range(n)]
        XR = ({r[i]: 1 << i for i in range(n)}, 0)
    else:
        r = []
        XR = ({}, R_x)
    Bc = ({}, S.E.b)
    A = e.sq(e.add(X[0], X[1]))
    Bq = e.materialize(e.mul(X[0], X[1]))
    C = e.add(e.sq(Bq), Bc)
    A2 = e.sq(e.add(X[2], XR))
    B2 = e.materialize(e.mul(X[2], XR))
    C2 = e.add(e.sq(B2), Bc)
    U = e.materialize(e.add(e.mul(A, C2), e.mul(A2, C)))
    W = e.materialize(e.add(e.mul(A, B2), e.mul(A2, Bq)))
    Z = e.materialize(e.add(e.mul(Bq, C2), e.mul(B2, C)))
    e.eqzero(e.add(e.sq(U), e.mul(W, Z)))
    if symbreak:
        _lex_le(e, c[0], c[1])
        _lex_le(e, c[1], c[2])
    rset = set(r)
    cone = set(r) | {w_ for w_, (u, v) in e.and_inputs.items() if u in rset or v in rset}
    return dict(nv=e.nv, clauses=e.clauses, c=c, r=r, cone=cone,
                nand=len(e.andc), nxor=e.nxor)
