#!/usr/bin/env python3
"""Step 2 over C: run the algebraic chain of fpquartic.py with complex numbers.

rho_ij = (e_j - e_i)^(1/4) for the seed-1 real configuration (derive2's branch) ->
theta^2 and same-row products (formulas/prod_001000.json) -> one square root per row,
row signs by the 315 Riemann relations -> Jacobi Aronhold matrix -> Riemann quartic.
Compared with the quartic from the directly summed odd-theta gradients of tau_B:
the Aronhold matrices must agree for EVERY relation-consistent row-sign pattern, and all
28 gradient lines must be bitangents of the chain's quartic.
Then the same with random fourth-root choices (another level structure): the chain's
quartic must still have 28 bitangents given by its own Jacobi lines (all 2016 triples
consistent) -- checked via the bitangency of the 21 non-Aronhold lines it predicts.
Writes data/cchain.json.
"""
import cmath
import itertools
import json
import math
import os
import random
import sys

import numpy as np

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import cnum as C  # noqa: E402
import tables as TB  # noqa: E402


def tc(x):
    return (tuple(x[0]), tuple(x[1]))


def chain_thetas(prods, PAIRS, es, rng=None):
    rho = []
    for (i, j) in PAIRS:
        r = complex(es[j] - es[i]) ** 0.25
        if rng is not None:
            r *= 1j ** rng.randrange(4)
        rho.append(r)
    P = {}
    for d in prods:
        acc = 0j
        for k, v in d["terms"]:
            acc += cmath.exp(2j * math.pi * k / 8) * math.prod(r ** e for r, e in zip(rho, v))
        P[(tuple(d["a"]), tuple(d["b1"]), tuple(d["b2"]))] = acc
    groups = {}
    for (a, b) in C.EVEN:
        groups.setdefault(a, []).append(b)
    base = {}
    for a, bs in groups.items():
        bs = sorted(bs)
        b0 = bs[0]
        t0 = cmath.sqrt(P[(a, b0, b0)])
        for b in bs:
            base[(a, b)] = t0 if b == b0 else P[(a, b0, b)] / t0
    return base


def main():
    pj = json.load(open(os.path.join(HERE, "formulas", "prod_001000.json")))
    rels = [([[tc(c) for c in k] for k in ks], ss) for ks, ss in
            json.load(open(os.path.join(HERE, "formulas", "riemann_rel.json")))]
    jac = {tuple(tc(n) for n in tr): ([tc(e) for e in es5], s) for tr, es5, s in
           json.load(open(os.path.join(HERE, "formulas", "jacobi.json")))}
    rng = random.Random(1)
    es = [3 * i + rng.uniform(0.2, 1.8) for i in range(8)]
    val, grad = C.theta_all(TB.tau_B(1, [0, 0, 1, 0, 0, 0]))
    sets = C.all_aronhold_sets()
    out = {}

    def patterns(base):
        res = []
        for bits in itertools.product((1, -1), repeat=7):
            s = dict(zip(C.BITS3, (1,) + bits))
            th = {c: base[c] * s[c[0]] for c in C.EVEN}
            mx = max(abs(x) for x in th.values())
            ok = all(abs(np.prod([th[c] / mx for c in ks[0]]) + s2 * np.prod([th[c] / mx for c in ks[1]])
                         + s3 * np.prod([th[c] / mx for c in ks[2]])) < 1e-9 for ks, (s2, s3) in rels)
            if ok:
                res.append(th)
        return res

    base = chain_thetas(pj["products"], pj["pairs"], es)
    pats = patterns(base)
    out["true_branch_relation_consistent_patterns"] = len(pats)
    worst = 0
    for ar in (sets[0], sets[97], sets[287]):
        H, Ag = C.weber_frame([grad[m] for m in ar])
        for th in pats:
            Aj = TB.jacobi_amatrix(jac, th, ar)
            worst = max(worst, float(np.abs(Aj - Ag).max()))
    out["true_branch_max_dev_chain_vs_gradient_aronhold_matrix"] = worst
    # random branches: the chain's own 28 lines (Aronhold frame + Jacobi) are bitangents
    rr = random.Random(7)
    devs = []
    for trial in range(5):
        base = chain_thetas(pj["products"], pj["pairs"], es, rng=rr)
        pats = patterns(base)
        th = pats[0]
        ar = sets[rr.randrange(288)]
        A = TB.jacobi_amatrix(jac, th, ar)
        Xi, kk, res = C.riemann_xi(A)
        F = C.quartic_eval_coeffs(Xi)
        # all 28 lines in the frame of ar: line(n) = (D(n,m2,m3)/D(m1,m2,m3)) / c1 ... via Cramer
        m = ar

        def D(*t):
            return TB.dval(jac, th, *t)
        c = [D(m[3], m[1], m[2]) / D(m[0], m[1], m[2]), D(m[0], m[3], m[2]) / D(m[0], m[1], m[2]),
             D(m[0], m[1], m[3]) / D(m[0], m[1], m[2])]
        dd = []
        nonaz = 0
        for n in C.ODD:
            try:
                l = np.array([D(n, m[1], m[2]) / D(m[0], m[1], m[2]) / c[0],
                              D(m[0], n, m[2]) / D(m[0], m[1], m[2]) / c[1],
                              D(m[0], m[1], n) / D(m[0], m[1], m[2]) / c[2]])
            except KeyError:
                nonaz += 1
                continue
            dd.append(C.bitangent_defect(F, l))
        devs.append({"n_patterns": len(pats), "lines_tested": len(dd), "lines_needing_syzygetic_D": nonaz,
                     "max_defect": float(max(dd))})
    out["random_branch_trials"] = devs
    json.dump(out, open(os.path.join(HERE, "data", "cchain.json"), "w"), indent=1)
    print(json.dumps(out, indent=1))


if __name__ == "__main__":
    main()
