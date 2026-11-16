module

public import Mathlib.FieldTheory.Finite.Basic
public import Mathlib.Tactic

@[expose] public section
namespace HasseTraceCoefficientAudit
noncomputable section
open scoped BigOperators
open Finset Polynomial
variable {K : Type*} [Field K] [Fintype K] [DecidableEq K]

theorem sum_pow_nonzero (n : ℕ) (hn : n ≠ 0) :
    (∑ x : K, x ^ n) = if Fintype.card K - 1 ∣ n then -1 else 0 := by
  let e : Kˣ ↪ K := ⟨fun x => x, Units.val_injective⟩
  have he : univ.map e = univ \ {(0 : K)} := by
    ext x
    simpa only [mem_map, mem_univ, Function.Embedding.coeFn_mk, true_and, mem_sdiff,
      mem_singleton, e] using! isUnit_iff_ne_zero
  calc
    _ = ∑ x ∈ univ \ {(0 : K)}, x ^ n := by
      rw [← sum_sdiff ({0} : Finset K).subset_univ, sum_singleton, zero_pow hn, add_zero]
    _ = ∑ x : Kˣ, (x ^ n : K) := by simp [e, ← he, univ.sum_map e]
    _ = _ := FiniteField.sum_pow_units K n

theorem divisor_window (n d : ℕ) (hd : 0 < d) (hlo : d ≤ n) (hhi : n < 2 * d) :
    d ∣ n ↔ n = d := by
  constructor
  · rintro ⟨k, hk⟩
    have hk0 : 0 < k := by
      by_contra h
      have hz : k = 0 := by omega
      rw [hz, mul_zero] at hk
      omega
    have hk2 : k < 2 := by
      by_contra h
      have hh := Nat.mul_le_mul_left d (show 2 ≤ k by omega)
      rw [← hk] at hh
      omega
    have hk1 : k = 1 := by omega
    simpa only [hk1, mul_one] using hk
  · rintro rfl
    exact dvd_rfl

theorem sum_pow_window (n : ℕ) (hn : n < 2 * (Fintype.card K - 1)) :
    (∑ x : K, x ^ n) = if n = Fintype.card K - 1 then -1 else 0 := by
  have hq : 1 < Fintype.card K := Fintype.one_lt_card
  by_cases hlo : n < Fintype.card K - 1
  · rw [FiniteField.sum_pow_lt_card_sub_one K n hlo, ite_eq_right (by omega)]
  · have hpos : n ≠ 0 := by omega
    rw [sum_pow_nonzero n hpos]
    simp only [divisor_window n (Fintype.card K - 1) (by omega) (by omega) hn]

/-- Exact coefficient extraction from the actual evaluation sum. The strict
degree bound excludes a second positive multiple of the group order. -/
theorem eval_sum_coefficient (f : K[X])
    (hf : f.natDegree < 2 * (Fintype.card K - 1)) :
    (∑ x : K, f.eval x) = -f.coeff (Fintype.card K - 1) := by
  have hq : 1 < Fintype.card K := Fintype.one_lt_card
  calc
    _ = ∑ n ∈ f.support, f.coeff n * (∑ x : K, x ^ n) := by
      simp only [Polynomial.eval_eq_sum, Polynomial.sum_def]
      rw [Finset.sum_comm]
      simp only [Finset.mul_sum]
    _ = f.coeff (Fintype.card K - 1) * (∑ x : K, x ^ (Fintype.card K - 1)) := by
      apply Finset.sum_eq_single (Fintype.card K - 1)
      · intro n hn hne
        have hd := Polynomial.le_natDegree_of_ne_zero (Polynomial.mem_support_iff.mp hn)
        rw [sum_pow_window n (lt_of_le_of_lt hd hf), ite_eq_right hne, mul_zero]
      · intro hn
        have hz : f.coeff (Fintype.card K - 1) = 0 := by
          simpa only [Polynomial.mem_support_iff, not_not] using hn
        rw [hz, zero_mul]
    _ = _ := by
      rw [sum_pow_window (Fintype.card K - 1) (by omega), ite_eq_left rfl]
      ring

def cubicPolynomial (a b c : K) : K[X] := X ^ 3 + C a * X ^ 2 + C b * X + C c

def hasseCoefficient (a b c : K) : K :=
  ((cubicPolynomial a b c) ^ ((Fintype.card K - 1) / 2)).coeff (Fintype.card K - 1)

omit [Fintype K] [DecidableEq K] in
theorem cubic_eval (a b c x : K) :
    (cubicPolynomial a b c).eval x = x ^ 3 + a * x ^ 2 + b * x + c := by
  simp only [cubicPolynomial, eval_add, eval_mul, eval_pow, eval_X, eval_C]

omit [Fintype K] [DecidableEq K] in
theorem cubic_degree_le (a b c : K) : (cubicPolynomial a b c).natDegree ≤ 3 := by
  unfold cubicPolynomial
  compute_degree!

omit [DecidableEq K] in
theorem half_eq (hF : ringChar K ≠ 2) :
    Fintype.card K / 2 = (Fintype.card K - 1) / 2 := by
  have hk := FiniteField.odd_card_of_char_ne_two hF
  omega

omit [DecidableEq K] in
theorem cubic_power_degree (hF : ringChar K ≠ 2) (a b c : K) :
    ((cubicPolynomial a b c) ^ ((Fintype.card K - 1) / 2)).natDegree <
      2 * (Fintype.card K - 1) := by
  have hq : 1 < Fintype.card K := Fintype.one_lt_card
  have hk := FiniteField.odd_card_of_char_ne_two hF
  have hd := Polynomial.natDegree_pow_le_of_le ((Fintype.card K - 1) / 2)
    (cubic_degree_le a b c)
  omega

theorem cubic_euler_sum (hF : ringChar K ≠ 2) (a b c : K) :
    (∑ x : K, (x ^ 3 + a * x ^ 2 + b * x + c) ^ ((Fintype.card K - 1) / 2)) =
      -hasseCoefficient a b c := by
  simpa only [hasseCoefficient, eval_pow, cubic_eval] using
    eval_sum_coefficient ((cubicPolynomial a b c) ^ ((Fintype.card K - 1) / 2))
      (cubic_power_degree hF a b c)

#print axioms sum_pow_window
#print axioms eval_sum_coefficient
#print axioms cubic_euler_sum
end
end HasseTraceCoefficientAudit
