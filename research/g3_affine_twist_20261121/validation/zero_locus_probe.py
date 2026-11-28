#!/usr/bin/env python3
"""Probe of the S_L = 0 kernels in the control (validator, exploratory).

For control kernels without an F_p-rational trigonal map, count the trigonal maps
g = N/D (deg 3, g(a_k) = g(b_k) for the 4 pairs) over F_{p^2} by exhaustive search
over P^1(F_{p^2}) x P^1(F_{p^2}) (normalisation g(a1)=g(b1)=inf, g(a2)=g(b2)=0), and
record candidates that satisfy the four pair conditions but fail deg 3 / coprimality.
Usage: python3 zero_locus_probe.py STUDY_DIR P NPER SEED
"""
import json
import os
import random
import sys


def main(study, p, nper, seed):
    rng = random.Random(seed)
    # F_{p^2} = F_p[s]/(s^2 - n)
    n = next(x for x in range(2, p) if pow(x, (p - 1) // 2, p) == p - 1)

    def mul(a, b):
        return ((a[0] * b[0] + n * a[1] * b[1]) % p, (a[0] * b[1] + a[1] * b[0]) % p)

    def sub(a, b):
        return ((a[0] - b[0]) % p, (a[1] - b[1]) % p)

    def add(a, b):
        return ((a[0] + b[0]) % p, (a[1] + b[1]) % p)

    elems = [(x, y) for x in range(p) for y in range(p)]
    lin = [((1, 0), e) for e in elems] + [((0, 0), (1, 0))]  # (alpha, beta): alpha x + beta

    def ev_lin(l, x):
        return add(mul(l[0], x), l[1])

    rows = [json.loads(l) for l in open(os.path.join(study, "data", "control.jsonl"))]
    rows = [r for r in rows if r["p"] == p and "smith" not in r and r["n_maps"] == 0]
    zero = [r for r in rows if r["vals"][0] == 0]
    nonzero = [r for r in rows if r["vals"][0] != 0]
    out = {"p": p, "zero": [], "nonzero": []}
    for tag, sel in (("zero", rng.sample(zero, min(nper, len(zero)))),
                     ("nonzero", rng.sample(nonzero, min(nper, len(nonzero))))):
        for r in sel:
            u = r["u"]
            pv = [((u[a] % p, 0), (u[b] % p, 0)) for a, b in r["pairs"]]
            (a1, b1), (a2, b2), (a3, b3), (a4, b4) = pv

            def q2(x):
                return mul(sub(x, a2), sub(x, b2))

            def q1(x):
                return mul(sub(x, a1), sub(x, b1))
            good = 0
            bad = 0
            # precompute per-point values
            pts = [a3, b3, a4, b4]
            Q2 = [q2(x) for x in pts]
            Q1 = [q1(x) for x in pts]
            Lv = {l: [ev_lin(l, x) for x in pts] for l in lin}
            for l1 in lin:
                N = [mul(Q2[i], Lv[l1][i]) for i in range(4)]
                for l2 in lin:
                    if l1[0] == (0, 0) and l2[0] == (0, 0):
                        continue  # degree 2
                    D = [mul(Q1[i], Lv[l2][i]) for i in range(4)]
                    if sub(mul(N[0], D[1]), mul(N[1], D[0])) != (0, 0):
                        continue
                    if sub(mul(N[2], D[3]), mul(N[3], D[2])) != (0, 0):
                        continue
                    # coprimality: roots of l1 not in {a1,b1, root l2}; roots of l2 not in {a2,b2}
                    ok = True
                    for x in (a1, b1):
                        if ev_lin(l1, x) == (0, 0):
                            ok = False
                    for x in (a2, b2):
                        if ev_lin(l2, x) == (0, 0):
                            ok = False
                    if l1[0] != (0, 0) and l2[0] != (0, 0):
                        # same root?  l1 = x + b, l2 = x + d
                        if l1[1] == l2[1]:
                            ok = False
                    if ok:
                        good += 1
                    else:
                        bad += 1
            out[tag].append({"pairs": r["pairs"], "u": u, "val": r["vals"][0],
                             "maps_Fp2": good, "degenerate_candidates": bad})
            print(p, tag, r["pairs"], "val", r["vals"][0], "maps over F_p^2:", good, "rejected:", bad, flush=True)
    return out


if __name__ == "__main__":
    res = main(sys.argv[1], int(sys.argv[2]), int(sys.argv[3]), int(sys.argv[4]))
    json.dump(res, open(os.path.join(os.path.dirname(os.path.abspath(__file__)),
                                     "zero_locus_probe_p%d.json" % res["p"]), "w"), indent=1)
