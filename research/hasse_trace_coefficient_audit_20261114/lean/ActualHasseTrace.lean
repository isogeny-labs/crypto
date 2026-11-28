import PowerSumCoefficient
import CubicTwistCount

namespace HasseTraceCoefficientAudit
noncomputable section
open scoped BigOperators
open Polynomial QuadraticTwistCountAudit
variable {K : Type*} [Field K] [Fintype K] [DecidableEq K]

theorem root_card_euler (hF : ringChar K ≠ 2) (z : K) :
    (rootCard z : K) = 1 + z ^ ((Fintype.card K - 1) / 2) := by
  have hq : 1 < Fintype.card K := Fintype.one_lt_card
  have hodd := FiniteField.odd_card_of_char_ne_two hF
  have he : (Fintype.card K - 1) / 2 ≠ 0 := by omega
  have h2 : (2 : K) ≠ 0 := by
    intro hz
    apply Ring.neg_one_ne_one_of_char_ne_two hF
    linear_combination -hz
  by_cases hz : z = 0
  · rw [hz, root_card_zero, Nat.cast_one, zero_pow he, add_zero]
  · by_cases hs : IsSquare z
    · have hp := (FiniteField.isSquare_iff hF hz).mp hs
      rw [half_eq hF] at hp
      rw [root_card_square h2 z hz hs, hp]
      norm_num
    · have hp : z ^ (Fintype.card K / 2) = -1 :=
        (FiniteField.pow_dichotomy hF hz).resolve_left
          (fun h => hs ((FiniteField.isSquare_iff hF hz).mpr h))
      rw [half_eq hF] at hp
      rw [root_card_nonsquare z hs, Nat.cast_zero, hp]
      ring

theorem actual_point_count_residue (hF : ringChar K ≠ 2) (a b c : K)
    (hΔ : (cubicModel a b c).Δ ≠ 0) :
    (Nat.card (cubicModel a b c).Point : K) = 1 - hasseCoefficient a b c := by
  calc
    _ = (∑ x : K, (rootCard (cubicValue a b c x) : K)) + 1 := by
      rw [cubic_actual_count a b c hΔ]
      push_cast
      rfl
    _ = (∑ x : K, (cubicValue a b c x) ^ ((Fintype.card K - 1) / 2)) + 1 := by
      simp_rw [root_card_euler hF]
      rw [Finset.sum_add_distrib]
      simp only [Finset.sum_const, Finset.card_univ, nsmul_eq_mul, mul_one,
        FiniteField.cast_card_eq_zero, zero_add]
    _ = _ := by
      unfold cubicValue
      rw [cubic_euler_sum hF]
      ring

theorem actual_trace_residue (hF : ringChar K ≠ 2) (a b c : K)
    (hΔ : (cubicModel a b c).Δ ≠ 0) :
    (SupersingularTraceClassesAudit.fieldTrace (cubicModel a b c) : K) =
      hasseCoefficient a b c := by
  unfold SupersingularTraceClassesAudit.fieldTrace
  push_cast
  rw [FiniteField.cast_card_eq_zero, actual_point_count_residue hF a b c hΔ]
  ring

theorem coefficient_zero_iff_char_divides (hF : ringChar K ≠ 2) (a b c : K)
    (hΔ : (cubicModel a b c).Δ ≠ 0) :
    hasseCoefficient a b c = 0 ↔
      (ringChar K : ℤ) ∣ SupersingularTraceClassesAudit.fieldTrace (cubicModel a b c) := by
  rw [← actual_trace_residue hF a b c hΔ]
  exact CharP.intCast_eq_zero_iff K (ringChar K) _

theorem prime_trace_residue (p : ℕ) [Fact p.Prime] (hp : p ≠ 2) (a b c : ZMod p)
    (hΔ : (cubicModel a b c).Δ ≠ 0) :
    (SupersingularTraceClassesAudit.fieldTrace (cubicModel a b c) : ZMod p) =
      ((cubicPolynomial a b c) ^ ((p - 1) / 2)).coeff (p - 1) := by
  have hF : ringChar (ZMod p) ≠ 2 := by simpa only [ZMod.ringChar_zmod_n] using hp
  simpa only [hasseCoefficient, ZMod.card] using actual_trace_residue hF a b c hΔ

theorem prime_coefficient_zero_iff_trace_divisible (p : ℕ) [Fact p.Prime]
    (hp : p ≠ 2) (a b c : ZMod p) (hΔ : (cubicModel a b c).Δ ≠ 0) :
    ((cubicPolynomial a b c) ^ ((p - 1) / 2)).coeff (p - 1) = 0 ↔
      (p : ℤ) ∣ SupersingularTraceClassesAudit.fieldTrace (cubicModel a b c) := by
  rw [← prime_trace_residue p hp a b c hΔ, ZMod.intCast_zmod_eq_zero_iff_dvd]

#print axioms root_card_euler
#print axioms actual_point_count_residue
#print axioms actual_trace_residue
#print axioms coefficient_zero_iff_char_divides
#print axioms prime_coefficient_zero_iff_trace_divisible
end
end HasseTraceCoefficientAudit
