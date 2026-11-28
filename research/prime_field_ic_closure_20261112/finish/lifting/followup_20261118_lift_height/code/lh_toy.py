#!/usr/bin/env python3
"""lift-height round (2026-11-18): toy xedni lifts with Neron-Tate Gram matrices, r = 2..9.  [pcc:unverified]

Two constructions:
  --origin O        : 9-monomial cubic (Y^3 coefficient 0), O = (0:1:0) on the cubic, r <= 8
                      (exactly xedni_cubic.py of the 20261115 round; heights module patched so the
                      lattice minimum is always enumerated, never replaced by an eigenvalue).
  --origin tangent  : 10-monomial cubic through the r lifts (r <= 9); origin O' = third intersection
                      of the tangent line at lift 1 (rational).  O' is moved to (0:1:0) by a unimodular
                      change of coordinates, then the 20261115 Washington map gives a Weierstrass model.
                      red(O') = -2 P_1 in E(F_p) (checked), so the group points are P_i - Q0 with
                      masks c_i + 2 c_1 (uniform and independent when q != 3).
Arms: X (honest), DET (P'_3 := P'_1 + P'_2 over Q), PS (planted mod p relation, translated group:
P_3 - Q0 = (P_1 - Q0) + (P_2 - Q0)).
"""
import sys, os, json, math, random, time, argparse
from fractions import Fraction
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
os.environ.setdefault("LIFT_CODE", HERE)
from flint import fmpz_mat
import xedni_toy as X0
import xedni_cubic as XC
import lh_heights

EXP9 = [(1, 2, 0), (0, 2, 1), (2, 1, 0), (1, 1, 1), (0, 1, 2), (3, 0, 0), (2, 0, 1), (1, 0, 2), (0, 0, 3)]
EXP10 = EXP9 + [(0, 3, 0)]


def mono10(X, Y, Z):
    return [X ** a * Y ** b * Z ** c for (a, b, c) in EXP10]


def fit_cubic10(lifts, p, A, B):
    rows = [mono10(*L) for L in lifts]
    ker = X0.int_kernel(rows, 10)
    uE = [0, 1, 0, 0, 0, -1, 0, (-A) % p, (-B) % p, 0]
    w = X0.solve_mod(ker, uE, p)
    if w is None:
        return None, "no-coset"
    m = len(ker)
    t = next(j for j in range(m) if w[j] % p)
    inv = pow(w[t], -1, p)
    wt = [(wj * inv) % p for wj in w]
    Wb = []
    for j in range(m):
        if j == t:
            Wb.append(wt)
        else:
            e = [0] * m
            e[j] = p
            Wb.append(e)
    Lb = [[sum(xi * ker[j][c] for j, xi in enumerate(xv)) for c in range(10)] for xv in Wb]
    red = [[int(v) for v in row] for row in fmpz_mat(Lb).lll().tolist()]
    red.sort(key=lambda u: sum(v * v for v in u))
    for u in red:
        if u[1] % p == 0:
            continue
        c = u[1] % p
        if all((ui - c * ei) % p == 0 for ui, ei in zip(u, uE)):
            for row in rows:
                assert sum(a * b for a, b in zip(row, u)) == 0
            return u, "ok"
    return None, "no-congruent"


def F_eval(u, P):
    X, Y, Z = P
    return sum(c * X ** a * Y ** b * Z ** cc for c, (a, b, cc) in zip(u, EXP10))


def grad(u, P):
    X, Y, Z = P
    gx = gy = gz = 0
    for c, (a, b, cc) in zip(u, EXP10):
        if a:
            gx += c * a * X ** (a - 1) * Y ** b * Z ** cc
        if b:
            gy += c * b * X ** a * Y ** (b - 1) * Z ** cc
        if cc:
            gz += c * cc * X ** a * Y ** b * Z ** (cc - 1)
    return (gx, gy, gz)


def cross(a, b):
    return (a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0])


def prim(v):
    g = 0
    for x in v:
        g = math.gcd(g, x)
    v = tuple(x // g for x in v)
    return v


def unimod_to_e1(v):
    """integer unimodular M (3x3) with M v = e_1 (v primitive)."""
    M = [[1, 0, 0], [0, 1, 0], [0, 0, 1]]
    w = list(v)
    while w != [1, 0, 0]:   # fixed 2026-11-18 ~1h into the run: old test exited at w = [-1,0,0] (then the assert skipped the trial)
        nz = [i for i in range(3) if w[i]]
        if len(nz) == 1:
            i = nz[0]
            if i != 0:
                M[0], M[i] = M[i], M[0]
                w[0], w[i] = w[i], w[0]
            if w[0] == -1:
                M[0] = [-x for x in M[0]]
                w[0] = 1
            continue
        nz.sort(key=lambda i: abs(w[i]))
        i = nz[0]
        for j in nz[1:]:
            qq = w[j] // w[i]
            w[j] -= qq * w[i]
            M[j] = [a - qq * b for a, b in zip(M[j], M[i])]
    assert [sum(M[i][k] * v[k] for k in range(3)) for i in range(3)] == [1, 0, 0]
    return M


def matinv_int(M):
    a = [[Fraction(x) for x in row] for row in M]
    det = (a[0][0] * (a[1][1] * a[2][2] - a[1][2] * a[2][1]) - a[0][1] * (a[1][0] * a[2][2] - a[1][2] * a[2][0])
           + a[0][2] * (a[1][0] * a[2][1] - a[1][1] * a[2][0]))
    assert abs(det) == 1
    C = [[None] * 3 for _ in range(3)]
    for i in range(3):
        for j in range(3):
            m = [[a[r][c] for c in range(3) if c != j] for r in range(3) if r != i]
            C[i][j] = (-1) ** (i + j) * (m[0][0] * m[1][1] - m[0][1] * m[1][0])
    inv = [[int(C[j][i] / det) for j in range(3)] for i in range(3)]
    for i in range(3):
        for j in range(3):
            assert sum(M[i][k] * inv[k][j] for k in range(3)) == (1 if i == j else 0)
    return inv


def poly_compose(u, Ainv):
    """coefficients (EXP10 order) of F'(X') = F(Ainv X')."""
    # linear forms: X = sum Ainv[0][j] X'_j etc.  polys as dict exp->coef
    lin = [{(1, 0, 0): Ainv[i][0], (0, 1, 0): Ainv[i][1], (0, 0, 1): Ainv[i][2]} for i in range(3)]

    def mul(p1, p2):
        out = {}
        for e1, c1 in p1.items():
            for e2, c2 in p2.items():
                e = (e1[0] + e2[0], e1[1] + e2[1], e1[2] + e2[2])
                out[e] = out.get(e, 0) + c1 * c2
        return out

    pw = [[{(0, 0, 0): 1}] for _ in range(3)]
    for i in range(3):
        for k in range(3):
            pw[i].append(mul(pw[i][-1], lin[i]))
    tot = {}
    for c, (a, b, cc) in zip(u, EXP10):
        if not c:
            continue
        t = mul(mul(pw[0][a], pw[1][b]), pw[2][cc])
        for e, v in t.items():
            tot[e] = tot.get(e, 0) + c * v
    return [tot.get(e, 0) for e in EXP10]


def tangent_model(lifts, p, A, B, q, Ppts, cs):
    u, why = fit_cubic10(lifts, p, A, B)
    if u is None:
        return None, why
    L1 = lifts[0]
    g = grad(u, L1)
    if not any(g):
        return None, "singular-at-L1"
    w = cross(g, L1)
    if not any(w):
        return None, "w=0"
    f1 = F_eval(u, tuple(a + b for a, b in zip(L1, w)))
    fm = F_eval(u, tuple(a - b for a, b in zip(L1, w)))
    c2, c3 = Fraction(f1 + fm, 2), Fraction(f1 - fm, 2)
    if c2 == 0:
        return None, "flex-at-L1"
    Oq = tuple(c3 * a - c2 * b for a, b in zip(L1, w))
    den = 1
    for x in Oq:
        den = math.lcm(den, x.denominator)
    Op = prim(tuple(int(x * den) for x in Oq))
    assert F_eval(u, Op) == 0
    # check red(O') = -2 P_1 in E(F_p)
    Q0 = X0.ec_mul(q - 2, Ppts[0], A, p)
    if Op[2] % p == 0:
        okred = (Q0 is None)
    else:
        zi = pow(Op[2], -1, p)
        okred = (Q0 is not None and (Op[0] * zi % p, Op[1] * zi % p) == tuple(Q0))
    if not okred:
        return None, "red-check-FAIL"
    M = unimod_to_e1(Op)
    U = [M[1], M[0], M[2]]                     # U O' = e_2
    # shear Z' -> Z' + a X' (fixes e_2) so that no lift lands on Z' = 0 (fixed 2026-11-18, ~1h in)
    for a_ in range(0, 50):
        Ua = [U[0], U[1], [z + a_ * x for z, x in zip(U[2], U[0])]]
        if all(sum(Ua[2][k] * L[k] for k in range(3)) != 0 for L in lifts):
            U = Ua
            break
    Uinv = matinv_int(U)
    up = poly_compose(u, Uinv)
    assert up[9] == 0, "Y^3 coefficient must vanish (O' on cubic)"
    nl = [tuple(sum(U[i][k] * L[k] for k in range(3)) for i in range(3)) for L in lifts]
    for L in nl:
        assert F_eval(up, L) == 0
        if L[2] == 0:
            return None, "lift-at-Z'=0"
    cw, why = XC.cubic_to_weierstrass(up[:9], nl)
    if cw is None:
        return None, why
    cs2 = [(c + 2 * cs[0]) % q for c in cs]
    if any(c == 0 for c in cs2):
        return None, "point-reduces-to-origin"
    return (cw, cs2, max(abs(v).bit_length() for v in up)), "ok"


def circuit_lattice(G, n):
    """lambda_1 of the lattice spanned by the points in supp(n), given the (numerically singular)
    Gram matrix G of all points and an exact relation n (sum n_i P'_i torsion)."""
    import numpy as np
    J = [i for i, x in enumerate(n) if x]
    nn = [n[i] for i in J]
    g = 0
    for x in nn:
        g = math.gcd(g, x)
    nn = [x // g for x in nn]
    k1 = len(J)
    # unimodular completion: M nn = e_1 by integer row ops (generalised Euclid)
    Mx = [[1 if i == j else 0 for j in range(k1)] for i in range(k1)]
    w = list(nn)
    while True:
        nz = [i for i in range(k1) if w[i]]
        if len(nz) == 1:
            i = nz[0]
            if i != 0:
                Mx[0], Mx[i] = Mx[i], Mx[0]
                w[0], w[i] = w[i], w[0]
            if w[0] < 0:
                Mx[0] = [-x for x in Mx[0]]
                w[0] = -w[0]
            break
        nz.sort(key=lambda i: abs(w[i]))
        i = nz[0]
        for j in nz[1:]:
            qq = w[j] // w[i]
            w[j] -= qq * w[i]
            Mx[j] = [a - qq * b for a, b in zip(Mx[j], Mx[i])]
    assert w[0] == 1
    Minv = fmpz_mat(Mx).inv()          # rational; integral since unimodular
    Bc = [[int(Minv[i, j]) for j in range(k1)] for i in range(k1)]   # columns: basis, col 0 = nn
    assert [Bc[i][0] for i in range(k1)] == nn
    GJ = np.array([[G[a][b] for b in J] for a in J])
    Bm = np.array([[Bc[i][j] for j in range(1, k1)] for i in range(k1)], dtype=float)
    GL = Bm.T @ GJ @ Bm
    if GL.shape[0] == 0:
        return dict(J=J, k=0, lambda_L=None)
    lam, vec = lh_heights.min_form(GL)
    return dict(J=J, k=k1 - 1, lambda_L=float(lam), resid=float(np.array(nn) @ GJ @ np.array(nn)),
                HJ=float(max(G[a][a] for a in J)))


HMODEL = "ml"


def run_trial(tg, r, arm, seed, n_dbl, origin):
    rng = random.Random(seed)
    p, A, B, q, S = tg["p"], tg["A"], tg["B"], tg["q"], tg["S"]
    k = rng.randrange(1, q)
    T = X0.ec_mul(k, S, A, p)
    s = [rng.randrange(q) for _ in range(r)]
    t = [rng.randrange(q) for _ in range(r)]
    if arm == "PS":
        if origin == "O":
            s[2], t[2] = (s[0] + s[1]) % q, (t[0] + t[1]) % q
        else:   # c_3 = 3 c_1 + c_2  <=>  (P_3 - Q0) = (P_1 - Q0) + (P_2 - Q0), Q0 = -2 P_1
            s[2], t[2] = (3 * s[0] + s[1]) % q, (3 * t[0] + t[1]) % q
    cs = [(si - k * ti) % q for si, ti in zip(s, t)]
    rec = dict(p=p, q=q, r=r, arm=arm, seed=seed, bits=p.bit_length(), origin=origin)
    if any(c == 0 for c in cs):
        rec["status"] = "point-at-infinity"
        return rec
    Ppts = [X0.ec_mul(c, S, A, p) for c in cs]
    if len(set(Ppts)) < r:
        rec["status"] = "repeated-point"
        return rec
    lifts = [XC.proj_lift(P[0], P[1], p) for P in Ppts]
    rec["lift_maxbits"] = max(abs(v).bit_length() for L in lifts for v in L)
    if origin == "O":
        try:
            u, why = XC.fit_cubic(lifts, p, A, B)
        except AssertionError:
            rec["status"] = "kernel-rank"
            return rec
        if u is None:
            rec["status"] = why
            return rec
        rec["u_maxbits"] = max(abs(v).bit_length() for v in u)
        cw, why = XC.cubic_to_weierstrass(u, lifts)
        if cw is None:
            rec["status"] = why
            return rec
    else:
        try:
            res, why = tangent_model(lifts, p, A, B, q, Ppts, cs)
        except AssertionError as ex:
            rec["status"] = "assert:" + str(ex)[:40]
            return rec
        if res is None:
            rec["status"] = why
            return rec
        cw, cs, rec["u_maxbits"] = res
    sh = XC.to_short_rational(*cw)
    # unscaled short model with rational points (for heights; lift-height round)
    _b2, _b4, _b6, _b8, _c4, _c6, _D = X0.weier_invariants(*cw[0])
    W_ = cw[0]
    sh0 = dict(A=-27 * _c4, B=-54 * _c6,
               pts=[(36 * x + 3 * _b2, 108 * (2 * y + W_[0] * x + W_[2])) for (x, y) in cw[1]])
    for (x, y) in sh0["pts"]:
        assert y * y == x ** 3 + sh0["A"] * x + sh0["B"]
    if sh is None:
        rec["status"] = "singular"
        return rec
    rec["status"] = "ok"
    rec["log2_absDW"] = math.log2(abs(sh["D"]))
    rec["max_xs_bits"] = max(abs(x).bit_length() for x, _ in sh["spts"])
    if arm == "DET":
        (x1, y1), (x2, y2) = [tuple(map(Fraction, P)) for P in sh["spts"][:2]]
        lam_ = (y2 - y1) / (x2 - x1)
        x3 = lam_ * lam_ - x1 - x2
        y3 = lam_ * (x1 - x3) - y1
        V = math.lcm(x3.denominator, y3.denominator)
        sp = [(x * V * V, y * V ** 3) for (x, y) in sh["spts"][:2]] + [(int(x3 * V * V), int(y3 * V ** 3))] + \
             [(x * V * V, y * V ** 3) for (x, y) in sh["spts"][3:]]
        sh = dict(sh, spts=sp, As=sh["As"] * V ** 4, Bs=sh["Bs"] * V ** 6, D=sh["D"] * V ** 12, c4=sh["c4"] * V ** 4)
        for (x, y) in sp:
            assert y * y == x ** 3 + sh["As"] * x + sh["Bs"]
        cs = cs[:2] + [(cs[0] + cs[1]) % q] + cs[3:]
        (u1, v1), (u2, v2) = sh0["pts"][:2]
        lm = (v2 - v1) / (u2 - u1)
        u3 = lm * lm - u1 - u2
        sh0["pts"] = sh0["pts"][:2] + [(u3, lm * (u1 - u3) - v1)] + sh0["pts"][3:]
    sv = X0.relation_sieve(sh, cs, q, random.Random(seed ^ 0x5EED))
    rec.update({k_: v for k_, v in sv.items() if k_ not in ("cand_raw",)})
    n = sv["cand"]
    rec["rel"] = n
    if n is not None:
        rec["rel_inf"] = max(abs(x) for x in n)
        assert sum(ni * ci for ni, ci in zip(n, cs)) % q == 0
    if arm == "PS":
        rec["planted_realized"] = bool(X0.exact_relation_holds([1, 1, -1] + [0] * (r - 3), sh["spts"], sh["As"]))
    if n_dbl:
        if HMODEL == "ml":
            ml = lh_heights.height_data_ml(sh0["A"], sh0["B"], sh0["pts"], levels=tuple(range(n_dbl - 2, n_dbl + 1)))
            rec["ml"] = ml
            best = ml["rich"]
            hd = dict(eps=ml[str(n_dbl)]["eps"], H=best["H"], hmin=None, lambda_G=best["lambda_G"],
                      rho_G=best["rho_G"], log_absDW=ml["log_absDs"], lang_lambdaG=best["lambda_G"] / ml["log_absDs"],
                      min_eig=None, lambda_vec=best["lambda_vec"], G=best["G"])
        elif HMODEL == "unscaled":
            hd = lh_heights.height_data_q(sh0["A"], sh0["B"], sh0["pts"], n_dbl=n_dbl)
        else:
            hd = lh_heights.height_data(sh, n_dbl=n_dbl)
        rec["hmodel"] = HMODEL
        rec["heights"] = {k_: hd.get(k_) for k_ in ("eps", "H", "hmin", "lambda_G", "rho_G", "log_absDW",
                                                 "lang_lambdaG", "min_eig", "lambda_vec", "G")}
        if n is not None and rec.get("verified") == "exact":
            try:
                rec["circuit"] = circuit_lattice(hd["G"], n)
                cl = rec["circuit"]
                if cl.get("lambda_L"):
                    cl["rho_L"] = hd["H"] / cl["lambda_L"]
                    cl["rho_LJ"] = cl["HJ"] / cl["lambda_L"]
            except Exception as ex:
                rec["circuit"] = dict(error=str(ex)[:80])
    return rec


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--bits", type=int, required=True)
    ap.add_argument("--r", type=int, required=True)
    ap.add_argument("--arm", default="X")
    ap.add_argument("--origin", default="O", choices=["O", "tangent"])
    ap.add_argument("--trials", type=int, default=15)
    ap.add_argument("--curves", type=int, default=2)
    ap.add_argument("--seed", type=int, default=20261118)
    ap.add_argument("--heights", type=int, default=5)
    ap.add_argument("--out", required=True)
    ap.add_argument("--hmodel", default="ml", choices=["ml", "unscaled", "scaled"])
    a = ap.parse_args()
    global HMODEL
    HMODEL = a.hmodel
    rng = random.Random(a.seed * 1000003 + a.bits * 101 + a.r * 7 + (1 if a.origin == "tangent" else 0)
                        + 13 * ["X", "DET", "PS"].index(a.arm))
    with open(a.out, "a") as fo:
        for ci in range(a.curves):
            tg = X0.make_target(a.bits, rng)
            for j in range(a.trials):
                sd = rng.randrange(1 << 62)
                t0 = time.time()
                rec = run_trial(tg, a.r, a.arm, sd, a.heights, a.origin)
                rec["sec"] = round(time.time() - t0, 3)
                rec["curve"] = ci
                fo.write(json.dumps(rec, default=str) + "\n")
                fo.flush()


if __name__ == "__main__":
    main()
