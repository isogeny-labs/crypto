#!/usr/bin/env python3
"""Separate schoolbook check for IMPLEMENTATION_SPEC.md's mul5 algebra.

This validates a prospective formula only. It does not call the Rust
implementation, measure performance, or establish a frontier result.
"""

from __future__ import annotations

import argparse
import itertools
import random
from collections.abc import Iterable


Element = tuple[int, int, int]


def div2(value: int, q: int) -> int:
    """Return value / 2 in F_q for canonical value and odd q."""
    return (value if value % 2 == 0 else value + q) // 2


def div3(value: int, q: int) -> int:
    """Return value / 3 in F_q for canonical value and gcd(q, 3) = 1."""
    for multiple in range(3):
        lifted = value + multiple * q
        if lifted % 3 == 0:
            return lifted // 3
    raise AssertionError("one of value, value+q, value+2q must be divisible by 3")


def eval5_mul(left: Element, right: Element, q: int, c: int) -> Element:
    """Five-product evaluation/interpolation followed by z^3 = c reduction."""
    a0, a1, a2 = left
    b0, b1, b2 = right
    v0 = a0 * b0 % q
    v1 = (a0 + a1 + a2) % q * ((b0 + b1 + b2) % q) % q
    vm1 = (a0 - a1 + a2) % q * ((b0 - b1 + b2) % q) % q
    v2 = (a0 + 2 * a1 + 4 * a2) % q * ((b0 + 2 * b1 + 4 * b2) % q) % q
    vinf = a2 * b2 % q

    d0 = v0
    d4 = vinf
    even = div2((v1 + vm1) % q, q)
    d2 = (even - d0 - d4) % q
    odd = div2((v1 - vm1) % q, q)
    at2 = div2((v2 - d0 - 4 * d2 - 16 * d4) % q, q)
    d3 = div3((at2 - odd) % q, q)
    d1 = (odd - d3) % q

    return ((d0 + c * d3) % q, (d1 + c * d4) % q, d2)


def schoolbook_mul(left: Element, right: Element, q: int, c: int) -> Element:
    """Separate nine-product convolution oracle followed by z^3 = c."""
    coefficients = [0] * 5
    for i, lhs in enumerate(left):
        for j, rhs in enumerate(right):
            coefficients[i + j] = (coefficients[i + j] + lhs * rhs) % q
    return (
        (coefficients[0] + c * coefficients[3]) % q,
        (coefficients[1] + c * coefficients[4]) % q,
        coefficients[2],
    )


def elements(q: int) -> Iterable[Element]:
    return itertools.product(range(q), repeat=3)


def check_pair(left: Element, right: Element, q: int, c: int) -> None:
    candidate = eval5_mul(left, right, q, c)
    reference = schoolbook_mul(left, right, q, c)
    if candidate != reference:
        raise AssertionError(
            f"q={q} c={c} left={left} right={right} "
            f"candidate={candidate} reference={reference}"
        )


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--random-pairs", type=int, default=10_000)
    args = parser.parse_args()
    if args.random_pairs < 1:
        parser.error("--random-pairs must be positive")

    exhaustive_pairs = 0
    for q in (5, 7):
        values = list(elements(q))
        for c in range(1, q):
            for left in values:
                for right in values:
                    check_pair(left, right, q, c)
                    exhaustive_pairs += 1

    rng = random.Random(0xECBE_223)
    random_cells = 0
    random_pairs = 0
    for q in (11, 13, 223, 4_051, 65_537, 2**31 - 1):
        assert q % 2 != 0 and q % 3 != 0
        for c in (1, 2, min(16, q - 1)):
            random_cells += 1
            for _ in range(args.random_pairs):
                left = tuple(rng.randrange(q) for _ in range(3))
                right = tuple(rng.randrange(q) for _ in range(3))
                check_pair(left, right, q, c)
                random_pairs += 1

    print(
        "mul5 formula PASS: "
        f"{exhaustive_pairs} exhaustive pairs over q=5,7/all nonzero c; "
        f"{random_pairs} deterministic random pairs over {random_cells} larger (q,c) cells"
    )


if __name__ == "__main__":
    main()
