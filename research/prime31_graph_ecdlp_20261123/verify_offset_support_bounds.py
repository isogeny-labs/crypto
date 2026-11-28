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
    centered = [Fraction(len(r), 1) - mean_root for r in roots]
    fourth_sum = Fraction(sum((Fraction(s) - mean_sum) ** 4 for s in sums), len(sums))
    fourth_root = sum(x**4 for x in centered) / p
    parallelogram = sum(
        centered[a] * centered[b] * centered[c]
        * centered[(a + b - c) % p]
        for a, b, c in product(range(p), repeat=3)
    ) / p**3
    dependent = 3 * (6**2 - 2 * 4**2 + 2**2)
    assert dependent == 24
    assert fourth_sum == 4 * fourth_root + 3 * 4 * 3 * variance_root**2 + dependent * parallelogram
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


def pair_index(permutation: tuple[int, ...]) -> int:
    """Ramification index of a permutation acting on ordered distinct pairs."""
    n = len(permutation)
    pairs = [(i, j) for i in range(n) for j in range(n) if i != j]
    unseen = set(pairs)
    cycles = 0
    while unseen:
        i, j = next(iter(unseen))
        cycles += 1
        while (i, j) in unseen:
            unseen.remove((i, j))
            i, j = permutation[i], permutation[j]
    return len(pairs) - cycles


def fourth_moment_bounds() -> None:
    p = 2**31 - 1
    affine = 2147446830
    mu = Fraction(affine, p)
    root_ceiling = isqrt(p) + 1
    assert [pair_index(x) for x in ((1, 0, 2, 3), (1, 0, 3, 2), (1, 2, 0, 3), (1, 2, 3, 0))] == [5, 6, 8, 9]
    assert [pair_index(x) for x in ((1, 0, 2), (1, 2, 0))] == [3, 4]

    for degree, genus, branch_values in ((4, 6, 5), (3, 3, 4)):
        pair_points_upper = (
            p + 1 + 2 * genus * root_ceiling
            + branch_values * degree * (degree - 1)
        )
        variance_upper = Fraction((affine + pair_points_upper) * p - affine**2, p**2)
        assert variance_upper > 0
        for bits in (8, 9, 10):
            labels = 1 << bits
            dependent = 3 * (6**bits - 2 * 4**bits + 2**bits)
            assert dependent >= 0
            fourth_upper = (
                labels * (degree - mu) ** 2 * variance_upper
                + 3 * labels * (labels - 1) * variance_upper**2
                + Fraction(dependent * (degree + 1) ** 2, p) * variance_upper
            )
            if bits < 10:
                capacity_upper = fourth_upper / (677 - labels * mu) ** 4
                thresholds = {(4, 8): Fraction(631064, 10**11),
                              (3, 8): Fraction(626826, 10**11),
                              (4, 9): Fraction(106553, 10**8),
                              (3, 9): Fraction(106181, 10**8)}
                assert capacity_upper < thresholds[(degree, bits)]
                print(f"M31 degree-{degree} {bits}-bit fourth-moment capacity upper:", decimal(capacity_upper))
            else:
                duplicate_upper = Fraction(labels * (labels - 1) * affine, p**2)
                failure_upper = (
                    fourth_upper / (labels * mu - 688) ** 4
                    + duplicate_upper / 13
                )
                success_lower = 1 - failure_upper
                assert success_lower > {(4, 10): Fraction(9997149, 10**7),
                                        (3, 10): Fraction(9997154, 10**7)}[(degree, bits)]
                print(f"M31 degree-{degree} ten-bit fourth-moment support lower:", decimal(success_lower))


if __name__ == "__main__":
    getcontext().prec = 35
    small_field_check()
    m31_bounds()
    fourth_moment_bounds()
