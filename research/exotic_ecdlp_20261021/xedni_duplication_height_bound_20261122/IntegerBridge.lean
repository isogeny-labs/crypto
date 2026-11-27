import Global

namespace XedniDuplicationIntegerBridge

open XedniDuplicationBound XedniDuplicationGlobal

private theorem F_cast (A B : ℤ) :
    F_h (A : ℝ) (B : ℝ) = (F A B : ℝ) := by
  unfold F_h F
  push_cast
  ring

private theorem G_cast (A B : ℤ) :
    G_h (A : ℝ) (B : ℝ) = (G A B : ℝ) := by
  unfold G_h G
  push_cast
  ring

theorem integer_homogeneous_lower (A B : ℤ) :
    (max |(A : ℝ)| |(B : ℝ)|) ^ 4 / 1024 ≤
      max |(F A B : ℝ)| |(G A B : ℝ)| := by
  simpa only [F_cast, G_cast] using
    global_homogeneous_lower (A : ℝ) (B : ℝ)

theorem reduced_forms_lower
    (A B d f g : ℤ) (hAB : Int.gcd A B = 1) (hd : 0 < d)
    (hf : F A B = d * f) (hg : G A B = d * g) :
    (max |(A : ℝ)| |(B : ℝ)|) ^ 4 ≤
      1024 * 8038359040000 * max |(f : ℝ)| |(g : ℝ)| := by
  have hdivF : d ∣ F A B := by rw [hf]; exact dvd_mul_right d f
  have hdivG : d ∣ G A B := by rw [hg]; exact dvd_mul_right d g
  have hresult := common_divisor_dvd_resultant A B d hAB hdivF hdivG
  have hdR : d ≤ (8038359040000 : ℤ) :=
    Int.le_of_dvd (by norm_num) hresult
  have hdRreal : (d : ℝ) ≤ 8038359040000 := by exact_mod_cast hdR
  have hdreal : (0 : ℝ) ≤ d := by exact_mod_cast (le_of_lt hd)
  have hmax : max |(f : ℝ)| |(g : ℝ)| ≥ 0 :=
    le_trans (abs_nonneg _) (le_max_left _ _)
  have hforms : max |(F A B : ℝ)| |(G A B : ℝ)| =
      (d : ℝ) * max |(f : ℝ)| |(g : ℝ)| := by
    rw [hf, hg]
    push_cast
    rw [abs_mul, abs_mul, abs_of_nonneg hdreal]
    exact (mul_max_of_nonneg _ _ hdreal).symm
  have hlow := integer_homogeneous_lower A B
  rw [hforms] at hlow
  have hscale := mul_nonneg (sub_nonneg.mpr hdRreal) hmax
  nlinarith

#print axioms integer_homogeneous_lower
#print axioms reduced_forms_lower

end XedniDuplicationIntegerBridge
