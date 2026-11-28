#!/usr/bin/env python3
"""Height-estimator control: on small curves built like lambda_calib.py (cubic through r small
points + O, so Delta is factorable), compare the doubling-limit Gram matrix (levels 3,4,5 and the
Richardson value) with PARI's ellheightmatrix / 2 (Silverman normalisation).  [pcc:unverified]"""
import sys, os, json, math, random, time
from fractions import Fraction
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
os.environ.setdefault("LIFT_CODE", HERE)
import numpy as np
import xedni_cubic as XC, xedni_toy as X0, lh_heights
from flint import fmpz_mat
import cypari2
from cysignals.alarm import alarm, AlarmInterrupt, cancel_alarm
pari = cypari2.Pari()
pari.allocatemem(10 ** 9)
pari.set_real_precision(60)


def curve(rng, r, e):
    pts = set()
    while len(pts) < r:
        X, Y, Z = rng.randint(-e, e), rng.randint(-e, e), rng.randint(1, e)
        g = math.gcd(math.gcd(X, Y), Z)
        pts.add((X // g, Y // g, Z // g))
    lifts = sorted(pts)
    rows = [XC.mono_vals(*L) for L in lifts]
    ker = X0.int_kernel(rows, 9)
    red = [[int(v) for v in row] for row in fmpz_mat(ker).lll().tolist()]
    for u in red:
        cw, why = XC.cubic_to_weierstrass(u, lifts)
        if cw is None:
            continue
        b2, b4, b6, b8, c4, c6, D = X0.weier_invariants(*cw[0])
        if D == 0:
            continue
        W = cw[0]
        A, B = -27 * c4, -54 * c6
        P = [(36 * x + 3 * b2, 108 * (2 * y + W[0] * x + W[2])) for (x, y) in cw[1]]
        return A, B, P
    return None


def main(out, n_per, seed=20261122):
    rng = random.Random(seed)
    with open(out, "w") as fo:
        for r in (3, 5, 7):
            for e in (4, 8):
                got = 0
                while got < n_per:
                    c = curve(rng, r, e)
                    if c is None:
                        continue
                    A, B, P = c
                    rec = dict(r=r, e=e)
                    try:
                        alarm(60)
                        E = pari.ellinit([A, B])
                        PP = [[pari(f"{x.numerator}/{x.denominator}"), pari(f"{y.numerator}/{y.denominator}")] for x, y in P]
                        GP = pari.ellheightmatrix(E, PP) / 2
                        cancel_alarm()
                    except AlarmInterrupt:
                        continue
                    Gp = np.array([[float(GP[i, j]) for j in range(r)] for i in range(r)])
                    if np.linalg.eigvalsh(Gp)[0] < 1e-6:
                        continue
                    lp, _ = lh_heights.min_form(Gp)
                    rec["pari"] = dict(H=float(max(np.diag(Gp))), lam=float(lp), rho=float(max(np.diag(Gp)) / lp))
                    ml = lh_heights.height_data_ml(A, B, P, levels=(3, 4, 5))
                    for k in ("5", "rich", "rich_prev"):
                        rec[k] = dict(H=ml[k]["H"], lam=ml[k]["lambda_G"], rho=ml[k]["rho_G"],
                                      maxabsG=float(np.max(np.abs(np.array(ml[k]["G"]) - Gp))))
                    rec["logD"] = ml["log_absDs"]
                    fo.write(json.dumps(rec) + "\n")
                    fo.flush()
                    got += 1


if __name__ == "__main__":
    main(sys.argv[1], int(sys.argv[2]))
