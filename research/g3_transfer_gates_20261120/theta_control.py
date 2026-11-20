#!/usr/bin/env python3
"""Added positive control for arm T (deviation D2, not in the frozen protocol): a curve
y^2 = prod (x^2 - c^2) whose branch points are stable under x -> -x and x -> s^2/x,
so J is isogenous to a product of three elliptic curves (Howe-Leprevost-Poonen type).
Checks that the codomain-side detector reports vanishing theta-nulls when they exist.
Same instrument as theta_arm.py (imported unchanged)."""
import itertools, json, os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import theta_arm as t
from mpmath import mpf

def run(outdir):
    s, a, b = mpf(4), mpf("2.6"), mpf("1.5")
    pos = sorted([s * a, s / a, s * b, s / b])
    es = sorted([-x for x in pos] + pos)
    asym, posdef, signs, tau = t.period_matrix(es, 5)
    T = [[(tau[i, j] + tau[j, i]) / 2 for j in range(3)] for i in range(3)]
    th, R, lam = t.thetanulls(T)
    typ, small, r0, r1 = t.classify(th)
    out = {"branch_points": [float(e) for e in es], "tau_asym": float(asym), "J_type": typ,
           "J_vanishing": [list(map(list, k)) for k in small]}
    rows = []
    cand = [("L0", [[2 * T[i][j] for j in range(3)] for i in range(3)])]
    for bits in itertools.product((0, 1), repeat=6):
        S = [[0] * 3 for _ in range(3)]
        idx = 0
        for i in range(3):
            for j in range(i, 3):
                S[i][j] = S[j][i] = bits[idx]; idx += 1
        cand.append(("S" + "".join(map(str, bits)), [[(T[i][j] + S[i][j]) / 2 for j in range(3)] for i in range(3)]))
    for name, Tp in cand:
        thp, Rp, lamp = t.thetanulls(Tp)
        typp, smallp, r0p, r1p = t.classify(thp)
        rows.append({"L": name, "codomain": typp, "n_vanishing": len(smallp), "min_rel": r0p})
    out["codomains"] = rows
    summ = {}
    for r in rows:
        k = "%s(%d)" % (r["codomain"], r["n_vanishing"]); summ[k] = summ.get(k, 0) + 1
    out["summary"] = summ
    print(typ, small, summ)
    json.dump(out, open(os.path.join(outdir, "theta_control_out.json"), "w"), indent=1)

run(sys.argv[1])
