#!/usr/bin/env python3
"""Kernel analysis of dumped quartic Diem attempt cores (diagnosis only)."""
import json
import sys
from collections import Counter

import numpy as np


def kernel(A, r):
    """Basis of {x : A x = 0} over F_r (exact Python integers)."""
    A = [[int(x) % r for x in row] for row in A]
    m, n = len(A), len(A[0])
    piv = []
    row = 0
    for c in range(n):
        p = next((i for i in range(row, m) if A[i][c]), None)
        if p is None:
            continue
        A[row], A[p] = A[p], A[row]
        inv = pow(A[row][c], r - 2, r)
        A[row] = [x * inv % r for x in A[row]]
        for i in range(m):
            if i != row and A[i][c]:
                f = A[i][c]
                A[i] = [(a - f * b) % r for a, b in zip(A[i], A[row])]
        piv.append(c)
        row += 1
        if row == m:
            break
    A = np.array(A, dtype=object)
    free = [c for c in range(n) if c not in set(piv)]
    basis = []
    for f in free:
        x = np.zeros(n, dtype=object)
        x[f] = 1
        for i, c in enumerate(piv):
            x[c] = (-A[i, f]) % r
        basis.append(x)
    return basis


def main():
    for line in open(sys.argv[1]):
        d = json.loads(line)
        r, cc, ct = d["r"], d["cc"], d["ct"]
        A = np.zeros((len(d["rows"]), cc), dtype=object)
        for i, (ents, _) in enumerate(d["rows"]):
            for c, v in ents:
                A[i, c] = v
        w = (A != 0).sum(axis=0)
        K = kernel(A, r)
        tk = [k for k in K if k[ct] != 0]
        out = {"q": d["q"], "seed": d["seed"], "attempt": d["attempt"], "rows": len(d["rows"]),
               "cc": cc, "kdim": len(K), "k_with_t": len(tk)}
        if tk:
            # combine to a kernel vector with t != 0 and smallest support
            best = min(tk, key=lambda k: int((k != 0).sum()))
            sup = [int(c) for c in np.nonzero(best)[0]]
            out["support"] = len(sup)
            out["kappa_in_support"] = (cc - 2) in sup
            out["support_weights"] = sorted(int(w[c]) for c in sup if c != ct)[:20]
            rows_touch = np.nonzero((A[:, sup] != 0).any(axis=1))[0]
            out["rows_touching"] = len(rows_touch)
            out["rows_touching_with_t"] = int((A[rows_touch, ct] != 0).sum())
            out["t_weight"] = int(w[ct])
            out["w_hist_all"] = dict(sorted(Counter(int(x) for x in w).items())[:6])
        print(json.dumps(out))


if __name__ == "__main__":
    main()
