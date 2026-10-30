module

public import TernaryPacket
public import Mathlib.NumberTheory.Padics.PadicVal.Basic

public section

/-!
A versioned addition to the frozen ternary packet label formalization.
Digits use Euclidean division of arbitrary signed integers. No representative
range is needed for the digit-addition identity when the difference is
divisible by the selected power of three.
-/

namespace TernaryDigits

open TernaryPacket

/-- The base-three digit at coordinate i, interpreted in ZMod 3. -/
def digit (i : Nat) (x : Int) : Digit := (x / (3 : Int) ^ i : Int)

/-- Exact signed-integer division identity; the divisibility assumption is
essential, since unrestricted floor-division does not distribute over sums. -/
theorem digit_add_of_dvd (i : Nat) (x t : Int) (hdiv : (3 : Int) ^ i ∣ t) :
    digit i (x + t) = digit i x + (t / (3 : Int) ^ i : Int) := by
  unfold digit
  rw [Int.add_ediv_of_dvd_right hdiv, Int.cast_add]

theorem digit_difference_of_dvd (i : Nat) (x y : Int)
    (hdiv : (3 : Int) ^ i ∣ y - x) :
    digit i y = digit i x + ((y - x) / (3 : Int) ^ i : Int) := by
  have he : x + (y - x) = y := by abel
  simpa only [he] using digit_add_of_dvd i x (y - x) hdiv

/-- The leading digit is defined from the actual Mathlib integer valuation. -/
def leadingDigit (t : Int) : Digit :=
  (t / (3 : Int) ^ padicValInt 3 t : Int)

theorem digit_difference_at_valuation (x y : Int) :
    digit (padicValInt 3 (y - x)) y =
      digit (padicValInt 3 (y - x)) x + leadingDigit (y - x) := by
  exact digit_difference_of_dvd _ x y (padicValInt_dvd (p := 3) (y - x))

/-- Normalizing by the maximal dividing power removes divisibility by p.
No primality assumption is needed for this integer statement. -/
theorem quotient_not_dvd {p : Nat} (hp : p ≠ 1) (t : Int) (ht : t ≠ 0) :
    ¬ (p : Int) ∣ t / (p : Int) ^ padicValInt p t := by
  intro hd
  have hv : (p : Int) ^ padicValInt p t ∣ t := padicValInt_dvd t
  have hbig : (p : Int) ^ (padicValInt p t + 1) ∣ t := by
    have hm := mul_dvd_mul_right hd ((p : Int) ^ padicValInt p t)
    rw [← pow_succ'] at hm
    simpa only [Int.ediv_mul_cancel_of_dvd hv] using hm
  rcases (padicValInt_dvd_iff_of_ne_one hp (padicValInt p t + 1) t).mp hbig with hz | hle
  · exact ht hz
  · exact Nat.not_succ_le_self _ hle

theorem leadingDigit_ne_zero (t : Int) (ht : t ≠ 0) : leadingDigit t ≠ 0 := by
  intro h
  exact quotient_not_dvd (p := 3) (by decide) t ht
    ((ZMod.intCast_zmod_eq_zero_iff_dvd _ _).mp h)

/-- Extract the first a digits of an integer, allowing signed integers. -/
def integerSlot (a : Nat) (x : Int) : Fin a → Digit := fun i => digit i.val x

/-- Actual integer valuation-to-locality bridge with its exact finite-index
bound explicit. Labels are arbitrary additive-commutative-group labels. -/
theorem integer_slot_locality {A : Type*} [AddCommGroup A] {a : Nat}
    (g : Fin a → Tag A) (x y : Int) (hne : y - x ≠ 0)
    (hi : padicValInt 3 (y - x) < a) :
    row (leadingDigit (y - x))
        (slotAction (integerSlot a x) g ⟨padicValInt 3 (y - x), hi⟩) =
      col (leadingDigit (y - x))
        (slotAction (integerSlot a y) g ⟨padicValInt 3 (y - x), hi⟩) := by
  apply slot_locality g (integerSlot a x) (integerSlot a y)
    ⟨padicValInt 3 (y - x), hi⟩ (leadingDigit (y - x))
    (leadingDigit_ne_zero (y - x) hne)
  exact digit_difference_at_valuation x y

/-- Distinct ordinary slots in [0,3^a) have a valuation coordinate below a. -/
theorem valuation_lt_of_bounded_slots (a : Nat) (x y : Int)
    (hx0 : 0 ≤ x) (hx : x < (3 : Int) ^ a)
    (hy0 : 0 ≤ y) (hy : y < (3 : Int) ^ a) (hne : y ≠ x) :
    padicValInt 3 (y - x) < a := by
  by_contra h
  have hle : a ≤ padicValInt 3 (y - x) := Nat.le_of_not_gt h
  have hdiv : (3 : Int) ^ a ∣ y - x :=
    dvd_trans (pow_dvd_pow (3 : Int) hle) (padicValInt_dvd (p := 3) (y - x))
  have habs : (y - x).natAbs < ((3 : Int) ^ a).natAbs := by
    rcases le_total x y with hxy | hyx
    · apply Int.natAbs_lt_natAbs_of_nonneg_of_lt (sub_nonneg.mpr hxy)
      omega
    · calc
        (y - x).natAbs = (x - y).natAbs := by
          rw [← Int.natAbs_neg (y - x), neg_sub]
        _ < ((3 : Int) ^ a).natAbs := by
          apply Int.natAbs_lt_natAbs_of_nonneg_of_lt (sub_nonneg.mpr hyx)
          omega
  exact hne (sub_eq_zero.mp (Int.eq_zero_of_dvd_of_natAbs_lt_natAbs hdiv habs))

/-- For the actual bounded packet slots, no valuation-index bound is assumed:
the preceding theorem constructs the required Fin a coordinate. -/
theorem bounded_integer_slot_locality {A : Type*} [AddCommGroup A] {a : Nat}
    (g : Fin a → Tag A) (x y : Int)
    (hx0 : 0 ≤ x) (hx : x < (3 : Int) ^ a)
    (hy0 : 0 ≤ y) (hy : y < (3 : Int) ^ a) (hne : y ≠ x) :
    let i : Fin a :=
      ⟨padicValInt 3 (y - x), valuation_lt_of_bounded_slots a x y hx0 hx hy0 hy hne⟩
    row (leadingDigit (y - x)) (slotAction (integerSlot a x) g i) =
      col (leadingDigit (y - x)) (slotAction (integerSlot a y) g i) := by
  exact integer_slot_locality g x y (sub_ne_zero.mpr hne)
    (valuation_lt_of_bounded_slots a x y hx0 hx hy0 hy hne)

end TernaryDigits

#print axioms TernaryDigits.digit_add_of_dvd
#print axioms TernaryDigits.digit_difference_of_dvd
#print axioms TernaryDigits.digit_difference_at_valuation
#print axioms TernaryDigits.quotient_not_dvd
#print axioms TernaryDigits.leadingDigit_ne_zero
#print axioms TernaryDigits.integer_slot_locality
#print axioms TernaryDigits.valuation_lt_of_bounded_slots
#print axioms TernaryDigits.bounded_integer_slot_locality
