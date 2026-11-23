module
public import LevelGeometry

@[expose] public section
namespace MillerLevelCompatibilityAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine WeierstrassCurve.Affine.CoordinateRing
open BinaryMillerAudit MillerPrincipalIdealAudit MillerFinitePointAudit MillerNormValuationAudit
open MillerRecursionAudit FunctionDivisorFinitenessAudit FinitePointEvaluationAudit
local instance : Fact (Nat.Prime 5) := ⟨by decide⟩
local instance : CommRing TestField := (inferInstance : Field TestField).toCommRing

theorem field_card_five : Fintype.card TestField = 5 := by decide +kernel

theorem first_mem_difference (A B : TestCurve.Point) (h : A ≠ B) :
    A ∈ (Finsupp.single A (1 : ℤ) - Finsupp.single B 1).support := by
  rw [Finsupp.mem_support_iff, Finsupp.sub_apply, Finsupp.single_eq_same,
    Finsupp.single_eq_of_ne h, sub_zero]
  exact one_ne_zero

theorem second_mem_difference (A B : TestCurve.Point) (h : A ≠ B) :
    B ∈ (Finsupp.single A (1 : ℤ) - Finsupp.single B 1).support := by
  rw [Finsupp.mem_support_iff, Finsupp.sub_apply, Finsupp.single_eq_of_ne (Ne.symm h),
    Finsupp.single_eq_same, zero_sub]
  exact neg_ne_zero.mpr one_ne_zero

/-- Actual residue evaluation of an actual coordinate fraction on the actual unit domain. -/
theorem support_fraction_value (T : Finset TestCurve.Point) (f : supportUnits TestCurve T)
    (x y : TestField) (h : TestCurve.Nonsingular x y) (hP : Point.some x y h ∈ T)
    (a b : TestCurve.CoordinateRing) (hb : pointEval TestCurve x y h.1 b ≠ 0)
    (hf : ((f : TestCurve.FunctionFieldˣ) : TestCurve.FunctionField) =
      algebraMap TestCurve.CoordinateRing TestCurve.FunctionField a /
        algebraMap TestCurve.CoordinateRing TestCurve.FunctionField b) :
    (supportPointEvaluation TestCurve T (.some x y h) f : TestField) =
      pointEval TestCurve x y h.1 a / pointEval TestCurve x y h.1 b := by
  have hv := point_unit_fraction_value TestCurve h a b hb
    (supportPointMap TestCurve T (.some x y h) hP f) hf
  simpa only [supportPointEvaluation, dite_eq_left hP, MonoidHom.comp_apply] using hv

theorem positive_value_two :
    (evaluateDivisor TestCurve positiveDivisor (millerUnit TestCurve twoTorsionPoint 2) positive_domain_two : TestField) = 2 := by
  let f := finalLift TestCurve twoTorsionPoint 2 positiveDivisor positive_domain_two
  have hA : basePoint ∈ positiveDivisor.support := first_mem_difference basePoint safePoint (by decide +kernel)
  have hB : safePoint ∈ positiveDivisor.support := second_mem_difference basePoint safePoint (by decide +kernel)
  have hf : ((f : TestCurve.FunctionFieldˣ) : TestCurve.FunctionField) =
      algebraMap TestCurve.CoordinateRing TestCurve.FunctionField (XClass TestCurve 4) /
        algebraMap TestCurve.CoordinateRing TestCurve.FunctionField 1 := by
    change millerFunction TestCurve twoTorsionPoint 2 = _
    simpa only [map_one, div_one, MillerNormValuationAudit.verticalFunction] using positive_function_two
  have hvA := support_fraction_value _ f 0 1 control_positive_nonsingular hA (XClass TestCurve 4) 1
    (by simp only [map_one, ne_eq, one_ne_zero, not_false_eq_true]) hf
  have hvB := support_fraction_value _ f 2 2 control_negative_nonsingular hB (XClass TestCurve 4) 1
    (by simp only [map_one, ne_eq, one_ne_zero, not_false_eq_true]) hf
  have ha : (supportPointEvaluation TestCurve positiveDivisor.support basePoint f : TestField) = 1 :=
    hvA.trans (by rw [point_eval_XClass, map_one]; decide +kernel)
  have hb : (supportPointEvaluation TestCurve positiveDivisor.support safePoint f : TestField) = 3 :=
    hvB.trans (by rw [point_eval_XClass, map_one]; decide +kernel)
  change (divisorEvaluationOn TestCurve positiveDivisor.support
    (Finsupp.single basePoint 1 - Finsupp.single safePoint 1) f : TestField) = 2
  rw [divisor_difference_value, Units.val_div_eq_div_val, ha, hb]
  decide +kernel

theorem negative_value_two :
    (evaluateDivisor TestCurve negativeDivisor (millerUnit TestCurve basePoint 2) negative_domain_two : TestField) = 1 := by
  let f := finalLift TestCurve basePoint 2 negativeDivisor negative_domain_two
  have hA : point23 ∈ negativeDivisor.support := first_mem_difference point23 twoTorsionPoint (by decide +kernel)
  have hB : twoTorsionPoint ∈ negativeDivisor.support := second_mem_difference point23 twoTorsionPoint (by decide +kernel)
  have hf : ((f : TestCurve.FunctionFieldˣ) : TestCurve.FunctionField) =
      algebraMap TestCurve.CoordinateRing TestCurve.FunctionField (lineClass TestCurve 0 1 0) /
        algebraMap TestCurve.CoordinateRing TestCurve.FunctionField (XClass TestCurve 0) := by
    change millerFunction TestCurve basePoint 2 = _
    exact second_miller_formula
  have hvA := support_fraction_value _ f 2 3 point23_nonsingular hA (lineClass TestCurve 0 1 0) (XClass TestCurve 0)
    (by rw [point_eval_XClass]; decide +kernel) hf
  have hvB := support_fraction_value _ f 4 0 control_two_nonsingular hB (lineClass TestCurve 0 1 0) (XClass TestCurve 0)
    (by rw [point_eval_XClass]; decide +kernel) hf
  have ha : (supportPointEvaluation TestCurve negativeDivisor.support point23 f : TestField) = 1 :=
    hvA.trans (by rw [line_evaluation, point_eval_XClass]; decide +kernel)
  have hb : (supportPointEvaluation TestCurve negativeDivisor.support twoTorsionPoint f : TestField) = 1 :=
    hvB.trans (by rw [line_evaluation, point_eval_XClass]; decide +kernel)
  change (divisorEvaluationOn TestCurve negativeDivisor.support
    (Finsupp.single point23 1 - Finsupp.single twoTorsionPoint 1) f : TestField) = 1
  rw [divisor_difference_value, Units.val_div_eq_div_val, ha, hb]
  decide +kernel

theorem negative_value_four :
    (evaluateDivisor TestCurve negativeDivisor (millerUnit TestCurve basePoint 4) negative_domain_four : TestField) = 3 := by
  let f := finalLift TestCurve basePoint 4 negativeDivisor negative_domain_four
  have hA : point23 ∈ negativeDivisor.support := first_mem_difference point23 twoTorsionPoint (by decide +kernel)
  have hB : twoTorsionPoint ∈ negativeDivisor.support := second_mem_difference point23 twoTorsionPoint (by decide +kernel)
  have hf : ((f : TestCurve.FunctionFieldˣ) : TestCurve.FunctionField) =
      algebraMap TestCurve.CoordinateRing TestCurve.FunctionField (lineClass TestCurve 0 1 0) /
        algebraMap TestCurve.CoordinateRing TestCurve.FunctionField 1 := by
    change millerFunction TestCurve basePoint 4 = _
    simpa only [map_one, div_one, MillerNormValuationAudit.lineFunction, lineClass] using negative_function_four
  have hvA := support_fraction_value _ f 2 3 point23_nonsingular hA (lineClass TestCurve 0 1 0) 1
    (by simp only [map_one, ne_eq, one_ne_zero, not_false_eq_true]) hf
  have hvB := support_fraction_value _ f 4 0 control_two_nonsingular hB (lineClass TestCurve 0 1 0) 1
    (by simp only [map_one, ne_eq, one_ne_zero, not_false_eq_true]) hf
  have ha : (supportPointEvaluation TestCurve negativeDivisor.support point23 f : TestField) = 2 :=
    hvA.trans (by rw [line_evaluation, map_one]; decide +kernel)
  have hb : (supportPointEvaluation TestCurve negativeDivisor.support twoTorsionPoint f : TestField) = 4 :=
    hvB.trans (by rw [line_evaluation, map_one]; decide +kernel)
  change (divisorEvaluationOn TestCurve negativeDivisor.support
    (Finsupp.single point23 1 - Finsupp.single twoTorsionPoint 1) f : TestField) = 3
  rw [divisor_difference_value, Units.val_div_eq_div_val, ha, hb]
  decide +kernel

theorem positive_value_four :
    (evaluateDivisor TestCurve positiveDivisor (millerUnit TestCurve twoTorsionPoint 4) positive_domain_four : TestField) = 4 := by
  have hv := congrArg (fun u : TestFieldˣ => (u : TestField))
    (value_divisible TestCurve control_delta twoTorsionPoint 2 4 two_point_double (by decide) positiveDivisor positive_domain_two)
  change (evaluateDivisor TestCurve positiveDivisor (millerUnit TestCurve twoTorsionPoint 4) positive_domain_four : TestField) =
    (evaluateDivisor TestCurve positiveDivisor (millerUnit TestCurve twoTorsionPoint 2) positive_domain_two : TestField) ^ 2 at hv
  rw [positive_value_two] at hv
  exact hv.trans (by decide +kernel)

theorem positive_final_compatibility :
    (evaluateDivisor TestCurve positiveDivisor (millerUnit TestCurve twoTorsionPoint 2) positive_domain_two : TestField) ^ (4 / 2 : ℕ) =
    (evaluateDivisor TestCurve positiveDivisor (millerUnit TestCurve twoTorsionPoint 4) positive_domain_four : TestField) ^ (4 / 4 : ℕ) := by
  rw [positive_value_two, positive_value_four]
  decide +kernel

theorem negative_final_failure :
    (evaluateDivisor TestCurve negativeDivisor (millerUnit TestCurve basePoint 2) negative_domain_two : TestField) ^ (4 / 2 : ℕ) ≠
    (evaluateDivisor TestCurve negativeDivisor (millerUnit TestCurve basePoint 4) negative_domain_four : TestField) ^ (4 / 4 : ℕ) := by
  rw [negative_value_two, negative_value_four]
  decide +kernel

#print axioms positive_value_two
#print axioms negative_value_two
#print axioms negative_value_four
#print axioms positive_final_compatibility
#print axioms negative_final_failure
end
end MillerLevelCompatibilityAudit
