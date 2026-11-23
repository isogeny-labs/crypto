module
public import CancellationValues

@[expose] public section
namespace BinaryMillerAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine WeierstrassCurve.Affine.CoordinateRing
open MillerPrincipalIdealAudit MillerFinitePointAudit MillerNormValuationAudit MillerRecursionAudit
open FunctionDivisorFinitenessAudit FinitePointEvaluationAudit

/-- One doubling quotient followed by the correctly simplified sum-zero vertical addition. -/
def totalizedThreeWord {K : Type*} [Field K] (lA lB vA vB : K) : K :=
  ((1 : K) ^ 2 * (lA / lB) / (vA / vB)) * (vA / vB)

/-- Both actual support-point values of both factors must be units, including negative coefficients. -/
def checkedThreeWord {K : Type*} [Field K] [DecidableEq K] (lA lB vA vB : K) : Option K :=
  if lA = 0 ∨ lB = 0 ∨ vA = 0 ∨ vB = 0 then none else some (totalizedThreeWord lA lB vA vB)

theorem checked_three_word_correct {K : Type*} [Field K] [DecidableEq K]
    (lA lB vA vB : K) (hlA : lA ≠ 0) (hlB : lB ≠ 0) (hvA : vA ≠ 0) (hvB : vB ≠ 0) :
    checkedThreeWord lA lB vA vB = some (lA / lB) := by
  unfold checkedThreeWord
  split_ifs with h
  · rcases h with h | h | h | h
    · exact False.elim (hlA h)
    · exact False.elim (hlB h)
    · exact False.elim (hvA h)
    · exact False.elim (hvB h)
  · apply congrArg some
    simpa only [totalizedThreeWord, one_pow, one_mul] using
      (div_mul_cancel₀ (lA / lB) (div_ne_zero hvA hvB))

local instance : Fact (Nat.Prime 5) := ⟨by decide⟩
local instance : CommRing TestField := (inferInstance : Field TestField).toCommRing

def badCheckedTrace : Option TestField := checkedThreeWord
  (pointEval TestCurve 0 4 pole_nonsingular.1 (lineClass TestCurve 0 1 0))
  (pointEval TestCurve 4 0 control_two_nonsingular.1 (lineClass TestCurve 0 1 0))
  (pointEval TestCurve 0 4 pole_nonsingular.1 (XClass TestCurve 0))
  (pointEval TestCurve 4 0 control_two_nonsingular.1 (XClass TestCurve 0))

def badRawTrace : TestField := totalizedThreeWord
  (pointEval TestCurve 0 4 pole_nonsingular.1 (lineClass TestCurve 0 1 0))
  (pointEval TestCurve 4 0 control_two_nonsingular.1 (lineClass TestCurve 0 1 0))
  (pointEval TestCurve 0 4 pole_nonsingular.1 (XClass TestCurve 0))
  (pointEval TestCurve 4 0 control_two_nonsingular.1 (XClass TestCurve 0))

def safeCheckedTrace : Option TestField := checkedThreeWord
  (pointEval TestCurve 2 2 control_negative_nonsingular.1 (lineClass TestCurve 0 1 0))
  (pointEval TestCurve 4 0 control_two_nonsingular.1 (lineClass TestCurve 0 1 0))
  (pointEval TestCurve 2 2 control_negative_nonsingular.1 (XClass TestCurve 0))
  (pointEval TestCurve 4 0 control_two_nonsingular.1 (XClass TestCurve 0))

theorem bad_checked_rejects : badCheckedTrace = none := by
  simp only [badCheckedTrace, tangent_pole_value, tangent_two_value, vertical_pole_value, vertical_two_value]
  decide +kernel

theorem bad_raw_zero : badRawTrace = 0 := by
  simp only [badRawTrace, tangent_pole_value, tangent_two_value, vertical_pole_value, vertical_two_value]
  decide +kernel

theorem bad_raw_wrong : badRawTrace ≠ (evaluateDivisor TestCurve badDivisor finalUnit final_disjoint_bad : TestField) := by
  rw [bad_raw_zero, final_bad_value]
  decide +kernel

theorem safe_checked_four : safeCheckedTrace = some 4 := by
  simp only [safeCheckedTrace, tangent_safe_value, tangent_two_value, vertical_safe_value, vertical_two_value]
  decide +kernel

theorem safe_checked_correct : safeCheckedTrace =
    some (evaluateDivisor TestCurve safeDivisor finalUnit final_disjoint_safe : TestField) := by
  rw [final_safe_value]
  exact safe_checked_four

theorem bad_vertical_not_disjoint : ¬ Disjoint badDivisor.support
    (functionDivisor TestCurve (verticalFunction TestCurve 0) (vertical_nonzero TestCurve 0)).support := by
  intro hd
  have hp : polePoint ∈ (functionDivisor TestCurve (verticalFunction TestCurve 0)
      (vertical_nonzero TestCurve 0)).support := by
    simp only [Finsupp.mem_support_iff, function_divisor_apply, vertical_order_pole,
      ne_eq, one_ne_zero, not_false_eq_true]
  exact Finset.disjoint_left.mp hd pole_mem_bad hp

theorem vertical_order_safe : pointOrder TestCurve (verticalFunction TestCurve 0) safePoint = 0 := by
  have hv := coordinate_nonvanishing_value TestCurve control_negative_nonsingular (XClass TestCurve 0)
    (by rw [vertical_safe_value]; decide +kernel)
  change -WithZero.log (finiteAdic TestCurve control_negative_nonsingular
    (algebraMap TestCurve.CoordinateRing TestCurve.FunctionField (XClass TestCurve 0))) = 0
  rw [hv, WithZero.log_one, neg_zero]

theorem vertical_order_two : pointOrder TestCurve (verticalFunction TestCurve 0) twoTorsionPoint = 0 := by
  have hv := coordinate_nonvanishing_value TestCurve control_two_nonsingular (XClass TestCurve 0)
    (by rw [vertical_two_value]; decide +kernel)
  change -WithZero.log (finiteAdic TestCurve control_two_nonsingular
    (algebraMap TestCurve.CoordinateRing TestCurve.FunctionField (XClass TestCurve 0))) = 0
  rw [hv, WithZero.log_one, neg_zero]

theorem safe_vertical_disjoint : Disjoint safeDivisor.support
    (functionDivisor TestCurve (verticalFunction TestCurve 0) (vertical_nonzero TestCurve 0)).support := by
  apply Finset.disjoint_left.mpr
  intro R hR hF
  have hc : R = safePoint ∨ R = twoTorsionPoint := support_pair hR
  have hz : pointOrder TestCurve (verticalFunction TestCurve 0) R = 0 := by
    rcases hc with rfl | rfl
    · exact vertical_order_safe
    · exact vertical_order_two
  have hn : pointOrder TestCurve (verticalFunction TestCurve 0) R ≠ 0 := by
    simpa only [Finsupp.mem_support_iff, function_divisor_apply] using hF
  exact hn hz

theorem tangent_nonzero : MillerNormValuationAudit.lineFunction TestCurve 0 1 0 ≠ 0 := by
  rw [← third_miller_line]
  exact miller_function_nonzero TestCurve basePoint 3

theorem safe_tangent_disjoint : Disjoint safeDivisor.support
    (functionDivisor TestCurve (MillerNormValuationAudit.lineFunction TestCurve 0 1 0) tangent_nonzero).support := by
  have he := GeometricPrincipalDivisorAudit.function_divisor_congr TestCurve tangent_nonzero
    (miller_function_nonzero TestCurve basePoint 3) third_miller_line.symm
  rw [he]
  exact final_disjoint_safe

#print axioms checked_three_word_correct
#print axioms bad_raw_wrong
#print axioms safe_checked_correct
#print axioms safe_vertical_disjoint
#print axioms safe_tangent_disjoint
end
end BinaryMillerAudit
