"""Idea 12: random search over shift families, with the certificate as reward.

The document proposes learning a shift-selection policy.  Before any learning,
the cheap question is whether the diamond family F (i+j <= 2(m-k)) is already
near-optimal among its own subsets.  We sample random subsets of F of several
sizes, run the same planted-recovery harness, and report the best.

A family that beats F would be a genuine finding; a flat landscape means the
LP/certificate optimum (Idea 1) is the right target, not search.
"""
import json
import random
import sys

import sympy as sp

from experiments.idea1_lattice import s3_xy, family
from lib import lattice as L
from lib.accounting import Record, write_jsonl
from lib.ec import Curve, Counter


def build_from_triples(f, m, p, triples):
    fj = [{(0, 0): 1}]
    for _ in range(m):
        fj.append(L.poly_mul(fj[-1], f))
    out = []
    for (i, j, k) in triples:
        g = {(u+i, v+j): c * p**(m-k) for (u, v), c in fj[k].items()}
        out.append(((0, 0), g))
    return out


def score(triples, pbits, delta, m, trials, a=0, b=7, seed=5):
    rng = random.Random(seed)
    counter = Counter()
    p = int(sp.nextprime(2**pbits))
    E = Curve(p, a, b, counter)
    B = max(8, int(round(p ** delta)))
    base = E.points_with_x_upto(B, rng)
    if len(base) < 8:
        return None
    found = solved = 0
    rank = monoms = 0
    for _ in range(trials):
        P, Q = rng.sample(base, 2)
        R = E.add(P, Q)
        f = s3_xy(R[0], a, b, p)
        M, mons = L.build({}, build_from_triples(f, m, p, triples), B, B)
        rank, monoms = len(M), len(mons)
        rows = L.reduce_fplll(M)
        pm = p ** m
        polys = []
        for row in rows:
            h = L.to_poly(row, mons, B, B)
            if abs(L.eval_int(h, P[0], Q[0])) < pm and not L.is_multiple_of(h, f):
                polys.append(h)
        if polys:
            found += 1
            if len(polys) >= 2 and L.recover(polys, p, P[0], Q[0]):
                solved += 1
    return {"found": found, "solved": solved, "rank": rank, "monomials": monoms}


def main(cfg):
    rng = random.Random(cfg.get("seed", 2))
    m = cfg.get("m", 2)
    pbits = cfg.get("p_bits", 40)
    delta = cfg.get("delta", 0.15)
    trials = cfg.get("trials", 4)
    full = family("F", m)
    rows = []

    rows.append({"name": "F", **(score(full, pbits, delta, m, trials) or {})})
    rows.append({"name": "A", **(score(family("A", m), pbits, delta, m, trials) or {})})
    for size in cfg.get("sizes", [8, 12, 16]):
        for k in range(cfg.get("per_size", 10)):
            sub = rng.sample(full, min(size, len(full)))
            r = score(sub, pbits, delta, m, trials)
            if r:
                rows.append({"name": f"rand{size}_{k}", "size": size, **r})
    rows.sort(key=lambda r: (-r.get("solved", 0), -r.get("found", 0)))
    for r in rows[:8]:
        print(r, flush=True)
    json.dump(rows, open(cfg.get("out", "idea12_shift_search.json"), "w"), indent=1)
    return rows


if __name__ == "__main__":
    cfg = json.loads(sys.argv[1]) if len(sys.argv) > 1 else {}
    main(cfg)
