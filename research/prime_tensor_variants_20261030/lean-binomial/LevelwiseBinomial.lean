module

public import Mathlib.Data.Nat.Choose.Lucas
public import Mathlib.NumberTheory.Padics.PadicVal.Basic
public import Mathlib.RingTheory.Binomial

public section

namespace LevelwiseBinomial

open Polynomial

/-- The actual generalized binomial polynomial over the rationals. -/
noncomputable def binomialPolynomial (n : Nat) : ℚ[X] := Ring.choose X n

theorem binomialPolynomial_eval_int (n : Nat) (t : Int) :
    (binomialPolynomial n).eval (t : ℚ) = (Ring.choose t n : Int) := by
  calc
    (binomialPolynomial n).eval (t : ℚ) = Ring.choose (t : ℚ) n := by
      simpa [binomialPolynomial] using
        Ring.map_choose (Polynomial.evalRingHom (t : ℚ)) (X : ℚ[X]) n
    _ = ((Ring.choose t n : Int) : ℚ) :=
      (Ring.map_choose (Int.castRingHom ℚ) t n).symm

theorem choose_nat_pow_two_digit (n i : Nat) :
    (n.choose (2 ^ i) : ZMod 2) = ((n / 2 ^ i : Nat) : ZMod 2) := by
  letI : Fact (Nat.Prime 2) := ⟨by decide⟩
  induction i generalizing n with
  | zero => simp
  | succ i ih =>
    have h : (n.choose (2 ^ (i + 1)) : ZMod 2) =
        (((n % 2).choose (2 ^ (i + 1) % 2) *
          (n / 2).choose (2 ^ (i + 1) / 2) : Nat) : ZMod 2) := by
      exact (ZMod.natCast_eq_natCast_iff _ _ _).mpr
        (Choose.choose_modEq_choose_mod_mul_choose_div_nat (p := 2))
    have hr : 2 ^ (i + 1) % 2 = 0 := by simp [pow_succ]
    have hq : 2 ^ (i + 1) / 2 = 2 ^ i := by simp [pow_succ]
    rw [hr, hq, Nat.choose_zero_right, one_mul] at h
    rw [h, ih]
    congr 1
    rw [Nat.div_div_eq_div_mul, pow_succ']

/-- The generalized binomial's parity is the actual signed binary digit. -/
theorem choose_int_pow_two_digit (t : Int) (i : Nat) :
    ((Ring.choose t (2 ^ i) : Int) : ZMod 2) =
      ((t / (2 : Int) ^ i : Int) : ZMod 2) := by
  have hdiv (n : Nat) : ((n : Int) / (2 : Int) ^ i : Int) = (n / 2 ^ i : Nat) := by
    have hp : (2 : Int) ^ i = ((2 ^ i : Nat) : Int) := by norm_cast
    rw [hp]
    rfl
  cases t with
  | ofNat n =>
    rw [Int.ofNat_eq_natCast, Ring.choose_natCast, hdiv]
    simpa only [Int.cast_natCast] using choose_nat_pow_two_digit n i
  | negSucc n =>
    have harg : ((n + 1 : Nat) : Int) + (2 ^ i : Nat) - 1 = (n + 2 ^ i : Nat) := by
      push_cast
      ring
    have ht : Int.negSucc n = -((n + 1 : Nat) : Int) := rfl
    rw [ht, Ring.choose_neg, harg, Ring.choose_natCast]
    simp only [Units.smul_def, zsmul_eq_mul, Int.cast_mul, Int.cast_natCast,
      Int.coe_negOnePow, Int.natAbs_natCast, Int.cast_pow, Int.cast_neg, Int.cast_one]
    rw [show (-1 : ZMod 2) = 1 from by decide, one_pow, one_mul,
      choose_nat_pow_two_digit]
    have hpos : (0 : Int) < (2 : Int) ^ i := pow_pos (by decide) i
    rw [← ht, Int.negSucc_ediv n hpos]
    have hraw : (n : Int).ediv ((2 : Int) ^ i) = (n / 2 ^ i : Nat) := hdiv n
    rw [hraw]
    simp only [Int.cast_neg, Int.cast_add, Int.cast_one, ZMod.neg_eq_self_mod_two,
      ← Int.natCast_pow, ← Int.natCast_ediv, Int.cast_natCast]
    rw [Nat.add_div_right _ (pow_pos (by decide) i)]
    simp

/-- Dividing out the maximal power removes divisibility by the base. -/
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

/-- The requested signed-node parity lemma, at the actual integer valuation. -/
theorem choose_int_valuation_odd (t : Int) (ht : t ≠ 0) :
    Odd (Ring.choose t (2 ^ padicValInt 2 t)) := by
  apply ZMod.intCast_eq_one_iff_odd.mp
  rw [choose_int_pow_two_digit]
  apply ZMod.intCast_eq_one_iff_odd.mpr
  apply Int.not_even_iff_odd.mp
  rw [even_iff_two_dvd]
  exact quotient_not_dvd (p := 2) (by decide) t ht

theorem choose_int_at_valuation_odd (t : Int) (ht : t ≠ 0) (i : Nat)
    (hi : padicValInt 2 t = i) : Odd (Ring.choose t (2 ^ i)) := by
  simpa only [hi] using choose_int_valuation_odd t ht

/-- The actual level-dependent family from the written construction. -/
noncomputable def levelPolynomial (i p : Nat) : ℚ[X] :=
  (1 - binomialPolynomial (2 ^ i)) ^ p

theorem levelPolynomial_eval_int (i p : Nat) (t : Int) :
    (levelPolynomial i p).eval (t : ℚ) =
      (((1 - Ring.choose t (2 ^ i)) ^ p : Int) : ℚ) := by
  simp only [levelPolynomial, Polynomial.eval_pow, Polynomial.eval_sub,
    Polynomial.eval_one, binomialPolynomial_eval_int, Int.cast_pow,
    Int.cast_sub, Int.cast_one]

theorem levelPolynomial_origin (i p : Nat) : (levelPolynomial i p).eval 0 = 1 := by
  have h := levelPolynomial_eval_int i p 0
  rw [Ring.choose_zero_pos Int (pow_pos (by decide) i)] at h
  simpa using h

/-- The node value is divisible by the claimed power. This theorem concerns
the chart's constant coefficient; full chart-coefficient divisibility is not
asserted or assumed here. -/
theorem level_integer_value_divisible (i p : Nat) (t : Int) (ht : t ≠ 0)
    (hi : padicValInt 2 t = i) :
    (2 : Int) ^ p ∣ (1 - Ring.choose t (2 ^ i)) ^ p := by
  have hc : ((Ring.choose t (2 ^ i) : Int) : ZMod 2) = 1 :=
    ZMod.intCast_eq_one_iff_odd.mpr (choose_int_at_valuation_odd t ht i hi)
  have hd : (2 : Int) ∣ 1 - Ring.choose t (2 ^ i) := by
    apply (ZMod.intCast_zmod_eq_zero_iff_dvd _ 2).mp
    rw [Int.cast_sub, Int.cast_one, hc, sub_self]
  exact pow_dvd_pow_of_dvd hd p

end LevelwiseBinomial

#print axioms LevelwiseBinomial.binomialPolynomial_eval_int
#print axioms LevelwiseBinomial.choose_nat_pow_two_digit
#print axioms LevelwiseBinomial.choose_int_pow_two_digit
#print axioms LevelwiseBinomial.quotient_not_dvd
#print axioms LevelwiseBinomial.choose_int_valuation_odd
#print axioms LevelwiseBinomial.choose_int_at_valuation_odd
#print axioms LevelwiseBinomial.levelPolynomial_eval_int
#print axioms LevelwiseBinomial.levelPolynomial_origin
#print axioms LevelwiseBinomial.level_integer_value_divisible
