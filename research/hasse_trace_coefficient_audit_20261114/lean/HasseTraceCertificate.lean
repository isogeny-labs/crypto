import HasseTraceControls

namespace HasseTraceCoefficientAudit
noncomputable section
open scoped BigOperators
open QuadraticTwistCountAudit

theorem actual_hasse_trace_certificate :
    (∀ (K : Type*) [Field K] [Fintype K] [DecidableEq K] (f : Polynomial K),
      f.natDegree < 2 * (Fintype.card K - 1) →
        (∑ x : K, f.eval x) = -f.coeff (Fintype.card K - 1)) ∧
    (∀ (K : Type*) [Field K] [Fintype K] [DecidableEq K] (a b c : K),
      ringChar K ≠ 2 → (cubicModel a b c).Δ ≠ 0 →
      (Nat.card (cubicModel a b c).Point : K) = 1 - hasseCoefficient a b c ∧
      (SupersingularTraceClassesAudit.fieldTrace (cubicModel a b c) : K) =
        hasseCoefficient a b c ∧
      (hasseCoefficient a b c = 0 ↔
        (ringChar K : ℤ) ∣ SupersingularTraceClassesAudit.fieldTrace (cubicModel a b c))) ∧
    (∀ (p : ℕ) [Fact p.Prime] (a b c : ZMod p), p ≠ 2 →
      (cubicModel a b c).Δ ≠ 0 →
      (SupersingularTraceClassesAudit.fieldTrace (cubicModel a b c) : ZMod p) =
        ((cubicPolynomial a b c) ^ ((p - 1) / 2)).coeff (p - 1) ∧
      (((cubicPolynomial a b c) ^ ((p - 1) / 2)).coeff (p - 1) = 0 ↔
        (p : ℤ) ∣ SupersingularTraceClassesAudit.fieldTrace (cubicModel a b c))) ∧
    (curve13a.Δ ≠ 0 ∧ curve13b.Δ ≠ 0 ∧
      Nat.card curve13a.Point = 8 ∧ Nat.card curve13b.Point = 21 ∧
      SupersingularTraceClassesAudit.fieldTrace curve13a = 6 ∧
      SupersingularTraceClassesAudit.fieldTrace curve13b = -7 ∧
      hasseCoefficient (0 : ZMod 13) (-1) 0 = 6 ∧
      hasseCoefficient (0 : ZMod 13) 0 4 = 6 ∧
      (¬∀ a b c a' b' c' : ZMod 13,
        (cubicModel a b c).Δ ≠ 0 → (cubicModel a' b' c').Δ ≠ 0 →
        hasseCoefficient a b c = hasseCoefficient a' b' c' →
          Nat.card (cubicModel a b c).Point = Nat.card (cubicModel a' b' c').Point)) ∧
    (∃ a : F9, (cubicModel a 0 1).Δ ≠ 0 ∧ characteristicCoefficient a 0 1 = a ∧
      hasseCoefficient a 0 1 = 1 ∧
      (SupersingularTraceClassesAudit.fieldTrace (cubicModel a 0 1) : F9) = 1 ∧
      characteristicCoefficient a 0 1 ≠
        (SupersingularTraceClassesAudit.fieldTrace (cubicModel a 0 1) : F9)) ∧
    ((rootCard (1 : ZMod 2) : ZMod 2) ≠
      1 + (1 : ZMod 2) ^ ((Fintype.card (ZMod 2) - 1) / 2)) ∧
    (¬∀ f : Polynomial (ZMod 3), f.natDegree ≤ 2 * (Fintype.card (ZMod 3) - 1) →
      (∑ x : ZMod 3, f.eval x) = -f.coeff (Fintype.card (ZMod 3) - 1)) := by
  refine ⟨?_, ?_, ?_, ?_, field9_control, char_two_euler_false, nonstrict_degree_bound_false⟩
  · intro K _ _ _ f hf
    exact eval_sum_coefficient f hf
  · intro K _ _ _ a b c hF hΔ
    exact ⟨actual_point_count_residue hF a b c hΔ, actual_trace_residue hF a b c hΔ,
      coefficient_zero_iff_char_divides hF a b c hΔ⟩
  · intro p _ a b c hp hΔ
    exact ⟨prime_trace_residue p hp a b c hΔ,
      prime_coefficient_zero_iff_trace_divisible p hp a b c hΔ⟩
  · exact ⟨fixture13a.1, fixture13b.1, actual_counts13.1, actual_counts13.2,
      actual_traces13.1, actual_traces13.2, coefficients13.1, coefficients13.2,
      prime13_uniqueness_false⟩

#print axioms actual_hasse_trace_certificate
end
end HasseTraceCoefficientAudit
