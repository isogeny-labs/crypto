import NonzeroLaw

namespace RhoNonzeroSupportAudit
noncomputable section
open RhoTwoChoiceCollisionAudit

theorem nonzero_mean_lower {p : ℕ} [Fact p.Prime] (m : ℕ) (hm : 0 < m)
    (hmp : 4 * m^2 * (4 * m + 1) ≤ p) :
    (m : ℝ)^2 / 2 - 2 ≤ nonzeroMean p := by
  linarith [ensemble_mean_lower (p := p) m hm hmp, all_mean_le_nonzero_add_two (p := p)]

theorem nonzero_mean_quarter {p : ℕ} [Fact p.Prime] (m : ℕ) (hm : 3 ≤ m)
    (hmp : 4 * m^2 * (4 * m + 1) ≤ p) :
    (m : ℝ)^2 / 4 ≤ nonzeroMean p := by
  have h := nonzero_mean_lower (p := p) m (by omega) hmp
  have hmR : (3 : ℝ) ≤ m := by exact_mod_cast hm
  nlinarith

theorem nonzero_ratio_unbounded (C : ℕ) :
    ∃ p : ℕ, ∃ hp : p.Prime, 2 < p ∧
      (C : ℝ) * p < (@nonzeroMean p ⟨hp⟩)^2 := by
  let m : ℕ := 1000 * (C + 1)
  have hm : 3 ≤ m := by dsimp [m]; omega
  let N := 4 * m^2 * (4 * m + 1)
  have hN : N ≠ 0 := by dsimp [N]; positivity
  obtain ⟨p,hp,hNp,hpN⟩ := Nat.exists_prime_lt_and_le_two_mul N hN
  let : Fact p.Prime := ⟨hp⟩
  have hp2 : 2 < p := by dsimp [N] at hNp; nlinarith
  refine ⟨p,hp,hp2,?_⟩
  have hE := nonzero_mean_quarter (p := p) m hm hNp.le
  have hmR : 0 < (m : ℝ) := Nat.cast_pos.mpr (by omega)
  have hC : (0 : ℝ) ≤ C := Nat.cast_nonneg C
  have hpR : (p : ℝ) ≤ 8 * (m : ℝ)^2 * (4 * m + 1) := by
    have hx : (p : ℝ) ≤ 2 * (4 * (m : ℝ)^2 * (4 * m + 1)) := by exact_mod_cast hpN
    nlinarith
  have hscale : 8 * (C : ℝ) * (4 * (m : ℝ) + 1) < (m : ℝ)^2 / 16 := by
    dsimp [m]
    push_cast
    nlinarith [sq_nonneg (C : ℝ)]
  have hprod := mul_lt_mul_of_pos_left hscale (sq_pos_of_pos hmR)
  have hstrict : (C : ℝ) * (8 * (m : ℝ)^2 * (4 * m + 1)) <
      ((m : ℝ)^2 / 4)^2 := by nlinarith
  have hsq : ((m : ℝ)^2 / 4)^2 ≤ (nonzeroMean p)^2 := by
    nlinarith [sq_nonneg (nonzeroMean p - (m : ℝ)^2 / 4)]
  exact lt_of_le_of_lt (mul_le_mul_of_nonneg_left hpR hC) (lt_of_lt_of_le hstrict hsq)

theorem nonzero_no_uniform_sqrt_bound :
    ¬∃ K : ℝ, 0 ≤ K ∧ ∀ p : ℕ, ∀ hp : p.Prime, 2 < p →
      @nonzeroMean p ⟨hp⟩ ≤ K * Real.sqrt p := by
  rintro ⟨K,hK,hbound⟩
  obtain ⟨C,hC⟩ := exists_nat_gt (K^2)
  obtain ⟨p,hp,hp2,hpC⟩ := nonzero_ratio_unbounded C
  let : Fact p.Prime := ⟨hp⟩
  have h0 := nonzeroMean_nonneg (p := p)
  have hb := hbound p hp hp2
  have hsq := mul_self_le_mul_self h0 hb
  have hs : (Real.sqrt (p : ℝ))^2 = p := Real.sq_sqrt (Nat.cast_nonneg p)
  have hp0 : (0 : ℝ) < p := Nat.cast_pos.mpr hp.pos
  have hCp := mul_lt_mul_of_pos_right hC hp0
  nlinarith

#print axioms nonzero_ratio_unbounded
#print axioms nonzero_no_uniform_sqrt_bound
end
end RhoNonzeroSupportAudit
