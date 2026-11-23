#!/usr/bin/env python3
"""Lead A: dual-rail encoding of m = 4 point decomposition (see PROTOCOL.md).

Reuses Field/Curve/CMS wrapper from sub.py (copied unchanged from
../../sat_smt_leads_20261114/followup-subcube/sub.py).  New here: fields of non-prime odd
degree, the m = 4 cell with ground truth, the 4-stage chain system, DFS over (x1, x2).

Chain (binary S3(a, b, c) = (ab + ac + bc)^2 + abc + B):
  d = x1 + x2, q = x1 * x2                       (l and 2l-1 bits, polynomial product, no reduction)
  (1) q^2 + (u q) + (u d)^2 + B = 0              S3(x1, x2, u):  u = x(P1 +- P2)
  (2) (uv)^2 + xR (uv) + xR^2 (u^2 + v^2) + B = 0   S3(u, xR, v):  v = x(R -+ Q)
  (3) p^2 + (v p) + (v s)^2 + B = 0              S3(x3, x4, v),  s = x3 + x4, p = x3 x4
  (4) z^2 + (s z) + p = 0                        x3 = z, x4 = s + z
Side clauses: x1 != 0, x2 != 0, p != 0, u != 0, v != 0, and x1 <= x2 (lex, MSB = coordinate l-1).

Soundness at 0 / infinity (the defect of the m = 3 pilot):
  * the cell's B is drawn so that sqrt(B) is not a product of two non-zero elements of V.  Then
    S3(a, b, 0) = (ab)^2 + B has no zero with a, b in V, so u = 0 and v = 0 (intermediate sum =
    the 2-torsion point T, the only way two twist points sum to a rational point) cannot occur in
    a true decomposition either; the clauses u != 0, v != 0 are implied, kept for propagation.
  * targets satisfy R != O, x(R) != 0, x(R) not in V and R not a sum of <= 2 signed base points,
    so in a true decomposition neither Q = +-P1 +- P2 nor R - Q is the point at infinity.
"""
import os, sys, random, itertools
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import sub
from sub import Curve, CMS

N_OF_L = {2: 9, 3: 13, 4: 17, 5: 21, 6: 25, 7: 29}   # n = 4l + 1


def _pmod(a, f):
    df = f.bit_length()
    while a.bit_length() >= df:
        a ^= f << (a.bit_length() - df)
    return a


def _pgcd(a, b):
    while b:
        a, b = b, _pmod(a, b)
    return a


class Field(sub.Field):
    """F_{2^n}, any odd n (Rabin irreducibility test instead of the prime-n shortcut)."""

    def _find_poly(self, n):
        cands = [(1 << n) | (1 << a) | 1 for a in range(1, n)]
        cands += [(1 << n) | (1 << a) | (1 << b) | (1 << c) | 1
                  for a in range(3, n) for b in range(2, a) for c in range(1, b)]
        primes = [q for q in range(2, n + 1) if n % q == 0 and all(q % r for r in range(2, q))]
        for f in cands:
            pw = [2]
            for _ in range(n):
                pw.append(self._red(self._clmul(pw[-1], pw[-1]), n, f))
            if pw[n] != 2:
                continue
            if all(_pgcd(f, pw[n // q] ^ 2) == 1 for q in primes):
                return f
        raise RuntimeError

    def sqrt(self, a):
        for _ in range(self.n - 1):
            a = self.sq(a)
        return a


class Cell4:
    """(l, n = 4l+1) cell: field, curve, factor base {P : x(P) in V \\ {0}}, V = span(1..t^(l-1))."""

    def __init__(self, l, seed):
        self.l, self.n, self.seed = l, N_OF_L[l], seed
        F = self.F = Field(self.n)
        rng = self.rng = random.Random(seed)
        VV = set(F.mul(a, b) for a in range(1, 1 << l) for b in range(a, 1 << l))
        self.B_rejected = 0
        while True:
            B = rng.randrange(2, 1 << self.n)
            if F.sqrt(B) not in VV:
                self.B = B
                self.E = Curve(F, B)
                self.base = [P for P in (self.E.lift_x(x) for x in range(1, 1 << l)) if P]
                if len(self.base) >= (1 << (l - 2)) + 1:     # about half the expected size
                    break
            self.B_rejected += 1
        self.signed = self.base + [self.E.neg(P) for P in self.base]
        E = self.E
        self.S2 = set(E.add(a, b) for a in self.signed for b in self.signed)  # contains None
        self.S2nz = self.S2 - {None}
        assert all(Q[0] != 0 for Q in self.S2nz)        # no pair sums to the 2-torsion point

    def sub_pt(self, R, Q):
        return self.E.add(R, self.E.neg(Q))

    def decomposable(self, R):
        """Truth by pair table: R = Q1 + Q2, Qi non-zero sums of two signed base points."""
        return any(self.sub_pt(R, Q) in self.S2nz for Q in self.S2nz)

    def brute(self, R):
        """Independent truth: every signed quadruple, left-to-right sums."""
        E = self.E
        for a in self.signed:
            for b in self.signed:
                ab = E.add(a, b)
                for c in self.signed:
                    abc = E.add(ab, c)
                    for d in self.signed:
                        if E.add(abc, d) == R:
                            return True
        return False

    def pair_ok(self, R, x1, x2):
        """Truth for one leaf: is there a decomposition using x-coordinates x1, x2 for P1, P2?"""
        E = self.E
        P1, P2 = E.lift_x(x1), E.lift_x(x2)
        if P1 is None or P2 is None:
            return False
        for Q in (E.add(P1, P2), E.add(P1, E.neg(P2)), E.add(E.neg(P1), P2),
                  E.add(E.neg(P1), E.neg(P2))):
            if Q is not None and self.sub_pt(R, Q) in self.S2nz:
                return True
        return False

    def lifts(self, xs, R):
        E = self.E
        pts = [E.lift_x(x) for x in xs]
        if any(p is None for p in pts):
            return False
        for sg in itertools.product((0, 1), repeat=len(xs)):
            S = None
            for p, g in zip(pts, sg):
                S = E.add(S, E.neg(p) if g else p)
            if S == R:
                return True
        return False

    def ok_target(self, R):
        return (R is not None and R[0] != 0 and R[0] >> self.l != 0 and R not in self.S2)

    def random_target(self):
        while True:
            x = self.rng.randrange(1, 1 << self.n)
            P = self.E.lift_x(x)
            if P is None:
                continue
            if self.rng.getrandbits(1):
                P = self.E.neg(P)
            if self.ok_target(P):
                return P

    def planted_target(self):
        while True:
            R = None
            for _ in range(4):
                R = self.E.add(R, self.rng.choice(self.signed))
            if self.ok_target(R):
                return R

    def targets(self, n_non, n_dec):
        non, dec = [], []
        while len(non) < n_non:
            R = self.random_target()
            if not self.decomposable(R):
                non.append(R)
        while len(dec) < n_dec:
            R = self.planted_target()
            if R not in dec:
                dec.append(R)
        return non, dec


class System4(sub.System):
    def __init__(self, cell, R):
        F, l, n = cell.F, cell.l, cell.n
        self.cell, self.l, self.n = cell, l, n
        self.nv = 0
        self.c1, self.c2, self.d, self.q = self.new(l), self.new(l), self.new(l), self.new(2 * l - 1)
        self.u, self.v = self.new(n), self.new(n)
        self.s, self.p, self.z = self.new(l), self.new(2 * l - 1), self.new(l)
        self.lex = self.new(l)
        self.nbase = self.nv
        self.gates, self.gkey, self.rows = [], {}, []
        xR = R[0]
        xR2 = F.sq(xR)
        tb = lambda vs: [(v, 1 << j) for j, v in enumerate(vs)]
        c1b, c2b, db, qb = tb(self.c1), tb(self.c2), tb(self.d), tb(self.q)
        ub, vb, sb, pb, zb = tb(self.u), tb(self.v), tb(self.s), tb(self.p), tb(self.z)
        sqf = F.sq
        e0a = [(1 << self.d[i]) | (1 << self.c1[i]) | (1 << self.c2[i]) for i in range(l)]
        e0b = self.xor(self.prod(c1b, c2b), self.elem(qb))
        e1 = self.xor(self.lin(sqf, self.elem(qb)), self.prod(qb, ub),
                      self.lin(sqf, self.prod(db, ub)), self.const(cell.B))
        w = self.prod(ub, vb)
        k2 = lambda e: F.mul(xR2, F.sq(e))
        e2 = self.xor(self.lin(sqf, w), self.lin(lambda e: F.mul(xR, e), w),
                      self.lin(k2, self.elem(ub)), self.lin(k2, self.elem(vb)), self.const(cell.B))
        e3 = self.xor(self.lin(sqf, self.elem(pb)), self.prod(vb, pb),
                      self.lin(sqf, self.prod(vb, sb)), self.const(cell.B))
        e4 = self.xor(self.lin(sqf, self.elem(zb)), self.prod(sb, zb), self.elem(pb))
        for e in (e0a, e0b, e1, e2, e3, e4):
            for f in e:
                assert f != 1
                if f:
                    self.rows.append(f)
        self.ors = [list(self.c1), list(self.c2), list(self.p), list(self.u), list(self.v)]
        # lex x1 <= x2, MSB = coordinate l-1.  lex[i] <=> coordinates i..l-1 of x1 and x2 agree.
        cl = []
        for i in range(l - 1, -1, -1):
            up = self.lex[i + 1] if i + 1 < l else None
            pre = [] if up is None else [-up]
            cl.append(pre + [-self.c1[i], self.c2[i]])
            cl.append([-self.lex[i], -self.d[i]])
            if up is not None:
                cl.append([-self.lex[i], up])
            cl.append(pre + [self.d[i], self.lex[i]])
        self.extra = cl

    def render(self, enc="dual"):
        """'plain', 'dual' (one rail r = ~sel & other, XOR t + r + other, as in the m = 3 pilot),
        'dual2' (additionally r2 = sel & ~other, XOR t + r2 + sel: linear whichever input is set)."""
        nv, clauses, xors = sub.System.render(self, "dual" if enc == "dual2" else enc)
        if enc == "dual2":
            for t, a, b in self.gates:
                nv += 1
                r = nv
                clauses += [[-r, a], [-r, -b], [r, -a, b]]
                xors.append(([t, r, a], False))
        return nv, clauses + [list(c) for c in self.extra], xors

    def x_of(self, model):
        val = lambda vs: sum(1 << i for i, v in enumerate(vs) if model[v])
        z, s = val(self.z), val(self.s)
        return val(self.c1), val(self.c2), z, s ^ z

    def assump(self, x1, x2):
        l = self.l
        return ([self.c1[k] if (x1 >> k) & 1 else -self.c1[k] for k in range(l)] +
                [self.c2[k] if (x2 >> k) & 1 else -self.c2[k] for k in range(l)])


# ------------------------------------------------------------------ DFS over (x1, x2)
LEAFCAP = 200000      # safety cap per leaf call; a capped leaf censors the run


class Censored(Exception):
    pass


def order_of(l, kind):
    """Decision order: list of (which, coordinate).  'il': interleaved MSB first; 'blk': x1 then x2."""
    if kind == "il":
        return [(w, i) for i in range(l - 1, -1, -1) for w in (0, 1)]
    return [(0, i) for i in range(l - 1, -1, -1)] + [(1, i) for i in range(l - 1, -1, -1)]


def driver_skip(l, a):
    """a[w][i] in (None, 0, 1).  True if the driver can discard the node without the solver:
    x1 > x2 certain (symmetry), or x1 = 0 / x2 = 0 fully assigned (not in the factor base)."""
    for w in (0, 1):
        if all(b == 0 for b in a[w]):
            return True
    for i in range(l - 1, -1, -1):
        b1, b2 = a[0][i], a[1][i]
        if b1 is None or b2 is None:
            return False
        if b1 != b2:
            return b1 > b2
    return False


def cms_dfs4(S, rendered, order, budget, keep, opts=None):
    """DFS over the 2l coordinates of (x1, x2).  At an internal node (budget > 0) the solver may
    refute the subcube within `budget` conflicts; at a leaf it solves without limit.
    keep: one incremental solver; else a fresh solver per solver call (discard control)."""
    l = S.l
    D = 2 * l
    st = {"calls": 0, "leaves": 0, "conflicts": 0, "leaf_conflicts": 0, "leaf_zero": 0,
          "leaf_max": 0, "skipped_nodes": 0, "prune_depth": [0] * (D + 1),
          "calls_depth": [0] * (D + 1), "confl_depth": [0] * (D + 1), "sat": False, "model": None}
    solver = CMS(rendered, opts) if keep else None
    a = [[None] * l, [None] * l]
    cv = (S.c1, S.c2)

    def rec(assump, k):
        if driver_skip(l, a):
            st["skipped_nodes"] += 1
            return False
        leaf = k == D
        if leaf or budget > 0:
            s = solver if keep else CMS(rendered, opts)
            st["calls"] += 1
            st["calls_depth"][k] += 1
            sat, model, cf = s.solve(assump, LEAFCAP if leaf else budget)
            st["conflicts"] += cf
            if leaf and sat is None:
                st["leaves"] += 1
                raise Censored
            st["confl_depth"][k] += cf
            if leaf:
                st["leaves"] += 1
                st["leaf_conflicts"] += cf
                st["leaf_zero"] += cf == 0
                st["leaf_max"] = max(st["leaf_max"], cf)
            if sat is True:
                st["sat"], st["model"] = True, model
                return True
            if sat is False:
                st["prune_depth"][k] += 1
                return False
            assert not leaf
        w, i = order[k]
        var = cv[w][i]
        for bit in (0, 1):
            a[w][i] = bit
            r = rec(assump + [var if bit else -var], k + 1)
            a[w][i] = None
            if r:
                return True
        return False

    try:
        rec([], 0)
    except Censored:
        st["censored"] = True
    return st


def core_scan(S, rendered, opts=None):
    """Decision-restricted learning, emulated: leaves in lex order of (x1, x2); after an UNSAT
    leaf the solver's final conflict (a subset of the 2l assumption literals) is kept as a nogood
    over the coordinates of (x1, x2), and later leaves covered by a nogood are not solved."""
    l = S.l
    st = {"leaves": 0, "conflicts": 0, "covered": 0, "nogoods": 0, "core_len_sum": 0,
          "core_len_min": 2 * l, "sat": False, "model": None}
    solver = CMS(rendered, opts)
    pos = {}
    for k in range(l):
        pos[S.c1[k]] = k
        pos[S.c2[k]] = l + k
    nogoods = []
    for x1 in range(1, 1 << l):
        for x2 in range(x1, 1 << l):
            key = x1 | (x2 << l)
            if any((key & m) == val for m, val in nogoods):
                st["covered"] += 1
                continue
            asm = S.assump(x1, x2)
            sat, model, cf = solver.solve(asm, LEAFCAP)
            st["leaves"] += 1
            st["conflicts"] += cf
            if sat is None:
                st["censored"] = True
                return st
            if sat:
                st["sat"], st["model"] = True, model
                return st
            core = solver.s.get_conflict()
            m = val = 0
            for lit in core:      # get_conflict returns negated assumptions
                b = pos[abs(lit)]
                m |= 1 << b
                if lit < 0:
                    val |= 1 << b
            assert (key & m) == val, (core, asm)
            nogoods.append((m, val))
            st["nogoods"] += 1
            st["core_len_sum"] += len(core)
            st["core_len_min"] = min(st["core_len_min"], len(core))
    return st
