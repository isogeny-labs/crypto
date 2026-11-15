#!/usr/bin/env sage -python
"""Exact mod-13 irreducibility certificate for the q16 restriction polynomial."""

import json
from pathlib import Path

from sage.all import GF, PolynomialRing, gcd

R = PolynomialRing(GF(13), "X")
X = R.gen()
f = X**14 - 22271 * X**7 + 16**7
assert f == X**14 + 11 * X**7 + 3
rem2 = pow(X, 13**2, f)
rem7 = pow(X, 13**7, f)
rem14 = pow(X, 13**14, f)
assert rem2 == 12 * X**8 + 6 * X
assert rem7 == 12 * X**13 + 2 * X**6
assert gcd(f, rem2 - X) == 1
assert gcd(f, rem7 - X) == 1
assert rem14 == X
assert f.is_irreducible()

result = {
    "schema": "ghs-n7-q16-genus-floor/v1",
    "restriction_polynomial_Z": "X^14 - 22271*X^7 + 268435456",
    "modulus_prime": 13,
    "polynomial_mod_13": str(f),
    "X_pow_13_squared_mod_f": str(rem2),
    "X_pow_13_seventh_mod_f": str(rem7),
    "X_pow_13_fourteenth_mod_f": str(rem14),
    "gcd_for_14_over_7": "1",
    "gcd_for_14_over_2": "1",
    "irreducible_mod_13": True,
    "conclusion": "degree-seven Weil restriction is simple; nonzero algebraic Jacobian image forces genus at least seven",
}
path = Path(__file__).with_name("genus_floor.json")
if path.exists():
    raise SystemExit("refusing to overwrite immutable certificate")
path.write_text(json.dumps(result, indent=2) + "\n")
print(json.dumps(result, indent=2))
