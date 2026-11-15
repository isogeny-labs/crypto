#!/usr/bin/env python3
"""KM3-LQR-CLOSURE driver. usage: run_cells.py m3 P T IDX | m2 P T IDX | plant P T | outdir via LQR_OUT"""
import sys, os, json, time, numpy as np
from hky_closure import *
OUT = os.environ.get("LQR_OUT", ".")

def fwd(C, c):
    """c flat (K^m,) coefficients -> sum_k om^{a.k} c[k] at grid a (flat)"""
    p, K, m = C.p, C.K, C.m
    x = (c.reshape((K,) * m) % p).astype(np.int64)
    for ax in range(m):
        x = np.moveaxis(np.tensordot(x, C.W, axes=([ax], [1])), -1, ax) % p
    return x.reshape(-1)

def inv_mod(a, p): return pow(int(a) % p, p - 2, p)

class Chain3:
    """pointwise quotient data of F1, F2 on the grid"""
    def __init__(s, C, F):
        p, N = C.p, C.N; s.C, s.F = C, F
        c0, c1, c2 = [F[0][v] for v in range(3)]
        d0, d1, d2 = [F[1][v] for v in range(3)]
        e1 = np.zeros(N, dtype=np.int64); e0 = np.zeros(N, dtype=np.int64)
        al = np.zeros(N, dtype=np.int64); be = np.zeros(N, dtype=np.int64)
        sx = np.zeros(N, dtype=np.int64); rx = np.zeros(N, dtype=np.int64)
        for n in range(N):
            di = inv_mod(d2[n], p)
            e1[n] = (-d1[n] * di) % p; e0[n] = (-d0[n] * di) % p
            al[n] = (c0[n] + c2[n] * e0[n]) % p; be[n] = (c1[n] + c2[n] * e1[n]) % p
            D = (al[n] * (al[n] + be[n] * e1[n]) - be[n] ** 2 * e0[n]) % p
            assert D != 0, "f1 not a unit mod f2"
            Di = inv_mod(D, p)
            sx[n] = (al[n] + be[n] * e1[n]) * Di % p; rx[n] = (-be[n]) * Di % p
        s.e1, s.e0, s.s, s.r = e1, e0, sx, rx
        s.a = [np.ones(N, dtype=np.int64)]; s.b = [np.zeros(N, dtype=np.int64)]
    def ab(s, j):
        p = s.C.p
        while len(s.a) <= j:
            a, b = s.a[-1], s.b[-1]
            s.a.append(b * s.e0 % p); s.b.append((a + b * s.e1) % p)
        return s.a[j], s.b[j]
    def val(s, j):
        p = s.C.p; a, b = s.ab(j)
        return (a * s.r + b * s.s + b * s.r % p * s.e1) % p

def lam_vec_m3(ch, S):
    C, p = ch.C, ch.C.p
    w = (C.Z[0] * C.Z[1] % p) * C.Z[2] % p
    out = np.zeros(S.n)
    cache = {}
    for j in range(S.c + 1):
        sel = np.nonzero(S.j == j)[0]
        if len(sel) == 0: continue
        L = fwd(C, w * ch.val(j) % p)                         # Lam_j[k flat]
        out[sel] = L[S.flat[sel]]
    return out

def pi_matrix_apply(ch, S, E):
    """pi(rows of E) in F_p^{2N}: [a-part | b-part]"""
    C, p, N = ch.C, ch.C.p, ch.C.N
    P = np.zeros((E.shape[0], 2 * N))
    chi_cache = None
    K, m = C.K, C.m
    for j in range(S.c + 1):
        sel = np.nonzero(S.j == j)[0]
        if len(sel) == 0: continue
        # Chi[kflat, xflat] = om^{a.k}
        ksel = S.ks[:, sel]
        Chi = np.ones((len(sel), N), dtype=np.int64)
        for i in range(m):
            Chi = Chi * C.pw[np.outer(ksel[i], C.A[i]) % K] % p
        Y = mod(E[:, sel] @ Chi.astype(np.float64), p)        # r x N: value of the u^j-coefficient
        a, b = ch.ab(j)
        P[:, :N] = mod(P[:, :N] + Y * a[None, :], p)
        P[:, N:] = mod(P[:, N:] + Y * b[None, :], p)
    return P

def gens_m3(C, F, S):
    """vectors of F1, F2 over columns of S (None if degree > c)"""
    out = []
    for i in range(2):
        coefs = [C.invdft(F[i][v]).reshape(-1) for v in range(3)]
        out.append(S.vec_from_coefs(coefs))
    return out

def run_m3(p, t, idx, cmax=9, log=print, lam_other=None, plant=False):
    t0 = time.time()
    if plant: C, meta, F = nsd.planted_instance(p, t)
    else: C, meta, F = nsd.make_instance(p, t, idx, 3)
    ch = Chain3(C, F) if not plant else None
    N = C.N
    chO = None
    if lam_other is not None:
        C2, meta2, F2 = nsd.make_instance(p, t, lam_other, 3); chO = Chain3(C2, F2)
    rec = dict(m=3, p=p, t=t, idx=idx, meta={k: v for k, v in meta.items()}, cells=[])
    dF = None; dFq = None
    for c in range(2, cmax + 1):
        S = Space(C, c, True)
        gens = [g for g in gens_m3(C, F, S) if g is not None]
        tc = time.time()
        E, piv, one = closure(S, gens, p) if gens else (np.zeros((0, S.n)), [], False)
        cell = dict(c=c, ncols=S.n, dim=len(piv) if not one else S.n, contains_one=bool(one), closure_s=round(time.time() - tc, 2))
        if ch is not None and not one:
            lam = lam_vec_m3(ch, S)
            lv = mod(E @ lam, p) if len(piv) else np.zeros(0)
            cell["lam_nonzero_rows"] = int((lv != 0).sum()); cell["lam_kills"] = bool(not (lv != 0).any())
            cell["lam_one"] = int(lam[S.one_col])
            if chO is not None:
                lamO = lam_vec_m3(chO, S)
                lvO = mod(E @ lamO, p) if len(piv) else np.zeros(0)
                cell["lamOther_nonzero_rows"] = int((lvO != 0).sum())
            P = pi_matrix_apply(ch, S, E) if len(piv) else np.zeros((0, 2 * N))
            b = np.zeros(2 * N); b[:N] = 1
            ins, rk = in_span(P, b, p)
            cell["pi_rank"] = rk; cell["pi_one_in_span"] = bool(ins); cell["ann_dim"] = 2 * N - rk
            if ins and dFq is None: dFq = c
        elif ch is not None and one:
            S1 = S; lam = lam_vec_m3(ch, S1)
            cell["lam_one"] = int(lam[S.one_col]); cell["pi_one_in_span"] = True
            if dFq is None: dFq = c
        if one and dF is None: dF = c
        rec["cells"].append(cell)
        log(json.dumps(cell))
        if one: break
        if plant and c >= cmax: break
    rec["d_F"] = dF; rec["dF_f2"] = dFq
    if ch is not None:
        ks = [x["c"] for x in rec["cells"] if x.get("lam_kills") and not x["contains_one"]]
        rec["c_star"] = max(ks) if ks else None
    rec["secs"] = round(time.time() - t0, 1)
    return rec

# ---------------- m = 2 control ----------------
def lam_vec_m2(C, f, S):
    p = C.p
    w = (C.Z[0] * C.Z[1]) % p
    inv = np.array([inv_mod(v, p) for v in f]); L = fwd(C, w * inv % p)
    return L[S.flat].astype(np.float64)

def run_m2(p, t, idx, cmax=7, log=print, lam_other=None):
    t0 = time.time()
    C, meta, f = nsd.make_instance(p, t, idx, 2)
    fO = nsd.make_instance(p, t, lam_other, 2)[2] if lam_other is not None else None
    rec = dict(m=2, p=p, t=t, idx=idx, meta=meta, cells=[])
    coef = C.invdft(f).reshape(-1)
    dF = None
    for c in range(2, cmax + 1):
        S = Space(C, c, False)
        g = S.vec_from_coefs([coef])
        gens = [g] if g is not None else []
        tc = time.time()
        E, piv, one = closure(S, gens, p) if gens else (np.zeros((0, S.n)), [], False)
        cell = dict(c=c, ncols=S.n, dim=len(piv) if not one else S.n, contains_one=bool(one), closure_s=round(time.time() - tc, 2))
        lam = lam_vec_m2(C, f, S); cell["lam_one"] = int(lam[S.one_col])
        if not one:
            lv = mod(E @ lam, p) if len(piv) else np.zeros(0)
            cell["lam_nonzero_rows"] = int((lv != 0).sum()); cell["lam_kills"] = bool(not (lv != 0).any())
            if fO is not None:
                lvO = mod(E @ lam_vec_m2(C, fO, S), p) if len(piv) else np.zeros(0)
                cell["lamOther_nonzero_rows"] = int((lvO != 0).sum())
        rec["cells"].append(cell); log(json.dumps(cell))
        if one: dF = c; break
    rec["d_F"] = dF
    ks = [x["c"] for x in rec["cells"] if x.get("lam_kills")]
    rec["c_star"] = max(ks) if ks else None
    rec["secs"] = round(time.time() - t0, 1)
    return rec

if __name__ == "__main__":
    mode = sys.argv[1]; p = int(sys.argv[2]); t = int(sys.argv[3])
    os.makedirs(OUT, exist_ok=True)
    if mode == "m3":
        idx = int(sys.argv[4]); other = (idx + 1) % 3
        rec = run_m3(p, t, idx, lam_other=other)
        name = f"m3_p{p}_t{t}_i{idx}.json"
    elif mode == "m2":
        idx = int(sys.argv[4]); other = (idx + 1) % 3
        rec = run_m2(p, t, idx, cmax=int(os.environ.get("CMAX", "7")), lam_other=other)
        name = f"m2_p{p}_t{t}_i{idx}.json"
    elif mode == "plant":
        rec = run_m3(p, t, 0, cmax=int(os.environ.get("CMAX", "7")), plant=True); name = f"plant_p{p}_t{t}.json"
    json.dump(rec, open(os.path.join(OUT, name), "w"), indent=1)
    print("DONE", name, "d_F", rec.get("d_F"), "dF_f2", rec.get("dF_f2"), "c*", rec.get("c_star"), "secs", rec["secs"])
