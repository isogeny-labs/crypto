#!/usr/bin/env python3
"""Exact integer-lattice control for the quotient collision lemma."""

import json
from itertools import product


def main() -> None:
    points = (2, 3)
    rank = 1
    torsion_order = 1
    coefficient_radius = 14
    quotient_modulus = 182
    size_bound = len(points) * coefficient_radius * max(abs(x) for x in points)
    box = list(product(range(-coefficient_radius, coefficient_radius + 1), repeat=len(points)))
    by_residue: dict[int, tuple[tuple[int, ...], int]] = {}
    equal_sum_collision = None
    for coeff in box:
        value = sum(a * point for a, point in zip(coeff, points))
        assert abs(value) <= size_bound
        residue = value % quotient_modulus
        previous = by_residue.setdefault(residue, (coeff, value))
        if previous[0] != coeff:
            assert previous[1] == value
            equal_sum_collision = (previous[0], coeff)
    assert equal_sum_collision is not None
    assert 2 * size_bound < quotient_modulus
    assert len(box) > torsion_order * quotient_modulus**rank
    a, b = equal_sum_collision
    relation = tuple(x - y for x, y in zip(a, b))
    assert any(relation)
    assert all(abs(x) <= 2 * coefficient_radius for x in relation)
    assert sum(x * point for x, point in zip(relation, points)) == 0
    print(
        json.dumps(
            {
                "status": "verified_control",
                "rank": rank,
                "points": points,
                "coefficient_radius": coefficient_radius,
                "box_cardinality": len(box),
                "quotient_cardinality": torsion_order * quotient_modulus**rank,
                "size_bound": size_bound,
                "kernel_gap": quotient_modulus,
                "distinct_residues": len(by_residue),
                "relation": relation,
            },
            indent=2,
            sort_keys=True,
        )
    )


if __name__ == "__main__":
    main()
