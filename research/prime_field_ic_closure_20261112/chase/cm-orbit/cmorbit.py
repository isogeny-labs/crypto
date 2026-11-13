#!/usr/bin/env python3
"""Toy m=2 index calculus on j=0 curves y^2 = x^3 + b over F_p (p = 7 mod 12),
prime group order n.  Arms:

  fold      : factor base = first F/2 abscissae x>=1 with x^3+b QR, both signs;
              one unknown per {+-P} (negation folding).          k = 2
  cmorb     : factor base = union of F/6 full <-1,zeta>-orbits (zeta(x,y)=(wx,y));
              one unknown per orbit, log(e*zeta^i U) = e*lam^i log U.  k = 6
  foldcm    : same point set as cmorb, negation-folded accounting only (control
              isolating the set geometry from the unknown count).  k = 2

Oracles (decomposition of a walk point R = aP + bQ as R = P1 + P2, P1,P2 in F):
  O2 'table' : pair-sum table keyed by the class of the sum under the arm's fold
               group (fold/foldcm: <-1>, |F|^2/4 sums; cmorb: <-1,zeta>, |F|^2/12),
               one probe per trial.  (Matches the incumbent's mitm / E12 oracle shape.)
  O1 'semaev': for each base abscissa x1, solve Semaev S3(x1, x2, xR)=0 (quadratic in
               x2), Legendre + sqrt, membership test of x2.  Per trial |F|/2 abscissae.

Linear algebra: rows have two unknowns, so the system is a weighted graph; an
incremental union-find solver pins log_P(Q) at the first component with two
independent cycle equations (first determination), and we keep streaming to full
rank (every unknown pinned) to record that count too.

Rho arms: r-adding walk (r=32) storing every point, on classes of
  plain (no class), neg (<-1>), aut6 (<-1,zeta>); fruitless cycles detected
  as trivial collisions and escaped by doubling.

Every field operation the code performs is counted at its call site:
  M (mult/square), I (inversion), L (Legendre), R (sqrt), H (hash insert/probe).
Nothing here is a ledger result: [pcc:unverified].
"""
import json, math, random, sys, time
from collections import defaultdict

# ---------------------------------------------------------------- utilities
def is_prime(m):
    if m < 2: return False
    for q in (2,3,5,7,11,13,17,19,23,29,31,37):
        if m % q == 0: return m == q
    d, s = m-1, 0
    while d % 2 == 0: d //= 2; s += 1
    for a in (2,3,5,7,11,13,17,19,23,29,31,37):
        x = pow(a, d, m)
        if x in (1, m-1): continue
        for _ in range(s-1):
            x = x*x % m
            if x == m-1: break
        else: return False
    return True

def isqrt_exact(v):
    if v < 0: return None
    r = math.isqrt(v)
    return r if r*r == v else None

class Ctr:
    __slots__ = ("M","I","L","R","H","A","D")
    def __init__(self): self.M=self.I=self.L=self.R=self.H=self.A=self.D=0
    def d(self): return {k: getattr(self,k) for k in self.__slots__}

def fermat_cost(e):  # square-and-multiply mults for exponent e
    return (e.bit_length()-1) + (bin(e).count("1")-1)

# ---------------------------------------------------------------- curve
class Curve:
    def __init__(self, p, b, n):
        self.p, self.b, self.n = p, b, n
        g = 2
        while True:
            w = pow(g, (p-1)//3, p)
            if w != 1: break
            g += 1
        self.w = w; self.w2 = w*w % p
        self.e_half = (p-1)//2; self.e_sqrt = (p+1)//4; self.e_inv = p-2
        self.cost_L = fermat_cost(self.e_half); self.cost_R = fermat_cost(self.e_sqrt)
        self.cost_I = fermat_cost(self.e_inv)

    # uncounted helpers for setup
    def on(self, P):
        x, y = P; return (y*y - x*x*x - self.b) % self.p == 0
    def add_u(self, P, Q):
        p = self.p
        if P is None: return Q
        if Q is None: return P
        x1,y1 = P; x2,y2 = Q
        if x1 == x2:
            if (y1+y2) % p == 0: return None
            l = 3*x1*x1 * pow(2*y1, p-2, p) % p
        else:
            l = (y2-y1) * pow(x2-x1, p-2, p) % p
        x3 = (l*l - x1 - x2) % p
        return (x3, (l*(x1-x3) - y1) % p)
    def mul_u(self, k, P):
        R = None; k %= self.n
        while k:
            if k & 1: R = self.add_u(R, P)
            P = self.add_u(P, P); k >>= 1
        return R
    def neg(self, P): return None if P is None else (P[0], (-P[1]) % self.p)
    def rand_point(self, rng):
        p = self.p
        while True:
            x = rng.randrange(1, p)
            v = (x*x*x + self.b) % p
            if pow(v, self.e_half, p) == 1:
                return (x, pow(v, self.e_sqrt, p))

    # counted group law (affine): add = I + 2M + 1S ; dbl = I + 2M + 2S (+3x^2)
    def add(self, P, Q, c):
        p = self.p
        if P is None: return Q
        if Q is None: return P
        x1,y1 = P; x2,y2 = Q
        if x1 == x2:
            if (y1+y2) % p == 0: return None
            c.I += 1; c.M += 4; c.D += 1
            l = 3*x1*x1 * pow(2*y1, p-2, p) % p
        else:
            c.I += 1; c.M += 3; c.A += 1
            l = (y2-y1) * pow(x2-x1, p-2, p) % p
        x3 = (l*l - x1 - x2) % p
        return (x3, (l*(x1-x3) - y1) % p)

def find_curve(bits, rng):
    """random p = 7 mod 12 near 2^bits, b with #E prime (j=0 CM order via 4p = t^2+3v^2)."""
    while True:
        p = rng.randrange(1 << (bits-1), 1 << bits)
        p -= (p % 12); p += 7
        if p >= (1 << bits) or not is_prime(p): continue
        # 4p = t^2 + 3 v^2
        sols = None
        for v in range(1, math.isqrt(4*p//3)+1):
            t = isqrt_exact(4*p - 3*v*v)
            if t is not None: sols = (t, v); break
        t, v = sols
        traces = set()
        for tr in (t, -t, (t+3*v)//2, -(t+3*v)//2, (t-3*v)//2, -(t-3*v)//2):
            traces.add(tr)
        traces = [tr for tr in traces if (t+3*v) % 2 == 0 or tr in (t, -t)]
        for b in range(2, 400):
            if pow(b, (p-1)//2, p) == 1: continue  # b square => 3-torsion (0,sqrt b)
            cands = [p+1-tr for tr in traces]
            E = Curve(p, b, 0)
            ok = None
            for _ in range(6):
                P = E.rand_point(rng)
                cands = [N for N in cands if mul_raw(E, N, P) is None]
                if len(cands) == 1: break
            if len(cands) == 1 and is_prime(cands[0]) and cands[0] != p:
                return Curve(p, b, cands[0])

def mul_raw(E, k, P):
    R = None
    while k:
        if k & 1: R = E.add_u(R, P)
        P = E.add_u(P, P); k >>= 1
    return R

def setup(E, rng):
    """P, Q = sP, lambda with zeta(P) = lambda P."""
    n, p = E.n, E.p
    P = E.rand_point(rng)
    s = rng.randrange(1, n)
    Q = E.mul_u(s, P)
    # roots of l^2 + l + 1 = 0 mod n
    lam = None
    for g in range(2, 200):
        l = pow(g, (n-1)//3, n)
        if l != 1:
            lam = l; break
    zP = (E.w * P[0] % p, P[1])
    if E.mul_u(lam, P) != zP:
        lam = lam*lam % n
    assert E.mul_u(lam, P) == zP
    return P, Q, s, lam

# ---------------------------------------------------------------- canonical forms
# Each returns (canonical point, multiplier m) with canonical = m * pt in the group.
def canon_none(E, pt, lam, c):
    return pt, 1
def canon_neg(E, pt, lam, c):
    x, y = pt; p = E.p
    if y <= (p-1)//2: return pt, 1
    return (x, p-y), E.n-1
def canon_aut(E, pt, lam, c):
    x, y = pt; p = E.p
    c.M += 1
    x1 = E.w * x % p          # zeta(pt) abscissa
    x2 = (-x - x1) % p        # zeta^2(pt) abscissa, free since 1+w+w^2 = 0
    if x <= x1 and x <= x2: xm, i = x, 0
    elif x1 <= x2: xm, i = x1, 1
    else: xm, i = x2, 2
    mult = pow(lam, i, E.n)
    if y > (p-1)//2:
        y = p-y; mult = (E.n - mult) % E.n
    return (xm, y), mult

# ---------------------------------------------------------------- factor bases
def build_base(E, arm, F, lam, c):
    """Returns dict with: pts (list of (pt, col, coef)), ncols, col_of (pt->(col, coef)), xs."""
    p, n = E.p, E.n
    col_of = {}; pts = []
    if arm == "fold":
        x = 1; col = 0
        while len(pts) < F:
            v = (x*x*x + E.b) % p; c.M += 2; c.L += 1
            if pow(v, E.e_half, p) == 1:
                c.R += 1; y = pow(v, E.e_sqrt, p)
                if y > (p-1)//2: y = p-y
                for pt, cf in (((x, y), 1), ((x, p-y), n-1)):
                    col_of[pt] = (col, cf); pts.append((pt, col, cf))
                col += 1
            x += 1
        ncols = col
    else:  # cmorb / foldcm share the point set
        x = 1; seen = set(); orbits = []
        while 6*len(orbits) < F:
            if x not in seen:
                v = (x*x*x + E.b) % p; c.M += 2; c.L += 1
                c.M += 1; x1 = E.w*x % p; x2 = (-x-x1) % p
                seen.update((x, x1, x2))
                if pow(v, E.e_half, p) == 1:
                    c.R += 1; y = pow(v, E.e_sqrt, p)
                    if y > (p-1)//2: y = p-y
                    xm = min(x, x1, x2)
                    orbits.append((xm, y))
            x += 1
        col = 0
        for (xm, y) in orbits:
            xs3 = [xm, E.w*xm % p, E.w2*xm % p]; c.M += 2
            if arm == "cmorb":
                for i in range(3):
                    li = pow(lam, i, n)
                    for pt, cf in (((xs3[i], y), li), ((xs3[i], p-y), (n-li) % n)):
                        col_of[pt] = (col, cf); pts.append((pt, col, cf))
                col += 1
            else:  # foldcm: each +-pair its own unknown
                for i in range(3):
                    for pt, cf in (((xs3[i], y), 1), ((xs3[i], p-y), n-1)):
                        col_of[pt] = (col, cf); pts.append((pt, col, cf))
                    col += 1
        ncols = col
    return dict(pts=pts, ncols=ncols, col_of=col_of, xs=sorted({pt[0] for pt, _, _ in pts}))

# ---------------------------------------------------------------- graph LA
class GraphLA:
    """Rows c1*u_i + c2*u_j = A + B*s (mod n).  Each node v: u_v = al + be*s + ga*r_root.
    Component equations: e0 + e1*s + e2*r = 0."""
    def __init__(self, n, ncols):
        self.n = n
        self.parent = list(range(ncols))
        self.expr = [(0, 0, 1) for _ in range(ncols)]  # relative to its root
        self.members = [[v] for v in range(ncols)]
        self.eqs = [[] for _ in range(ncols)]
        self.s = None; self.ops = 0; self.rows = 0
        self.pinned_nodes = 0; self.pinned = [False]*ncols
        self.ncols = ncols; self.touched = set()
    def inv(self, a):
        self.ops += 1; return pow(a, self.n-2, self.n)
    def _apply(self, v):
        r = self.parent[v]; return r, self.expr[v]
    def add_row(self, i, ci, j, cj, A, B):
        n = self.n; self.rows += 1; self.touched.update((i, j))
        ri, (ai, bi, gi) = self._apply(i)
        rj, (aj, bj, gj) = self._apply(j)
        if ri != rj:
            # ci*(ai+bi s+gi ri) + cj*(aj+bj s+gj rj) = A + B s  -> rj = alpha + beta s + gamma ri
            k = self.inv(cj*gj % n)
            alpha = (A - ci*ai - cj*aj) * k % n
            beta = (B - ci*bi - cj*bj) * k % n
            gamma = (-ci*gi) * k % n
            self.ops += 6
            # merge smaller into larger: if comp j larger, express ri via rj instead
            if len(self.members[rj]) > len(self.members[ri]):
                # ri = (rj - alpha - beta s)/gamma
                kg = self.inv(gamma)
                alpha, beta, gamma = (-alpha*kg) % n, (-beta*kg) % n, kg
                self.ops += 2
                ri, rj = rj, ri
            # now substitute rj = alpha + beta s + gamma*ri into comp rj
            for v in self.members[rj]:
                a, b_, g = self.expr[v]
                self.expr[v] = ((a + g*alpha) % n, (b_ + g*beta) % n, g*gamma % n)
                self.parent[v] = ri; self.ops += 3
            for (e0, e1, e2) in self.eqs[rj]:
                self.eqs[ri].append(((e0 + e2*alpha) % n, (e1 + e2*beta) % n, e2*gamma % n))
                self.ops += 3
            self.members[ri].extend(self.members[rj]); self.members[rj] = []; self.eqs[rj] = []
            if self.pinned[rj] or self.pinned[ri]:
                pass
            root = ri
        else:
            e0 = (ci*ai + cj*aj - A) % n
            e1 = (ci*bi + cj*bj - B) % n
            e2 = (ci*gi + cj*gj) % n
            self.ops += 6
            self.eqs[ri].append((e0, e1, e2)); root = ri
        self._reduce(root)
        return self.s
    def _reduce(self, r):
        n = self.n; eqs = self.eqs[r]
        if self.s is None:
            for (e0, e1, e2) in eqs:
                if e2 == 0 and e1 != 0:
                    self.s = (-e0) * self.inv(e1) % n; self.ops += 1; break
            if self.s is None and len(eqs) >= 2:
                for a in range(len(eqs)):
                    for b_ in range(a+1, len(eqs)):
                        (p0, p1, p2), (q0, q1, q2) = eqs[a], eqs[b_]
                        det = (p1*q2 - p2*q1) % n; self.ops += 2
                        if det:
                            self.s = ((-p0*q2 + p2*q0) % n) * self.inv(det) % n
                            self.ops += 3; break
                    if self.s is not None: break
            # keep at most 3 eqs in a component
            if len(eqs) > 3: self.eqs[r] = eqs[:3]
    def full_rank(self):
        """after s known: a component is pinned if any eq has e2 != 0 (or it is a singleton
        never touched -> not pinned). Returns number of pinned unknowns among all ncols."""
        if self.s is None: return 0
        cnt = 0
        for r in range(self.ncols):
            if self.members[r] and any(e2 for (_, _, e2) in self.eqs[r]):
                cnt += len(self.members[r])
        return cnt

# ---------------------------------------------------------------- IC
def make_walk(E, P, Q, rng, r=32):
    Ms = []
    for _ in range(r):
        a, b = rng.randrange(E.n), rng.randrange(E.n)
        Ms.append((E.add_u(E.mul_u(a, P), E.mul_u(b, Q)), a, b))
    return Ms

def run_ic(E, arm, oracle, F, rng, curve_setup, fullrank=True, max_trials=None):
    P, Q, s_true, lam = curve_setup
    n, p = E.n, E.p
    c_base, c_tab, c_rel, c_wlk = Ctr(), Ctr(), Ctr(), Ctr()
    base = build_base(E, arm, F, lam, c_base)
    prime_cache(base)
    canon = canon_aut if arm == "cmorb" else canon_neg
    col_of = base["col_of"]
    table = None
    if oracle == "table":
        table = {}
        reps = {}
        for pt, col, cf in base["pts"]:
            if cf == 1: reps[col] = pt
        for col_u in range(base["ncols"]):
            U = reps[col_u]
            for V, col_v, cf_v in base["pts"]:
                if col_v < col_u: continue
                W = E.add(U, V, c_tab)
                if W is None: continue
                Wc, mW = canon(E, W, lam, c_tab)
                c_tab.H += 1
                if Wc not in table:
                    table[Wc] = (col_u, col_v, cf_v, mW)
    # walk
    Ms = make_walk(E, P, Q, rng)
    a0, b0 = rng.randrange(n), rng.randrange(n)
    R = E.add_u(E.mul_u(a0, P), E.mul_u(b0, Q)); a, b = a0, b0
    la = GraphLA(n, base["ncols"])
    trials = 0; rels = 0; rels_det = None; trials_det = None; c_det = None
    xset = set(base["xs"])
    full_rels = None; full_trials = None
    cap = max_trials or 10**9
    while trials < cap:
        j = rng.randrange(len(Ms)); Mj, ma, mb = Ms[j]
        R = E.add(R, Mj, c_wlk); a = (a+ma) % n; b = (b+mb) % n
        if R is None:
            R = E.add_u(Ms[0][0], Ms[1][0]); a = (Ms[0][1]+Ms[1][1]) % n; b = (Ms[0][2]+Ms[1][2]) % n
            continue
        trials += 1
        row = None
        if oracle == "table":
            Rc, mR = canon(E, R, lam, c_rel); c_rel.H += 1
            hit = table.get(Rc)
            if hit is not None:
                col_u, col_v, cf_v, mW = hit
                # mR*R = mW*(u + cf_v*v)
                row = (col_u, mW, col_v, mW*cf_v % n, mR*a % n, mR*b % n)
        else:
            row = semaev_decomp(E, R, base, xset, col_of, c_rel)
            if row is not None:
                i, ci, jj, cj = row; row = (i, ci, jj, cj, a, b)
        if row is not None:
            rels += 1
            i, ci, jj, cj, A, B = row
            la.add_row(i, ci, jj, cj, A, B)
            if la.s is not None and rels_det is None:
                rels_det, trials_det = rels, trials
                c_det = {k: (c_base.d()[k], c_tab.d()[k], c_rel.d()[k], c_wlk.d()[k]) for k in Ctr.__slots__}
                la_ops_det = la.ops
                if not fullrank: break
                cap = min(cap, trials*30)
            if rels_det is not None and fullrank and rels % 5 == 0:
                if la.full_rank() == base["ncols"]:
                    full_rels, full_trials = rels, trials; break
    if rels_det is None:
        return dict(ok=False, arm=arm, F=F, trials=trials)
    ok = (la.s == s_true)
    return dict(ok=ok, arm=arm, oracle=oracle, F=len(base["pts"]), ncols=base["ncols"],
                rels_det=rels_det, trials_det=trials_det, la_ops_det=la_ops_det,
                full_rels=full_rels, full_trials=full_trials,
                counts=c_det, table_entries=(len(table) if table is not None else 0))

def semaev_decomp(E, R, base, xset, col_of, c):
    p, b, n = E.p, E.b, E.n
    xR, yR = R
    c.M += 1; xR2 = xR*xR % p
    fourb = 4*b % p
    for x1 in base["xs"]:
        A = (x1 - xR) * (x1 - xR) % p
        t = x1*xR % p
        B = (-2*(t*(x1+xR) + 2*b)) % p
        C = (x1*x1 % p * xR2 - fourb*(x1+xR)) % p
        D = (B*B - 4*A*C) % p
        c.M += 9
        if A == 0: continue
        if D:
            c.L += 1
            if pow(D, E.e_half, p) != 1: continue
            c.R += 1; sq = pow(D, E.e_sqrt, p)
        else:
            sq = 0
        c.I += 1; inv2A = pow(2*A, p-2, p); c.M += 2
        for x2 in {(-B + sq) * inv2A % p, (-B - sq) * inv2A % p}:
            c.H += 1
            if x2 in xset:
                P1 = _ptcache[(id(col_of), x1)]; P2 = _ptcache[(id(col_of), x2)]
                nP2 = (P2[0], (p-P2[1]) % p)
                for Q2 in (P2, nP2):
                    S = E.add(P1, Q2, c)
                    if S is None: continue
                    if S == R:
                        return col_of[P1] + col_of[Q2]
                    if S[0] == xR:   # S = -R
                        nP1 = (P1[0], (p-P1[1]) % p); nQ2 = (Q2[0], (p-Q2[1]) % p)
                        return col_of[nP1] + col_of[nQ2]
    return None

def find_pt(col_of, x, p):
    # base points stored with both signs; find one with abscissa x (dict scan avoided by cache)
    return _ptcache[(id(col_of), x)]

_ptcache = {}
def prime_cache(base):
    _ptcache.clear()
    for pt, col, cf in base["pts"]:
        _ptcache[(id(base["col_of"]), pt[0])] = pt

# ---------------------------------------------------------------- rho
def run_rho(E, cls, rng, curve_setup, r=32):
    P, Q, s_true, lam = curve_setup
    n, p = E.n, E.p
    canon = {"plain": canon_none, "neg": canon_neg, "aut6": canon_aut}[cls]
    c = Ctr()
    Ms = make_walk(E, P, Q, rng, r)
    a, b = rng.randrange(n), rng.randrange(n)
    X = E.add_u(E.mul_u(a, P), E.mul_u(b, Q))
    X, m = canon(E, X, lam, c); a, b = a*m % n, b*m % n
    seen = {}; steps = 0; fruitless = 0
    while True:
        c.H += 1
        hit = seen.get(X)
        if hit is not None:
            a2, b2 = hit
            if (b - b2) % n:
                s = (a2 - a) * pow(b - b2, n-2, n) % n
                return dict(ok=(s == s_true), steps=steps, fruitless=fruitless, counts=c.d())
            # trivial collision: fruitless cycle -> escape by doubling
            fruitless += 1
            X = E.add(X, X, c); a, b = 2*a % n, 2*b % n; steps += 1
            X, m = canon(E, X, lam, c); a, b = a*m % n, b*m % n
            continue
        seen[X] = (a, b)
        j = X[0] % r
        Mj, ma, mb = Ms[j]
        X = E.add(X, Mj, c); a, b = (a+ma) % n, (b+mb) % n; steps += 1
        if X is None:
            X = Ms[0][0]; a, b = Ms[0][1], Ms[0][2]
        X, m = canon(E, X, lam, c); a, b = a*m % n, b*m % n
