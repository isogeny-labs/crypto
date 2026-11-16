#!/usr/bin/env python3
r"""Exact small-size checks for research/lfd_lower_bound_20261111/NOTE.md.

Binary Weil descent of Sem3 (m=2): E: y^2+xy = x^3+a2 x^2+a6 over GF(2^n),
X1 = sum a_j v_j, X2 = sum b_j v_j, V = span(v_1..v_k) random, target abscissa z
of a random curve point.  S = X1^2X2^2 + z^2(X1^2+X2^2) + z X1 X2 + a6.
The Boolean system F is the n coordinate functions of S (field equations implicit:
we work with multilinear representatives in B = F2[x]/(x_i^2+x_i)).

Quantities (all exact, over F2):
  dF   : least D with 1 in W_D(F)          (HKY/PC closure; Prop. bridges)
  dNS  : least D with 1 in M_D(F)          (plain Macaulay / Nullstellensatz degree)
  dSEL : least D with 1 in span{m(c) f : m a monomial in c = a+b only,
                                       deg m + deg f <= D}
         (the proof system of the treatise's k+2 upper bound; Note sec. 5)
  minrank : min over delta in ker(Tr)\{0} of rank_F2 of M_delta[i][j]=Tr(delta v_i v_j / z)
Generators are first replaced by a degree-reduced echelon basis of span(F),
so dNS and dSEL do not depend on the coordinate basis chosen for the descent.

Pure Python + numpy only.  Usage:
  python3 lfd_small.py --n 9 --seeds 1 2 3 [--ns] [--sel] [--maxD 6] [--restrict]
"""
import argparse, itertools, random, sys, time
import numpy as np

# ---------------- GF(2^n) ----------------
def find_irred(n):
    # lowest-weight irreducible: trinomial, else pentanomial
    def is_irred(poly):
        # Rabin/Ben-Or: x^(2^i) mod poly, gcd with x^(2^i)-x for i<=n/2
        def mulmod(a, b):
            r = 0
            while b:
                if b & 1: r ^= a
                b >>= 1; a <<= 1
                if a >> n & 1: a ^= poly
            return r
        def pgcd(a, b):
            while b:
                while a and a.bit_length() >= b.bit_length():
                    a ^= b << (a.bit_length() - b.bit_length())
                a, b = b, a
            return a
        x = 2; xp = x
        for i in range(1, n // 2 + 1):
            xp = mulmod(xp, xp)
            if pgcd(poly, xp ^ x) != 1: return False
        return True
    for t in range(1, n):
        p = (1 << n) | (1 << t) | 1
        if is_irred(p): return p
    for t1, t2, t3 in itertools.combinations(range(1, n), 3):
        p = (1 << n) | (1 << t1) | (1 << t2) | (1 << t3) | 1
        if is_irred(p): return p
    raise ValueError

class GF:
    def __init__(self, n):
        self.n = n; self.poly = find_irred(n)
    def mul(self, a, b):
        r = 0; n = self.n; p = self.poly
        while b:
            if b & 1: r ^= a
            b >>= 1; a <<= 1
            if a >> n & 1: a ^= p
        return r
    def sq(self, a): return self.mul(a, a)
    def pow(self, a, e):
        r = 1
        while e:
            if e & 1: r = self.mul(r, a)
            a = self.sq(a); e >>= 1
        return r
    def inv(self, a): return self.pow(a, (1 << self.n) - 2)
    def tr(self, a):
        t = 0; x = a
        for _ in range(self.n):
            t ^= x; x = self.sq(x)
        assert t in (0, 1)
        return t

def rank_f2(rows):
    piv = {}
    r = 0
    for v in rows:
        while v:
            p = v.bit_length() - 1
            if p in piv: v ^= piv[p]
            else: piv[p] = v; r += 1; break
    return r

# ---------------- instance ----------------
def make_instance(n, seed, k=None, coords='ab'):
    rng = random.Random(seed)
    K = GF(n)
    k = k if k is not None else (n + 1) // 2
    a2 = rng.randrange(2)
    while True:
        a6 = rng.randrange(1, 1 << n)
        # target: abscissa of a curve point: need Tr(z + a2 + a6/z^2) = 0
        z = rng.randrange(1, 1 << n)
        if K.tr(z ^ a2 ^ K.mul(a6, K.inv(K.sq(z)))) == 0: break
    while True:
        V = [rng.randrange(1, 1 << n) for _ in range(k)]
        if rank_f2(V) == k: break
    return dict(n=n, k=k, K=K, a2=a2, a6=a6, z=z, V=V, seed=seed)

def descend(inst, coords='ab', V=None):
    """ANF of the n coordinate functions of S. Variables 0..k-1 = a, k..2k-1 = b (or c=a+b)."""
    K = inst['K']; n = inst['n']; z = inst['z']; a6 = inst['a6']
    V = V if V is not None else inst['V']; k = len(V)
    N = 2 * k
    # span table: val[mask] for mask over k bits
    span = [0] * (1 << k)
    for m in range(1, 1 << k):
        low = m & -m; j = low.bit_length() - 1
        span[m] = span[m ^ low] ^ V[j]
    sq = [K.sq(x) for x in span]
    z2 = K.sq(z)
    vals = np.zeros(1 << N, dtype=np.int64)
    for i1 in range(1 << k):
        X1 = span[i1]
        for i2 in range(1 << k):
            if coords == 'ab':
                X2 = span[i2]; s2 = sq[i1] ^ sq[i2]
            else:  # second block is c = a+b, X2 = X1 + C
                X2 = X1 ^ span[i2]; s2 = sq[i2]
            S = K.mul(sq[i1], K.sq(X2)) ^ K.mul(z2, s2) ^ K.mul(z, K.mul(X1, X2)) ^ a6
            vals[i1 | (i2 << k)] = S
    nzero = int(np.count_nonzero(vals == 0))
    # Moebius transform (ANF), XOR on n-bit words
    a = vals.copy()
    for i in range(N):
        bit = 1 << i
        idx = np.arange(1 << N)
        sel = idx[(idx & bit) != 0]
        a[sel] ^= a[sel ^ bit]
    polys = []
    nz = np.nonzero(a)[0]
    for j in range(n):
        mons = [int(m) for m in nz if (int(a[m]) >> j) & 1]
        polys.append(mons)
    return polys, N, nzero

def popcount(x): return bin(x).count('1')

class Cols:
    def __init__(self, N, D):
        self.N, self.D = N, D
        mons = []
        for d in range(D + 1):
            for c in itertools.combinations(range(N), d):
                m = 0
                for i in c: m |= 1 << i
                mons.append(m)
        self.mons = mons
        self.idx = {m: i for i, m in enumerate(mons)}
        self.T = sum(1 for m in mons if popcount(m) < D)  # indices < T have degree < D
    def vec(self, monlist):
        v = 0
        for m in monlist: v ^= 1 << self.idx[m]
        return v
    def monomials(self, v):
        out = []
        while v:
            low = v & -v; out.append(self.mons[low.bit_length() - 1]); v ^= low
        return out
    def deg(self, v):
        return popcount(self.mons[v.bit_length() - 1]) if v else -1

def reduced_basis(polys, N):
    """Degree-reduced echelon basis of span(polys) (leading term w.r.t. degree order)."""
    D = max(popcount(m) for p in polys for m in p)
    C = Cols(N, D)
    piv = {}
    for p in polys:
        v = C.vec(p)
        while v:
            q = v.bit_length() - 1
            if q in piv: v ^= piv[q]
            else: piv[q] = v; break
    return [C.monomials(v) for v in piv.values()]

class Echelon:
    def __init__(self): self.piv = {}
    def reduce(self, v):
        piv = self.piv
        while v:
            q = v.bit_length() - 1
            r = piv.get(q)
            if r is None: return v
            v ^= r
        return 0
    def insert(self, v):
        v = self.reduce(v)
        if v: self.piv[v.bit_length() - 1] = v
        return v

def closure(gens, N, D, mult_vars=None):
    """W_D: returns (one_in, dim, dims of W_D cap B_{<=e} for e=0..D).
    mult_vars: if given, only multiplications by these variables are allowed (PC_c)."""
    mv = list(mult_vars) if mult_vars is not None else list(range(N))
    C = Cols(N, D)
    E = Echelon(); queue = []
    for g in gens:
        if max((popcount(m) for m in g), default=0) > D: continue
        r = E.insert(C.vec(g))
        if r and (r.bit_length() - 1) < C.T: queue.append(r)
    while queue:
        g = queue.pop()
        mons = C.monomials(g)
        for i in mv:
            b = 1 << i
            acc = {}
            for m in mons:
                mm = m | b
                acc[mm] = acc.get(mm, 0) ^ 1
            v = 0
            for mm, c in acc.items():
                if c: v ^= 1 << C.idx[mm]
            r = E.insert(v)
            if r and (r.bit_length() - 1) < C.T: queue.append(r)
        if 0 in E.piv: break
    dims = [0] * (D + 1)
    for q in E.piv: dims[popcount(C.mons[q])] += 1
    cum = list(itertools.accumulate(dims))
    return (0 in E.piv), len(E.piv), cum

def macaulay(gens, N, D, mult_vars=None):
    """1 in span{m f : deg m + deg f <= D, m a monomial in mult_vars (default all)}?"""
    C = Cols(N, D)
    mask = sum(1 << i for i in (mult_vars if mult_vars is not None else range(N)))
    E = Echelon()
    for g in gens:
        dg = max(popcount(m) for m in g)
        for m in C.mons:
            if popcount(m) + dg > D or (m & ~mask): continue
            acc = {}
            for t in g:
                mm = t | m; acc[mm] = acc.get(mm, 0) ^ 1
            v = 0
            for mm, c in acc.items():
                if c: v ^= 1 << C.idx[mm]
            E.insert(v)
            if 0 in E.piv: return True, len(E.piv)
    return (0 in E.piv), len(E.piv)

def least_D(fn, start, maxD):
    for D in range(start, maxD + 1):
        if fn(D): return D
    return None  # > maxD

def min_rank(inst):
    K = inst['K']; n = inst['n']; V = inst['V']; k = len(V)
    zi = K.inv(inst['z'])
    P = [[K.mul(K.mul(V[i], V[j]), zi) for j in range(k)] for i in range(k)]
    best = None; hist = {}
    for d in range(1, 1 << n):
        if K.tr(d): continue
        rows = []
        for i in range(k):
            r = 0
            for j in range(k):
                if K.tr(K.mul(d, P[i][j])): r |= 1 << j
            rows.append(r)
        rk = rank_f2(rows)
        hist[rk] = hist.get(rk, 0) + 1
        best = rk if best is None else min(best, rk)
    return best, dict(sorted(hist.items()))

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--n', type=int, required=True)
    ap.add_argument('--k', type=int, default=None)
    ap.add_argument('--seeds', type=int, nargs='+', default=[1])
    ap.add_argument('--unsat', type=int, default=0, help='scan seeds from seeds[0] until this many unsatisfiable instances are found (satisfiable ones are skipped silently but counted)')
    ap.add_argument('--maxD', type=int, default=6)
    ap.add_argument('--ns', action='store_true')
    ap.add_argument('--sel', action='store_true')
    ap.add_argument('--rank', action='store_true')
    ap.add_argument('--pcc', action='store_true', help='closure allowing multiplication by c=a+b variables only (dPCc)')
    ap.add_argument('--restrict', action='store_true', help='also compute dF on the codim-1 subspace V\'=span(v_1..v_{k-1})')
    args = ap.parse_args()
    seeds = args.seeds
    if args.unsat:
        seeds = itertools.count(args.seeds[0])
    found = 0; skipped = 0
    for seed in seeds:
        if args.unsat and found >= args.unsat: break
        t0 = time.time()
        inst = make_instance(args.n, seed, args.k)
        polys, N, nz = descend(inst, 'ab')
        if args.unsat:
            if nz: skipped += 1; continue
            found += 1
        gens = reduced_basis(polys, N)
        degs = sorted(max(popcount(m) for m in g) for g in gens)
        out = dict(n=args.n, k=inst['k'], seed=seed, poly=hex(inst['K'].poly), a2=inst['a2'],
                   a6=inst['a6'], z=inst['z'], V=inst['V'], sols=nz, gen_degs=degs)
        if nz == 0:
            dF = least_D(lambda D: closure(gens, N, D)[0], 1, args.maxD)
            out['dF'] = dF if dF is not None else '>%d' % args.maxD
            top = dF if dF is not None else args.maxD
            out['W_dims_at_dF_minus_1'] = closure(gens, N, top - 1)[2] if top and top > 1 else None
            if args.ns:
                d = least_D(lambda D: macaulay(gens, N, D)[0], 1, args.maxD)
                out['dNS'] = d if d is not None else '>%d' % args.maxD
            if args.sel:
                polc, _, _ = descend(inst, 'ac')
                gc = reduced_basis(polc, N)
                cvars = list(range(inst['k'], N))
                d = least_D(lambda D: macaulay(gc, N, D, cvars)[0], 1, args.maxD)
                out['dSEL'] = d if d is not None else '>%d' % args.maxD
            if args.pcc:
                polc2, _, _ = descend(inst, 'ac')
                gc2 = reduced_basis(polc2, N)
                cv = list(range(inst['k'], N))
                d = least_D(lambda D: closure(gc2, N, D, cv)[0], 1, args.maxD)
                out['dPCc'] = d if d is not None else '>%d' % args.maxD
            if args.restrict:
                Vp = inst['V'][:-1]
                pr, Np, nzp = descend(inst, 'ab', Vp)
                gp = reduced_basis(pr, Np)
                dp = least_D(lambda D: closure(gp, Np, D)[0], 1, args.maxD)
                out['dF_restricted_k-1'] = dp if dp is not None else '>%d' % args.maxD
                out['restricted_sols'] = nzp
        if args.rank:
            out['minrank'], out['rank_hist'] = min_rank(inst)
        out['sec'] = round(time.time() - t0, 1)
        print(out, flush=True)
    if args.unsat: print({'satisfiable_seeds_skipped': skipped}, flush=True)

if __name__ == '__main__':
    main()
