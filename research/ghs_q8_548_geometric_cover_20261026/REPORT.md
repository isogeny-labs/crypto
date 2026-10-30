# The new order-548 plane quartic has no geometric degree-two elliptic cover

26 October 2026. The [frozen protocol](PROTOCOL.md), exact
[base-field](base.magma) and [geometric](geometric.magma) Magma inputs,
their [base receipt](base_magma_receipt.json) and
[geometric receipt](geometric_magma_receipt.json), and the
[Sage audit](audit.py) with [certificate](certificate.json) test the
exact smooth plane quartic in the
[new order-548 family](../ghs_quartic_new_classes_q8_20261026/REPORT.md).

## Result

The online Magma 2.30-1 run reconstructed Conway `GF(8)` with polynomial
`a³+a+1`, independently counted nine rational points, and reported a
smooth plane quartic. Its `AutomorphismGroupOfPlaneQuartic` calls returned
group `C1` both over `GF(8)` and **over the algebraic closure**.
The receipts retain the exact input hashes, full output, Magma version,
success status, and absence of truncation. The Sage audit validates these
fields and independently proves that the Weil polynomial
`X⁶+35X³+512` is irreducible over `QQ`.

A separable degree-two map from a smooth curve to an elliptic curve is a
quadratic Galois extension of function fields, hence has a nonidentity
deck involution. Because this quartic has trivial geometric automorphism
group, **it has no separable degree-two map to an elliptic curve over any
extension of `GF(8)`**. In particular the
[Nart--Ritzenthaler](https://arxiv.org/pdf/0905.0546) characteristic-two
`C2 × C2` elliptic-quotient construction cannot supply the desired map
for this exact quartic. The irreducible Weil polynomial separately
excludes a `GF(8)`-rational elliptic factor of its Jacobian.

This result narrows the construction route for the nine order-548 source
curves. It does not exclude degree-three or higher curve maps over
`GF(512)`, isogeny-only correspondences, unlisted curves or
polarizations, or a full GHS transfer. No evaluated source-to-Jacobian
map or complete same-target attack cost is supplied.

## Prior work and reproducibility

The [Magma plane-quartic handbook](https://docs.magma-maths.org/ArithmeticGeometry/PlaneQuartics/quartic_isomorphisms.html)
specifies base-field and geometric automorphism modes. The
[calculator API](https://github.com/Magma-Maths/calculator/blob/main/README.md)
executed the public equation in a separate service and retained the
responses. The geometric calculation is a Magma computer-algebra
certificate, not a Lean formalization of the automorphism group.
The earlier [Lean arithmetic check](../ghs_quartic_new_classes_q8_20261026/Counts.lean)
still proves `gcd(137,111)=1`; it does not provide an algebraic isogeny.
