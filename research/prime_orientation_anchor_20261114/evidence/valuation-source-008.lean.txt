module
public import OrientationAnchor
import all OrientationAnchor
public section
noncomputable section
namespace OrientationAnchor
open Polynomial Finset

/-- The actual first differing position, computed from the signed displacement. -/
def order (x y : ℕ) : ℕ := padicValInt 2 ((y:ℤ)-x)

theorem delta_as_int (x y : ℕ) : delta x y = (((y:ℤ)-x : ℤ) : ℚ) := by
  simp [delta]

theorem signed_ne_zero (x y : ℕ) (hxy : x ≠ y) : (y:ℤ)-x ≠ 0 := by
  exact sub_ne_zero.mpr (by exact_mod_cast Ne.symm hxy)

/-- Divisibility through level s and failure at s+1 forces a bit disagreement. -/
theorem bit_diff_of_exact_divisibility (x y s : ℕ)
    (hlo : (2:ℤ)^s ∣ (y:ℤ)-x)
    (hhi : ¬(2:ℤ)^(s+1) ∣ (y:ℤ)-x) :
    x.testBit s ≠ y.testBit s := by
  intro hb
  have hmod : x ≡ y [MOD 2^s] := by
    apply Int.natCast_modEq_iff.mp
    apply Int.modEq_iff_dvd.mpr
    exact_mod_cast hlo
  have hbit : x / 2^s % 2 = y / 2^s % 2 := by
    simpa only [Nat.toNat_testBit] using congrArg Bool.toNat hb
  have hnext : x ≡ y [MOD 2^(s+1)] := by
    change x % 2^(s+1) = y % 2^(s+1)
    rw [Nat.mod_pow_succ, Nat.mod_pow_succ, hmod, hbit]
  apply hhi
  have h := Int.modEq_iff_dvd.mp (Int.natCast_modEq_iff.mpr hnext)
  exact_mod_cast h

/-- The valuation index is genuinely omitted by the orientation mask. -/
theorem order_bit_diff (x y : ℕ) (hxy : x ≠ y) :
    x.testBit (order x y) ≠ y.testBit (order x y) := by
  apply bit_diff_of_exact_divisibility
  · exact padicValInt_dvd _
  · intro h
    have hn := signed_ne_zero x y hxy
    have ht := (padicValInt_dvd_iff (p := 2) (order x y+1) ((y:ℤ)-x)).mp h
    rcases ht with hz | hv
    · exact hn hz
    · dsimp [order] at hv
      omega

theorem mask_omits_order (u x y : ℕ) (hxy : x ≠ y) : order x y ∉ mask u x y := by
  intro h
  exact order_bit_diff x y hxy (mem_filter.mp h).2

/-- Any earlier bit agrees, as a consequence of exact integer divisibility. -/
theorem bits_below_order (x y i : ℕ) (hi : i < order x y) :
    x.testBit i = y.testBit i := by
  have hdiv : (2:ℤ)^(i+1) ∣ (y:ℤ)-x :=
    (padicValInt_dvd_iff (p := 2) _ _).mpr (Or.inr (by change i+1 ≤ order x y; omega))
  have hm : x ≡ y [MOD 2^(i+1)] := by
    apply Int.natCast_modEq_iff.mp
    apply Int.modEq_iff_dvd.mpr
    exact_mod_cast hdiv
  have ht := congrArg (fun n : ℕ => n.testBit i) hm
  simpa using ht

/-- Noncrossings have a first differing position at or below the chart level. -/
theorem order_le_of_bit_diff (x y i : ℕ) (hbit : x.testBit i ≠ y.testBit i) :
    order x y ≤ i := by
  by_contra h
  exact hbit (bits_below_order x y i (by omega))

/-- Rational and signed-integer valuation conventions agree on the actual displacement. -/
theorem delta_valuation (x y : ℕ) : padicValRat 2 (delta x y) = (order x y : ℤ) := by
  rw [delta_as_int, padicValRat.of_int]
  rfl

/-- Exact anchor denominator valuation, derived from the actual mask. -/
theorem denominator_valuation (u x y j : ℕ) (hxy : x ≠ y)
    (hj : j ∈ mask u x y) :
    padicValRat 2 ((2:ℚ)^j-delta x y) = min (j:ℤ) (order x y) := by
  have hjn : j ≠ order x y := by
    intro h
    exact mask_omits_order u x y hxy (h ▸ hj)
  have hd : delta x y ≠ 0 := by rw [delta_as_int]; exact_mod_cast signed_ne_zero x y hxy
  have hpow : padicValRat 2 ((2:ℚ)^j) = (j:ℤ) := by
    rw [padicValRat.pow, show padicValRat 2 (2:ℚ)=1 from padicValRat.self (p:=2) (by decide), mul_one]
  rw [sub_eq_add_neg, padicValRat.add_eq_min
    (by simpa only [← sub_eq_add_neg] using denominator_ne_zero u x y j hj)
    (by positivity) (neg_ne_zero.mpr hd)
    (by rw [hpow, padicValRat.neg, delta_valuation]; exact_mod_cast hjn),
    hpow, padicValRat.neg, delta_valuation]

/-- The diagonal case has no zero denominator and its exact valuation is j. -/
theorem diagonal_denominator_valuation (x j : ℕ) :
    padicValRat 2 ((2:ℚ)^j-delta x x) = (j:ℤ) := by
  simp only [delta, sub_self, sub_zero, padicValRat.pow]
  rw [show padicValRat 2 (2:ℚ)=1 from padicValRat.self (p:=2) (by decide), mul_one]

end OrientationAnchor
