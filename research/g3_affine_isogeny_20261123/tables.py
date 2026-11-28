#!/usr/bin/env python3
"""Step 1-2 over C: derive and check the tables used by the finite-field construction.

(1) Jacobi's derivative formula in genus 3, identified numerically on tau_B (seed 1,
    S = 001000): for every azygetic triple n1 < n2 < n3 of odd characteristics,
        det(grad theta[n1], grad theta[n2], grad theta[n3])(0) = s * pi^3 * prod_{e in E5} theta[e](0)
    with a unique 5-set E5 of even characteristics and a sign s = +-1.
(2) Riemann's quartic theta relations M1 + s2 M2 + s3 M3 = 0, M = prod of theta over a
    coset m + V of a 2-dimensional subspace V (all four characteristics even).
(3) The Riemann quartic built from an Aronhold set of odd-theta gradients has all 28
    gradient lines as bitangents; the Jacobi-formula Aronhold matrix equals the gradient one.
All tables are re-checked on independent period matrices (other seeds, other S).

Usage: python tables.py   (writes formulas/jacobi.json, formulas/riemann_rel.json,
                           data/tables_check.json)
"""
import bisect
import cmath
import itertools
import json
import math
import os
import sys

import numpy as np

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import cnum as C  # noqa: E402


def tau_B(seed, sbits, cache={}):
    if seed not in cache:
        es, T, asym = C.period_matrix_seed(seed)
        cache[seed] = T
    T = cache[seed]
    S = np.zeros((3, 3))
    k = 0
    for i in range(3):
        for j in range(i, 3):
            S[i, j] = S[j, i] = sbits[k]
            k += 1
    return (T + S) / 2


def ck(c):
    return [list(c[0]), list(c[1])]


def tc(x):
    return (tuple(x[0]), tuple(x[1]))


def identify_jacobi(val, grad):
    E = C.EVEN
    pairs = [((c1, c2), val[c1] * val[c2]) for c1, c2 in itertools.combinations(E, 2)]
    logs = sorted((math.log(abs(v)), i) for i, (k, v) in enumerate(pairs))
    lv = [x for x, _ in logs]
    trip = [((a, b, c), val[a] * val[b] * val[c]) for a, b, c in itertools.combinations(E, 3)]

    def find5(r, tol=1e-8):
        out = []
        for k3, v3 in trip:
            t = math.log(abs(r)) - math.log(abs(v3))
            i = bisect.bisect_left(lv, t - tol)
            while i < len(lv) and lv[i] <= t + tol:
                k2, v2 = pairs[logs[i][1]]
                if not set(k2) & set(k3):
                    z = r / (v3 * v2)
                    ph = cmath.phase(z) / (2 * math.pi) * 8
                    if abs(ph - round(ph)) < 1e-6 and abs(abs(z) - 1) < 1e-6:
                        out.append((tuple(sorted(k2 + k3)), round(ph) % 8))
                i += 1
        return sorted(set(out))
    tab = {}
    for tr in itertools.combinations(C.ODD, 3):
        if C.q(C.cadd(*tr)) != 0:
            continue
        D = np.linalg.det(np.array([grad[n] for n in tr])) / np.pi ** 3
        res = find5(D)
        assert len(res) == 1, (tr, res)
        es5, k = res[0]
        assert k in (0, 4)
        tab[tr] = (es5, 1 if k == 0 else -1)
    return tab


def check_jacobi(tab, val, grad):
    worst = 0
    for tr, (es5, s) in tab.items():
        D = np.linalg.det(np.array([grad[n] for n in tr])) / np.pi ** 3
        pred = s * np.prod([val[e] for e in es5])
        worst = max(worst, abs(D - pred) / abs(D))
    return worst


def find_relations(val):
    Z = ((0, 0, 0), (0, 0, 0))
    vecs = [c for c in C.CHARS if c != Z]
    subs = set()
    for v1, v2 in itertools.combinations(vecs, 2):
        subs.add(frozenset([v1, v2, C.cadd(v1, v2)]))
    mx = max(abs(val[c]) for c in C.EVEN)
    rels = []
    for V in sorted(subs, key=lambda s: sorted(s)):
        Vl = [Z] + sorted(V)
        cos = {}
        for m in C.EVEN:
            cs = frozenset(C.cadd(m, v) for v in Vl)
            if all(C.q(c) == 0 for c in cs):
                cos[cs] = np.prod([val[c] / mx for c in cs])
        cl = sorted(cos.items(), key=lambda kv: sorted(kv[0]))
        for (k1, a), (k2, b), (k3, c) in itertools.combinations(cl, 3):
            for s2 in (1, -1, 1j, -1j):
                for s3 in (1, -1, 1j, -1j):
                    if abs(a + s2 * b + s3 * c) < 1e-11 * max(abs(a), abs(b), abs(c)):
                        assert s2 in (1, -1) and s3 in (1, -1)
                        rels.append(([sorted(k1), sorted(k2), sorted(k3)], [int(s2.real), int(s3.real)]))
    return rels


def check_relations(rels, val):
    mx = max(abs(val[c]) for c in C.EVEN)
    worst = 0
    for ks, (s2, s3) in rels:
        m = [np.prod([val[c] / mx for c in k]) for k in ks]
        worst = max(worst, abs(m[0] + s2 * m[1] + s3 * m[2]) / max(abs(x) for x in m))
    return worst


def dval(tab, th, n1, n2, n3):
    trip = (n1, n2, n3)
    srt = tuple(sorted(trip, key=C.ODD.index))
    perm = [srt.index(x) for x in trip]
    sg = 1
    for i in range(3):
        for j in range(i + 1, 3):
            if perm[i] > perm[j]:
                sg = -sg
    es5, s = tab[srt]
    return sg * s * np.prod([th[e] for e in es5])


def jacobi_amatrix(tab, th, ar):
    m = ar
    A = np.zeros((3, 3), dtype=complex)
    for r, k in enumerate((4, 5, 6)):
        A[r, 0] = dval(tab, th, m[k], m[1], m[2]) / dval(tab, th, m[3], m[1], m[2])
        A[r, 1] = dval(tab, th, m[0], m[k], m[2]) / dval(tab, th, m[0], m[3], m[2])
        A[r, 2] = dval(tab, th, m[0], m[1], m[k]) / dval(tab, th, m[0], m[1], m[3])
    return A / A[:, :1]


def quartic_check(val, grad, tab, ar):
    L7 = [grad[m] for m in ar]
    H, A = C.weber_frame(L7)
    Xi, kk, res = C.riemann_xi(A)
    F = C.quartic_eval_coeffs(Xi)
    Hi = np.linalg.inv(H)
    defs = [C.bitangent_defect(F, grad[m] @ Hi) for m in C.ODD]
    rnd = np.random.default_rng(0).normal(size=(28, 3))
    rdefs = [C.bitangent_defect(F, l) for l in rnd]
    Aj = jacobi_amatrix(tab, val, ar)
    return {"xi_residual": float(res), "max_defect_28_gradient_lines": float(max(defs)),
            "min_defect_28_random_lines": float(min(rdefs)),
            "jacobi_vs_gradient_aronhold_matrix": float(np.abs(Aj - A).max())}


def main():
    os.makedirs(os.path.join(HERE, "formulas"), exist_ok=True)
    os.makedirs(os.path.join(HERE, "data"), exist_ok=True)
    val, grad = C.theta_all(tau_B(1, [0, 0, 1, 0, 0, 0]))
    tab = identify_jacobi(val, grad)
    rels = find_relations(val)
    json.dump([[[ck(n) for n in tr], [ck(e) for e in es5], s] for tr, (es5, s) in tab.items()],
              open(os.path.join(HERE, "formulas", "jacobi.json"), "w"))
    json.dump([[[[ck(c) for c in k] for k in ks], ss] for ks, ss in rels],
              open(os.path.join(HERE, "formulas", "riemann_rel.json"), "w"))
    out = {"n_jacobi_triples": len(tab), "n_relations": len(rels), "checks": []}
    sets = C.all_aronhold_sets()
    out["n_aronhold_sets"] = len(sets)
    for seed, sb in [(1, [0, 0, 1, 0, 0, 0]), (2, [0, 0, 1, 0, 0, 0]), (2, [1, 1, 0, 1, 1, 1]),
                     (3, [0, 1, 1, 1, 0, 1]), (4, [1, 0, 1, 1, 1, 0])]:
        v, g = C.theta_all(tau_B(seed, sb))
        mx = max(abs(v[c]) for c in C.EVEN)
        row = {"seed": seed, "S": sb,
               "min_even_over_max": float(min(abs(v[c]) for c in C.EVEN) / mx),
               "max_odd_value_over_max": float(max(abs(v[c]) for c in C.ODD) / mx),
               "jacobi_max_reldev": float(check_jacobi(tab, v, g)),
               "relations_max_resid": float(check_relations(rels, v))}
        qc = [quartic_check(v, g, tab, sets[i]) for i in (0, 97, 287)]
        row["quartic_checks_3_aronhold_sets"] = qc
        out["checks"].append(row)
        print(json.dumps(row))
    json.dump(out, open(os.path.join(HERE, "data", "tables_check.json"), "w"), indent=1)


if __name__ == "__main__":
    main()
