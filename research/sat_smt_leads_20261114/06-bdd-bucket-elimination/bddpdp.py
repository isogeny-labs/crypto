#!/usr/bin/env python3
"""Idea 6 pilot: ROBDD bucket elimination for the point decomposition problem.

Self-contained: F_{2^n} arithmetic, binary curve y^2+xy=x^3+a x^2+b (a=0,b=1),
Semaev S3, Weil descent to bit ANF, a small ROBDD package, bucket elimination.
See PROTOCOL.md.  Usage:
  python3 bddpdp.py validate
  python3 bddpdp.py run --m 3 --l 3 4 5 --targets 8 --out results_m3.json
"""
import sys, time, json, random, argparse

sys.setrecursionlimit(100000)

IRRED = {5: 0b100101, 7: 0b10000011, 11: (1 << 11) | 0b101, 13: (1 << 13) | 0b11011,
         17: (1 << 17) | 0b1001, 19: (1 << 19) | 0b100111, 23: (1 << 23) | 0b100001}


# ----------------------------------------------------------------- field
class GF:
    def __init__(self, n):
        self.n = n
        self.mod = IRRED[n]
        self.mask = (1 << n) - 1
        # irreducibility (n prime): x^(2^n) == x, and no root in F2
        x = 2
        y = x
        for _ in range(n):
            y = self.mul(y, y)
        assert y == x, "modulus not irreducible"
        assert self.mod & 1 and bin(self.mod).count("1") % 2 == 1

    def mul(self, a, b):
        r = 0
        n, mod = self.n, self.mod
        while b:
            if b & 1:
                r ^= a
            b >>= 1
            a <<= 1
            if (a >> n) & 1:
                a ^= mod
        return r

    def sq(self, a):
        return self.mul(a, a)

    def inv(self, a):
        assert a
        # a^(2^n-2)
        r = 1
        e = (1 << self.n) - 2
        base = a
        while e:
            if e & 1:
                r = self.mul(r, base)
            base = self.mul(base, base)
            e >>= 1
        return r

    def sqrt(self, a):
        for _ in range(self.n - 1):
            a = self.mul(a, a)
        return a

    def trace(self, a):
        t = 0
        for _ in range(self.n):
            t ^= a
            a = self.mul(a, a)
        assert t in (0, 1)
        return t

    def halftrace(self, a):
        # n odd: H(a) = sum a^(4^i), i=0..(n-1)/2 ; H^2+H = a + Tr(a)
        h = 0
        for _ in range((self.n - 1) // 2 + 1):
            h ^= a
            a = self.mul(a, a)
            a = self.mul(a, a)
        return h

    def solve_quad(self, A, B, C):
        """roots of A t^2 + B t + C = 0 in the field (list, len<=2); None if every t."""
        if A == 0:
            if B == 0:
                return None if C == 0 else []
            return [self.mul(C, self.inv(B))]
        if B == 0:
            return [self.sqrt(self.mul(C, self.inv(A)))]
        Bi = self.inv(B)
        c = self.mul(self.mul(A, C), self.mul(Bi, Bi))  # z^2+z = AC/B^2, t = (B/A) z
        if self.trace(c):
            return []
        z = self.halftrace(c)
        assert self.mul(z, z) ^ z == c
        f = self.mul(B, self.inv(A))
        return [self.mul(f, z), self.mul(f, z ^ 1)]


# ----------------------------------------------------------------- curve
class Curve:
    """y^2 + xy = x^3 + a x^2 + b ; points are (x,y) or None (infinity)."""

    def __init__(self, F, a=0, b=1):
        self.F, self.a, self.b = F, a, b

    def on(self, P):
        if P is None:
            return True
        F = self.F
        x, y = P
        return F.sq(y) ^ F.mul(x, y) == F.mul(F.sq(x), x) ^ F.mul(self.a, F.sq(x)) ^ self.b

    def lift(self, x):
        """points with this x (list of 0,1,2)."""
        F = self.F
        if x == 0:
            return [(0, F.sqrt(self.b))]
        xi = F.inv(x)
        c = x ^ self.a ^ F.mul(self.b, F.sq(xi))
        if F.trace(c):
            return []
        z = F.halftrace(c)
        return [(x, F.mul(x, z)), (x, F.mul(x, z ^ 1))]

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

    def S3(self, x1, x2, x3):
        F = self.F
        e = F.mul(x1, x2) ^ F.mul(x1, x3) ^ F.mul(x2, x3)
        return F.sq(e) ^ F.mul(F.mul(x1, x2), x3) ^ self.b

    def s3_roots_t(self, x1, x2):
        """t with S3(x1,x2,t)=0: s^2 t^2 + p t + p^2 + b."""
        F = self.F
        p = F.mul(x1, x2)
        s = x1 ^ x2
        return F.solve_quad(F.sq(s), p, F.sq(p) ^ self.b)


# ----------------------------------------------------------------- BDD
class Blowup(Exception):
    pass


class BDD:
    BIG = 1 << 30

    def __init__(self, node_cap=4_000_000, time_cap=300.0):
        self.var = [self.BIG, self.BIG]
        self.lo = [0, 1]
        self.hi = [0, 1]
        self.uniq = {}
        self.c_and = {}
        self.c_xor = {}
        self.c_or = {}
        self.calls = 0
        self.node_cap = node_cap
        self.time_cap = time_cap
        self.t0 = time.time()

    def reset_run(self):
        self.c_and.clear()
        self.c_xor.clear()
        self.c_or.clear()
        self.calls = 0
        self.t0 = time.time()

    def mark(self):
        return len(self.var)

    def rollback(self, mark):
        var, lo, hi, uniq = self.var, self.lo, self.hi, self.uniq
        for i in range(mark, len(var)):
            del uniq[(var[i], lo[i], hi[i])]
        del var[mark:], lo[mark:], hi[mark:]
        self.c_and.clear()
        self.c_xor.clear()
        self.c_or.clear()

    def mk(self, v, l, h):
        if l == h:
            return l
        k = (v, l, h)
        r = self.uniq.get(k)
        if r is None:
            r = len(self.var)
            if r >= self.node_cap:
                raise Blowup("node cap")
            if (r & 0xFFFF) == 0 and time.time() - self.t0 > self.time_cap:
                raise Blowup("time cap")
            self.var.append(v)
            self.lo.append(l)
            self.hi.append(h)
            self.uniq[k] = r
        return r

    def ithvar(self, v):
        return self.mk(v, 0, 1)

    def AND(self, a, b):
        if a == b or b == 1:
            return a
        if a == 0 or b == 0:
            return 0
        if a == 1:
            return b
        if a > b:
            a, b = b, a
        k = (a, b)
        c = self.c_and
        r = c.get(k)
        self.calls += 1
        if r is not None:
            return r
        var = self.var
        va, vb = var[a], var[b]
        if va == vb:
            r = self.mk(va, self.AND(self.lo[a], self.lo[b]), self.AND(self.hi[a], self.hi[b]))
        elif va < vb:
            r = self.mk(va, self.AND(self.lo[a], b), self.AND(self.hi[a], b))
        else:
            r = self.mk(vb, self.AND(a, self.lo[b]), self.AND(a, self.hi[b]))
        c[k] = r
        return r

    def OR(self, a, b):
        if a == b or b == 0:
            return a
        if a == 1 or b == 1:
            return 1
        if a == 0:
            return b
        if a > b:
            a, b = b, a
        k = (a, b)
        c = self.c_or
        r = c.get(k)
        self.calls += 1
        if r is not None:
            return r
        var = self.var
        va, vb = var[a], var[b]
        if va == vb:
            r = self.mk(va, self.OR(self.lo[a], self.lo[b]), self.OR(self.hi[a], self.hi[b]))
        elif va < vb:
            r = self.mk(va, self.OR(self.lo[a], b), self.OR(self.hi[a], b))
        else:
            r = self.mk(vb, self.OR(a, self.lo[b]), self.OR(a, self.hi[b]))
        c[k] = r
        return r

    def XOR(self, a, b):
        if a == b:
            return 0
        if a == 0:
            return b
        if b == 0:
            return a
        if a > b:
            a, b = b, a
        k = (a, b)
        c = self.c_xor
        r = c.get(k)
        self.calls += 1
        if r is not None:
            return r
        var = self.var
        va, vb = var[a], var[b]
        if va == vb:
            r = self.mk(va, self.XOR(self.lo[a], self.lo[b]), self.XOR(self.hi[a], self.hi[b]))
        elif va < vb:
            r = self.mk(va, self.XOR(self.lo[a], b), self.XOR(self.hi[a], b))
        else:
            r = self.mk(vb, self.XOR(a, self.lo[b]), self.XOR(a, self.hi[b]))
        c[k] = r
        return r

    def NOT(self, a):
        return self.XOR(a, 1)

    def exists(self, f, v):
        cache = {}
        var, lo, hi = self.var, self.lo, self.hi

        def rec(g):
            vg = var[g]
            if vg > v:
                return g
            r = cache.get(g)
            self.calls += 1
            if r is not None:
                return r
            if vg == v:
                r = self.OR(lo[g], hi[g])
            else:
                r = self.mk(vg, rec(lo[g]), rec(hi[g]))
            cache[g] = r
            return r

        return rec(f)

    def size(self, roots):
        seen = set()
        stack = [r for r in roots if r > 1]
        lo, hi = self.lo, self.hi
        while stack:
            g = stack.pop()
            if g in seen:
                continue
            seen.add(g)
            a = lo[g]
            if a > 1 and a not in seen:
                stack.append(a)
            a = hi[g]
            if a > 1 and a not in seen:
                stack.append(a)
        return len(seen)

    def support(self, f):
        seen = set()
        sup = set()
        stack = [f]
        while stack:
            g = stack.pop()
            if g <= 1 or g in seen:
                continue
            seen.add(g)
            sup.add(self.var[g])
            stack.append(self.lo[g])
            stack.append(self.hi[g])
        return sup

    def allsat(self, f, levels):
        """all assignments over the given sorted levels (f's support must be inside)."""
        out = []

        def rec(g, i, acc):
            if g == 0:
                return
            if i == len(levels):
                assert g == 1
                out.append(dict(acc))
                return
            v = levels[i]
            if self.var[g] == v:
                acc[v] = 0
                rec(self.lo[g], i + 1, acc)
                acc[v] = 1
                rec(self.hi[g], i + 1, acc)
            else:
                assert self.var[g] > v
                acc[v] = 0
                rec(g, i + 1, acc)
                acc[v] = 1
                rec(g, i + 1, acc)

        rec(f, 0, {})
        return out


# ----------------------------------------------------------------- ANF -> BDD
def anf_to_bdds(B, n, monos, level):
    """monos: dict frozenset(problem vars) -> field coefficient. Returns list of n
    BDDs, bit r of the field-valued polynomial (XOR of ANDs)."""
    outs = [0] * n
    for mono, coef in monos.items():
        if coef == 0:
            continue
        m = 1
        for v in sorted((level[q] for q in mono), reverse=True):
            m = B.AND(B.ithvar(v), m)
        for r in range(n):
            if (coef >> r) & 1:
                outs[r] = B.XOR(outs[r], m)
    return outs


def madd(d, key, c):
    d[key] = d.get(key, 0) ^ c


def zpow(F, e):
    r = 1
    for _ in range(e):
        r = F.mul(r, 2)
    return r


def side_A_constraints(B, F, l, level, b=1):
    """bit r of S3(x1,x2,t) = p^2 + s^2 t^2 + p t + b == 0 ; returns list of n BDDs
    (the constraint, i.e. negated polynomial bit)."""
    n = F.n
    zp = [zpow(F, e) for e in range(4 * n)]
    h = {frozenset(): b}
    g = [dict() for _ in range(n)]
    for a in range(l):
        for bb in range(l):
            key = frozenset([("x", 0, a), ("x", 1, bb)])
            madd(h, key, F.sq(zp[a + bb]))
            for k in range(n):
                madd(g[k], key, zp[a + bb + k])
    for i in (0, 1):
        for a in range(l):
            for k in range(n):
                madd(g[k], frozenset([("x", i, a)]), zp[2 * a + 2 * k])
    acc = anf_to_bdds(B, n, h, level)
    for k in range(n):
        gk = anf_to_bdds(B, n, g[k], level)
        tv = B.ithvar(level[("t", k)])
        for r in range(n):
            acc[r] = B.XOR(acc[r], B.AND(tv, gk[r]))
    return [B.NOT(f) for f in acc]


def side_B_constraints(B, F, l, level, xR, b=1, summand=2):
    """bit r of S3(t,x3,xR) = (t x3)^2 + (t+x3)^2 xR^2 + t x3 xR + b == 0."""
    n = F.n
    zp = [zpow(F, e) for e in range(4 * n)]
    xR2 = F.sq(xR)
    d = {frozenset(): b}
    for a in range(l):
        madd(d, frozenset([("x", summand, a)]), F.mul(zp[2 * a], xR2))
        for k in range(n):
            madd(d, frozenset([("x", summand, a), ("t", k)]),
                 zp[2 * a + 2 * k] ^ F.mul(xR, zp[a + k]))
    for k in range(n):
        madd(d, frozenset([("t", k)]), F.mul(zp[2 * k], xR2))
    return [B.NOT(f) for f in anf_to_bdds(B, n, d, level)]


def m2_constraints(B, F, l, level, xR, b=1):
    """bit r of S3(x1,x2,xR) = p^2 + s^2 xR^2 + p xR + b == 0."""
    n = F.n
    zp = [zpow(F, e) for e in range(4 * n)]
    xR2 = F.sq(xR)
    d = {frozenset(): b}
    for a in range(l):
        for bb in range(l):
            madd(d, frozenset([("x", 0, a), ("x", 1, bb)]),
                 F.sq(zp[a + bb]) ^ F.mul(xR, zp[a + bb]))
    for i in (0, 1):
        for a in range(l):
            madd(d, frozenset([("x", i, a)]), F.mul(zp[2 * a], xR2))
    return [B.NOT(f) for f in anf_to_bdds(B, n, d, level)]


# ----------------------------------------------------------------- orders
def order_m3(name, l, n):
    xint = [("x", i, j) for j in range(l) for i in range(3)]
    xblk = [("x", i, j) for i in range(3) for j in range(l)]
    t = [("t", k) for k in range(n)]
    seq = {"xint_t": xint + t, "xblk_t": xblk + t, "t_xint": t + xint}[name]
    return {q: i for i, q in enumerate(seq)}


def sched_m3(name, l, n):
    x3 = [("x", 2, j) for j in range(l)]
    x12 = [("x", i, j) for j in range(l) for i in range(2)]
    t = [("t", k) for k in range(n)]
    return {"t_first": t + x12 + x3, "x_first": x3 + x12 + t, "x3_t_x12": x3 + t + x12}[name]


def order_m2(name, l):
    if name == "int":
        seq = [("x", i, j) for j in range(l) for i in range(2)]
    elif name == "blk":
        seq = [("x", i, j) for i in range(2) for j in range(l)]
    elif name == "rev":
        seq = []
        for j in range(l):
            seq += [("x", 0, j), ("x", 1, l - 1 - j)]
    return {q: i for i, q in enumerate(seq)}


# ----------------------------------------------------------------- bucket elimination
def bucket_eliminate(B, pool, elim_levels, keep_result=False):
    """pool: list of (bdd, support set of levels). Returns stats dict.
    Peak live = max over steps of nodes reachable from pool + current work."""
    pool = list(pool)
    stats = {}
    init = B.size([f for f, _ in pool])
    peak_live = init
    peak_work = 0
    stats["pool_initial"] = init
    stats["max_single_constraint"] = max(B.size([f]) for f, _ in pool)
    unsat_at = None
    step = 0
    for v in elim_levels:
        bucket = [(f, s) for f, s in pool if v in s]
        if not bucket:
            continue
        rest = [(f, s) for f, s in pool if v not in s]
        bucket.sort(key=lambda fs: B.size([fs[0]]))
        cur, sup = bucket[0]
        remaining = bucket[1:]
        while remaining:
            f, s = remaining.pop(0)
            cur = B.AND(cur, f)
            sup = sup | s
            step += 1
            w = B.size([cur])
            peak_work = max(peak_work, w)
            live = B.size([cur] + [g for g, _ in remaining] + [g for g, _ in rest])
            peak_live = max(peak_live, live)
            if cur == 0:
                break
        if cur == 0:
            unsat_at = step
            pool = [(0, set())]
            break
        cur = B.exists(cur, v)
        sup = sup - {v}
        w = B.size([cur])
        peak_work = max(peak_work, w)
        live = B.size([cur] + [g for g, _ in rest])
        peak_live = max(peak_live, live)
        pool = rest + [(cur, sup)]
    res = 1
    for f, _ in pool:
        res = B.AND(res, f)
    stats.update(peak_live=peak_live, peak_work=peak_work, sat=(res != 0),
                 early_unsat_step=unsat_at)
    if keep_result:
        stats["_result"] = res
    return stats


# ----------------------------------------------------------------- instances / ground truth
def base_points(E, l):
    pts = {}
    for x in range(1 << l):
        P = E.lift(x)
        if P:
            pts[x] = P
    return pts


def make_targets(E, l, m, rng, n_planted, n_random):
    F = E.F
    pts = base_points(E, l)
    xs = sorted(pts)
    out = []
    while len([1 for t in out if t["kind"] == "planted"]) < n_planted:
        ch = [rng.choice(xs) for _ in range(m)]
        Ps = [rng.choice(pts[x]) for x in ch]
        R = None
        degenerate = False
        for P in Ps:
            R = E.add(R, P)
            if R is None:  # partial sum at infinity: intermediate x undefined
                degenerate = True
                break
        if degenerate or R[0] == 0:
            continue
        assert E.on(R)
        out.append({"kind": "planted", "R": R, "planted_x": ch})
    while len([1 for t in out if t["kind"] == "random"]) < n_random:
        x = rng.randrange(1, 1 << F.n)
        P = E.lift(x)
        if not P:
            continue
        out.append({"kind": "random", "R": rng.choice(P), "planted_x": None})
    rng.shuffle(out)
    return out


def truth_m2(E, l, xR):
    sols = set()
    for x1 in range(1 << l):
        for x2 in range(1 << l):
            if E.S3(x1, x2, xR) == 0:
                sols.add((x1, x2))
    return sols


def umap_m3(E, l):
    """t -> list of (x1,x2) with S3(x1,x2,t)=0 (target independent)."""
    U = {}
    for x1 in range(1 << l):
        for x2 in range(1 << l):
            r = E.s3_roots_t(x1, x2)
            assert r is not None  # p=0,s=0 => b=0 impossible
            for t in set(r):
                assert E.S3(x1, x2, t) == 0
                U.setdefault(t, []).append((x1, x2))
    return U


def truth_m3(E, l, xR, U):
    """set of (x1,x2,x3) with a field element t: S3(x1,x2,t)=0 and S3(t,x3,xR)=0."""
    sols = set()
    for x3 in range(1 << l):
        r = E.s3_roots_t(x3, xR)
        for t in set(r):
            for (x1, x2) in U.get(t, ()):
                sols.add((x1, x2, x3))
    return sols


def truth_m3_exhaustive(E, l, xR):
    F = E.F
    sols = set()
    for t in range(1 << F.n):
        b3 = [x3 for x3 in range(1 << l) if E.S3(t, x3, xR) == 0]
        if not b3:
            continue
        for x1 in range(1 << l):
            for x2 in range(1 << l):
                if E.S3(x1, x2, t) == 0:
                    for x3 in b3:
                        sols.add((x1, x2, x3))
    return sols


def lifts_to_points(E, xs, R):
    """is there a sign choice of rational points with these x summing to R?"""
    import itertools
    cands = [E.lift(x) for x in xs]
    if any(not c for c in cands):
        return False
    for combo in itertools.product(*cands):
        S = None
        for P in combo:
            S = E.add(S, P)
        if S == R:
            return True
    return False


def decode(assign, level, l, m):
    xs = []
    for i in range(m):
        xs.append(sum(assign[level[("x", i, j)]] << j for j in range(l)))
    return tuple(xs)


def n_for(m, l):
    for p in (5, 7, 11, 13, 17, 19, 23):
        if p >= m * l:
            return p


# ----------------------------------------------------------------- validation
def validate():
    rng = random.Random(1)
    ok = True
    # 1. S3 vs real addition
    for n in (7, 11, 13):
        F = GF(n)
        E = Curve(F)
        cnt = 0
        while cnt < 200:
            x1, x2 = rng.randrange(1, 1 << n), rng.randrange(1, 1 << n)
            P, Q = E.lift(x1), E.lift(x2)
            if not P or not Q or x1 == x2:
                continue
            P, Q = P[0], Q[0]
            assert E.on(P) and E.on(Q)
            S, D = E.add(P, Q), E.add(P, E.neg(Q))
            assert E.on(S) and E.on(D)
            assert E.S3(x1, x2, S[0]) == 0 and E.S3(x1, x2, D[0]) == 0
            roots = set(E.s3_roots_t(x1, x2))
            assert roots == {S[0], D[0]}, (roots, S, D)
            cnt += 1
        # group law sanity: associativity
        for _ in range(50):
            Ps = []
            while len(Ps) < 3:
                c = E.lift(rng.randrange(1, 1 << n))
                if c:
                    Ps.append(c[0])
            assert E.add(E.add(Ps[0], Ps[1]), Ps[2]) == E.add(Ps[0], E.add(Ps[1], Ps[2]))
    print("S3 vs point addition: OK (n=7,11,13)")

    # 2/3. m=2 full solution sets
    for l in (3, 4):
        n = n_for(2, l)
        F = GF(n)
        E = Curve(F)
        tg = make_targets(E, l, 2, random.Random(100 + l), 3, 3)
        for oname in ("int", "blk", "rev"):
            level = order_m2(oname, l)
            for T in tg:
                B = BDD()
                cons = m2_constraints(B, F, l, level, T["R"][0])
                f = 1
                for c in cons:
                    f = B.AND(f, c)
                got = {decode(a, level, l, 2) for a in B.allsat(f, sorted(level.values()))}
                want = truth_m2(E, l, T["R"][0])
                assert got == want, (l, oname, got, want)
                if T["kind"] == "planted":
                    assert tuple(T["planted_x"]) in got
                    assert any(lifts_to_points(E, s, T["R"]) for s in got)
                st = bucket_eliminate(B, [(c, set(level.values())) for c in cons],
                                      sorted(level.values(), reverse=True))
                assert st["sat"] == bool(want)
    print("m=2 BDD solution sets == brute force (l=3,4; 3 orders; planted lift): OK")

    # m=3
    for l in (2, 3):
        n = n_for(3, l)
        F = GF(n)
        E = Curve(F)
        U = umap_m3(E, l)
        tg = make_targets(E, l, 3, random.Random(200 + l), 3, 3)
        for T in tg:
            want = truth_m3(E, l, T["R"][0], U)
            if l == 2 or T is tg[0]:
                assert want == truth_m3_exhaustive(E, l, T["R"][0])
            T["want"] = want
        for oname in ("xint_t", "xblk_t", "t_xint"):
            level = order_m3(oname, l, n)
            B = BDD()
            A = side_A_constraints(B, F, l, level)
            supA = {level[("x", i, j)] for i in (0, 1) for j in range(l)} | \
                   {level[("t", k)] for k in range(n)}
            supB = {level[("x", 2, j)] for j in range(l)} | {level[("t", k)] for k in range(n)}
            xl = sorted(level[("x", i, j)] for i in range(3) for j in range(l))
            for T in tg:
                Bc = side_B_constraints(B, F, l, level, T["R"][0])
                f = 1
                for c in Bc + A:
                    f = B.AND(f, c)
                for k in range(n):
                    f = B.exists(f, level[("t", k)])
                got = {decode(a, level, l, 3) for a in B.allsat(f, xl)}
                assert got == T["want"], (l, oname, len(got), len(T["want"]))
                if T["kind"] == "planted":
                    assert tuple(T["planted_x"]) in got
                    assert any(lifts_to_points(E, s, T["R"]) for s in got)
                for sname in ("t_first", "x_first", "x3_t_x12"):
                    pool = [(c, supA) for c in A] + [(c, supB) for c in Bc]
                    st = bucket_eliminate(B, pool, [level[q] for q in sched_m3(sname, l, n)])
                    assert st["sat"] == bool(T["want"]), (l, oname, sname)
    print("m=3 BDD solution sets == brute force (l=2,3; exhaustive-t check; 3 orders x 3 schedules; planted lift): OK")
    return ok


# ----------------------------------------------------------------- measured runs
def run_m3(l, ntargets, orders, scheds, out_rows, node_cap, time_cap, seed_base=20261114):
    n = n_for(3, l)
    seed = seed_base * 1000 + 300 + l
    F = GF(n)
    E = Curve(F)
    U = umap_m3(E, l)
    tg = make_targets(E, l, 3, random.Random(seed), ntargets // 2, ntargets - ntargets // 2)
    mgr = {}
    for oname in orders:
        level = order_m3(oname, l, n)
        B = BDD(node_cap=node_cap, time_cap=time_cap)
        t0 = time.time()
        try:
            A = side_A_constraints(B, F, l, level)
            mgr[oname] = (B, level, A, B.calls, time.time() - t0, B.mark(),
                          B.size(A), max(B.size([a]) for a in A))
            print(f"  [m3 l={l} n={n}] order {oname}: side A built, calls={B.calls} "
                  f"poolA={mgr[oname][6]} maxA={mgr[oname][7]} {time.time()-t0:.1f}s", flush=True)
        except Blowup as e:
            mgr[oname] = None
            print(f"  [m3 l={l} n={n}] order {oname}: side A BLOWUP {e} after {time.time()-t0:.1f}s", flush=True)
            for ti in range(len(tg)):
                for sname in scheds:
                    out_rows.append(dict(m=3, l=l, n=n, seed=seed, target=ti, kind=tg[ti]["kind"],
                                         order=oname, sched=sname, status="blowup_build_A",
                                         reason=str(e)))
    for ti, T in enumerate(tg):
        xR = T["R"][0]
        want = truth_m3(E, l, xR, U)
        nl = sum(1 for s in want if lifts_to_points(E, s, T["R"]))
        for oname in orders:
            if mgr[oname] is None:
                continue
            B, level, A, callsA, timeA, mark, poolA, maxA = mgr[oname]
            supA = {level[("x", i, j)] for i in (0, 1) for j in range(l)} | \
                   {level[("t", k)] for k in range(n)}
            supB = {level[("x", 2, j)] for j in range(l)} | {level[("t", k)] for k in range(n)}
            for sname in scheds:
                row = dict(m=3, l=l, n=n, seed=seed, target=ti, kind=T["kind"], xR=xR,
                           truth_solutions=len(want), truth_lifting=nl, truth_sat=bool(want),
                           order=oname, sched=sname, build_calls_A=callsA, build_time_A=timeA,
                           poolA=poolA, max_single_A=maxA)
                B.reset_run()
                t0 = time.time()
                try:
                    Bc = side_B_constraints(B, F, l, level, xR)
                    row["build_calls_B"] = B.calls
                    row["poolB"] = B.size(Bc)
                    B.calls = 0
                    pool = [(c, supA) for c in A] + [(c, supB) for c in Bc]
                    st = bucket_eliminate(B, pool, [level[q] for q in sched_m3(sname, l, n)])
                    row.update(st)
                    row["elim_calls"] = B.calls
                    row["status"] = "ok"
                    row["answer_matches_truth"] = (st["sat"] == bool(want))
                except Blowup as e:
                    row["status"] = "blowup"
                    row["reason"] = str(e)
                    row["nodes_at_abort"] = len(B.var)
                row["time_s"] = time.time() - t0
                B.rollback(mark)
                out_rows.append(row)
                print(f"  m3 l={l} tgt={ti}({T['kind'][0]},sols={len(want)}) {oname:7s} {sname:9s} "
                      f"{row['status']} peak_live={row.get('peak_live')} peak_work={row.get('peak_work')} "
                      f"calls={row.get('elim_calls')} {row['time_s']:.1f}s", flush=True)


def run_m2(l, ntargets, orders, out_rows, node_cap, time_cap, seed_base=20261114):
    n = n_for(2, l)
    seed = seed_base * 1000 + 200 + l
    F = GF(n)
    E = Curve(F)
    tg = make_targets(E, l, 2, random.Random(seed), ntargets // 2, ntargets - ntargets // 2)
    for ti, T in enumerate(tg):
        xR = T["R"][0]
        want = truth_m2(E, l, xR)
        for oname in orders:
            level = order_m2(oname, l)
            B = BDD(node_cap=node_cap, time_cap=time_cap)
            row = dict(m=2, l=l, n=n, seed=seed, target=ti, kind=T["kind"], xR=xR,
                       truth_solutions=len(want), truth_sat=bool(want), order=oname, sched="bottom_up")
            t0 = time.time()
            try:
                cons = m2_constraints(B, F, l, level, xR)
                row["build_calls"] = B.calls
                B.reset_run()
                st = bucket_eliminate(B, [(c, set(level.values())) for c in cons],
                                      sorted(level.values(), reverse=True))
                row.update(st)
                row["elim_calls"] = B.calls
                row["status"] = "ok"
                row["answer_matches_truth"] = (st["sat"] == bool(want))
            except Blowup as e:
                row["status"] = "blowup"
                row["reason"] = str(e)
            row["time_s"] = time.time() - t0
            out_rows.append(row)
            print(f"  m2 l={l} n={n} tgt={ti}({T['kind'][0]},sols={len(want)}) {oname:4s} {row['status']} "
                  f"peak_live={row.get('peak_live')} peak_work={row.get('peak_work')} "
                  f"calls={row.get('elim_calls')} {row['time_s']:.2f}s", flush=True)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("cmd", choices=["validate", "run"])
    ap.add_argument("--m", type=int, default=3)
    ap.add_argument("--l", type=int, nargs="+", default=[3])
    ap.add_argument("--targets", type=int, default=8)
    ap.add_argument("--orders", nargs="+", default=None)
    ap.add_argument("--scheds", nargs="+", default=["t_first", "x_first", "x3_t_x12"])
    ap.add_argument("--node-cap", type=int, default=4_000_000)
    ap.add_argument("--time-cap", type=float, default=300.0)
    ap.add_argument("--out", default=None)
    a = ap.parse_args()
    if a.cmd == "validate":
        validate()
        return
    rows = []
    for l in a.l:
        if a.m == 3:
            run_m3(l, a.targets, a.orders or ["xint_t", "xblk_t", "t_xint"], a.scheds, rows,
                   a.node_cap, a.time_cap)
        else:
            run_m2(l, a.targets, a.orders or ["int", "blk", "rev"], rows, a.node_cap, a.time_cap)
        if a.out:
            json.dump(rows, open(a.out, "w"), indent=0)


if __name__ == "__main__":
    main()
