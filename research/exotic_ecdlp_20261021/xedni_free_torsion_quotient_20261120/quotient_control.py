#!/usr/bin/env python3
"""Finite enumeration control for the free-plus-torsion quotient."""

import json
from itertools import product


def main() -> None:
    rank = 2
    modulus = 6
    torsion_order = 6
    quotient = set(product(range(modulus), repeat=rank + 1))
    assert len(quotient) == torsion_order * modulus**rank
    seen = set()
    kernel_count = 0
    for x, y, torsion in product(range(-12, 13), range(-12, 13), range(torsion_order)):
        residue = (x % modulus, y % modulus, torsion)
        seen.add(residue)
        if residue == (0, 0, 0):
            kernel_count += 1
            assert torsion == 0
            assert x % modulus == y % modulus == 0
            assert (x, y, torsion) == (modulus * (x // modulus), modulus * (y // modulus), 0)
    assert seen == quotient
    print(json.dumps({
        "status": "verified_control",
        "free_rank": rank,
        "torsion_group": "Z/6Z",
        "modulus": modulus,
        "quotient_cardinality": len(quotient),
        "enumerated_lift_points": 25 * 25 * torsion_order,
        "kernel_points_in_window": kernel_count,
        "surjective_in_window": True,
        "kernel_divisibility_checked": True,
    }, sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
