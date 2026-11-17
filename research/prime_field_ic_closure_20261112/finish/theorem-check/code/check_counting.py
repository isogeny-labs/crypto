"""Exhaustive toy checks of Lemma 1 (K1), Lemma 2 (counting), Lemma 7 (Hilbert series) and the
Wald step of Lemma 9.  [pcc:unverified]"""
import sys, json, random, itertools
from math import comb
import numpy as np
import icmodel as M
import fastdelta as F

out = {}
rng = random.Random("counting")
# ---------------- Lemma 1: K1 on random polynomials (split, kummer, random) over F_p
p = 12289
a, b, pts, n = M.rand_curve(p, rng)
absc = {P[0] for P in pts}
ok = True; worst = 0.0
for trial in range(200):
    d = rng.randrange(1, 40)
    kind = trial % 3
    if kind == 0:
        roots = rng.sample(range(p), d)
        f = [1]
        for r in roots: f = M.polmul1(f, [(-r) % p, 1], p)
    elif kind == 1:
        f = [rng.randrange(p) for _ in range(d)] + [1]
    else:
        dd = [x for x in range(1, 64) if (p - 1) % x == 0]
        d = rng.choice(dd); g = rng.randrange(1, p)
        f = [(-pow(g, d, p)) % p] + [0] * (d - 1) + [1]
    deg = len(f) - 1
    Z = [x for x in range(p) if sum(c * pow(x, i, p) for i, c in enumerate(f)) % p == 0]
    Bx = [x for x in Z if x in absc]
    B = [P for P in pts if P[0] in set(Bx)]
    ok &= len(Z) <= deg and len(B) <= 2 * len(Bx) <= 2 * deg
    worst = max(worst, len(B) / (2 * deg))
out["L1"] = {"polys": 200, "all_bounds_hold": ok, "max_|B|/(2deg f)": round(worst, 4)}

# ---------------- Lemma 2: exhaustive over all targets on a small prime-order curve
def lemma2(p, base, m, curve_rng):
    from sympy import isprime
    while True:
        a, b, pts, n = M.rand_curve(p, curve_rng)
        if isprime(n): break
    absc = {P[0] for P in pts}
    W = base.pts
    Bx = sorted({w[0] for w in W if w[0] in absc})
    B = [P for P in pts if P[0] in set(Bx)]
    # decomposable targets: sums of multisets of size m
    sums = set()
    for combo in itertools.combinations_with_replacement(range(len(B)), m):
        R = None
        for i in combo: R = M.ec_add(R, B[i], p, a)
        sums.add(R)
    Ndec = len(sums)
    # V_R for every target abscissa (targets come in +-pairs with the same V_R)
    xs_grid = [tuple(w[0] for w in ws) for ws in itertools.product(W, repeat=m)]
    xvals = sorted({P[0] for P in pts})
    tot = 0; maxV = 0; degen_targets = 0
    piW = {w[0] for w in W}
    for xR in xvals:
        SR = M.semaev_special(m, p, a, b, xR)
        V = [t for t in xs_grid if M.eval_poly(SR, t, p) == 0]
        mult = sum(1 for P in pts if P[0] == xR)          # 1 or 2 targets with this abscissa
        tot += len(V) * mult
        maxV = max(maxV, len(V))
        if V and {t[0] for t in V} == piW:
            degen_targets += mult
    phi = max(sum(1 for w in W if w[0] == x) for x in piW)
    w = len(W)
    return {"p": p, "n": n, "m": m, "family": base.kind, "|W|": w, "|B_x|": len(Bx), "|B|": len(B),
            "N_dec": Ndec, "bound_multichoose": comb(len(B) + m - 1, m), "a_ok": Ndec <= comb(len(B) + m - 1, m) <= len(B) ** m,
            "max|V_R|": maxV, "bound_b": 2 ** (m - 1) * phi * w ** (m - 1), "b_ok": maxV <= 2 ** (m - 1) * phi * w ** (m - 1),
            "sum|V_R|": tot, "bound_c": 2 ** m * w ** m, "c_ok": tot <= 2 ** m * w ** m,
            "degenerate_targets": degen_targets, "bound_d": 2 ** m * phi * w ** (m - 1), "d_ok": degen_targets <= 2 ** m * phi * w ** (m - 1)}

L2 = []
crng = random.Random("L2curves")
p2 = 1153          # 9*2^7 + 1
for m in (2, 3):
    for d in (4, 6, 8):
        roots = rng.sample(range(1, p2), d)
        L2.append(lemma2(p2, M.uni_base(p2, roots), m, crng))
    for k in (2, 3):
        L2.append(lemma2(p2, M.tower_base(p2, k, rng.randrange(2, p2)), m, crng))
out["L2"] = L2
out["L2_all_ok"] = all(r["a_ok"] and r["b_ok"] and r["c_ok"] and r["d_ok"] for r in L2)

# ---------------- Lemma 7: Hilbert function by evaluation ranks vs the series, every t
L7 = []
p7 = 12289
def check7(base, m, hvec):
    N = m * base.nloc
    W = M.grid(base, m)
    tmax = m * (len(hvec) - 1)
    Hev = M.hilbert_function(N, W, p7, tmax)
    cdf, coeffs = M.poly_series_cdf(hvec, m, tmax)
    pal = coeffs == coeffs[::-1]
    half = cdf[tmax // 2] >= len(W) / 2
    return {"family": base.kind, "m": m, "N": N, "|W|^m": len(W), "H_eval": Hev, "H_series": cdf,
            "equal": Hev == cdf, "palindromic": pal, "H(rho/2)>=|W|^m/2": half}
L7.append(check7(M.uni_base(p7, rng.sample(range(p7), 5)), 2, [1] * 5))
L7.append(check7(M.uni_base(p7, rng.sample(range(p7), 4)), 3, [1] * 4))
L7.append(check7(M.tower_base(p7, 3, rng.randrange(2, p7)), 2, [1, 3, 3, 1]))
L7.append(check7(M.tower_base(p7, 2, rng.randrange(2, p7)), 3, [1, 2, 1]))
L7.append(check7(M.bits_base(p7, 3), 2, [1, 3, 3, 1]))
out["L7"] = [{k: v for k, v in r.items() if k not in ("H_eval", "H_series")} | {"H": r["H_eval"]} for r in L7]
out["L7_all_ok"] = all(r["equal"] and r["palindromic"] and r["H(rho/2)>=|W|^m/2"] for r in L7)

# ---------------- Lemma 9 (Wald step): attempts per relation = n / N_dec, by simulation
p9 = 1153
from sympy import isprime
while True:
    a9, b9, pts9, n9 = M.rand_curve(p9, crng)
    if isprime(n9): break
B9 = rng.sample(pts9, 24)
B9 = list({P for P in B9} | {M.ec_neg(P, p9) for P in B9})
dec = set()
for c in itertools.combinations_with_replacement(range(len(B9)), 2):
    dec.add(M.ec_add(B9[c[0]], B9[c[1]], p9, a9))
Gpts = pts9 + [None]
h = 50; runs = 400; Ts = []
for _ in range(runs):
    succ = 0; T = 0
    while succ < h:
        T += 1
        if rng.choice(Gpts) in dec: succ += 1
    Ts.append(T)
pred = h * n9 / len(dec)
out["L9_wald"] = {"n": n9, "N_dec": len(dec), "h": h, "runs": runs, "mean_attempts": round(float(np.mean(Ts)), 2),
                  "pred_h*n/N_dec": round(pred, 2), "rel_err": round(abs(float(np.mean(Ts)) - pred) / pred, 4),
                  "stderr": round(float(np.std(Ts) / np.sqrt(runs)), 2)}
print(json.dumps(out, indent=1))
