import GreenBound

namespace MarkovGreenBoundAudit
noncomputable section
open scoped BigOperators

/-- An actual positive reset law on the prime state space of size11. -/
def biasedWeight (y : ZMod 11) : ℝ := if y=0 then 1/2 else 1/20

theorem biased_weight_positive (y : ZMod 11) : 0 < biasedWeight y := by
  unfold biasedWeight
  split_ifs <;> norm_num

theorem biased_weight_sum : (∑ y : ZMod 11, biasedWeight y) = 1 := by
  have he (y : ZMod 11) : biasedWeight y = (if y=0 then 9/20 else 0)+1/20 := by
    unfold biasedWeight
    split_ifs <;> norm_num
  simp only [he, Finset.sum_add_distrib, Finset.sum_ite_eq', Finset.mem_univ, ite_true,
    Finset.sum_const, Finset.card_univ, ZMod.card, nsmul_eq_mul]
  norm_num

def biasedRaw : Matrix (ZMod 11) (ZMod 11) ℝ := fun _ y => biasedWeight y

theorem biased_raw_stochastic : biasedRaw ∈ Matrix.rowStochastic ℝ (ZMod 11) := by
  apply Matrix.mem_rowStochastic_iff_sum.mpr
  exact ⟨fun _ y => (biased_weight_positive y).le, fun _ => biased_weight_sum⟩

def biasedLaw (x : ZMod 11) : PMF (ZMod 11) := kernel biasedRaw biased_raw_stochastic x
def biasedMatrix : Matrix (ZMod 11) (ZMod 11) ℝ := fun x y => (biasedLaw x y).toReal

theorem biased_entry (x y : ZMod 11) : biasedMatrix x y = biasedWeight y :=
  kernel_entry biasedRaw biased_raw_stochastic x y

theorem biased_row_stochastic : biasedMatrix ∈ Matrix.rowStochastic ℝ (ZMod 11) := by
  have he : biasedMatrix = biasedRaw := by ext x y; exact biased_entry x y
  rw [he]
  exact biased_raw_stochastic

theorem biased_stationary (y : ZMod 11) :
    (∑ x : ZMod 11, biasedWeight x * biasedMatrix x y) = biasedWeight y := by
  simp_rw [biased_entry]
  rw [← Finset.sum_mul, biased_weight_sum, one_mul]

theorem biased_irreducible : ∀ x y : ZMod 11,
    ∃ n : ℕ, 0<n ∧ 0<(biasedMatrix^n) x y := by
  intro x y
  exact ⟨1, by decide, by simpa only [pow_one, biased_entry] using biased_weight_positive y⟩

theorem biased_aperiodic : ∀ x : ZMod 11, ∀ d : ℕ,
    (∀ n : ℕ, 0<n → 0<(biasedMatrix^n) x x → d∣n) → d=1 := by
  intro x d h
  exact Nat.dvd_one.mp (h 1 (by decide)
    (by simpa only [pow_one, biased_entry] using biased_weight_positive x))

theorem biased_column_zero : (∑ x : ZMod 11, biasedMatrix x 0) = 11/2 := by
  simp only [biased_entry, biasedWeight, ite_true, Finset.sum_const,
    Finset.card_univ, ZMod.card, nsmul_eq_mul]
  norm_num

theorem biased_not_column_stochastic : biasedMatrix ∉ Matrix.colStochastic ℝ (ZMod 11) := by
  intro h
  have hs := Matrix.sum_col_of_mem_colStochastic h 0
  rw [biased_column_zero] at hs
  norm_num at hs

theorem biased_uniform_not_stationary :
    ¬ (∀ y : ZMod 11, (∑ x : ZMod 11, (1/11 : ℝ)*biasedMatrix x y) = 1/11) := by
  intro h
  have hs := h 0
  rw [← Finset.mul_sum, biased_column_zero] at hs
  norm_num at hs

theorem max_entry_zero {S : Type*} [Fintype S] [DecidableEq S] [Nonempty S]
    (P : Matrix S S ℝ) : maxEntry P 0 = 1 := by
  apply le_antisymm
  · apply Finset.sup'_le
    intro xy _
    simp only [pow_zero, Matrix.one_apply]
    split_ifs <;> norm_num
  · let x : S := Classical.choice inferInstance
    simpa only [pow_zero, Matrix.one_apply, ite_true] using entry_le_max P 0 x x

theorem biased_max_entry_one : maxEntry biasedMatrix 1 = 1/2 := by
  apply le_antisymm
  · apply Finset.sup'_le
    intro xy _
    simp only [pow_one, biased_entry, biasedWeight]
    split_ifs <;> norm_num
  · simpa only [pow_one, biased_entry, biasedWeight, ite_true] using
      entry_le_max biasedMatrix 1 0 0

theorem biased_source_bound_one : sourceRHS biasedMatrix 1 = 4 := by
  norm_num [sourceRHS, Finset.sum_range_succ, max_entry_zero, biased_max_entry_one]

theorem biased_green_one (x y : ZMod 11) :
    green biasedMatrix 1 x y = (if x=y then 1 else 0)+biasedWeight y := by
  simp only [green, partialGreen, Finset.sum_range_succ, Finset.sum_range_zero,
    zero_add, pow_zero, pow_one, Matrix.one_apply, biased_entry]

theorem biased_column_energy_one : columnEnergy biasedMatrix 1 0 = 19/4 := by
  have he (y : ZMod 11) : green biasedMatrix 1 y 0 ^ 2 =
      (if y=0 then 2 else 0)+1/4 := by
    rw [biased_green_one]
    simp only [biasedWeight, ite_true]
    split_ifs <;> norm_num
  simp only [columnEnergy, he, Finset.sum_add_distrib, Finset.sum_ite_eq',
    Finset.mem_univ, ite_true, Finset.sum_const, Finset.card_univ, ZMod.card, nsmul_eq_mul]
  norm_num

/-- Dropping uniform stationarity breaks the column half even on a positive chain. -/
theorem uniform_stationarity_guard_counter :
    columnMaximum biasedMatrix 1 > sourceRHS biasedMatrix 1 := by
  have h : columnEnergy biasedMatrix 1 0 ≤ columnMaximum biasedMatrix 1 :=
    Finset.le_sup' (columnEnergy biasedMatrix 1) (Finset.mem_univ 0)
  rw [biased_column_energy_one] at h
  rw [biased_source_bound_one]
  linarith

#print axioms biased_stationary
#print axioms biased_irreducible
#print axioms biased_uniform_not_stationary
#print axioms uniform_stationarity_guard_counter
end
end MarkovGreenBoundAudit
