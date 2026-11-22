#!/usr/bin/env python3
"""Exact linear algebra mod p (p < 2^21) with numpy; matmuls go through float64
BLAS, exact because every partial sum stays below 2^53 (entries < 2^21,
inner dimension <= 2^11)."""
import numpy as np


def mmod(X, Y, p):
    """(X @ Y) mod p, exact for p < 2^21 and inner dim <= 2048 (chunked otherwise)."""
    k = X.shape[1]
    out = np.zeros((X.shape[0], Y.shape[1]), dtype=np.int64)
    step = 2048
    for s in range(0, k, step):
        Z = X[:, s:s + step].astype(np.float64) @ Y[s:s + step].astype(np.float64)
        out = (out + np.rint(Z).astype(np.int64) % p) % p
    return out


def rref(M, p, order=None, counter=None):
    """Reduced row echelon form of M (int64, entries in [0,p)) with pivots chosen
    in column order `order` (default natural). Returns (R, pivots) with R having
    rank rows. counter: dict accumulating 'muladds' (rows x cols per pivot step)."""
    M = M.copy() % p
    rows, cols = M.shape
    if order is None:
        order = np.arange(cols)
    piv = []
    r = 0
    for c in order:
        if r >= rows:
            break
        nz = np.nonzero(M[r:, c])[0]
        if nz.size == 0:
            continue
        i = r + nz[0]
        if i != r:
            M[[r, i]] = M[[i, r]]
        inv = pow(int(M[r, c]), p - 2, p)
        M[r] = M[r] * inv % p
        f = M[:, c].copy()
        f[r] = 0
        nzr = np.nonzero(f)[0]
        if nzr.size:
            M[nzr] = (M[nzr] - f[nzr, None] * M[r][None, :]) % p
            if counter is not None:
                counter['muladds'] = counter.get('muladds', 0) + int(nzr.size) * cols
        piv.append(int(c))
        r += 1
    return M[:r], piv


def nullspace(M, p):
    """Basis of {x : M x = 0} mod p."""
    R, piv = rref(M, p)
    cols = M.shape[1]
    free = [c for c in range(cols) if c not in set(piv)]
    basis = []
    for fcol in free:
        x = np.zeros(cols, dtype=np.int64)
        x[fcol] = 1
        for i, pc in enumerate(piv):
            x[pc] = (-R[i, fcol]) % p
        basis.append(x)
    return basis
