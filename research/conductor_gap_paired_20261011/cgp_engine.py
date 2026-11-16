"""Point-decomposition engines over F_{p^2} (Gaudry-Diem, n = 2) and relation accounting.

Task: given R in <G> (G of prime order r), find R = e1*P1 + e2*P2 with P_i in the factor
base F = {P in E(F_q) : x(P) in F_p}.  Semaev:
    S_3(x1, x2, X3) = (x1-x2)^2 X3^2 - 2((x1+x2)(x1 x2 + a) + 2b) X3 + (x1 x2 - a)^2 - 4b(x1+x2)
vanishes iff P1 +- P2 +- P3 = O for points with those abscissae.  With e1 = x1+x2,
e2 = x1*x2 (symmetrisation) and X3 = u0 + u1*theta, S_3 = 0 is two quadrics over F_p in
(e1, e2): the Weil restriction.

Engines
  exhaustive : all unordered pairs of the factor base, S_3 evaluated in F_q.
  weil_gb    : the two quadrics; Buchberger Groebner basis in pure Python (NOT F4/F5, NOT SAT):
               grevlex run for the observed solving degree, lex run to read off solutions.

Python prototype under a task-level exception to the no-Python rule (PROTOCOL.md s.1).
"""
from __future__ import annotations

import hashlib
from math import gcd

from cgp_field import Fq
from cgp_curve import Curve


# ------------------------------------------------------------------ Semaev S_3
def s3_value(F: Fq, a: int, b: int, x1: int, x2: int, x3: int) -> int:
    """S_3(x1, x2, x3) in F_q (direct formula, used by the exhaustive engine and the tests)."""
    c = F.const
    d = F.sub(x1, x2)
    t1 = F.mul(F.sq(d), F.sq(x3))
    s = F.add(x1, x2)
    pr = F.mul(x1, x2)
    t2 = F.mul(F.mul(c(2), F.add(F.mul(s, F.add(pr, a)), F.mul(c(2), b))), x3)
    t3 = F.sub(F.sq(F.sub(pr, a)), F.mul(c(4), F.mul(b, s)))
    return F.add(F.sub(t1, t2), t3)


def s3_quadrics(F: Fq, a: int, b: int, X3: int):
    """Coefficients of S_3 in (e1, e2) as {(i, j): c} (monomial e1^i e2^j, c in F_q):
    X3^2 e1^2 - 2 X3 e1 e2 + e2^2 - (2 a X3 + 4 b) e1 - (4 X3^2 + 2 a) e2 + a^2 - 4 b X3."""
    c = F.const
    X2 = F.sq(X3)
    return {
        (2, 0): X2,
        (1, 1): F.neg(F.mul(c(2), X3)),
        (0, 2): 1,
        (1, 0): F.neg(F.add(F.mul(c(2), F.mul(a, X3)), F.mul(c(4), b))),
        (0, 1): F.neg(F.add(F.mul(c(4), X2), F.mul(c(2), a))),
        (0, 0): F.sub(F.sq(a), F.mul(c(4), F.mul(b, X3))),
    }


def weil_restrict(F: Fq, a: int, b: int, X3: int):
    """The two F_p-quadrics (f0, f1) in (e1, e2): coordinates of S_3 over 1 and theta."""
    f0, f1 = {}, {}
    for m, cq in s3_quadrics(F, a, b, X3).items():
        c0, c1 = F.comps(cq)
        if c0:
            f0[m] = c0
        if c1:
            f1[m] = c1
    return f0, f1


# --------------------------------------------------------------- Groebner basis
class Buchberger:
    """Buchberger's algorithm in F_p[x, y] with the normal selection strategy and the
    product/chain criteria.  Monomials are (i, j) = x^i y^j."""

    def __init__(self, p: int, order: str):
        assert order in ("grevlex", "lex")
        self.p = p
        self.order = order
        self.key = (lambda m: (m[0] + m[1], -m[1])) if order == "grevlex" else (lambda m: (m[0], m[1]))
        self.ops = 0                 # coefficient multiplications
        self.max_spoly_deg = 0       # largest lcm degree of an S-pair actually processed
        self.max_elem_deg = 0        # largest total degree of a polynomial entering the basis
        self.spairs = 0
        self.zero_reductions = 0

    def lm(self, f):
        return max(f, key=self.key)

    def monic(self, f):
        m = self.lm(f)
        inv = pow(f[m], -1, self.p)
        self.ops += len(f)
        return {k: (v * inv) % self.p for k, v in f.items()}

    def reduce(self, f, G):
        p, key = self.p, self.key
        f = dict(f)
        rem = {}
        lms = [self.lm(g) for g in G]
        while f:
            m = max(f, key=key)
            c = f[m]
            for g, gm in zip(G, lms):
                if m[0] >= gm[0] and m[1] >= gm[1]:
                    s0, s1 = m[0] - gm[0], m[1] - gm[1]
                    for gmon, gc in g.items():
                        mm = (gmon[0] + s0, gmon[1] + s1)
                        nv = (f.get(mm, 0) - c * gc) % p
                        if nv:
                            f[mm] = nv
                        else:
                            f.pop(mm, None)
                    self.ops += len(g)
                    break
            else:
                rem[m] = c
                del f[m]
        return rem

    def groebner(self, F):
        p = self.p
        G: list[dict] = []
        pending: set[tuple[int, int]] = set()

        def lcm(m1, m2):
            return (max(m1[0], m2[0]), max(m1[1], m2[1]))

        def add(h):
            h = self.monic(h)
            self.max_elem_deg = max(self.max_elem_deg, max(a + b for a, b in h))
            G.append(h)
            k = len(G) - 1
            for i in range(k):
                pending.add((i, k))

        for f in F:
            if not f:
                continue
            h = self.reduce(f, G) if G else dict(f)
            if h:
                add(h)
        while pending:
            best = None
            for (i, j) in pending:
                l = lcm(self.lm(G[i]), self.lm(G[j]))
                d = l[0] + l[1]
                cand = (d, self.key(l), i, j)
                if best is None or cand < best:
                    best = cand
            _, _, i, j = best
            pending.discard((i, j))
            mi, mj = self.lm(G[i]), self.lm(G[j])
            l = lcm(mi, mj)
            if l == (mi[0] + mj[0], mi[1] + mj[1]):  # product criterion: coprime leads
                continue
            skip = False
            for k in range(len(G)):  # chain criterion
                if k in (i, j):
                    continue
                mk = self.lm(G[k])
                if l[0] >= mk[0] and l[1] >= mk[1] and \
                        (min(i, k), max(i, k)) not in pending and (min(j, k), max(j, k)) not in pending:
                    skip = True
                    break
            if skip:
                continue
            self.spairs += 1
            self.max_spoly_deg = max(self.max_spoly_deg, l[0] + l[1])
            s: dict = {}
            for gmon, gc in G[i].items():
                mm = (gmon[0] + l[0] - mi[0], gmon[1] + l[1] - mi[1])
                s[mm] = (s.get(mm, 0) + gc) % p
            for gmon, gc in G[j].items():
                mm = (gmon[0] + l[0] - mj[0], gmon[1] + l[1] - mj[1])
                s[mm] = (s.get(mm, 0) - gc) % p
            s = {k: v for k, v in s.items() if v}
            self.ops += len(G[i]) + len(G[j])
            h = self.reduce(s, G) if s else {}
            if h:
                add(h)
            else:
                self.zero_reductions += 1
        return self._reduced(G)

    def _reduced(self, G):
        lms = [self.lm(g) for g in G]
        keep = []
        for i, g in enumerate(G):
            red = False
            for k, gm in enumerate(lms):
                if k == i:
                    continue
                if lms[i][0] >= gm[0] and lms[i][1] >= gm[1] and (lms[i] != gm or k < i):
                    red = True
                    break
            if not red:
                keep.append(g)
        out = []
        for i, g in enumerate(keep):
            others = keep[:i] + keep[i + 1:]
            h = self.reduce(g, others) if others else g
            out.append(self.monic(h) if h else h)
        return [h for h in out if h]

    def quotient_dim(self, G):
        """Number of standard monomials (None if the ideal is not zero-dimensional)."""
        lms = [self.lm(g) for g in G]
        if not lms:
            return None
        ax = min((m[0] for m in lms if m[1] == 0), default=None)
        ay = min((m[1] for m in lms if m[0] == 0), default=None)
        if ax is None or ay is None:
            return None
        return sum(1 for i in range(ax) for j in range(ay)
                   if not any(i >= m[0] and j >= m[1] for m in lms))


def _eval_univ(coefs, x, p):
    acc = 0
    for c in reversed(coefs):
        acc = (acc * x + c) % p
    return acc


def _univ_roots(coefs, p):
    """Roots in F_p of a univariate polynomial (low degree first) by evaluation (p is tiny)."""
    if not any(coefs):
        return None  # identically zero
    return [x for x in range(p) if _eval_univ(coefs, x, p) == 0]


def solve_zero_dim(G_lex, p):
    """F_p-rational solutions (x, y) of a lex Groebner basis (x > y); None if not zero-dim."""
    if not G_lex:
        return None
    if G_lex == [{(0, 0): 1}]:
        return []
    uni = [g for g in G_lex if all(m[0] == 0 for m in g)]
    if not uni:
        return None
    gy = min(uni, key=lambda g: max(m[1] for m in g))
    dy = max(m[1] for m in gy)
    coefs = [gy.get((0, k), 0) for k in range(dy + 1)]
    sols = []
    for y0 in _univ_roots(coefs, p) or []:
        polys = []
        for g in G_lex:
            dx = max(m[0] for m in g)
            cx = [0] * (dx + 1)
            for (i, j), c in g.items():
                cx[i] = (cx[i] + c * pow(y0, j, p)) % p
            polys.append(cx)
        for x0 in range(p):
            if all(_eval_univ(cx, x0, p) == 0 for cx in polys):
                sols.append((x0, y0))
    return sols


def roots_of_quadratic(e1: int, e2: int, p: int, sqrt_table: dict[int, int]):
    """x1 <= x2 in F_p with x1 + x2 = e1, x1 x2 = e2 (the roots of z^2 - e1 z + e2), or None."""
    disc = (e1 * e1 - 4 * e2) % p
    s = sqrt_table.get(disc)
    if s is None:
        return None
    inv2 = pow(2, -1, p)
    x1 = ((e1 - s) * inv2) % p
    x2 = ((e1 + s) * inv2) % p
    return (min(x1, x2), max(x1, x2))


# ----------------------------------------------------------------- instances
class Instance:
    """One curve with its truncated factor base and the data both engines share."""

    def __init__(self, F: Fq, a: int, b: int, M: int):
        self.F, self.a, self.b = F, a, b
        self.E = Curve(F, a, b)
        self.p = F.p
        self.sqrt_fp = {}
        for z in range(F.p):
            self.sqrt_fp.setdefault((z * z) % F.p, z)
        # full factor base (x-classes with a rational point), in increasing integer x
        self.full = []
        for x in range(F.p):
            v = self.E.f(x)
            y = F.sqrt(v)
            if y is not None:
                self.full.append((x, min(y, F.neg(y))))
        self.full_size = len(self.full)
        self.M = M
        assert M <= self.full_size
        self.base = self.full[:M]
        self.base_y = dict(self.base)
        self.rational_x = {x for x, _ in self.full}
        self._pair_consts = None

    def pair_constants(self):
        """(A, B, C) of S_3 = A X3^2 + B X3 + C for every unordered pair of base classes."""
        if self._pair_consts is None:
            F, a, b = self.F, self.a, self.b
            c = F.const
            out = []
            for i, (x1, _) in enumerate(self.base):
                for (x2, _) in self.base[i:]:
                    s = F.add(x1, x2)
                    pr = F.mul(x1, x2)
                    A = F.sq(F.sub(x1, x2))
                    B = F.neg(F.mul(c(2), F.add(F.mul(s, F.add(pr, a)), F.mul(c(2), b))))
                    C = F.sub(F.sq(F.sub(pr, a)), F.mul(c(4), F.mul(b, s)))
                    out.append((x1, x2, A, B, C))
            self._pair_consts = out
        return self._pair_consts


# ------------------------------------------------------------------ engines
def decompose_exhaustive(inst: Instance, X3: int):
    """In-base unordered pairs (x1 <= x2) with S_3(x1, x2, X3) = 0.  Returns (pairs, metrics)."""
    F = inst.F
    X2 = F.sq(X3)
    sols = []
    n = 0
    for (x1, x2, A, B, C) in inst.pair_constants():
        n += 1
        if F.add(F.add(F.mul(A, X2), F.mul(B, X3)), C) == 0:
            sols.append((x1, x2))
    return {"in_base": sols, "outside_base": [], "not_rational": []}, {"pair_evals": n}


def decompose_weil_gb(inst: Instance, X3: int):
    """Weil-restricted symmetrised S_3 solved with Groebner bases.  Returns (classified
    solutions, metrics).  Solutions with x_i in F_p are classified as in-base, rational but
    outside the truncated base, or not rational over F_q (invalid)."""
    F, p = inst.F, inst.p
    f0, f1 = weil_restrict(F, inst.a, inst.b, X3)
    grev = Buchberger(p, "grevlex")
    Ggrev = grev.groebner([f0, f1])
    qdim = grev.quotient_dim(Ggrev)
    lex = Buchberger(p, "lex")
    Glex = lex.groebner([f0, f1])
    sols_e = solve_zero_dim(Glex, p)
    metrics = {
        "variables": 2, "equations": 2, "input_degrees": [max((i + j) for i, j in f) if f else 0 for f in (f0, f1)],
        "quotient_dim": qdim,
        "solving_degree": max(2, grev.max_spoly_deg, grev.max_elem_deg),
        "grevlex_max_spoly_deg": grev.max_spoly_deg, "grevlex_max_elem_deg": grev.max_elem_deg,
        "grevlex_spairs": grev.spairs, "grevlex_ops": grev.ops,
        "lex_max_spoly_deg": lex.max_spoly_deg, "lex_spairs": lex.spairs, "lex_ops": lex.ops,
        "ops": grev.ops + lex.ops,
        "non_zero_dim": sols_e is None,
        "eliminant_degree": None,
    }
    in_base, outside, not_rat = [], [], []
    for (e1, e2) in (sols_e or []):
        xs = roots_of_quadratic(e1, e2, p, inst.sqrt_fp)
        if xs is None:
            continue  # x1, x2 not in F_p: not a factor-base decomposition
        x1, x2 = xs
        ok1, ok2 = x1 in inst.rational_x, x2 in inst.rational_x
        if not (ok1 and ok2):
            not_rat.append((x1, x2))
        elif x1 in inst.base_y and x2 in inst.base_y:
            in_base.append((x1, x2))
        else:
            outside.append((x1, x2))
    return {"in_base": sorted(in_base), "outside_base": sorted(outside),
            "not_rational": sorted(not_rat)}, metrics


# ------------------------------------------------------- relations and rank
class Echelon:
    """Incremental row echelon form over F_r (r prime)."""

    def __init__(self, r: int, ncols: int):
        self.r, self.ncols = r, ncols
        self.basis: dict[int, list[int]] = {}
        self.rank = 0
        self.seen: set[tuple] = set()

    def add(self, row: list[int]) -> str:
        """'independent', 'dependent' or 'duplicate' (identical to an earlier row)."""
        r = self.r
        key = tuple(x % r for x in row)
        if key in self.seen:
            return "duplicate"
        self.seen.add(key)
        v = list(key)
        for col in range(self.ncols):
            if v[col] == 0:
                continue
            if col in self.basis:
                c = v[col]
                bv = self.basis[col]
                for k in range(col, self.ncols):
                    if bv[k]:
                        v[k] = (v[k] - c * bv[k]) % r
            else:
                inv = pow(v[col], -1, r)
                self.basis[col] = [(x * inv) % r for x in v]
                self.rank += 1
                return "independent"
        return "dependent"


def verify_relation(E: Curve, R, P1, P2):
    """Exact check: return (e1, e2) in {+-1}^2 with e1*P1 + e2*P2 == R by the affine group law,
    or None.  P1, P2 must be affine points on the curve."""
    if not (E.on_curve(P1) and E.on_curve(P2) and E.on_curve(R)):
        return None
    for e1 in (1, -1):
        Q1 = P1 if e1 == 1 else E.neg(P1)
        for e2 in (1, -1):
            Q2 = P2 if e2 == 1 else E.neg(P2)
            if E.add(Q1, Q2) == R:
                return (e1, e2)
    return None


def relation_row(E: Curve, R, P1, P2, x1: int, x2: int, colinfo, logC, ncols: int, colG: int,
                 colH: int, h: int, aj: int, bj: int, d: int, r: int):
    """Exact verification of a candidate decomposition R = e1*P1 + e2*P2.

    1. P1, P2, R lie on the curve and e1*P1 + e2*P2 == R by the affine group law for some
       signs (e1, e2) in {+-1}^2;
    2. the discrete-log identity  e1*s1*log(C1) + e2*s2*log(C2) == h*(aj + bj*d) (mod r),
       where [h]P_i = s_i*C_i with C_i the canonical column point and log taken in <G>.
    Returns (ok, e1, e2, row) where row is the relation over F_r with columns
    (classes..., G, H); row is None when ok is False.
    """
    vr = verify_relation(E, R, P1, P2)
    if vr is None:
        return False, None, None, None
    e1, e2 = vr
    row = [0] * ncols
    rhs = 0
    for (xx, ee) in ((x1, e1), (x2, e2)):
        col, sg = colinfo[xx]
        if col is not None:
            row[col] += ee * sg
            rhs += ee * sg * logC[col]
    if (h * (aj + bj * d)) % r != rhs % r:
        return False, e1, e2, None
    row[colG] = (-h * aj) % r
    row[colH] = (-h * bj) % r
    return True, e1, e2, row


def fingerprint(items) -> str:
    h = hashlib.sha256()
    for it in items:
        h.update(repr(it).encode())
        h.update(b"|")
    return h.hexdigest()
