#!/usr/bin/env python3
"""Exact budget thresholds under Mazur's rational torsion classification."""

import json
import math


CASES = ((2, 1), (3, 2), (4, 3))
BIT_SIZES = (131, 256)
TORSION_CARDINALITY = 16
MAZUR_EXPONENT = 2520


def threshold(r: int, D: int, U: int, exponent: int) -> int:
    """Smallest integer q satisfying the strict coefficient-box inequality."""
    assert 0 <= D < r and U >= 0 and exponent > 0
    s = 2 * r * U + 1  # m=1; least integer strictly above 2rU/m
    c = exponent * (s + 1)
    B = 2 * (TORSION_CARDINALITY * c**D + 1)
    assert 2 * r * U < c
    return (2 * B + 1) ** r


def max_U_all_orders(r: int, D: int, bits: int) -> int | None:
    """Largest integral U for which every bits-bit q clears the budget."""
    boundary = 2 ** (bits - 1)
    if threshold(r, D, 0, MAZUR_EXPONENT) > boundary:
        return None
    lo, hi = 0, 1
    while threshold(r, D, hi, MAZUR_EXPONENT) <= boundary:
        hi *= 2
    while lo + 1 < hi:
        mid = (lo + hi) // 2
        if threshold(r, D, mid, MAZUR_EXPONENT) <= boundary:
            lo = mid
        else:
            hi = mid
    assert threshold(r, D, lo, MAZUR_EXPONENT) <= boundary
    assert threshold(r, D, lo + 1, MAZUR_EXPONENT) > boundary
    return lo


def main() -> None:
    cyclic_orders = list(range(1, 11)) + [12]
    product_exponents = [2 * v for v in range(1, 5)]
    assert math.lcm(*(cyclic_orders + product_exponents)) == MAZUR_EXPONENT
    assert max([*cyclic_orders, *(4 * v for v in range(1, 5))]) == 16
    rows = []
    for r, D in CASES:
        variants = {}
        for name, annihilator in (
            ("factorial_16", math.factorial(16)),
            ("cardinality_lcm_16", math.lcm(*range(1, 17))),
            ("mazur_exponent", MAZUR_EXPONENT),
        ):
            qmin = threshold(r, D, 1, annihilator)
            variants[name] = {
                "annihilator": annihilator,
                "minimum_integer_q": str(qmin),
                "minimum_integer_q_bit_length": qmin.bit_length(),
                "all_131_bit_q_pass": qmin <= 2**130,
                "no_131_bit_q_pass": qmin >= 2**131,
                "all_256_bit_q_pass": qmin <= 2**255,
                "no_256_bit_q_pass": qmin >= 2**256,
            }
        rows.append({
            "r": r, "rank_cap_D": D, "torsion_cardinality_cap_T": 16,
            "upper_norm_U": 1, "minimum_norm_m": 1,
            "variants": variants,
            "max_integral_U_for_all_q_at_m_1": {
                str(bits): max_U_all_orders(r, D, bits)
                for bits in BIT_SIZES
            },
            "status": "derived_parameter_control",
        })
    print(json.dumps(rows, indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
