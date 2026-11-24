import RationalBridge

namespace XedniDuplicationOutputBridge

open XedniDuplicationBound XedniDuplicationIntegerBridge
  XedniDuplicationRationalBridge XedniDuplicationPointBridge

theorem canonical_output_lower (A B : ℤ) (hAB : Int.gcd A B = 1)
    (hG : G A B ≠ 0) :
    let q : ℚ := (F A B : ℚ) / (G A B : ℚ)
    (max |(A : ℝ)| |(B : ℝ)|) ^ 4 ≤
      1024 * 8038359040000 * max |(q.num : ℝ)| |(q.den : ℝ)| := by
  dsimp
  let q : ℚ := (F A B : ℚ) / (G A B : ℚ)
  have hqdf : q = Rat.divInt (F A B) (G A B) := by
    simp [q, Rat.divInt_eq_div]
  obtain ⟨c, hFn, hGn⟩ := Rat.num_den_mk hG hqdf
  have hc : c ≠ 0 := by
    intro hz
    apply hG
    simpa [hz] using hGn
  have hcF : c ∣ F A B := by rw [hFn]; exact dvd_mul_right c _
  have hcG : c ∣ G A B := by rw [hGn]; exact dvd_mul_right c _
  have hcR := common_divisor_dvd_resultant A B c hAB hcF hcG
  have hcabsR : |c| ∣ (8038359040000 : ℤ) := (abs_dvd _ _).mpr hcR
  have hcabspos : (0 : ℤ) < |c| := abs_pos.mpr hc
  have hcRbound : |c| ≤ (8038359040000 : ℤ) :=
    Int.le_of_dvd (by norm_num) hcabsR
  have hcRreal : |(c : ℝ)| ≤ 8038359040000 := by exact_mod_cast hcRbound
  have hmax : 0 ≤ max |(q.num : ℝ)| |(q.den : ℝ)| :=
    le_trans (abs_nonneg _) (le_max_left _ _)
  have hforms : max |(F A B : ℝ)| |(G A B : ℝ)| =
      |(c : ℝ)| * max |(q.num : ℝ)| |(q.den : ℝ)| := by
    rw [hFn, hGn]
    push_cast
    rw [abs_mul, abs_mul]
    exact (mul_max_of_nonneg _ _ (abs_nonneg (c : ℝ))).symm
  have hlow := integer_homogeneous_lower A B
  rw [hforms] at hlow
  have hscale := mul_nonneg (sub_nonneg.mpr hcRreal) hmax
  nlinarith

theorem point_canonical_output_lower {x y : ℚ}
    (h : FixedCurve.Nonsingular x y) (hn : y ≠ FixedCurve.negY x y) :
    let q : ℚ := (F x.num x.den : ℚ) / (G x.num x.den : ℚ)
    (max |(x.num : ℝ)| |(x.den : ℝ)|) ^ 4 ≤
      1024 * 8038359040000 * max |(q.num : ℝ)| |(q.den : ℝ)| :=
  canonical_output_lower x.num x.den (primitive_x x)
    (denominator_ne_of_double_ne h hn)

#print axioms canonical_output_lower
#print axioms point_canonical_output_lower

end XedniDuplicationOutputBridge
