module
public import PointDivisor
public import GuardControls

@[expose] public section
namespace MillerFinitePointAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine WeierstrassCurve.Affine.CoordinateRing
open MillerPrincipalIdealAudit
local instance : Fact (Nat.Prime 5) := ⟨by decide⟩
local instance : CommRing F := (inferInstance : Field F).toCommRing

theorem control_two_nonsingular : controlCurve.Nonsingular 4 0 := by
  apply (equation_iff_nonsingular_of_Δ_ne_zero control_delta).mp
  rw [equation_iff]
  decide +kernel

theorem control_two_neg : controlCurve.negY 4 0 = 0 := by decide +kernel

/-- Exact actual adic vertical zero order at an actual two-torsion point. -/
theorem control_two_vertical_order :
    -WithZero.log (finiteAdic controlCurve control_two_nonsingular
      (MillerNormValuationAudit.verticalFunction controlCurve 4)) = (2 : ℤ) := by
  rw [finite_vertical_order controlCurve control_two_nonsingular control_two_nonsingular,
    control_two_neg]
  norm_num [coordinateMultiplicity]

/-- Deleting the two-torsion exception makes X-x_P fail as an actual local uniformizer. -/
theorem control_two_x_not_uniformizer :
    IsLocalRing.maximalIdeal (PointLocalRing controlCurve 4 0 control_two_nonsingular.1) ≠
      Ideal.span {localU controlCurve control_two_nonsingular.1} := by
  intro he
  have ho := finite_order_of_principal_power controlCurve control_two_nonsingular
    (localU controlCurve control_two_nonsingular.1) 1 (by simpa only [pow_one] using he.symm)
  have hf : pointToField controlCurve control_two_nonsingular.1
      (localU controlCurve control_two_nonsingular.1) =
      MillerNormValuationAudit.verticalFunction controlCurve 4 :=
    point_to_field_map controlCurve control_two_nonsingular.1 _
  rw [hf, control_two_vertical_order] at ho
  norm_num at ho

theorem control_negative_third_coordinates :
    controlCurve.addX 2 2 (controlCurve.slope 2 2 2 2) = 0 ∧
    controlCurve.negY (controlCurve.addX 2 2 (controlCurve.slope 2 2 2 2))
      (controlCurve.addY 2 2 2 (controlCurve.slope 2 2 2 2)) = 1 := by decide +kernel

/-- The ordinary tangent at (2,2) has actual order two, not three. -/
theorem control_negative_tangent_order :
    -WithZero.log (finiteAdic controlCurve control_negative_nonsingular
      (MillerNormValuationAudit.lineFunction controlCurve 2 2 (controlCurve.slope 2 2 2 2))) =
      (2 : ℤ) := by
  rw [finite_chord_order controlCurve control_negative_nonsingular control_negative_nonsingular
    control_negative_nonsingular (fun h => control_negative_nonvertical h.2)]
  unfold chordMultiplicity
  rw [control_negative_third_coordinates.2, control_negative_third_coordinates.1]
  norm_num [coordinateMultiplicity]; decide +kernel

/-- The same tangent has an actual additional simple zero at (0,1). -/
theorem control_negative_other_zero :
    -WithZero.log (finiteAdic controlCurve control_positive_nonsingular
      (MillerNormValuationAudit.lineFunction controlCurve 2 2 (controlCurve.slope 2 2 2 2))) =
      (1 : ℤ) := by
  rw [finite_chord_order controlCurve control_positive_nonsingular control_negative_nonsingular
    control_negative_nonsingular (fun h => control_negative_nonvertical h.2)]
  unfold chordMultiplicity
  rw [control_negative_third_coordinates.2, control_negative_third_coordinates.1]
  norm_num [coordinateMultiplicity]; decide +kernel

/-- Genuine actual principal-divisor counterexample to dropping the three-torsion hypothesis. -/
theorem control_negative_tangent_divisor :
    principalPointDivisor controlCurve
      (MillerNormValuationAudit.lineFunction controlCurve 2 2 (controlCurve.slope 2 2 2 2))
      (chord_support_finite controlCurve control_negative_nonsingular control_negative_nonsingular
        (fun h => control_negative_nonvertical h.2)) ≠
      Finsupp.single (Point.some 2 2 control_negative_nonsingular) 3 - Finsupp.single 0 3 := by
  intro he
  have hv := congrArg (fun D : controlCurve.Point →₀ ℤ =>
    D (Point.some 0 1 control_positive_nonsingular)) he
  change -WithZero.log (finiteAdic controlCurve control_positive_nonsingular
    (MillerNormValuationAudit.lineFunction controlCurve 2 2 (controlCurve.slope 2 2 2 2))) = _ at hv
  rw [control_negative_other_zero] at hv
  have hne : (Point.some 2 2 control_negative_nonsingular : controlCurve.Point) ≠
      Point.some 0 1 control_positive_nonsingular := by decide +kernel
  simp [hne, eq_comm] at hv

#print axioms control_two_x_not_uniformizer
#print axioms control_negative_tangent_divisor
end
end MillerFinitePointAudit
