#!/usr/bin/env python3
"""Skeptic computation for MP-ml_empirical-4 (cross-n template mining of fall polynomials).

Exact Boolean closure W_D (treatise falldegree.tex, Def. 'Boolean closure') of the
Weil-descended Semaev S_3 system, m=2, on an l-dim random subspace V of F_{2^n}:

    X1 = sum a_j v_j,  X2 = sum b_j v_j,   S = (X1 X2 + X1 z + X2 z)^2 + X1 X2 z + a6,
    F  = the n F_2-coordinates of S  (Boolean ring B = F_2[a,b]/(a^2+a, b^2+b)).

For each D we record dim W_D, the degree filtration dim(W_D cap B_{<=e}), the new-fall
space Fall_D = (W_D cap B_{<=D-1}) / W_{D-1} with its degree distribution, D* (least D with
1 in W_D for UNSAT draws; least D with W_D = I(S) cap B_{<=D} for SAT draws), and the
Lemma-deg3 (falldegree.tex, lem:deg3) span test: do the explicit n-independent falls
Tr(lambda X_i w), lambda in V^perp, and Tr(s w) + Tr(a6/z^2) Tr(w) together with span F
and l0*B_{<=1} span the whole degree-<=2 part of W_3?

Pure python, exact arithmetic, rows as python ints. Cells: n in {11,...,21}, l = ceil(n/3)
(the idea's cell) and the diagonal l = ceil(n/2) at n = 11, 13 as a cross-check against
the treatise (d_F = 3 there).
"""
import json, random, sys, time, os
from math import comb

IRRED = {11: (11, 2, 0), 13: (13, 4, 3, 1, 0), 15: (15, 1, 0), 17: (17, 3, 0),
         19: (19, 5, 2, 1, 0), 21: (21, 2, 0)}


def pdeg(p):
    return p.bit_length() - 1


def pmod(a, m):
    dm = pdeg(m)
    while a and pdeg(a) >= dm:
        a ^= m << (pdeg(a) - dm)
    return a


def pgcd(a, b):
    while b:
        a, b = b, pmod(a, b)
    return a


def pmulmod(a, b, m):
    r = 0
    while b:
        if b & 1:
            r ^= a
        b >>= 1
        a <<= 1
        if pdeg(a) >= pdeg(m):
            a ^= m
    return r


def is_irreducible(f):
    n = pdeg(f)
    x = 2
    xp = x
    for k in range(1, n // 2 + 1):
        xp = pmulmod(xp, xp, f)          # x^(2^k)
        if pgcd(f, xp ^ x) != 1:
            return False
    return True


class GF:
    def __init__(self, n):
        self.n = n
        self.mod = sum(1 << e for e in IRRED[n])
        assert is_irreducible(self.mod), n

    def mul(self, a, b):
        return pmulmod(a, b, self.mod)

    def sq(self, a):
        return self.mul(a, a)

    def pow(self, a, e):
        r = 1
        while e:
            if e & 1:
                r = self.mul(r, a)
            a = self.sq(a)
            e >>= 1
        return r

    def inv(self, a):
        assert a
        return self.pow(a, (1 << self.n) - 2)

    def tr(self, a):
        t, x = 0, a
        for _ in range(self.n):
            t ^= x
            x = self.sq(x)
        return t  # in {0,1}

    def halftrace(self, c):
        # n odd: G(c)^2 + G(c) = c + Tr(c)
        g, x = 0, c
        for _ in range((self.n - 1) // 2 + 1):
            g ^= x
            x = self.sq(self.sq(x))
        return g


# ---- Boolean-variable polynomials with K coefficients: dict{monomask: coeff} ----
def kp_mul(A, B, K):
    C = {}
    for ma, ca in A.items():
        for mb, cb in B.items():
            v = K.mul(ca, cb)
            if v:
                m = ma | mb
                C[m] = C.get(m, 0) ^ v
    return {m: c for m, c in C.items() if c}


def kp_add(*Ps):
    C = {}
    for P in Ps:
        for m, c in P.items():
            C[m] = C.get(m, 0) ^ c
    return {m: c for m, c in C.items() if c}


def kp_scale(P, c, K):
    return {m: K.mul(v, c) for m, v in P.items() if K.mul(v, c)}


def kp_trace(P, K):
    """F_2-valued Boolean polynomial Tr(P): set of monomials with odd trace coefficient."""
    return frozenset(m for m, c in P.items() if K.tr(c))


def kp_coords(P, n):
    """The n F_2-coordinate polynomials of P (polynomial basis)."""
    out = []
    for j in range(n):
        f = frozenset(m for m, c in P.items() if (c >> j) & 1)
        if f:
            out.append(f)
    return out


def popcount(x):
    return bin(x).count("1")


# ---- curve helpers: E: y^2 + x y = x^3 + a2 x^2 + a6 over K ----
def point_with_x(K, x, a2, a6):
    if x == 0:
        return None
    c = x ^ a2 ^ K.mul(a6, K.inv(K.sq(x)))
    if K.tr(c):
        return None
    z = K.halftrace(c)
    return (x, K.mul(x, z))


def add_pts(K, P, Q, a2):
    (x1, y1), (x2, y2) = P, Q
    if x1 == x2:
        if y1 == y2 and x1 != 0:
            lam = x1 ^ K.mul(y1, K.inv(x1))
            x3 = K.sq(lam) ^ lam ^ a2
            y3 = K.sq(x1) ^ K.mul(lam ^ 1, x3)
            return (x3, y3)
        return None  # P = -Q
    lam = K.mul(y1 ^ y2, K.inv(x1 ^ x2))
    x3 = K.sq(lam) ^ lam ^ x1 ^ x2 ^ a2
    y3 = K.mul(lam, x1 ^ x3) ^ x3 ^ y1
    return (x3, y3)


def semaev3_value(K, x1, x2, x3, a6):
    e2 = K.mul(x1, x2) ^ K.mul(x1, x3) ^ K.mul(x2, x3)
    return K.sq(e2) ^ K.mul(K.mul(x1, x2), x3) ^ a6


# ---- exact Boolean closure ----
class Closure:
    def __init__(self, N, Dmax):
        self.N = N
        monos = [m for m in range(1 << N) if popcount(m) <= Dmax]
        monos.sort(key=lambda m: (-popcount(m), m))     # index 0 = highest degree
        self.monos = monos
        self.idx = {m: i for i, m in enumerate(monos)}
        self.coldeg = [popcount(m) for m in monos]
        self.const_col = self.idx[0]

    def row(self, f):
        r = 0
        for m in f:
            r |= 1 << self.idx[m]
        return r

    def unrow(self, r):
        out = set()
        while r:
            lb = (r & -r).bit_length() - 1
            out.add(self.monos[lb])
            r ^= 1 << lb
        return frozenset(out)

    @staticmethod
    def lead(r):
        return (r & -r).bit_length() - 1

    def reduce(self, r, piv):
        while r:
            lb = self.lead(r)
            p = piv.get(lb)
            if p is None:
                return r
            r ^= p
        return 0

    def insert(self, r, piv):
        r = self.reduce(r, piv)
        if r:
            piv[self.lead(r)] = r
            return True
        return False

    def closure(self, F_rows, D):
        """W_D: smallest subspace of B_{<=D} containing F cap B_{<=D}, closed under
        multiplication by variables while degree stays <= D."""
        piv = {}
        # Macaulay rows m*f with deg(m|f) <= D
        for r in F_rows:
            f = self.unrow(r)
            df = max(popcount(m) for m in f)
            if df > D:
                continue
            for m in self.monos:
                if popcount(m) > D - df:
                    continue
                prod = frozenset(mm | m for mm in f)
                if max(popcount(x) for x in prod) <= D:
                    self.insert(self.row(prod), piv)
        # iterate: multiply every basis element of degree < D by every variable
        done = set()
        while True:
            new = 0
            for lb, r in list(piv.items()):
                if self.coldeg[lb] >= D or r in done:
                    continue
                done.add(r)
                f = self.unrow(r)
                for v in range(self.N):
                    prod = frozenset(mm | (1 << v) for mm in f)
                    if self.insert(self.row(prod), piv):
                        new += 1
            if not new:
                break
        return piv

    def filtration(self, piv, D):
        return [sum(1 for lb in piv if self.coldeg[lb] <= e) for e in range(D + 1)]

    def contains(self, r, piv):
        return self.reduce(r, piv) == 0

    def rank_of(self, rows):
        piv = {}
        for r in rows:
            self.insert(r, piv)
        return len(piv)


def build_instance(n, l, seed, sat):
    K = GF(n)
    rng = random.Random(seed)
    a2 = rng.randrange(1 << n)
    a6 = rng.randrange(1, 1 << n)
    # random l-dim subspace V
    while True:
        basis = [rng.randrange(1, 1 << n) for _ in range(l)]
        # rank check over F_2
        piv = {}
        ok = True
        for v in basis:
            r = v
            while r:
                lb = r.bit_length() - 1
                if lb in piv:
                    r ^= piv[lb]
                else:
                    piv[lb] = r
                    break
            if r == 0:
                ok = False
                break
        if ok:
            break
    if sat:
        while True:
            c1 = rng.randrange(1, 1 << l)
            c2 = rng.randrange(1, 1 << l)
            x1 = 0
            x2 = 0
            for j in range(l):
                if (c1 >> j) & 1:
                    x1 ^= basis[j]
                if (c2 >> j) & 1:
                    x2 ^= basis[j]
            P = point_with_x(K, x1, a2, a6)
            Q = point_with_x(K, x2, a2, a6)
            if P is None or Q is None or x1 == x2:
                continue
            R = add_pts(K, P, Q, a2)
            if R is None or R[0] == 0:
                continue
            xR = R[0]
            break
    else:
        while True:
            x = rng.randrange(1, 1 << n)
            if point_with_x(K, x, a2, a6) is not None:
                xR = x
                break
    return K, a2, a6, basis, xR


def analyse(n, l, seed, sat, Dmax):
    t0 = time.time()
    K, a2, a6, basis, z = build_instance(n, l, seed, sat)
    N = 2 * l
    X1 = {1 << j: basis[j] for j in range(l)}
    X2 = {1 << (l + j): basis[j] for j in range(l)}
    Z = {0: z}
    e2 = kp_add(kp_mul(X1, X2, K), kp_mul(X1, Z, K), kp_mul(X2, Z, K))
    S = kp_add(kp_mul(e2, e2, K), kp_mul(kp_mul(X1, X2, K), Z, K), {0: a6})
    F = kp_coords(S, n)
    eqdeg = sorted(set(max(popcount(m) for m in f) for f in F))
    # Boolean solutions by brute force
    sols = []
    for ass in range(1 << N):
        x1 = 0
        x2 = 0
        for j in range(l):
            if (ass >> j) & 1:
                x1 ^= basis[j]
            if (ass >> (l + j)) & 1:
                x2 ^= basis[j]
        if semaev3_value(K, x1, x2, z, a6) == 0:
            sols.append(ass)
    C = Closure(N, Dmax)
    F_rows = [C.row(f) for f in F]
    # evaluation ranks for SAT: rank of B_{<=D} -> F_2^sols
    def eval_rank(D):
        if not sols:
            return 0
        rows = []
        for s in sols:
            r = 0
            for i, m in enumerate(C.monos):
                if popcount(m) <= D and (m & ~s) == 0:   # monomial = 1 at assignment s
                    r |= 1 << i
            rows.append(r)
        return C.rank_of(rows)

    out = dict(n=n, l=l, N=N, seed=seed, sat=sat, n_eqs=len(F), eq_degrees=eqdeg,
               n_boolean_solutions=len(sols), per_degree={}, Dstar=None)
    prev_filt = None
    prevW = None
    for D in range(1, Dmax + 1):
        piv = C.closure(F_rows, D)
        filt = C.filtration(piv, D)           # dim(W_D cap B_{<=e}), e=0..D
        dimB = sum(comb(N, e) for e in range(D + 1))
        rec = dict(dimW=len(piv), filt=filt, codim=dimB - len(piv))
        if prev_filt is not None:
            # Fall_D = (W_D cap B_{<=D-1}) / W_{D-1}; degree distribution via filtrations
            dimW_prev = prev_filt[-1]
            fall_dim = filt[D - 1] - dimW_prev
            # new fall elements of degree exactly e (as a filtration difference)
            pf = prev_filt + [dimW_prev] * (D - len(prev_filt))
            by_deg = [filt[e] - pf[e] - (filt[e - 1] - pf[e - 1] if e > 0 else 0) for e in range(D)]
            rec["fall_dim"] = fall_dim
            rec["fall_by_degree"] = by_deg
        else:
            rec["fall_dim"] = filt[0] if D == 1 else None
        one_in = C.contains(1 << C.const_col, piv)
        rec["one_in_W"] = one_in
        if sols:
            er = eval_rank(D)
            rec["eval_rank"] = er
            rec["W_equals_I(S)_leD"] = (dimB - len(piv) == er)
        if D == 3:
            rec["lemma_deg3"] = lemma_deg3_test(K, n, l, basis, z, a6, X1, X2, F, C, piv, filt)
        out["per_degree"][D] = rec
        if out["Dstar"] is None:
            if (not sols and one_in) or (sols and rec.get("W_equals_I(S)_leD")):
                out["Dstar"] = D
        prev_filt = filt
        if out["Dstar"] is not None and D >= out["Dstar"]:
            break
    out["seconds"] = round(time.time() - t0, 1)
    return out


def lemma_deg3_test(K, n, l, basis, z, a6, X1, X2, F, C, piv3, filt3):
    """Explicit n-independent falls of falldegree.tex lem:deg3, tested at this cell."""
    zinv = K.inv(z)
    u = kp_scale(kp_mul(X1, X2, K), zinv, K)
    # s^2 = sum_j (a_j + b_j) v_j^2 ; G(s^2) = sum_j (a_j+b_j) G(v_j^2)
    Gs2 = {}
    for j in range(l):
        g = K.halftrace(K.sq(basis[j]))
        if g:
            Gs2[1 << j] = Gs2.get(1 << j, 0) ^ g
            Gs2[1 << (l + j)] = Gs2.get(1 << (l + j), 0) ^ g
    Gs2 = {m: c for m, c in Gs2.items() if c}
    c0 = K.mul(a6, K.sq(zinv))                       # a6 / z^2
    w = kp_add(u, Gs2, {0: K.halftrace(c0)})
    s = kp_add(X1, X2)
    # l0 = Tr(s) + Tr(a6/z^2)
    l0 = set(m for m, c in s.items() if K.tr(c))
    if K.tr(c0):
        l0 ^= {0}
    l0 = frozenset(l0)
    # V^perp: lambda with Tr(lambda v_j) = 0 for all j  (nullspace over F_2)
    # matrix rows j: bits i -> Tr(x^i v_j)
    rows = []
    for j in range(l):
        r = 0
        for i in range(n):
            if K.tr(K.mul(1 << i, basis[j])):
                r |= 1 << i
        rows.append(r)
    # nullspace of the l x n matrix
    piv = {}
    for r in rows:
        while r:
            lb = r.bit_length() - 1
            if lb in piv:
                r ^= piv[lb]
            else:
                piv[lb] = r
                break
    # reduced form to read off nullspace
    pivcols = sorted(piv)
    # back-substitute to RREF
    for c in pivcols:
        for c2 in pivcols:
            if c2 != c and (piv[c2] >> c) & 1:
                piv[c2] ^= piv[c]
    free = [i for i in range(n) if i not in piv]
    Vperp = []
    for fcol in free:
        lam = 1 << fcol
        for c in pivcols:
            if (piv[c] >> fcol) & 1:
                lam |= 1 << c
        Vperp.append(lam)
    assert len(Vperp) == n - l
    for lam in Vperp:
        for v in basis:
            assert K.tr(K.mul(lam, v)) == 0
    explicit = []
    for lam in Vperp:
        for X in (X1, X2):
            explicit.append(kp_trace(kp_mul(kp_scale(X, lam, K), w, K), K))
    omega = kp_trace(w, K)
    f_last = set(kp_trace(kp_mul(s, w, K), K))
    if K.tr(c0):
        f_last ^= set(omega)
    explicit.append(frozenset(f_last))
    degs = [max((popcount(m) for m in f), default=0) for f in explicit]
    in_W3 = [C.contains(C.row(f), piv3) for f in explicit]
    l0_in_spanF = C.rank_of([C.row(f) for f in F] + [C.row(l0)]) == C.rank_of([C.row(f) for f in F])
    # span(F) + l0*B_{<=1} + explicit  vs  W_3 cap B_{<=2}
    gens = [C.row(f) for f in F] + [C.row(l0)]
    for v in range(C.N):
        gens.append(C.row(frozenset(m | (1 << v) for m in l0)))
    rank_F_l0 = C.rank_of(gens)
    gens_all = gens + [C.row(f) for f in explicit]
    rank_all = C.rank_of(gens_all)
    return dict(n_explicit=len(explicit), explicit_degrees=sorted(set(degs)),
                all_in_W3=all(in_W3), l0_in_spanF=l0_in_spanF,
                dim_spanF_plus_l0B1=rank_F_l0, dim_with_explicit=rank_all,
                dim_W3_cap_B2=filt3[2], explained_fraction_of_W3_deg_le2=(rank_all / filt3[2] if filt3[2] else None),
                residual_dim=filt3[2] - rank_all)


def main():
    outdir = os.path.dirname(os.path.abspath(__file__))
    cells = []
    for n in (11, 13, 15, 17, 19, 21):
        l = -(-n // 3)
        for seed in (1, 2):
            cells.append((n, l, seed, False))
        cells.append((n, l, 3, True))
    for n in (11, 13):
        l = -(-n // 2)
        cells.append((n, l, 11, False))
    results = []
    for (n, l, seed, sat) in cells:
        Dmax = min(2 * l, 6)
        r = analyse(n, l, seed, sat, Dmax)
        results.append(r)
        pd = r["per_degree"]
        summ = {D: (v["dimW"], v["filt"], v.get("fall_dim"), v.get("fall_by_degree"), v["one_in_W"]) for D, v in pd.items()}
        print(f"n={n} l={l} N={r['N']} seed={seed} sat={sat} eqs={r['n_eqs']} eqdeg={r['eq_degrees']} sols={r['n_boolean_solutions']} D*={r['Dstar']} t={r['seconds']}s")
        for D, v in pd.items():
            print(f"   D={D}: dimW={v['dimW']} filt={v['filt']} fall_dim={v.get('fall_dim')} fall_by_deg={v.get('fall_by_degree')} 1inW={v['one_in_W']}"
                  + (f" evalrank={v['eval_rank']} W=I(S)_<=D:{v['W_equals_I(S)_leD']}" if 'eval_rank' in v else ""))
            if "lemma_deg3" in v:
                print("        lemma_deg3:", json.dumps(v["lemma_deg3"]))
        sys.stdout.flush()
    with open(os.path.join(outdir, "RESULTS.json"), "w") as fh:
        json.dump(results, fh, indent=1)


if __name__ == "__main__":
    main()
