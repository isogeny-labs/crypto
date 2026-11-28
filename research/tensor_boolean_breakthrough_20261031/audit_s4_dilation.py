#!/usr/bin/env python3
"""Exhaustively audit simultaneous-scale S4 over small prime fields.

For every nonsingular short curve ``y^2 = x^3 + A*x + B`` and every ordered
4-tuple ``t``, construct the coefficient polynomial ``S4(s*t)`` in ``F_p[s]``.
The audit counts identically-zero polynomials and separates the upstream
``TrivL`` tuples (every value has even multiplicity) from nontrivial tuples.

This is bounded experimental evidence, not a proof for untested fields or
higher arity.  The quadratic/quadratic resultant formula matches the frozen
prime-field implementation in ``research/theorem_census_20261109/native``.
"""

from __future__ import annotations

import argparse
import collections
import itertools
import json


def trim(values: list[int], prime: int) -> list[int]:
    values = [value % prime for value in values]
    while len(values) > 1 and values[-1] == 0:
        values.pop()
    return values


def add(left: list[int], right: list[int], prime: int) -> list[int]:
    size = max(len(left), len(right))
    return trim(
        [
            (left[i] if i < len(left) else 0)
            + (right[i] if i < len(right) else 0)
            for i in range(size)
        ],
        prime,
    )


def scale(poly: list[int], scalar: int, prime: int) -> list[int]:
    return trim([scalar * value for value in poly], prime)


def multiply(left: list[int], right: list[int], prime: int) -> list[int]:
    result = [0] * (len(left) + len(right) - 1)
    for i, left_value in enumerate(left):
        for j, right_value in enumerate(right):
            result[i + j] = (result[i + j] + left_value * right_value) % prime
    return trim(result, prime)


def square(poly: list[int], prime: int) -> list[int]:
    return multiply(poly, poly, prime)


def subtract(left: list[int], right: list[int], prime: int) -> list[int]:
    return add(left, scale(right, -1, prime), prime)


def s3_coefficients(
    left_value: int, right_value: int, curve_a: int, curve_b: int, prime: int
) -> tuple[list[int], list[int], list[int]]:
    """Descending coefficients of S3(s*left, s*right, Z)."""
    left = [0, left_value]
    right = [0, right_value]
    product = multiply(left, right, prime)
    qa = square(subtract(left, right, prime), prime)
    qb = scale(
        add(
            multiply(add(left, right, prime), add(product, [curve_a], prime), prime),
            [2 * curve_b],
            prime,
        ),
        -2,
        prime,
    )
    qc = subtract(
        square(subtract(product, [curve_a], prime), prime),
        scale(add(left, right, prime), 4 * curve_b, prime),
        prime,
    )
    return qa, qb, qc


def s4_scale_polynomial(
    values: tuple[int, int, int, int], curve_a: int, curve_b: int, prime: int
) -> list[int]:
    """Coefficient list of S4(s*t0,...,s*t3), lowest degree first."""
    a, b, c = s3_coefficients(values[0], values[1], curve_a, curve_b, prime)
    d, e, f = s3_coefficients(values[2], values[3], curve_a, curve_b, prime)
    terms = [
        multiply(square(a, prime), square(f, prime), prime),
        scale(multiply(multiply(multiply(a, b, prime), e, prime), f, prime), -1, prime),
        scale(multiply(multiply(multiply(a, c, prime), d, prime), f, prime), -2, prime),
        multiply(multiply(a, c, prime), square(e, prime), prime),
        multiply(multiply(square(b, prime), d, prime), f, prime),
        scale(multiply(multiply(multiply(b, c, prime), d, prime), e, prime), -1, prime),
        multiply(square(c, prime), square(d, prime), prime),
    ]
    result = [0]
    for term in terms:
        result = add(result, term, prime)
    return result


def is_trivial(values: tuple[int, ...]) -> bool:
    return all(count % 2 == 0 for count in collections.Counter(values).values())


def audit_prime(prime: int) -> dict[str, object]:
    curves = 0
    zero_polynomials = 0
    nontrivial_zero_polynomials = 0
    degree_counts: collections.Counter[int] = collections.Counter()
    witnesses: list[dict[str, object]] = []
    field = range(prime)
    for curve_a, curve_b in itertools.product(field, repeat=2):
        if (4 * curve_a**3 + 27 * curve_b**2) % prime == 0:
            continue
        curves += 1
        for values in itertools.product(field, repeat=4):
            polynomial = s4_scale_polynomial(values, curve_a, curve_b, prime)
            degree_counts[len(polynomial) - 1] += 1
            if polynomial != [0]:
                continue
            zero_polynomials += 1
            if is_trivial(values):
                continue
            nontrivial_zero_polynomials += 1
            if len(witnesses) < 12:
                witnesses.append({"A": curve_a, "B": curve_b, "tuple": values})
    return {
        "prime": prime,
        "nonsingular_curves": curves,
        "zero_polynomials": zero_polynomials,
        "nontrivial_zero_polynomials": nontrivial_zero_polynomials,
        "degree_counts": dict(sorted(degree_counts.items())),
        "witnesses": witnesses,
    }


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("primes", nargs="*", type=int, default=[5, 7, 11])
    args = parser.parse_args()
    print(json.dumps([audit_prime(prime) for prime in args.primes], sort_keys=True))


if __name__ == "__main__":
    main()
