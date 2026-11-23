#!/usr/bin/env python3
"""Toy prime-field xedni, general-cubic path (O = (0:1:0) on the cubic, not a flex), r = 2..8.
[pcc:unverified]

Extends the R5 toy (7-coefficient flex-at-O ansatz, r <= 6) to r = 7, 8.
Per trial:
  1. target E/F_p : Y^2 Z = X^3 + A X Z^2 + B Z^3, #E = q prime (make_target of the R5 toy).
  2. P_i = s_i S - t_i T, fresh uniform s, t (honest arm).  Arm PS plants P_3 = P_1 + P_2.
  3. projective lifts (X_i:Y_i:Z_i): shortest vector (LLL) of {v in Z^3 : v == c (x_i, y_i, 1) mod p}
     (size ~ p^(2/3)).
  4. cubic F = sum u_m m over the 9 monomials with zero Y^3 coefficient (so O = (0:1:0) is on it),
     through the r lifts, with u == c * u_E (mod p); shortest such u by LLL.
  5. birational map to Weierstrass with O -> infinity: projection from O gives
     (2 L1 y + Q2)^2 = Q2^2 - 4 L1 K3 (quartic in t = X/Z), shift to the tangent point t0 of O,
     Washington Thm 2.17 quartic -> Weierstrass (point (0, q0) -> infinity), then integral short model.
     Every image point is checked on the curve exactly.
  6. relation sieve (R5 toy) + Neron-Tate Gram matrix by the doubling limit (R5 heights.py),
     lambda_G = min of the Gram form on Z^r (Fincke-Pohst), rho_G = H / lambda_G.
"""
import sys, os, json, math, random, time, argparse
from fractions import Fraction
sys.path.insert(0, os.environ.get("LIFT_CODE", "/home/user/crypto/research/prime_field_ic_closure_20261112/finish/lifting/code"))
from flint import fmpz_mat
import xedni_toy as X0
from heights import height_data

MONO = ["Y2X", "Y2Z", "YX2", "YXZ", "YZ2", "X3", "X2Z", "XZ2", "Z3"]


def mono_vals(X, Y, Z):
    return [Y * Y * X, Y * Y * Z, Y * X * X, Y * X * Z, Y * Z * Z, X ** 3, X * X * Z, X * Z * Z, Z ** 3]


def proj_lift(x, y, p):
    rows = [[x, y, 1], [p, 0, 0], [0, p, 0], [0, 0, p]]
    red = [[int(v) for v in row] for row in fmpz_mat(rows).lll().tolist()]
    cand = [v for v in red if any(v) and v[2] % p != 0]
    v = min(cand, key=lambda w: max(abs(c) for c in w))
    c = v[2] * pow(1, -1, p) % p
    assert (v[0] - c * x) % p == 0 and (v[1] - c * y) % p == 0
    return tuple(v)


def fit_cubic(lifts, p, A, B):
    rows = [mono_vals(*L) for L in lifts]
    ker = X0.int_kernel(rows, 9)
    uE = [0, 1, 0, 0, 0, -1, 0, (-A) % p, (-B) % p]
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
    Lb = [[sum(xi * ker[j][c] for j, xi in enumerate(xv)) for c in range(9)] for xv in Wb]
    red = [[int(v) for v in row] for row in fmpz_mat(Lb).lll().tolist()]
    red.sort(key=lambda u: sum(v * v for v in u))
    for u in red:
        if u[1] % p == 0:
            continue
        c = u[1] * pow(1, -1, p) % p
        if all((ui - c * ei) % p == 0 for ui, ei in zip(u, uE)):
            for row in rows:
                assert sum(a * b for a, b in zip(row, u)) == 0
            return u, "ok"
    return None, "no-congruent"


def poly_eval(cs, t):
    acc = Fraction(0)
    for c in reversed(cs):
        acc = acc * t + c
    return acc


def pmul(a, b):
    out = [Fraction(0)] * (len(a) + len(b) - 1)
    for i, x in enumerate(a):
        for j, y in enumerate(b):
            out[i + j] += x * y
    return out


def padd(a, b):
    n = max(len(a), len(b))
    return [(a[i] if i < len(a) else 0) + (b[i] if i < len(b) else 0) for i in range(n)]


def taylor_shift(cs, t0):
    """coefficients of g(t0 + s) in s."""
    out = [Fraction(0)]
    for c in reversed(cs):
        out = padd(pmul(out, [Fraction(t0), Fraction(1)]), [Fraction(c)])
    return out + [Fraction(0)] * (5 - len(out))


def cubic_to_weierstrass(u, lifts):
    """returns (W integral a-invariants, list of integral points on W) or (None, reason)."""
    u = [Fraction(v) for v in u]
    L1 = [u[1], u[0]]                     # L1(t) = u0 t + u1   (coeffs low -> high)
    Q2 = [u[4], u[3], u[2]]               # u2 t^2 + u3 t + u4
    K3 = [u[8], u[7], u[6], u[5]]         # u5 t^3 + ...
    if u[0] == 0:
        return None, "tangent-at-O-is-Z=0"
    t0 = -u[1] / u[0]
    q0 = -poly_eval(Q2, t0)
    if q0 == 0:
        return None, "q0=0"
    g = padd(pmul(Q2, Q2), [-4 * c for c in pmul(L1, K3)])
    G = taylor_shift(g, t0)               # G[0] + G[1] s + ... + G[4] s^4
    assert G[0] == q0 * q0, "constant term must be q0^2"
    d, c, b, a = G[1], G[2], G[3], G[4]
    a1 = d / q0
    a2 = c - d * d / (4 * q0 * q0)
    a3 = 2 * q0 * b
    a4 = -4 * q0 * q0 * a
    a6 = a2 * a4
    pts = []
    for (Xl, Yl, Zl) in lifts:
        tt, yy = Fraction(Xl, Zl), Fraction(Yl, Zl)
        w = 2 * poly_eval(L1, tt) * yy + poly_eval(Q2, tt)
        s = tt - t0
        if s == 0:
            return None, "point-on-tangent-at-O"
        assert w * w == poly_eval(G, s)
        x = (2 * q0 * (w + q0) + d * s) / (s * s)
        y = (4 * q0 * q0 * (w + q0) + 2 * q0 * (d * s + c * s * s) - d * d * s * s / (2 * q0)) / s ** 3
        assert y * y + a1 * x * y + a3 * y == x ** 3 + a2 * x * x + a4 * x + a6, "Washington map"
        pts.append((x, y))
    # integral model: (x, y) -> (U^2 x, U^3 y), a_i -> U^i a_i
    den = 1
    for i, ai in zip((1, 2, 3, 4, 6), (a1, a2, a3, a4, a6)):
        # need U^i a_i integral: take U = lcm of denominators (crude but valid)
        den = math.lcm(den, ai.denominator)
    U = den
    W = tuple(int(ai * U ** i) for i, ai in zip((1, 2, 3, 4, 6), (a1, a2, a3, a4, a6)))
    assert all(Fraction(W[k]) == ai * U ** i for k, (i, ai) in enumerate(zip((1, 2, 3, 4, 6), (a1, a2, a3, a4, a6))))
    ipts = [(x * U * U, y * U ** 3) for (x, y) in pts]
    return (W, ipts), "ok"


def to_short_rational(W, ipts):
    b2, b4, b6, b8, c4, c6, D = X0.weier_invariants(*W)
    if D == 0:
        return None
    As, Bs = -27 * c4, -54 * c6
    spts = []
    for (x, y) in ipts:
        xs, ys = 36 * x + 3 * b2, 108 * (2 * y + W[0] * x + W[2])
        assert ys * ys == xs ** 3 + As * xs + Bs
        spts.append((xs, ys))
    # clear denominators of points by scaling with V: (x,y)->(V^2 x, V^3 y), A->V^4 A, B->V^6 B
    V = 1
    for (x, y) in spts:
        V = math.lcm(V, x.denominator, y.denominator)
    # V^2 x integral needs x.den | V^2 ; choose V = lcm of dens (sufficient)
    spts2 = [(int(x * V * V), int(y * V ** 3)) for (x, y) in spts]
    for (x, y), (xo, yo) in zip(spts2, spts):
        assert Fraction(x) == xo * V * V and Fraction(y) == yo * V ** 3
    As2, Bs2 = int(As) * V ** 4, int(Bs) * V ** 6
    for (x, y) in spts2:
        assert y * y == x ** 3 + As2 * x + Bs2
    return dict(W=W, c4=int(c4) * V ** 4, c6=int(c6) * V ** 6, D=int(D) * V ** 12, As=As2, Bs=Bs2,
                spts=spts2, V_bits=V.bit_length())


def run_trial(tg, r, arm, seed, n_dbl):
    rng = random.Random(seed)
    p, A, B, q, S = tg["p"], tg["A"], tg["B"], tg["q"], tg["S"]
    k = rng.randrange(1, q)
    T = X0.ec_mul(k, S, A, p)
    s = [rng.randrange(q) for _ in range(r)]
    t = [rng.randrange(q) for _ in range(r)]
    if arm == "PS":
        s[2], t[2] = (s[0] + s[1]) % q, (t[0] + t[1]) % q
    cs = [(si - k * ti) % q for si, ti in zip(s, t)]
    rec = dict(p=p, q=q, r=r, arm=arm, seed=seed, bits=p.bit_length())
    if any(c == 0 for c in cs):
        rec["status"] = "point-at-infinity"
        return rec
    Ppts = [X0.ec_mul(c, S, A, p) for c in cs]
    if len(set(Ppts)) < r:
        rec["status"] = "repeated-point"
        return rec
    lifts = [proj_lift(P[0], P[1], p) for P in Ppts]
    rec["lift_maxbits"] = max(abs(v).bit_length() for L in lifts for v in L)
    try:
        u, why = fit_cubic(lifts, p, A, B)
    except AssertionError as e:
        rec["status"] = "kernel-rank"
        return rec
    if u is None:
        rec["status"] = why
        return rec
    rec["u_maxbits"] = max(abs(v).bit_length() for v in u)
    cw, why = cubic_to_weierstrass(u, lifts)
    if cw is None:
        rec["status"] = why
        return rec
    sh = to_short_rational(*cw)
    if sh is None:
        rec["status"] = "singular"
        return rec
    rec["status"] = "ok"
    rec["W"] = [hex(v) for v in cw[0]]
    rec["log2_absDW"] = math.log2(abs(sh["D"]))
    rec["max_xs_bits"] = max(abs(x).bit_length() for x, _ in sh["spts"])
    if arm == "DET":   # detector positive control: replace P'_3 by P'_1 + P'_2 computed over Q
        (x1, y1), (x2, y2) = [tuple(map(Fraction, P)) for P in sh["spts"][:2]]
        lam_ = (y2 - y1) / (x2 - x1)
        x3 = lam_ * lam_ - x1 - x2
        y3 = lam_ * (x1 - x3) - y1
        V = math.lcm(x3.denominator, y3.denominator)
        sp = [(x * V * V, y * V ** 3) for (x, y) in sh["spts"][:2]] + [(int(x3 * V * V), int(y3 * V ** 3))] + \
             [(x * V * V, y * V ** 3) for (x, y) in sh["spts"][3:]]
        assert Fraction(sp[2][0]) == x3 * V * V
        sh = dict(sh, spts=sp, As=sh["As"] * V ** 4, Bs=sh["Bs"] * V ** 6, D=sh["D"] * V ** 12, c4=sh["c4"] * V ** 4)
        for (x, y) in sp:
            assert y * y == x ** 3 + sh["As"] * x + sh["Bs"]
        cs = cs[:2] + [(cs[0] + cs[1]) % q] + cs[3:]
        rec["det_target"] = [1, 1, -1] + [0] * (r - 3)
    sv = X0.relation_sieve(sh, cs, q, random.Random(seed ^ 0x5EED))
    rec.update({k_: v for k_, v in sv.items() if k_ not in ("cand_raw",)})
    n = sv["cand"]
    rec["rel"] = n
    if n is not None:
        rec["rel_inf"] = max(abs(x) for x in n)
        assert sum(ni * ci for ni, ci in zip(n, cs)) % q == 0
        den = sum(ni * ti for ni, ti in zip(n, t)) % q
        rec["useful"] = den != 0
        supp = [x for x in n if x]
        rec["rel_trivial"] = bool(len(supp) <= 2 and all(abs(x) == 1 for x in supp))
    else:
        rec["useful"] = False
    if arm == "PS":
        rec["planted_realized"] = bool(X0.exact_relation_holds([1, 1, -1] + [0] * (r - 3), sh["spts"], sh["As"]))
    # detector control: append P'_1 + P'_2 computed over Q, sieve must find (1,1,-1)
    if n_dbl:
        hd = height_data(sh, n_dbl=n_dbl)
        rec["heights"] = {k_: hd[k_] for k_ in ("eps", "H", "hmin", "lambda_G", "rho_G", "log_absDW",
                                                 "lang_lambdaG", "H_over_logDW", "min_eig", "lambda_vec")}
    return rec


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--bits", type=int, required=True)
    ap.add_argument("--r", type=int, required=True)
    ap.add_argument("--arm", default="X")
    ap.add_argument("--trials", type=int, default=20)
    ap.add_argument("--curves", type=int, default=2)
    ap.add_argument("--seed", type=int, default=20261115)
    ap.add_argument("--heights", type=int, default=4)
    ap.add_argument("--out", required=True)
    a = ap.parse_args()
    rng = random.Random(a.seed * 1000003 + a.bits * 101 + a.r)
    with open(a.out, "a") as fo:
        for ci in range(a.curves):
            tg = X0.make_target(a.bits, rng)
            for j in range(a.trials):
                sd = rng.randrange(1 << 62)
                t0 = time.time()
                rec = run_trial(tg, a.r, a.arm, sd, a.heights)
                rec["sec"] = round(time.time() - t0, 3)
                rec["curve"] = ci
                fo.write(json.dumps(rec, default=str) + "\n")
                fo.flush()


if __name__ == "__main__":
    main()
