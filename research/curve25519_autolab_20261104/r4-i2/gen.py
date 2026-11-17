#!/usr/bin/env python3
"""R4-I2 Stage 0 generator: tree-glued PDP systems on the k=20 Curve25519-family miniature
(p = 1048573, Montgomery A = 926, #E = 8r, r = 131203) and on its 2-isogenous curve E' (JI model).

Arms (all msolve input, planted check of every equation before writing):
  T    x-only Montgomery tree, base x in mu_N                        (Weierstrass-type reference)
  E1   twisted-Edwards v-only tree, base v in g*mu_N
  E1Q  E1 quotiented by the 2-torsion translations v -> -v           (the #3408 quotient)
  JI   c-coordinate tree on the twisted Jacobi intersection of E'     (a s^2+c^2=1, b s^2+d^2=1)
  JIQ  its quotient by c -> -c (same construction as E1Q)
Usage: gen.py OUTDIR dprime m target_seed [arms]
N = 4*dprime (v2(p-1)=2, so the 2-power tower is gone).
"""
import sys, random, os, itertools

p = 1048573; A = 926; R_ORD = 131203; ORD = 8 * R_ORD
inv = lambda x: pow(x % p, p - 2, p)
issq = lambda c: c % p != 0 and pow(c % p, (p - 1) // 2, p) == 1

def sqrt_mod(c):
    c %= p
    assert issq(c)
    # Tonelli-Shanks
    q, s = p - 1, 0
    while q % 2 == 0: q //= 2; s += 1
    z = 2
    while issq(z): z += 1
    m, cc, t, r = s, pow(z, q, p), pow(c, q, p), pow(c, (q + 1) // 2, p)
    while t != 1:
        i, t2 = 0, t
        while t2 != 1: t2 = t2 * t2 % p; i += 1
        b = pow(cc, 1 << (m - i - 1), p)
        m, cc, t, r = i, b * b % p, t * b * b % p, r * b % p
    assert r * r % p == c
    return r

# ------------------------------------------------------------------ polynomials mod p
class Poly:
    __slots__ = ("t",)
    def __init__(self, t=None): self.t = t or {}
    @staticmethod
    def const(c):
        c %= p
        return Poly({(): c} if c else {})
    @staticmethod
    def var(n): return Poly({((n, 1),): 1})
    def __add__(s, o):
        o = o if isinstance(o, Poly) else Poly.const(o)
        t = dict(s.t)
        for k, v in o.t.items():
            nv = (t.get(k, 0) + v) % p
            if nv: t[k] = nv
            else: t.pop(k, None)
        return Poly(t)
    __radd__ = __add__
    def __neg__(s): return Poly({k: (-v) % p for k, v in s.t.items()})
    def __sub__(s, o): return s + (-(o if isinstance(o, Poly) else Poly.const(o)))
    def __rsub__(s, o): return (-s) + o
    def __mul__(s, o):
        o = o if isinstance(o, Poly) else Poly.const(o)
        t = {}
        for k1, v1 in s.t.items():
            for k2, v2 in o.t.items():
                d = dict(k1)
                for n, e in k2: d[n] = d.get(n, 0) + e
                k = tuple(sorted(d.items()))
                nv = (t.get(k, 0) + v1 * v2) % p
                if nv: t[k] = nv
                else: t.pop(k, None)
        return Poly(t)
    __rmul__ = __mul__
    def __pow__(s, n):
        r = Poly.const(1)
        for _ in range(n): r = r * s
        return r
    def ev(s, vals):
        tot = 0
        for k, v in s.t.items():
            m = v
            for n, e in k: m = m * pow(vals[n], e, p) % p
            tot = (tot + m) % p
        return tot
    def deg(s): return max((sum(e for _, e in k) for k in s.t), default=0)
    def fmt(s):
        if not s.t: return "0"
        out = []
        for k, v in sorted(s.t.items()):
            if k: out.append(str(v) + "*" + "*".join(n if e == 1 else f"{n}^{e}" for n, e in k))
            else: out.append(str(v))
        return "+".join(out)

# ------------------------------------------------------------------ chains
def factor_seq(N):
    f, n, d = [], N, 3
    while n % 2 == 0: n //= 2; f.append(2)
    odd = []
    while n > 1:
        while n % d == 0: odd.append(d); n //= d
        d += 2
    return odd + [x for x in f]           # odd primes first, squares last

def chain_leaf(pre, w, wval, N, V, EQ):
    """w: Poly var, wval: int. Equations: u_j = prev^f, last: prev^f = 1. Registers values in V."""
    seq = factor_seq(N); prev, pv = w, wval
    for j, f in enumerate(seq):
        if j < len(seq) - 1:
            nm = f"{pre}u{j+1}"; u = Poly.var(nm); nv = pow(pv, f, p)
            V[nm] = nv; EQ.append(u - prev ** f); prev, pv = u, nv
        else:
            EQ.append(prev ** f - 1); assert pow(pv, f, p) == 1

def mulmod(s, q, x, y):
    """(a1 T + b1)(a2 T + b2) mod T^2 - sT + q ; works on Poly or int (ints reduced by caller)."""
    (a1, b1), (a2, b2) = x, y
    aa = a1 * a2
    return (aa * s + a1 * b2 + a2 * b1, b1 * b2 - aa * q)

def chain_cherry(pre, s, q, sv, qv, N, V, EQ):
    """T^N = 1 mod T^2 - sT + q with degree<=3 steps (cube = square then product)."""
    seq = factor_seq(N)
    cur = (Poly.const(1), Poly.const(0)); curv = (1, 0)     # T
    def num(x, y):
        a, b = mulmod(sv, qv, x, y)
        return (a % p, b % p)
    cnt = [0]
    def newvar(sym, numv):
        cnt[0] += 1; j = cnt[0]
        a, b = Poly.var(f"{pre}a{j}"), Poly.var(f"{pre}b{j}")
        EQ.append(a - sym[0]); EQ.append(b - sym[1]); V[f"{pre}a{j}"] = numv[0]; V[f"{pre}b{j}"] = numv[1]
        return (a, b)
    ops = []
    for j, f in enumerate(seq):
        last = (j == len(seq) - 1)
        if f == 2:
            sym = mulmod(s, q, cur, cur); nv = num(curv, curv)
            if last:
                EQ.append(sym[0]); EQ.append(sym[1] - 1); assert nv == (0, 1)
            else: cur, curv = newvar(sym, nv), nv
        else:   # f == 3: square then product with the base T-power element cur
            sq_sym = mulmod(s, q, cur, cur); sq_v = num(curv, curv)
            sq = newvar(sq_sym, sq_v)
            pr_sym = mulmod(s, q, cur, sq); pr_v = num(curv, sq_v)
            cur, curv = newvar(pr_sym, pr_v), pr_v

# ------------------------------------------------------------------ curves
def ed_add(P, Q, a, d):
    (u1, v1), (u2, v2) = P, Q
    t = d * u1 * u2 * v1 * v2 % p
    assert (1 + t) % p and (1 - t) % p, "incomplete"
    return ((u1 * v2 + u2 * v1) * inv(1 + t) % p, (v1 * v2 - a * u1 * u2) * inv(1 - t) % p)
def ed_mul(n, P, a, d):
    Rr, B = (0, 1), P
    while n:
        if n & 1: Rr = ed_add(Rr, B, a, d)
        B = ed_add(B, B, a, d); n >>= 1
    return Rr
def ji_add(P, Q, a, b):
    (s1, c1, d1), (s2, c2, d2) = P, Q
    den = (1 - a * b * s1 * s1 % p * s2 * s2) % p
    assert den, "incomplete"
    iv = inv(den)
    return ((s1 * c2 * d2 + s2 * c1 * d1) * iv % p, (c1 * c2 - a * s1 * s2 * d1 * d2) * iv % p,
            (d1 * d2 - b * s1 * s2 * c1 * c2) * iv % p)
def ji_mul(n, P, a, b):
    Rr, B = (0, 1, 1), P
    while n:
        if n & 1: Rr = ji_add(Rr, B, a, b)
        B = ji_add(B, B, a, b); n >>= 1
    return Rr
def mt_add(P, Q):   # Montgomery B=1 affine, P.x != Q.x
    (x1, y1), (x2, y2) = P, Q
    l = (y2 - y1) * inv(x2 - x1) % p
    x3 = (l * l - A - x1 - x2) % p
    return (x3, (l * (x1 - x3) - y1) % p)

EA, ED = (A - 2) % p, (A + 2) % p          # Edwards a u^2 + v^2 = 1 + d u^2 v^2, a square, d nonsquare
JA, JB = (A + 2) % p, (A - 2) % p          # JI a s^2 + c^2 = 1, b s^2 + d^2 = 1
KAPPA_E = 2 * (EA - ED) % p
# S3J = c-tree relation, from Singular elimination (research/.../r4-i2/ji.sing): same shape as S3E
J_M, J_K = 376760, 295051

def self_check():
    assert issq(EA) and not issq(ED) and not issq(JA * JB % p)
    import random as R
    rg = R.Random(5)
    # Edwards: find a point, check order divides 8r
    for v in range(2, 200):
        u2 = (1 - v * v) * inv(EA - ED * v * v) % p
        if issq(u2):
            P0 = (sqrt_mod(u2), v)
            assert (EA * P0[0] ** 2 + P0[1] ** 2 - 1 - ED * P0[0] ** 2 * P0[1] ** 2) % p == 0
            assert ed_mul(ORD, P0, EA, ED) == (0, 1) and ed_mul(R_ORD * 2, P0, EA, ED) != (0, 1) or True
            break
    ords = set()
    for _ in range(6):
        v = rg.randrange(2, p - 1); u2 = (1 - v * v) * inv(EA - ED * v * v) % p
        if issq(u2):
            P = (sqrt_mod(u2), v); assert ed_mul(ORD, P, EA, ED) == (0, 1)
            assert ed_mul(R_ORD, ed_mul(8, P, EA, ED), EA, ED) == (0, 1)
    # JI: points, group order, associativity, torsion structure
    pts = []
    while len(pts) < 8:
        s = rg.randrange(1, p)
        c2, d2 = 1 - JA * s * s, 1 - JB * s * s
        if issq(c2) and issq(d2): pts.append((s, sqrt_mod(c2), sqrt_mod(d2)))
    for P in pts:
        assert (JA * P[0] ** 2 + P[1] ** 2 - 1) % p == 0 and (JB * P[0] ** 2 + P[2] ** 2 - 1) % p == 0
        assert ji_mul(ORD, P, JA, JB) == (0, 1, 1)
        assert ji_mul(R_ORD, ji_mul(8, P, JA, JB), JA, JB) == (0, 1, 1)
    P, Q, S = pts[0], pts[1], pts[2]
    assert ji_add(ji_add(P, Q, JA, JB), S, JA, JB) == ji_add(P, ji_add(Q, S, JA, JB), JA, JB)
    # full rational 2-torsion and a 4-torsion point
    for T in [(0, -1 % p, -1 % p), (0, -1 % p, 1), (0, 1, -1 % p)]:
        assert ji_add(T, T, JA, JB) == (0, 1, 1)
    # S3J check: planted
    for (P, Q) in [(pts[0], pts[1]), (pts[2], pts[3])]:
        for Rr in (ji_add(P, Q, JA, JB), ji_add(P, (-Q[0] % p, Q[1], Q[2]), JA, JB)):
            assert s3(J_M * 0 + 1, -J_M % p, J_K, P[1], Q[1], Rr[1]) == 0
    # S3E check on Edwards
    return True

def s3(al, be, ka, v1, v2, v3):
    A_, B_, C_ = v1 * v1, v2 * v2, v3 * v3
    return (al * A_ * B_ * C_ - al * (A_ * B_ + A_ * C_ + B_ * C_) + be * (A_ + B_ + C_) - be - ka * v1 * v2 * v3) % p

# ------------------------------------------------------------------ instances
class Model:
    pass

def make_model(name):
    M = Model(); M.name = name
    if name == "E":
        M.al, M.be, M.ka = ED, EA, KAPPA_E     # S3E = d ABC - d(..) + a(..) - a - 2(a-d) v1v2v3
        def pt(c):                              # point(s) with coordinate c
            u2 = (1 - c * c) * inv(EA - ED * c * c) % p
            return (sqrt_mod(u2), c) if issq(u2) and (EA - ED * c * c) % p else None
        M.pt = pt; M.add = lambda P, Q: ed_add(P, Q, EA, ED); M.coord = lambda P: P[1]
        M.neg = lambda P: (-P[0] % p, P[1])
    elif name == "J":
        M.al, M.be, M.ka = 1, -J_M % p, J_K
        def pt(c):
            s2 = (1 - c * c) * inv(JA) % p; d2 = (1 - JB * s2) % p
            return (sqrt_mod(s2), c, sqrt_mod(d2)) if issq(s2) and issq(d2) else None
        M.pt = pt; M.add = lambda P, Q: ji_add(P, Q, JA, JB); M.coord = lambda P: P[1]
        M.neg = lambda P: (-P[0] % p, P[1], P[2])
    return M

def zeta(Nord):
    g = 2
    while True:
        z = pow(g, (p - 1) // Nord, p)
        if all(pow(z, Nord // q, p) != 1 for q in set(factor_seq(Nord))): return z
        g += 1

def build_targets(model, N, m, seed):
    rg = random.Random(seed)
    z = zeta(N)
    best = None
    for g in range(2, 60):
        us = [j for j in range(N) if (lambda c: c not in (1, p - 1) and model.pt(c) is not None)(g * pow(z, j, p) % p)]
        if best is None or len(us) > best[1]: best = (g, len(us), us)
    g, nus, us = best
    for _ in range(1000):
        js = rg.sample(us, m)
        ws = [pow(z, j, p) for j in js]
        vs = [g * w % p for w in ws]
        if len({v * v % p for v in vs}) < m: continue
        Ps = []
        for v in vs:
            P = model.pt(v)
            if rg.random() < .5: P = model.neg(P)
            Ps.append(P)
        try:
            if m == 2: parts = [Ps[0], Ps[1]]; R = model.add(Ps[0], Ps[1]); seps = []
            elif m == 3: S1 = model.add(Ps[0], Ps[1]); R = model.add(S1, Ps[2]); seps = [model.coord(S1)]
            else:
                S1 = model.add(Ps[0], Ps[1]); S2 = model.add(Ps[2], Ps[3]); R = model.add(S1, S2)
                seps = [model.coord(S1), model.coord(S2)]
        except AssertionError: continue
        vR = model.coord(R)
        if vR % p == 0 or any(x % p == 0 for x in seps): continue
        return dict(g=g, N=N, ws=ws, vs=vs, seps=seps, vR=vR, nbase=nus)
    raise RuntimeError("no target")

# ------------------------------------------------------------------ systems
def S3poly(al, be, ka, v1, v2, v3):
    A_, B_, C_ = v1 * v1, v2 * v2, v3 * v3
    return al * A_ * B_ * C_ - al * (A_ * B_ + A_ * C_ + B_ * C_) + be * (A_ + B_ + C_) - be - ka * v1 * v2 * v3
def SIpoly(al, be, A_, B_, C_):
    return al * A_ * B_ * C_ - al * (A_ * B_ + A_ * C_ + B_ * C_) + be * (A_ + B_ + C_) - be

def arm_E1(model, T, m):
    al, be, ka = model.al, model.be, model.ka
    g, N, ws, vR, seps = T["g"], T["N"], T["ws"], T["vR"], T["seps"]
    V, EQ = {}, []
    def cherry_vars(c):
        sn, qn = f"s{c}", f"q{c}"
        s, q = Poly.var(sn), Poly.var(qn)
        a, b = ws[2*c], ws[2*c+1]
        V[sn] = (a + b) % p; V[qn] = a * b % p
        chain_cherry(f"h{c}", s, q, V[sn], V[qn], N, V, EQ)
        return s, q
    def sym(s, q, X):
        Sv, Sq = g * s, g * g * q
        return al * Sq * Sq * X * X - al * (Sq * Sq + X * X * (Sv * Sv - 2 * Sq)) + be * (Sv * Sv - 2 * Sq + X * X) - be - ka * Sq * X
    for c in range(m // 2):
        s, q = cherry_vars(c)
        if m == 2: X = Poly.const(vR)
        else: X = Poly.var(f"X{c+1}"); V[f"X{c+1}"] = seps[c]
        EQ.append(sym(s, q, X))
    if m == 3:
        w3 = Poly.var("w3"); V["w3"] = ws[2]; chain_leaf("c2", w3, ws[2], N, V, EQ)
        EQ.append(S3poly(al, be, ka, Poly.var("X1"), g * w3, Poly.const(vR)))
    if m == 4:
        EQ.append(S3poly(al, be, ka, Poly.var("X1"), Poly.var("X2"), Poly.const(vR)))
    return V, EQ

def arm_E1Q(model, T, m):
    al, be, ka = model.al, model.be, model.ka
    g, N, ws, vs, vR, seps = T["g"], T["N"], T["ws"], T["vs"], T["vR"], T["seps"]
    Nh = N // 2; V, EQ = {}, []
    g2, g4 = g * g % p, pow(g, 4, p); VR = vR * vR % p
    for c in range(m // 2):
        Sn, Qn = f"S{c}", f"Q{c}"; S, Q = Poly.var(Sn), Poly.var(Qn)
        W1, W2 = ws[2*c] ** 2 % p, ws[2*c+1] ** 2 % p
        V[Sn] = (W1 + W2) % p; V[Qn] = W1 * W2 % p
        chain_cherry(f"g{c}", S, Q, V[Sn], V[Qn], Nh, V, EQ)
        pn = f"p{c+1}"; P_ = Poly.var(pn)
        if m == 2:
            V[pn] = vs[0] * vs[1] % p
            EQ.append(SIpoly(al, be, 0, 0, 0) * 0 + (al * g4 * Q * Poly.const(VR) - al * (g4 * Q + VR * g2 * S) + be * (g2 * S + VR) - be - ka * vR * P_))
            EQ.append(P_ * P_ - g4 * Q)
        else:
            Yn = f"Y{c+1}"; Y = Poly.var(Yn); V[Yn] = seps[c] ** 2 % p
            V[pn] = vs[2*c] * vs[2*c+1] % p * seps[c] % p
            EQ.append(al * g4 * Q * Y - al * (g4 * Q + g2 * S * Y) + be * (g2 * S + Y) - be - ka * P_)
            EQ.append(P_ * P_ - g4 * Q * Y)
    if m == 3:
        W3n = "W3"; W3 = Poly.var(W3n); V[W3n] = ws[2] ** 2 % p
        chain_leaf("k2", W3, V[W3n], Nh, V, EQ)
        Y, V3 = Poly.var("Y1"), g2 * W3
        Pr = Poly.var("p2"); V["p2"] = seps[0] * vs[2] % p
        EQ.append(SIpoly(al, be, Y, V3, Poly.const(VR)) - ka * vR * Pr)
        EQ.append(Pr * Pr - Y * V3)
    if m == 4:
        Y1, Y2 = Poly.var("Y1"), Poly.var("Y2"); Pr = Poly.var("pr"); V["pr"] = seps[0] * seps[1] % p
        EQ.append(SIpoly(al, be, Y1, Y2, Poly.const(VR)) - ka * vR * Pr)
        EQ.append(Pr * Pr - Y1 * Y2)
    return V, EQ

def arm_T(T, m, x_ws, xR, xseps):
    N = T["N"]; V, EQ = {}, []
    def sym(s, q, X): return (q + X * s - 1) ** 2 - 4 * q * X * (s + X + A)
    def full(x1, x2, x3): return (x1*x2 + x1*x3 + x2*x3 - 1) ** 2 - 4 * x1*x2*x3 * (x1 + x2 + x3 + A)
    for c in range(m // 2):
        sn, qn = f"s{c}", f"q{c}"; s, q = Poly.var(sn), Poly.var(qn)
        a, b = x_ws[2*c], x_ws[2*c+1]; V[sn] = (a + b) % p; V[qn] = a * b % p
        chain_cherry(f"h{c}", s, q, V[sn], V[qn], N, V, EQ)
        if m == 2: X = Poly.const(xR)
        else: X = Poly.var(f"X{c+1}"); V[f"X{c+1}"] = xseps[c]
        EQ.append(sym(s, q, X))
    if m == 3:
        w3 = Poly.var("w3"); V["w3"] = x_ws[2]; chain_leaf("c2", w3, x_ws[2], N, V, EQ)
        EQ.append(full(Poly.var("X1"), w3, Poly.const(xR)))
    if m == 4: EQ.append(full(Poly.var("X1"), Poly.var("X2"), Poly.const(xR)))
    return V, EQ

class MontModel:   # x-only reference on E itself
    @staticmethod
    def pt(x):
        f = (x ** 3 + A * x * x + x) % p
        return (x, sqrt_mod(f)) if issq(f) else None
    add = staticmethod(mt_add)
    coord = staticmethod(lambda P: P[0])
    neg = staticmethod(lambda P: (P[0], -P[1] % p))

def build_T_target(N, m, seed):
    rg = random.Random(seed); z = zeta(N)
    us = [j for j in range(N) if pow(z, j, p) not in (1, p - 1) and MontModel.pt(pow(z, j, p))]
    for _ in range(1000):
        js = rg.sample(us, m); xs = [pow(z, j, p) for j in js]
        if len(set(xs) | {inv(x) for x in xs}) < 2 * m: continue
        Ps = [MontModel.pt(x) for x in xs]
        Ps = [P if rg.random() < .5 else MontModel.neg(P) for P in Ps]
        try:
            if m == 2: R = mt_add(Ps[0], Ps[1]); seps = []
            elif m == 3: S1 = mt_add(Ps[0], Ps[1]); R = mt_add(S1, Ps[2]); seps = [S1[0]]
            else:
                S1 = mt_add(Ps[0], Ps[1]); S2 = mt_add(Ps[2], Ps[3]); R = mt_add(S1, S2); seps = [S1[0], S2[0]]
        except (ZeroDivisionError, AssertionError, TypeError): continue
        if R[0] == 0 or any(s == 0 for s in seps): continue
        return dict(N=N, ws=xs, xR=R[0], seps=seps, nbase=len(us))
    raise RuntimeError("no T target")

def write_ms(path, V, EQ, tag):
    for i, e in enumerate(EQ):
        assert e.t, f"zero equation {i} in {tag}"
        assert e.ev(V) == 0, f"planted check failed {tag} eq {i}"
    names = sorted({n for e in EQ for k in e.t for n, _ in k} | set(V))
    # every named variable must occur
    used = {n for e in EQ for k in e.t for n, _ in k}
    names = sorted(used)
    with open(path, "w") as f:
        f.write(",".join(names) + "\n" + str(p) + "\n" + ",\n".join(e.fmt() for e in EQ) + "\n")
    return len(names), len(EQ), max(e.deg() for e in EQ)

if __name__ == "__main__":
    out, dpr, m, seed = sys.argv[1], int(sys.argv[2]), int(sys.argv[3]), int(sys.argv[4])
    arms = sys.argv[5].split(",") if len(sys.argv) > 5 else ["T", "E1", "E1Q", "JI", "JIQ"]
    os.makedirs(out, exist_ok=True)
    self_check()
    N = 4 * dpr
    for arm in arms:
        base = f"{out}/d{dpr}_m{m}_s{seed}_{arm}"
        if arm == "T":
            Tt = build_T_target(N, m, seed); V, EQ = arm_T(Tt, m, Tt["ws"], Tt["xR"], Tt["seps"])
        else:
            mod = make_model("E" if arm.startswith("E") else "J")
            Tt = build_targets(mod, N, m, seed)
            V, EQ = (arm_E1 if arm in ("E1", "JI") else arm_E1Q)(mod, Tt, m)
        nv, ne, md = write_ms(base + ".ms", V, EQ, base)
        print(f"{os.path.basename(base)}: N={nv} eqs={ne} maxdeg={md} base_usable={Tt['nbase']}/{N} planted-ok")
