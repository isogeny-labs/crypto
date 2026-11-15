#!/usr/bin/env python3
"""Depth of the HKY closure (wave 3, 2026-11-15).

Round-indexed closure at degree c:
    W_0 = span(F cap R_{<=c}),  W_r = W_{r-1} + span{ x*g : g in W_{r-1} cap R_{<c}, x a variable }.
r*(c) = least r with 1 in W_r (None if never);  R(c) = rounds until W stops growing.

Depth lemma (proved in README): every element of W_r is sum a_mu mu*F_i with |mu| <= r, so the
D10 functional lambda (m=3) or lambda_f (m=2) vanishes on W_r unless r >= 2t+2 (m=3) / 2t (m=2).
Since lambda(1) != 0, r*(d_F) >= 2t+2 (m=3), >= 2t (m=2). This script measures r*.
Reuses lab A's Space / gauss_rref / generators (research/kummer_wave2_20261115/KM3-LQR-CLOSURE).
usage: depth.py m2|m3 P T IDX C
"""
import sys, json, time, os
sys.path.insert(0, "/home/user/crypto/research/kummer_wave2_20261115/KM3-LQR-CLOSURE")
from run_cells import *   # Space, gauss_rref, mod, nsd, gens_m3, lam_vec_m2, lam_vec_m3, Chain3

def reduce_rows(Q, E, piv, p):
    if len(piv):
        Q = mod(Q - Q[:, piv] @ E, p)
    return Q

def merge(E, piv, Q, p):
    Q = reduce_rows(Q, E, piv, p)
    Q = Q[Q.any(axis=1)]
    if Q.shape[0] == 0: return E, piv, 0
    NR, npiv = gauss_rref(Q, p)
    if not npiv: return E, piv, 0
    if len(piv): E = mod(E - E[:, npiv] @ NR, p)
    return np.vstack([E, NR]), piv + npiv, len(npiv)

def rounds(S, gens, p, lam=None, maxr=200, batch=2000):
    n, c = S.n, S.c
    E = np.zeros((0, n)); piv = []
    E, piv, _ = merge(E, piv, np.array(gens, dtype=np.float64).reshape(-1, n), p)
    hist = [dict(r=0, dim=len(piv))]
    if S.one_col in piv: return 0, hist
    for r in range(1, maxr + 1):
        low = [i for i, pj in enumerate(piv) if S.deg[pj] < c]
        L = E[low]
        added = 0
        for v, tg in enumerate(S.vars):
            ok = tg >= 0
            assert not L[:, ~ok].any()
            M = np.zeros((L.shape[0], n)); M[:, tg[ok]] = L[:, ok]
            for s0 in range(0, M.shape[0], batch):
                E, piv, a = merge(E, piv, M[s0:s0 + batch], p); added += a
        rec = dict(r=r, dim=len(piv), added=added)
        if lam is not None and S.one_col not in piv:
            rec["lam_nonzero_rows"] = int((mod(E @ lam, p) != 0).sum())
        hist.append(rec)
        if S.one_col in piv: return r, hist
        if added == 0: return None, hist
    return None, hist

def main():
    mode, p, t, idx, c = sys.argv[1], int(sys.argv[2]), int(sys.argv[3]), int(sys.argv[4]), int(sys.argv[5])
    t0 = time.time()
    if mode == "m2":
        C, meta, f = nsd.make_instance(p, t, idx, 2)
        S = Space(C, c, False)
        gens = [S.vec_from_coefs([C.invdft(f).reshape(-1)])]
        lam = lam_vec_m2(C, f, S); bound = 2 * t
    else:
        C, meta, F = nsd.make_instance(p, t, idx, 3)
        S = Space(C, c, True)
        gens = [g for g in gens_m3(C, F, S) if g is not None]
        lam = lam_vec_m3(Chain3(C, F), S); bound = 2 * t + 2
    rstar, hist = rounds(S, gens, p, lam=lam)
    out = dict(mode=mode, p=p, t=t, idx=idx, c=c, r_star=rstar, lemma_bound=bound,
               lemma_ok=(rstar is None or rstar >= bound), lam_one_nonzero=bool(int(lam[S.one_col]) % p),
               rounds=len(hist) - 1, hist=hist, secs=round(time.time() - t0, 1))
    print(json.dumps(out))

if __name__ == "__main__":
    main()
