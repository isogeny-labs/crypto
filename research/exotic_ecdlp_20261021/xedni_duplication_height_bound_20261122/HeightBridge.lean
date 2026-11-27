import UpperBridge

namespace XedniDuplicationHeightBridge

open Height XedniDuplicationBound XedniDuplicationPointBridge
  XedniDuplicationRationalBridge XedniDuplicationOutputBridge
  XedniDuplicationUpperBridge

theorem point_lower_height_defect {x y : ℚ}
    (h : FixedCurve.Nonsingular x y) (hn : y ≠ FixedCurve.negY x y) :
    4 * logHeight₁ x -
      (WeierstrassCurve.Affine.Point.some x y h +
        WeierstrassCurve.Affine.Point.some x y h).naiveHeight ≤
      53 * Real.log 2 := by
  let q : ℚ := (F x.num x.den : ℚ) / (G x.num x.den : ℚ)
  let M : ℝ := (max x.num.natAbs x.den : ℕ)
  let N : ℝ := (max q.num.natAbs q.den : ℕ)
  have hM : max |(x.num : ℝ)| |(x.den : ℝ)| = M := by
    simp [M, Nat.cast_max, Nat.cast_natAbs]
  have hN : max |(q.num : ℝ)| |(q.den : ℝ)| = N := by
    simp [N, Nat.cast_max, Nat.cast_natAbs]
  have hMpos : 0 < M := by
    rw [← hM]
    exact lt_of_lt_of_le (by exact_mod_cast (Rat.pos x)) (le_max_right _ _)
  have hNpos : 0 < N := by
    rw [← hN]
    exact lt_of_lt_of_le (by exact_mod_cast (Rat.pos q)) (le_max_right _ _)
  have hraw := point_canonical_output_lower h hn
  dsimp at hraw
  rw [hM, hN] at hraw
  have hcap : (1024 * 8038359040000 : ℝ) ≤ 2 ^ 53 := by norm_num
  have hpow : M ^ 4 ≤ (2 : ℝ) ^ 53 * N := by
    nlinarith [mul_nonneg (sub_nonneg.mpr hcap) (le_of_lt hNpos)]
  have hlog := Real.log_le_log (pow_pos hMpos _) hpow
  rw [Real.log_pow, Real.log_mul (pow_ne_zero _ (by norm_num : (2 : ℝ) ≠ 0))
    (ne_of_gt hNpos), Real.log_pow] at hlog
  have hinput : logHeight₁ x = Real.log M := by
    simpa [M] using Rat.logHeight₁_eq_log_max x
  have houtput :
      (WeierstrassCurve.Affine.Point.some x y h +
        WeierstrassCurve.Affine.Point.some x y h).naiveHeight = Real.log N := by
    rw [naiveHeight_double_normalized h hn]
    simpa [N, q] using Rat.logHeight₁_eq_log_max q
  rw [hinput, houtput]
  exact sub_le_iff_le_add.mpr hlog

#print axioms point_lower_height_defect

theorem point_upper_height_defect {x y : ℚ}
    (h : FixedCurve.Nonsingular x y) (hn : y ≠ FixedCurve.negY x y) :
    (WeierstrassCurve.Affine.Point.some x y h +
      WeierstrassCurve.Affine.Point.some x y h).naiveHeight -
      4 * logHeight₁ x ≤ 18 * Real.log 2 := by
  let q : ℚ := (F x.num x.den : ℚ) / (G x.num x.den : ℚ)
  let M : ℝ := (max x.num.natAbs x.den : ℕ)
  let N : ℝ := (max q.num.natAbs q.den : ℕ)
  have hM : max |(x.num : ℝ)| |(x.den : ℝ)| = M := by
    simp [M, Nat.cast_max, Nat.cast_natAbs]
  have hN : max |(q.num : ℝ)| |(q.den : ℝ)| = N := by
    simp [N, Nat.cast_max, Nat.cast_natAbs]
  have hMpos : 0 < M := by
    rw [← hM]
    exact lt_of_lt_of_le (by exact_mod_cast (Rat.pos x)) (le_max_right _ _)
  have hNpos : 0 < N := by
    rw [← hN]
    exact lt_of_lt_of_le (by exact_mod_cast (Rat.pos q)) (le_max_right _ _)
  have hraw := point_canonical_output_upper h hn
  dsimp at hraw
  rw [hM, hN] at hraw
  have hlog := Real.log_le_log hNpos hraw
  rw [Real.log_mul (pow_ne_zero _ (by norm_num : (2 : ℝ) ≠ 0))
    (ne_of_gt (pow_pos hMpos _)), Real.log_pow, Real.log_pow] at hlog
  have hinput : logHeight₁ x = Real.log M := by
    simpa [M] using Rat.logHeight₁_eq_log_max x
  have houtput :
      (WeierstrassCurve.Affine.Point.some x y h +
        WeierstrassCurve.Affine.Point.some x y h).naiveHeight = Real.log N := by
    rw [naiveHeight_double_normalized h hn]
    simpa [N, q] using Rat.logHeight₁_eq_log_max q
  rw [hinput, houtput]
  exact sub_le_iff_le_add.mpr hlog

#print axioms point_upper_height_defect

theorem point_affine_abs_height_defect {x y : ℚ}
    (h : FixedCurve.Nonsingular x y) (hn : y ≠ FixedCurve.negY x y) :
    |(WeierstrassCurve.Affine.Point.some x y h +
        WeierstrassCurve.Affine.Point.some x y h).naiveHeight -
        4 * logHeight₁ x| ≤ 53 * Real.log 2 := by
  have hlo := point_lower_height_defect h hn
  have hhi := point_upper_height_defect h hn
  have hlog : 0 ≤ Real.log 2 := Real.log_nonneg (by norm_num)
  exact abs_le.mpr (by constructor <;> linarith)

#print axioms point_affine_abs_height_defect

end XedniDuplicationHeightBridge
