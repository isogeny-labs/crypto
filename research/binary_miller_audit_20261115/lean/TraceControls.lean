module
public import GuardedBinary

@[expose] public section
namespace BinaryMillerAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerPrincipalIdealAudit MillerNormValuationAudit MillerRecursionAudit
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit FinitePointEvaluationAudit
local instance : Fact (Nat.Prime 5) := ⟨by decide⟩
local instance : CommRing TestField := (inferInstance : Field TestField).toCommRing

theorem doubling_factor_fields :
    ((stepFactors TestCurve basePoint basePoint).1 : TestCurve.FunctionField) =
      MillerNormValuationAudit.lineFunction TestCurve 0 1 0 ∧
    ((stepFactors TestCurve basePoint basePoint).2 : TestCurve.FunctionField) = verticalFunction TestCurve 0 := by
  simp only [basePoint, stepFactors, test_slope, test_addX]
  exact ⟨rfl, rfl⟩

theorem final_factor_fields :
    ((stepFactors TestCurve polePoint basePoint).1 : TestCurve.FunctionField) = verticalFunction TestCurve 0 ∧
    ((stepFactors TestCurve polePoint basePoint).2 : TestCurve.FunctionField) = 1 := by
  simp only [polePoint, basePoint, stepFactors, pole_vertical]
  exact ⟨rfl, rfl⟩

theorem safe_doubling_admissible : FactorsAdmissible TestCurve safeDivisor basePoint basePoint := by
  constructor
  · have he := function_divisor_congr TestCurve (stepFactors TestCurve basePoint basePoint).1.ne_zero
      tangent_nonzero doubling_factor_fields.1
    rw [he]
    exact safe_tangent_disjoint
  · have he := function_divisor_congr TestCurve (stepFactors TestCurve basePoint basePoint).2.ne_zero
      (vertical_nonzero TestCurve 0) doubling_factor_fields.2
    rw [he]
    exact safe_vertical_disjoint

theorem safe_addition_admissible : FactorsAdmissible TestCurve safeDivisor polePoint basePoint := by
  constructor
  · have he := function_divisor_congr TestCurve (stepFactors TestCurve polePoint basePoint).1.ne_zero
      (vertical_nonzero TestCurve 0) final_factor_fields.1
    rw [he]
    exact safe_vertical_disjoint
  · have he := function_divisor_congr TestCurve (stepFactors TestCurve polePoint basePoint).2.ne_zero
      one_ne_zero final_factor_fields.2
    rw [he, function_divisor_one]
    simp

theorem bad_doubling_inadmissible : ¬ FactorsAdmissible TestCurve badDivisor basePoint basePoint := by
  intro h
  have he := function_divisor_congr TestCurve (stepFactors TestCurve basePoint basePoint).2.ne_zero
    (vertical_nonzero TestCurve 0) doubling_factor_fields.2
  have hd := h.2
  rw [he] at hd
  exact bad_vertical_not_disjoint hd

theorem one_point_state : (binaryState TestCurve basePoint 1).1 = basePoint := by
  rw [binary_state_small TestCurve basePoint 1 (by omega)]
  rfl

theorem base_double_add : basePoint + basePoint = polePoint := by
  simpa only [two_nsmul] using double_base

theorem safe_trace_admissible : TraceAdmissible TestCurve basePoint safeDivisor 3 := by
  rw [TraceAdmissible]
  change TraceAdmissible TestCurve basePoint safeDivisor 1 ∧
    FactorsAdmissible TestCurve safeDivisor (binaryState TestCurve basePoint 1).1
      (binaryState TestCurve basePoint 1).1 ∧
    ((3 : ℕ) % 2 ≠ 0 → FactorsAdmissible TestCurve safeDivisor
      ((binaryState TestCurve basePoint 1).1 + (binaryState TestCurve basePoint 1).1) basePoint)
  rw [one_point_state, base_double_add]
  exact ⟨by rw [TraceAdmissible]; trivial, safe_doubling_admissible, fun _ => safe_addition_admissible⟩

theorem bad_trace_inadmissible : ¬ TraceAdmissible TestCurve basePoint badDivisor 3 := by
  intro h
  rw [TraceAdmissible] at h
  change TraceAdmissible TestCurve basePoint badDivisor 1 ∧
    FactorsAdmissible TestCurve badDivisor (binaryState TestCurve basePoint 1).1
      (binaryState TestCurve basePoint 1).1 ∧ _ at h
  exact bad_doubling_inadmissible (by simpa only [one_point_state] using h.2.1)

#print axioms safe_trace_admissible
#print axioms bad_trace_inadmissible
end
end BinaryMillerAudit
