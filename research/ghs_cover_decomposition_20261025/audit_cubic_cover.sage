"""Exact Frobenius and Prym-factor audit of the fixed GF(8) cubic cover.

H: v^2 + u^2 v = u^7 + u^4 + d u, d^2 = z+1,
E: y^2 + xy = x^3 + x^2 + (z+1), z^3+z+1=0.
The repository's exhaustive arbitrary-class norm fixture uses this model.
"""

from __future__ import annotations

import json
from pathlib import Path

from sage.all import EllipticCurve, GF, PolynomialRing, ZZ


HERE = Path(__file__).resolve().parent
q = 8
ring = PolynomialRing(ZZ, "X")
X = ring.gen()
counts = []
field_data = []

for extension_degree in (1, 2, 3):
    field = GF(2 ** (3 * extension_degree), f"z{extension_degree}")
    poly_ring = PolynomialRing(field, "T")
    T = poly_ring.gen()
    # Each root gives a Frobenius-conjugate embedding of the exact GF(8)
    # fixture into this extension; point counts must be independent of it.
    roots = (T**3 + T + 1).roots(multiplicities=False)
    assert len(roots) == 3
    embedding_counts = []
    for z in roots:
        a, b = field.one(), z + 1
        d = b.sqrt()
        assert d*d == b
        count = 1  # unique rational point at infinity
        for u in field:
            h = u*u
            f = u**7 + a*u**4 + d*u
            if h == 0:
                count += 1
            elif (f / (h*h)).trace() == 0:
                count += 2
        embedding_counts.append(int(count))
    assert len(set(embedding_counts)) == 1
    counts.append(embedding_counts[0])
    field_data.append({
        "extension_degree": extension_degree,
        "field_size": int(field.order()),
        "three_embedding_counts": embedding_counts,
    })

    if extension_degree == 1:
        elliptic_order = int(EllipticCurve(field, [a, a, 0, 0, b]).cardinality())
        assert elliptic_order == 6
        # Independent equation enumeration checks the binary trace count.
        brute = 1 + sum(
            v*v + u*u*v == u**7 + u**4 + d*u
            for u in field for v in field
        )
        assert brute == counts[0]

# Newton identities recover the degree-six Weil polynomial of Jac(H).
power_sums = [q**j + 1 - counts[j-1] for j in (1, 2, 3)]
s1, s2, s3 = power_sums
assert (s1*s1 - s2) % 2 == 0
e1 = s1
e2 = (s1*s1 - s2) // 2
assert (s1**3 - 3*s1*s2 + 2*s3) % 6 == 0
e3 = (s1**3 - 3*s1*s2 + 2*s3) // 6
frobenius = X**6 - e1*X**5 + e2*X**4 - e3*X**3 + q*e2*X**2 - q*q*e1*X + q**3
elliptic_trace = q + 1 - elliptic_order
elliptic_factor = X**2 - elliptic_trace*X + q
assert frobenius % elliptic_factor == 0
prym_factor = frobenius // elliptic_factor
assert prym_factor.degree() == 4
assert int(frobenius(1)) == 486  # independent exhaustive Jacobian fixture
assert int(prym_factor(1)) * elliptic_order == 486
supersingular_factor = X**2 + q
assert prym_factor == supersingular_factor**2
base_field = GF(8, "z")
supersingular_curve = EllipticCurve(base_field, [0, 0, 1, 0, 0])
assert int(supersingular_curve.cardinality()) == 9
assert [int(c) for c in supersingular_curve.frobenius_polynomial()] == [int(c) for c in supersingular_factor]
# An integral Bezout identity gives a scaled factor separation. Its scalar
# 72 is divisible by both primes in the rational source order six, so it
# does not invert to an integral projector on either selected primary part.
bezout_scalar = 72
assert 3*X*elliptic_factor + (9-3*X)*supersingular_factor == bezout_scalar
assert int(elliptic_factor.resultant(supersingular_factor)) == bezout_scalar

result = {
    "schema": "cubic-cover-prym-audit/v1",
    "fixture": "src/binary_ecc/cover_transfer.rs; GF(8), z^3+z+1, a=1, b=z+1 (word 3)",
    "cover": "v^2+u^2*v=u^7+u^4+d*u, d^2=z+1",
    "map": "(u,v)->(u^3,u*v+d), degree 3",
    "point_counts": field_data,
    "power_sums": power_sums,
    "genus3_frobenius_coefficients_low_to_high": [int(c) for c in frobenius],
    "elliptic_factor_coefficients_low_to_high": [int(c) for c in elliptic_factor],
    "prym_factor_coefficients_low_to_high": [int(c) for c in prym_factor],
    "supersingular_elliptic_factor_coefficients_low_to_high": [int(c) for c in supersingular_factor],
    "supersingular_elliptic_curve": "y^2+y=x^3 over GF(8)",
    "supersingular_elliptic_curve_order": int(supersingular_curve.cardinality()),
    "integral_bezout_identity": "3*X*P_E(X)+(9-3*X)*(X^2+8)=72",
    "factor_resultant": bezout_scalar,
    "prym_factorization_over_Z": [
        {"coefficients_low_to_high": [int(c) for c in f], "multiplicity": int(m)}
        for f, m in prym_factor.factor()
    ],
    "elliptic_order": elliptic_order,
    "jacobian_order": int(frobenius(1)),
    "prym_order": int(prym_factor(1)),
    "embedding_invariance_checked": True,
    "base_curve_equation_enumeration_checked": True,
    "source_group_placement": "The canonical pullback is annihilated by [3]id-pull*push, conditional on the checked norm-pullback identity; the Prym order alone is not a transfer map.",
}
(HERE / "cubic_results.json").write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")
print(json.dumps(result, indent=2, sort_keys=True))
