import Mathlib.AlgebraicGeometry.EllipticCurve.VariableChange
import Mathlib.AlgebraicGeometry.EllipticCurve.Reduction

/-!
# Actual Weierstrass variable changes and model discriminants

This strengthens the elementary polynomial calculation in
`JKSSTModelDiscriminant` using Mathlib's bundled `WeierstrassCurve` and
`WeierstrassCurve.VariableChange` APIs.

For the short model `y² = x³ + A*x + B`, the admissible coordinate change
`(x,y) ↦ (u²*x,u³*y)` identifies it with the model having coefficients
`A*u⁴, B*u⁶`.  Mathlib's general weight-twelve theorem then proves that
the model discriminant is multiplied by `u¹²`.

This is an isomorphism/change-of-model statement over a field.  It does not
identify either equation as a *global* minimal integral model.  Mathlib's
`WeierstrassCurve.IsMinimal` in `Reduction.lean` is minimality for one chosen
discrete valuation ring, so an additional global comparison is still needed
for the JKSST/Lang application over `ℚ`.
-/

namespace JKSSTWeierstrassScaling

open WeierstrassCurve

variable {K : Type*} [Field K]

/-- The bundled short Weierstrass model `y²=x³+A*x+B`. -/
def shortModel (A B : K) : WeierstrassCurve K :=
  ⟨0, 0, 0, A, B⟩

/-- The admissible change whose action sends `(A,B)` to `(A*u⁴,B*u⁶)`.
The inverse appears because Mathlib's action writes transformed coefficients
using `C.u⁻¹`. -/
def scalingChange (u : Kˣ) : WeierstrassCurve.VariableChange K :=
  ⟨u⁻¹, 0, 0, 0⟩

/-- The library's bundled variable-change action produces exactly the scaled
short model. -/
theorem scalingChange_smul_shortModel (A B : K) (u : Kˣ) :
    scalingChange u • shortModel A B =
      shortModel (A * (u : K) ^ 4) (B * (u : K) ^ 6) := by
  ext <;>
    simp [scalingChange, shortModel, variableChange_a₁, variableChange_a₂,
      variableChange_a₃, variableChange_a₄, variableChange_a₆,
      mul_comm]

/-- Mathlib's discriminant specializes to the standard short-model formula. -/
theorem shortModel_discriminant (A B : K) :
    (shortModel A B).Δ = -16 * (4 * A ^ 3 + 27 * B ^ 2) := by
  simp only [shortModel, WeierstrassCurve.Δ, WeierstrassCurve.b₂,
    WeierstrassCurve.b₄, WeierstrassCurve.b₆, WeierstrassCurve.b₈]
  ring

/-- The actual bundled model discriminant has weight twelve under the
admissible scaling change. -/
theorem scalingChange_discriminant (A B : K) (u : Kˣ) :
    (scalingChange u • shortModel A B).Δ =
      (u : K) ^ 12 * (shortModel A B).Δ := by
  rw [WeierstrassCurve.variableChange_Δ]
  simp [scalingChange]

/-- Equivalent coefficient-level form of the same bundled theorem. -/
theorem scaled_shortModel_discriminant (A B : K) (u : Kˣ) :
    (shortModel (A * (u : K) ^ 4) (B * (u : K) ^ 6)).Δ =
      (u : K) ^ 12 * (shortModel A B).Δ := by
  rw [← scalingChange_smul_shortModel]
  exact scalingChange_discriminant A B u

/-- Every Weierstrass curve over the fraction field of a discrete valuation
ring has an isomorphic model minimal at that one valuation.  This records the
precise local theorem available in Mathlib and deliberately does not call it
a global minimal discriminant. -/
theorem exists_locally_minimal_change
    (R : Type*) [CommRing R] [IsDomain R] [IsDiscreteValuationRing R]
    [Algebra R K] [IsFractionRing R K] (W : WeierstrassCurve K) :
    ∃ C : WeierstrassCurve.VariableChange K,
      WeierstrassCurve.IsMinimal R (C • W) :=
  WeierstrassCurve.exists_isMinimal R W

end JKSSTWeierstrassScaling
