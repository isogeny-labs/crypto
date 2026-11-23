#!/usr/bin/env python3
"""(4) P5 2-torsion-translate counts and P6 diagonal; (5) roots == x(P1 +- P2) by the validator's own
point arithmetic (long-Weierstrass model, twist via X = n x), and mixed pairs over F_p^2.
Usage: v_p5p6_points.py OUT.json"""
import json, random, sys, time
import numpy as np
sys.path.insert(0, __import__("os").path.dirname(__file__))
from vcore import CURVES, Tables, roots_list, s3_coeffs, lw_add, lw_neg

out = {}
NS = {16: 20000, 20: 20000, 24: 5000}
for k, (p, A) in CURVES.items():
    t0 = time.process_time()
    T = Tables(p, A)
    res = {}
    x = np.arange(p, dtype=np.int64)
    inv = T.inv
    # ---------------- P5: Z = {0} only (A^2-4 non-square, checked in facts); translate by T0 = (0,0)
    assert T.Zx.tolist() == [0]
    # (i) x(P + T0) = 1/x on actual points (sample, both curves) by explicit addition
    rng = random.Random(f"validator-L4-P5-{k}")
    n_non = max(int(v) for v in np.flatnonzero(T.qr == 2)[-50:])  # a LARGE non-residue, not the smallest
    ok_tr = 0
    for _ in range(2000):
        for cl in (1, 2):
            lst = T.Ex if cl == 1 else T.Tx
            xx = int(lst[rng.randrange(len(lst))])
            if cl == 1:
                a2, a4, sc = A, 1, 1
            else:
                a2, a4, sc = A * n_non % p, n_non * n_non % p, n_non
            X = xx * sc % p
            rhs = (X * X % p * X + a2 * X * X + a4 * X) % p
            y = int(T.sqrt[rhs]); assert y * y % p == rhs and y != 0
            R = lw_add((X, y), (0, 0), a2, a4, p)
            xr = R[0] * pow(sc, -1, p) % p
            ok_tr += (xr == pow(xx, -1, p))
    res["P5_translate_by_T0_is_inverse_sampled"] = {"tested": 4000, "match": ok_tr}
    # (ii) class preserved: f(1/x) = f(x)/x^4 for every x != 0
    nz = x[1:]
    lhs = T.f[inv[nz]]
    i2 = inv[nz] * inv[nz] % p
    rhs = T.f[nz] * (i2 * i2 % p) % p
    res["f(1/x)==f(x)/x^4_all_x"] = bool(np.all(lhs == rhs))
    # (iii) S3(x1, x2, 0) = (x1 x2 - 1)^2: a root at X = 0 iff x1 x2 = 1 (algebra), so the pairs with a
    # root in Z are exactly {x, 1/x}; count them, x != 1/x, by class
    cnt = {}
    for nm, cl in (("EE", 1), ("TT", 2)):
        xs = x[T.cls == cl]
        cnt[nm] = int((inv[xs] > xs).sum())
    cnt["ET"] = int(((T.cls[inv[nz]] != T.cls[nz]) & (T.cls[nz] != 0)).sum()) // 2
    cnt["diag_x_eq_1_over_x"] = int(((inv[nz] == nz)).sum())
    res["P5_pairs_x_1overx"] = cnt
    # ---------------- P6: diagonal x1 = x2 = x
    # S3 root (linear) vs x(2P) by the Weierstrass doubling formula, every x
    a, B2, c = s3_coeffs(x, x, A, p)
    assert bool(np.all(a == 0))
    nonz = T.cls != 0
    lin_root = np.where(B2 != 0, c * T.inv[2 * B2 % p] % p, -1)
    lam_num = (3 * x * x + 2 * A * x + 1) % p
    dbl = (lam_num * lam_num % p * T.inv[4 * T.f % p] % p - A - 2 * x) % p   # x(2P) = lambda^2 - A - 2x, lambda^2 = num^2/(4 f)
    res["P6_lin_root_eq_x2P_all_nonZ_x"] = bool(np.all(lin_root[nonz] == dbl[nonz]))
    res["P6_x0_diag"] = {"B2": int(B2[0]), "c": int(c[0]), "finite_roots": 0 if (B2[0] == 0 and c[0] != 0) else "?",
                         "reason": "2 T0 = O: the polynomial degenerates to the constant c = 1"}
    p6 = {}
    for nm, cl in (("E", 1), ("T", 2)):
        m = T.cls == cl
        rc = T.cls[dbl[m]]
        p6[nm] = {"x": int(m.sum()), "x2P_on_own_curve": int((rc == cl).sum()), "x2P_in_Z": int((rc == 0).sum()),
                  "x2P_on_other_curve": int((rc == 3 - cl).sum())}
    res["P6_diag_classes"] = p6
    # ---------------- (5) roots == x(P1 +- P2) by explicit points
    pts = {}
    for nm, cl in (("EE", 1), ("TT", 2)):
        lst = T.Ex if cl == 1 else T.Tx
        sc = 1 if cl == 1 else n_non
        a2, a4 = (A, 1) if cl == 1 else (A * n_non % p, n_non * n_non % p)
        isc = pow(sc, -1, p)
        tested = match = s3zero = 0
        for _ in range(NS[k]):
            u, v = int(lst[rng.randrange(len(lst))]), int(lst[rng.randrange(len(lst))])
            if u == v:
                continue
            P_ = []
            for xx in (u, v):
                X = xx * sc % p
                rhs = (X * X % p * X + a2 * X * X + a4 * X) % p
                y = int(T.sqrt[rhs]); assert y * y % p == rhs
                P_.append((X, y))
            S = lw_add(P_[0], P_[1], a2, a4, p)
            D = lw_add(P_[0], lw_neg(P_[1], p), a2, a4, p)
            want = sorted({S[0] * isc % p, D[0] * isc % p})
            got = roots_list(T, u, v)
            aa, bb, cc = (int(t) for t in s3_coeffs(u, v, A, p))
            s3zero += all((aa * w * w - 2 * bb * w + cc) % p == 0 for w in want)
            tested += 1
            match += (want == got)
        pts[nm] = {"tested": tested, "roots_eq_x(P1+-P2)": match, "S3_vanishes_at_x(P1+-P2)": s3zero}
    # mixed pairs over F_p^2 = F_p[s]/(s^2 - n): P1 in E(F_p), P2 = (x2, w s) with w^2 = f(x2)/n
    def m2(u, v):
        return ((u[0] * v[0] + n_non * u[1] * v[1]) % p, (u[0] * v[1] + u[1] * v[0]) % p)
    tested = conj = notfp = s3z = nofp = 0
    NM = 2000
    xs_e = [int(T.Ex[rng.randrange(len(T.Ex))]) for _ in range(NM)]
    xs_t = [int(T.Tx[rng.randrange(len(T.Tx))]) for _ in range(NM)]
    for u, v in zip(xs_e, xs_t):
        y1 = int(T.sqrt[T.f[u]])
        w = int(T.sqrt[T.f[v] * pow(n_non, -1, p) % p]); assert w * w % p * n_non % p == T.f[v]
        dx = pow(v - u, -1, p)
        xr = []
        for sg in (1, -1):
            lam = ((-y1) * dx % p, sg * w * dx % p)            # (sg w s - y1)/(x2 - x1)
            l2 = m2(lam, lam)
            xr.append(((l2[0] - A - u - v) % p, l2[1]))
        aa, bb, cc = (int(t) for t in s3_coeffs(u, v, A, p))
        def S3(X):
            X2 = m2(X, X)
            return ((aa * X2[0] - 2 * bb * X[0] + cc) % p, (aa * X2[1] - 2 * bb * X[1]) % p)
        tested += 1
        s3z += (S3(xr[0]) == (0, 0) and S3(xr[1]) == (0, 0))
        conj += (xr[0][0] == xr[1][0] and (xr[0][1] + xr[1][1]) % p == 0)
        notfp += (xr[0][1] != 0)
        nofp += (roots_list(T, u, v) == [])
    pts["ET_over_Fp2"] = {"tested": tested, "both_roots_of_S3": s3z, "conjugate": conj, "not_in_Fp": notfp,
                          "no_Fp_root_by_validator_formula": nofp}
    # brute force (no discriminant) at k = 16: evaluate S3 at every X in F_p for 300 mixed pairs
    if k == 16:
        X = x
        bf = 0
        for u, v in list(zip(xs_e, xs_t))[:300]:
            aa, bb, cc = (int(t) for t in s3_coeffs(u, v, A, p))
            bf += int(((aa * (X * X % p) - 2 * bb * X + cc) % p == 0).sum())
        pts["ET_bruteforce_all_X_300_pairs_roots_found"] = bf
    res["points"] = pts
    res["twist_nonresidue_used"] = n_non
    res["cpu_s"] = round(time.process_time() - t0, 1)
    out[k] = res
    print(k, json.dumps(res))
    del T
json.dump(out, open(sys.argv[1], "w"), indent=1)
