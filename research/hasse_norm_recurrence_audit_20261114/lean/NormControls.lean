import NormRecurrence
import HasseTraceControls

namespace HasseNormRecurrenceAudit
noncomputable section
open Polynomial Finset
open HasseTraceCoefficientAudit QuadraticTwistCountAudit
variable {K : Type*} [Field K] [Fintype K] [DecidableEq K]

theorem actual_trace_norm (p : ℕ) [Fact (Nat.Prime p)] [CharP K p]
    (hp : p ≠ 2) (r : ℕ) (hq : Fintype.card K = p ^ r) (a b c : K)
    (hΔ : (cubicModel a b c).Δ ≠ 0) :
    (SupersingularTraceClassesAudit.fieldTrace (cubicModel a b c) : K) =
      coefficientAt (cubicPolynomial a b c) p ^ (∑ i ∈ range r, p ^ i) := by
  have hF : ringChar K ≠ 2 := by simpa only [ringChar.eq K p] using hp
  rw [actual_trace_residue hF a b c hΔ]
  change coefficientAt (cubicPolynomial a b c) (Fintype.card K) = _
  rw [hq, coefficient_norm p hp _ (cubic_degree_le a b c) r]

theorem actual_trace_divisibility (p : ℕ) [Fact (Nat.Prime p)] [CharP K p]
    (hp : p ≠ 2) (r : ℕ) (hr : 0 < r) (hq : Fintype.card K = p ^ r) (a b c : K)
    (hΔ : (cubicModel a b c).Δ ≠ 0) :
    (p : ℤ) ∣ SupersingularTraceClassesAudit.fieldTrace (cubicModel a b c) ↔
      coefficientAt (cubicPolynomial a b c) p = 0 := by
  have hF : ringChar K ≠ 2 := by simpa only [ringChar.eq K p] using hp
  rw [← CharP.intCast_eq_zero_iff K p, actual_trace_residue hF a b c hΔ]
  change coefficientAt (cubicPolynomial a b c) (Fintype.card K) = 0 ↔ _
  rw [hq, coefficient_zero_iff p hp _ (cubic_degree_le a b c) r hr]

theorem characteristic_two_fixture :
    coefficientAt (X ^ 3 : (ZMod 2)[X]) 2 = 0 ∧
      coefficientAt (X ^ 3 : (ZMod 2)[X]) 4 = 1 := by
  norm_num [coefficientAt, Polynomial.coeff_one, Polynomial.coeff_X]

theorem characteristic_two_recurrence_false :
    ¬∀ f : (ZMod 2)[X], f.natDegree ≤ 3 →
      coefficientAt f (2 ^ (1 + 1)) = coefficientAt f (2 ^ 1) *
        coefficientAt f 2 ^ (2 ^ 1) := by
  intro h
  have hf : (X ^ 3 : (ZMod 2)[X]).natDegree ≤ 3 := by compute_degree!
  have ht := h (X ^ 3) hf
  norm_num only [Nat.reduceAdd, Nat.reducePow] at ht
  rw [characteristic_two_fixture.1, characteristic_two_fixture.2] at ht
  norm_num at ht

theorem exponent_zero_fixture :
    coefficientAt (X ^ 3 : (ZMod 3)[X]) (3 ^ 0) = 1 ∧
      coefficientAt (X ^ 3 : (ZMod 3)[X]) 3 = 0 := by
  norm_num [coefficientAt, Polynomial.coeff_one, Polynomial.coeff_X]

theorem exponent_zero_vanishing_false :
    ¬∀ f : (ZMod 3)[X], f.natDegree ≤ 3 →
      (coefficientAt f (3 ^ 0) = 0 ↔ coefficientAt f 3 = 0) := by
  intro h
  have hf : (X ^ 3 : (ZMod 3)[X]).natDegree ≤ 3 := by compute_degree!
  have ht := (h (X ^ 3) hf).mpr exponent_zero_fixture.2
  rw [exponent_zero_fixture.1] at ht
  exact one_ne_zero ht

theorem high_degree_fixture :
    coefficientAt (X + X ^ 5 : (ZMod 3)[X]) 3 = 0 ∧
      coefficientAt (X + X ^ 5 : (ZMod 3)[X]) 9 = 1 := by
  constructor
  · norm_num [coefficientAt, Polynomial.coeff_one, Polynomial.coeff_X]
  · have hh : (X + X ^ 5 : (ZMod 3)[X]) ^ 4 =
        X ^ 4 + X ^ 8 + X ^ 16 + X ^ 20 := by
      calc
        _ = (X + X ^ 5) * (X + X ^ 5) ^ 3 := by ring
        _ = (X + X ^ 5) * (X ^ 3 + (X ^ 5) ^ 3) := by rw [add_pow_expChar]
        _ = _ := by ring
    norm_num [coefficientAt, hh]

theorem unrestricted_degree_recurrence_false :
    ¬∀ f : (ZMod 3)[X], coefficientAt f (3 ^ (1 + 1)) =
      coefficientAt f (3 ^ 1) * coefficientAt f 3 ^ (3 ^ 1) := by
  intro h
  have ht := h (X + X ^ 5)
  norm_num only [Nat.reduceAdd, Nat.reducePow] at ht
  rw [high_degree_fixture.1, high_degree_fixture.2] at ht
  norm_num at ht

theorem field9_norm_control :
    ∃ a : F9, (cubicModel a 0 1).Δ ≠ 0 ∧
      coefficientAt (cubicPolynomial a 0 1) 3 = a ∧
      coefficientAt (cubicPolynomial a 0 1) 9 = a ^ 4 ∧
      a ^ 4 = 1 ∧
      (SupersingularTraceClassesAudit.fieldTrace (cubicModel a 0 1) : F9) = a ^ 4 ∧
      coefficientAt (cubicPolynomial a 0 1) 3 ≠
        coefficientAt (cubicPolynomial a 0 1) 9 := by
  obtain ⟨a, hΔ, hp, hq, ht, hne⟩ := field9_control
  have hnorm := coefficient_norm 3 (K := F9) (by decide) (cubicPolynomial a 0 1)
    (cubic_degree_le a 0 1) 2
  have hp' : coefficientAt (cubicPolynomial a 0 1) 3 = a := by
    simpa only [characteristicCoefficient, field9_char, coefficientAt] using hp
  have hq' : coefficientAt (cubicPolynomial a 0 1) 9 = 1 := by
    simpa only [hasseCoefficient, field9_card, coefficientAt] using hq
  have hn : coefficientAt (cubicPolynomial a 0 1) 9 = a ^ 4 := by
    simpa [hp', sum_range_succ] using hnorm
  have ha4 : a ^ 4 = 1 := hn.symm.trans hq'
  refine ⟨a, hΔ, hp', hn, ha4, ?_, ?_⟩
  · simpa only [ha4] using ht
  · simpa only [hp', hq'] using (show a ≠ 1 by simpa only [hp, ht] using hne)

#print axioms actual_trace_norm
#print axioms actual_trace_divisibility
#print axioms characteristic_two_recurrence_false
#print axioms exponent_zero_vanishing_false
#print axioms unrestricted_degree_recurrence_false
#print axioms field9_norm_control
end
end HasseNormRecurrenceAudit
