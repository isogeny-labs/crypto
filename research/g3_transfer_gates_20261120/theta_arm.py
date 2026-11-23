#!/usr/bin/env python3
"""Lead #3, numerical arm T (PROTOCOL.md): type of the codomain J/L of the
(2,2,2)-isogenies with kernel an affine-type or pair-type Lagrangian L of J[2], for
random genus-3 hyperelliptic curves over C with real branch points.

Usage: venv/bin/python -I theta_arm.py OUTDIR   (needs mpmath)
"""
import cmath
import itertools
import json
import math
import os
import random
import sys

import mpmath
from mpmath import mp, mpf, mpc
from mpmath.calculus.quadrature import GaussLegendre

mp.dps = 30


def Y(x, es):
    """Product of principal square roots: analytic off the cuts [e1,e2],...,[e7,e8]."""
    r = mpc(1)
    for e in es:
        r *= mpmath.sqrt(x - e)
    return r


def seg_integral(z0, z1, es, sign, nodes, maxlen=mpf("0.4")):
    """integral of (1, x, x^2) dx / (sign*Y) along the straight segment z0 -> z1."""
    L = abs(z1 - z0)
    nseg = max(1, int(mpmath.ceil(L / maxlen)))
    out = [mpc(0), mpc(0), mpc(0)]
    for s in range(nseg):
        a = z0 + (z1 - z0) * s / nseg
        b = z0 + (z1 - z0) * (s + 1) / nseg
        half = (b - a) / 2
        mid = (a + b) / 2
        for t, w in nodes:
            x = mid + half * t
            g = w * half / (sign * Y(x, es))
            out[0] += g
            out[1] += g * x
            out[2] += g * x * x
    return out


def path_integral(pieces, es, nodes):
    tot = [mpc(0)] * 3
    for z0, z1, sign in pieces:
        v = seg_integral(z0, z1, es, sign, nodes)
        tot = [tot[i] + v[i] for i in range(3)]
    return tot


def rect_pieces_a(x0, x1, h):
    # counterclockwise rectangle, sheet +Y (encloses one whole cut, crosses none)
    c = [mpc(x0, -h), mpc(x1, -h), mpc(x1, h), mpc(x0, h)]
    return [(c[i], c[(i + 1) % 4], 1) for i in range(4)]


def rect_pieces_b(xl, xr, h):
    # counterclockwise; crosses cut 4 at xr (sheet change) and cut i at xl
    return [
        (mpc(xl, -h), mpc(xr, -h), 1),
        (mpc(xr, -h), mpc(xr, 0), 1),
        (mpc(xr, 0), mpc(xr, h), -1),
        (mpc(xr, h), mpc(xl, h), -1),
        (mpc(xl, h), mpc(xl, 0), -1),
        (mpc(xl, 0), mpc(xl, -h), 1),
    ]


def period_matrix(es, degree):
    gl = GaussLegendre(mp)
    nodes = gl.calc_nodes(degree, mp.prec)
    e = es
    A = mpmath.matrix(3, 3)
    B = mpmath.matrix(3, 3)
    m = mpf("0.5")
    for i in range(3):
        lo, hi = e[2 * i], e[2 * i + 1]
        va = path_integral(rect_pieces_a(lo - m, hi + m, mpf("0.5")), es, nodes)
        xl = (lo + hi) / 2
        xr = e[6] + (e[7] - e[6]) * (mpf("0.7") - mpf("0.2") * i)
        hb = mpf("2.0") - mpf("0.5") * i
        vb = path_integral(rect_pieces_b(xl, xr, hb), es, nodes)
        for k in range(3):
            A[k, i] = va[k]
            B[k, i] = vb[k]
    best = None
    for signs in itertools.product((1, -1), repeat=3):
        Bs = B.copy()
        for i in range(3):
            for k in range(3):
                Bs[k, i] *= signs[i]
        tau = A ** -1 * Bs
        asym = max(abs(tau[i, j] - tau[j, i]) for i in range(3) for j in range(3))
        im = mpmath.matrix([[tau[i, j].imag for j in range(3)] for i in range(3)])
        try:
            mpmath.cholesky(im)
            posdef = True
        except Exception:
            posdef = False
        cand = (asym, posdef, signs, tau)
        if posdef and (best is None or asym < best[0]):
            best = cand
    return best


EVEN = [(a, b) for a in itertools.product((0, 1), repeat=3) for b in itertools.product((0, 1), repeat=3)
        if sum(x * y for x, y in zip(a, b)) % 2 == 0]


def thetanulls(tau):
    """36 even theta constants theta[a;b](0, tau), double precision."""
    t = [[complex(tau[i][j]) for j in range(3)] for i in range(3)]
    im = [[t[i][j].imag for j in range(3)] for i in range(3)]
    # smallest eigenvalue lower bound via Gershgorin-free approach: power iteration on inverse is
    # overkill; use min over unit-vector sampling of the quadratic form (conservative radius).
    lam = min(sum(v[i] * im[i][j] * v[j] for i in range(3) for j in range(3))
              for v in _unit_vectors())
    R = int(math.ceil(math.sqrt(16.0 / max(lam, 1e-3)))) + 1
    acc = {}
    for a in itertools.product((0, 1), repeat=3):
        sums = {b: 0j for b in itertools.product((0, 1), repeat=3)}
        for n in itertools.product(range(-R, R + 1), repeat=3):
            v = [n[i] + a[i] / 2 for i in range(3)]
            Q = sum(v[i] * t[i][j] * v[j] for i in range(3) for j in range(3))
            E = cmath.exp(1j * math.pi * Q)
            for b in sums:
                if (n[0] * b[0] + n[1] * b[1] + n[2] * b[2]) % 2:
                    sums[b] -= E
                else:
                    sums[b] += E
        for b, s in sums.items():
            ab = sum(x * y for x, y in zip(a, b))
            if ab % 2 == 0:
                acc[(a, b)] = (1j ** ab) * s
    return acc, R, lam


def _unit_vectors():
    vs = []
    for th in range(0, 180, 6):
        for ph in range(0, 360, 6):
            x, y = math.radians(th), math.radians(ph)
            vs.append((math.sin(x) * math.cos(y), math.sin(x) * math.sin(y), math.cos(x)))
    return vs


def classify(th):
    mx = max(abs(v) for v in th.values())
    rel = sorted((abs(v) / mx, k) for k, v in th.items())
    small = [k for r, k in rel if r < 1e-12]
    mid = [k for r, k in rel if 1e-12 <= r <= 1e-6]
    if mid:
        typ = "undetermined"
    elif len(small) == 0:
        typ = "plane_quartic"
    elif len(small) == 1:
        typ = "hyperelliptic"
    else:
        typ = "decomposable_or_other"
    return typ, small, rel[0][0], rel[1][0]


def main(outdir):
    res = {"curves": []}
    for seed in (1, 2, 3):
        rng = random.Random(seed)
        es = [mpf(3 * i) + mpf(rng.uniform(0.2, 1.8)) for i in range(8)]
        out = {"seed": seed, "branch_points": [float(e) for e in es]}
        pm = {}
        for degree in (4, 5):
            asym, posdef, signs, tau = period_matrix(es, degree)
            pm[degree] = (asym, signs, tau)
        asym, signs, tau = pm[5]
        dtau = max(abs(pm[5][2][i, j] - pm[4][2][i, j]) for i in range(3) for j in range(3))
        out["tau_asym"] = float(asym)
        out["tau_quadrature_diff_24_vs_48_nodes"] = float(dtau)
        out["b_signs"] = signs
        T = [[(tau[i, j] + tau[j, i]) / 2 for j in range(3)] for i in range(3)]
        out["tau"] = [[[float(T[i][j].real), float(T[i][j].imag)] for j in range(3)] for i in range(3)]
        th, R, lam = thetanulls(T)
        typ, small, r0, r1 = classify(th)
        out["J"] = {"type": typ, "vanishing": [list(map(list, k)) for k in small], "min_rel": r0,
                    "second_rel": r1, "R": R}
        gate = (asym < 1e-20 and typ == "hyperelliptic")
        out["gate_T0"] = gate
        if not gate:
            res["curves"].append(out)
            continue
        a0, b0 = small[0]
        # Lagrangians: L_0 and L_S
        lagr = []
        # L_0 = {(0, n)}: affine iff a0 == 0
        lagr.append(("L0", [[2 * T[i][j] for j in range(3)] for i in range(3)],
                     all(x == 0 for x in a0)))
        for bits in itertools.product((0, 1), repeat=6):
            S = [[0] * 3 for _ in range(3)]
            idx = 0
            for i in range(3):
                for j in range(i, 3):
                    S[i][j] = S[j][i] = bits[idx]
                    idx += 1
            # q0 on (m, Sm) is linear: diag(S) + S a0 + b0
            lin = [(S[i][i] + sum(S[i][j] * a0[j] for j in range(3)) + b0[i]) % 2 for i in range(3)]
            affine = all(x == 0 for x in lin)
            Tp = [[(T[i][j] + S[i][j]) / 2 for j in range(3)] for i in range(3)]
            lagr.append(("S" + "".join(map(str, bits)), Tp, affine))
        rows = []
        for name, Tp, affine in lagr:
            thp, Rp, lamp = thetanulls(Tp)
            typp, smallp, r0p, r1p = classify(thp)
            rows.append({"L": name, "kind": "affine" if affine else "pair", "codomain": typp,
                         "min_rel": r0p, "second_rel": r1p, "R": Rp})
            print(seed, name, "affine" if affine else "pair", typp, "%.2e %.2e" % (r0p, r1p), flush=True)
        out["lagrangians"] = rows
        summ = {}
        for r in rows:
            key = "%s->%s" % (r["kind"], r["codomain"])
            summ[key] = summ.get(key, 0) + 1
        out["summary"] = summ
        res["curves"].append(out)
        print(seed, summ, flush=True)
    json.dump(res, open(os.path.join(outdir, "theta_out.json"), "w"), indent=1, default=str)


if __name__ == "__main__":
    main(sys.argv[1])
