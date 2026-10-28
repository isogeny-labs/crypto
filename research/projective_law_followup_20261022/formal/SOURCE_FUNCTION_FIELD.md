# Source-style function on the actual weighted-Proj curve

**Status:** Pinned Lean 4.34.1 checks the chart embedding and the source-shaped expression under the exact hypotheses in `WeightedProjSourceFunctionField.lean`. The associated closed-point divisor and Picard-class statements remain open.

Let `X = Proj(degreeSubmodule a3 a2 a1 a0)` over a field `K`, and assume `WeightedProjSmoothPoints.quintic a3 a2 a1 a0` is separable. The checked `WeightedProjIsIntegral.whole_curve_integral` supplies `IsIntegral X`; the actual `Z`-basic open is nonempty. The new `affineToFunctionField` is the composite of the `Z`-chart isomorphism, section restriction, and generic germ. Its injectivity is checked. Thus the chart coordinates `xbar` and `ybar` have specific images in `X.functionField`.

For the eight source coefficients `(q1,r1,s1,t1,q2,r2,s2,t2)`, `sourceNumerator` is

`N = B(s1*x+t1) - (C*x+A)(x^2+q1*x+r1)`,

where `(B,A,C,Q,Rbar) = Common22SupportHead.coefficientData(...)`. The checked `sourceFunction` is the field element `phi(N)/phi(B)-phi(ybar)`. `sourceDenominator_ne_zero` adds the explicit source-branch condition `B ≠ 0` and proves `phi(B) ≠ 0`. The definition alone does not assert that an input support is valid or that any divisor equality holds.

## Closed-point gate

Pinned Mathlib `AlgebraicGeometry/OrderOfVanishing.lean` defines `Scheme.ordHom` and `Scheme.ord` only under `[IsIntegral X] [IsLocallyNoetherian X]`, with a point `z` satisfying `coheight z = 1` for the non-junk order. This packet proves `IsIntegral X` via the prior theorem but contains no checked `IsLocallyNoetherian X`, no identification of all closed support points with coheight-one points, and no finite-support divisor construction. Existing rational-point DVR/norm lemmas apply at specified rational points; a prior exact F3 diagnostic has irreducible quadratic supports and demonstrates why those rational points alone are insufficient for the input class. The next direct proof obligations are local Noetherianity, all-closed-point order compatibility of the source function with the existing norm identity, finite support, the infinity pole, and the global principal-divisor/Pic0 class equality. Compiled runtime correspondence is a separate gate.

The [CSV status table](source-function-field.csv), [SVG dependency graph](source-function-field.svg), [PDF report](SOURCE_FUNCTION_FIELD.pdf), and [direct Lean output](source-function-field-lean.log) give a reviewable record. No DLP experiments or cost comparisons were run. The PDF and source checksums are recorded in the session log.

An independent scratch check recompiled `WeightedProjIntegral`,
`WeightedProjOverlapNonempty`, `WeightedProjIsIntegral`, and this source module
with pinned Lean 4.34.1 against the repository's cached Mathlib revision.
The final [independent compiler receipt](source-function-field-independent-lean.log)
records exit code 0 and the standard axiom lists; its SHA-256 is
`591d9f0baea50f185dc0f5cba1975e718d8b4951dbefc35eac2507b7c1731b23`.
This check did not add a divisor or runtime theorem.
