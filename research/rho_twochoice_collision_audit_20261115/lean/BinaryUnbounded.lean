import BinaryEnsemble
import TwoChoiceImports

namespace RhoTwoChoiceCollisionAudit
noncomputable section

/-- The support table is averaged uniformly, rather than chosen adversarially.
The conclusion concerns first-repeat expectation in the two-label model. -/
theorem ensemble_ratio_unbounded (C : ℕ) :
    ∃ p : ℕ, ∃ hp : p.Prime,
      letI : Fact p.Prime := ⟨hp⟩
      (C : ℝ) * p < (ensembleMean p)^2 := by
  let m : ℕ := 200 * (C + 1)
  have hm : 0 < m := by dsimp [m]; omega
  let N := 4 * m^2 * (4 * m + 1)
  have hN : N ≠ 0 := by dsimp [N]; positivity
  obtain ⟨p,hp,hNp,hpN⟩ := Nat.exists_prime_lt_and_le_two_mul N hN
  let : Fact p.Prime := ⟨hp⟩
  refine ⟨p,hp,?_⟩
  have hE := ensemble_mean_lower (p := p) m hm hNp.le
  have hmR : 0 < (m : ℝ) := Nat.cast_pos.mpr hm
  have hC : (0 : ℝ) ≤ C := Nat.cast_nonneg C
  have hpR : (p : ℝ) ≤ 8 * (m : ℝ)^2 * (4 * m + 1) := by
    have hx : (p : ℝ) ≤ 2 * (4 * (m : ℝ)^2 * (4 * m + 1)) := by
      exact_mod_cast hpN
    nlinarith
  have hscale : 8 * (C : ℝ) * (4 * (m : ℝ) + 1) < (m : ℝ)^2 / 4 := by
    dsimp [m]
    push_cast
    nlinarith [sq_nonneg (C : ℝ)]
  have hprod := mul_lt_mul_of_pos_left hscale (sq_pos_of_pos hmR)
  have hstrict : (C : ℝ) * (8 * (m : ℝ)^2 * (4 * m + 1)) <
      ((m : ℝ)^2 / 2)^2 := by nlinarith
  have hsq : ((m : ℝ)^2 / 2)^2 ≤ (ensembleMean p)^2 := by
    nlinarith [sq_nonneg (ensembleMean p - (m : ℝ)^2 / 2)]
  exact lt_of_le_of_lt (mul_le_mul_of_nonneg_left hpR hC) (lt_of_lt_of_le hstrict hsq)

/-- In particular no natural squared-constant bounds this actual ensemble. -/
theorem no_uniform_squared_root_bound :
    ¬∃ C : ℕ, ∀ p : ℕ, ∀ hp : p.Prime,
      letI : Fact p.Prime := ⟨hp⟩
      (ensembleMean p)^2 ≤ (C : ℝ) * p := by
  rintro ⟨C,hC⟩
  obtain ⟨p,hp,hpC⟩ := ensemble_ratio_unbounded C
  exact (not_lt_of_ge (hC p hp)) hpC

theorem ensembleMean_nonneg (p : ℕ) [Fact p.Prime] : 0 ≤ ensembleMean p := by
  apply finiteMean_nonneg
  intro f
  apply Finset.sum_nonneg
  intro h _
  exact mul_nonneg (WalkWeightedGeneralAudit.weight_nonnegative fair fair_nonneg h)
    (Nat.cast_nonneg _)

/-- A literal real-constant square-root upper bound is also impossible. -/
theorem no_uniform_sqrt_bound :
    ¬∃ K : ℝ, 0 ≤ K ∧ ∀ p : ℕ, ∀ hp : p.Prime,
      letI : Fact p.Prime := ⟨hp⟩
      ensembleMean p ≤ K * Real.sqrt p := by
  rintro ⟨K,hK,hbound⟩
  obtain ⟨C,hC⟩ := exists_nat_gt (K^2)
  obtain ⟨p,hp,hpC⟩ := ensemble_ratio_unbounded C
  let : Fact p.Prime := ⟨hp⟩
  have h0 := ensembleMean_nonneg p
  have hb := hbound p hp
  have hsq := mul_self_le_mul_self h0 hb
  have hs : (Real.sqrt (p : ℝ))^2 = p := Real.sq_sqrt (Nat.cast_nonneg p)
  have hp0 : (0 : ℝ) < p := Nat.cast_pos.mpr hp.pos
  have hCp := mul_lt_mul_of_pos_right hC hp0
  nlinarith

#print axioms ensemble_ratio_unbounded
#print axioms no_uniform_squared_root_bound
#print axioms no_uniform_sqrt_bound
end
end RhoTwoChoiceCollisionAudit
