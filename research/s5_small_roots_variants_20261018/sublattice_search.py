#!/usr/bin/env python3
"""How much can a sublattice of the Jochemsz–May basis for S₃ add to the
small-root reach?  Exact Gram determinants, greedy row removal.

W10's ceiling 1/(m·d) covers full-rank triangular lattices only; Boneh–Durfee
geometrically progressive sublattices and Herrmann–May rewritings are
outside it (W10 §7).  This script measures the gain such sublattices can
give at the levels the probes used, on the actual S₃ of a 40-bit a = −3
curve with x_R fixed:

  * build the level-t JM basis  g_{a,j} = x^a · f^j · p^{t−j},
    one row per monomial x^i of f^t: (x^i / l^j)·f^j·p^{t−j} (dim 9, 25, 49 at t = 1, 2, 3);
  * for bounds X = Y = ⌊p^δ⌋ scale the coefficient matrix;
  * the Howgrave-Graham condition for a sublattice L_S of rank r is
        det(L_S)^{1/r} < p^t / √r,  det(L_S) = √det(B_S B_Sᵀ);
  * find the largest δ for which the full lattice passes, then greedily
    remove the row whose removal raises that δ most, until no removal helps.

Every determinant is an exact integer (Bareiss on B_S B_Sᵀ); δ is found by
bisection to 1e-4.  Output: full-rank δ at each level (should reproduce the
probes' 1/18, 1/10, 0.119 trend) and the best greedy sublattice δ with the
rows it dropped.  No LLL is run; this is the determinant bound, which is
what the reach numbers quoted everywhere are.

Run: python3 sublattice_search.py [--levels 1,2,3] [--bits 40]
"""
import argparse
import math
import random
from fractions import Fraction
from itertools import product


def bareiss_det(M):
    """Exact determinant of an integer matrix (fraction-free elimination)."""
    n = len(M)
    A = [row[:] for row in M]
    sign = 1
    prev = 1
    for k in range(n - 1):
        if A[k][k] == 0:
            swap = next((i for i in range(k + 1, n) if A[i][k] != 0), None)
            if swap is None:
                return 0
            A[k], A[swap] = A[swap], A[k]
            sign = -sign
        for i in range(k + 1, n):
            for j in range(k + 1, n):
                A[i][j] = (A[i][j] * A[k][k] - A[i][k] * A[k][j]) // prev
        prev = A[k][k]
    return sign * A[n - 1][n - 1]


def poly_mul(f, g, p):
    h = {}
    for (a1, b1), c1 in f.items():
        for (a2, b2), c2 in g.items():
            k = (a1 + a2, b1 + b2)
            h[k] = (h.get(k, 0) + c1 * c2) % p
    return {k: v for k, v in h.items() if v}


def s3_fixed_xr(a, b, xr, p):
    """S₃(x₁, x₂, x_R) for y² = x³ + a x + b with x_R known, as {(i,j): c}:
    (x₁ − x₂)² x_R² − 2((x₁ + x₂)(x₁x₂ + a) + 2b) x_R + (x₁x₂ − a)² − 4b(x₁ + x₂)."""
    f = {}

    def add(i, j, c):
        f[(i, j)] = (f.get((i, j), 0) + c) % p

    add(2, 0, xr * xr); add(0, 2, xr * xr); add(1, 1, -2 * xr * xr)
    # −2 x_R (x₁²x₂ + x₁x₂² + a x₁ + a x₂ + 2b)
    add(2, 1, -2 * xr); add(1, 2, -2 * xr); add(1, 0, -2 * xr * a); add(0, 1, -2 * xr * a); add(0, 0, -4 * xr * b)
    # (x₁x₂ − a)² = x₁²x₂² − 2a x₁x₂ + a²
    add(2, 2, 1); add(1, 1, -2 * a); add(0, 0, a * a)
    add(1, 0, -4 * b); add(0, 1, -4 * b)
    return {k: v for k, v in f.items() if v}


def jm_basis(f, p, t, d=2):
    """Jochemsz–May basic strategy (their Section 2.2): for every monomial
    x^i of f^t let j be the largest power with x^i / l^j a monomial of f^{t-j}
    (l = x₁^d x₂^d the leading monomial; in the box model j = min_c ⌊i_c/d⌋),
    and take the row (x^i / l^j) · f^j · p^{t−j}.  Each row's leading monomial
    is x^i, so the basis is triangular and the rows partition Mon(f^t)."""
    powers = [{(0, 0): 1}]
    for _ in range(t):
        powers.append(poly_mul(powers[-1], f, p))
    rows = []
    for i in product(range(0, d * t + 1), repeat=2):
        j = min(t, min(i[0] // d, i[1] // d))
        a = (i[0] - d * j, i[1] - d * j)
        g = {}
        for (u, v), c in powers[j].items():
            g[(u + a[0], v + a[1])] = c * p ** (t - j)
        rows.append(((i, j), g))
    cols = sorted({mon for _, g in rows for mon in g})
    assert len(cols) == len(rows), (len(cols), len(rows))
    return rows, cols


def passes(rows_idx, rows, cols, X, p, t):
    """Howgrave-Graham on the sublattice of the chosen rows: det^{1/r} < p^t/√r."""
    r = len(rows_idx)
    B = []
    for ri in rows_idx:
        _, g = rows[ri]
        B.append([g.get(mon, 0) * X ** (mon[0] + mon[1]) for mon in cols])
    G = [[sum(B[i][k] * B[j][k] for k in range(len(cols))) for j in range(r)] for i in range(r)]
    det2 = bareiss_det(G)  # det(L)^2
    if det2 <= 0:
        return False
    # det^{1/r} < p^t / sqrt(r)  <=>  log(det2)/2 < r t log p − (r/2) log r
    return math.log(det2) / 2 < r * t * math.log(p) - (r / 2) * math.log(r)


def max_delta(rows_idx, rows, cols, p, t, lo=0.0, hi=0.5, iters=14):
    for _ in range(iters):
        mid = (lo + hi) / 2
        X = max(2, int(p ** mid))
        if passes(rows_idx, rows, cols, X, p, t):
            lo = mid
        else:
            hi = mid
    return lo


def greedy(rows, cols, p, t):
    idx = list(range(len(rows)))
    best = max_delta(idx, rows, cols, p, t)
    full = best
    dropped = []
    while len(idx) > 2:
        cand = None
        for i in idx:
            trial = [k for k in idx if k != i]
            d = max_delta(trial, rows, cols, p, t)
            if d > best + 1e-4 and (cand is None or d > cand[1]):
                cand = (i, d)
        if cand is None:
            break
        idx.remove(cand[0])
        dropped.append(rows[cand[0]][0])
        best = cand[1]
    return full, best, dropped, len(idx)


if __name__ == "__main__":
    ap = argparse.ArgumentParser()
    ap.add_argument("--levels", default="1,2")
    ap.add_argument("--bits", type=int, default=40)
    ap.add_argument("--seed", type=int, default=20261018)
    args = ap.parse_args()
    rng = random.Random(args.seed)
    # A 40-bit prime (the probes' size class); the curve is y² = x³ − 3x + b.
    p = 1099511627791  # 2^40 + 15, prime
    a = p - 3
    b = rng.randrange(1, p)
    xr = rng.randrange(1, p)
    f = s3_fixed_xr(a, b, xr, p)
    print(f"p = {p} ({args.bits}-bit), S₃ with x_R fixed: {len(f)} monomials, box [0,2]²")
    for t in (int(s) for s in args.levels.split(",")):
        rows, cols = jm_basis(f, p, t)
        full, best, dropped, rank = greedy(rows, cols, p, t)
        print(f"level t = {t}: dim {len(rows)}; full-rank δ = {full:.4f}; "
              f"best greedy sublattice δ = {best:.4f} at rank {rank} "
              f"(dropped {len(dropped)} rows: {dropped[:6]}{'…' if len(dropped) > 6 else ''})")
