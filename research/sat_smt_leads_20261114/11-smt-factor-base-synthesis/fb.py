"""Self-contained F_{2^n} arithmetic, binary curve y^2+xy=x^3+a x^2+b, subspace tools,
linear 2+1 scan and CNF generators for the idea-11 pilot."""
import random, itertools
MOD = {7: (1 << 7) | 3, 11: (1 << 11) | 5, 13: (1 << 13) | 0b11011, 15: (1 << 15) | 3,
       17: (1 << 17) | 9, 19: (1 << 19) | 0b100111, 23: (1 << 23) | 33}

def pmulmod(a, b, f, n):
    r = 0
    while b:
        if b & 1: r ^= a
        b >>= 1; a <<= 1
        if a >> n: a ^= f
    return r

def pgcd(a, b):
    while b:
        while a and a.bit_length() >= b.bit_length():
            a ^= b << (a.bit_length() - b.bit_length())
        a, b = b, a
    return a

def is_irreducible(f, n):
    def frob_pow(k):
        x = 2
        for _ in range(k): x = pmulmod(x, x, f, n)
        return x
    if frob_pow(n) != 2: return False
    for q in [q for q in range(2, n + 1) if n % q == 0 and all(q % d for d in range(2, q))]:
        if pgcd(f, frob_pow(n // q) ^ 2) != 1: return False
    return True

class Field:
    def __init__(self, n, a=0, b=1):
        self.n, self.f, self.a, self.b = n, MOD[n], a, b
        assert is_irreducible(self.f, n)
        self.sqtab = [self.mul(1 << i, 1 << i) for i in range(n)]
        if n % 2:
            self.httab = [self._ht(1 << i) for i in range(n)]
        self.trmask = 0
        for i in range(n):
            if self._tr(1 << i): self.trmask |= 1 << i
        self.sqrt_b = self.sqrt(b)
    def mul(self, a, b): return pmulmod(a, b, self.f, self.n)
    def sq(self, a):
        r = 0; i = 0
        while a:
            if a & 1: r ^= self.sqtab[i]
            a >>= 1; i += 1
        return r
    def _sq(self, a): return self.mul(a, a)
    def sqrt(self, a):
        for _ in range(self.n - 1): a = self.mul(a, a)
        return a
    def inv(self, a):
        assert a
        r = 1; e = a
        for _ in range(self.n - 1):
            e = self.mul(e, e); r = self.mul(r, e)
        return r
    def _tr(self, a):
        t = 0; e = a
        for _ in range(self.n):
            t ^= e; e = self.mul(e, e)
        assert t in (0, 1); return t
    def tr(self, a): return bin(a & self.trmask).count('1') & 1
    def _ht(self, a):
        t = 0; e = a
        for _ in range((self.n + 1) // 2):
            t ^= e; e = self.mul(self.mul(e, e), self.mul(e, e))
        return t
    def ht(self, a):
        r = 0; i = 0
        while a:
            if a & 1: r ^= self.httab[i]
            a >>= 1; i += 1
        return r
    # curve
    def lift(self, x):
        """points with this x: list of (x,y) (two points, or one if x==0, or none)."""
        if x == 0: return [(0, self.sqrt_b)]
        c = x ^ self.a ^ self.mul(self.b, self.inv(self.sq(x)))
        if self.tr(c): return []
        z = self.ht(c); y = self.mul(x, z)
        assert self.on_curve((x, y))
        return [(x, y), (x, x ^ y)]
    def on_curve(self, P):
        x, y = P
        return self.sq(y) ^ self.mul(x, y) == self.mul(self.sq(x), x) ^ self.mul(self.a, self.sq(x)) ^ self.b
    def neg(self, P): return None if P is None else (P[0], P[0] ^ P[1])
    def add(self, P, Q):
        if P is None: return Q
        if Q is None: return P
        x1, y1 = P; x2, y2 = Q
        if x1 == x2:
            if y1 != y2 or x1 == 0: return None
            lam = x1 ^ self.mul(y1, self.inv(x1))
            x3 = self.sq(lam) ^ lam ^ self.a
            return (x3, self.sq(x1) ^ self.mul(lam ^ 1, x3))
        lam = self.mul(y1 ^ y2, self.inv(x1 ^ x2))
        x3 = self.sq(lam) ^ lam ^ x1 ^ x2 ^ self.a
        return (x3, self.mul(lam, x1 ^ x3) ^ x3 ^ y1)
    def rand_point(self, rng):
        while True:
            x = rng.randrange(1, 1 << self.n)
            L = self.lift(x)
            if L: return L[rng.randrange(2)]

# ---------- F2 linear algebra on ints ----------
def echelon(vecs):
    """reduced-ish echelon basis: dict pivotbit -> vec (each vec's top bit is its pivot)."""
    B = {}
    for v in vecs:
        v = reduce(v, B)
        if v: B[v.bit_length() - 1] = v
    return B
def reduce(v, B):
    while v:
        h = v.bit_length() - 1
        if h not in B: break
        v ^= B[h]
    return v
def rank(vecs): return len(echelon(vecs))
def span_basis(vecs):
    """fully reduced basis list (sorted by pivot desc) for coordinates."""
    B = echelon(vecs)
    piv = sorted(B, reverse=True)
    for i, h in enumerate(piv):
        for h2 in piv[i + 1:]:
            if B[h] >> h2 & 1: B[h] ^= B[h2]
    return [B[h] for h in piv], piv
def coords(v, basis, piv):
    """coordinates of v in fully reduced basis; None if not in span."""
    c = 0
    for k, h in enumerate(piv):
        if v >> h & 1:
            v ^= basis[k]; c |= 1 << k
    return c if v == 0 else None

def rand_subspace(n, l, rng, within=None):
    while True:
        if within is None: vs = [rng.randrange(1, 1 << n) for _ in range(l)]
        else: vs = [within(rng) for _ in range(l)]
        if rank(vs) == l: return vs
def poly_span(F, l, rng):
    while True:
        t = rng.randrange(2, 1 << F.n); c = rng.randrange(1, 1 << F.n)
        vs = [c]
        for _ in range(l - 1): vs.append(F.mul(vs[-1], t))
        if rank(vs) == l: return vs
def frob_span(F, l, rng):
    while True:
        t = rng.randrange(2, 1 << F.n); vs = [t]
        for _ in range(l - 1): vs.append(F.sq(vs[-1]))
        if rank(vs) == l: return vs
def trace0_span(F, l, rng):
    def w(r):
        while True:
            x = r.randrange(1, 1 << F.n)
            if F.tr(x) == 0: return x
    return rand_subspace(F.n, l, rng, within=w)
def products(F, vs):
    return [F.mul(vs[i], vs[j]) for i in range(len(vs)) for j in range(i, len(vs))]
def prod_dim(F, vs): return rank(products(F, vs))

# ---------- literal Macaulay defect of the m=2 descended system ----------
def macaulay_m2(F, vs, xR):
    """returns (nmon2, rank2, nmon3, rank3). Variables: c1_0..c1_{l-1}, c2_0..c2_{l-1}."""
    l = len(vs); n = F.n
    poly = {}
    def addm(m, c):
        if c: poly[m] = poly.get(m, 0) ^ c
    xR2 = F.sq(xR)
    addm(0, F.b)
    for j in range(l):
        c = F.mul(xR2, F.sq(vs[j]))           # s^2 xR^2
        addm(1 << j, c); addm(1 << (l + j), c)
        for k in range(l):
            pr = F.mul(vs[j], vs[k])
            addm((1 << j) | (1 << (l + k)), F.sq(pr) ^ F.mul(xR, pr))
    poly = {m: c for m, c in poly.items() if c}
    nv = 2 * l
    nmon2 = 1 + nv + nv * (nv - 1) // 2
    nmon3 = nmon2 + nv * (nv - 1) * (nv - 2) // 6
    rank2 = rank(list(poly.values()))
    idx = {}
    rows = []
    for mult in [0] + [1 << v for v in range(nv)]:
        q = {}
        for m, c in poly.items():
            mm = m | mult; q[mm] = q.get(mm, 0) ^ c
        for i in range(n):
            r = 0
            for mm, c in q.items():
                if c >> i & 1:
                    if mm not in idx: idx[mm] = len(idx)
                    r |= 1 << idx[mm]
            if r: rows.append(r)
    return nmon2, rank2, nmon3, rank(rows)

# ---------- factor base and linear 2+1 ----------
class Base:
    def __init__(self, F, vs):
        self.F, self.vs, self.l = F, list(vs), len(vs)
        self.Vb, self.Vp = span_basis(vs)
        self.Wb, self.Wp = span_basis(products(F, vs))
        self.w = len(self.Wb)
        self.xs = []
        for cmask in range(1 << self.l):
            x = 0
            for j in range(self.l):
                if cmask >> j & 1: x ^= vs[j]
            self.xs.append(x)
        self.points = [P for x in self.xs for P in F.lift(x)]
    def in_V(self, x): return reduce(x, {h: b for h, b in zip(self.Vp, self.Vb)}) == 0

def solve_affine(cols, rhs):
    """all masks m with XOR_{j in m} cols[j] == rhs: returns (particular or None, kernel list)."""
    B = {}; ker = []
    for j, v in enumerate(cols):
        m = 1 << j
        while v:
            h = v.bit_length() - 1
            if h not in B: break
            v ^= B[h][0]; m ^= B[h][1]
        if v: B[v.bit_length() - 1] = (v, m)
        else: ker.append(m)
    v = rhs; m = 0
    while v:
        h = v.bit_length() - 1
        if h not in B: return None, ker
        v ^= B[h][0]; m ^= B[h][1]
    return m, ker

def leaf(F, base, u, cap=1 << 14):
    """solve S3(x1,x2,u)=0, x1,x2 in V, via (s,p) linearisation.
    returns (ncands, truncated, list of (x1,x2) solutions with both x liftable)."""
    l, w = base.l, base.w
    u2 = F.sq(u)
    cols = [F.mul(u2, F.sq(v)) for v in base.Vb] + [F.sq(wk) ^ F.mul(u, wk) for wk in base.Wb]
    part, ker = solve_affine(cols, F.b)
    if part is None: return 0, False, []
    nu = len(ker); tot = 1 << nu; trunc = tot > cap
    sols = []
    Vd = {h: b for h, b in zip(base.Vp, base.Vb)}
    cur = part
    for g in range(min(tot, cap)):
        if g:
            cur ^= ker[(g & -g).bit_length() - 1]
        s = 0; p = 0
        for j in range(l):
            if cur >> j & 1: s ^= base.Vb[j]
        for k in range(w):
            if cur >> (l + k) & 1: p ^= base.Wb[k]
        if s == 0:
            x1 = F.sqrt(p)
            if reduce(x1, Vd) == 0 and F.lift(x1): sols.append((x1, x1))
            continue
        c = F.mul(p, F.inv(F.sq(s)))
        if F.tr(c): continue
        x1 = F.mul(s, F.ht(c))
        if reduce(x1, Vd): continue
        x2 = x1 ^ s
        if F.lift(x1) and F.lift(x2): sols.append((x1, x2))
    return tot, trunc, sols

def lifts_to(F, xs, R):
    """True if some choice of points with the given x's sums to R."""
    for Ps in itertools.product(*[F.lift(x) for x in xs]):
        S = None
        for P in Ps: S = F.add(S, P)
        if S == R: return True
    return False

def l21(F, base, R, cap=1 << 14):
    leaves = cands = consistent = le1 = 0; trunc = False; found = []
    for P3 in base.points:
        Q = F.add(R, F.neg(P3))
        if Q is None or Q[0] == 0: continue
        leaves += 1
        nc, tr, sols = leaf(F, base, Q[0], cap)
        cands += nc; trunc |= tr; consistent += nc > 0; le1 += nc <= 1
        for (x1, x2) in sols:
            assert lifts_to(F, (x1, x2, P3[0]), R) or lifts_to(F, (x1, x2, P3[0]), F.neg(R)), "lift failed"
            found.append((x1, x2, P3[0]))
    return dict(leaves=leaves, cands=cands, consistent=consistent, le1=le1, trunc=trunc,
                dec=bool(found), nsol=len(found))

def m2_truth(F, base, R):
    nc, tr, sols = leaf(F, base, R[0])
    for (x1, x2) in sols: assert lifts_to(F, (x1, x2), R) or lifts_to(F, (x1, x2), F.neg(R))
    return bool(sols), nc

def brute3(F, base, R):
    pts = base.points
    for P1 in pts:
        for P2 in pts:
            Q = F.add(P1, P2)
            if Q is None or Q[0] == 0: continue
            for P3 in pts:
                if F.add(Q, P3) == R: return True
    return False

# ---------- CNF ----------
class CNF:
    def __init__(self): self.nv = 0; self.cl = []
    def new(self): self.nv += 1; return self.nv
    def AND(self, a, b):
        c = self.new(); self.cl += [[-c, a], [-c, b], [c, -a, -b]]; return c
    def xor(self, lits, rhs, cut=5):
        """XOR of variables lits == rhs."""
        lits = list(lits)
        while len(lits) > cut:
            t = self.new()
            self._xor(lits[:cut - 1] + [t], 0)
            lits = lits[cut - 1:] + [t]
        self._xor(lits, rhs)
    def _xor(self, lits, rhs):
        k = len(lits)
        if k == 0:
            if rhs: self.cl.append([])
            return
        for bits in range(1 << k):
            if bin(bits).count('1') & 1 != rhs:   # forbid assignments with wrong parity
                self.cl.append([(-lits[i] if bits >> i & 1 else lits[i]) for i in range(k)])

def bits_eq(cnf, F, terms, const):
    """terms: list of (var, field coeff). Adds n XOR constraints sum var*coeff == const."""
    for r in range(F.n):
        cnf.xor([v for v, c in terms if c >> r & 1], const >> r & 1)

def cnf_m3(F, base, R):
    l, n, w = base.l, F.n, base.w; vs = base.vs; xR = R[0]
    cnf = CNF()
    c = [[cnf.new() for _ in range(l)] for _ in range(3)]
    t = [cnf.new() for _ in range(n)]
    cnf.cl.append(list(t))  # t != 0: excludes Q = 2-torsion point (and twist pairs summing to it)
    s = []
    for j in range(l):
        sj = cnf.new(); cnf._xor([c[0][j], c[1][j], sj], 0); s.append(sj)
    # P_k: coordinates of p=x1*x2 in W basis
    Pterms = [[] for _ in range(w)]
    for j in range(l):
        for k in range(l):
            a = cnf.AND(c[0][j], c[1][k])
            co = coords(F.mul(vs[j], vs[k]), base.Wb, base.Wp)
            for q in range(w):
                if co >> q & 1: Pterms[q].append(a)
    P = []
    for q in range(w):
        pv = cnf.new(); cnf.xor(Pterms[q] + [pv], 0); P.append(pv)
    # A: p^2 + t p + (t s)^2 + b = 0
    terms = [(P[q], F.sq(base.Wb[q])) for q in range(w)]
    for i in range(n):
        for q in range(w):
            terms.append((cnf.AND(t[i], P[q]), F.mul(1 << i, base.Wb[q])))
        for j in range(l):
            terms.append((cnf.AND(t[i], s[j]), F.sq(F.mul(1 << i, vs[j]))))
    bits_eq(cnf, F, terms, F.b)
    # B: q^2 + xR q + xR^2 (t + x3)^2 + b = 0, q = t*x3
    xR2 = F.sq(xR); terms = []
    for i in range(n):
        terms.append((t[i], F.mul(xR2, F.sq(1 << i))))
        for j in range(l):
            pr = F.mul(1 << i, vs[j])
            terms.append((cnf.AND(t[i], c[2][j]), F.sq(pr) ^ F.mul(xR, pr)))
    for j in range(l):
        terms.append((c[2][j], F.mul(xR2, F.sq(vs[j]))))
    bits_eq(cnf, F, terms, F.b)
    return cnf, c

def cnf_m2(F, base, R):
    l, n = base.l, F.n; vs = base.vs; xR = R[0]; xR2 = F.sq(xR)
    cnf = CNF()
    c = [[cnf.new() for _ in range(l)] for _ in range(2)]
    terms = []
    for j in range(l):
        co = F.mul(xR2, F.sq(vs[j]))
        terms += [(c[0][j], co), (c[1][j], co)]
        for k in range(l):
            pr = F.mul(vs[j], vs[k])
            terms.append((cnf.AND(c[0][j], c[1][k]), F.sq(pr) ^ F.mul(xR, pr)))
    bits_eq(cnf, F, terms, F.b)
    return cnf, c

def model_xs(base, c, model):
    ms = set(l for l in model if l > 0); out = []
    for blk in c:
        x = 0
        for j, v in enumerate(blk):
            if v in ms: x ^= base.vs[j]
        out.append(x)
    return out

def sat_solve(cnf, name='cadical153'):
    from pysat.solvers import Solver
    import time
    t0 = time.process_time()
    with Solver(name=name, bootstrap_with=cnf.cl) as S:
        res = S.solve()
        st = S.accum_stats() or {}
        model = S.get_model() if res else None
    return res, dict(conflicts=st.get('conflicts'), decisions=st.get('decisions'),
                     propagations=st.get('propagations'), cpu=time.process_time() - t0,
                     vars=cnf.nv, clauses=len(cnf.cl)), model
