#!/usr/bin/env python3
"""syzygy-structure: exact GF(2) tests of hypotheses H_a, H_b, H_c (see RESULT.md preregistration)
on the selector system of Prop. S.  Independent of syz.c (own field arithmetic, own rows, own
elimination with Python integers as bit vectors); kernel dims are cross-checked against syz.

Usage: python3 struct.py N SEED E_MIN  -> one JSON line on stdout.

Column layout as in emin.c/syz.c: column j*nmons + t = coefficient of x^j in the ANF coefficient
of monomial mons[t] (mons = masks of popcount <= e in increasing order).
"""
import json, os, random, sys, time
import numpy as np

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.abspath(os.path.join(HERE, '..', '..', '..')))
from lfd_small import make_instance  # noqa: E402


class F:
    def __init__(s, n, poly):
        s.n, s.poly = n, poly

    def mul(s, a, b):
        r, top = 0, 1 << s.n
        while b:
            if b & 1:
                r ^= a
            b >>= 1
            a <<= 1
            if a & top:
                a ^= s.poly
        return r

    def sq(s, a):
        return s.mul(a, a)

    def pw(s, a, e):
        r = 1
        while e:
            if e & 1:
                r = s.mul(r, a)
            a = s.sq(a)
            e >>= 1
        return r

    def inv(s, a):
        return s.pw(a, (1 << s.n) - 2)

    def tr(s, a):
        t, x = 0, a
        for _ in range(s.n):
            t ^= x
            x = s.sq(x)
        assert t in (0, 1)
        return t


def popc(x):
    return bin(x).count('1')


class Elim:
    """Incremental GF(2) row space with pivot = highest bit."""

    def __init__(s):
        s.piv = {}

    def reduce(s, r):
        piv = s.piv
        while r:
            hb = r.bit_length() - 1
            p = piv.get(hb)
            if p is None:
                return r
            r ^= p
        return 0

    def add(s, r):
        r = s.reduce(r)
        if r:
            s.piv[r.bit_length() - 1] = r
            return True
        return False

    @property
    def rank(s):
        return len(s.piv)


def nullspace(rows, ncols):
    """Basis of {x : <r, x> = 0 for all r in rows} (right kernel), as ints."""
    E = Elim()
    for r in rows:
        E.add(r)
    # full RREF
    keys = sorted(E.piv)  # ascending pivot bits
    red = {}
    for b in keys:
        r = E.piv[b]
        # eliminate lower pivots from r
        x = r ^ (1 << b)
        while x:
            hb = x.bit_length() - 1
            if hb in red:
                r ^= red[hb]
                x = r ^ (1 << b)
                x &= (1 << hb) - 1
            else:
                x ^= 1 << hb
        red[b] = r
    pivset = set(red)
    basis = []
    # free columns
    colpiv = {}  # for each free column f, the pivots whose row has bit f
    for b, r in red.items():
        x = r ^ (1 << b)
        while x:
            hb = x.bit_length() - 1
            x ^= 1 << hb
            colpiv.setdefault(hb, []).append(b)
    for f in range(ncols):
        if f in pivset:
            continue
        v = 1 << f
        for b in colpiv.get(f, []):
            v |= 1 << b
        basis.append(v)
    return basis


def rank(vecs):
    E = Elim()
    for v in vecs:
        E.add(v)
    return E.rank


def span_dim_intersection(A, B):
    """dim(span A ∩ span B) = rank A + rank B - rank(A ∪ B)."""
    return rank(A) + rank(B) - rank(list(A) + list(B))


def Mbin(k, e):
    from math import comb
    return sum(comb(k, d) for d in range(0, e + 1)) if e >= 0 else 0


class Sys:
    def __init__(s, n, seed, a6=None):
        inst = make_instance(n, seed)
        s.inst = inst
        s.n, s.k = n, inst['k']
        s.K = F(n, inst['K'].poly)
        s.a6, s.z, s.V = inst['a6'], inst['z'], inst['V']
        if a6 is not None:  # opt-in override (Addendum A); default unchanged
            s.a6 = a6
        K, n, k = s.K, s.n, s.k
        s.P = 1 << k
        span = [0] * s.P
        for m in range(1, s.P):
            low = m & -m
            span[m] = span[m ^ low] ^ s.V[low.bit_length() - 1]
        s.span = span
        trp = []
        xm = 1
        for _ in range(2 * n - 1):
            trp.append(K.tr(xm))
            xm = K.mul(xm, 2)
        s.Tcol = [sum(1 << j for j in range(n) if trp[i + j]) for i in range(n)]
        s.zeta = K.inv(K.sq(s.z))
        s.t0 = K.tr(K.mul(s.a6, s.zeta))
        s.ell0 = [K.tr(span[c]) ^ s.t0 for c in range(s.P)]
        # rows: R[c][i] = trrow(Phi*_C(v_i)) for i<k, R[c][k] = trrow(h_C); also the raw field elts
        s.W = []
        s.R = []
        for c in range(s.P):
            C = span[c]
            w = [s.phistar(C, v) for v in s.V] + [s.hC(C)]
            s.W.append(w)
            s.R.append([s.trrow(x) for x in w])
        s.Rnp = np.array(s.R, dtype=np.uint64)  # (P, k+1)

    def trrow(s, w):
        r = 0
        for b in range(s.n):
            if w >> b & 1:
                r ^= s.Tcol[b]
        return r

    def phistar(s, C, A):
        K = s.K
        a = K.sq(C) ^ s.z
        b = K.mul(s.z, C)
        A2 = K.sq(A)
        return K.sq(A2) ^ K.mul(a, A2) ^ K.mul(b, A)

    def hC(s, C):
        K = s.K
        return K.mul(K.sq(s.z), K.sq(C)) ^ s.a6

    # ---- degree-e layout
    def layout(s, e):
        s.e = e
        s.mons = [m for m in range(s.P) if popc(m) <= e]
        s.midx = {m: t for t, m in enumerate(s.mons)}
        s.nm = len(s.mons)
        s.ncols = s.n * s.nm
        # sub[c] = int over nm bits: monomials dividing c
        s.sub = [sum(1 << t for t, m in enumerate(s.mons) if m & c == m) for c in range(s.P)]

    def row(s, c, r):
        tr = s.R[c][r]
        v = 0
        sb = s.sub[c]
        for j in range(s.n):
            if tr >> j & 1:
                v |= sb << (j * s.nm)
        return v

    def rows(s, which):  # which: 'I' (k rows per point) or 'IS' (k+1 rows)
        nr = s.k + (1 if which == 'IS' else 0)
        return [s.row(c, r) for c in range(s.P) for r in range(nr)]

    def from_anf(s, coeffs):
        """coeffs: dict monomial-mask -> field element -> column int."""
        v = 0
        for m, b in coeffs.items():
            t = s.midx[m]
            for j in range(s.n):
                if b >> j & 1:
                    v ^= 1 << (j * s.nm + t)
        return v

    def to_anf(s, v):
        """column int -> numpy array over mons of field elements."""
        a = np.zeros(s.nm, dtype=np.uint64)
        nm = s.nm
        mask = (1 << nm) - 1
        for j in range(s.n):
            blk = (v >> (j * nm)) & mask
            while blk:
                t = (blk & -blk).bit_length() - 1
                blk &= blk - 1
                a[t] |= np.uint64(1 << j)
        return a

    def evaluate(s, v):
        """values lambda(C) for all c (numpy uint64, length P), by subset-sum (Moebius) transform."""
        L = np.zeros(s.P, dtype=np.uint64)
        a = s.to_anf(v)
        L[np.array(s.mons, dtype=np.int64)] = a
        for i in range(s.k):
            L = L.reshape(-1, 2, 1 << i)
            L[:, 1, :] ^= L[:, 0, :]
            L = L.reshape(-1)
        return L

    def violations(s, L, nr=None):
        """parity bits Tr(lambda(C) w_r) for all C, r (bool array (P, nr))."""
        nr = s.k + 1 if nr is None else nr
        x = s.Rnp[:, :nr] & L[:, None]
        return (np.bitwise_count(x) & 1).astype(np.uint8)

    def tr_vals(s, L):
        """the function C -> Tr(lambda(C) h_C) for all C, from the h trace row."""
        return (np.bitwise_count(s.Rnp[:, s.k] & L) & 1).astype(np.uint8)

    def mult_mono(s, v, i):
        """multiply a section by the coordinate c_i (multilinear); caller ensures degree fits."""
        a = s.to_anf(v)
        out = {}
        for t, m in enumerate(s.mons):
            b = int(a[t])
            if b:
                mm = m | (1 << i)
                out[mm] = out.get(mm, 0) ^ b
        return s.from_anf(out)

    def bits_to_int(s, arr):
        b = np.packbits(arr.reshape(-1).astype(np.uint8), bitorder='little')
        return int.from_bytes(b.tobytes(), 'little')

    def anf_deg_of_function(s, f):
        """Boolean function on V (uint8 array) -> ANF degree."""
        a = f.astype(np.uint8).copy()
        for i in range(s.k):
            a = a.reshape(-1, 2, 1 << i)
            a[:, 1, :] ^= a[:, 0, :]
            a = a.reshape(-1)
        nz = np.nonzero(a)[0]
        return max((popc(int(m)) for m in nz), default=-1)


def independent_check(S, v, nr):
    """pointwise field-arithmetic check Tr(lambda(C) w) = 0, no trace rows used."""
    L = S.evaluate(v)
    for c in range(S.P):
        lc = int(L[c])
        for r in range(nr):
            if S.K.tr(S.K.mul(lc, S.W[c][r])):
                return False
    return True


def stabiliser(S, Kb, maxeq=None, rng=None):
    """S(K) = {M in Mat_n(F2) : M K ⊆ K}; M_{ab} at bit a*n+b, (M y)_a = sum_b M_ab y_b.
    Equations: for mu in K, all C, r: parity(trrow_r & M mu(C)) = 0, i.e.
    sum_{a,b} M_ab trrow_r[a] mu(C)[b] = 0.  Uses random combinations of K and all points,
    then verifies each solution on every basis vector exactly."""
    n = S.n
    rng = rng or random.Random(1)
    E = Elim()
    full = n * n
    combos = 0
    Ls = [S.evaluate(v) for v in Kb]
    while combos < 3 * len(Kb) + 20:
        if combos < len(Kb):
            L = Ls[combos]
        else:
            L = np.zeros(S.P, dtype=np.uint64)
            for i in range(len(Kb)):
                if rng.random() < 0.5:
                    L ^= Ls[i]
        combos += 1
        before = E.rank
        for c in range(S.P):
            y = int(L[c])
            if not y:
                continue
            for r in range(S.k + 1):
                tr = S.R[c][r]
                eq = 0
                a = tr
                while a:
                    ai = (a & -a).bit_length() - 1
                    a &= a - 1
                    eq |= y << (ai * n)
                E.add(eq)
        if E.rank == full - 1 and combos > 5:
            break
        if E.rank == before and combos > len(Kb):
            break
    sol = nullspace(list(E.piv.values()), full)
    # exact verification: apply each M to every basis vector, check all rows
    ok = True
    for Mi in sol:
        M = [(Mi >> (a * n)) & ((1 << n) - 1) for a in range(n)]  # row a: bits b
        for L in Ls:
            ML = np.zeros(S.P, dtype=np.uint64)
            for a in range(n):
                ML |= ((np.bitwise_count(L & np.uint64(M[a])) & 1).astype(np.uint64) << np.uint64(a))
            if S.violations(ML).any():
                ok = False
                break
    return sol, ok


def main():
    n, seed, emin = int(sys.argv[1]), int(sys.argv[2]), int(sys.argv[3])
    t0 = time.time()
    out = {'n': n, 'seed': seed, 'e_min': emin}
    S = Sys(n, seed)
    k = S.k
    out['k'] = k
    out['rank_W_hist'] = {}
    for c in range(S.P):
        r = rank(S.R[c])
        out['rank_W_hist'][r] = out['rank_W_hist'].get(r, 0) + 1
    # dims of I_d, K_d for d = 0..emin
    Idims, Kdims, Rdims, Rdeg, Rfull = [], [], [], [], []
    Ib_prev = None
    res = {}
    for d in range(0, emin + 1):
        S.layout(d)
        Ib = nullspace(S.rows('I'), S.ncols)
        # rho on I_d
        Rv = [S.tr_vals(S.evaluate(v)) for v in Ib]
        Rints = [S.bits_to_int(x) for x in Rv]
        rR = rank(Rints)
        # kernel of rho restricted to I_d: combos of Ib with rho = 0
        # coefficient space: nullspace of the transpose map
        # Build matrix rows = points, cols = basis index
        P = S.P
        cols_by_point = []
        for c in range(P):
            w = 0
            for i, x in enumerate(Rv):
                if x[c]:
                    w |= 1 << i
            cols_by_point.append(w)
        comb = nullspace(cols_by_point, len(Ib))
        Kb = []
        for cv in comb:
            v = 0
            x = cv
            while x:
                i = (x & -x).bit_length() - 1
                x &= x - 1
                v ^= Ib[i]
            Kb.append(v)
        degs = [S.anf_deg_of_function(x) for x in Rv]
        # is R_d = RM(d+1)?  R_d ⊆ RM(d+1) iff max deg <= d+1; equality iff rank = M(k, d+1)
        Idims.append(len(Ib))
        Kdims.append(len(Kb))
        Rdims.append(rR)
        Rdeg.append(max(degs, default=-1))
        Rfull.append(rR == Mbin(k, d + 1) and max(degs, default=-1) <= d + 1)
        # rho(I_d) contains 1?
        one = S.bits_to_int(np.ones(P, dtype=np.uint8))
        E = Elim()
        for x in Rints:
            E.add(x)
        feas = E.reduce(one) == 0
        # ell0 * RM(d) inside R_d? dimension check
        res[d] = dict(Ib=Ib, Kb=Kb, Rv=Rv, Rints=Rints, feas=feas)
        sys.stderr.write(f"n={n} s={seed} d={d}: dim I={len(Ib)} dim K={len(Kb)} dim R={rR} "
                         f"maxdeg R={max(degs, default=-1)} feasible={feas} ({time.time()-t0:.0f}s)\n")
    out['dim_I'] = Idims
    out['dim_K'] = Kdims
    out['dim_R'] = Rdims
    out['maxdeg_R'] = Rdeg
    out['R_is_RM_d_plus_1'] = Rfull
    out['M_k_dplus1'] = [Mbin(k, d + 1) for d in range(emin + 1)]
    out['M_k_d'] = [Mbin(k, d) for d in range(emin + 1)]
    out['T_dim'] = [Mbin(k - 1, d - 1) for d in range(emin + 1)]
    out['feasible'] = [res[d]['feas'] for d in range(emin + 1)]
    # R_d = ell0 RM(d) iff dim R_d = M(k-1, d) (ell0 RM(d) ⊆ R_d always, via zeta q in I_d)
    out['R_is_ell0_RM_d'] = [Rdims[d] == Mbin(k - 1, d) for d in range(emin + 1)]

    # ---- work at e = emin and e = emin-1 in the emin layout
    e = emin
    S.layout(e)
    # re-embed lower-degree bases into the emin layout: recompute I_{e-1} in degree-e layout via
    # degree filtration (elements of I_e whose top part vanishes) -- simpler: recompute directly
    Ie = res[e]['Ib']
    Ke = res[e]['Kb']
    # H_c1 : I_{e-1} == zeta * RM(e-1)
    zeta_RM = []
    for m in S.mons:
        if popc(m) <= e - 1:
            zeta_RM.append(S.from_anf({m: S.zeta}))
    # I_{e-1} in degree-e layout: sections of I_e of degree <= e-1
    top_mask = 0
    for t, m in enumerate(S.mons):
        if popc(m) == e:
            for j in range(S.n):
                top_mask |= 1 << (j * S.nm + t)
    # elements of I_e with zero top part: nullspace of the top projection over the I_e basis
    low_cols = []  # rows = top columns, cols = basis index -> we need combos with top = 0
    # build per top-column bitmask over basis indices
    tops = [v & top_mask for v in Ie]
    colmap = {}
    for i, tv in enumerate(tops):
        x = tv
        while x:
            b = (x & -x).bit_length() - 1
            x &= x - 1
            colmap[b] = colmap.get(b, 0) | (1 << i)
    combI = nullspace(list(colmap.values()), len(Ie))
    Ilow = []
    for cv in combI:
        v = 0
        x = cv
        while x:
            i = (x & -x).bit_length() - 1
            x &= x - 1
            v ^= Ie[i]
        Ilow.append(v)
    out['dim_I_emin_minus_1_via_filtration'] = rank(Ilow)
    out['dim_zetaRM_e_minus_1'] = rank(zeta_RM)
    out['dim_I_e_minus_1_cap_zetaRM'] = span_dim_intersection(Ilow, zeta_RM)
    out['H_c1_holds'] = (rank(Ilow) == rank(zeta_RM) == out['dim_I_e_minus_1_cap_zetaRM'])
    out['H_c2_holds'] = Rfull[e]
    out['H_c_formula_rhs'] = Idims[e] - Mbin(k, e + 1) - Mbin(k - 1, e - 1)
    out['excess'] = Kdims[e] - Mbin(k - 1, e - 1)

    # ---- controls: T ⊆ K, independent pointwise check on a sample + full on all at small n
    T = []
    tv = [i for i in range(k) if S.K.tr(S.V[i])]
    for m in S.mons:
        if popc(m) > e - 1:
            continue
        coeffs = {}
        if not S.t0:
            coeffs[m] = coeffs.get(m, 0) ^ S.zeta
        for i in tv:
            mm = m | (1 << i)
            coeffs[mm] = coeffs.get(mm, 0) ^ S.zeta
        T.append(S.from_anf(coeffs))
    out['rank_T'] = rank(T)
    out['T_in_K'] = span_dim_intersection(T, Ke) == rank(T)
    rngc = random.Random(seed * 7 + n)
    sample = Ke if len(Ke) <= 200 else rngc.sample(Ke, 200)
    out['indep_check_K'] = all(independent_check(S, v, k + 1) for v in sample)
    out['indep_check_K_vectors'] = len(sample)

    # ---- H_c0: a feasible lambda_0 at e_min (affine solution set = lambda_0 + K)
    one = S.bits_to_int(np.ones(S.P, dtype=np.uint8))
    Rints = res[e]['Rints']
    # solve sum a_i Rints[i] = one by elimination with tracking
    piv = {}
    for i, x in enumerate(Rints):
        tag = 1 << i
        while x:
            hb = x.bit_length() - 1
            if hb in piv:
                px, pt = piv[hb]
                x ^= px
                tag ^= pt
            else:
                piv[hb] = (x, tag)
                break
    x, tag = one, 0
    while x:
        hb = x.bit_length() - 1
        if hb not in piv:
            break
        px, pt = piv[hb]
        x ^= px
        tag ^= pt
    lam0 = None
    if x == 0:
        lam0 = 0
        t = tag
        while t:
            i = (t & -t).bit_length() - 1
            t &= t - 1
            lam0 ^= Ie[i]
        L0 = S.evaluate(lam0)
        viol = S.violations(L0)
        okI = not viol[:, :k].any()
        okII = bool((viol[:, k] == 1).all())
        out['lambda0_found'] = True
        out['lambda0_verified_all_points'] = okI and okII
        # pointwise field-arithmetic check
        good = True
        for c in range(S.P):
            lc = int(L0[c])
            for r in range(k):
                if S.K.tr(S.K.mul(lc, S.W[c][r])):
                    good = False
            if S.K.tr(S.K.mul(lc, S.W[c][k])) != 1:
                good = False
        out['lambda0_indep_check'] = good
        out['affine_solution_dim'] = len(Ke)
    else:
        out['lambda0_found'] = False

    # ---- H_a: K_e ⊆ G_e = I_{e-1} + sum_i c_i I_{e-1} ?
    G = list(Ilow)
    for v in Ilow:
        for i in range(k):
            G.append(S.mult_mono(v, i))
    rG = rank(G)
    out['dim_G_e'] = rG
    out['dim_K_cap_G'] = span_dim_intersection(Ke, G)
    out['H_a_holds'] = out['dim_K_cap_G'] == len(Ke)

    # ---- H_a': exists lambda in lambda0 + K with deg(ell0 (lambda + zeta)) <= e ?
    if lam0 is not None:
        # ell0 = t0 + sum_{i in tv} c_i ; multiply in a degree-(e+1) layout
        def ell0_times(v):
            a = S.to_anf(v)
            outc = {}
            for t, m in enumerate(S.mons):
                b = int(a[t])
                if not b:
                    continue
                if S.t0:
                    outc[m] = outc.get(m, 0) ^ b
                for i in tv:
                    mm = m | (1 << i)
                    outc[mm] = outc.get(mm, 0) ^ b
            # top part (degree e+1) as an int keyed by (mask, j)
            top = 0
            for mm, b in outc.items():
                if popc(mm) == e + 1 and b:
                    top ^= b << (mm * S.n)  # unique encoding
            return top
        tops_K = [ell0_times(v) for v in Ke]
        top0 = ell0_times(lam0)  # zeta part has degree 1 <= e
        Eh = Elim()
        for x in tops_K:
            Eh.add(x)
        out['H_a_prime_exists'] = Eh.reduce(top0) == 0
        out['rank_top_ell0K'] = Eh.rank

    # ---- H_b: value stabiliser algebra of K_e, of T_e (positive control)
    sol, okv = stabiliser(S, Ke)
    out['stab_K_dim'] = len(sol)
    out['stab_K_verified'] = okv
    ident = sum(1 << (a * S.n + a) for a in range(S.n))
    out['stab_K_is_scalars'] = len(sol) == 1 and sol[0] == ident
    if n <= 17:
        solT, okT = stabiliser(S, T)
        out['stab_T_dim'] = len(solT)
        out['stab_T_expected'] = S.n * S.n - S.n + 1
        out['stab_T_verified'] = okT
    # translations: dim {mu in K : mu(.+v_i) in K} for each basis vector, and K/T version
    trans = []
    Ls = [S.evaluate(v) for v in Ke]
    for i in range(k):
        perm = np.arange(S.P) ^ (1 << i)
        vi = []
        for L in Ls:
            vi.append(S.bits_to_int(S.violations(L[perm])))
        r = rank(vi)
        trans.append(len(Ke) - r)
    out['translation_stable_dims'] = trans
    out['dim_K'] = Kdims
    out['seconds'] = round(time.time() - t0, 1)
    print(json.dumps(out))


def rank_rows(rows):
    return rank(rows)


if __name__ == '__main__':
    main()
