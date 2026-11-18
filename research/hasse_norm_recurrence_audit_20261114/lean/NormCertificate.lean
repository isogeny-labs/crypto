import NormControls

namespace HasseNormRecurrenceAudit
noncomputable section
open Polynomial Finset HasseTraceCoefficientAudit QuadraticTwistCountAudit

/-- Complete source recurrence, actual trace bridge and exact guard controls. -/
theorem hasse_norm_recurrence_certificate :
    (∀ (K : Type) [Field K] (A B : Polynomial K) (σ : K →+* K) (M n : ℕ),
      0 < M → A.natDegree < 2 * M - 1 →
      (A * (expand K M B).map σ).coeff (M * n + (M - 1)) =
        A.coeff (M - 1) * σ (B.coeff n)) ∧
    (∀ (K : Type) [Field K] (p : ℕ) [Fact (Nat.Prime p)] [CharP K p],
      p ≠ 2 → ∀ f : Polynomial K, f.natDegree ≤ 3 → ∀ r : ℕ,
      coefficientAt f (p ^ (r + 1)) =
        coefficientAt f (p ^ r) * coefficientAt f p ^ (p ^ r) ∧
      coefficientAt f (p ^ r) = coefficientAt f p ^ (∑ i ∈ range r, p ^ i) ∧
      (0 < r → (coefficientAt f (p ^ r) = 0 ↔ coefficientAt f p = 0))) ∧
    (∀ (K : Type) [Field K] [Fintype K] [DecidableEq K]
      (p : ℕ) [Fact (Nat.Prime p)] [CharP K p], p ≠ 2 →
      ∀ r : ℕ, 0 < r → Fintype.card K = p ^ r → ∀ a b c : K,
      (cubicModel a b c).Δ ≠ 0 →
      (SupersingularTraceClassesAudit.fieldTrace (cubicModel a b c) : K) =
        coefficientAt (cubicPolynomial a b c) p ^ (∑ i ∈ range r, p ^ i) ∧
      ((p : ℤ) ∣ SupersingularTraceClassesAudit.fieldTrace (cubicModel a b c) ↔
        coefficientAt (cubicPolynomial a b c) p = 0)) ∧
    (coefficientAt (X ^ 3 : Polynomial (ZMod 2)) 2 = 0 ∧
      coefficientAt (X ^ 3 : Polynomial (ZMod 2)) 4 = 1) ∧
    (¬∀ f : Polynomial (ZMod 2), f.natDegree ≤ 3 →
      coefficientAt f (2 ^ (1 + 1)) = coefficientAt f (2 ^ 1) *
        coefficientAt f 2 ^ (2 ^ 1)) ∧
    (¬∀ f : Polynomial (ZMod 3), f.natDegree ≤ 3 →
      (coefficientAt f (3 ^ 0) = 0 ↔ coefficientAt f 3 = 0)) ∧
    (coefficientAt (X + X ^ 5 : Polynomial (ZMod 3)) 3 = 0 ∧
      coefficientAt (X + X ^ 5 : Polynomial (ZMod 3)) 9 = 1) ∧
    (¬∀ f : Polynomial (ZMod 3), coefficientAt f (3 ^ (1 + 1)) =
      coefficientAt f (3 ^ 1) * coefficientAt f 3 ^ (3 ^ 1)) ∧
    (∃ a : F9, (cubicModel a 0 1).Δ ≠ 0 ∧
      coefficientAt (cubicPolynomial a 0 1) 3 = a ∧
      coefficientAt (cubicPolynomial a 0 1) 9 = a ^ 4 ∧ a ^ 4 = 1 ∧
      (SupersingularTraceClassesAudit.fieldTrace (cubicModel a 0 1) : F9) = a ^ 4 ∧
      coefficientAt (cubicPolynomial a 0 1) 3 ≠
        coefficientAt (cubicPolynomial a 0 1) 9) := by
  refine ⟨?_, ?_, ?_, characteristic_two_fixture, characteristic_two_recurrence_false,
    exponent_zero_vanishing_false, high_degree_fixture,
    unrestricted_degree_recurrence_false, field9_norm_control⟩
  · intro K _ A B σ M n hM hA
    exact sparse_product_coefficient A B σ M n hM hA
  · intro K _ p _ _ hp f hf r
    exact ⟨coefficient_recurrence p hp f hf r, coefficient_norm p hp f hf r,
      coefficient_zero_iff p hp f hf r⟩
  · intro K _ _ _ p _ _ hp r hr hq a b c hΔ
    exact ⟨actual_trace_norm p hp r hq a b c hΔ,
      actual_trace_divisibility p hp r hr hq a b c hΔ⟩

#print axioms hasse_norm_recurrence_certificate
end
end HasseNormRecurrenceAudit
