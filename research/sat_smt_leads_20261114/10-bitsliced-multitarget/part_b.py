#!/usr/bin/env python3
"""Idea 10 pilot, part (b): CDCL divergence statistics + Python cross-validation.
Usage: part_b.py validate | run <lmin> <lmax> <budget_s>
"""
import json, sys, time, random, itertools, os
from pysat.solvers import Solver, Cadical195
from pysat.engines import Propagator

HERE = os.path.dirname(os.path.abspath(__file__))
N = 31
POLY = (1 << 31) | (1 << 3) | 1
A_CURVE = 1


def mul(a, b):
    r = 0
    while b:
        if b & 1:
            r ^= a
        b >>= 1
        a <<= 1
        if a >> N:
            a ^= POLY
    return r


def sq(a):
    return mul(a, a)


def inv(a):
    r = a
    for _ in range(29):
        r = mul(sq(r), a)
    return sq(r)


def tr(a):
    t = a
    acc = a
    for _ in range(N - 1):
        t = sq(t)
        acc ^= t
    return acc


def halftrace(a):
    t = a
    acc = a
    for _ in range(15):
        t = sq(sq(t))
        acc ^= t
    return acc


def s3(x1, x2, x, b):
    t = mul(x1, x2) ^ mul(x1, x) ^ mul(x2, x)
    return sq(t) ^ mul(mul(x1, x2), x) ^ b


def lift(x, b):
    """a point (x, y) on y^2+xy=x^3+a x^2+b, or None"""
    if x == 0:
        return None
    rhs = x ^ A_CURVE ^ mul(b, inv(sq(x)))
    if tr(rhs):
        return None
    z = halftrace(rhs)
    assert sq(z) ^ z == rhs
    return (x, mul(x, z))


def neg(P):
    return (P[0], P[0] ^ P[1])


def add(P, Q):
    if P is None:
        return Q
    if Q is None:
        return P
    (x1, y1), (x2, y2) = P, Q
    if x1 == x2:
        if y1 ^ y2 == x1:
            return None
        lam = x1 ^ mul(y1, inv(x1))
        x3 = sq(lam) ^ lam ^ A_CURVE
        y3 = sq(x1) ^ mul(lam ^ 1, x3)
        return (x3, y3)
    lam = mul(y1 ^ y2, inv(x1 ^ x2))
    x3 = sq(lam) ^ lam ^ x1 ^ x2 ^ A_CURVE
    y3 = mul(lam, x1 ^ x3) ^ x3 ^ y1
    return (x3, y3)


def on_curve(P, b):
    x, y = P
    return sq(y) ^ mul(x, y) == mul(sq(x), x) ^ mul(A_CURVE, sq(x)) ^ b


def load(l):
    return json.load(open(f"{HERE}/instances/instance_l{l}.json"))


def elems_of(basis):
    out = [0]
    for v in basis:
        out += [e ^ v for e in out]
    # index bit j <-> basis[j]
    res = [0] * len(out)
    for idx in range(len(out)):
        e = 0
        for j, v in enumerate(basis):
            if idx >> j & 1:
                e ^= v
        res[idx] = e
    return res


# ---------------- encodings ----------------
class CNF:
    def __init__(self):
        self.nv = 0
        self.cl = []

    def new(self):
        self.nv += 1
        return self.nv

    def xor(self, lits, rhs):
        """sum(lits) = rhs, chunked width 4 with aux vars"""
        lits = list(lits)
        if not lits:
            if rhs:
                self.cl.append([])
            return
        while len(lits) > 4:
            a = self.new()
            self._xor_raw(lits[:3] + [a], 0)
            lits = [a] + lits[3:]
        self._xor_raw(lits, rhs)

    def _xor_raw(self, lits, rhs):
        k = len(lits)
        for m in range(1 << k):
            # forbid assignments with wrong parity: assignment where lit i true iff bit i of m
            if (bin(m).count("1") & 1) != rhs:
                self.cl.append([(-lits[i] if m >> i & 1 else lits[i]) for i in range(k)])

    def and_(self, a, b):
        t = self.new()
        self.cl += [[-t, a], [-t, b], [t, -a, -b]]
        return t


def enc_E1(basis, b, X):
    l = len(basis)
    c = CNF()
    av = [c.new() for _ in range(l)]
    bv = [c.new() for _ in range(l)]
    t = {}
    for i in range(l):
        for j in range(l):
            t[i, j] = c.and_(av[i], bv[j])
    shared = list(c.cl)
    X2 = sq(X)
    coef = {(i, j): sq(mul(basis[i], basis[j])) ^ mul(mul(basis[i], basis[j]), X) for i in range(l) for j in range(l)}
    lin = [mul(sq(basis[i]), X2) for i in range(l)]
    for k in range(N):
        lits = [t[i, j] for i in range(l) for j in range(l) if coef[i, j] >> k & 1]
        lits += [av[i] for i in range(l) if lin[i] >> k & 1]
        lits += [bv[i] for i in range(l) if lin[i] >> k & 1]
        c.xor(lits, b >> k & 1)
    return c, av, bv, shared


def enc_E2(basis, b):
    """universal: X bits are variables 2l+1..2l+31, set by assumptions"""
    l = len(basis)
    c = CNF()
    av = [c.new() for _ in range(l)]
    bv = [c.new() for _ in range(l)]
    xv = [c.new() for _ in range(N)]
    t = {}
    for i in range(l):
        for j in range(l):
            t[i, j] = c.and_(av[i], bv[j])
    # B = sum t_ij v_i v_j ; C = sum (a_i+b_i) v_i^2 ; Y = X^2
    Bv = [c.new() for _ in range(N)]
    Cv = [c.new() for _ in range(N)]
    Yv = [c.new() for _ in range(N)]
    vv = {(i, j): mul(basis[i], basis[j]) for i in range(l) for j in range(l)}
    for k in range(N):
        c.xor([Bv[k]] + [t[i, j] for (i, j) in vv if vv[i, j] >> k & 1], 0)
        c.xor([Cv[k]] + [av[i] for i in range(l) if sq(basis[i]) >> k & 1] + [bv[i] for i in range(l) if sq(basis[i]) >> k & 1], 0)
        c.xor([Yv[k]] + [xv[s] for s in range(N) if sq(1 << s) >> k & 1], 0)
    g = {(r, s): c.and_(Bv[r], xv[s]) for r in range(N) for s in range(N)}
    h = {(r, s): c.and_(Cv[r], Yv[s]) for r in range(N) for s in range(N)}
    xp = {(r, s): mul(1 << r, 1 << s) for r in range(N) for s in range(N)}
    for k in range(N):
        lits = [t[i, j] for (i, j) in vv if sq(vv[i, j]) >> k & 1]
        lits += [g[r, s] for (r, s) in g if xp[r, s] >> k & 1]
        lits += [h[r, s] for (r, s) in h if xp[r, s] >> k & 1]
        c.xor(lits, b >> k & 1)
    return c, av, bv, xv


def model_xy(model, av, bv, basis):
    x1 = x2 = 0
    for i, v in enumerate(basis):
        if model[av[i] - 1] > 0:
            x1 ^= v
        if model[bv[i] - 1] > 0:
            x2 ^= v
    return x1, x2


# ---------------- decision tracer ----------------
class Tracer(Propagator):
    def __init__(self, seed, cap=200):
        self.dec = []
        self.stack = []
        self.newlevel = False
        self.samples = []
        self.rng = random.Random(seed)
        self.cap = cap
        self.nbt = 0

    def on_assignment(self, lit, fixed=False):
        if self.newlevel:
            self.newlevel = False
            self.dec.append(lit)
            self.stack.append(lit)
            n = len(self.dec)
            if len(self.samples) < self.cap:
                self.samples.append(list(self.stack))
            else:
                r = self.rng.randrange(n)
                if r < self.cap:
                    self.samples[r] = list(self.stack)

    def on_new_level(self):
        self.newlevel = True

    def on_backtrack(self, to):
        self.nbt += 1
        del self.stack[to:]
        self.newlevel = False

    def check_model(self, model):
        return True

    def decide(self):
        return 0

    def propagate(self):
        return []

    def provide_reason(self, lit):
        return []

    def add_clause(self):
        return []


def traced_solve(cnf, assumptions=()):
    s = Cadical195(bootstrap_with=cnf.cl)
    tr_ = Tracer(seed=1)
    s.connect_propagator(tr_)
    for v in range(1, cnf.nv + 1):
        s.observe(v)
    t0 = time.time()
    sat = s.solve(assumptions=list(assumptions))
    dt = time.time() - t0
    st = s.accum_stats()
    model = s.get_model() if sat else None
    s.delete()
    return sat, model, tr_, st, dt


def lcp(a, b):
    n = 0
    for x, y in zip(a, b):
        if x != y:
            break
        n += 1
    return n


def plain_stats(name, clauses, assumptions=()):
    with Solver(name=name, bootstrap_with=clauses) as s:
        sat = s.solve(assumptions=list(assumptions))
        return sat, s.accum_stats()


def med(v):
    v = sorted(v)
    return v[len(v) // 2] if v else None


def run(lmin, lmax, budget):
    T0 = time.time()
    out = []
    for l in range(lmin, lmax + 1):
        inst = load(l)
        basis, b = inst["basis"], inst["b"]
        targets = [("planted", x, c) for x, c in zip(inst["planted"][:8], inst["count_planted"][:8])]
        targets += [("random", x, c) for x, c in zip(inst["random"][:8], inst["count_random"][:8])]
        e2, av2, bv2, xv2 = enc_E2(basis, b)
        traces = {"E1": [], "E2": []}
        for ti, (cls, X, refcount) in enumerate(targets):
            if time.time() - T0 > budget:
                print("budget exhausted at l", l, "target", ti, file=sys.stderr)
                break
            e1, av, bv, shared = enc_E1(basis, b, X)
            row = dict(kind="cdcl", l=l, target_index=ti, cls=cls, X=X, ref_ordered_solutions=refcount,
                       E1_vars=e1.nv, E1_clauses=len(e1.cl), E1_shared_clauses=len(shared),
                       E2_vars=e2.nv, E2_clauses=len(e2.cl))
            # baseline: untreated solvers on E1 and E2 (fresh)
            for name in ("cadical153", "glucose4", "minisat22"):
                sat, st = plain_stats(name, e1.cl)
                assert sat == (refcount > 0), (l, ti, name)
                row[f"E1_{name}"] = st
            ass = [xv2[s] if X >> s & 1 else -xv2[s] for s in range(N)]
            for name in ("cadical153", "glucose4"):
                sat, st = plain_stats(name, e2.cl, ass)
                assert sat == (refcount > 0), (l, ti, name, "E2")
                row[f"E2_{name}_fresh"] = st
            # traced CaDiCaL 1.9.5
            sat, model, tr1, st, dt = traced_solve(e1)
            assert sat == (refcount > 0)
            if sat:
                x1, x2 = model_xy(model, av, bv, basis)
                assert s3(x1, x2, X, b) == 0 and x1 != x2
            row["E1_trace"] = dict(decisions=len(tr1.dec), backtracks=tr1.nbt, stats=st, seconds=round(dt, 3))
            traces["E1"].append((cls, tr1.dec, [d for d in tr1.dec if abs(d) <= 2 * l]))
            # (b2) target-independent share of propagation
            full = Solver(name="minisat22", bootstrap_with=e1.cl)
            sh = Solver(name="minisat22", bootstrap_with=shared)
            nf = ns = 0
            for stck in tr1.samples:
                _, pf = full.propagate(assumptions=stck)
                _, ps = sh.propagate(assumptions=stck)
                nf += max(0, len(pf) - len(stck))
                ns += max(0, len(ps) - len(stck))
            full.delete(); sh.delete()
            row["E1_prop_sampled_stacks"] = len(tr1.samples)
            row["E1_implied_full"] = nf
            row["E1_implied_shared_only"] = ns
            row["E1_shared_prop_fraction"] = (ns / nf) if nf else None
            sat, model, tr2, st, dt = traced_solve(e2, ass)
            assert sat == (refcount > 0)
            xset = set(xv2)
            d2 = [d for d in tr2.dec if abs(d) not in xset]
            row["E2_trace"] = dict(decisions=len(d2), decisions_incl_assumptions=len(tr2.dec), backtracks=tr2.nbt, stats=st, seconds=round(dt, 3))
            traces["E2"].append((cls, d2, [d for d in d2 if abs(d) <= 2 * l]))
            out.append(row)
            print(l, ti, cls, "E1 cad153", row["E1_cadical153"], "trace dec", len(tr1.dec), "sharedprop", row["E1_shared_prop_fraction"], "E2dec", len(d2), flush=True)
        # (b1) pairwise divergence
        for enc in ("E1", "E2"):
            tl = traces[enc]
            for (i, (c1, d1, p1)), (j, (c2, dd2, p2)) in itertools.combinations(enumerate(tl), 2):
                mean_all = (len(d1) + len(dd2)) / 2 or 1
                mean_p = (len(p1) + len(p2)) / 2 or 1
                out.append(dict(kind="pair", l=l, enc=enc, i=i, j=j, classes=c1 + "-" + c2,
                                lcp_all=lcp(d1, dd2), frac_all=lcp(d1, dd2) / mean_all,
                                lcp_ab=lcp(p1, p2), frac_ab=lcp(p1, p2) / mean_p,
                                len_i=len(d1), len_j=len(dd2)))
        # (b3) incremental reuse on E2
        if len(traces["E2"]) == len(targets):
            for name in ("cadical153", "glucose4"):
                s = Solver(name=name, bootstrap_with=e2.cl)
                prev = dict(conflicts=0, decisions=0, propagations=0, restarts=0)
                for ti, (cls, X, refcount) in enumerate(targets):
                    ass = [xv2[q] if X >> q & 1 else -xv2[q] for q in range(N)]
                    sat = s.solve(assumptions=ass)
                    assert sat == (refcount > 0)
                    st = s.accum_stats()
                    out.append(dict(kind="incremental", l=l, solver=name, order=ti, cls=cls, X=X,
                                    conflicts=st["conflicts"] - prev["conflicts"],
                                    decisions=st["decisions"] - prev["decisions"],
                                    propagations=st["propagations"] - prev["propagations"]))
                    prev = st
                s.delete()
        json.dump(out, open(f"{HERE}/raw_part_b.json", "w"))
    json.dump(out, open(f"{HERE}/raw_part_b.json", "w"))


def validate():
    rep = {}
    rng = random.Random(20261114)
    for l in (5, 6):
        inst = load(l)
        basis, b = inst["basis"], inst["b"]
        el = elems_of(basis)
        # brute force against the Rust counts (independent field code)
        ok = 0
        tg = list(zip(inst["planted"][:8], inst["count_planted"][:8])) + list(zip(inst["random"][:8], inst["count_random"][:8]))
        for X, c in tg:
            bf = sum(1 for x1 in el for x2 in el if x1 != x2 and s3(x1, x2, X, b) == 0)
            assert bf == c, (l, X, bf, c)
            ok += 1
        # point-level planting
        pts = [P for P in (lift(x, b) for x in el) if P]
        assert all(on_curve(P, b) for P in pts)
        lifted = 0
        other = 0
        made = 0
        while made < 8:
            P1, P2 = rng.choice(pts), rng.choice(pts)
            if P1[0] == P2[0]:
                continue
            if rng.random() < 0.5:
                P2 = neg(P2)
            R = add(P1, P2)
            assert on_curve(R, b)
            assert s3(P1[0], P2[0], R[0], b) == 0
            made += 1
            e1, av, bv, _ = enc_E1(basis, b, R[0])
            with Solver(name="cadical153", bootstrap_with=e1.cl) as s:
                assert s.solve()
                x1, x2 = model_xy(s.get_model(), av, bv, basis)
            assert s3(x1, x2, R[0], b) == 0
            Q1, Q2 = lift(x1, b), lift(x2, b)
            good = False
            if Q1 and Q2:
                for S1 in (Q1, neg(Q1)):
                    for S2 in (Q2, neg(Q2)):
                        if add(S1, S2) in (R, neg(R)):
                            good = True
            if good:
                lifted += 1
            else:
                other += 1
            # E2 agrees
            e2, av2, bv2, xv2 = enc_E2(basis, b)
            with Solver(name="cadical153", bootstrap_with=e2.cl) as s:
                assert s.solve(assumptions=[xv2[q] if R[0] >> q & 1 else -xv2[q] for q in range(N)])
                y1, y2 = model_xy(s.get_model(), av2, bv2, basis)
                assert s3(y1, y2, R[0], b) == 0
        rep[l] = dict(bruteforce_targets_matching_rust=ok, base_points_x=len(pts), point_planted=made,
                      sat_model_lifts_to_points_summing_to_pmR=lifted, sat_model_x_only_no_rational_lift=other)
        print(l, rep[l], flush=True)
    json.dump(rep, open(f"{HERE}/validation.json", "w"), indent=1)


if __name__ == "__main__":
    if sys.argv[1] == "validate":
        validate()
    else:
        run(int(sys.argv[2]), int(sys.argv[3]), float(sys.argv[4]))
