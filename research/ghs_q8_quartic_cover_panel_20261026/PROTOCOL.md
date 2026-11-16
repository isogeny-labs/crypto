# Four new GF(8) quartic geometric-cover tests

Frozen 26 October 2026, Conductor T-5429. Inputs are the independently
verified order `482,524,554,556` plane quartics in
[the structural search](../ghs_quartic_new_classes_q8_20261026/REPORT.md).
The separate [order-548 audit](../ghs_q8_548_geometric_cover_20261026/REPORT.md)
found a trivial geometric automorphism group.

[Nart--Ritzenthaler](https://arxiv.org/pdf/0905.0546) give explicit
elliptic quotients for characteristic-two genus-three curves with a
`C2 × C2` group of involutions. The
[Magma plane-quartic handbook](https://docs.magma-maths.org/ArithmeticGeometry/PlaneQuartics/quartic_isomorphisms.html)
defines `AutomorphismGroupOfPlaneQuartic` with `geometric:=true` and
`explicit:=true`. Submit each public equation to the documented
[Magma calculator API](https://github.com/Magma-Maths/calculator/blob/main/README.md),
retaining input and response hashes.

For each of the four quartics, first verify the Conway `GF(8)` defining
polynomial, smoothness and nine rational points. Then compute the
base-field and geometric automorphism groups. Retain generator matrices
for any nontrivial group and test each involution's quotient genus,
equation, field of definition, and order against its source class.
If a computation fails or times out, retain that outcome and leave the
corresponding cover status unknown. A trivial geometric group excludes
separable degree-two curve maps to elliptic curves over any extension,
but does not exclude higher-degree maps or isogeny-only transfers.
