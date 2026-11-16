"""Conservative (guaranteed) reach of a shift family -- the "LP bound" side.

Howgrave-Graham: if the LLL output h satisfies ||h(xX,yY)|| < p^m / sqrt(n),
then h(x0,y0) = 0 over Z.  LLL guarantees ||b1|| <= 2^((n-1)/4) det(L)^(1/n),
so recovery is *guaranteed* when

    2^((n-1)/4) * det(L)^(1/n) < p^m / sqrt(n).

For a fixed family this is monotone in delta, so the guaranteed reach delta*
is found by bisection.  This is a lower bound on what the family achieves
(it is sufficient, not necessary): the measured reach can be larger, because
the planted vector may be shorter than the volume heuristic.  That gap is the
subject of REPORT.md S7.

det is computed exactly (sympy over Z) on a maximal independent row subset.
m >= 4 is skipped by default: the determinant is 80x80 or larger.
"""
import json
import random
import sys

import sympy as sp

from experiments.idea1_lattice import s3_xy, build_shifts
from lib import lattice as L
from lib.ec import Curve


def lattice_stats(p, B, f, m, fam):
    M, mons = L.build({}, build_shifts(f, m, p, fam), B, B)
    Sm = sp.Matrix(M)
    piv = Sm.T.rref()[1]
    Mi = [M[i] for i in piv]
    n = len(Mi)
    A = sp.Matrix(Mi)
    gram = A * A.T                      # n x n, det = vol^2
    d = int(gram.det())
    if d <= 0:
        return None
    return n, d                          # d = vol^2


def guaranteed(pbits, delta, m, fam, a=0, b=7, seed=5):
    rng = random.Random(seed)
    p = int(sp.nextprime(2**pbits))
    E = Curve(p, a, b)
    B = max(8, int(round(p ** delta)))
    base = E.points_with_x_upto(B, rng)
    if len(base) < 2:
        return None
    P, Q = rng.sample(base, 2)
    R = E.add(P, Q)
    st = lattice_stats(p, B, s3_xy(R[0], a, b, p), m, fam)
    if st is None:
        return None
    n, vol2 = st
    # log2 of the guaranteed bound, compared with m*log2(p)
    log2_vol = (vol2.bit_length() - 1) / 2   # vol = sqrt(det Gram)
    lhs = log2_vol / n + (n - 1) / 4 + 0.5 * (n.bit_length())
    return lhs, m * (p.bit_length() - 1), n, B


def reach(pbits, m, fam, lo=0.02, hi=0.30, iters=6):
    """Bisection on delta for lhs(delta) = m*log2(p)."""
    ok_lo = guaranteed(pbits, lo, m, fam)
    ok_hi = guaranteed(pbits, hi, m, fam)
    if ok_lo is None or ok_hi is None:
        return None
    f_lo = ok_lo[0] - ok_lo[1]
    f_hi = ok_hi[0] - ok_hi[1]
    if f_lo < 0:
        return {"delta_star": lo, "note": "guaranteed at lo"}
    if f_hi >= 0:
        return {"delta_star": 0.0, "note": "no certified reach (bound above p^m at every delta)"}
    for _ in range(iters):
        mid = (lo + hi) / 2
        r = guaranteed(pbits, mid, m, fam)
        if r is None:
            break
        f_mid = r[0] - r[1]
        if f_mid < 0:
            lo = mid
        else:
            hi = mid
    return {"delta_star": round((lo + hi) / 2, 4), "rank": r[2], "B": r[3]}


if __name__ == "__main__":
    cfg = json.loads(sys.argv[1]) if len(sys.argv) > 1 else {}
    out = []
    for pbits in cfg.get("pbits", [40]):
        for m in cfg.get("ms", [2, 3]):
            for fam in cfg.get("families", ["A", "F"]):
                r = reach(pbits, m, fam)
                rec = {"p_bits": pbits, "m": m, "family": fam, **(r or {})}
                print(rec, flush=True)
                out.append(rec)
    json.dump(out, open(cfg.get("out", "reach_bound.json"), "w"), indent=1)


def univariate_certificate(pbits, m, deg=2, a=3, lo=0.02, hi=0.60, iters=8):
    """Validation: the same criterion on degree-2 univariate Coppersmith.

    Theory: the certified reach tends to 1/2 as m grows.  If this sequence does
    not rise towards 0.50 the criterion is implemented wrongly.
    """
    import sympy as sp
    from experiments.control_univariate import univariate_shifts
    N = int(sp.nextprime(2**pbits))

    def lhs(e):
        x0 = int(N**e)
        b = (-x0*x0 - a*x0) % N
        M, _ = L.build({}, univariate_shifts([b, a, 1], m, N), x0, 1)
        A = sp.Matrix(M); d = int((A*A.T).det())
        n = len(M)
        return (d.bit_length()-1)/2/n + (n-1)/4 + 0.5*n.bit_length()

    rhs = m*(N.bit_length()-1)
    for _ in range(iters):
        mid = (lo+hi)/2
        if lhs(mid) < rhs:
            lo = mid
        else:
            hi = mid
    return round((lo+hi)/2, 3)
