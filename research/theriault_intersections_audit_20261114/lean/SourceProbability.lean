import PairedLaw
import MeanBounds

namespace TheriaultIntersectionsAudit
noncomputable section
open Finset WalkBoundaryAudit

/-- Source Q(n,s,i), using the actual uniform point-word PMF and actual count. -/
def intersectionProbability (M s k : ℕ) [NeZero M] : ℝ :=
  ∑ w : PairedWord M s, (uniformPMF (PairedWord M s) w).toReal *
    (if pairedCount w = k then 1 else 0)

/-- The exact finite weighted-probability expression printed for E(n,s). -/
def sourceMean (M s : ℕ) [NeZero M] : ℝ :=
  ∑ k ∈ range s, (k : ℝ) * intersectionProbability M s k

theorem pairedCount_lt (M s : ℕ) (hs : 0 < s) (w : PairedWord M s) :
    pairedCount w < s := by
  have hc : 0 < (occupied (fun i => (w i).1)).card := by
    apply card_pos.mpr
    refine ⟨(w ⟨0, hs⟩).1, ?_⟩
    exact mem_image.mpr ⟨⟨0, hs⟩, mem_univ _, rfl⟩
  unfold pairedCount repeatCount
  omega

theorem word_mass_sum (M s : ℕ) [NeZero M] :
    (∑ w : PairedWord M s, (uniformPMF (PairedWord M s) w).toReal) = 1 := by
  have hm : (M : ℝ) ≠ 0 := by exact_mod_cast NeZero.ne M
  simp only [word_mass_real, sum_const, card_univ, Fintype.card_fun,
    Fintype.card_prod, Fintype.card_fin, Fintype.card_bool, Nat.cast_pow,
    Nat.cast_mul, Nat.cast_ofNat, nsmul_eq_mul]
  field_simp

theorem intersectionProbability_nonnegative (M s k : ℕ) [NeZero M] :
    0 ≤ intersectionProbability M s k := by
  unfold intersectionProbability
  positivity

theorem intersectionProbability_sum (M s : ℕ) [NeZero M] (hs : 0 < s) :
    (∑ k ∈ range s, intersectionProbability M s k) = 1 := by
  unfold intersectionProbability
  rw [Finset.sum_comm]
  calc
    _ = ∑ w : PairedWord M s, (uniformPMF (PairedWord M s) w).toReal := by
      apply Finset.sum_congr rfl
      intro w hw
      rw [← Finset.mul_sum]
      have hlt := pairedCount_lt M s hs w
      simp [hlt]
    _ = 1 := word_mass_sum M s

theorem source_mean_eq (M s : ℕ) [NeZero M] (hs : 0 < s) :
    sourceMean M s = pairedMean M s := by
  unfold sourceMean intersectionProbability pairedMean
  simp_rw [Finset.mul_sum]
  rw [Finset.sum_comm]
  apply Finset.sum_congr rfl
  intro w hw
  rw [Finset.sum_eq_single (pairedCount w)]
  · simp [mul_comm]
  · intro k hk hne
    simp [Ne.symm hne]
  · intro hn
    exact False.elim (hn (mem_range.mpr (pairedCount_lt M s hs w)))

theorem source_mean_closed (M s : ℕ) [NeZero M] (hs : 0 < s) :
    sourceMean M s = (s : ℝ) - M + M * (1 - (M : ℝ)⁻¹) ^ s := by
  rw [source_mean_eq M s hs, paired_mean_closed]

theorem source_mean_three (M : ℕ) [NeZero M] :
    sourceMean M 3 = 3 / (M : ℝ) - 1 / (M : ℝ) ^ 2 := by
  have hm : (M : ℝ) ≠ 0 := by exact_mod_cast NeZero.ne M
  rw [source_mean_closed M 3 (by decide)]
  field_simp
  ring

theorem source_lower_bound_counterfamily (M : ℕ) [NeZero M] (hM : 4 ≤ M) :
    3 ≤ (3 : ℕ) ∧ 3 < (2 * M) / 2 ∧
      sourceMean M 3 < 2 * (3 : ℝ) ^ 2 / (3 * (2 * M)) := by
  have hm : (0 : ℝ) < M := by exact_mod_cast (show 0 < M by omega)
  have hm0 : (M : ℝ) ≠ 0 := ne_of_gt hm
  have hr : 2 * (3 : ℝ) ^ 2 / (3 * (2 * M)) = 3 / (M : ℝ) := by field_simp
  refine ⟨by decide, by omega, ?_⟩
  rw [source_mean_three, hr]
  have hi : 0 < 1 / (M : ℝ) ^ 2 := one_div_pos.mpr (sq_pos_of_pos hm)
  linarith

theorem literal_theorem_one_lower_false :
    ¬∀ (M s : ℕ) [NeZero M], 3 ≤ s → s < (2 * M) / 2 →
      2 * (s : ℝ) ^ 2 / (3 * (2 * M)) ≤ sourceMean M s := by
  intro h
  exact (not_le_of_gt (source_lower_bound_counterfamily 4 (by decide)).2.2)
    (h 4 3 (by decide) (by decide))

theorem concrete_eight_point_fixture :
    Fintype.card (PairedPoint 4) = 8 ∧ Fintype.card (PairedWord 4 3) = 512 ∧
      sourceMean 4 3 = 11 / 16 ∧ 2 * (3 : ℝ) ^ 2 / (3 * 8) = 3 / 4 ∧
      sourceMean 4 3 < 3 / 4 := by
  rw [source_mean_three]
  norm_num [PairedPoint, PairedWord]

theorem repaired_source_window (M s : ℕ) [NeZero M] (hs : 3 ≤ s)
    (hhi : s < (2 * M) / 2) :
    4 * (s : ℝ) ^ 2 / (9 * (2 * M)) ≤ sourceMean M s ∧
      sourceMean M s ≤ (s : ℝ) ^ 2 / (2 * M) := by
  rw [source_mean_eq M s (by omega), paired_mean_eq]
  exact repaired_window M s (Nat.pos_of_ne_zero (NeZero.ne M)) hs (by omega)

#print axioms source_mean_eq
#print axioms source_mean_three
#print axioms source_lower_bound_counterfamily
#print axioms literal_theorem_one_lower_false
#print axioms repaired_source_window
end
end TheriaultIntersectionsAudit
