"""Multigraded Hilbert-series predictor of the MutantXL closure degree (T06).

FROZEN before compare.py was run; compare.py checks FROZEN.sha256.

System shape (src/cryptanalysis/koblitz_groebner.rs, build_decomposition_system;
documented in koblitz_bench.rs and RESEARCH_ECC2K130_DESCENT_DEGREE.md s.1):

  m summand blocks x_1..x_m of ell bits, m-2 intermediate blocks e_1..e_{m-2}
  of n bits.  N = m*ell + (m-2)*n.  Links S3(x1,x2,e1), S3(e_i,x_{i+2},e_{i+1}),
  S3(e_{m-2},x_m,x(R)); each link gives n Boolean equations over F2, so
  (m-1)*n equations.  Every link is multilinear in its blocks (squaring is
  F2-linear): a link with three variable blocks (a,b,c) has top form abc, i.e.
  multidegree e_a+e_b+e_c (cubic, (m-2)*n equations); the closing link has one
  constant argument and top form e_{m-2}+e_m (bilinear, n equations).
  m = 2: single link S3(x1,x2,x(R)), top form e_1+e_2, n equations.
  Grading group: Z^g with g = 2m-2 blocks (m = 2: g = 2), NOT Z^m: the
  intermediate blocks carry their own degree.

Model: associated graded ring of the Boolean quotient, F2[x]/(x_i^2): the
multigraded series of the ambient ring is R(t) = prod_b (1+t_b)^{s_b}; a generic
(semi-regular, Boolean) system with top-form multidegrees delta_j has
  c(t) = R(t) / prod_j (1 + t^{delta_j}).
ALIVE set A = {d : c(d') > 0 for every d' <= d componentwise} (the multigraded
analogue of "all coefficients before the first non-positive one"; univariate
case: A = {0..D_reg-1}).  COVERED set U = union_j (delta_j + N^g): the
multidegrees that receive rows from some generator's top form.  Predicted
closure degree
  D_multi = 1 + max{|d| : d in A and d in U}.
That is the first total degree at which every block that has generator rows has
at least as many independent rows as monomials.  (Design note, fixed before any
draw was compared: with U omitted, A is unbounded because monomials in one
block alone, e.g. x_1^k, are never touched by a top form; the version without
U returns the cap for every cell and is discarded.)  Control arm: the univariate semi-regular D_reg, the first
non-positive coefficient of (1+z)^N / ((1+z^2)^{n}(1+z^3)^{(m-2)n}) (quadratic
count n, cubic count (m-2)n).
"""
from __future__ import annotations

from collections import defaultdict
from itertools import product
from math import comb

DMAX = 14  # cap on total degree; result is ">DMAX" (returned DMAX+1) beyond it


def blocks(m: int, ell: int, n: int) -> list[int]:
    return [ell] * m + [n] * max(m - 2, 0)


def top_forms(m: int, ell: int, n: int) -> dict[tuple, int]:
    """{multidegree (tuple over blocks): number of Boolean equations}."""
    g = m + max(m - 2, 0)
    def e(*idx):
        v = [0] * g
        for i in idx:
            v[i] += 1
        return tuple(v)
    if m == 2:
        return {e(0, 1): n}
    X = lambda i: i            # summand block i (0-based)
    E = lambda i: m + i        # intermediate block i (0-based)
    out: dict[tuple, int] = defaultdict(int)
    out[e(X(0), X(1), E(0))] += n
    for i in range(m - 3):
        out[e(E(i), X(i + 2), E(i + 1))] += n
    out[e(E(m - 3), X(m - 1))] += n   # closing link: x(R) constant
    return dict(out)


def _series_inv_one_plus(k: int, i: int) -> int:
    """Coefficient of u^i in (1+u)^(-k)."""
    return (-1) ** i * comb(k + i - 1, i)


def coefficients(m: int, ell: int, n: int, dmax: int = DMAX) -> dict[tuple, int]:
    """Exact integer coefficients c(d) for d <= s componentwise, |d| <= dmax."""
    s = blocks(m, ell, n)
    g = len(s)
    # R(d)
    cur: dict[tuple, int] = {}
    def rec(i, d, tot, val):
        if i == g:
            cur[tuple(d)] = val
            return
        for k in range(min(s[i], dmax - tot) + 1):
            d.append(k)
            rec(i + 1, d, tot + k, val * comb(s[i], k))
            d.pop()
    rec(0, [], 0, 1)
    for delta, k in top_forms(m, ell, n).items():
        dd = sum(delta)
        new: dict[tuple, int] = defaultdict(int)
        for d, v in cur.items():
            i = 0
            while True:
                nd = tuple(a + i * b for a, b in zip(d, delta))
                if sum(nd) > dmax or any(a > b for a, b in zip(nd, s)):
                    break
                new[nd] += v * _series_inv_one_plus(k, i)
                i += 1
        cur = dict(new)
    return cur


def alive_set(m: int, ell: int, n: int, dmax: int = DMAX) -> set[tuple]:
    c = coefficients(m, ell, n, dmax)
    order = sorted(c, key=sum)
    alive: set[tuple] = set()
    for d in order:
        if c[d] <= 0:
            continue
        ok = True
        for b in range(len(d)):
            if d[b] > 0:
                p = d[:b] + (d[b] - 1,) + d[b + 1:]
                if p not in alive:
                    ok = False
                    break
        if ok:
            alive.add(d)
    return alive


def predict_multi(m: int, ell: int, n: int, dmax: int = DMAX) -> int:
    """Frozen primary prediction D_multi (dmax+1 means '> dmax')."""
    A = alive_set(m, ell, n, dmax)
    deltas = list(top_forms(m, ell, n))
    cov = [d for d in A if any(all(a >= b for a, b in zip(d, dl)) for dl in deltas)]
    return (max(sum(d) for d in cov) + 1) if cov else min(sum(dl) for dl in deltas)


def predict_semiregular(m: int, ell: int, n: int, dmax: int = DMAX) -> int:
    """Univariate control: first non-positive coefficient of
    (1+z)^N / ((1+z^2)^n (1+z^3)^((m-2) n)); m = 2 has no cubic factor."""
    N = m * ell + (m - 2) * n
    nq, nc = n, (m - 2) * n
    poly = [comb(N, k) if k <= N else 0 for k in range(dmax + 1)]
    for deg, cnt in ((2, nq), (3, nc)):
        for _ in range(cnt):
            for k in range(deg, dmax + 1):
                poly[k] -= poly[k - deg]
    for k, v in enumerate(poly):
        if v <= 0:
            return k
    return dmax + 1


def fss_bound(m: int, ell: int, n: int):
    """Faugere-Safey El Din-Spaenlehauer (JSC 2011), affine zero-dimensional
    generic bilinear system with n_x + n_y equations in n_x + n_y variables:
    d_reg <= min(n_x, n_y) + 1.  Applies here only to m = 2 (blocks ell, ell, n
    bilinear equations) and only when the square case holds (n = 2*ell); for
    n > 2*ell the overdetermined system's regularity is at most that of a
    square subsystem, so min(ell,ell)+1 = ell+1 is still an upper bound.
    Returns None where no bihomogeneous (1,1) system is present (m >= 3: the
    cubic links have top form of multidegree (1,1,1))."""
    if m == 2 and n >= 2 * ell:
        return ell + 1
    return None


if __name__ == "__main__":
    # univariate control against RESEARCH_ECC2K130_DESCENT_DEGREE.md s.1 (m = 3)
    ref = {(5, 2): 5, (4, 3): 5, (5, 3): 5, (7, 3): 6, (9, 3): 6, (7, 4): 6,
           (8, 4): 6, (11, 4): 7, (13, 4): 7, (10, 5): 7, (11, 5): 7, (13, 5): 7,
           (15, 5): 8}
    bad = [(k, v, predict_semiregular(3, k[1], k[0])) for k, v in ref.items()
           if predict_semiregular(3, k[1], k[0]) != v]
    print("semireg control mismatches vs committed table:", bad)
    for cell in [(3, 10, 5), (3, 19, 7), (4, 12, 5), (3, 24, 6)]:
        print(cell, predict_multi(*[cell[0], cell[2], cell[1]]), predict_semiregular(cell[0], cell[2], cell[1]))
