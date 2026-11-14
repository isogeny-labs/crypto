"""Self-contained F_{2^n} + binary curve + chained m=4 S3 CNF generator + oracle. Pilot code."""
import random, itertools
POLY = {7: (1<<7)|0b11, 11: (1<<11)|0b101, 13: (1<<13)|0b11011, 17: (1<<17)|0b1001, 19: (1<<19)|0b100111}

class GF:
    def __init__(s, n):
        s.n = n; s.f = POLY[n]; s.mask = (1 << n) - 1
        # irreducibility check (n prime): z^(2^n)=z and no roots
        z = 2; t = z
        for _ in range(n): t = s.mul(t, t)
        assert t == z and bin(s.f).count('1') % 2 == 1 and s.f & 1
    def mul(s, a, b):
        r = 0
        while b:
            if b & 1: r ^= a
            b >>= 1; a <<= 1
            if a >> s.n: a ^= s.f
        return r
    def sq(s, a): return s.mul(a, a)
    def inv(s, a):
        assert a; r = 1; e = (1 << s.n) - 2; b = a
        while e:
            if e & 1: r = s.mul(r, b)
            b = s.mul(b, b); e >>= 1
        return r
    def sqrt(s, a):
        for _ in range(s.n - 1): a = s.mul(a, a)
        return a
    def tr(s, a):
        t = a
        for _ in range(s.n - 1): a = s.mul(a, a); t ^= a
        return t
    def htr(s, a):  # half trace, n odd: h^2+h = a if tr(a)=0
        h = a
        for _ in range((s.n - 1) // 2): a = s.mul(s.mul(a, a), s.mul(a, a)); h ^= a
        return h
    def quad(s, a, b, c):
        """solutions of a u^2 + b u + c = 0; returns list (None = every u)"""
        if a == 0:
            if b == 0: return None if c == 0 else []
            return [s.mul(c, s.inv(b))]
        if b == 0: return [s.sqrt(s.mul(c, s.inv(a)))]
        k = s.mul(s.mul(a, c), s.inv(s.sq(b)))
        if s.tr(k): return []
        t = s.htr(k); r = s.mul(b, s.inv(a))
        return [s.mul(r, t), s.mul(r, t ^ 1)]

class Curve:  # y^2 + xy = x^3 + a x^2 + b
    def __init__(s, F, a=1, b=1): s.F = F; s.a = a; s.b = b
    def S3(s, x1, x2, x3):
        F = s.F; e = F.mul(x1, x2) ^ F.mul(x1, x3) ^ F.mul(x2, x3)
        return F.sq(e) ^ F.mul(F.mul(x1, x2), x3) ^ s.b
    def lift(s, x):
        F = s.F
        if x == 0: return (0, F.sqrt(s.b))
        k = x ^ s.a ^ F.mul(s.b, F.inv(F.sq(x)))
        if F.tr(k): return None
        return (x, F.mul(x, F.htr(k)))
    def neg(s, P): return None if P is None else (P[0], P[0] ^ P[1])
    def add(s, P, Q):
        F = s.F
        if P is None: return Q
        if Q is None: return P
        x1, y1 = P; x2, y2 = Q
        if x1 == x2:
            if y1 != y2 or x1 == 0: return None
            lam = x1 ^ F.mul(y1, F.inv(x1))
            x3 = F.sq(lam) ^ lam ^ s.a
            y3 = F.sq(x1) ^ F.mul(lam ^ 1, x3)
            return (x3, y3)
        lam = F.mul(y1 ^ y2, F.inv(x1 ^ x2))
        x3 = F.sq(lam) ^ lam ^ x1 ^ x2 ^ s.a
        y3 = F.mul(lam, x1 ^ x3) ^ x3 ^ y1
        return (x3, y3)
    def on(s, P):
        F = s.F; x, y = P
        return F.sq(y) ^ F.mul(x, y) == F.mul(F.sq(x), x) ^ F.mul(s.a, F.sq(x)) ^ s.b

def oracle(E, l, xR):
    """algebraic MITM for the relaxed chained system. returns (decomposable, witness, quad_solves, |U|)"""
    F = E.F; U = {}; nq = 0
    for x1 in range(1 << l):
        for x2 in range(x1, 1 << l):
            s_ = x1 ^ x2; p = F.mul(x1, x2); nq += 1
            sols = F.quad(F.sq(s_), p, F.sq(p) ^ E.b)
            assert sols is not None
            for u in sols: U.setdefault(u, (x1, x2))
    wit = None
    for u in U:
        nq += 1
        sols = F.quad(F.sq(u ^ xR), F.mul(u, xR), F.sq(F.mul(u, xR)) ^ E.b)
        if sols is None: sols = list(U)[:1]
        for v in sols:
            if v in U: wit = (U[u], U[v], u, v); break
        if wit: break
    return wit is not None, wit, nq, len(U)

# ---------------- CNF ----------------
class Enc:
    def __init__(s, F):
        s.F = F; s.nv = 0; s.cl = []; s.andc = {}
    def new(s): s.nv += 1; return s.nv
    def AND(s, a, b):
        if a == b: return a
        k = (min(a, b), max(a, b))
        if k not in s.andc:
            c = s.new(); s.andc[k] = c
            s.cl += [[-c, a], [-c, b], [c, -a, -b]]
        return s.andc[k]
    def xor(s, vs, rhs):
        """XOR(vs) = rhs"""
        vs = list(vs)
        if not vs:
            if rhs: s.cl.append([])
            return
        while len(vs) > 4:
            t = s.new(); a, b, c = vs[:3]; vs = vs[3:] + [t]
            s._x([a, b, c, t], 0)
        s._x(vs, rhs)
    def _x(s, vs, rhs):
        k = len(vs)
        for bits in range(1 << k):
            if bin(bits).count('1') % 2 != rhs:  # forbid assignments with wrong parity
                s.cl.append([(-v if (bits >> i) & 1 else v) for i, v in enumerate(vs)])
    # element expression: (const, {var: fieldconst})
    def var_elt(s, basis):
        vs = [s.new() for _ in basis]
        return vs, (0, {v: b for v, b in zip(vs, basis)})
    @staticmethod
    def addx(*es):
        c = 0; d = {}
        for e in es:
            c ^= e[0]
            for v, a in e[1].items():
                d[v] = d.get(v, 0) ^ a
                if d[v] == 0: del d[v]
        return (c, d)
    def cmul(s, k, e):
        return (s.F.mul(k, e[0]), {v: s.F.mul(k, a) for v, a in e[1].items()} if k else {})
    def sqx(s, e):
        return (s.F.sq(e[0]), {v: s.F.sq(a) for v, a in e[1].items()})
    def mulx(s, e1, e2):
        F = s.F; d = {}
        for v1, a1 in e1[1].items():
            for v2, a2 in e2[1].items():
                w = s.AND(v1, v2); d[w] = d.get(w, 0) ^ F.mul(a1, a2)
        d = {v: a for v, a in d.items() if a}
        return s.addx((F.mul(e1[0], e2[0]), d), s.cmul(e1[0], (0, e2[1])), s.cmul(e2[0], (0, e1[1])))
    def eqzero(s, e):
        for k in range(s.F.n):
            s.xor([v for v, a in e[1].items() if (a >> k) & 1], (e[0] >> k) & 1)
    def materialize(s, e):
        basis = [1 << k for k in range(s.F.n)]
        vs, m = s.var_elt(basis)
        s.eqzero(s.addx(e, m)); return vs, m

def encode(E, l, xR):
    F = E.F; en = Enc(F); n = F.n
    full = [1 << k for k in range(n)]; vb = [1 << k for k in range(l)]
    X = [en.var_elt(vb) for _ in range(4)]
    ub, u = en.var_elt(full); vbits, v = en.var_elt(full)
    one = (E.b, {})
    def s3(xa, xb, c):
        s_ = en.addx(xa, xb)
        _, p = en.materialize(en.mulx(xa, xb))
        en.eqzero(en.addx(en.sqx(p), en.mulx(en.sqx(s_), en.sqx(c)), en.mulx(p, c), one))
    s3(X[0][1], X[1][1], u); s3(X[2][1], X[3][1], v)
    _, w = en.materialize(en.mulx(u, v))
    en.eqzero(en.addx(en.sqx(w), en.cmul(F.sq(xR), en.addx(en.sqx(u), en.sqx(v))), en.cmul(xR, w), one))
    return en, [x[0] for x in X], ub, vbits

def val(model, bits):
    return sum(1 << i for i, b in enumerate(bits) if model[b - 1] > 0)

def check_model(E, l, xR, model, xb, ub, vbits):
    xs = [val(model, b) for b in xb]; u = val(model, ub); v = val(model, vbits)
    ok = E.S3(xs[0], xs[1], u) == 0 and E.S3(xs[2], xs[3], v) == 0 and E.S3(u, v, xR) == 0
    pts = [E.lift(x) for x in xs]
    lift = 'nonrational'
    if all(p is not None for p in pts):
        lift = 'rational_no_sign'
        for sg in itertools.product([0, 1], repeat=4):
            S = None
            for p, g in zip(pts, sg): S = E.add(S, E.neg(p) if g else p)
            if S is not None and S[0] == xR: lift = 'lifted'; break
    return ok, lift, xs, u, v

def gen_planted(E, l, rng):
    while True:
        xs = [rng.randrange(1, 1 << l) for _ in range(4)]
        pts = [E.lift(x) for x in xs]
        if any(p is None for p in pts): continue
        pts = [E.neg(p) if rng.random() < .5 else p for p in pts]
        A = E.add(pts[0], pts[1]); B = E.add(pts[2], pts[3])
        if A is None or B is None: continue
        R = E.add(A, B)
        if R is None or R[0] == 0: continue
        assert E.on(R)
        return R[0], xs

def gen_random_unsat(E, l, rng):
    tries = 0
    while True:
        tries += 1
        x = rng.randrange(1, 1 << E.F.n)
        if E.lift(x) is None: continue
        dec, _, nq, nu = oracle(E, l, x)
        if not dec: return x, tries
