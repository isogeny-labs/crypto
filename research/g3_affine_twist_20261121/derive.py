#!/usr/bin/env python3
"""Derive the algebraic chi_18 formula for J/L (L = L_S, the kernel of
J = C^3/(Z^3+tau Z^3) -> C^3/(Z^3 + (tau+S)/2 Z^3)) in terms of fourth roots of
branch-point differences, from numerics on one real configuration.

Output formula JSON: for each pair (c1,c2) of even chars of B, a list of terms
(zeta8 exponent k, exponent vector over the 28 pairs i<j) so that
  theta_B[c1] theta_B[c2] = kappa^(1/2) det(A) * sum zeta8^k prod rho_ij^v_ij,
rho_ij = (e_j - e_i)^(1/4) (positive for the real configuration).
Usage: python -I derive.py SEED SBITS OUT.json   (SBITS: 6 bits s11 s12 s13 s22 s23 s33)
"""
import cmath
import itertools
import json
import math
import random
import sys

from mpmath import mpf
import thetalib as TL

PAIRS = [(i, j) for i in range(8) for j in range(i + 1, 8)]
PIDX = {pr: k for k, pr in enumerate(PAIRS)}
FOURSETS = [A for A in itertools.combinations(range(8), 4) if 0 in A]  # 35 partitions


def thomae_mono(A):
    Ac = [i for i in range(8) if i not in A]
    v = [0] * 28
    for grp in (A, Ac):
        for i, j in itertools.combinations(sorted(grp), 2):
            v[PIDX[(i, j)]] = 1
    return v


def match(th, detA, es, tag):
    """Match nonvanishing even thetas to partitions via Thomae. Returns kappa,
    dict char -> (A, eps exponent k), vanishing char."""
    mx = max(abs(v) for v in th.values())
    vanish = [c for c in TL.EVEN if abs(th[c]) / mx < 1e-9]
    assert len(vanish) == 1, (tag, len(vanish))
    nz = [c for c in TL.EVEN if c not in vanish]
    T = {A: math.prod(float(es[j] - es[i]) for (i, j), b in zip(PAIRS, thomae_mono(A)) if b)
         for A in FOURSETS}
    vals = {c: th[c] ** 4 / detA ** 2 for c in nz}
    c0 = nz[0]
    best = None
    for A0 in FOURSETS:
        kap = vals[c0] / T[A0]
        asg = {}
        ok = True
        used = set()
        for c in nz:
            cands = [A for A in FOURSETS if abs(abs(vals[c] / T[A]) / abs(kap) - 1) < 1e-7]
            if len(cands) != 1 or cands[0] in used:
                ok = False
                break
            asg[c] = cands[0]
            used.add(cands[0])
        if ok:
            assert best is None, "ambiguous matching"
            best = (kap, asg)
    assert best is not None, "no Thomae matching " + tag
    return best[0], best[1], vanish[0]


def main(seed, sbits, outpath):
    rng = random.Random(seed)
    es = [mpf(3 * i) + mpf(rng.uniform(0.2, 1.8)) for i in range(8)]
    asym, posdef, signs, tau = None, None, None, None
    asym, posdef, signs, tau = TL.theta_arm.period_matrix(es, 5)
    # recompute A (a-periods) as theta_arm does, to get det(A)
    from mpmath.calculus.quadrature import GaussLegendre
    from mpmath import mp
    gl = GaussLegendre(mp)
    nodes = gl.calc_nodes(5, mp.prec)
    import mpmath
    Am = mpmath.matrix(3, 3)
    for i in range(3):
        lo, hi = es[2 * i], es[2 * i + 1]
        va = TL.theta_arm.path_integral(TL.theta_arm.rect_pieces_a(lo - mpf("0.5"), hi + mpf("0.5"), mpf("0.5")), es, nodes)
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
    kap0, asg0, van0 = match(th0, detA, es, "tau")
    kapS, asgS, vanS = match(thS, detA, es, "tau+S")
    rep = {"seed": seed, "S": sbits, "tau_asym": float(asym), "kappa_tau": [kap0.real, kap0.imag],
           "kappa_tauS": [kapS.real, kapS.imag]}
    # kappa: use kap0 (should be equal up to sign)
    kap = kap0
    k4 = kap ** 0.25
    d2 = cmath.sqrt(detA)
    rho = [float(es[j] - es[i]) ** 0.25 for (i, j) in PAIRS]

    def eps_of(th, asg):
        out = {}
        worst = 0
        for c, A in asg.items():
            mono = thomae_mono(A)
            m = math.prod(r for r, b in zip(rho, mono) if b)
            e = th[c] / (k4 * d2 * m)
            ang = cmath.phase(e) / (2 * math.pi) * 8
            k = round(ang) % 8
            worst = max(worst, abs(abs(e) - 1), abs(ang - round(ang)))
            out[c] = (A, k)
        return out, worst
    eps0, w0 = eps_of(th0, asg0)
    epsS, wS = eps_of(thS, asgS)
    rep["eps_dev_tau"] = w0
    rep["eps_dev_tauS"] = wS
    # 2-torsion <-> subsets via c - eta_U at basis tau
    def vec(c):
        return tuple(c[0]) + tuple(c[1])
    eta = vec(van0)
    xmap = {}
    for c, A in asg0.items():
        x = tuple((u + w) % 2 for u, w in zip(vec(c), eta))
        xmap[x] = frozenset(A)
    # check linearity on 4-sets: x_A + x_A' = x_{A^A'} when |A^A'| mod complement is 4
    def norm(Sset):
        Sset = frozenset(Sset)
        comp = frozenset(range(8)) - Sset
        return min(Sset, comp, key=lambda s: (len(s), sorted(s)))
    full = {}
    for x, A in xmap.items():
        full[x] = norm(A)
    changed = True
    while changed:
        changed = False
        items = list(full.items())
        for x1, A1 in items:
            for x2, A2 in items:
                x = tuple((u + w) % 2 for u, w in zip(x1, x2))
                s = norm(set(A1) ^ set(A2))
                if x in full:
                    assert full[x] == s, "nonlinear subset map"
                else:
                    full[x] = s
                    changed = True
    assert len(full) == 64
    rep["subset_map_linear"] = True
    # L_S elements: (m, S m)
    L = []
    for m in TL.BITS3:
        if m == (0, 0, 0):
            continue
        Sm = tuple(sum(S[i][j] * m[j] for j in range(3)) % 2 for i in range(3))
        L.append(sorted(full[tuple(m) + Sm]))
    kind = "affine" if all(len(s) == 4 for s in L) else "pair"
    rep["L_subsets"] = L
    rep["kind"] = kind
    # formula for B products and check against direct thetas of B
    mxB = max(abs(v) for v in thB.values())
    vanB = [c for c in TL.EVEN if abs(thB[c]) / mxB < 1e-9]
    rep["B_vanishing_even"] = len(vanB)
    groups = {}
    for (a, b) in TL.EVEN:
        groups.setdefault(a, []).append(b)
    pairs = []
    for a, bs in groups.items():
        for t in range(0, len(bs), 2):
            pairs.append((a, bs[t], bs[t + 1]))
    assert len(pairs) == 18
    formula = []
    worst = 0
    for (a, b1, b2) in pairs:
        terms = []
        direct = thB[(a, b1)] * thB[(a, b2)]
        acc = 0j
        for beta in TL.BITS3:
            c1 = reduce_char = TL.reduce_char(tuple(x + y for x, y in zip(a, beta)), tuple(x + y for x, y in zip(b1, b2)))
            c2 = TL.reduce_char(beta, tuple(x - y for x, y in zip(b1, b2)))
            A1, G1, s1 = c1
            A2, G2, s2 = c2
            if (A1, G1) not in epsS or (A2, G2) not in epsS:
                acc += s1 * s2 * thS[(A1, G1)] * thS[(A2, G2)]  # should be ~0
                continue
            P1, k1 = epsS[(A1, G1)]
            P2, k2 = epsS[(A2, G2)]
            k = (k1 + k2 + (0 if s1 * s2 == 1 else 4)) % 8
            v = [x + y for x, y in zip(thomae_mono(P1), thomae_mono(P2))]
            terms.append((k, v))
            acc += s1 * s2 * thS[(A1, G1)] * thS[(A2, G2)]
        alg = sum(cmath.exp(2j * math.pi * k / 8) * math.prod(r ** e for r, e in zip(rho, v)) for k, v in terms)
        alg *= k4 ** 2 * detA
        worst = max(worst, abs(acc - direct) / abs(direct) if direct else abs(acc), abs(alg - direct) / abs(direct) if direct else abs(alg))
        formula.append({"a": a, "b1": b1, "b2": b2, "terms": terms})
    rep["bilinear_and_algebraic_max_reldev"] = worst
    # S value and invariance check at numeric config
    def Sval(rh):
        tot = 1
        for fp in formula:
            tot *= sum(cmath.exp(2j * math.pi * k / 8) * math.prod(r ** e for r, e in zip(rh, v)) for k, v in fp["terms"])
        return tot
    s0 = Sval(rho)
    rep["S_value"] = [s0.real, s0.imag]
    inv = []
    r2 = random.Random(99)
    for _ in range(5):
        rh = [r * (1j ** r2.randrange(4)) for r in rho]
        s1 = Sval(rh)
        inv.append(abs(s1 - s0) / abs(s0))
    rep["S_invariance_reldev_5_random_root_choices"] = inv
    rep["S_phase_over_2pi_times_8"] = cmath.phase(s0) / (2 * math.pi) * 8
    out = {"report": rep, "formula": formula, "pairs": PAIRS}
    json.dump(out, open(outpath, "w"), indent=0)
    print(json.dumps(rep, indent=1))


if __name__ == "__main__":
    main(int(sys.argv[1]), [int(c) for c in sys.argv[2]], sys.argv[3])
