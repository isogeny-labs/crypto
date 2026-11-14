#!/usr/bin/env python3
"""Break-even degree for the PKM tower route (derived, not measured).

Cost model (crypto research/prime_ic_leads_20261110/README.md sections 2-3,
re-derived in PRESET.txt): base B = 2^t points, arity m, N = m*t tower bits,
per-target cost T = B^s.  Total cost < r^(1/2) iff s < m - 3.
A degree-D closure in the tower quotient ring spans at most
M(N, D) = sum_{k<=D} C(N, k) square-free monomials; if the per-target cost is
at least M(N, D)^w (w = 1: read the dense matrix once; w = 2: sparse
elimination), parity needs  w * log2 M(N, D) < (m - 3) * t = (1 - 3/m) * N.

Outputs
  alpha*(m, w) = H^{-1}((1 - 3/m) / w)       asymptotic break-even slope D/N
  Dstar(m, N, w) = max{D : w*log2 M(N,D) < (1-3/m) N}   finite-N break-even
  Dstar_u(m, N, w): the same count with the chain's m-2 free unknowns u
                    (not reduced by the tower) included in the monomial count.
"""
import json
import math
import sys


def H(a):
    if a <= 0 or a >= 1:
        return 0.0
    return -a * math.log2(a) - (1 - a) * math.log2(1 - a)


def Hinv(y):
    """inverse of H on [0, 1/2]"""
    if y >= 1:
        return 0.5
    lo, hi = 0.0, 0.5
    for _ in range(200):
        mid = (lo + hi) / 2
        if H(mid) < y:
            lo = mid
        else:
            hi = mid
    return (lo + hi) / 2


def M(N, D):
    return sum(math.comb(N, k) for k in range(0, min(D, N) + 1))


def M_u(N, k, D):
    # monomials of total degree <= D: square-free in N tower vars, free in k vars
    tot = 0
    for a in range(0, min(D, N) + 1):
        for b in range(0, D - a + 1):
            tot += math.comb(N, a) * (math.comb(k + b - 1, b) if k > 0 else (1 if b == 0 else 0))
    return tot


def dstar(m, N, w, k=0):
    budget = (1 - 3 / m) * N
    best = -1
    for D in range(0, 4 * N + 8):
        cnt = M_u(N, k, D) if k else M(N, D)
        if w * math.log2(cnt) < budget:
            best = D
        else:
            break
    return best


def main():
    out = {"alpha_star": {}, "cells": []}
    for m in (4, 5, 6, 7):
        for w in (1, 2):
            out["alpha_star"][f"m{m}_w{w}"] = round(Hinv((1 - 3 / m) / w), 4)
    for m in (4, 5, 6, 7):
        for t in (1, 2, 3, 4):
            N = m * t
            row = {"m": m, "t": t, "N": N, "budget_bits_(m-3)t": (m - 3) * t}
            for w in (1, 2):
                row[f"Dstar_w{w}"] = dstar(m, N, w)
                row[f"Dstar_u_w{w}"] = dstar(m, N, w, k=m - 2)
                row[f"alphaN_w{w}"] = round(Hinv((1 - 3 / m) / w) * N, 2)
            out["cells"].append(row)
    json.dump(out, sys.stdout, indent=1)
    print()


if __name__ == "__main__":
    main()
