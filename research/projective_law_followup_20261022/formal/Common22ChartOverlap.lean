import Common22InfinityChart

/-! The two algebraic curve charts agree on their nonzero-coordinate overlap.
This is a field-level identity, not yet an isomorphism of affine schemes or a
construction of a smooth projective curve. -/

set_option autoImplicit false

namespace Common22ChartOverlap

variable {K : Type*} [Field K]

theorem infinity_to_affine
    (a3 a2 a1 a0 z w : K) (hz : z ≠ 0)
    (hchart : w ^ 2 = Common22InfinityChart.chartRight a3 a2 a1 a0 z) :
    (w * z⁻¹ ^ 3) ^ 2 =
      z⁻¹ ^ 5 + a3 * z⁻¹ ^ 3 + a2 * z⁻¹ ^ 2 + a1 * z⁻¹ + a0 := by
  calc
    (w * z⁻¹ ^ 3) ^ 2 = w ^ 2 * z⁻¹ ^ 6 := by ring
    _ = Common22InfinityChart.chartRight a3 a2 a1 a0 z * z⁻¹ ^ 6 := by rw [hchart]
    _ = _ := by
      dsimp [Common22InfinityChart.chartRight]
      field_simp [hz]

theorem x_round_trip (x : K) : (x⁻¹)⁻¹ = x := by
  exact inv_inv x

theorem y_round_trip (x y : K) (hx : x ≠ 0) :
    (y * x⁻¹ ^ 3) * (x⁻¹)⁻¹ ^ 3 = y := by
  rw [inv_inv]
  field_simp [hx]

theorem z_round_trip (z : K) : (z⁻¹)⁻¹ = z := by
  exact inv_inv z

theorem w_round_trip (z w : K) (hz : z ≠ 0) :
    (w * z⁻¹ ^ 3) * (z⁻¹)⁻¹ ^ 3 = w := by
  rw [inv_inv]
  field_simp [hz]

theorem affine_chart_equation_iff
    (a3 a2 a1 a0 x y : K) (hx : x ≠ 0) :
    y ^ 2 = x ^ 5 + a3 * x ^ 3 + a2 * x ^ 2 + a1 * x + a0 ↔
      (y * x⁻¹ ^ 3) ^ 2 =
        Common22InfinityChart.chartRight a3 a2 a1 a0 x⁻¹ := by
  constructor
  · exact Common22InfinityChart.curve_chart_equation a3 a2 a1 a0 x y hx
  · intro hchart
    have h := infinity_to_affine a3 a2 a1 a0 x⁻¹ (y * x⁻¹ ^ 3)
      (inv_ne_zero hx) hchart
    rw [y_round_trip x y hx] at h
    simpa only [inv_inv] using h

#print axioms infinity_to_affine
#print axioms x_round_trip
#print axioms y_round_trip
#print axioms z_round_trip
#print axioms w_round_trip
#print axioms affine_chart_equation_iff

end Common22ChartOverlap
