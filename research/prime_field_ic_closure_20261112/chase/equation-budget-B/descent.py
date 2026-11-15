#!/usr/bin/env python3
"""Toy Weil-descent experiments for E/GF(p) viewed over GF(p^k).  [pcc:unverified]

Experiments
  E1  span: what subgroup of E(GF(p^k)) do points with x in GF(p) generate?
  E2  k=2, m=2 descent: S3(x1,x2,xR')=0 with xR' in GF(p^2)\\GF(p) splits into 2
      GF(p) equations in 2 unknowns.  Count GF(p) solutions by brute force, compute
      the sympy Groebner basis over GF(p), and check every solution against the
      trivial projection R' = A + B (A in E(GF(p)), B in the twist).
  E3  k=2, m=3: descent gives 2 equations in 3 unknowns.  Count all decompositions
      and check that the count equals the count predicted by "decompose A over
      GF(p)" x "decompose B over the twist" (no new information).
  E4  k=3, m=3: base x in GF(p) has no twist part (odd k); verify no target outside
      E(GF(p)) decomposes, and that the Groebner variety has points only outside GF(p).
  E5  subspace base V = theta*GF(p) in GF(p^2): yield per trial and base size, i.e.
      the cost of IC in E(GF(p^2)) used to solve a DLP that lives in E(GF(p)).
"""
import itertools, random, sys, time, json
from sympy import symbols, groebner, Poly, GF as sGF

# ---------------- extension field GF(p^k) = GF(p)[t]/(f) -----------------
class Ext:
    def __init__(self, p, mod):           # mod: monic coeffs c0..c_{k-1} of t^k = -sum c_i t^i
        self.p, self.k, self.mod = p, len(mod), mod
    def red(self, c):
        p, k = self.p, self.k
        c = [x % p for x in c]
        for i in range(len(c) - 1, k - 1, -1):
            a = c[i]
            if a:
                c[i] = 0
                for j in range(k):
                    c[i - k + j] = (c[i - k + j] - a * self.mod[j]) % p
        return tuple(c[:k] + [0] * (k - len(c)))
    def add(s, a, b): return tuple((x + y) % s.p for x, y in zip(a, b))
    def sub(s, a, b): return tuple((x - y) % s.p for x, y in zip(a, b))
    def neg(s, a): return tuple((-x) % s.p for x in a)
    def mul(s, a, b):
        c = [0] * (2 * s.k - 1)
        for i, x in enumerate(a):
            if x:
                for j, y in enumerate(b):
                    c[i + j] += x * y
        return s.red(c)
    def pw(s, a, e):
        r = s.one();
        while e:
            if e & 1: r = s.mul(r, a)
            a = s.mul(a, a); e >>= 1
        return r
    def inv(s, a): return s.pw(a, s.p ** s.k - 2)
    def one(s): return (1,) + (0,) * (s.k - 1)
    def zero(s): return (0,) * s.k
    def c(s, x): return (x % s.p,) + (0,) * (s.k - 1)
    def isbase(s, a): return all(x == 0 for x in a[1:])
    def sqrt(s, a):  # brute-ish Tonelli for small fields: use exponent trick when q=3 mod 4 else search
        q = s.p ** s.k
        if a == s.zero(): return s.zero()
        if s.pw(a, (q - 1) // 2) != s.one(): return None
        if q % 4 == 3: return s.pw(a, (q + 1) // 4)
        # Tonelli-Shanks
        Q, S = q - 1, 0
        while Q % 2 == 0: Q //= 2; S += 1
        z = None
        for cand in itertools.product(range(s.p), repeat=s.k):
            if cand != s.zero() and s.pw(cand, (q - 1) // 2) != s.one(): z = cand; break
        M, cc, t, R = S, s.pw(z, Q), s.pw(a, Q), s.pw(a, (Q + 1) // 2)
        while t != s.one():
            i, tt = 0, t
            while tt != s.one(): tt = s.mul(tt, tt); i += 1
            b = s.pw(cc, 1 << (M - i - 1))
            M, cc, t, R = i, s.mul(b, b), s.mul(t, s.mul(b, b)), s.mul(R, b)
        return R

def irreducible(p, k):
    # find monic t^k + ... irreducible over GF(p) by brute force (small p only)
    for coeffs in itertools.product(range(p), repeat=k):
        if coeffs[0] == 0: continue
        F = Ext(p, list(coeffs))
        # irreducible iff no root in GF(p) for k<=3; for k=2,3 sufficient
        if k <= 3 and all(sum(c * pow(x, i, p) for i, c in enumerate(coeffs)) + pow(x, k, p) != 0 for x in range(p) if True) \
           and all((pow(x, k, p) + sum(c * pow(x, i, p) for i, c in enumerate(coeffs))) % p for x in range(p)):
            return list(coeffs)
    raise ValueError

# ---------------- curve arithmetic over Ext -----------------
O = None
class Curve:
    def __init__(s, F, a, b): s.F, s.a, s.b = F, F.c(a), F.c(b)
    def rhs(s, x):
        F = s.F; return F.add(F.add(F.mul(F.mul(x, x), x), F.mul(s.a, x)), s.b)
    def add(s, P, Q):
        F = s.F
        if P is O: return Q
        if Q is O: return P
        if P[0] == Q[0]:
            if F.add(P[1], Q[1]) == F.zero(): return O
            lam = F.mul(F.add(F.mul(F.c(3), F.mul(P[0], P[0])), s.a), F.inv(F.mul(F.c(2), P[1])))
        else:
            lam = F.mul(F.sub(Q[1], P[1]), F.inv(F.sub(Q[0], P[0])))
        x3 = F.sub(F.sub(F.mul(lam, lam), P[0]), Q[0])
        return (x3, F.sub(F.mul(lam, F.sub(P[0], x3)), P[1]))
    def neg(s, P): return O if P is O else (P[0], s.F.neg(P[1]))
    def mul(s, n, P):
        R = O
        if n < 0: n, P = -n, s.neg(P)
        while n:
            if n & 1: R = s.add(R, P)
            P = s.add(P, P); n >>= 1
        return R

def points_x_in_base(E):
    """All affine points of E(GF(p^k)) whose x lies in GF(p)."""
    F = E.F; out = []
    for x0 in range(F.p):
        x = F.c(x0); y = F.sqrt(E.rhs(x))
        if y is None: continue
        out.append((x, y))
        if y != F.zero(): out.append((x, F.neg(y)))
    return out

def count_base_points(p, a, b):
    n = 1
    for x in range(p):
        r = (x * x * x + a * x + b) % p
        n += 1 if r == 0 else (2 if pow(r, (p - 1) // 2, p) == 1 else 0)
    return n

# ---------------- Semaev S3 and descent -----------------
def S3(x1, x2, x3, a, b):
    return (x1 - x2) ** 2 * x3 ** 2 - 2 * ((x1 + x2) * (x1 * x2 + a) + 2 * b) * x3 + (x1 * x2 - a) ** 2 - 4 * b * (x1 + x2)

def descend(expr_coeffs_in_t, F, gens):
    """expr given as sympy poly in gens and t; reduce mod f(t) and return GF(p) components."""
    raise NotImplementedError

def main():
    rng = random.Random(20261112)
    res = {}
    # ======================== k = 2 ========================
    for p in (101, 107):
        while True:
            a, b = rng.randrange(p), rng.randrange(p)
            if (4 * a ** 3 + 27 * b ** 2) % p: break
        n = None
        for x in range(2, p):
            if pow(x, (p - 1) // 2, p) == p - 1: n = x; break
        F = Ext(p, [(-n) % p, 0])        # t^2 = n
        E = Curve(F, a, b)
        NE = count_base_points(p, a, b); NT = 2 * p + 2 - NE
        base = points_x_in_base(E)
        nE = sum(1 for P in base if F.isbase(P[1])); nT = len(base) - nE
        # E1: span check -- is every base point in E(F_p) or twist-embedded (y in t*F_p)?
        tw_ok = all(F.isbase(P[1]) or P[1][0] == 0 for P in base)
        # group order of E(F_p2) = NE*NT
        trE = p + 1 - NE
        NE2 = p * p + 1 - (trE * trE - 2 * p)
        r = {"p": p, "a": a, "b": b, "nonres": n, "#E(Fp)": NE, "#Et(Fp)": NT, "#E(Fp2)": NE2,
             "NE*NT==#E(Fp2)": NE * NT == NE2, "base_x_in_Fp_points": len(base),
             "base_in_E(Fp)": nE, "base_on_twist(y in t*Fp)": nT, "every_base_pt_in_E(Fp)_or_twist": tw_ok}
        # sums of base points: set reached by 2-sums -- check they lie in E(Fp)+Et(Fp) trivially (= whole group if gcd=1)
        # E2: k=2,m=2 descent for random targets R' with x not in F_p
        sumset = {}
        ptsO = base + [O]
        for P1 in base:
            for P2 in base:
                S = E.add(P1, P2)
                if S is O: continue
                sumset.setdefault(S, []).append((P1, P2))
        # pick random targets: random point with x in F_p2 \ F_p
        targets = []
        while len(targets) < 40:
            x = (rng.randrange(p), rng.randrange(1, p))
            y = F.sqrt(E.rhs(x))
            if y is None: continue
            targets.append((x, y))
        trivial = nontrivial = nodec = 0; sols_per_target = []
        for R in targets:
            decs = sumset.get(R, [])
            # ordered pairs; unordered count
            uniq = {frozenset([P1, P2]) for P1, P2 in decs}
            sols_per_target.append(len(uniq))
            if not uniq: nodec += 1
            for s in uniq:
                s = list(s)
                if len(s) == 1: s = s * 2
                kinds = sorted(F.isbase(P[1]) for P in s)
                if kinds == [False, True]: trivial += 1
                else: nontrivial += 1
        r["E2_targets"] = len(targets)
        r["E2_unordered_decomps_hist"] = {str(v): sols_per_target.count(v) for v in sorted(set(sols_per_target))}
        r["E2_decomps_of_type(E(Fp)pt + twist pt)"] = trivial
        r["E2_decomps_other_type"] = nontrivial
        r["E2_targets_with_no_decomp"] = nodec
        # E3: m=3 decompositions (unordered multisets) vs prediction from GF(p)-only decompositions
        if p == 101:
            bE = [P for P in base if F.isbase(P[1])]; bT = [P for P in base if not F.isbase(P[1])]
            setbase = set(base)
            def proj(R):
                # R = A + B with A in E(Fp) (order NE), B in twist (order NT); valid when gcd(NE,NT)=1
                from math import gcd
                assert gcd(NE, NT) == 1
                u = pow(NT, -1, NE); v = pow(NE, -1, NT)
                A = E.mul(NT * u, R); B = E.mul(NE * v, R)
                return A, B
            tot = pred = 0; detail = []
            for R in targets[:12]:
                A, B = proj(R)
                # count ordered triples (P1,P2,P3) in base^3 with sum R
                cnt = 0
                for (P1, P2), _ in [((k[0], k[1]), None) for S, lst in sumset.items() for k in lst]:
                    pass
                cnt = 0
                for S, lst in sumset.items():
                    D = E.add(R, E.neg(S))
                    if D in setbase: cnt += len(lst)
                # prediction: choose which of 3 slots are E(Fp) (j of them) and which twist (3-j);
                # A must be sum of j E(Fp)-base pts, B of (3-j) twist pts
                cache = {}
                def dist(pts, j, key):
                    if (key, j) in cache: return cache[(key, j)]
                    d = {O: 1}
                    for _ in range(j):
                        nd = {}
                        for S, c in d.items():
                            for P in pts:
                                T = E.add(S, P); nd[T] = nd.get(T, 0) + c
                        d = nd
                    cache[(key, j)] = d; return d
                def nsum(target, pts, j):
                    key = 'E' if pts is bE else 'T'
                    if j == 3:  # use 2-sum dist and one more point
                        d2 = dist(pts, 2, key)
                        return sum(d2.get(E.add(target, E.neg(P)), 0) for P in pts)
                    return dist(pts, j, key).get(target, 0)
                pr = 0
                from math import comb
                for j in range(4):
                    if j == 3 or j == 0:
                        x = nsum(A, bE, j) * nsum(B, bT, 3 - j) if (j in (0, 3)) else 0
                    else:
                        x = nsum(A, bE, j) * nsum(B, bT, 3 - j)
                    pr += comb(3, j) * x
                tot += cnt; pred += pr; detail.append((cnt, pr))
            r["E3_m3_ordered_counts_(observed,predicted_from_Fp_and_twist_splits)"] = detail
            r["E3_all_equal"] = all(c == q for c, q in detail)
        res[f"k2_p{p}"] = r
        print(json.dumps(r), flush=True)
    return res

if __name__ == "__main__":
    main()
