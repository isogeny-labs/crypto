#!/usr/bin/env python3
"""Exact derived subgroup-order thresholds for the checked height bound."""

import json


def row(r: int, d: int, torsion_order: int, upper: int, minimum: int) -> dict:
    assert 0 <= d < r and torsion_order > 0 and upper >= 0 and minimum > 0
    c = torsion_order
    while c * minimum <= 2 * r * upper:
        c += torsion_order
    k = torsion_order * c**d + 1
    n = c * k
    bound = 2 * k
    box_nonzero = (2 * bound + 1) ** r - 1
    assert c % torsion_order == 0
    assert 2 * r * k * upper < n * minimum
    assert torsion_order * n**d < (2 * k + 1) ** r
    return {
        "r": r,
        "rank_d": d,
        "torsion_order": torsion_order,
        "upper_norm": upper,
        "minimum_norm": minimum,
        "annihilator_c": c,
        "coefficient_radius_k": k,
        "relation_bound": bound,
        "quotient_modulus_n": n,
        "nonzero_candidate_vectors": box_nonzero,
        "minimum_integer_q_for_strict_budget": box_nonzero + 1,
        "status": "derived_parameter_control",
    }


def main() -> None:
    cases = [(2, 1, 1, 1, 1), (2, 1, 16, 1, 1),
             (3, 2, 16, 1, 1), (2, 1, 16, 2, 1)]
    print(json.dumps([row(*case) for case in cases], indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
