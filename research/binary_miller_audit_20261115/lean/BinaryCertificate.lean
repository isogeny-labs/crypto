module
public import TraceControls

@[expose] public section
namespace BinaryMillerAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerPrincipalIdealAudit MillerNormValuationAudit MillerRecursionAudit MillerBasechangeAudit
open FunctionDivisorFinitenessAudit FinitePointEvaluationAudit MillerFinitePointAudit GeometricPrincipalDivisorAudit
local instance : Fact (Nat.Prime 5) := ⟨by decide⟩
local instance : CommRing TestField := (inferInstance : Field TestField).toCommRing
local instance : DecidableEq (AlgebraicClosure TestField) := Classical.decEq _

theorem binary_miller_certificate :
    (∀ n : ℕ, binaryRounds n = Nat.log2 n ∧ binaryAdditions n ≤ binaryRounds n) ∧
    (∀ (K : Type) [Field K] [DecidableEq K] (W : WeierstrassCurve.Affine K) (_ : W.Δ ≠ 0)
      (P : W.Point) (n : ℕ), binaryState W P n = (n • P, millerFunction W P n) ∧
      GeometricMiller W P n (binaryFunction W P n)) ∧
    (∀ (K : Type) [Field K] [DecidableEq K] (W : WeierstrassCurve.Affine K) (_ : W.Δ ≠ 0)
      (P : W.Point) (D : W.Point →₀ ℤ) (n : ℕ) (_ : TraceAdmissible W P D n),
      ∃ (R : W.Point) (f : supportUnits W D.support) (v : Kˣ),
        GuardedRun W P D n R f v ∧ R = n • P ∧
        ((f : W.FunctionFieldˣ) : W.FunctionField) = millerFunction W P n ∧ divisorEvaluation W D f = v) ∧
    TestCurve.Δ ≠ 0 ∧ addOrderOf basePoint = 3 ∧ (3 : ℕ) • basePoint = 0 ∧
    GeometricCurve.Δ ≠ 0 ∧ (3 : ℕ) • geometricPoint basePoint = 0 ∧
    divisorDegree TestCurve badDivisor = 0 ∧ divisorDegree TestCurve safeDivisor = 0 ∧
    divisorDegree GeometricCurve geometricBadDivisor = 0 ∧ divisorDegree GeometricCurve geometricSafeDivisor = 0 ∧
    Disjoint geometricBadDivisor.support (functionDivisor GeometricCurve geometricFunction geometric_function_nonzero).support ∧
    Disjoint geometricSafeDivisor.support (functionDivisor GeometricCurve geometricFunction geometric_function_nonzero).support ∧
    millerFunction TestCurve basePoint 3 = MillerNormValuationAudit.lineFunction TestCurve 0 1 0 ∧
    pointOrder TestCurve (millerFunction TestCurve basePoint 2) polePoint = -1 ∧
    pointOrder GeometricCurve
      (fieldBaseChange TestCurve (millerFunction TestCurve basePoint 2)) (geometricPoint polePoint) = -1 ∧
    (evaluateDivisor TestCurve badDivisor finalUnit final_disjoint_bad : TestField) = 2 ∧
    badCheckedTrace = none ∧ badRawTrace = 0 ∧
    badRawTrace ≠ (evaluateDivisor TestCurve badDivisor finalUnit final_disjoint_bad : TestField) ∧
    ¬ TraceAdmissible TestCurve basePoint badDivisor 3 ∧
    TraceAdmissible TestCurve basePoint safeDivisor 3 ∧
    (evaluateDivisor TestCurve safeDivisor finalUnit final_disjoint_safe : TestField) = 4 ∧
    safeCheckedTrace = some (evaluateDivisor TestCurve safeDivisor finalUnit final_disjoint_safe : TestField) ∧
    binaryRounds 3 = 1 ∧ binaryAdditions 3 = 1 := by
  refine ⟨?_, ?_, ?_, control_delta, base_order_three, base_three, geometric_delta, geometric_base_three,
    bad_degree_zero, safe_degree_zero, geometric_bad_degree_zero, geometric_safe_degree_zero,
    geometric_final_disjoint_bad, geometric_final_disjoint_safe, third_miller_line, intermediate_order_pole,
    geometric_intermediate_order_pole, final_bad_value, bad_checked_rejects, bad_raw_zero, bad_raw_wrong,
    bad_trace_inadmissible, safe_trace_admissible, final_safe_value, safe_checked_correct,
    three_rounds, three_additions⟩
  · intro n
    exact ⟨binary_rounds_eq_log2 n, binary_additions_le_rounds n⟩
  · intro K _ _ W hΔ P n
    exact ⟨binary_state_correct W hΔ P n, binary_geometric_miller W hΔ P n⟩
  · intro K _ _ W hΔ P D n hd
    obtain ⟨R, f, v, hr⟩ := admissible_run_exists W P D n hd
    exact ⟨R, f, v, hr, guarded_run_correct W hΔ P D hr⟩

#print axioms binary_miller_certificate
end
end BinaryMillerAudit
