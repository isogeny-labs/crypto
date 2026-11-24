import PointBridge
import IntegerBridge

namespace XedniDuplicationRationalBridge

open Height XedniDuplicationBound XedniDuplicationPointBridge

private def numForm (x : ℚ) : ℚ :=
  x ^ 4 - 1502 * x ^ 2 + 31976 * x - 187435

private def denForm (x : ℚ) : ℚ :=
  4 * x ^ 3 - 188 * x ^ 2 + 3004 * x - 15988

theorem primitive_x (x : ℚ) : Int.gcd x.num x.den = 1 := by
  exact Int.isCoprime_iff_gcd_eq_one.mp (Rat.isCoprime_num_den x)

private theorem numForm_normalized (x : ℚ) :
    numForm x = (F x.num x.den : ℚ) / (x.den : ℚ) ^ 4 := by
  have hd : (x.den : ℚ) ≠ 0 := by exact_mod_cast x.den_nz
  conv_lhs => rw [← Rat.num_div_den x]
  unfold numForm F
  field_simp
  push_cast
  ring

private theorem denForm_normalized (x : ℚ) :
    denForm x = (G x.num x.den : ℚ) / (x.den : ℚ) ^ 4 := by
  have hd : (x.den : ℚ) ≠ 0 := by exact_mod_cast x.den_nz
  conv_lhs => rw [← Rat.num_div_den x]
  unfold denForm G
  field_simp
  push_cast
  ring

theorem duplication_fraction_normalized (x : ℚ) :
    numForm x / denForm x =
      (F x.num x.den : ℚ) / (G x.num x.den : ℚ) := by
  rw [numForm_normalized, denForm_normalized]
  have hd : (x.den : ℚ) ^ 4 ≠ 0 := by
    exact pow_ne_zero _ (by exact_mod_cast x.den_nz)
  by_cases hG : (G x.num x.den : ℚ) = 0
  · simp [hG]
  · field_simp

theorem naiveHeight_double_normalized {x y : ℚ}
    (h : FixedCurve.Nonsingular x y) (hn : y ≠ FixedCurve.negY x y) :
    (WeierstrassCurve.Affine.Point.some x y h +
      WeierstrassCurve.Affine.Point.some x y h).naiveHeight =
      logHeight₁ ((F x.num x.den : ℚ) / (G x.num x.den : ℚ)) := by
  rw [naiveHeight_double_ne h hn]
  exact congrArg logHeight₁ (duplication_fraction_normalized x)

#print axioms primitive_x
#print axioms duplication_fraction_normalized
#print axioms naiveHeight_double_normalized

end XedniDuplicationRationalBridge
