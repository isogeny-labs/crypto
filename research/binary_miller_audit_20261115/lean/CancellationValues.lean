module
public import CancellationFunctions

@[expose] public section
namespace BinaryMillerAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine
open MillerPrincipalIdealAudit MillerFinitePointAudit MillerNormValuationAudit MillerRecursionAudit
open FunctionDivisorFinitenessAudit FinitePointEvaluationAudit
local instance : Fact (Nat.Prime 5) := ⟨by decide⟩
local instance : CommRing TestField := (inferInstance : Field TestField).toCommRing

def finalUnit : TestCurve.FunctionFieldˣ :=
  Units.mk0 (millerFunction TestCurve basePoint 3) (miller_function_nonzero TestCurve basePoint 3)

def badFinalUnit : supportUnits TestCurve badDivisor.support :=
  ⟨finalUnit, (disjoint_principal_support_iff TestCurve finalUnit badDivisor).mp final_disjoint_bad⟩

def safeFinalUnit : supportUnits TestCurve safeDivisor.support :=
  ⟨finalUnit, (disjoint_principal_support_iff TestCurve finalUnit safeDivisor).mp final_disjoint_safe⟩

theorem pole_mem_bad : polePoint ∈ badDivisor.support := by
  simp only [badDivisor, Finsupp.mem_support_iff, Finsupp.sub_apply, Finsupp.single_eq_same,
    Finsupp.single_eq_of_ne pole_ne_two, sub_zero, ne_eq, one_ne_zero, not_false_eq_true]

theorem two_mem_bad : twoTorsionPoint ∈ badDivisor.support := by
  simp only [badDivisor, Finsupp.mem_support_iff, Finsupp.sub_apply,
    Finsupp.single_eq_of_ne (Ne.symm pole_ne_two), Finsupp.single_eq_same, zero_sub,
    neg_ne_zero]
  norm_num

theorem safe_mem_safe : safePoint ∈ safeDivisor.support := by
  simp only [safeDivisor, Finsupp.mem_support_iff, Finsupp.sub_apply, Finsupp.single_eq_same,
    Finsupp.single_eq_of_ne safe_ne_two, sub_zero, ne_eq, one_ne_zero, not_false_eq_true]

theorem two_mem_safe : twoTorsionPoint ∈ safeDivisor.support := by
  simp only [safeDivisor, Finsupp.mem_support_iff, Finsupp.sub_apply,
    Finsupp.single_eq_of_ne (Ne.symm safe_ne_two), Finsupp.single_eq_same, zero_sub,
    neg_ne_zero]
  norm_num

/-- Values are derived from the actual local residue map and actual coordinate fraction. -/
theorem support_final_point_value (T : Finset TestCurve.Point) (f : supportUnits TestCurve T)
    (hf : ((f : TestCurve.FunctionFieldˣ) : TestCurve.FunctionField) = millerFunction TestCurve basePoint 3)
    (x y : TestField) (h : TestCurve.Nonsingular x y) (hP : Point.some x y h ∈ T) :
    (supportPointEvaluation TestCurve T (.some x y h) f : TestField) =
      pointEval TestCurve x y h.1 (lineClass TestCurve 0 1 0) := by
  have hff : (((supportPointMap TestCurve T (.some x y h) hP f :
      (RegularRing TestCurve (.some x y h)).unitGroup) : TestCurve.FunctionFieldˣ) : TestCurve.FunctionField) =
      algebraMap TestCurve.CoordinateRing TestCurve.FunctionField (lineClass TestCurve 0 1 0) /
        algebraMap TestCurve.CoordinateRing TestCurve.FunctionField 1 := by
    change ((f : TestCurve.FunctionFieldˣ) : TestCurve.FunctionField) = _
    rw [hf, third_miller_coordinate, map_one, div_one]
  have hv := point_unit_fraction_value TestCurve h (lineClass TestCurve 0 1 0) 1
    (by simp only [map_one, ne_eq, one_ne_zero, not_false_eq_true])
    (supportPointMap TestCurve T (.some x y h) hP f) hff
  rw [map_one, div_one] at hv
  simpa only [supportPointEvaluation, dite_eq_left hP, MonoidHom.comp_apply] using hv

theorem bad_pole_value :
    (supportPointEvaluation TestCurve badDivisor.support polePoint badFinalUnit : TestField) = 3 :=
  (support_final_point_value _ badFinalUnit rfl 0 4 pole_nonsingular pole_mem_bad).trans tangent_pole_value

theorem bad_two_value :
    (supportPointEvaluation TestCurve badDivisor.support twoTorsionPoint badFinalUnit : TestField) = 4 :=
  (support_final_point_value _ badFinalUnit rfl 4 0 control_two_nonsingular two_mem_bad).trans tangent_two_value

theorem safe_point_value :
    (supportPointEvaluation TestCurve safeDivisor.support safePoint safeFinalUnit : TestField) = 1 :=
  (support_final_point_value _ safeFinalUnit rfl 2 2 control_negative_nonsingular safe_mem_safe).trans tangent_safe_value

theorem safe_two_value :
    (supportPointEvaluation TestCurve safeDivisor.support twoTorsionPoint safeFinalUnit : TestField) = 4 :=
  (support_final_point_value _ safeFinalUnit rfl 4 0 control_two_nonsingular two_mem_safe).trans tangent_two_value

theorem divisor_difference_value (T : Finset TestCurve.Point) (A B : TestCurve.Point)
    (f : supportUnits TestCurve T) :
    divisorEvaluationOn TestCurve T (Finsupp.single A 1 - Finsupp.single B 1) f =
      supportPointEvaluation TestCurve T A f / supportPointEvaluation TestCurve T B f := by
  classical
  have h0 (P : TestCurve.Point) : (supportPointEvaluation TestCurve T P f) ^ (0 : ℤ) = 1 := zpow_zero _
  rw [divisor_evaluation_on_apply, sub_eq_add_neg,
    Finsupp.prod_add_index' h0 (fun _ _ _ => zpow_add _ _ _),
    Finsupp.prod_single_index (a := A) (b := (1 : ℤ))
      (h := fun P n => supportPointEvaluation TestCurve T P f ^ n) (h0 A),
    Finsupp.prod_neg_index h0,
    Finsupp.prod_single_index (a := B) (b := (1 : ℤ))
      (h := fun P n => supportPointEvaluation TestCurve T P f ^ (-n))
      (by simp only [neg_zero, zpow_zero])]
  simp only [zpow_one, zpow_neg_one, div_eq_mul_inv]

theorem final_bad_value :
    (evaluateDivisor TestCurve badDivisor finalUnit final_disjoint_bad : TestField) = 2 := by
  change (divisorEvaluationOn TestCurve badDivisor.support
    (Finsupp.single polePoint 1 - Finsupp.single twoTorsionPoint 1) badFinalUnit : TestField) = 2
  rw [divisor_difference_value, Units.val_div_eq_div_val, bad_pole_value, bad_two_value]
  decide +kernel

theorem final_safe_value :
    (evaluateDivisor TestCurve safeDivisor finalUnit final_disjoint_safe : TestField) = 4 := by
  change (divisorEvaluationOn TestCurve safeDivisor.support
    (Finsupp.single safePoint 1 - Finsupp.single twoTorsionPoint 1) safeFinalUnit : TestField) = 4
  rw [divisor_difference_value, Units.val_div_eq_div_val, safe_point_value, safe_two_value]
  decide +kernel

#print axioms support_final_point_value
#print axioms final_bad_value
#print axioms final_safe_value
end
end BinaryMillerAudit
