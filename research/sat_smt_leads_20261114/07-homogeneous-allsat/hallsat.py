#!/usr/bin/env python3
"""Idea 7 pilot: homogeneous AllSAT for factor-base-only relations vs per-target SAT.
Self-contained: F_{2^n}, curve y^2+xy=x^3+a x^2+b, S3 / chained S4 CNF, Glucose4 via pysat.
"""
import sys, json, time, random, itertools
from pysat.solvers import Solver

A_COEF, B_COEF = 0, 1
SOLVER = "glucose4"
BUDGET = 5_000_000


# ---------------- field ----------------
class GF:
    def __init__(self, n):
        self.n = n
        self.mod = self._find_irred(n)
        self.mask = (1 << n) - 1
        # solver for z^2+z=c
        self._qs = self._build_qsolver()

    def _mulmod(self, a, b, mod, n):
        r = 0
        while b:
            if b & 1:
                r ^= a
            b >>= 1
            a <<= 1
            if a >> n & 1:
                a ^= mod
        return r

    def _find_irred(self, n):
        # brute-force: f irreducible iff x^(2^n)=x mod f and gcd conditions; use simple trial division
        def pmod(a, b):
            db = b.bit_length()
            while a.bit_length() >= db:
                a ^= b << (a.bit_length() - db)
            return a
        for low in range(1, 1 << n, 2):
            f = (1 << n) | low
            ok = True
            for d in range(2, 1 << (n // 2 + 1)):
                if pmod(f, d) == 0:
                    ok = False
                    break
            if ok:
                return f
        raise RuntimeError

    def mul(self, a, b):
        return self._mulmod(a, b, self.mod, self.n)

    def sq(self, a):
        return self.mul(a, a)

    def inv(self, a):
        r, e, base = 1, (1 << self.n) - 2, a
        while e:
            if e & 1:
                r = self.mul(r, base)
            base = self.mul(base, base)
            e >>= 1
        return r

    def _build_qsolver(self):
        # linear map z -> z^2+z ; gaussian elimination storing preimages
        piv = {}
        for i in range(self.n):
            z = 1 << i
            img = self.sq(z) ^ z
            while img:
                h = img.bit_length() - 1
                if h in piv:
                    pi, pz = piv[h]
                    img ^= pi
                    z ^= pz
                else:
                    piv[h] = (img, z)
                    break
        return piv

    def solve_quad(self, c):
        """z with z^2+z=c or None"""
        z = 0
        while c:
            h = c.bit_length() - 1
            if h not in self._qs:
                return None
            pi, pz = self._qs[h]
            c ^= pi
            z ^= pz
        return z


class Curve:
    def __init__(self, F, a=A_COEF, b=B_COEF):
        self.F, self.a, self.b = F, a, b

    def lift(self, x):
        F = self.F
        if x == 0:
            # y^2 = b
            y = self.b
            for _ in range(F.n - 1):
                y = F.sq(y)
            return (0, y)
        xi = F.inv(x)
        c = x ^ self.a ^ F.mul(self.b, F.sq(xi))
        z = F.solve_quad(c)
        if z is None:
            return None
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


def S3_eval(F, x1, x2, x3, b=B_COEF):
    e = F.mul(x1, x2) ^ F.mul(x1, x3) ^ F.mul(x2, x3)
    return F.sq(e) ^ F.mul(F.mul(x1, x2), x3) ^ b


# ---------------- base ----------------
class Base:
    def __init__(self, F, l, rng):
        self.F, self.l = F, l
        while True:
            vs = [rng.randrange(1, 1 << F.n) for _ in range(l)]
            if self._rank(vs) == l:
                break
        self.vs = vs  # vs[0] is the most significant coefficient
        self.elems = {}
        for c in range(1 << l):
            x = 0
            for j in range(l):
                if c >> (l - 1 - j) & 1:
                    x ^= vs[j]
            self.elems[x] = c
        self.by_code = {c: x for x, c in self.elems.items()}

    @staticmethod
    def _rank(vs):
        piv = {}
        r = 0
        for v in vs:
            while v:
                h = v.bit_length() - 1
                if h in piv:
                    v ^= piv[h]
                else:
                    piv[h] = v
                    r += 1
                    break
        return r


# ---------------- CNF builder ----------------
class Enc:
    """Symbolic field elements: (dict var -> field const, const)."""

    def __init__(self, F):
        self.F = F
        self.nv = 0
        self.clauses = []
        self.andc = {}
        self.stats = {"ands": 0, "xors": 0, "xor_len": 0}

    def new(self):
        self.nv += 1
        return self.nv

    def AND(self, u, w):
        if u == w:
            return u
        k = (u, w) if u < w else (w, u)
        z = self.andc.get(k)
        if z is None:
            z = self.new()
            self.andc[k] = z
            self.clauses += [[-z, u], [-z, w], [z, -u, -w]]
            self.stats["ands"] += 1
        return z

    def xor(self, vs, rhs):
        vs = list(vs)
        self.stats["xors"] += 1
        self.stats["xor_len"] += len(vs)
        if not vs:
            if rhs:
                self.clauses.append([])
            return
        while len(vs) > 4:
            a, b, c = vs[:3]
            z = self.new()
            self._xor_clauses([a, b, c, z], 0)
            vs = vs[3:] + [z]
        self._xor_clauses(vs, rhs)

    def _xor_clauses(self, vs, rhs):
        k = len(vs)
        for bits in range(1 << k):
            if bin(bits).count("1") & 1 != rhs:
                self.clauses.append([(-v if bits >> i & 1 else v) for i, v in enumerate(vs)])

    # elements
    def const(self, c):
        return ({}, c)

    def lin(self, vars_, consts):
        return ({v: c for v, c in zip(vars_, consts)}, 0)

    def add(self, *es):
        d, c = {}, 0
        for (dd, cc) in es:
            c ^= cc
            for v, k in dd.items():
                nk = d.get(v, 0) ^ k
                if nk:
                    d[v] = nk
                else:
                    d.pop(v, None)
        return (d, c)

    def scale(self, e, k):
        F = self.F
        return ({v: F.mul(c, k) for v, c in e[0].items()}, F.mul(e[1], k))

    def sq(self, e):
        F = self.F
        return ({v: F.sq(c) for v, c in e[0].items()}, F.sq(e[1]))

    def mul(self, A, B):
        F = self.F
        d = {}

        def acc(v, k):
            nk = d.get(v, 0) ^ k
            if nk:
                d[v] = nk
            else:
                d.pop(v, None)
        for u, au in A[0].items():
            if B[1]:
                acc(u, F.mul(au, B[1]))
            for w, bw in B[0].items():
                acc(self.AND(u, w), F.mul(au, bw))
        if A[1]:
            for w, bw in B[0].items():
                acc(w, F.mul(A[1], bw))
        return (d, F.mul(A[1], B[1]))

    def bits(self):
        vs = [self.new() for _ in range(self.F.n)]
        return vs, self.lin(vs, [1 << k for k in range(self.F.n)])

    def eq_zero(self, e):
        d, c = e
        for k in range(self.F.n):
            self.xor([v for v, kk in d.items() if kk >> k & 1], c >> k & 1)

    def reify(self, e):
        vs, r = self.bits()
        self.eq_zero(self.add(e, r))
        return r

    def lex_le(self, a, b):
        """bit lists MSB first: a <= b"""
        eq = None  # None == true
        for j, (x, y) in enumerate(zip(a, b)):
            pre = [] if eq is None else [-eq]
            self.clauses.append(pre + [-x, y])
            if j + 1 < len(a):
                ne = self.new()
                self.clauses.append(pre + [x, y, ne])
                self.clauses.append(pre + [-x, -y, ne])
                eq = ne


def enc_s3(E, x1, x2, x3):
    """S3(x1,x2,x3)=0; x1x2 is reified when x3 is non-constant."""
    p = E.mul(x1, x2)
    if x3[0]:
        p = E.reify(p)
    e = E.add(p, E.mul(E.add(x1, x2), x3))
    E.eq_zero(E.add(E.sq(e), E.mul(p, x3), E.const(B_COEF)))


def enc_curve(E, x):
    """y^2 + x y + x^3 + a x^2 + b = 0 with fresh y bits"""
    _, y = E.bits()
    x2 = E.sq(x)
    E.eq_zero(E.add(E.sq(y), E.mul(x, y), E.mul(x2, x), E.scale(x2, A_COEF), E.const(B_COEF)))


def build(F, base, k, target=None, membership=False):
    """k free variables in V. target None: homogeneous S_k. else S_{k+1}(x1..xk, target)."""
    E = Enc(F)
    l = base.l
    cb = [[E.new() for _ in range(l)] for _ in range(k)]
    xs = [E.lin(cb[i], base.vs) for i in range(k)]
    for i in range(k - 1):
        E.lex_le(cb[i], cb[i + 1])
    allx = xs + ([E.const(target)] if target is not None else [])
    if len(allx) == 3:
        enc_s3(E, *allx)
    elif len(allx) == 4:
        _, t = E.bits()
        enc_s3(E, allx[0], allx[1], t)
        enc_s3(E, allx[2], allx[3], t)
    else:
        raise ValueError
    if membership:
        for x in xs:
            enc_curve(E, x)
    return E, cb


def decode(model, cb, base):
    ms = set(v for v in model if v > 0)
    codes = []
    for bits in cb:
        c = 0
        for v in bits:
            c = (c << 1) | (v in ms)
        codes.append(c)
    return tuple(codes)


def stats_of(s):
    st = s.accum_stats()
    return {k: st[k] for k in ("conflicts", "decisions", "propagations", "restarts")}


# ---------------- ground truth ----------------
def base_points(C, base):
    pts = {}
    for x, c in base.elems.items():
        P = C.lift(x)
        if P is not None:
            pts[c] = P
    return pts


def sums_to_zero(C, pts_list):
    """exists signs with sum == O"""
    k = len(pts_list)
    for signs in range(1 << (k - 1)):
        S = pts_list[0]
        for i in range(1, k):
            S = C.add(S, C.neg(pts_list[i]) if signs >> (i - 1) & 1 else pts_list[i])
        if S is None:
            return True
    return False


def truth_homog(C, base, pts, k):
    """set of sorted code tuples (nontrivial curve relations of k base points summing to O up to signs)"""
    codes = sorted(pts)
    rel = set()
    pair = {}
    for i, a in enumerate(codes):
        for b in codes[i:]:
            for Q in (C.add(pts[a], pts[b]), C.add(pts[a], C.neg(pts[b]))):
                if Q is None:
                    continue
                pair.setdefault(Q[0], set()).add((a, b))
    if k == 3:
        for x, prs in pair.items():
            c = base.elems.get(x)
            if c is not None and c in pts:
                for (a, b) in prs:
                    rel.add(tuple(sorted((a, b, c))))
    else:
        for x, prs in pair.items():
            prs = list(prs)
            for (a, b) in prs:
                for (c, d) in prs:
                    t = tuple(sorted((a, b, c, d)))
                    rel.add(t)
        # remove trivial ones / keep only those genuinely summing to O
        rel = set(t for t in rel if nontrivial4(C, pts, t))
    return rel


def nontrivial4(C, pts, t):
    """sorted t=(a,b,c,d): exists signs with (Pa +- Pb) = +-(Pc +- Pd) and Pa+-Pb != O (chain pairing (12)(34))"""
    a, b, c, d = t
    for s1 in (0, 1):
        Q = C.add(pts[a], C.neg(pts[b]) if s1 else pts[b])
        if Q is None:
            continue
        for s2 in (0, 1):
            R = C.add(pts[c], C.neg(pts[d]) if s2 else pts[d])
            if R is not None and R[0] == Q[0]:
                return True
    return False


def truth_target(C, base, pts, m, R):
    """number of sorted code m-tuples with +-P1..+-Pm = +-R (chain pairing for m=3: (12)(3R))"""
    codes = sorted(pts)
    sols = set()
    if m == 2:
        for i, a in enumerate(codes):
            for b in codes[i:]:
                for Q in (C.add(pts[a], pts[b]), C.add(pts[a], C.neg(pts[b]))):
                    if Q is not None and Q[0] == R[0]:
                        sols.add((a, b))
    else:
        need = {}
        for c in codes:
            for Q in (C.add(R, pts[c]), C.add(R, C.neg(pts[c]))):
                if Q is not None:
                    need.setdefault(Q[0], set()).add(c)
        for i, a in enumerate(codes):
            for b in codes[i:]:
                for Q in (C.add(pts[a], pts[b]), C.add(pts[a], C.neg(pts[b]))):
                    if Q is not None and Q[0] in need:
                        for c in need[Q[0]]:
                            sols.add(tuple(sorted((a, b, c))))
    return sols


# ---------------- arms ----------------
def run_homog(F, C, base, pts, k, membership):
    t0 = time.time()
    E, cb = build(F, base, k, None, membership)
    s = Solver(name=SOLVER, bootstrap_with=E.clauses)
    curve_rel, twist_rel, other = set(), set(), set()
    complete = True
    while True:
        s.conf_budget(BUDGET)
        r = s.solve_limited()
        if r is None:
            complete = False
            break
        if not r:
            break
        model = s.get_model()
        codes = decode(model, cb, base)
        assert list(codes) == sorted(codes), codes
        xs = [base.by_code[c] for c in codes]
        if k == 3:
            assert S3_eval(F, *xs) == 0
        lifted = [pts.get(c) for c in codes]
        if all(P is not None for P in lifted):
            assert sums_to_zero(C, lifted), ("curve model not summing to O", codes)
            curve_rel.add(codes)
        elif all(P is None for P in lifted):
            twist_rel.add(codes)
        else:
            other.add(codes)
        s.add_clause([(-v if model[v - 1] > 0 else v) for bits in cb for v in bits])
    st = stats_of(s)
    s.delete()
    return dict(st, wall=time.time() - t0, complete=complete, n_curve=len(curve_rel),
                n_twist=len(twist_rel), n_other=len(other), vars=E.nv, clauses=len(E.clauses),
                ands=E.stats["ands"]), curve_rel, twist_rel, other


def run_target(F, C, base, pts, m, R):
    t0 = time.time()
    E, cb = build(F, base, m, R[0], False)
    s = Solver(name=SOLVER, bootstrap_with=E.clauses)
    s.conf_budget(BUDGET)
    r = s.solve_limited()
    codes = None
    if r:
        codes = decode(s.get_model(), cb, base)
        lifted = [pts.get(c) for c in codes]
        assert all(P is not None for P in lifted), ("non-curve model in per-target", codes)
        assert sums_to_zero(C, lifted + [R]), ("model does not sum to R", codes)
    st = stats_of(s)
    s.delete()
    return dict(st, wall=time.time() - t0, sat=r, vars=E.nv, clauses=len(E.clauses)), codes


def random_target(C, base, rng):
    F = C.F
    while True:
        x = rng.randrange(1, 1 << F.n)
        if x in base.elems:
            continue
        P = C.lift(x)
        if P is not None:
            return P


def cell(k, l, n, seed, ntargets, arms=("H0", "H1", "PT"), check_twist=False):
    """k = m+1 homogeneous arity. returns list of rows."""
    F = GF(n)
    C = Curve(F)
    rng = random.Random(1000 * l + seed + 100000 * k)
    base = Base(F, l, rng)
    pts = base_points(C, base)
    truth = truth_homog(C, base, pts, k)
    common = dict(k=k, m=k - 1, l=l, n=n, seed=seed, n_base_x_on_curve=len(pts), truth_curve_rel=len(truth))
    rows = []
    for arm in arms:
        if arm in ("H0", "H1"):
            st, cr, tw, ot = run_homog(F, C, base, pts, k, arm == "H1")
            ok = (cr == truth) if st["complete"] else None
            if st["complete"]:
                assert cr == truth, (arm, k, l, n, seed, sorted(cr ^ truth)[:5])
                if arm == "H1":
                    assert st["n_twist"] == 0
                if check_twist and k == 3 and arm == "H0":
                    bf = set()
                    cs = sorted(base.by_code)
                    for a, b, c in itertools.combinations_with_replacement(cs, 3):
                        if S3_eval(F, base.by_code[a], base.by_code[b], base.by_code[c]) == 0:
                            bf.add((a, b, c))
                    assert bf == cr | tw | ot, "full model set != brute force S3 zero set"
            rows.append(dict(common, arm=arm, set_equals_truth=ok, **st))
        elif arm == "PT":
            for ti in range(ntargets):
                R = random_target(C, base, rng)
                sols = truth_target(C, base, pts, k - 1, R)
                st, codes = run_target(F, C, base, pts, k - 1, R)
                if st["sat"] is not None:
                    assert bool(st["sat"]) == (len(sols) > 0), ("PT mismatch", k, l, n, seed, ti)
                    if st["sat"]:
                        assert codes in sols, (codes, sorted(sols)[:3])
                rows.append(dict(common, arm="PT", target=ti, truth_n_decomp=len(sols), **st))
        elif arm == "PLANT":
            codes = sorted(pts)
            for ti in range(ntargets):
                while True:
                    ch = [rng.choice(codes) for _ in range(k - 1)]
                    R = None
                    for c in ch:
                        R = C.add(R, pts[c] if rng.random() < 0.5 else C.neg(pts[c]))
                    if R is not None and R[0] not in base.elems and nonneg_ok(C, pts, ch, R, k):
                        break
                st, got = run_target(F, C, base, pts, k - 1, R)
                assert st["sat"] is True, ("planted target UNSAT", k, l, n, seed, ch)
                rows.append(dict(common, arm="PLANT", target=ti, **st))
    return rows


def nonneg_ok(C, pts, ch, R, k):
    if k == 3:
        return True
    # chain pairing needs a finite intermediate for the sorted triple
    a, b, c = sorted(ch)
    for s1 in (0, 1):
        Q = C.add(pts[a], C.neg(pts[b]) if s1 else pts[b])
        if Q is None:
            continue
        for s2 in (0, 1):
            T = C.add(R, C.neg(pts[c]) if s2 else pts[c])
            if T is not None and T[0] == Q[0]:
                return True
    return False


if __name__ == "__main__":
    mode = sys.argv[1]
    if mode == "validate":
        for (k, l, n) in [(3, 4, 7), (3, 5, 9), (3, 6, 11), (4, 3, 7), (4, 4, 10)]:
            for seed in (101, 102):
                rows = cell(k, l, n, seed, 6, arms=("H0", "H1", "PT", "PLANT"), check_twist=True)
                h = [r for r in rows if r["arm"] == "H0"][0]
                print("OK", k, l, n, seed, "curve", h["n_curve"], "twist", h["n_twist"], "other", h["n_other"],
                      "H0 confl", h["conflicts"],
                      "PT sat", sum(1 for r in rows if r["arm"] == "PT" and r["sat"]),
                      "planted", sum(1 for r in rows if r["arm"] == "PLANT"))
    else:
        # run k out.jsonl seeds l:n,l:n,... ntargets
        k = int(sys.argv[2])
        out = sys.argv[3]
        seeds = [int(x) for x in sys.argv[4].split(",")]
        cells = [tuple(int(y) for y in x.split(":")) for x in sys.argv[5].split(",")]
        nt = int(sys.argv[6])
        with open(out, "a") as f:
            for (l, n) in cells:
                for seed in seeds:
                    t0 = time.time()
                    rows = cell(k, l, n, seed, nt)
                    for r in rows:
                        f.write(json.dumps(r) + "\n")
                    f.flush()
                    print("done", k, l, n, seed, round(time.time() - t0, 1), flush=True)
