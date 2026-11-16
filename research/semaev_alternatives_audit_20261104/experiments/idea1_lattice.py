"""Idea 1 audit, v1: shift-family probe for S_3 over F_p.

For a fixed target R, f(x,y) = S_3(x, y, x_R) mod p has bidegree (2,2).  A
shift family is a set of polynomials  x^i y^j f^k p^(m-k)  which vanish to
order m at the planted root.  Column scaling by B^(u+v) and LLL give
polynomials h; success = |h(x0,y0)| < p^m (then h = 0 over Z).

The harness is validated by `control_univariate.py` (recovers the theoretical
1/2 bound on a quadratic).  The bivariate families here are probes, not the
cuso-optimised shift set: the report's §7 volume table explains why they are
weak (rank << monomial count, so det^(1/n) > p^m).

Costs: curve ops from lib.ec counters, coefficient multiplications counted in
poly_mul, lattice reduction charged by lib.lattice.lll_charge.
"""
import json
import random
import sys

import sympy as sp

from lib import lattice as L
from lib.accounting import Record, write_jsonl
from lib.ec import Curve, Counter


def s3_xy(z, a, b, p):
    f = {}
    def add(u, v, c):
        f[(u, v)] = f.get((u, v), 0) + c
    add(2, 2, 1)
    add(2, 1, -2*z)
    add(1, 2, -2*z)
    add(1, 1, -2*z*z - 2*a)
    add(2, 0, z*z)
    add(0, 2, z*z)
    add(1, 0, -2*a*z - 4*b)
    add(0, 1, -2*a*z - 4*b)
    add(0, 0, a*a - 4*b*z)
    return {k: v % p for k, v in f.items() if v % p}


def family(name, m):
    """Return list of (i, j, k) shift triples."""
    if name == "A":            # i <= 2(m-k)
        return [(i, 0, k) for k in range(m+1) for i in range(2*(m-k)+1)]
    if name == "B":            # i < 2
        return [(i, 0, k) for k in range(m+1) for i in range(2)]
    if name == "E":            # i, j < 2
        return [(i, j, k) for k in range(m+1) for i in range(2) for j in range(2)]
    if name == "F":            # i + j <= 2(m-k)
        return [(i, j, k) for k in range(m+1) for i in range(2*(m-k)+1)
                for j in range(2*(m-k)+1-i)]
    raise ValueError(name)


def build_shifts(f, m, p, name):
    fj = [{(0, 0): 1}]
    for _ in range(m):
        fj.append(L.poly_mul(fj[-1], f))
    shifts = []
    for (i, j, k) in family(name, m):
        g = {(u+i, v+j): c * p**(m-k) for (u, v), c in fj[k].items()}
        shifts.append(((0, 0), g))
    return shifts


def cell(pbits, delta, m, fam, trials, a=0, b=7, seed=5, solve=True):
    rng = random.Random(seed)
    counter = Counter()
    p = int(sp.nextprime(2**pbits))
    E = Curve(p, a, b, counter)
    B = max(8, int(round(p ** delta)))
    base = E.points_with_x_upto(B, rng)
    if len(base) < 8:
        return None
    found = solved = 0
    dim = monoms = bits = 0
    coeff_mults = lll_ops = 0
    for _ in range(trials):
        P, Q = rng.sample(base, 2)
        R = E.add(P, Q)
        xR, x0, y0 = R[0], P[0], Q[0]
        f = s3_xy(xR, a, b, p)
        shifts = build_shifts(f, m, p, fam)
        M, mons = L.build({}, shifts, B, B)
        n = len(M)
        Lb = max(abs(c).bit_length() for row in M for c in row if c)
        rows = L.reduce_fplll(M)
        pm = p ** m
        polys, trivial = [], 0
        for row in rows:
            h = L.to_poly(row, mons, B, B)
            if abs(L.eval_int(h, x0, y0)) < pm:
                if L.is_multiple_of(h, f):
                    trivial += 1
                else:
                    polys.append(h)
        if polys:
            found += 1
            if solve and len(polys) >= 2 and L.recover(polys, p, x0, y0):
                solved += 1
        dim, monoms, bits = n, len(mons), max(bits, Lb)
        coeff_mults += n * len(mons)
        lll_ops += L.lll_charge(n, Lb)
    return Record(
        experiment="idea1_shift_family",
        config={"p_bits": pbits, "delta": float(delta), "m": m, "family": fam,
                "variant": "j0" if a == 0 else "generic", "trials": trials},
        counts={"group_add": counter.group_add, "inv": counter.inv,
                "mul": counter.mul, "sqr": counter.sqr, "pow": counter.pow,
                "coeff_mults": coeff_mults, "lattice_word_ops": lll_ops},
        metrics={"rank": dim, "monomials": monoms, "max_bits": bits,
                 "found": found, "solved": solved, "trivial_multiples": trivial,
                 "B": B, "support": len(f)},
    )


if __name__ == "__main__":
    cfg = json.loads(sys.argv[1]) if len(sys.argv) > 1 else {}
    recs = []
    for pbits in cfg.get("pbits", [40, 48]):
        for delta in cfg.get("deltas", [0.10, 0.12, 0.14, 0.16]):
            for m in cfg.get("ms", [2, 3]):
                for fam in cfg.get("families", ["A", "B", "E"]):
                    for (av, tag) in cfg.get("variants", [[0, "j0"]]):
                        r = cell(pbits, delta, m, fam, cfg.get("trials", 8), a=av)
                        if r:
                            d = r.to_dict()
                            d["config"]["variant"] = tag
                            print(json.dumps(d), flush=True)
                            recs.append(d)
    write_jsonl(cfg.get("out", "idea1_lattice.jsonl"), recs)
