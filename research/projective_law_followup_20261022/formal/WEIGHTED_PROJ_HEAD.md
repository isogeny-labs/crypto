# Weighted-Proj affine-chart bridge: bounded API and compiler attempt

The proposed projective target is `Proj A` for the degree-`(1,3,1)` weighted
polynomial quotient

`A = K[X,Y,Z]/(Y² - X⁵Z - a3 X³Z³ - a2 X²Z⁴ - a1 XZ⁵ - a0 Z⁶)`.

The intended first bridge is an isomorphism from its standard open `D₊(Z)`
to `Spec K[x,y]/(y² - x⁵ - a3 x³ - a2 x² - a1 x - a0)`, carrying `x` to
`X/Z` and `y` to `Y/Z³`. This is not proved here. The file
`weighted-proj-head-unchecked-lean.txt` preserves the exact unverified Lean
attempt to prove the weighted homogeneity of the sextic and its ideal. It is
stored as text so it cannot be mistaken for a proof-gate target.

## Pinned APIs found

- `MvPolynomial.weightedGradedAlgebra` is defined in pinned Mathlib's
  `RingTheory/MvPolynomial/WeightedHomogeneous.lean` (near line 700). Its
  `IsWeightedHomogeneous` constructors include `X`, `C`, `mul`, `pow`, `add`,
  and `sub`.
- `Ideal.homogeneous_span` is defined in
  `RingTheory/GradedAlgebra/Homogeneous/Ideal.lean` (near line 156), and
  would turn a checked degree-six relation into a homogeneous ideal.
- `Proj.basicOpenIsoSpec` in
  `AlgebraicGeometry/ProjectiveSpectrum/Basic.lean` supplies the standard
  `D₊(Z) ≅ Spec (A[Z⁻¹])₀` isomorphism once `A` has a `GradedRing` instance.
  `Proj.toSpecZero` is proper for a finite-type graded ring in
  `ProjectiveSpectrum/Proper.lean` (near line 372).

The precise missing construction is a `GradedRing` on the quotient `A`, with
degree-`n` component the image of `MvPolynomial.weightedHomogeneousSubmodule
K weights n` under the quotient map. It requires a direct-sum decomposition
modulo the homogeneous ideal and multiplication compatibility. Targeted
searches of the pinned `Mathlib/RingTheory/GradedAlgebra`,
`Mathlib/Algebra/DirectSum`, and `Mathlib/AlgebraicGeometry/ProjectiveSpectrum`
trees found no ready homogeneous-ideal quotient-grading constructor. That is
an API search result, not a proof of absence. After constructing the grading,
one still needs an explicit isomorphism `(A[Z⁻¹])₀ ≃ K[x,y]/(y²-f(x))`;
`basicOpenIsoSpec` alone does not supply it.

## Compiler boundary

Three bounded commands and their exit codes are recorded in
`weighted-proj-head-exit.txt`. The Lake wrapper timed out during startup
(exit 124, empty output). A direct Lean invocation with only Mathlib on
`LEAN_PATH` failed to resolve `Batteries`; its exact output is retained in
`weighted-proj-head-build.log`. A second direct invocation with every pinned
package OLean path reached the compiler process but timed out during an
uninterruptible I/O wait (exit 124, empty output). The process was inspected
and is terminal. There is no exit-0 compiler receipt or axiom audit for the
weighted-head candidate, so neither the equation-homogeneity statement nor
the quotient ideal is reported as a checked Lean theorem.

The already checked two-chart gluing and unique infinity point remain
separate results. No `Proj` chart isomorphism, projectivity, smoothness, or
`Pic⁰` consequence is promoted by this attempt.
