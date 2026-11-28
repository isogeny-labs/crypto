#!/usr/bin/env python3
"""Individual theta-null squares and same-top-row products of B = J/L_S, algebraically.

Extends ../g3_affine_twist_20261121/derive.py (imported read-only, same seed-1 real
configuration, same Thomae partition matching and 8th roots eps_c). derive.py wrote only
18 pair products. Here, for EVERY pair of even characteristics (a;b1), (a;b2) with the
same top row a (b1 = b2 allowed, giving the square), the duplication identity

    theta_B[a;b1] theta_B[a;b2] = sum_beta theta[a+beta; b1+b2](tau+S) theta[beta; b1-b2](tau+S)

(tau_B = (tau+S)/2) is turned into kappa^(1/2) det(A) * sum_k zeta8^k prod rho_ij^v_ij.
Every identity is checked against directly summed theta series of tau_B.

Usage: python derive2.py SBITS OUT.json     (seed 1, as in the twist study)
"""
import cmath
import importlib.util
import itertools
import json
import math
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
TW = os.path.join(HERE, "..", "g3_affine_twist_20261121")
sys.path.insert(0, TW)
import thetalib as TL  # noqa: E402
_spec = importlib.util.spec_from_file_location("derive_tw", os.path.join(TW, "derive.py"))
DV = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(DV)

import mpmath  # noqa: E402
from mpmath import mpf, mp  # noqa: E402


def main(sbits, outpath, seed=1):
    import random
    rng = random.Random(seed)
    es = [mpf(3 * i) + mpf(rng.uniform(0.2, 1.8)) for i in range(8)]
    asym, posdef, signs, tau = TL.theta_arm.period_matrix(es, 5)
    from mpmath.calculus.quadrature import GaussLegendre
    gl = GaussLegendre(mp)
    nodes = gl.calc_nodes(5, mp.prec)
    Am = mpmath.matrix(3, 3)
    for i in range(3):
        lo, hi = es[2 * i], es[2 * i + 1]
        va = TL.theta_arm.path_integral(
            TL.theta_arm.rect_pieces_a(lo - mpf("0.5"), hi + mpf("0.5"), mpf("0.5")), es, nodes)
        for k in range(3):
            Am[k, i] = va[k]
    detA = complex(mpmath.det(Am))
    T = [[complex((tau[i, j] + tau[j, i]) / 2) for j in range(3)] for i in range(3)]
    S = [[0] * 3 for _ in range(3)]
    idx = 0
    for i in range(3):
        for j in range(i, 3):
            S[i][j] = S[j][i] = sbits[idx]
            idx += 1
    TS = [[T[i][j] + S[i][j] for j in range(3)] for i in range(3)]
    TB = [[TS[i][j] / 2 for j in range(3)] for i in range(3)]
    th0 = TL.thetas(T)
    thS = TL.thetas(TS)
    thB = TL.thetas(TB)
    kap0, asg0, van0 = DV.match(th0, detA, es, "tau")
    kapS, asgS, vanS = DV.match(thS, detA, es, "tau+S")
    k4 = kap0 ** 0.25
    rho = [float(es[j] - es[i]) ** 0.25 for (i, j) in DV.PAIRS]
    epsS = {}
    worst_eps = 0
    for c, A in asgS.items():
        mono = DV.thomae_mono(A)
        m = math.prod(r for r, b in zip(rho, mono) if b)
        e = thS[c] / (k4 * cmath.sqrt(detA) * m)
        ang = cmath.phase(e) / (2 * math.pi) * 8
        worst_eps = max(worst_eps, abs(abs(e) - 1), abs(ang - round(ang)))
        epsS[c] = (A, round(ang) % 8)
    groups = {}
    for (a, b) in TL.EVEN:
        groups.setdefault(a, []).append(b)
    prods = []
    worst = 0
    for a, bs in sorted(groups.items()):
        for b1, b2 in itertools.combinations_with_replacement(sorted(bs), 2):
            terms = []
            direct = thB[(a, b1)] * thB[(a, b2)]
            for beta in TL.BITS3:
                A1, G1, s1 = TL.reduce_char(tuple(x + y for x, y in zip(a, beta)),
                                            tuple(x + y for x, y in zip(b1, b2)))
                A2, G2, s2 = TL.reduce_char(beta, tuple(x - y for x, y in zip(b1, b2)))
                if (A1, G1) not in epsS or (A2, G2) not in epsS:
                    continue  # odd or the vanishing even char of tau+S: theta = 0
                P1, k1 = epsS[(A1, G1)]
                P2, k2 = epsS[(A2, G2)]
                k = (k1 + k2 + (0 if s1 * s2 == 1 else 4)) % 8
                v = [x + y for x, y in zip(DV.thomae_mono(P1), DV.thomae_mono(P2))]
                terms.append((k, v))
            alg = sum(cmath.exp(2j * math.pi * k / 8) * math.prod(r ** e for r, e in zip(rho, v))
                      for k, v in terms) * k4 ** 2 * detA
            worst = max(worst, abs(alg - direct) / max(abs(direct), 1e-300))
            prods.append({"a": a, "b1": b1, "b2": b2, "terms": terms})
    rep = {"seed": seed, "S": sbits, "tau_asym": float(asym), "eps_dev": worst_eps,
           "n_products": len(prods), "max_reldev_alg_vs_direct": worst,
           "vanishing_even_tauS": [list(map(list, vanS))]}
    json.dump({"report": rep, "products": prods, "pairs": DV.PAIRS}, open(outpath, "w"))
    print(json.dumps(rep))


if __name__ == "__main__":
    main([int(c) for c in sys.argv[1]], sys.argv[2])
