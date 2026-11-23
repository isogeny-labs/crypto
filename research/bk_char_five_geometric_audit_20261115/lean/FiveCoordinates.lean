module
public import S3Completion
public import Eliminant

@[expose] public section
namespace BKCharFiveGeometricAudit
noncomputable section
open WeierstrassCurve.Affine SemaevS3 TheoremAudit

variable (K : Type*) [Field K]
def curve : WeierstrassCurve.Affine K := ⟨0, 0, 0, 3, 2⟩

variable {K} [DecidableEq K] [CharP K 5]

omit [DecidableEq K] in
theorem two_ne_zero : (2 : K) ≠ 0 := by
  change ¬ (2 : ℕ) = (0 : K)
  rw [CharP.cast_eq_zero_iff K 5]
  decide

theorem coprime_kill_zero {G : Type*} [AddCommGroup G] (P : G) (n : ℕ)
    (hn : Nat.gcd 5 n = 1) (h5 : (5 : ℕ) • P = 0) (hk : n • P = 0) : P = 0 := by
  have hd := Nat.dvd_gcd (addOrderOf_dvd_iff_nsmul_eq_zero.mpr h5)
    (addOrderOf_dvd_iff_nsmul_eq_zero.mpr hk)
  rw [hn] at hd
  exact AddMonoid.addOrderOf_eq_one_iff.mp (Nat.dvd_one.mp hd)

omit [DecidableEq K] [CharP K 5] in
/-- Credited actual equal-x classification from GeneralS4. -/
theorem same_pointX_eq_or_neg (W : WeierstrassCurve.Affine K)
    {P Q : W.Point} (h : pointX P = pointX Q) : P = Q ∨ P = -Q := by
  apply Point.eq_or_eq_neg_of_xRep_eq_xRep
  cases P <;> cases Q <;> simp_all [pointX, Point.xRep]

omit [DecidableEq K] [CharP K 5] in
theorem repeated_value (x u : K) :
    S3 (3 : K) 2 x x u = numerator x - denominator x * u := by
  unfold S3 numerator denominator
  ring

omit [DecidableEq K] [CharP K 5] in
theorem diagonal_value (x : K) : S3 (3 : K) 2 x x x = -triple x := by
  unfold S3 triple
  ring

theorem triple_nonzero_of_five {x y : K} (hN : (curve K).Nonsingular x y)
    (ht : (5 : ℕ) • Point.some x y hN = 0) : triple x ≠ 0 := by
  intro hz
  let P : (curve K).Point := .some x y hN
  have hP : P ≠ 0 := Point.some_ne_zero hN
  have hs : S3 (3 : K) 2 x x x = 0 := by rw [diagonal_value, hz, neg_zero]
  have he := (S3_repeated_mathlib_iff (curve K) rfl rfl rfl two_ne_zero hN x).mp hs
  rcases same_pointX_eq_or_neg (curve K) (P := P + P) (Q := P) he with he | he
  · exact hP (add_left_cancel (he.trans (add_zero P).symm))
  · have h3 : (3 : ℕ) • P = 0 := by
      rw [show (3 : ℕ) = 2 + 1 by decide, add_nsmul, two_nsmul, one_nsmul, he,
        neg_add_cancel]
    exact hP (coprime_kill_zero P 3 (by decide) ht h3)

theorem five_x_coordinates {x y : K} (hN : (curve K).Nonsingular x y)
    (ht : (5 : ℕ) • Point.some x y hN = 0) : x = 1 ∨ x = 2 := by
  let P : (curve K).Point := .some x y hN
  have hP : P ≠ 0 := Point.some_ne_zero hN
  have hQ : P + P ≠ 0 := by
    intro h
    exact hP (coprime_kill_zero P 2 (by decide) ht (by simpa only [two_nsmul] using h))
  have hQQ : (P + P) + (P + P) = -P := by
    apply eq_neg_iff_add_eq_zero.mpr
    simpa only [show (5 : ℕ) = 2 + 2 + 1 by decide, add_nsmul, two_nsmul,
      one_nsmul] using ht
  cases he : P + P with
  | zero => exact (hQ he).elim
  | some u v hU =>
    have hs1 : S3 (3 : K) 2 x x u = 0 := by
      apply (S3_repeated_mathlib_iff (curve K) rfl rfl rfl two_ne_zero hN u).mpr
      exact congrArg pointX he
    have hs2 : S3 (3 : K) 2 u u x = 0 := by
      apply (S3_repeated_mathlib_iff (curve K) rfl rfl rfl two_ne_zero hU x).mpr
      have h : Point.some u v hU + Point.some u v hU = -P := by
        rw [← he]
        exact hQQ
      rw [h, pointX_neg]
      rfl
    rw [repeated_value] at hs1 hs2
    have hz := eliminant_zero_of_double x u (sub_eq_zero.mp hs1) hs2
    rw [eliminant_factor, mul_assoc] at hz
    have hf := (mul_eq_zero.mp hz).resolve_left (neg_ne_zero.mpr (triple_nonzero_of_five hN ht))
    rcases mul_eq_zero.mp hf with hf | hf
    · exact Or.inl (sub_eq_zero.mp (eq_zero_of_pow_eq_zero hf))
    · exact Or.inr (sub_eq_zero.mp (eq_zero_of_pow_eq_zero hf))

omit [DecidableEq K] [CharP K 5] in
theorem actual_equation {x y : K} (hN : (curve K).Nonsingular x y) :
    y ^ 2 = x ^ 3 + 3 * x + 2 := by
  simpa [curve, WeierstrassCurve.Affine.equation_iff] using hN.1

theorem five_coordinates {x y : K} (hN : (curve K).Nonsingular x y)
    (ht : (5 : ℕ) • Point.some x y hN = 0) :
    (x = 1 ∨ x = 2) ∧ (y = 1 ∨ y = -1) := by
  have hx := five_x_coordinates hN ht
  have he := actual_equation hN
  have h5 : (5 : K) = 0 := CharP.cast_eq_zero K 5
  refine ⟨hx, sq_eq_one_iff.mp ?_⟩
  rcases hx with rfl | rfl
  · linear_combination he + h5
  · linear_combination he + 3 * h5

#print axioms triple_nonzero_of_five
#print axioms five_x_coordinates
#print axioms five_coordinates
end
end BKCharFiveGeometricAudit
