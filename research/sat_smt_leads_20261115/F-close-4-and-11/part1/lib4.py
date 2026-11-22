"""Part 1 library: instances, the two CNFs (x-bit ANF as on 2026-11-14, and the symmetrised
(e1,e2,e3) system), cube runs with randomised cube order, brute-force checks.
pdplib.py and symbolic_e.py are unmodified copies of yesterday's files."""
import json, os, random, itertools
import numpy as np
from pysat.solvers import Solver
from pdplib import (Field, S4, span_table, rank_f2, random_basis, lift_x, add, neg, coords,
                    eval_table, moebius, B_CURVE)

HERE = os.path.dirname(os.path.abspath(__file__))
MASTER = 2026111504


# ---------------- instances ----------------
def in_span(basis, x):
    return rank_f2(list(basis) + [x]) == len(basis)


def zero_set(F, basis, xR):
    """indices (u1 | u2<<l | u3<<2l) of S4 zeros with all x_i != 0."""
    l = len(basis)
    full, (u1, u2, u3) = eval_table(F, basis, xR)
    ok = (full == 0) & (u1 != 0) & (u2 != 0) & (u3 != 0)
    return full, np.nonzero(ok)[0]


def make_target(F, l, seed, want_sat):
    """want_sat: planted R = +-P1+-P2+-P3 (distinct x in V, x != 0).
    else: x of a random curve point with NO S4 zero in (V minus 0)^3 (redrawn until so).
    Always x(R) != 0 and x(R) not in V (excludes the trivial P + (-P) + R)."""
    rng = random.Random(seed)
    while True:
        basis = random_basis(F, l, rng)
        tab = span_table(basis)
        cand = [int(x) for x in tab if x and lift_x(F, int(x)) is not None]
        if len(cand) >= 3:
            break
    tries = 0
    while True:
        tries += 1
        if want_sat:
            xs = rng.sample(cand, 3)
            pts = []
            for x in xs:
                P = (x, lift_x(F, x))
                if rng.random() < 0.5:
                    P = neg(P)
                pts.append(P)
            R = add(F, add(F, pts[0], pts[1]), pts[2])
            if R is None or R[0] == 0 or in_span(basis, R[0]):
                continue
            return dict(basis=basis, xR=R[0], planted=xs, seed=seed, tries=tries)
        x = rng.randrange(1, 1 << F.n)
        if lift_x(F, x) is None or in_span(basis, x):
            continue
        _, z = zero_set(F, basis, x)
        if len(z) == 0:
            return dict(basis=basis, xR=x, planted=None, seed=seed, tries=tries)


def curve_bruteforce(F, basis, xR):
    """set of x-coordinate index triples (u1,u2,u3) such that some affine points with those
    x (x != 0) sum to a point with x-coordinate xR.  Uses the group law only (handles doubling
    and an intermediate point at infinity); the point at infinity and the 2-torsion point
    (x = 0) are not factor-base elements."""
    l = len(basis)
    tab = span_table(basis)
    pts = []
    for u in range(1, 1 << l):
        x = int(tab[u])
        if x == 0:
            continue
        y = lift_x(F, x)
        if y is not None:
            pts.append((u, (x, y)))
            pts.append((u, (x, x ^ y)))
    out = set()
    for (u1, P1) in pts:
        for (u2, P2) in pts:
            Q = add(F, P1, P2)
            for (u3, P3) in pts:
                S = add(F, Q, P3)
                if S is not None and S[0] == xR:
                    out.add((u1, u2, u3))
    return out


# ---------------- CNF helpers ----------------
class CNF:
    def __init__(self):
        self.nv = 0
        self.cl = []

    def new(self):
        self.nv += 1
        return self.nv

    def AND(self, lits):
        c = self.new()
        for a in lits:
            self.cl.append([-c, a])
        self.cl.append([c] + [-a for a in lits])
        return c

    def xor(self, lits, rhs):
        """XOR(lits) = rhs, chopped to 3-literal chunks exactly as yesterday's xor_cnf."""
        lits = list(lits)
        if not lits:
            if rhs:
                self.cl.append([])
            return
        while len(lits) > 3:
            t = self.new()
            a, b = lits[0], lits[1]
            self.cl += [[-a, -b, -t], [a, b, -t], [a, -b, t], [-a, b, t]]
            lits = lits[2:] + [t]
        k = len(lits)
        for m in range(1 << k):
            if bin(m).count('1') % 2 != rhs:
                self.cl.append([-lits[i] if m >> i & 1 else lits[i] for i in range(k)])


def _common(cnf, l):
    """x-bit variables 1..3l (x1 | x2 | x3), y_j = x1_j^x2_j^x3_j, x_i != 0."""
    c = [[cnf.new() for _ in range(l)] for _ in range(3)]
    yv = []
    for j in range(l):
        y = cnf.new()
        yv.append(y)
    for j in range(l):
        cnf.xor([c[0][j], c[1][j], c[2][j], yv[j]], 0)
    for i in range(3):
        cnf.cl.append(list(c[i]))
    return c, yv


def build_x(F, I):
    """Yesterday's encoding: exact ANF of S4 on the 3l x-bits, one AND variable per monomial of
    degree >= 2, n XOR rows chopped to 3-literal chunks, plus y_j definitions.
    Added today: one clause per x_i forbidding x_i = 0."""
    l, n = len(I['basis']), F.n
    full, _ = eval_table(F, I['basis'], I['xR'])
    anf = moebius(full)
    nb = 3 * l
    cnf = CNF()
    c, yv = _common(cnf, l)
    masks = [int(m) for m in np.nonzero(anf)[0] if m]
    mono = {}
    for mk in masks:
        vs = [b + 1 for b in range(nb) if mk >> b & 1]
        mono[mk] = vs[0] if len(vs) == 1 else cnf.AND(vs)
    c0 = int(anf[0])
    for i in range(n):
        cnf.xor([mono[mk] for mk in masks if int(anf[mk]) >> i & 1], c0 >> i & 1)
    return dict(cnf=cnf, c=c, yv=yv, l=l, kind='x')


_E = None


def e_monomials():
    global _E
    if _E is None:
        _E = json.load(open(os.path.join(HERE, 'symbolic_e.json')))['monomials']
    return _E


def fpow(F, a, e):
    r = 1
    for _ in range(e):
        r = F.smul(r, a)
    return r


def build_e(F, I):
    """Symmetrised system.  Variables: x-bits (3l), e1 = sum y_j v_j (l bits, the same y_j),
    e2 = x1x2+x1x3+x2x3 (n bits), e3 = x1x2x3 (n bits), and S4 written in (e1,e2,e3)
    (11 monomials, bit-degree <= 2) as n XOR rows over AND variables of pairs of e-bits."""
    basis = I['basis']
    l, n = len(basis), F.n
    xR = I['xR']
    cnf = CNF()
    c, yv = _common(cnf, l)
    Z = [1 << i for i in range(n)]

    def rows(terms, const=0, eqvars=None):
        """sum coeff*var (+const) == sum_i eqvars_i z^i (or 0)."""
        for r in range(n):
            lits = [v for v, co in terms if co >> r & 1]
            if eqvars is not None:
                lits.append(eqvars[r])
            cnf.xor(lits, const >> r & 1)

    e2 = [cnf.new() for _ in range(n)]
    e3 = [cnf.new() for _ in range(n)]
    t2 = []
    for (a, b) in ((0, 1), (0, 2), (1, 2)):
        for j in range(l):
            for k in range(l):
                t2.append((cnf.AND([c[a][j], c[b][k]]), F.smul(basis[j], basis[k])))
    rows(t2, 0, e2)
    t3 = []
    for i in range(l):
        for j in range(l):
            pij = F.smul(basis[i], basis[j])
            for k in range(l):
                t3.append((cnf.AND([c[0][i], c[1][j], c[2][k]]), F.smul(pij, basis[k])))
    rows(t3, 0, e3)
    # S4(e1,e2,e3)
    a13 = {}
    a23 = {}
    a33 = {}

    def and13(j, i):
        if (j, i) not in a13:
            a13[(j, i)] = cnf.AND([yv[j], e3[i]])
        return a13[(j, i)]

    def and23(i, j):
        if (i, j) not in a23:
            a23[(i, j)] = cnf.AND([e2[i], e3[j]])
        return a23[(i, j)]

    def and33(i, j):
        i, j = min(i, j), max(i, j)
        if (i, j) not in a33:
            a33[(i, j)] = cnf.AND([e3[i], e3[j]])
        return a33[(i, j)]

    terms = []
    const = 0
    for mrow in e_monomials():
        a, b, cc = mrow['e1'], mrow['e2'], mrow['e3']
        co = 0
        for (ar, ab) in mrow['coeff_terms_xR_b']:
            co ^= F.smul(fpow(F, xR, ar), fpow(F, B_CURVE, ab))
        if co == 0:
            continue
        key = (a, b, cc)
        if key == (0, 0, 0):
            const ^= co
        elif key in ((0, 0, 1), (0, 0, 2), (0, 0, 4)):
            for i in range(n):
                terms.append((e3[i], F.smul(co, fpow(F, Z[i], cc))))
        elif key in ((0, 2, 0), (0, 4, 0)):
            for i in range(n):
                terms.append((e2[i], F.smul(co, fpow(F, Z[i], b))))
        elif key == (4, 0, 0):
            for j in range(l):
                terms.append((yv[j], F.smul(co, fpow(F, basis[j], 4))))
        elif key == (0, 0, 3):  # e3 * e3^2
            for i in range(n):
                for j in range(n):
                    v = e3[i] if i == j else and33(i, j)
                    terms.append((v, F.smul(co, F.smul(Z[i], fpow(F, Z[j], 2)))))
        elif key == (0, 2, 1):
            for i in range(n):
                for j in range(n):
                    terms.append((and23(i, j), F.smul(co, F.smul(fpow(F, Z[i], 2), Z[j]))))
        elif key in ((2, 0, 1), (2, 0, 2)):
            for j in range(l):
                for i in range(n):
                    terms.append((and13(j, i), F.smul(co, F.smul(fpow(F, basis[j], 2), fpow(F, Z[i], cc)))))
        else:
            raise ValueError(key)
    # merge duplicate variables (XOR of coefficients)
    acc = {}
    for v, co in terms:
        acc[v] = acc.get(v, 0) ^ co
    rows([(v, co) for v, co in acc.items() if co], const, None)
    return dict(cnf=cnf, c=c, yv=yv, l=l, kind='e')


# ---------------- solving ----------------
def decode(B, model):
    l = B['l']
    ms = set(v for v in model if v > 0)
    us = [sum(1 << j for j in range(l) if B['c'][i][j] in ms) for i in range(3)]
    return us


def all_models(B, solver='glucose4'):
    """all x-projections of models (blocking clauses on the 3l x-bits)."""
    out = set()
    xvars = [v for blk in B['c'] for v in blk]
    with Solver(name=solver, bootstrap_with=B['cnf'].cl) as s:
        while s.solve():
            m = s.get_model()
            ms = set(v for v in m if v > 0)
            out.add(tuple(decode(B, m)))
            s.add_clause([-v if v in ms else v for v in xvars])
    return out


def cube_lits(B, cubevars, k, cval):
    return [cubevars[j] if cval >> j & 1 else -cubevars[j] for j in range(k)]


def run_cubes(B, arm, k, order_seed, solver='cadical153', full=None, conf_budget=50000):
    """All 2^k cubes on the first k cube variables (arm 'lit': bits of x1; 'xor'/'sym': bits of
    e1), one incremental solver, cube order = seeded shuffle (order_seed None = numeric).
    Each cube solve is capped at conf_budget conflicts (solve_limited); if a cube hits the budget
    the run is marked censored and stopped (conflicts recorded are a lower bound)."""
    l = B['l']
    cubevars = B['c'][0] if arm == 'lit' else B['yv']
    order = list(range(1 << k))
    if order_seed is not None:
        random.Random(order_seed).shuffle(order)
    keys = ('conflicts', 'decisions', 'propagations', 'restarts')
    nsat = 0
    censored = False
    with Solver(name=solver, bootstrap_with=B['cnf'].cl, use_timer=False) as s:
        for cval in order:
            if conf_budget:
                s.conf_budget(conf_budget)
                r = s.solve_limited(assumptions=cube_lits(B, cubevars, k, cval), expect_interrupt=False)
            else:
                r = s.solve(assumptions=cube_lits(B, cubevars, k, cval))
            if r is None:
                censored = True
                break
            if r:
                nsat += 1
                u = decode(B, s.get_model())
                if full is not None:
                    assert full[u[0] | u[1] << l | u[2] << 2 * l] == 0, 'model is not an S4 zero'
                    assert all(u), 'x = 0 in model'
                e1 = u[0] ^ u[1] ^ u[2]
                tgt = u[0] if arm == 'lit' else e1
                assert tgt & ((1 << k) - 1) == cval, 'model outside its cube'
        st = s.accum_stats()
    out = {key: int(st[key]) for key in keys}
    out['n_sat_cubes'] = nsat
    out['censored'] = censored
    return out
