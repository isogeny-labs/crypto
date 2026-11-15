#!/usr/bin/env sage -python
"""Exact Rabin irreducibility certificate for the q32 Weil restriction."""

import hashlib
import json
from pathlib import Path

from sage.all import GF, PolynomialRing, gcd
from sage.version import version as sage_version

R = PolynomialRing(GF(139), "X")
X = R.gen()
f = X**14 + 155709 * X**7 + 32**7
assert f == X**14 + 29 * X**7 + 50
rem2 = pow(X, 139**2, f)
rem7 = pow(X, 139**7, f)
rem14 = pow(X, 139**14, f)
assert rem2 == 38 * X**8 + 50 * X
assert rem7 == 105 * X**13 + 126 * X**6
assert gcd(f, rem2 - X) == 1
assert gcd(f, rem7 - X) == 1
assert rem14 == X
assert f.is_irreducible()

result = {
    "schema": "ghs-n7-q32-genus-floor/v1",
    "sage_version": sage_version,
    "script_sha256": hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
    "restriction_polynomial_Z": "X^14 + 155709*X^7 + 34359738368",
    "modulus_prime": 139,
    "polynomial_mod_139": str(f),
    "X_pow_139_squared_mod_f": str(rem2),
    "X_pow_139_seventh_mod_f": str(rem7),
    "X_pow_139_fourteenth_mod_f": str(rem14),
    "gcd_for_14_over_7": "1",
    "gcd_for_14_over_2": "1",
    "irreducible_mod_139": True,
    "conclusion": "the degree-seven Weil restriction is simple; every nonzero algebraic Jacobian image forces genus at least seven",
}
path = Path(__file__).with_name("genus_floor.json")
if path.exists():
    raise SystemExit("refusing to overwrite immutable certificate")
path.write_text(json.dumps(result, indent=2) + "\n")
print(json.dumps(result, indent=2))
