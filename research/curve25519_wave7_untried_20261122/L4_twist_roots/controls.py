#!/usr/bin/env python3
"""L4 controls (positive, negative, identity checks).  Single core, stdlib + numpy.

  python3 controls.py --k 16 --n 20000 --out DIR

C1  Positive control, (E,E) and (T,T): for sampled non-degenerate pairs, build the points
    explicitly (y from the square-root table), compute x(P1+P2) and x(P1-P2) by the affine
    chord formula (Montgomery B = 1 for E, B = n non-residue for the twist), and require
    {x(P1+P2), x(P1-P2)} == the F_p root set of S3.  Expected 100%.
C2  Positive control for the mechanism, (E,T): points P1 in E(F_p), P2 in E'(F_p^2) with
    y2 = w*s (s^2 = n), explicit sum in F_p^2; require both x(P1+-P2) are roots of S3, are
    conjugate (X2 == X1^p), and are not in F_p.  Expected 100% of sampled pairs.
C3  Negative/sensitivity control: the same root counter with a wrong curve constant
    A' = A + 1 on (E,T) and (E,E) samples: root fractions must come out near 1/2 for both
    classes, showing the counter can return F_p roots in the mixed class when the structure
    is absent.
C4  Identity checks: disc(std)/4 == 4 f(x1) f(x2); the 'given' (task-text) polynomial's roots
    are the inverses of the std roots.
C5  Z-explanation predictor: x(P+T) = f(x)/(x-t)^2 - A - x - t for T = (t, 0); the set of
    non-Z pairs (x, phi_t(x)) is the predicted set of pairs with a root on Z.
"""
import argparse, json, os, sys, time
import numpy as np
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import l4core as L


def fval(C, x):
    p, A = C.p, C.A
    return x * ((x * x + A * x + 1) % p) % p


def affine_x(p, A, B, P, Q, sign):
    """x of P + sign*Q on B y^2 = x^3 + A x^2 + x (affine, x1 != x2), ints."""
    (x1, y1), (x2, y2) = P, Q
    y2 = y2 * sign % p
    lam = (y2 - y1) * pow(x2 - x1, -1, p) % p
    return (B * lam * lam - A - x1 - x2) % p


def sqrt_of(C, v):
    return int(C.sqrt[v % C.p])


def c1(C, n, seed_tag):
    p, A = C.p, C.A
    out = {}
    for name, lst, B in (("EE", C.Ex, 1), ("TT", C.Tx, C.nonres)):
        rng = np.random.default_rng(L.derived_seed("L4ctl", C.k, "C1", name, seed_tag))
        x1 = lst[rng.integers(0, len(lst), n)].astype(np.int64)
        x2 = lst[rng.integers(0, len(lst), n)].astype(np.int64)
        keep = x1 != x2
        x1, x2 = x1[keep], x2[keep]
        r = L.roots(C, x1, x2, want_roots=True)
        invB = pow(B, -1, p)
        match = tested = 0
        for i in range(len(x1)):
            a, b = int(x1[i]), int(x2[i])
            y1 = sqrt_of(C, int(fval(C, np.int64(a))) * invB)
            y2 = sqrt_of(C, int(fval(C, np.int64(b))) * invB)
            # both points must satisfy B y^2 = f
            assert B * y1 * y1 % p == int(fval(C, np.int64(a))) and B * y2 * y2 % p == int(fval(C, np.int64(b)))
            s = {affine_x(p, A, B, (a, y1), (b, y2), +1), affine_x(p, A, B, (a, y1), (b, y2), -1)}
            roots = {int(v) for v in (r["X1"][i], r["X2"][i]) if v >= 0}
            tested += 1
            match += (s == roots)
        out[name] = dict(tested=tested, match=match)
    return out


def f2mul(n, p, u, v):
    return ((u[0] * v[0] + n * u[1] * v[1]) % p, (u[0] * v[1] + u[1] * v[0]) % p)


def f2inv(n, p, u):
    nm = (u[0] * u[0] - n * u[1] * u[1]) % p
    iv = pow(nm, -1, p)
    return (u[0] * iv % p, (-u[1]) * iv % p)


def c2(C, n, seed_tag):
    p, A, nn = C.p, C.A, C.nonres
    rng = np.random.default_rng(L.derived_seed("L4ctl", C.k, "C2", seed_tag))
    xe = C.Ex[rng.integers(0, len(C.Ex), n)]
    xt = C.Tx[rng.integers(0, len(C.Tx), n)]
    tested = both_roots = conj = not_in_fp = fp_root_by_counter = 0
    for a, b in zip(xe.tolist(), xt.tolist()):
        y1 = sqrt_of(C, int(fval(C, np.int64(a))))                       # in F_p
        w = sqrt_of(C, int(fval(C, np.int64(b))) * pow(nn, -1, p))       # y2 = w*s
        if y1 == 0 or w == 0:
            continue
        dx = (b - a) % p
        idx = pow(dx, -1, p)
        # lambda = (y2 -+ y1)/dx with y2 = (0, w):  (-+y1, w)/dx
        xs = []
        for sg in (+1, -1):
            lam = ((-y1) % p * idx % p, (sg * w) % p * idx % p)
            lam2 = f2mul(nn, p, lam, lam)
            xs.append(((lam2[0] - A - a - b) % p, lam2[1]))
        # S3(x1,x2,X) over F_p^2 for std
        m = a * b % p
        B2 = ((a + b) * (m + 1) + 2 * A * m) % p
        ca, cc = (a - b) ** 2 % p, (m - 1) ** 2 % p
        def S3(X):
            X2 = f2mul(nn, p, X, X)
            return ((ca * X2[0] - 2 * B2 * X[0] + cc) % p, (ca * X2[1] - 2 * B2 * X[1]) % p)
        tested += 1
        both_roots += (S3(xs[0]) == (0, 0) and S3(xs[1]) == (0, 0))
        conj += (xs[0][0] == xs[1][0] and (xs[0][1] + xs[1][1]) % p == 0)
        not_in_fp += (xs[0][1] != 0)
        rr = L.roots(C, np.array([a], dtype=np.int64), np.array([b], dtype=np.int64))
        fp_root_by_counter += int(rr["nroots"][0])
    return dict(tested=tested, both_roots_verified=both_roots, conjugate=conj,
                not_in_Fp=not_in_fp, Fp_roots_found_by_counter=fp_root_by_counter)


def c3(C, n, seed_tag):
    out = {}
    for name, la, lb in (("ET", C.Ex, C.Tx), ("EE", C.Ex, C.Ex), ("TT", C.Tx, C.Tx)):
        rng = np.random.default_rng(L.derived_seed("L4ctl", C.k, "C3", name, seed_tag))
        x1 = la[rng.integers(0, len(la), n)].astype(np.int64)
        x2 = lb[rng.integers(0, len(lb), n)].astype(np.int64)
        res = {}
        for lbl, A2 in (("true_A", C.A), ("wrong_A_plus1", (C.A + 1) % C.p)):
            r = L.roots(C, x1, x2, A=A2)
            res[lbl] = dict(frac_with_Fp_root=float((r["nroots"] > 0).mean()), n=int(n))
        out[name] = res
    return out


def c4(C, n, seed_tag):
    p, A = C.p, C.A
    rng = np.random.default_rng(L.derived_seed("L4ctl", C.k, "C4", seed_tag))
    x1 = rng.integers(0, p, n).astype(np.int64)
    x2 = rng.integers(0, p, n).astype(np.int64)
    r = L.roots(C, x1, x2, want_roots=True)
    g = L.roots(C, x1, x2, variant="given", want_roots=True)
    disc_ok = bool(np.all(r["disc"] == (4 * fval(C, x1) % p) * fval(C, x2) % p))
    good = (~r["degen"]) & (~g["degen"]) & (r["X1"] != 0) & (r["X2"] != 0) & (g["X1"] != 0) & (g["X2"] != 0)
    same_n = bool(np.all(r["nroots"][good] == g["nroots"][good]))
    ok = 0
    idx = np.nonzero(good & (r["nroots"] > 0))[0]
    for i in idx[:20000]:
        s = {pow(int(v), -1, p) for v in (r["X1"][i], r["X2"][i]) if v >= 0}
        t = {int(v) for v in (g["X1"][i], g["X2"][i]) if v >= 0}
        ok += (s == t)
    return dict(disc_equals_4_f1_f2=disc_ok, same_root_count_std_vs_given=same_n,
                given_roots_are_inverses=dict(tested=int(min(len(idx), 20000)), match=int(ok)))


def predict_Z_pairs(C):
    """Predicted unordered non-Z pairs {x, phi_t(x)} (x != phi_t(x)), t in Z, by class.
    Returns dict(EE=count, TT=count, ET=count, diag=count of x with phi_t(x)=x)."""
    p, A = C.p, C.A
    pairs = set()
    diag = 0
    for t in C.Zx.tolist():
        for x in range(p):
            if x == t or C.cls[x] == 0:
                continue
            xs = np.int64(x)
            den = pow((x - t) % p, 2, p)
            ph = (int(fval(C, xs)) * pow(den, -1, p) - A - x - t) % p
            if C.cls[ph] == 0:
                continue
            if ph == x:
                diag += 1
                continue
            pairs.add((min(x, ph), max(x, ph)))
    cnt = dict(EE=0, TT=0, ET=0)
    for a, b in pairs:
        ca, cb = int(C.cls[a]), int(C.cls[b])
        cnt["EE" if ca == cb == 1 else "TT" if ca == cb == 2 else "ET"] += 1
    cnt["diag_x_with_phi_x_eq_x"] = diag
    return cnt


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--k", type=int, required=True)
    ap.add_argument("--n", type=int, default=20000)
    ap.add_argument("--n3", type=int, default=1_000_000)
    ap.add_argument("--tag", default="main")
    ap.add_argument("--out", required=True)
    ap.add_argument("--predict", action="store_true", help="also run the (slow, O(p |Z|)) Z predictor")
    a = ap.parse_args()
    os.makedirs(a.out, exist_ok=True)
    t0 = time.time()
    C = L.Curve(a.k)
    res = dict(k=a.k, tag=a.tag, curve_facts=C.facts())
    res["C1_explicit_addition_EE_TT"] = c1(C, a.n, a.tag)
    res["C2_mixed_in_Fp2"] = c2(C, a.n, a.tag)
    res["C3_wrongA_sensitivity"] = c3(C, a.n3, a.tag)
    res["C4_identities"] = c4(C, min(a.n3, 400000), a.tag)
    if a.predict:
        res["C5_predicted_Z_pairs"] = predict_Z_pairs(C)
    res["cpu_seconds"] = time.time() - t0
    fn = os.path.join(a.out, f"controls_k{a.k}_{a.tag}.json")
    with open(fn, "w") as f:
        json.dump(res, f, indent=1)
    print(fn, f"{res['cpu_seconds']:.1f}s")
    print(json.dumps({k: v for k, v in res.items() if k != "curve_facts"}, indent=1))


if __name__ == "__main__":
    main()
