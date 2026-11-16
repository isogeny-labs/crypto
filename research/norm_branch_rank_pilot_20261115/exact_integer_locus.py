#!/usr/bin/env python3
"""Exact integer certificate for one squared-Vandermonde 6x6 locus.

The identity is *only* for seven abscissae (0,1,2,3,4,5,6), and all
values of the eighth symbol z. It holds in Z[z], hence by reduction over
any field; it is not a universal eight-variable identity or ECDLP result.

The determinant is evaluated by fraction-free Bareiss; an independent
Leibniz expansion is checked at two points. 19 exact sample points
suffice because each of the three z-dependent columns has degree <= 6
and all other columns are z-independent, so both sides have degree <=18.
"""
from itertools import combinations, permutations
import json
from math import isqrt

LEFT = tuple(combinations(range(4), 2))
RIGHT = tuple(combinations(range(4, 8), 2))
FIXED = (0, 1, 2, 3, 4, 5, 6)
P = 1031
Z = 338


def matrix(z):
    xs = FIXED + (z,)
    out = []
    for a in LEFT:
        row = []
        for b in RIGHT:
            indices = sorted(a + b)
            v = 1
            for i, j in combinations(indices, 2):
                v *= (xs[j] - xs[i]) ** 2
            row.append(v)
        out.append(row)
    return out


def bareiss_determinant(a):
    a = [row[:] for row in a]
    n = len(a)
    sign = 1
    last = 1
    for k in range(n - 1):
        if a[k][k] == 0:
            pivot = next((i for i in range(k + 1, n) if a[i][k] != 0), None)
            if pivot is None:
                return 0
            a[k], a[pivot] = a[pivot], a[k]
            sign = -sign
        pivot = a[k][k]
        for i in range(k + 1, n):
            for j in range(k + 1, n):
                numerator = a[i][j] * pivot - a[i][k] * a[k][j]
                quotient, rem = divmod(numerator, last)
                assert rem == 0
                a[i][j] = quotient
            a[i][k] = 0
        last = pivot
    return sign * a[-1][-1]


def leibniz_determinant(a):
    out = 0
    for perm in permutations(range(len(a))):
        inversions = sum(perm[i] > perm[j] for i in range(len(a)) for j in range(i + 1, len(a)))
        prod = 1
        for i, j in enumerate(perm):
            prod *= a[i][j]
        out += (-1 if inversions % 2 else 1) * prod
    return out


def cubic(z):
    return 11879 * z**3 - 58033 * z**2 + 78934 * z - 25880


def proposed_determinant(z):
    # The minus sign matters: det K is negative an integer square.
    t = 41472 * (z - 4)**2 * (z - 5)**2 * (z - 6)**2 * cubic(z)
    return -t*t


def rank_mod(a, p):
    a = [[t % p for t in row] for row in a]
    rank = 0
    for col in range(len(a[0])):
        pivot = next((i for i in range(rank, len(a)) if a[i][col]), None)
        if pivot is None:
            continue
        a[rank], a[pivot] = a[pivot], a[rank]
        inv = pow(a[rank][col], -1, p)
        a[rank] = [x * inv % p for x in a[rank]]
        for i in range(len(a)):
            if i != rank:
                f = a[i][col]
                a[i] = [(x - f*y) % p for x, y in zip(a[i], a[rank])]
        rank += 1
    return rank


def main():
    # Argument independent of any particular prime: equality at 19 integers
    # between polynomials of degree <=18 proves equality in Q[z] / Z[z].
    for z in range(19):
        lhs = bareiss_determinant(matrix(z))
        rhs = proposed_determinant(z)
        assert lhs == rhs, (z, lhs, rhs)
    for z in (7, 20):
        assert leibniz_determinant(matrix(z)) == bareiss_determinant(matrix(z))
    assert cubic(Z) % P == 0
    assert Z not in FIXED
    assert all(P % d for d in range(2, isqrt(P) + 1))  # toy field is prime
    a = matrix(Z)
    assert all(bareiss_determinant([[a[i][j] for j in cols] for i in rows]) % P == 0
               for rows in combinations(range(6), 5) for cols in combinations(range(6), 5))
    assert bareiss_determinant([r[:4] for r in a[:4]]) % P == 188
    assert rank_mod(a, P) == 4
    assert rank_mod(matrix(7), P) == 6
    print(json.dumps({'identity':'det K(z) = -(41472 (z-4)^2 (z-5)^2 (z-6)^2 C(z))^2',
        'C(z)':'11879z^3-58033z^2+78934z-25880',
        'integer_evaluations':19,'degree_bound':18,
        'independent_leibniz_checks':2,'modulus':P,'noncollision_root':Z,
        'all_5x5_minors_zero':36,'leading_4x4_minor_mod_p':188,'rank_at_root':4,'rank_at_z7':6,
        'status':'exact one-parameter integer polynomial identity; not universal eight-variable theorem'},sort_keys=True,indent=2))


if __name__ == '__main__':
    main()
