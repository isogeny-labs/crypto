#!/usr/bin/env sage -python
"""Finite-field certificates for the three simple Weil-restriction classes."""

import json
from pathlib import Path

from sage.all import GF, PolynomialRing, gcd, power_mod

HERE = Path(__file__).resolve().parent
cases = [(3974, 11), (3978, 11), (4178, 5)]
records = []
for order, p in cases:
    trace = 16**3 + 1 - order
    R = PolynomialRing(GF(p), "z")
    z = R.gen()
    f = z**6 - trace*z**3 + 16**3
    test2 = gcd(f, power_mod(z, p**2, f) - z)
    test3 = gcd(f, power_mod(z, p**3, f) - z)
    test6 = power_mod(z, p**6, f)
    assert f.degree() == 6 and test2 == 1 and test3 == 1 and test6 == z
    assert f.is_irreducible()
    records.append({
        "source_order": order, "trace": trace, "modulus_prime": p,
        "weil_polynomial_high_to_low": [1, 0, 0, -trace, 0, 0, 4096],
        "gcd_x_p2_minus_x": str(test2),
        "gcd_x_p3_minus_x": str(test3),
        "x_p6_mod_f": str(test6),
        "irreducible_mod_p": True,
        "consequence_over_Q": "irreducible by Gauss lemma",
    })

receipt = {
    "schema": "ghs-q16-simple-weil-polynomial-v1",
    "method": "degree-six Rabin irreducibility test in Sage finite polynomial rings",
    "records": records,
}
(HERE / "irreducibility_receipt.json").write_text(json.dumps(receipt, indent=2) + "\n")
print(json.dumps(receipt, indent=2))
