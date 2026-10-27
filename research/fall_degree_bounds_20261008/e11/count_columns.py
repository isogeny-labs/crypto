#!/usr/bin/env python3
"""Exact column counts of the E11 restricted degree-4 closure, by policy.

Counts are combinatorial (no system is built): the number of degree-4
Boolean monomials allowed by window policy W(k), and the full degree-<=3
space, for a chained cell m:n:ell.  Block layout as
`build_decomposition_system`: m summand blocks of ell bits, then m-2
intermediate blocks of n bits; link 0 = {X0, X1, E0}, link i =
{E_{i-1}, X_{i+1}, E_i}, last link = {E_{m-3}, X_{m-1}} (x(R) is a constant).
Usage: count_columns.py m:n:ell [...]
"""
import math
import sys


def layout(m, n, ell):
    sizes = [ell] * m + [n] * (m - 2)
    X = lambda i: i  # noqa: E731
    E = lambda j: m + j  # noqa: E731
    links = [{X(0), X(1), E(0)}]
    links += [{E(i - 1), X(i + 1), E(i)} for i in range(1, m - 2)]
    links += [{E(m - 3), X(m - 1)}]
    return sizes, links


def windows(links, k):
    if k >= len(links):
        return [set().union(*links)]
    return [set().union(*links[j:j + k]) for j in range(len(links) - k + 1)]


def count_w(m, n, ell, k, d=4):
    sizes, links = layout(m, n, ell)
    wins = windows(links, k)
    tot = 0

    def rec(i, left, used, mult):
        nonlocal tot
        if i == len(sizes):
            if left == 0 and any(used <= w for w in wins):
                tot += mult
            return
        for c in range(0, min(left, sizes[i]) + 1):
            rec(i + 1, left - c, used | ({i} if c else set()), mult * math.comb(sizes[i], c))

    rec(0, d, set(), 1)
    return tot


if __name__ == "__main__":
    for cell in sys.argv[1:]:
        m, n, ell = map(int, cell.split(":"))
        N = m * ell + (m - 2) * n
        full4 = math.comb(N, 4)
        low = sum(math.comb(N, d) for d in range(4))
        row = {"cell": cell, "N": N, "low_cols": low, "deg4_full": full4,
               "dense_cols": low + full4}
        for k in (1, 2):
            w = count_w(m, n, ell, k)
            row[f"W{k}_deg4"] = w
            row[f"W{k}_cols"] = low + w
            row[f"W{k}_frac"] = round(w / full4, 3)
        print(row)
