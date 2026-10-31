import Common22Domain

/-!
# A nonempty algebraic overlap prerequisite

The infinity chart's `z` coordinate is nonzero in its domain ring. Hence
the `z` basic open of its affine spectrum has a point (the zero prime).
This is a ring-level overlap prerequisite, not yet a proof that the actual
weighted-Proj curve is integral: its transport through the checked chart
isomorphisms and a topology theorem gluing the two irreducible opens remain
separate obligations.
-/

set_option autoImplicit false
noncomputable section

namespace WeightedProjIntegral

open Common22ExplicitChart

variable {K : Type*} [Field K]

theorem infinity_chart_z_ne_zero (a3 a2 a1 a0 : K) :
    zbar a3 a2 a1 a0 ≠ 0 := by
  intro hz
  have h := quotient_chart_relation a3 a2 a1 a0
  rw [hz, zero_mul] at h
  exact Common22Domain.chart_wbar_ne_zero a3 a2 a1 a0
    (sq_eq_zero_iff.mp h)

#print axioms infinity_chart_z_ne_zero

end WeightedProjIntegral
