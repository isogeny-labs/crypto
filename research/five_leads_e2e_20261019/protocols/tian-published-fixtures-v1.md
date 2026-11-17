# Published Tian fixture checks

This is a deterministic construction-stage replay, not an isogeny or DLP
solve. The source fixtures are [Tian 2020, section 3.4](https://arxiv.org/html/2012.07173)
and [Tian 2020, section 6](https://arxiv.org/html/2007.03172).

For the cubic elliptic fixture use `F_523[w]/(w^3+2)` and
`F_523[w,theta]/(w^3+2,theta^2-w-6)` exactly. Check that the field tower is
irreducible, the nine listed points lie on the three Frobenius-conjugate
curves and have exact order three, the three triples generate exactly 27
points, and Frobenius sends each generator into that subgroup. Compute the
Weil pairing via tangent-line divisor evaluation with disjoint auxiliary
divisors; require third roots of unity, independence of two auxiliary
choices, antisymmetry and product pairing one for every generator pair.
For the source group order, prove `143069629` prime by trial division, exhibit
a nonzero rational point annihilated by that number, and use Hasse uniqueness
to certify `#E(F_523^3)=143069629`. This also makes the source subgroup's
intersection with the 3-kernel trivial. All calculations use native Rust
in `native/tian_cubic_kernel.rs`.

For the genus-3 fixture, feed the exact eight published `F_257` curve
coefficients and first rational Mumford divisor to the repository's checked
one-infinity Jacobian CLI. Require that the divisor validates and `3T1=0`.
The other two torsion generators require `F_257^6` arithmetic and are not
checked by this fixture.

For both published rational plane quartic equations, compute the curve and
its three homogeneous partial derivatives. In the chart `z=1`, prove their
ideal is the unit ideal using the native finite-field F4 engine. Repeat at
infinity in `x=1,z=0` and check `(0:1:0)` directly. A unit ideal rules out
singular points over the algebraic closure in each chart. Retain the exact
coefficient order and F4 reports. This checks the quartic equations as
written; it does not prove they represent the claimed quotients.

The published quartic equations and rational-map formulas are not accepted
as executed transfer evidence. Construction of the quotient, rational
descent, exception handling, image/dual maps, 100 known-answer target
transfers and complete source-to-answer costs are separate obligations in
[`problems/odd-isogeny-quartic-transfer-execution.md`](../../../problems/odd-isogeny-quartic-transfer-execution.md).
