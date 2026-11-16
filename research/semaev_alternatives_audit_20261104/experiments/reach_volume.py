"""Volume diagnostic for a shift family: is the lattice able to reach p^m?

For rank r in Z^N, the lattice volume is sqrt(det(B B^T)) and the Gaussian
heuristic shortest vector is ~ vol^(1/r).  Coppersmith needs a vector shorter
than p^m, so a family with vol^(1/r) > p^m cannot succeed regardless of LLL
slack.  This script prints log2 vol^(1/r) against log2 p^m.
"""
import json
import random
import sys

import sympy as sp

from experiments.idea1_lattice import s3_xy, build_shifts
from lib import lattice as L
from lib.ec import Curve


def volume(pbits, delta, m, fam, a=0, b=7, seed=5):
    rng = random.Random(seed)
    p = int(sp.nextprime(2**pbits))
    E = Curve(p, a, b)
    B = max(8, int(round(p ** delta)))
    base = E.points_with_x_upto(B, rng)
    if len(base) < 2:
        return None
    P, Q = rng.sample(base, 2)
    R = E.add(P, Q)
    M, mons = L.build({}, build_shifts(s3_xy(R[0], a, b, p), m, p, fam), B, B)
    Sm = sp.Matrix(M)
    # keep a maximal independent row subset (some families contain a redundant shift)
    pivots = Sm.T.rref()[1]
    Mi = [M[i] for i in pivots]
    r = len(Mi)
    gram = sp.Matrix(Mi) * sp.Matrix(Mi).T
    det_bits = int(gram.det()).bit_length()
    return {"p_bits": pbits, "delta": delta, "m": m, "family": fam,
            "rank": r, "monomials": len(mons),
            "log2_vol_root": round(det_bits / (2*r), 1),
            "log2_pm": (p ** m).bit_length() - 1,
            "reaches": det_bits / (2*r) < (p ** m).bit_length() - 1}


if __name__ == "__main__":
    cfg = json.loads(sys.argv[1]) if len(sys.argv) > 1 else {}
    rows = []
    for pbits in cfg.get("pbits", [40]):
        for delta in cfg.get("deltas", [0.10, 0.14, 0.18]):
            for m in cfg.get("ms", [2, 3]):
                for fam in cfg.get("families", ["A", "B", "E", "F"]):
                    v = volume(pbits, delta, m, fam)
                    if v:
                        print(v, flush=True); rows.append(v)
    json.dump(rows, open(cfg.get("out", "reach_volume.json"), "w"), indent=1)
