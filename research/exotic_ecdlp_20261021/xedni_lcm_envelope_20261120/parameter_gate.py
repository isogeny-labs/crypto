#!/usr/bin/env python3
"""Exact integer sensitivity control for the checked torsion envelopes."""

import json
import math
from functools import reduce


CASES = (
    (2, 1, 16, 1, 1),
    (3, 2, 16, 1, 1),
    (4, 3, 16, 1, 1),
    (3, 2, 8, 1, 1),
    (3, 2, 4, 1, 1),
)


def envelope(r: int, d: int, T: int, U: int, m: int, kind: str) -> dict:
    assert 0 <= d < r and T > 0 and U >= 0 and m > 0
    annihilator = (math.factorial(T) if kind == "factorial" else
                   reduce(math.lcm, range(1, T + 1), 1))
    s = (2 * r * U) // m + 1
    c = annihilator * (s + 1)
    B = 2 * (T * c**d + 1)
    minimum_integer_q = (2 * B + 1) ** r
    assert all(c % n == 0 for n in range(1, T + 1))
    assert 2 * r * U < c * m
    assert minimum_integer_q - 1 < minimum_integer_q
    return {
        "r": r, "rank_cap_D": d, "torsion_cardinality_cap_T": T,
        "upper_norm_U": U, "minimum_norm_m": m, "annihilator_kind": kind,
        "universal_annihilator": annihilator, "integer_s": s,
        "common_c": c, "coefficient_bound_B": B,
        "minimum_integer_q_for_strict_budget": str(minimum_integer_q),
        "minimum_integer_q_bit_length": minimum_integer_q.bit_length(),
        "all_256_bit_q_pass_budget": minimum_integer_q <= 2**255,
        "no_256_bit_q_pass_budget": minimum_integer_q >= 2**256,
        "status": "derived_parameter_control",
    }


def main() -> None:
    rows = [envelope(*case, kind) for case in CASES
            for kind in ("factorial", "lcm")]
    print(json.dumps(rows, indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
