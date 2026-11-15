#!/usr/bin/env python3
"""Independent HKY closure on R = F_p[z_1..z_m, u]/(z_i^K - 1), Fourier/monomial basis (D2).
Column (k, j): monomial z^k u^j, degree = sum popcount(k_i) + j. Multiplication by the tower variable
y_{i,l} = z_i^(2^l) shifts k_i by 2^l mod K (a bijection of the index set); by u raises j.
V_{F,c} = least subspace W of R_{<=c} with F cap R_{<=c} in W and x_v (W cap R_{<c}) in W.
Dense mod-p linear algebra in float64 (exact: p < 2^16, sums < 2^53)."""
import sys, numpy as np
sys.path.insert(0, "/home/user/crypto/research/kummer_m3_ns_degree_20261115")
import ns_degree as nsd

pc = nsd.pc

def mod(a, p): return np.mod(a, p)

class Space:
    def __init__(s, C, c, has_u):
        s.C, s.c, s.has_u = C, c, has_u
        K, m = C.K, C.m
        s.K, s.m = K, m
        ks = np.indices((K,) * m).reshape(m, -1)            # (m, K^m)
        popsum = sum(C.popk[ks[i]] for i in range(m))
        nj = c + 1 if has_u else 1
        flat, js, degs = [], [], []
        for j in range(nj):
            sel = np.nonzero(popsum + j <= c)[0]
            flat.append(sel); js.append(np.full(len(sel), j)); degs.append(popsum[sel] + j)
        flat = np.concatenate(flat); js = np.concatenate(js); deg = np.concatenate(degs)
        order = np.lexsort((js, flat, -deg))                 # degree descending, deterministic
        s.flat, s.j, s.deg = flat[order], js[order], deg[order]
        s.n = len(s.flat)
        s.ks = ks[:, s.flat]
        s.colidx = -np.ones((K ** m, nj + 1), dtype=np.int64)
        s.colidx[s.flat, s.j] = np.arange(s.n)
        s.one_col = int(s.colidx[0, 0])
        assert s.deg[s.one_col] == 0 and s.one_col == s.n - 1
        s.vars = []                                          # list of target index arrays (-1 = out)
        for i in range(m):
            for l in range(C.t):
                k2 = s.ks.copy(); k2[i] = (k2[i] + (1 << l)) % K
                fl2 = np.zeros(s.n, dtype=np.int64)
                for q in range(m): fl2 = fl2 * K + k2[q]
                s.vars.append(s.colidx[fl2, s.j])
        if has_u:
            s.vars.append(s.colidx[s.flat, s.j + 1])
    def vec_from_coefs(s, coef_by_j):
        """coef_by_j[j] = array length K^m (coeffs of z^k u^j) -> vector over columns (entries beyond
        degree c must be zero, else returns None)"""
        v = np.zeros(s.n, dtype=np.float64)
        for j, cf in enumerate(coef_by_j):
            cf = np.asarray(cf)
            if not cf.any(): continue
            if j >= s.colidx.shape[1] - 1 and s.has_u:
                return None
            if (not s.has_u) and j > 0: return None
            idx = s.colidx[:, j]
            nz = np.nonzero(cf)[0]
            if (idx[nz] < 0).any(): return None
            v[idx[nz]] = cf[nz]
        return v

def gauss_rref(A, p, deg_of_col=None):
    """RREF of A (rows x n, float64 mod p). pivot = first nonzero column. Returns (R, pivots)."""
    A = A.copy(); b, n = A.shape; r = 0; piv = []
    for j in range(n):
        if r >= b: break
        col = A[r:, j]
        nz = np.nonzero(col)[0]
        if len(nz) == 0: continue
        pr = r + nz[0]
        if pr != r: A[[r, pr]] = A[[pr, r]]
        inv = pow(int(A[r, j]), p - 2, p)
        A[r] = mod(A[r] * inv, p)
        f = A[:, j].copy(); f[r] = 0
        oi = np.nonzero(f)[0]
        if len(oi):
            A[oi] = mod(A[oi] - np.outer(f[oi], A[r]), p)
        piv.append(j); r += 1
    return A[:r], piv

def closure(S, gens, p, batch=1500, log=None):
    """returns (E, piv, contains_one). E is RREF (rows x n) of V_{F,c}."""
    n, c = S.n, S.c
    E = np.zeros((0, n)); piv = []
    queue = [np.array(gens, dtype=np.float64).reshape(-1, n).astype(np.uint16)] if len(gens) else []
    one = False
    nv = len(S.vars)
    valid = [t >= 0 for t in S.vars]
    while queue and not one:
        Qall = queue.pop()
        for s0 in range(0, Qall.shape[0], batch):
            Q = Qall[s0:s0 + batch].astype(np.float64)
            if len(piv):
                Q = mod(Q - Q[:, piv] @ E, p)
            keep = Q.any(axis=1)
            if not keep.any(): continue
            NR, npiv = gauss_rref(Q[keep], p)
            if len(npiv) == 0: continue
            if len(piv):
                E = mod(E - E[:, npiv] @ NR, p)
            E = np.vstack([E, NR]); piv = piv + npiv
            if S.one_col in npiv: one = True; break
            lowrows = [i for i, pj in enumerate(npiv) if S.deg[pj] < c]
            if lowrows:
                L = NR[lowrows]
                for v in range(nv):
                    tg = S.vars[v]; ok = tg >= 0
                    assert not L[:, ~ok].any()
                    M = np.zeros((L.shape[0], n)); M[:, tg[ok]] = L[:, ok]
                    queue.append(M.astype(np.uint16))
            if log: log(f"      rows={len(piv)}/{n} queue={sum(q.shape[0] for q in queue)}")
        if one: break
    # order pivots for consistency
    return E, piv, one

def in_span(P, b, p):
    """is b in rowspace of P (mod p)?  returns (bool, rank)"""
    R, pv = gauss_rref(P, p) if P.shape[0] else (np.zeros((0, len(b))), [])
    v = b.copy() % p
    for i, j in enumerate(pv):
        if v[j]: v = mod(v - v[j] * R[i], p)
    return (not v.any()), len(pv)
