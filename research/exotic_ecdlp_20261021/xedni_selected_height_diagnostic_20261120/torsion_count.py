#!/usr/bin/env python3
"""Independent exact finite-field point counts on the three selected curves."""

import json
import math
from fractions import Fraction
from pathlib import Path


ROOT = Path(__file__).resolve().parent
PRIMES = (3, 5, 7, 11, 13, 17, 19, 23, 29, 31)


def reduced_fraction(x, p):
    assert x.denominator % p
    return x.numerator * pow(x.denominator, -1, p) % p


def count_points(a, b, p):
    squares = {y * y % p for y in range(p)}
    total = 1
    for x in range(p):
        rhs = (x**3 + a * x + b) % p
        total += (1 if rhs == 0 else 2 if rhs in squares else 0)
    return total


def main():
    fixtures = json.loads((ROOT / "fixtures.json").read_text())
    rows = []
    for item in fixtures["curves"]:
        A, B = (Fraction(x) for x in item["weierstrass_coefficients"][3:])
        delta = -16 * (4 * A**3 + 27 * B**2)
        counts = []
        for p in PRIMES:
            if any(z.denominator % p == 0 for z in (A, B, delta)):
                continue
            if delta.numerator % p == 0:
                continue
            a, b = reduced_fraction(A, p), reduced_fraction(B, p)
            counts.append({"p": p, "group_order": count_points(a, b, p),
                           "a_mod_p": a, "b_mod_p": b})
        chosen = next(((x, y) for i, x in enumerate(counts)
                       for y in counts[i + 1:]
                       if math.gcd(x["group_order"], y["group_order"]) == 1), None)
        assert chosen is not None
        rows.append({"id": item["id"], "good_prime_counts": counts,
                     "first_coprime_pair": [chosen[0]["p"], chosen[1]["p"]],
                     "orders": [chosen[0]["group_order"], chosen[1]["group_order"]],
                     "gcd": 1})
    print(json.dumps({"schema": "xedni-selected-torsion-count-v1",
                      "method": "exact quadratic-residue point enumeration",
                      "rows": rows}, indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
