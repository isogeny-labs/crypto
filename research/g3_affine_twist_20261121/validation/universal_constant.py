#!/usr/bin/env python3
"""Validator check that one constant links prod(theta_B) and S_L for every S.

For B_S = C^3/(Z^3 + (tau+S)/2 Z^3), S in {0,1}^6 symmetric (all 64), compute the 36 even
theta-nulls of B_S with a validator-written theta series (numpy), and S_L(rho) from the
stored formulas/f_S.json with rho_ij = (e_j - e_i)^(1/4) > 0 at the producer's real
configuration (derive.py seed 1).  The claim chi_18(B_S, omega) = D * S_L with a single
D for pair and affine S requires prod_even theta_B_S / S_L(S) to be the same number for
all 64 S.  The period matrix tau and the a-period matrix A come from the frozen lead-#3
theta_arm.py (read only); everything else is independent of derive.py / thetalib.py.
Usage (needs mpmath): python -I universal_constant.py STUDY_DIR
"""
import cmath
import importlib.util
import itertools
import json
import math
import os
import random
import sys

import numpy as np


def load_theta_arm(study):
    path = os.path.join(study, "..", "g3_transfer_gates_20261120", "theta_arm.py")
    spec = importlib.util.spec_from_file_location("theta_arm_ro", path)
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


def thetas(T):
    T = np.array(T, dtype=complex)
    lam = np.linalg.eigvalsh(T.imag).min()
    R = int(math.ceil(math.sqrt(60.0 / (math.pi * lam)))) + 2
    rng = np.arange(-R, R + 1)
    N = np.array(list(itertools.product(rng, rng, rng)), dtype=float)
    out = {}
    for a in itertools.product((0, 1), repeat=3):
        V = N + np.array(a) / 2.0
        Q = np.einsum("ij,jk,ik->i", V, T, V)
        E = np.exp(1j * math.pi * Q)
        for b in itertools.product((0, 1), repeat=3):
            ph = np.exp(1j * math.pi * (V @ np.array(b, dtype=float)))
            out[(a, b)] = complex((E * ph).sum())
    return out, R


def main(study):
    from mpmath import mpf, mp
    import mpmath
    from mpmath.calculus.quadrature import GaussLegendre
    ta = load_theta_arm(study)
    rng = random.Random(1)
    es = [mpf(3 * i) + mpf(rng.uniform(0.2, 1.8)) for i in range(8)]
    _, _, _, tau = ta.period_matrix(es, 5)
    T0 = [[complex((tau[i, j] + tau[j, i]) / 2) for j in range(3)] for i in range(3)]
    gl = GaussLegendre(mp)
    nodes = gl.calc_nodes(5, mp.prec)
    Am = mpmath.matrix(3, 3)
    for i in range(3):
        lo, hi = es[2 * i], es[2 * i + 1]
        va = ta.path_integral(ta.rect_pieces_a(lo - mpf("0.5"), hi + mpf("0.5"), mpf("0.5")), es, nodes)
        for k in range(3):
            Am[k, i] = va[k]
    detA = complex(mpmath.det(Am))
    PAIRS = [(i, j) for i in range(8) for j in range(i + 1, 8)]
    rho = [float(es[j] - es[i]) ** 0.25 for (i, j) in PAIRS]
    even = [(a, b) for a in itertools.product((0, 1), repeat=3) for b in itertools.product((0, 1), repeat=3)
            if sum(x * y for x, y in zip(a, b)) % 2 == 0]
    assert len(even) == 36
    rows = []
    for sb in itertools.product((0, 1), repeat=6):
        name = "f_" + "".join(map(str, sb)) + ".json"
        j = json.load(open(os.path.join(study, "formulas", name)))
        S = [[0] * 3 for _ in range(3)]
        idx = 0
        for i in range(3):
            for k in range(i, 3):
                S[i][k] = S[k][i] = sb[idx]
                idx += 1
        TB = [[(T0[i][k] + S[i][k]) / 2 for k in range(3)] for i in range(3)]
        th, R = thetas(TB)
        mx = max(abs(v) for v in th.values())
        nvan = sum(1 for c in even if abs(th[c]) / mx < 1e-9)
        P = 1 + 0j
        for c in even:
            P *= th[c]
        SL = 1 + 0j
        for fac in j["formula"]:
            acc = 0j
            for k, v in fac["terms"]:
                acc += cmath.exp(2j * math.pi * k / 8) * math.prod(r ** e for r, e in zip(rho, v))
            SL *= acc
        rows.append({"S": "".join(map(str, sb)), "kind": j["report"]["kind"], "R": R, "B_vanishing_even": nvan,
                     "ratio": P / SL})
    ratios = np.array([r["ratio"] for r in rows])
    med = np.median(ratios.real) + 1j * np.median(ratios.imag)
    dev = np.abs(ratios / med - 1)
    kappa = -(2 * math.pi) ** -6
    predicted = kappa ** 9 * detA ** 18
    res = {
        "n_formulas": len(rows),
        "kinds": {k: sum(1 for r in rows if r["kind"] == k) for k in ("pair", "affine")},
        "max_rel_dev_of_ratio_from_median_all": float(dev.max()),
        "max_rel_dev_pair": float(max(d for d, r in zip(dev, rows) if r["kind"] == "pair")),
        "max_rel_dev_affine": float(max(d for d, r in zip(dev, rows) if r["kind"] == "affine")),
        "median_ratio": [med.real, med.imag],
        "kappa^9_detA^18_with_kappa=-(2pi)^-6": [predicted.real, predicted.imag],
        "median_over_predicted": [(med / predicted).real, (med / predicted).imag],
        "any_vanishing_even_theta_of_B": sum(r["B_vanishing_even"] for r in rows),
        "theta_truncation_R": sorted({r["R"] for r in rows}),
        "es": [float(e) for e in es],
    }
    print(json.dumps(res, indent=1))
    json.dump({"summary": res, "rows": [{k: (v if k != "ratio" else [v.real, v.imag]) for k, v in r.items()}
                                        for r in rows]},
              open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "universal_constant.out.json"), "w"),
              indent=1)


if __name__ == "__main__":
    main(sys.argv[1])
