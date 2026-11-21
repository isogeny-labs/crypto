#!/usr/bin/env python3
"""Follow-up pilot: subcube refutation for m = 3 point decomposition (see PROTOCOL.md).

Self-contained library: F_{2^n}, curve y^2 + xy = x^3 + x^2 + B, target generator with
brute-force ground truth, the S3-chain system as XOR rows over AND gates, two CNF/XOR
renderings for CryptoMiniSat (plain AND gates; dual-rail AND gates), and a small own
engine (Gauss-Jordan + AND-gate substitution, no learning).
"""
import os, re, sys, random, tempfile, ctypes, itertools

# ------------------------------------------------------------------ field
CELLS = {3: 11, 4: 13, 5: 17, 6: 19, 7: 23, 8: 29}  # l -> prime n (n >= 3l + 1)


class Field:
    def __init__(self, n):
        self.n = n
        self.poly = self._find_poly(n)
        self.sqm = [self.mul(1 << j, 1 << j) for j in range(n)]

    @staticmethod
    def _red(x, n, poly):
        while x >> n:
            x ^= poly << (x.bit_length() - 1 - n)
        return x

    def _find_poly(self, n):
        # lowest-weight (then lexicographically first) irreducible trinomial / pentanomial
        cands = [(1 << n) | (1 << a) | 1 for a in range(1, n)]
        cands += [(1 << n) | (1 << a) | (1 << b) | (1 << c) | 1
                  for a in range(3, n) for b in range(2, a) for c in range(1, b)]
        for f in cands:
            t = 2
            for _ in range(n):
                t = self._red(self._clmul(t, t), n, f)
            if t == 2:  # n prime, f(0) = f(1) = 1  => irreducible
                return f
        raise RuntimeError

    @staticmethod
    def _clmul(a, b):
        r = 0
        while b:
            if b & 1:
                r ^= a
            b >>= 1
            a <<= 1
        return r

    def mul(self, a, b):
        return self._red(self._clmul(a, b), self.n, self.poly)

    def sq(self, a):
        return self.mul(a, a)

    def inv(self, a):
        assert a
        r, t = 1, a
        for _ in range(self.n - 1):
            t = self.sq(t)
            r = self.mul(r, t)
        return r

    def tr(self, a):
        r, t = a, a
        for _ in range(self.n - 1):
            t = self.sq(t)
            r ^= t
        return r

    def htr(self, c):  # n odd
        r, t = c, c
        for _ in range((self.n - 1) // 2):
            t = self.sq(self.sq(t))
            r ^= t
        return r


class Curve:
    """y^2 + xy = x^3 + x^2 + B over F_{2^n}; affine points (x, y), None = infinity."""

    def __init__(self, F, B):
        self.F, self.B, self.A = F, B, 1

    def lift_x(self, x):
        F = self.F
        if x == 0:
            return None  # the 2-torsion point is excluded from the factor base
        c = x ^ self.A ^ F.mul(self.B, F.inv(F.sq(x)))
        if F.tr(c):
            return None
        z = F.htr(c)
        P = (x, F.mul(x, z))
        assert self.on(P)
        return P

    def on(self, P):
        F = self.F
        x, y = P
        return F.sq(y) ^ F.mul(x, y) == F.mul(F.sq(x), x) ^ F.mul(self.A, F.sq(x)) ^ self.B

    @staticmethod
    def neg(P):
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
            x3 = F.sq(lam) ^ lam ^ self.A
            return (x3, F.sq(x1) ^ F.mul(lam ^ 1, x3))
        lam = F.mul(y1 ^ y2, F.inv(x1 ^ x2))
        x3 = F.sq(lam) ^ lam ^ x1 ^ x2 ^ self.A
        return (x3, F.mul(lam, x1 ^ x3) ^ x3 ^ y1)


class Cell:
    """One (l, n) cell: field, curve, factor base V \\ {0}, V = span(1, t, .., t^(l-1))."""

    def __init__(self, l, seed):
        self.l, self.n, self.seed = l, CELLS[l], seed
        self.F = Field(self.n)
        rng = random.Random(seed)
        self.B = rng.randrange(2, 1 << self.n)
        self.E = Curve(self.F, self.B)
        self.base = [P for P in (self.E.lift_x(x) for x in range(1, 1 << l)) if P]
        self.signed = self.base + [self.E.neg(P) for P in self.base]
        self.S2 = set(self.E.add(a, b) for a in self.signed for b in self.signed)
        self.rng = rng

    def decomposable(self, R):
        """Ground truth: R = e1 P1 + e2 P2 + e3 P3, x(Pi) in V \\ {0}, by pair table."""
        return any(self.E.add(R, c) in self.S2 for c in self.signed)

    def brute(self, R):
        """Independent ground truth: plain enumeration of all signed triples."""
        E = self.E
        return any(E.add(E.add(a, b), c) == R
                   for a in self.signed for b in self.signed for c in self.signed)

    def lifts(self, xs, R):
        E = self.E
        pts = [E.lift_x(x) for x in xs]
        if any(p is None for p in pts):
            return False
        for sg in itertools.product((0, 1), repeat=3):
            S = None
            for p, g in zip(pts, sg):
                S = E.add(S, E.neg(p) if g else p)
            if S == R:
                return True
        return False

    def ok_target(self, R):
        return R is not None and R[0] != 0 and R[0] >> self.l != 0

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
            for _ in range(3):
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
            dec.append(self.planted_target())
        return non, dec

    def alt_basis(self, seed):
        """A second basis of the same V (random invertible change of coordinates)."""
        rng = random.Random(seed)
        l = self.l
        while True:
            bs = [rng.randrange(1, 1 << l) for _ in range(l)]
            piv = {}
            okb = True
            for v in bs:
                while v:
                    h = v.bit_length()
                    if h in piv:
                        v ^= piv[h]
                    else:
                        piv[h] = v
                        break
                if not v:
                    okb = False
            if okb and bs != [1 << i for i in range(l)]:
                return bs


# ------------------------------------------------------------------ system
class System:
    """S3 chain as XOR rows over variables and AND gates.

    Variables (1-based): c[l] coordinates of x1 in `basis`; u[n]; s[l]; p[2l-1]; z[l].
    Rows are ints: bit v = variable v, bit 0 = constant.  Gates: (t, sel, other), t = sel & other.
      (1) w^2 + xR w + xR^2 u^2 + xR^2 x1^2 + B = 0,  w = x1 u       [= S3(x1, xR, u)]
      (2) p^2 + (u p) + (u s)^2 + B = 0                              [= S3(x2, x3, u)]
      (3) z^2 + (s z) + p = 0                                        [x2 = z, x3 = s + z]
    Side clauses: x1 != 0 (OR c), p != 0 (OR p; excludes x2 = 0 and x3 = 0).
    """

    def __init__(self, cell, R, basis=None):
        F, l, n = cell.F, cell.l, cell.n
        self.cell, self.l, self.n = cell, l, n
        self.basis = basis or [1 << i for i in range(l)]
        self.nv = 0
        self.c, self.u, self.s = self.new(l), self.new(n), self.new(l)
        self.p, self.z = self.new(2 * l - 1), self.new(l)
        self.nbase = self.nv
        self.gates, self.gkey, self.rows = [], {}, []
        xR = R[0]
        xR2 = F.sq(xR)
        cb = list(zip(self.c, self.basis))
        tb = lambda vs: [(v, 1 << j) for j, v in enumerate(vs)]
        ub, sb, pb, zb = tb(self.u), tb(self.s), tb(self.p), tb(self.z)
        sqf = F.sq
        mulc = lambda k: (lambda e: F.mul(k, e))
        w = self.prod(cb, ub)
        e1 = self.xor(self.lin(sqf, w), self.lin(mulc(xR), w),
                      self.lin(lambda e: F.mul(xR2, F.sq(e)), self.elem(ub)),
                      self.lin(lambda e: F.mul(xR2, F.sq(e)), self.elem(cb)), self.const(cell.B))
        e2 = self.xor(self.lin(sqf, self.elem(pb)), self.prod(ub, pb),
                      self.lin(sqf, self.prod(ub, sb)), self.const(cell.B))
        e3 = self.xor(self.lin(sqf, self.elem(zb)), self.prod(sb, zb), self.elem(pb))
        self.stage_rows = []
        for st, e in enumerate((e1, e2, e3)):
            for f in e:
                if f:
                    self.rows.append(f)
                    self.stage_rows.append(st)
        self.ors = [list(self.c), list(self.p)]

    def new(self, k):
        r = list(range(self.nv + 1, self.nv + k + 1))
        self.nv += k
        return r

    def gate(self, sel, other):
        t = self.gkey.get((sel, other))
        if t is None:
            t = self.new(1)[0]
            self.gkey[(sel, other)] = t
            self.gates.append((t, sel, other))
        return t

    def const(self, k):
        return [(k >> i) & 1 for i in range(self.n)]

    def elem(self, vb):
        out = [0] * self.n
        for v, b in vb:
            for i in range(self.n):
                if (b >> i) & 1:
                    out[i] ^= 1 << v
        return out

    def lin(self, f, e):
        """Apply the F2-linear field map f to the symbolic element e (n forms)."""
        out = [0] * self.n
        for j, form in enumerate(e):
            if form:
                assert not form & 1
                m = f(1 << j)
                for i in range(self.n):
                    if (m >> i) & 1:
                        out[i] ^= form
        return out

    def prod(self, ab, bb):
        out = [0] * self.n
        F = self.cell.F
        for a, ea in ab:
            for b, eb in bb:
                t = self.gate(a, b)
                m = F.mul(ea, eb)
                for i in range(self.n):
                    if (m >> i) & 1:
                        out[i] ^= 1 << t
        return out

    def xor(self, *es):
        out = [0] * self.n
        for e in es:
            for i in range(self.n):
                out[i] ^= e[i]
        return out

    def x_of(self, model):
        """(x1, x2, x3) from a model indexable by variable number."""
        x1 = 0
        for v, b in zip(self.c, self.basis):
            if model[v]:
                x1 ^= b
        val = lambda vs: sum(1 << i for i, v in enumerate(vs) if model[v])
        z, s = val(self.z), val(self.s)
        return x1, z, s ^ z

    # ---- CNF/XOR renderings
    def render(self, enc):
        """enc 'plain': t = sel & other as 3 clauses.
        enc 'dual': additionally r = ~sel & other and the XOR t + r + other = 0, so that once
        sel is assigned one rail is a unit and the other is tied to `other` inside the XOR set."""
        clauses, xors = [list(o) for o in self.ors], []
        nv = self.nv
        for row in self.rows:
            vs = [i for i in range(1, row.bit_length()) if (row >> i) & 1]
            xors.append((vs, bool(row & 1)))
        for t, a, b in self.gates:
            clauses += [[-t, a], [-t, b], [t, -a, -b]]
            if enc == "dual":
                nv += 1
                r = nv
                clauses += [[-r, -a], [-r, b], [r, a, -b]]
                xors.append(([t, r, b], False))
        return nv, clauses, xors


# ------------------------------------------------------------------ CMS wrapper
_CAP = None
_LIBC = ctypes.CDLL(None)
SOLVE_RE = re.compile(r"c \[solve\] \d+ ret: (\S+) confl: (\d+)")
MAT_RE = re.compile(r"Using (\d+) matrices")


def capture_stdout():
    """Send fd 1 (CMS verbose lines) to a temp file; Python prints go to stderr."""
    global _CAP
    if _CAP is None:
        _CAP = tempfile.TemporaryFile(mode="w+b")
        sys.stdout.flush()
        os.dup2(_CAP.fileno(), 1)
        sys.stdout = sys.stderr


def _cap_pos():
    _LIBC.fflush(None)
    return os.lseek(_CAP.fileno(), 0, os.SEEK_END)


def _cap_read(pos):
    end = _cap_pos()
    data = os.pread(_CAP.fileno(), end - pos, pos).decode("latin1")
    if end > (1 << 26):
        os.ftruncate(_CAP.fileno(), 0)
        os.lseek(_CAP.fileno(), 0, os.SEEK_SET)
    return data


CMS_OPTS = {}


class CMS:
    def __init__(self, rendered, opts=None):
        import pycryptosat
        capture_stdout()
        nv, clauses, xors = rendered
        o = dict(CMS_OPTS)
        o.update(opts or {})
        self.s = pycryptosat.Solver(verbose=1, options=o or None)
        self.s.add_clauses(clauses)
        for vs, rhs in xors:
            self.s.add_xor_clause(vs, rhs)
        self.matrices = None
        self.last_out = ""

    def solve(self, assumptions=(), confl_limit=None):
        """-> (True/False/None, model, conflicts spent in this call)."""
        pos = _cap_pos()
        if confl_limit is None:
            sat, model = self.s.solve(list(assumptions))
        else:
            sat, model = self.s.solve(list(assumptions), confl_limit=int(confl_limit))
        data = _cap_read(pos)
        self.last_out = data
        m = SOLVE_RE.findall(data)
        assert m, data[-400:]
        mm = MAT_RE.findall(data)
        if mm:
            self.matrices = int(mm[-1])
        return sat, model, int(m[-1][1])


# ------------------------------------------------------------------ own engine
class Eng:
    """Gauss-Jordan over the XOR rows + AND-gate substitution, to fixpoint.  No learning.

    A gate t = a & b becomes the linear row t = 0, t = a or t = b as soon as Gauss-Jordan
    shows a or b constant (or a = b, a = ~b).  Unresolved gates leave t a free variable:
    this is the degree-2 linearisation of the system restricted to the current subcube.
    Work counters: xors (row additions), fix (fixpoint calls).
    """

    def __init__(self, sysm):
        self.sys = sysm
        self.rows = {}  # pivot var -> fully reduced row
        self.pend = list(sysm.gates)
        self.conflict = False
        self.xors = 0
        self.fix = 0
        for r in sysm.rows:
            self.add(r)
        self.fixpoint()

    def clone(self):
        e = Eng.__new__(Eng)
        e.sys, e.rows, e.pend = self.sys, dict(self.rows), list(self.pend)
        e.conflict, e.xors, e.fix = self.conflict, 0, 0
        return e

    def add(self, r):
        rows = self.rows
        m = r >> 1
        i = 1
        # reduce by pivots (rows are fully reduced, so one pass suffices)
        while m:
            low = (m & -m).bit_length()
            v = low + i - 1
            pr = rows.get(v)
            if pr is not None:
                r ^= pr
                self.xors += 1
            m >>= low
            i += low
        if r == 0:
            return
        if r == 1:
            self.conflict = True
            return
        pv = r.bit_length() - 1
        bit = 1 << pv
        for k, q in rows.items():
            if q & bit:
                rows[k] = q ^ r
                self.xors += 1
        rows[pv] = r

    def form(self, v):
        r = self.rows.get(v)
        return (1 << v) if r is None else r ^ (1 << v)

    def fixpoint(self):
        self.fix += 1
        changed = True
        while changed and not self.conflict:
            changed = False
            keep = []
            for g in self.pend:
                t, a, b = g
                fa, fb = self.form(a), self.form(b)
                if fa == 0 or fb == 0 or fa == fb ^ 1:
                    self.add(1 << t)
                elif fa == 1:
                    self.add((1 << t) ^ (1 << b))
                elif fb == 1 or fa == fb:
                    self.add((1 << t) ^ (1 << a))
                else:
                    keep.append(g)
                    continue
                changed = True
                if self.conflict:
                    return
            self.pend = keep
            for o in self.sys.ors:
                if all(self.form(v) == 0 for v in o):
                    self.conflict = True
                    return

    def assign(self, v, val):
        self.add((1 << v) | int(val))
        if not self.conflict:
            self.fixpoint()

    def free_var(self):
        """First base variable (order c, u, s, p, z) that is not constant, else None."""
        for v in range(1, self.sys.nbase + 1):
            if self.form(v) > 1:
                return v
        return None

    def model(self):
        return [None] + [self.form(v) & 1 for v in range(1, self.sys.nv + 1)]


def eng_solve_leafdown(e, stats):
    """Finish below a fully assigned x1 by branching on free base variables. -> model or None."""
    if e.conflict:
        return None
    v = e.free_var()
    if v is None:
        return e.model()
    for val in (0, 1):
        ch = e.clone()
        ch.assign(v, val)
        stats["sub_nodes"] += 1
        stats["xors"] += ch.xors
        stats["fix"] += ch.fix
        ch.xors = ch.fix = 0
        m = eng_solve_leafdown(ch, stats)
        if m is not None:
            return m
    return None


def eng_dfs(sysm, order):
    """DFS over x1's coordinates with the engine.  -> stats dict."""
    l = sysm.l
    stats = {"nodes": 0, "leaves": 0, "sub_nodes": 0, "xors": 0, "fix": 0,
             "prune_depth": [0] * (l + 1), "sat": False, "model": None}
    root = Eng(sysm)
    stats["xors"] += root.xors
    stats["fix"] += root.fix
    root.xors = root.fix = 0

    def rec(e, k):
        stats["nodes"] += 1
        if e.conflict:
            stats["prune_depth"][k] += 1
            return False
        if k == l:
            stats["leaves"] += 1
            m = eng_solve_leafdown(e, stats)
            if m is not None:
                stats["sat"], stats["model"] = True, m
                return True
            stats["prune_depth"][k] += 1
            return False
        for val in (0, 1):
            ch = e.clone()
            ch.assign(sysm.c[order[k]], val)
            stats["xors"] += ch.xors
            stats["fix"] += ch.fix
            ch.xors = ch.fix = 0
            if rec(ch, k + 1):
                return True
        return False

    rec(root, 0)
    return stats


# ------------------------------------------------------------------ CMS DFS
def budget_of(arm, j):
    """arm: ('abs', B) or ('prop', c);  j = dimension of the subcube."""
    kind, v = arm
    return int(v) if kind == "abs" else int(v * (1 << j))


def cms_dfs(sysm, rendered, order, arm, keep, opts=None):
    """DFS over x1's coordinates; at each node try to refute the subcube within the budget.
    keep=True: one incremental solver for the whole DFS; keep=False: fresh solver per node."""
    l = sysm.l
    stats = {"calls": 0, "leaves": 0, "conflicts": 0, "prune_depth": [0] * (l + 1),
             "sat": False, "model": None, "leaf_conflicts": 0, "leaf_unknown": 0}
    solver = CMS(rendered, opts) if keep else None

    def rec(assump, k):
        s = solver if keep else CMS(rendered, opts)
        j = l - k
        lim = None if k == l else budget_of(arm, j)
        stats["calls"] += 1
        sat, model, cf = s.solve(assump, lim)
        stats["conflicts"] += cf
        if k == l:
            stats["leaves"] += 1
            stats["leaf_conflicts"] += cf
        if sat is True:
            stats["sat"], stats["model"] = True, model
            return True
        if sat is False:
            stats["prune_depth"][k] += 1
            return False
        assert k < l
        v = sysm.c[order[k]]
        for lit in (-v, v):
            if rec(assump + [lit], k + 1):
                return True
        return False

    rec([], 0)
    return stats
