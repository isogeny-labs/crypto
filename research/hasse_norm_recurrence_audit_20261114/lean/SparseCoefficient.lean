module

public import PowerSumCoefficient
public import Mathlib.Algebra.Polynomial.Expand

@[expose] public section
namespace HasseNormRecurrenceAudit
noncomputable section
open Polynomial Finset
variable {K : Type*} [Field K]

theorem residue_window (M n i j : ℕ) (hM : 0 < M)
    (hi : i < 2 * M - 1) (hs : i + j = M * n + (M - 1))
    (hj : M ∣ j) : i = M - 1 ∧ j = M * n := by
  have hm : M - 1 < M := by omega
  have hmod := congrArg (fun z : ℕ => z % M) hs
  simp only [Nat.add_mod, Nat.mod_eq_zero_of_dvd hj, add_zero,
    Nat.mod_mod, Nat.mul_mod_right, zero_add, Nat.mod_eq_of_lt hm] at hmod
  have hdiv := Nat.mod_add_div i M
  have hz : i / M = 0 := by
    by_contra hn
    have ht := Nat.mul_le_mul_left M (Nat.one_le_iff_ne_zero.mpr hn)
    rw [mul_one] at ht
    rw [hmod] at hdiv
    omega
  rw [hz, mul_zero, add_zero, hmod] at hdiv
  exact ⟨hdiv.symm, by omega⟩

/-- No higher residue contribution fits below the strict degree threshold. -/
theorem sparse_product_coefficient (A B : K[X]) (σ : K →+* K)
    (M n : ℕ) (hM : 0 < M) (hA : A.natDegree < 2 * M - 1) :
    (A * (expand K M B).map σ).coeff (M * n + (M - 1)) =
      A.coeff (M - 1) * σ (B.coeff n) := by
  rw [coeff_mul]
  calc
    _ = A.coeff (M - 1) * ((expand K M B).map σ).coeff (M * n) := by
      apply Finset.sum_eq_single (M - 1, M * n)
      · intro ij hij hne
        by_cases ha : A.coeff ij.1 = 0
        · simp only [ha, zero_mul]
        by_cases hj : M ∣ ij.2
        · have hi := lt_of_le_of_lt (le_natDegree_of_ne_zero ha) hA
          have hs := Finset.mem_antidiagonal.mp hij
          have he := residue_window M n ij.1 ij.2 hM hi hs hj
          exact False.elim (hne (Prod.ext he.1 he.2))
        · simp only [coeff_map, coeff_expand hM, hj, ite_false, map_zero, mul_zero]
      · intro hnot
        exact False.elim (hnot (Finset.mem_antidiagonal.mpr (by omega)))
    _ = _ := by rw [coeff_map, coeff_expand_mul' hM]

#print axioms residue_window
#print axioms sparse_product_coefficient
end
end HasseNormRecurrenceAudit
