module
public import Mathlib.Analysis.Normed.Ring.InfiniteSum
public import Mathlib.Tactic

@[expose] public section
namespace HPSFirstFallAudit
noncomputable section
open Polynomial

/-- The actual source q-nomial coefficient, including the negative-index convention. -/
def qnomial (q n : ℕ) (k : ℤ) : ℤ :=
  if 0 ≤ k then ((∑ i ∈ Finset.range q, (X : Polynomial ℤ) ^ i) ^ n).coeff k.toNat else 0

theorem qnomial_one (q : ℕ) (k : ℤ) :
    qnomial q 1 k = if 0 ≤ k ∧ k < q then 1 else 0 := by
  by_cases hk : 0 ≤ k
  · simp [qnomial, hk, coeff_X_pow, Int.toNat_lt hk, eq_comm]
  · simp [qnomial, hk]

/-- Source §4's actual bi-infinite sum, not a cutoff approximation. -/
def gamma (q n d r : ℕ) (k : ℤ) : ℤ :=
  ∑' j : ℤ, (qnomial q n (k - j * d * q) - qnomial q n (k - r * d - j * d * q))

theorem gamma_five_zero : gamma 5 1 2 4 2 = 0 := by
  have hh (j : ℤ) : qnomial 5 1 (2 - j * 2 * 5) -
      qnomial 5 1 (2 - 4 * 2 - j * 2 * 5) =
      if j = 0 then 1 else if j = -1 then -1 else 0 := by
    rw [qnomial_one, qnomial_one]
    split_ifs <;> omega
  unfold gamma
  norm_num only at hh
  norm_num only [Nat.cast_ofNat]
  rw [tsum_congr hh]
  rw [tsum_eq_sum (s := ({0, -1} : Finset ℤ))]
  · norm_num
  · intro j hj
    have hj' : j ≠ 0 ∧ j ≠ -1 := by simpa using hj
    simp [hj'.1, hj'.2]

theorem literal_gamma_positivity_false :
    ¬ (∀ q n d : ℕ, q.Prime → 0 < n → 2 ≤ d → d ≤ n * (q - 1) →
      0 < gamma q n d (q - 1) ((n * (q - 1) - d + 2) / 2)) := by
  intro h
  have hh := h 5 1 2 (by decide) (by omega) (by omega) (by omega)
  norm_num only at hh
  rw [gamma_five_zero] at hh
  omega

#print axioms qnomial_one
#print axioms gamma_five_zero
#print axioms literal_gamma_positivity_false
end
end HPSFirstFallAudit
