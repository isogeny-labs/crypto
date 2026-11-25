import Cells

namespace XedniDuplicationGlobal

open XedniDuplicationCells

private theorem outer_quartic (u : ℝ) (hu : 128 ≤ u) :
    u ^ 4 / 1024 ≤ u ^ 4 - 1502 * u ^ 2 - 31976 * u - 187435 := by
  have hu₂ : (128 : ℝ) ^ 2 ≤ u ^ 2 := by gcongr
  have hu₃ : (128 : ℝ) ^ 3 ≤ u ^ 3 := by gcongr
  have hu₄ : (128 : ℝ) ^ 4 ≤ u ^ 4 := by gcongr
  have h₂ : (12016 : ℝ) ≤ u ^ 2 := by norm_num at hu₂; linarith
  have h₃ : (511616 : ℝ) ≤ u ^ 3 := by norm_num at hu₃; linarith
  have h₄ : (2998960 : ℝ) ≤ u ^ 4 := by norm_num at hu₄; linarith
  have hterm₂ : 1502 * u ^ 2 ≤ u ^ 4 / 8 := by
    have hp := mul_nonneg (sub_nonneg.mpr h₂) (sq_nonneg u)
    nlinarith
  have hterm₃ : 31976 * u ≤ u ^ 4 / 16 := by
    have hp := mul_nonneg (sub_nonneg.mpr h₃) (by linarith : 0 ≤ u)
    nlinarith
  have hterm₄ : (187435 : ℝ) ≤ u ^ 4 / 16 := by linarith
  nlinarith [pow_nonneg (by linarith : 0 ≤ u) 4]

private theorem outer_positive (t : ℝ) (ht : 128 ≤ t) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have ht₀ : 0 ≤ t := by linarith
  have hmax : max (1 : ℝ) |t| = t := by
    rw [abs_of_nonneg ht₀]
    exact max_eq_right (by linarith)
  have hF : t ^ 4 / 1024 ≤ F_real t := by
    have h := outer_quartic t ht
    dsimp [F_real]
    nlinarith
  calc
    (max 1 |t|) ^ 4 / 1024 = t ^ 4 / 1024 := by rw [hmax]
    _ ≤ F_real t := hF
    _ ≤ |F_real t| := le_abs_self _
    _ ≤ max |F_real t| |G_real t| := le_max_left _ _

private theorem outer_negative (t : ℝ) (ht : t ≤ -128) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  have ht₀ : t ≤ 0 := by linarith
  have hmax : max (1 : ℝ) |t| = -t := by
    rw [abs_of_nonpos ht₀]
    exact max_eq_right (by linarith)
  have hF : (-t) ^ 4 / 1024 ≤ F_real t := by
    convert outer_quartic (-t) (by linarith) using 1 <;> dsimp [F_real] <;> ring
  calc
    (max 1 |t|) ^ 4 / 1024 = (-t) ^ 4 / 1024 := by rw [hmax]
    _ ≤ F_real t := hF
    _ ≤ |F_real t| := le_abs_self _
    _ ≤ max |F_real t| |G_real t| := le_max_left _ _

theorem global_univariate_lower (t : ℝ) :
    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by
  by_cases hl : (-128 : ℝ) ≤ t
  · by_cases hu : t ≤ 128
    · exact finite_archimedean_lower t hl hu
    · exact outer_positive t (by linarith)
  · exact outer_negative t (by linarith)

def F_h (A B : ℝ) : ℝ :=
  A ^ 4 - 1502 * A ^ 2 * B ^ 2 + 31976 * A * B ^ 3 - 187435 * B ^ 4

def G_h (A B : ℝ) : ℝ :=
  4 * A ^ 3 * B - 188 * A ^ 2 * B ^ 2 + 3004 * A * B ^ 3 - 15988 * B ^ 4

theorem global_homogeneous_lower (A B : ℝ) :
    (max |A| |B|) ^ 4 / 1024 ≤ max |F_h A B| |G_h A B| := by
  by_cases hB : B = 0
  · subst B
    simp only [F_h, G_h, mul_zero, zero_pow (by norm_num : (4 : ℕ) ≠ 0),
      zero_pow (by norm_num : (2 : ℕ) ≠ 0), zero_pow (by norm_num : (3 : ℕ) ≠ 0),
      sub_zero, add_zero, abs_zero, max_zero, abs_pow]
    have hA : (0 : ℝ) ≤ |A| ^ 4 := pow_nonneg (abs_nonneg _) _
    have hmax : max (|A| ^ 4) 0 = |A| ^ 4 := max_eq_left hA
    rw [hmax, max_eq_left (abs_nonneg A)]
    nlinarith [pow_nonneg (abs_nonneg A) 4]
  · let t : ℝ := A / B
    have hF : F_h A B = B ^ 4 * F_real t := by
      dsimp [F_h, F_real, t]
      field_simp
    have hG : G_h A B = B ^ 4 * G_real t := by
      dsimp [G_h, G_real, t]
      field_simp
    have hA : |A| = |B| * |t| := by
      dsimp [t]
      rw [abs_div]
      field_simp [abs_ne_zero.mpr hB]
    have hM : max |A| |B| = |B| * max 1 |t| := by
      rw [hA]
      calc
        max (|B| * |t|) |B| = max (|B| * |t|) (|B| * 1) := by ring
        _ = |B| * max |t| 1 := (mul_max_of_nonneg _ _ (abs_nonneg B)).symm
        _ = |B| * max 1 |t| := by rw [max_comm]
    have hForms : max |F_h A B| |G_h A B| =
        |B| ^ 4 * max |F_real t| |G_real t| := by
      rw [hF, hG, abs_mul, abs_mul, abs_pow]
      exact (mul_max_of_nonneg _ _ (pow_nonneg (abs_nonneg B) 4)).symm
    rw [hM, hForms, mul_pow]
    have hcell := global_univariate_lower t
    calc
      |B| ^ 4 * (max 1 |t|) ^ 4 / 1024 =
          |B| ^ 4 * ((max 1 |t|) ^ 4 / 1024) := by ring
      _ ≤ |B| ^ 4 * max |F_real t| |G_real t| :=
        mul_le_mul_of_nonneg_left hcell (pow_nonneg (abs_nonneg B) 4)

#print axioms global_univariate_lower
#print axioms global_homogeneous_lower

end XedniDuplicationGlobal
