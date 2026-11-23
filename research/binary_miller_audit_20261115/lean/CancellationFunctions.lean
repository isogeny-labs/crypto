module
public import GeometricCancellation

@[expose] public section
namespace BinaryMillerAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine WeierstrassCurve.Affine.CoordinateRing
open MillerPrincipalIdealAudit MillerFinitePointAudit MillerNormValuationAudit
open MillerRecursionAudit FunctionDivisorFinitenessAudit
local instance : Fact (Nat.Prime 5) := ⟨by decide⟩
local instance : CommRing TestField := (inferInstance : Field TestField).toCommRing

theorem test_slope : TestCurve.slope 0 0 1 1 = 0 := by decide +kernel

theorem test_addX : TestCurve.addX 0 0 0 = 0 := by decide +kernel

theorem base_not_vertical : ¬ ((0 : TestField) = 0 ∧ (1 : TestField) = TestCurve.negY 0 1) :=
  by decide +kernel

theorem pole_vertical : (0 : TestField) = 0 ∧ (4 : TestField) = TestCurve.negY 0 1 :=
  by decide +kernel

theorem doubling_step_formula : stepFunction TestCurve basePoint basePoint =
    MillerNormValuationAudit.lineFunction TestCurve 0 1 0 / verticalFunction TestCurve 0 := by
  change (if (0 : TestField) = 0 ∧ (1 : TestField) = TestCurve.negY 0 1 then
    verticalFunction TestCurve 0 else
    MillerNormValuationAudit.lineFunction TestCurve 0 1 (TestCurve.slope 0 0 1 1) /
      verticalFunction TestCurve (TestCurve.addX 0 0 (TestCurve.slope 0 0 1 1))) = _
  split_ifs with h
  · exact False.elim (base_not_vertical h)
  · rw [test_slope, test_addX]

/-- Correct source sum-zero simplification is retained, rather than introducing an artificial 0/0. -/
theorem pole_base_step : stepFunction TestCurve polePoint basePoint = verticalFunction TestCurve 0 := by
  change (if (0 : TestField) = 0 ∧ (4 : TestField) = TestCurve.negY 0 1 then
    verticalFunction TestCurve 0 else
    MillerNormValuationAudit.lineFunction TestCurve 0 4 (TestCurve.slope 0 0 4 1) /
      verticalFunction TestCurve (TestCurve.addX 0 0 (TestCurve.slope 0 0 4 1))) = _
  split_ifs with h
  · rfl
  · exact False.elim (h pole_vertical)

theorem first_miller_one : millerFunction TestCurve basePoint 1 = 1 := by
  change (1 : TestCurve.FunctionField) * 1 = 1
  exact mul_one 1

theorem second_miller_formula : millerFunction TestCurve basePoint 2 =
    MillerNormValuationAudit.lineFunction TestCurve 0 1 0 / verticalFunction TestCurve 0 := by
  rw [millerFunction, one_nsmul, first_miller_one, one_mul, doubling_step_formula]

/-- Actual field cancellation proves the final chosen function is the normalized tangent. -/
theorem third_miller_line : millerFunction TestCurve basePoint 3 = MillerNormValuationAudit.lineFunction TestCurve 0 1 0 := by
  rw [millerFunction, double_base, pole_base_step, second_miller_formula]
  exact div_mul_cancel₀ _ (vertical_nonzero TestCurve 0)

theorem third_miller_coordinate : millerFunction TestCurve basePoint 3 =
    algebraMap TestCurve.CoordinateRing TestCurve.FunctionField (lineClass TestCurve 0 1 0) :=
  third_miller_line

theorem tangent_pole_value : pointEval TestCurve 0 4 pole_nonsingular.1 (lineClass TestCurve 0 1 0) = 3 := by
  rw [line_evaluation]
  decide +kernel

theorem tangent_two_value : pointEval TestCurve 4 0 control_two_nonsingular.1 (lineClass TestCurve 0 1 0) = 4 := by
  rw [line_evaluation]
  decide +kernel

theorem tangent_safe_value : pointEval TestCurve 2 2 control_negative_nonsingular.1 (lineClass TestCurve 0 1 0) = 1 := by
  rw [line_evaluation]
  decide +kernel

theorem vertical_pole_value : pointEval TestCurve 0 4 pole_nonsingular.1 (XClass TestCurve 0) = 0 := by
  rw [point_eval_XClass, sub_self]

theorem vertical_two_value : pointEval TestCurve 4 0 control_two_nonsingular.1 (XClass TestCurve 0) = 4 := by
  rw [point_eval_XClass, sub_zero]

theorem vertical_safe_value : pointEval TestCurve 2 2 control_negative_nonsingular.1 (XClass TestCurve 0) = 2 := by
  rw [point_eval_XClass, sub_zero]

theorem vertical_order_pole : pointOrder TestCurve (verticalFunction TestCurve 0) polePoint = 1 := by
  rw [vertical_point_order TestCurve control_positive_nonsingular]
  change verticalDivisor TestCurve basePoint polePoint = 1
  simp only [verticalDivisor, Finsupp.sub_apply, Finsupp.add_apply]
  rw [← pole_is_negative]
  simp only [Finsupp.single_eq_of_ne pole_ne_base, Finsupp.single_eq_of_ne pole_ne_zero,
    Finsupp.single_eq_same, zero_add, sub_zero]

#print axioms third_miller_line
#print axioms tangent_pole_value
#print axioms vertical_order_pole
end
end BinaryMillerAudit
