import Common22InfinityChart
import Mathlib.RingTheory.DiscreteValuationRing.Basic

/-!
# Conditional local-ring step for the common 2+2 infinity chart

This packet begins after a smooth projective completion and its local DVR have
been supplied. It does not construct that completion or identify its germ.
-/

set_option autoImplicit false

namespace Common22LocalValuation

variable {R : Type*} [CommRing R] [IsDomain R] [IsDiscreteValuationRing R]

private theorem unit_add_nonunit {a b : R} (ha : IsUnit a) (hb : b ∈ nonunits R) :
    IsUnit (a + b) := by
  by_contra h
  have hs : a + b ∈ nonunits R := h
  have hneg : -b ∈ nonunits R := by
    exact fun h => hb (by simpa using h.neg)
  have hx := IsLocalRing.nonunits_add hs hneg
  have : a ∈ nonunits R := by simpa only [add_neg_cancel_right] using hx
  exact this ha

/-- In a DVR, the local equation `w²=z*u` and a uniformizer `w` force
`ord(z)=2`. The unit `u` and the existence of the DVR remain hypotheses. -/
theorem chart_z_order (w z : R) (u : Rˣ) (hw : Irreducible w)
    (heq : w ^ 2 = z * (u : R)) :
    IsDiscreteValuationRing.addVal R z = 2 := by
  have h := congrArg (IsDiscreteValuationRing.addVal R) heq
  rw [IsDiscreteValuationRing.addVal_pow,
    IsDiscreteValuationRing.addVal_uniformizer hw,
    IsDiscreteValuationRing.addVal_mul,
    IsDiscreteValuationRing.addVal_eq_zero_iff.mpr u.isUnit] at h
  simpa using h.symm

/-- A nonzero constant residue in the chart gives a unit, provided the
chart coordinates are in the maximal ideal. -/
theorem nonzero_residue_unit (c z w t b : R) (hc : IsUnit c)
    (hz : z ∈ nonunits R) (hw : w ∈ nonunits R) :
    IsUnit (-c + z * t - b * w) := by
  have hzt : z * t ∈ nonunits R := by
    exact fun h => hz (isUnit_of_mul_isUnit_left h)
  have hbw : b * w ∈ nonunits R := by
    exact fun h => hw (isUnit_of_mul_isUnit_right h)
  have hneg : -(b * w) ∈ nonunits R :=
    fun h => hbw (by simpa only [neg_neg] using h.neg)
  have hrest : z * t - b * w ∈ nonunits R := by
    simpa only [sub_eq_add_neg] using IsLocalRing.nonunits_add hzt hneg
  convert unit_add_nonunit hc.neg hrest using 1 <;> ring

/-- If the cubic leading coefficient vanishes, the chart expression has
order one as long as `b` is a unit. This is still conditional on identifying
the actual projective local ring as a DVR with uniformizer `w`. -/
theorem zero_residue_order_one (z w t : R) (u b : Rˣ)
    (hw : Irreducible w) (heq : w ^ 2 = z * (u : R)) :
    IsDiscreteValuationRing.addVal R (z * t - (b : R) * w) = 1 := by
  have hz : z = w ^ 2 * (↑(u⁻¹) : R) := by
    calc
      z = (z * (u : R)) * (↑(u⁻¹) : R) := by simp [mul_assoc]
      _ = w ^ 2 * (↑(u⁻¹) : R) := by rw [← heq]
  have hwu : w * (↑(u⁻¹) : R) * t ∈ nonunits R :=
    fun h => hw.not_isUnit (isUnit_of_mul_isUnit_left (isUnit_of_mul_isUnit_left h))
  have hv : IsUnit (-(b : R) + w * (↑(u⁻¹) : R) * t) :=
    unit_add_nonunit b.isUnit.neg hwu
  have hfactor : z * t - (b : R) * w =
      w * (-(b : R) + w * (↑(u⁻¹) : R) * t) := by
    rw [hz]
    ring
  rw [hfactor, IsDiscreteValuationRing.addVal_mul,
    IsDiscreteValuationRing.addVal_uniformizer hw,
    IsDiscreteValuationRing.addVal_eq_zero_iff.mpr hv]
  simp

/-- The exact chart-polynomial shape has order zero when `c` is a unit.
The `z,w` maximal-ideal hypotheses are the missing geometric specialization. -/
theorem chart_interpolant_order_zero (c z w n2 n1 n0 b : R)
    (hc : IsUnit c) (hz : z ∈ nonunits R) (hw : w ∈ nonunits R) :
    IsDiscreteValuationRing.addVal R
      (-c + n2 * z + n1 * z ^ 2 + n0 * z ^ 3 - b * w) = 0 := by
  have hunit := nonzero_residue_unit c z w
    (n2 + n1 * z + n0 * z ^ 2) b hc hz hw
  have heq : -c + n2 * z + n1 * z ^ 2 + n0 * z ^ 3 - b * w =
      -c + z * (n2 + n1 * z + n0 * z ^ 2) - b * w := by ring
  rw [heq]
  exact IsDiscreteValuationRing.addVal_eq_zero_iff.mpr hunit

/-- The exact chart-polynomial shape has order one in the `c=0` branch,
provided the cleared `b` is a unit. -/
theorem chart_interpolant_order_one (z w n2 n1 n0 : R) (u b : Rˣ)
    (hw : Irreducible w) (heq : w ^ 2 = z * (u : R)) :
    IsDiscreteValuationRing.addVal R
      (n2 * z + n1 * z ^ 2 + n0 * z ^ 3 - (b : R) * w) = 1 := by
  have heq' : n2 * z + n1 * z ^ 2 + n0 * z ^ 3 - (b : R) * w =
      z * (n2 + n1 * z + n0 * z ^ 2) - (b : R) * w := by ring
  rw [heq']
  exact zero_residue_order_one z w _ u b hw heq

end Common22LocalValuation

#print axioms Common22LocalValuation.chart_z_order
#print axioms Common22LocalValuation.nonzero_residue_unit
#print axioms Common22LocalValuation.zero_residue_order_one
#print axioms Common22LocalValuation.chart_interpolant_order_zero
#print axioms Common22LocalValuation.chart_interpolant_order_one
