#!/usr/bin/env python3
"""Exact certificate: the balanced 6x6 squared-Vandermonde determinant is -a square.

Det K(x) = -[V_L(x)^2 V_R(x)^2 S(x)]^2 for eight formal x variables,
where V_L and V_R are the four-point Vandermonde products and S is an
integer polynomial of total degree 12. Construct S from integer roots on a
4^6 grid, interpolate degree <=3 in six coordinates, and prove the identity
by 7^6 exact determinant comparisons (degree <=6 in each coordinate).

The degree and collision-divisibility arguments are in the accompanying README.
This is finite polynomial identity certification, not an ECDLP solver or PCC proof.
"""
import hashlib
import json
from fractions import Fraction
from itertools import combinations, permutations, product
from math import comb, isqrt, prod

PAIRS = tuple(combinations(range(4), 2))
# x0=0, x1=1; six free x's vary on pairwise disjoint, collision-free grids.
BASE = (0, 1, 10, 20, 30, 40, 50, 60)
MONOMIAL_FROM_VALUES = (
    (Fraction(1), Fraction(0), Fraction(0), Fraction(0)),
    (Fraction(-11, 6), Fraction(3), Fraction(-3, 2), Fraction(1, 3)),
    (Fraction(1), Fraction(-5, 2), Fraction(2), Fraction(-1, 2)),
    (Fraction(-1, 6), Fraction(1, 2), Fraction(-1, 2), Fraction(1, 6)),
)


def bareiss(a):
    a = [row[:] for row in a]
    n = len(a)
    sign, prior = 1, 1
    for k in range(n - 1):
        if not a[k][k]:
            i = next((i for i in range(k + 1, n) if a[i][k]), None)
            if i is None:
                return 0
            a[k], a[i] = a[i], a[k]
            sign = -sign
        pivot = a[k][k]
        for i in range(k + 1, n):
            for j in range(k + 1, n):
                numerator = a[i][j] * pivot - a[i][k] * a[k][j]
                quotient, remainder = divmod(numerator, prior)
                assert not remainder
                a[i][j] = quotient
            a[i][k] = 0
        prior = pivot
    return sign * a[-1][-1]


def original_matrix(x):
    left = PAIRS
    right = tuple((j + 4, k + 4) for j, k in PAIRS)
    return [[prod((x[j] - x[i])**2 for i, j in combinations(sorted(I + J), 2))
             for J in right] for I in left]


def data_from_offsets(t):
    x = BASE[:2] + tuple(BASE[i + 2] + t[i] for i in range(6))
    c = [[(x[j + 4] - x[i])**2 for j in range(4)] for i in range(4)]
    u = [[c[i][j] * c[k][j] for j in range(4)] for i, k in PAIRS]
    n = [[row[j] * row[k] for j, k in PAIRS] for row in u]
    vl2vr2 = prod((x[j] - x[i])**2
                  for group in (range(4), range(4, 8))
                  for i, j in combinations(group, 2))
    quotient, remainder = divmod(bareiss(n), vl2vr2)
    assert remainder == 0, ('nonintegral polynomial value', t)
    return x, quotient, n, vl2vr2


def extend_four(values):
    a, b, c, d = values
    d1 = b - a
    d2 = c - 2*b + a
    d3 = d - 3*c + 3*b - a
    return [a + j*d1 + comb(j, 2)*d2 + comb(j, 3)*d3
            for j in (4, 5, 6)]


def integer_coefficient_receipt(seed):
    """Convert the 4^6 values of H to ordinary monomials with exact Q arithmetic."""
    coeff = seed.copy()
    for axis in range(6):
        updated = {}
        for rest in product(range(4), repeat=5):
            def idx(i):
                tup = list(rest)
                tup.insert(axis, i)
                return tuple(tup)
            vals = [coeff[idx(j)] for j in range(4)]
            for power in range(4):
                updated[idx(power)] = sum(MONOMIAL_FROM_VALUES[power][j]*vals[j]
                                           for j in range(4))
        coeff = updated
    used = [(exps, value) for exps, value in sorted(coeff.items()) if value]
    assert used
    assert all(value.denominator == 1 for _, value in used)
    max_degree = max(sum(exps) for exps, _ in used)
    assert max_degree <= 12, max_degree
    payload = '\n'.join(','.join(map(str, exps)) + ':' + str(value.numerator)
                        for exps, value in used).encode('ascii')
    return {'nonzero_integer_coefficients': len(used),
            'max_total_degree': max_degree,
            'coefficient_sha256': hashlib.sha256(payload).hexdigest()}


def main():
    seeds = {}
    for t in product(range(4), repeat=6):
        _, quotient, _, _ = data_from_offsets(t)
        assert quotient < 0, ('nonnegative quotient', t)
        root = isqrt(-quotient)
        assert root*root == -quotient, ('nonsquare seed', t)
        seeds[t] = root
    receipt = integer_coefficient_receipt(seeds)

    # Polynomial H has degree <=3 separately in each of the six variables.
    # Tensor Newton extrapolation never uses determinant values beyond seed.
    grid = seeds.copy()
    for axis in range(6):
        for others in product(*(range(7) if i < axis else range(4)
                                for i in range(6) if i != axis)):
            def idx(j):
                tup = list(others)
                tup.insert(axis, j)
                return tuple(tup)
            for j, value in zip((4, 5, 6), extend_four([grid[idx(i)] for i in range(4)])):
                grid[idx(j)] = value
    assert len(grid) == 7**6
    for t in product(range(7), repeat=6):
        _, quotient, _, _ = data_from_offsets(t)
        assert quotient == -grid[t]**2, ('polynomial identity mismatch', t)

    # Independently compare the factored cross-product matrix with original K,
    # plus a 720-term Leibniz expansion on two simple, off-grid inputs.
    for t in ((0, 0, 0, 0, 0, 0), (3, 2, 1, 4, 5, 6)):
        x, quotient, n, factor = data_from_offsets(t)
        k = original_matrix(x)
        det_k = bareiss(k)
        assert det_k == factor * bareiss(n)
        assert det_k == -(factor * isqrt(-quotient))**2
        leibniz = 0
        for perm in permutations(range(6)):
            inversions = sum(perm[i] > perm[j] for i in range(6)
                             for j in range(i + 1, 6))
            leibniz += (-1 if inversions % 2 else 1)*prod(k[i][perm[i]] for i in range(6))
        assert leibniz == det_k
    print(json.dumps({'claim':'universal det K = -[Delta_L^2 Delta_R^2 S]^2',
        'fixed_abscissae':[0,1], 'offset_bases':BASE[2:], 'seed_checks':4**6,
        'polynomial_identity_checks':7**6, 'independent_leibniz_checks':2,
        'polynomial_domain':'Z[x0,...,x7]; characteristic-free reduction',
        'proof_method':'degree-six tensor interpolation + integral affine homogenization',
        'status':'producer-local exact computational polynomial certificate; PCC unverified',
        **receipt}, sort_keys=True, indent=2))


if __name__ == '__main__':
    main()
