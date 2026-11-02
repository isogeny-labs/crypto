#!/usr/bin/env python3
"""Prototype for MP-symmetry-1 (Z/3 instance): unchained Weil-descended binary S_4
with x_1,x_2,x_3 in the same l-dim F_2-subspace V of F_{2^n}; the system is invariant
under the cyclic permutation g of the three summand coordinate blocks.  |Z/3| odd, so
the Reynolds operator R = 1+g+g^2 exists over F_2 and 1 in W_c  <=>  1 in R(W_c).
We build the Boolean Macaulay matrix at degree c (rows: monomial multiples of the n
descended components, reduced in B = F_2[x]/(x^2-x)), compute the closure degree
c* = min{c : 1 in W_c} on the FULL matrix and on the ORBIT-SUM (trivial-character)
matrix, and report sizes.  Pure python/numpy, F_2 bit-packed elimination.
"""
import sys, random, itertools, time
import numpy as np

# ---------- F_{2^n} arithmetic (polynomial basis, ints) ----------
MODS = {5: 0b100101, 7: 0b10000011, 9: 0b1000010001, 11: 0b100000000101, 13: 0b10000000011011}

def fmul(a, b, n, mod):
    r = 0
    while b:
        if b & 1: r ^= a
        b >>= 1; a <<= 1
        if a >> n: a ^= mod
    return r

def fpow(a, e, n, mod):
    r = 1
    while e:
        if e & 1: r = fmul(r, a, n, mod)
        a = fmul(a, a, n, mod); e >>= 1
    return r

def finv(a, n, mod):
    return fpow(a, (1 << n) - 2, n, mod)

# ---------- Boolean polynomials: dict{monomial(frozenset): 1} as set of monomials ----------
def padd(p, q):
    return p ^ q  # sets of monomials, symmetric difference

def pmul(p, q):
    r = set()
    for a in p:
        for b in q:
            m = a | b
            if m in r: r.remove(m)
            else: r.add(m)
    return frozenset(r)

# ---------- field element with Boolean-polynomial coordinates ----------
# represent an element of F_{2^n}[Boolean vars] as list of n Boolean polys (coordinate i)
def elem_const(c, n):
    return [frozenset([frozenset()]) if (c >> i) & 1 else frozenset() for i in range(n)]

def elem_add(A, B):
    return [padd(a, b) for a, b in zip(A, B)]

def elem_mul(A, B, n, mod):
    # schoolbook: sum_{i,j} A_i B_j * (t^{i+j} mod f) ; t^k reduced constants
    res = [frozenset() for _ in range(n)]
    tk = [fpow(2, k, n, mod) for k in range(2 * n - 1)]  # t^k as int
    prods = {}
    for i in range(n):
        if not A[i]: continue
        for j in range(n):
            if not B[j]: continue
            prods.setdefault(i + j, [])
            prods[i + j].append(pmul(A[i], B[j]))
    for k, lst in prods.items():
        s = frozenset()
        for p in lst: s = padd(s, p)
        c = tk[k]
        for b in range(n):
            if (c >> b) & 1: res[b] = padd(res[b], s)
    return res

def elem_sq(A, n, mod):
    # Frobenius is F_2-linear on coordinates: x^2 = sum_i A_i t^{2i}
    res = [frozenset() for _ in range(n)]
    for i in range(n):
        if not A[i]: continue
        c = fpow(2, 2 * i, n, mod)
        for b in range(n):
            if (c >> b) & 1: res[b] = padd(res[b], A[i])
    return res

def S3_elem(X1, X2, X3, b6, n, mod):
    # binary S_3 = (x1x2+x1x3+x2x3)^2 + x1x2x3 + b
    e2 = elem_add(elem_add(elem_mul(X1, X2, n, mod), elem_mul(X1, X3, n, mod)), elem_mul(X2, X3, n, mod))
    e3 = elem_mul(elem_mul(X1, X2, n, mod), X3, n, mod)
    return elem_add(elem_add(elem_sq(e2, n, mod), e3), elem_const(b6, n))

# S_4 = Res_X(S3(x1,x2,X), S3(x3,x4,X)); binary S3 is quadratic in X: S3 = a X^2 + b X + c
# with a = (x1+x2)^2, b = x1 x2 (coefficient of X from x1x2X), c = (x1x2)^2 + b6.
# Resultant of a1 X^2+b1 X+c1 and a2 X^2+b2 X+c2 (Sylvester 4x4) =
#   (a1 c2 - a2 c1)^2 - (a1 b2 - a2 b1)(b1 c2 - b2 c1)   (char 2: minus = plus)
def S4_elem(X1, X2, X3, X4, b6, n, mod):
    def coeffs(P, Q):
        a = elem_sq(elem_add(P, Q), n, mod)
        b = elem_mul(P, Q, n, mod)
        c = elem_add(elem_sq(b, n, mod), elem_const(b6, n))
        return a, b, c
    a1, b1, c1 = coeffs(X1, X2)
    a2, b2, c2 = coeffs(X3, X4)
    t1 = elem_add(elem_mul(a1, c2, n, mod), elem_mul(a2, c1, n, mod))
    t2 = elem_add(elem_mul(a1, b2, n, mod), elem_mul(a2, b1, n, mod))
    t3 = elem_add(elem_mul(b1, c2, n, mod), elem_mul(b2, c1, n, mod))
    return elem_add(elem_sq(t1, n, mod), elem_mul(t2, t3, n, mod))

# ---------- curve points / ground truth ----------
def trace(a, n, mod):
    t, s = a, a
    for _ in range(n - 1):
        s = fmul(s, s, n, mod); t ^= s
    return t

def lifts(x, a2, b6, n, mod):
    # y^2 + x y = x^3 + a2 x^2 + b6 has a solution iff Tr(x + a2 + b6/x^2) = 0 (x != 0)
    if x == 0: return True
    rhs = x ^ a2 ^ fmul(b6, finv(fmul(x, x, n, mod), n, mod), n, mod)
    return trace(rhs, n, mod) == 0

def main():
    n = int(sys.argv[1]) if len(sys.argv) > 1 else 7
    l = int(sys.argv[2]) if len(sys.argv) > 2 else 3
    seed = int(sys.argv[3]) if len(sys.argv) > 3 else 1
    cmax = int(sys.argv[4]) if len(sys.argv) > 4 else 9
    mod = MODS[n]; a2, b6 = 0, 1  # K_0-like model y^2+xy=x^3+1
    rng = random.Random(seed)
    # random l-dim subspace basis
    while True:
        basis = [rng.randrange(1, 1 << n) for _ in range(l)]
        # independence check
        span = {0}
        for b in basis: span |= {s ^ b for s in span}
        if len(span) == 1 << l: break
    V = sorted(span)
    # Boolean variables: block i (i=0,1,2) coords j=0..l-1 -> index i*l+j
    N = 3 * l
    def Xelem(i):
        E = [frozenset() for _ in range(n)]
        for j in range(l):
            var = frozenset([frozenset([i * l + j])])
            for bit in range(n):
                if (basis[j] >> bit) & 1: E[bit] = padd(E[bit], var)
        return E
    X = [Xelem(i) for i in range(3)]
    # choose an UNSAT target: x_R such that no (x1,x2,x3) in V^3 gives S4 = 0 with points on E
    # ground truth via S4 evaluation over all V^3 (cheap: 2^(3l))
    def S4_int(x1, x2, x3, x4):
        def co(p, q):
            a = fmul(p ^ q, p ^ q, n, mod); b = fmul(p, q, n, mod); c = fmul(b, b, n, mod) ^ b6
            return a, b, c
        a1, b1, c1 = co(x1, x2); a2_, b2, c2 = co(x3, x4)
        t1 = fmul(a1, c2, n, mod) ^ fmul(a2_, c1, n, mod)
        t2 = fmul(a1, b2, n, mod) ^ fmul(a2_, b1, n, mod)
        t3 = fmul(b1, c2, n, mod) ^ fmul(b2, c1, n, mod)
        return fmul(t1, t1, n, mod) ^ fmul(t2, t3, n, mod)
    targets = []
    for xr in range(1, 1 << n):
        if not lifts(xr, a2, b6, n, mod): continue
        nsol = sum(1 for x1 in V for x2 in V for x3 in V if S4_int(x1, x2, x3, xr) == 0)
        targets.append((nsol, xr))
    unsat = [xr for ns, xr in targets if ns == 0]
    sat = [xr for ns, xr in targets if ns > 0]
    print(f"n={n} l={l} |V|={len(V)} targets on E: {len(targets)} unsat: {len(unsat)} sat: {len(sat)}")
    xr = rng.choice(unsat)
    t0 = time.time()
    F = S4_elem(X[0], X[1], X[2], elem_const(xr, n), b6, n, mod)  # n Boolean polys (sets of monomials)
    gens = [f for f in F if f]
    degs = [max(len(m) for m in f) for f in gens]
    print(f"built {len(gens)} descended components, Boolean degrees {degs}, {time.time()-t0:.1f}s")
    # Z/3 action: cyclic shift of blocks
    def gmap(v): return ((v // l + 1) % 3) * l + (v % l)
    def gmono(m): return frozenset(gmap(v) for v in m)
    def gpoly(p): return frozenset(gmono(m) for m in p)
    inv = all(gpoly(f) in set(gens) or gpoly(f) in {frozenset()} for f in gens)
    # the SET of components need not be permuted (components are F_2-coordinates of a G-invariant
    # field element), but the SPAN is: check span invariance by rank
    def polys_to_rows(polys, cols):
        idx = {m: i for i, m in enumerate(cols)}
        M = np.zeros((len(polys), len(cols)), dtype=np.uint8)
        for r, p in enumerate(polys):
            for m in p: M[r, idx[m]] = 1
        return M
    def rank_f2(M):
        M = M.copy(); r = 0; rows, cols = M.shape
        for c in range(cols):
            piv = np.nonzero(M[r:, c])[0]
            if len(piv) == 0: continue
            p = r + piv[0]
            if p != r: M[[r, p]] = M[[p, r]]
            others = np.nonzero(M[:, c])[0]
            others = others[others != r]
            M[others] ^= M[r]
            r += 1
            if r == rows: break
        return r, M
    allmons = set()
    for f in gens: allmons |= set(f)
    for f in gens: allmons |= set(gpoly(f))
    cols = sorted(allmons, key=lambda m: (len(m), sorted(m)))
    r1 = rank_f2(polys_to_rows(gens, cols))[0]
    r2 = rank_f2(polys_to_rows(gens + [gpoly(f) for f in gens], cols))[0]
    print(f"Z/3 invariance of the component span: rank {r1} -> {r2} (equal => invariant)")
    # Macaulay closure degree, full vs orbit-sum
    vars_ = list(range(N))
    def monos_upto(c):
        out = []
        for d in range(c + 1):
            out += [frozenset(s) for s in itertools.combinations(vars_, d)]
        return out
    results = {}
    for c in range(min(degs), cmax + 1):
        cols = monos_upto(c)
        rows = []
        for f, d in zip(gens, degs):
            for m in monos_upto(c - d):
                rows.append(pmul(f, frozenset([m])))
        rows = [r for r in rows if r]
        M = polys_to_rows(rows, cols)
        rk, R = rank_f2(M)
        # 1 in row space?  the constant monomial is column 0; after RREF, 1 in span iff some row == e_0
        one = np.zeros(len(cols), dtype=np.uint8); one[0] = 1
        in_full = any((R[i] == one).all() for i in range(rk))
        # orbit sums: columns = orbit-sum basis; rows = orbit sums of rows
        orbit_of = {}
        for m in cols:
            o = frozenset([m, gmono(m), gmono(gmono(m))])
            orbit_of[m] = o
        orbits = sorted(set(orbit_of.values()), key=lambda o: (len(min(o, key=len)), sorted(sorted(x) for x in o)))
        oidx = {o: i for i, o in enumerate(orbits)}
        Mo = np.zeros((len(rows), len(orbits)), dtype=np.uint8)
        for r_, p in enumerate(rows):
            # Reynolds: R(p) = p + g p + g^2 p ; in orbit-sum coordinates, coefficient of orbit o
            # = (# of monomials of p in o) mod 2 ... for size-3 orbits R(m) = orbit sum; size-1: R(m)=3m=m
            cnt = {}
            for m in p:
                o = orbit_of[m]; cnt[o] = cnt.get(o, 0) + 1
            for o, k in cnt.items():
                if k & 1: Mo[r_, oidx[o]] = 1
        rko, Ro = rank_f2(Mo)
        one_o = np.zeros(len(orbits), dtype=np.uint8); one_o[oidx[orbit_of[frozenset()]]] = 1
        in_triv = any((Ro[i] == one_o).all() for i in range(rko))
        results[c] = (in_full, in_triv, M.shape, rk, Mo.shape, rko)
        print(f"c={c}: full {M.shape} rank {rk} 1inW={in_full} | trivial-block {Mo.shape} rank {rko} 1inR(W)={in_triv}  ({time.time()-t0:.1f}s)")
        if in_full and in_triv: break
    print("AGREEMENT:", all(v[0] == v[1] for v in results.values()))

if __name__ == "__main__":
    main()
