"""Boolean tensor network (constraint network) for ECDL relation finding.

Prototype of the Bochum/MPI construction (Akshima et al., v0.2): encode
"A1 + A2 = R with A_i in hashed factor base F_i" on a Koblitz curve
y^2 + xy = x^3 + 1 over GF(2^n) as a network of local Boolean predicates,
count satisfying assignments with a bucket-elimination contractor (a stand-in
for the claimed subexponential contraction algorithm), and extract a witness
bit by bit (Lemma 1).  Toy sizes only; no speedup is claimed.
"""
import numpy as np
from math import comb

MAXW = 0
LET = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"


class Field:
    def __init__(self, n, poly):
        self.n, self.f = n, poly
        self.taps = [i for i in range(n) if poly >> i & 1]  # low part of f

    def mul(self, a, b):
        r = 0
        while b:
            if b & 1:
                r ^= a
            b >>= 1
            a <<= 1
            if a >> self.n & 1:
                a ^= self.f
        return r

    def sq(self, a):
        return self.mul(a, a)

    def inv(self, a):
        r, e = 1, (1 << self.n) - 2
        while e:
            if e & 1:
                r = self.mul(r, a)
            a = self.mul(a, a)
            e >>= 1
        return r


class Curve:
    """y^2 + xy = x^3 + 1 over GF(2^n); points are (x, y), None is O."""

    def __init__(self, F):
        self.F = F

    def on(self, P):
        F = self.F
        x, y = P
        return F.sq(y) ^ F.mul(x, y) == F.mul(F.sq(x), x) ^ 1

    def add(self, P, Q):
        F = self.F
        if P is None: return Q
        if Q is None: return P
        if P[0] == Q[0]:
            if P[1] != Q[1] or P[0] == 0: return None
            lam = P[0] ^ F.mul(P[1], F.inv(P[0]))
            x3 = F.sq(lam) ^ lam
            return (x3, F.sq(P[0]) ^ F.mul(lam ^ 1, x3))
        lam = F.mul(P[1] ^ Q[1], F.inv(P[0] ^ Q[0]))
        x3 = F.sq(lam) ^ lam ^ P[0] ^ Q[0]
        return (x3, F.mul(lam, P[0] ^ x3) ^ x3 ^ P[1])

    def mulk(self, k, P):
        R = None
        while k:
            if k & 1: R = self.add(R, P)
            P = self.add(P, P)
            k >>= 1
        return R


class Net:
    """Variables are bit ids; each constraint is (scope, ndarray of 0/1)."""

    def __init__(self):
        self.nv = 0
        self.cons = []

    def word(self, n):
        w = list(range(self.nv, self.nv + n))
        self.nv += n
        return w

    def add(self, scope, fn):
        k = len(scope)
        t = np.zeros((2,) * k, dtype=np.int64)
        for idx in np.ndindex(*t.shape):
            t[idx] = 1 if fn(*idx) else 0
        self.cons.append((tuple(scope), t))

    def fix(self, w, value):
        for i, v in enumerate(w):
            self.add([v], lambda b, i=i: b == (value >> i & 1))

    def xor(self, out, ins):  # out = xor(ins)
        self.add([out] + list(ins), lambda o, *a: o == sum(a) % 2)

    def xorw(self, out, *ws):
        for i in range(len(out)):
            self.xor(out[i], [w[i] for w in ws])

    def mul(self, F, a, b):
        """c = a*b in GF(2^n), Horner from the top bit of b (parsimonious)."""
        n = F.n
        acc = None
        for j in range(n - 1, -1, -1):
            new = self.word(n)
            for i in range(n):
                terms = []  # new[i] = acc[i-1] ^ tap_i*acc[n-1] ^ b_j*a_i
                sc = [new[i], b[j], a[i]]
                parts = []
                if acc is not None:
                    if i >= 1: sc.append(acc[i - 1]); parts.append("s")
                    if i in F.taps: sc.append(acc[n - 1]); parts.append("t")
                else:
                    parts = []
                # dedupe scope (acc[i-1] and acc[n-1] coincide when i == n)
                def fn(*v, parts=parts):
                    o, bj, ai = v[0], v[1], v[2]
                    s = bj & ai
                    for x in v[3:]: s ^= x
                    return o == s
                self.add(sc, fn)
            acc = new
        return acc

    def sq(self, F, a):
        n = F.n
        cols = [F.sq(1 << j) for j in range(n)]
        c = self.word(n)
        for i in range(n):
            ins = [a[j] for j in range(n) if cols[j] >> i & 1]
            self.xor(c[i], ins)
        return c

    def stats(self):
        """Edges/degree after replacing each variable by an equality tree
        of degree-3 vertices (BTN form): k uses -> 2k-3 edges (k>=2)."""
        uses = {}
        for sc, _ in self.cons:
            for v in sc: uses[v] = uses.get(v, 0) + 1
        edges = sum(1 if k == 1 else (k if k == 2 else 2 * k - 3) for k in uses.values())
        # k=2: the two constraints share one edge -> 1 edge; k>=3: 2k-3
        edges = sum(1 if k <= 2 else 2 * k - 3 for k in uses.values())
        deg = max(len(sc) for sc, _ in self.cons)
        return dict(variables=self.nv, constraints=len(self.cons), btn_edges=edges, max_arity=deg)


def simplify(cons):
    """Slice out variables pinned by a unary factor with one nonzero entry
    (constants and interface prefixes); returns None if some factor vanishes."""
    cons = [(tuple(s), t) for s, t in cons]
    pinned = set()
    while True:
        pin = None
        for s, t in cons:
            if len(s) == 1 and t.sum() == 1:
                pin = (s[0], int(t[1])); break
            if t.sum() == 0: return None, pinned
        if pin is None: return cons, pinned
        v, b = pin
        pinned.add(v)
        out = []
        for s, t in cons:
            if v in s:
                ax = s.index(v)
                t = np.take(t, b, axis=ax); s = s[:ax] + s[ax + 1:]
                if len(s) == 0:
                    if int(t) == 0: return None, pinned
                    continue
            out.append((s, t))
        cons = out


def contract(nv, cons):
    """Number of satisfying assignments (all nv bit variables) by greedy
    min-degree bucket elimination."""
    facs, pinned = simplify(cons)
    if facs is None: return 0
    alive = set()
    for s, _ in facs:
        alive |= set(s)
    scalar = 2 ** (nv - len(alive) - len(pinned))
    adj = {v: set() for v in alive}
    for s, _ in facs:
        for v in s: adj[v] |= set(s) - {v}
    while alive:
        def fill(u):
            nb = list(adj[u])
            return sum(1 for i in range(len(nb)) for j in range(i) if nb[j] not in adj[nb[i]])
        cand = sorted(alive, key=lambda u: len(adj[u]))[:40]
        v = min(cand, key=lambda u: (fill(u), len(adj[u])))
        bucket = [f for f in facs if v in f[0]]
        facs = [f for f in facs if v not in f[0]]
        alive.discard(v)
        if not bucket:
            scalar *= 2
            continue
        vs = []
        for s, _ in bucket:
            for u in s:
                if u not in vs: vs.append(u)
        out = [u for u in vs if u != v]
        assert len(vs) <= len(LET)
        global MAXW; MAXW = max(MAXW, len(vs))
        m = {u: LET[i] for i, u in enumerate(vs)}
        sub = ",".join("".join(m[u] for u in s) for s, _ in bucket)
        res = np.einsum(sub + "->" + "".join(m[u] for u in out), *[t for _, t in bucket], optimize=False)
        for u in out:
            adj[u] |= set(out) - {u}
            adj[u].discard(v)
        del adj[v]
        if out: facs.append((tuple(out), res))
        else: scalar *= int(res)
    return scalar


def contract_cut(nv, cons, cut):
    """Cutset conditioning: sum of contract() over all assignments of the
    variables in `cut` (cost 2^|cut| contractions of a narrower network)."""
    total = 0
    for m in range(1 << len(cut)):
        extra = [((v,), np.array([1 - (m >> i & 1), m >> i & 1], dtype=np.int64)) for i, v in enumerate(cut)]
        total += contract(nv, list(cons) + extra)
    return total
