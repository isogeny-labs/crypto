module

public import ActualOccupancy

@[expose] public section
namespace TheriaultIntersectionsAudit
noncomputable section

theorem mean_nonnegative (M s : ℕ) : 0 ≤ mean M s := by
  unfold mean
  positivity

theorem mean_upper (M s : ℕ) (hM : 0 < M) :
    mean M s ≤ (s : ℝ) * ((s : ℝ) - 1) / (2 * M) := by
  have hm : (0 : ℝ) < M := by exact_mod_cast hM
  have hm0 : (M : ℝ) ≠ 0 := ne_of_gt hm
  induction s with
  | zero => simp [mean_closed M 0 hM]
  | succ s ih =>
    have hdrop : (1 - (M : ℝ)⁻¹) * mean M s ≤ mean M s := by
      have hh := mean_nonnegative M s
      have hi : 0 ≤ (M : ℝ)⁻¹ := (inv_pos.mpr hm).le
      nlinarith
    calc
      _ ≤ mean M s + (s : ℝ) / M := by rw [mean_recurrence M s hM]; linarith
      _ ≤ (s : ℝ) * ((s : ℝ) - 1) / (2 * M) + (s : ℝ) / M :=
        by linarith
      _ = _ := by push_cast; field_simp; ring

theorem mean_lower (M s : ℕ) (hM : 0 < M) (hs : s ≤ M + 1) :
    (s : ℝ) * ((s : ℝ) - 1) / (3 * M) ≤ mean M s := by
  have hm : (0 : ℝ) < M := by exact_mod_cast hM
  have hm0 : (M : ℝ) ≠ 0 := ne_of_gt hm
  have hm1 : (1 : ℝ) ≤ M := by exact_mod_cast hM
  have hcoeff : 0 ≤ 1 - (M : ℝ)⁻¹ := by
    have he : 1 - (M : ℝ)⁻¹ = ((M : ℝ) - 1) / M := by field_simp
    rw [he]
    exact div_nonneg (by linarith) hm.le
  induction s with
  | zero => simp [mean_closed M 0 hM]
  | succ s ih =>
    have hsM : s ≤ M := by omega
    have hsR : (s : ℝ) ≤ M := by exact_mod_cast hsM
    have ih' := ih (show s ≤ M + 1 by omega)
    have hgap :
        (1 - (M : ℝ)⁻¹) * ((s : ℝ) * ((s : ℝ) - 1) / (3 * M)) + (s : ℝ) / M -
          (((s : ℝ) + 1) * (s : ℝ) / (3 * M)) =
        (s : ℝ) * ((M : ℝ) - s + 1) / (3 * (M : ℝ) ^ 2) := by
      field_simp
      ring
    have hpos : 0 ≤ (s : ℝ) * ((M : ℝ) - s + 1) / (3 * (M : ℝ) ^ 2) := by
      have ht : 0 ≤ (M : ℝ) - s + 1 := by linarith
      positivity
    calc
      _ ≤ (1 - (M : ℝ)⁻¹) * ((s : ℝ) * ((s : ℝ) - 1) / (3 * M)) +
          (s : ℝ) / M := by
        have hle : ((s : ℝ) + 1) * (s : ℝ) / (3 * M) ≤
            (1 - (M : ℝ)⁻¹) * ((s : ℝ) * ((s : ℝ) - 1) / (3 * M)) +
              (s : ℝ) / M := by linarith
        simpa only [Nat.cast_add, Nat.cast_one, add_sub_cancel_right] using hle
      _ ≤ (1 - (M : ℝ)⁻¹) * mean M s + (s : ℝ) / M :=
        by have hh := mul_le_mul_of_nonneg_left ih' hcoeff; linarith
      _ = _ := (mean_recurrence M s hM).symm

theorem repaired_window (M s : ℕ) (hM : 0 < M) (hs : 3 ≤ s) (hhi : s < M) :
    4 * (s : ℝ) ^ 2 / (9 * (2 * M)) ≤ mean M s ∧
      mean M s ≤ (s : ℝ) ^ 2 / (2 * M) := by
  have hm : (0 : ℝ) < M := by exact_mod_cast hM
  have hm0 : (M : ℝ) ≠ 0 := ne_of_gt hm
  have hs3 : (3 : ℝ) ≤ s := by exact_mod_cast hs
  constructor
  · apply le_trans _ (mean_lower M s hM (by omega))
    apply (div_le_div_iff₀ (by positivity : (0 : ℝ) < 9 * (2 * M))
      (by positivity : (0 : ℝ) < 3 * M)).mpr
    have hh : 0 ≤ (M : ℝ) * (s : ℝ) * ((s : ℝ) - 3) :=
      mul_nonneg (mul_nonneg hm.le (Nat.cast_nonneg s)) (sub_nonneg.mpr hs3)
    nlinarith
  · apply le_trans (mean_upper M s hM)
    apply (div_le_div_iff_of_pos_right (by positivity : (0 : ℝ) < 2 * M)).mpr
    have hs0 : (0 : ℝ) ≤ s := Nat.cast_nonneg s
    nlinarith

#print axioms mean_upper
#print axioms mean_lower
#print axioms repaired_window
end
end TheriaultIntersectionsAudit
