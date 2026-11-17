# Order-548 plane-quartic elliptic-quotient audit

Frozen 26 October 2026, Conductor T-5428. Input is the exact order-548
quartic and source family in
[the new structural report](../ghs_quartic_new_classes_q8_20261026/REPORT.md).

[Nart--Ritzenthaler](https://arxiv.org/pdf/0905.0546) classify genus-three
characteristic-two curves with a rational `C2 × C2` of elliptic
involutions and give explicit degree-two quotient maps. The
[Magma plane-quartic documentation](https://docs.magma-maths.org/ArithmeticGeometry/PlaneQuartics/quartic_isomorphisms.html)
specifies base-field and geometric automorphism calculations. The
[Magma calculator API](https://github.com/Magma-Maths/calculator/blob/main/README.md)
has a 300-second ceiling; this experiment submits the public quartic
equation and records exact input/output. The prior
[Tian construction](https://arxiv.org/pdf/2012.07173) is an isogeny-only
route and is not excluded by a negative automorphism result.

## Checks

1. Reconstruct `GF(8)` using its Conway polynomial and verify that the
   submitted quartic has nine rational points and is geometrically smooth.
2. Factor `X^6+35X^3+512` over `QQ`. Irreducibility rules out an
   elliptic isogeny factor and hence an elliptic quotient defined over
   `GF(8)`.
3. Ask Magma for base-field and geometric plane-quartic automorphisms.
   If an involution exists, retain its matrix, field of definition,
   quotient genus, quotient equation, and tested source-order match.
4. If the geometric automorphism computation fails or times out,
   retain the exact failure and do not claim an exclusion.

A trivial geometric automorphism group excludes all separable degree-two
curve maps to elliptic curves over the algebraic closure, but does not
exclude degree-three or higher maps, isogeny-only correspondences,
or a GHS attack. An evaluated subgroup-preserving source map and full
same-target attack cost remain separate gates.
