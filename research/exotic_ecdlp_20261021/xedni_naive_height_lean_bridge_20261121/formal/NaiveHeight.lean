import Mathlib.NumberTheory.Height.EllipticCurve
import Mathlib.NumberTheory.Height.NumberField
import FixedCurveSquareDenominator

namespace XedniNaiveHeightBridge

open XedniFixedCurveSquareDenominator
open Height

def CoordinateHeightAtMost (P : FixedCurve.Point) (H : ℕ) : Prop :=
  match P with
  | .zero => False
  | .some x _ _ => ∃ b : ℕ, 0 < b ∧ x.den = b ^ 2 ∧ max x.num.natAbs b ≤ H

theorem exists_coordinate_height (P : FixedCurve.Point) (hP : P ≠ 0) :
    ∃ H : ℕ, CoordinateHeightAtMost P H := by
  cases P with
  | zero => exact (hP rfl).elim
  | some x y hn =>
    have hcurve : y ^ 2 = x ^ 3 + (-47) * x ^ 2 + 751 * x + (-3997) := by
      have heq := (FixedCurve.equation_iff x y).mp hn.1
      simpa [FixedCurve] using heq
    obtain ⟨b, hb⟩ := square_denominator_of_integral_affine
      (-47) 751 (-3997) x y (by simpa only [Int.cast_neg, Int.cast_ofNat] using hcurve)
    have hden : x.den = b ^ 2 := by simpa [pow_two] using hb
    have hbpos : 0 < b := by
      by_contra h
      have hzero : b = 0 := by omega
      simp [hzero] at hden
    exact ⟨max x.num.natAbs b, ⟨b, hbpos, hden, le_refl _⟩⟩

theorem naiveHeight_eq_log_coordinate (x y : ℚ)
    (hn : FixedCurve.Nonsingular x y) :
    (WeierstrassCurve.Affine.Point.some x y hn).naiveHeight =
      Real.log ((max x.num.natAbs x.den : ℕ) : ℝ) := by
  rw [WeierstrassCurve.Affine.Point.naiveHeight_eq_logHeight₁]
  simpa [WeierstrassCurve.Affine.Point.xRep] using Rat.logHeight₁_eq_log_max x

theorem coordinate_cap_implies_naiveHeight_bound (P : FixedCurve.Point)
    (hcap : CoordinateHeightAtMost P (2 ^ 130)) :
    P.naiveHeight ≤ 260 * Real.log 2 := by
  cases P with
  | zero => exact hcap.elim
  | some x y hn =>
    obtain ⟨b, hbpos, hden, hmax⟩ := hcap
    have ha : x.num.natAbs ≤ 2 ^ 130 := le_trans (le_max_left _ _) hmax
    have hb : b ≤ 2 ^ 130 := le_trans (le_max_right _ _) hmax
    have hb2 : b ^ 2 ≤ (2 ^ 130 : ℕ) ^ 2 := Nat.pow_le_pow_left hb 2
    have hbase : (2 ^ 130 : ℕ) ≤ (2 ^ 130 : ℕ) ^ 2 := by
      nlinarith [show 1 ≤ (2 ^ 130 : ℕ) by norm_num]
    have hpow : ((2 : ℕ) ^ 130) ^ 2 = 2 ^ 260 := by
      rw [← pow_mul]
    have hnum : max x.num.natAbs x.den ≤ 2 ^ 260 := by
      rw [hden]
      apply max_le
      · exact le_trans ha (hpow ▸ hbase)
      · exact hpow ▸ hb2
    have hpositive : (0 : ℝ) < (max x.num.natAbs x.den : ℕ) := by
      rw [hden]
      exact_mod_cast (lt_of_lt_of_le (Nat.pow_pos hbpos) (le_max_right _ _))
    rw [naiveHeight_eq_log_coordinate x y hn]
    calc
      Real.log ((max x.num.natAbs x.den : ℕ) : ℝ) ≤ Real.log ((2 ^ 260 : ℕ) : ℝ) :=
        Real.log_le_log hpositive (by exact_mod_cast hnum)
      _ = 260 * Real.log 2 := by
        simp only [Nat.cast_pow, Nat.cast_ofNat, Real.log_pow]

#print axioms exists_coordinate_height
#print axioms naiveHeight_eq_log_coordinate
#print axioms coordinate_cap_implies_naiveHeight_bound

end XedniNaiveHeightBridge
