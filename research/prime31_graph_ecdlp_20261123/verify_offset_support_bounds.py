#!/usr/bin/env python3
"""Exact arithmetic replay of the quadratic and linear offset support bounds.

The M31 calculation uses only the proved point count and global split-count
interval. An exhaustive F17 check independently tests the moment and
duplicate-loss identities used by the bound. Standard library only.
"""

from decimal import Decimal, getcontext
from fractions import Fraction
from itertools import product
from math import comb, isqrt


def decimal(value: Fraction) -> str:
    return str(Decimal(value.numerator) / Decimal(value.denominator))


def small_field_check() -> None:
    p = 17
    f = lambda x: (x**3 + 2 * x + 2) % p
    h = lambda x: (4 * x * x + 9 * x + 9) % p
    roots = [
        frozenset(x for x in range(p) if ((h(x) + t) ** 2 - f(x)) % p == 0)
        for t in range(p)
    ]
    affine = sum((y * y - f(x)) % p == 0 for x in range(p) for y in range(p))
    assert affine == 18
    assert all(f(x) != 0 for x in range(p))
    assert sum(map(len, roots)) == affine

    sums = []
    duplicate_losses = []
    for t0, w0, w1 in product(range(p), repeat=3):
        offsets = (t0, t0 + w0, t0 + w1, t0 + w0 + w1)
        fibers = [roots[t % p] for t in offsets]
        total = sum(map(len, fibers))
        sums.append(total)
        duplicate_losses.append(total - len(set().union(*fibers)))

    mean_root = Fraction(affine, p)
    variance_root = Fraction(sum(len(r) ** 2 for r in roots), p) - mean_root**2
    mean_sum = Fraction(sum(sums), len(sums))
    variance_sum = Fraction(sum(s * s for s in sums), len(sums)) - mean_sum**2
    duplicate_mean = Fraction(sum(duplicate_losses), len(duplicate_losses))
    duplicate_pair_bound = comb(4, 2) * Fraction(2 * affine, p * p)
    assert mean_sum == 4 * mean_root
    assert variance_sum == 4 * variance_root
    assert duplicate_mean <= duplicate_pair_bound
    print("F17 exhaustive moment and duplicate checks: PASS")


def m31_bounds() -> None:
    p = 2**31 - 1
    curve_order = 2147446831
    split_lower, split_upper = 89416692, 89540279
    mu = Fraction(curve_order - 1, p)

    # Pairwise-independent indicator variables for full-split offsets.
    low = Fraction(split_lower, p)
    high = Fraction(split_upper, p)
    mean8_upper = 256 * high
    variance8_upper = 256 * high * (1 - low)
    eight_tail = variance8_upper / (variance8_upper + (80 - mean8_upper) ** 2)
    assert eight_tail < Fraction(2124, 10**6)

    # Pairwise-independent root counts, with every duplicate abscissa charged.
    m = 1024
    mean10 = m * mu
    variance10_upper = m * (4 * mu - mu**2)
    duplicate10_upper = comb(m, 2) * Fraction(2 * (curve_order - 1), p * p)
    lower_tail = variance10_upper / (variance10_upper + (mean10 - 676) ** 2)
    failure10_upper = lower_tail + duplicate10_upper
    assert failure10_upper < Fraction(252291, 10**7)
    print("M31 eight-bit full-split tail upper:", decimal(eight_tail))
    print("M31 ten-bit duplicate-loss mean upper:", decimal(duplicate10_upper))
    print("M31 ten-bit support success lower:", decimal(1 - failure10_upper))

    # For the linear graph, the degree-three S3 closure has genus at most
    # three, hence N3 <= (p+1)/6 + sqrt(p) + 5. Round sqrt(p) upward so
    # every following bound is a rational number checked exactly.
    root_ceiling = isqrt(p) + 1
    assert (root_ceiling - 1) ** 2 < p < root_ceiling**2
    triple_upper = Fraction(p + 1, 6) + root_ceiling + 5
    triple_density_upper = triple_upper / p
    cubic_mean8_upper = 256 * triple_density_upper
    cubic_variance8_upper = 256 * triple_density_upper * (1 - triple_density_upper)
    cubic_tail = cubic_variance8_upper / (
        cubic_variance8_upper + (165 - cubic_mean8_upper) ** 2
    )
    assert cubic_tail < Fraction(2371, 10**6)

    cubic_variance10_upper = m * (3 * mu - mu**2)
    cubic_lower_tail = cubic_variance10_upper / (
        cubic_variance10_upper + (mean10 - 676) ** 2
    )
    cubic_success_lower = 1 - cubic_lower_tail - duplicate10_upper
    assert cubic_success_lower > Fraction(98288, 10**5)
    print("M31 linear-graph eight-bit three-root tail upper:", decimal(cubic_tail))
    print("M31 linear-graph ten-bit support success lower:", decimal(cubic_success_lower))


if __name__ == "__main__":
    getcontext().prec = 35
    small_field_check()
    m31_bounds()
