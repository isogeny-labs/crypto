import SameSupportClassifier

set_option autoImplicit false
namespace SameSupportCandidate
open Polynomial
variable {K : Type*} [Field K]

lemma cross_scale (x z1 z2 w1 w2 : K) (n : ℕ)
    (hz1 : z1 ≠ 0) (hw1 : w1 ≠ 0) :
    x * z2^n * w2 = ((z1*z2)^n * (w1*w2)) * (x / (z1^n*w1)) := by
  rw [mul_pow]
  field_simp
  <;> ring

theorem weighted_valid_inputs_exclude_same_support_error [DecidableEq K]
    (f : Polynomial K) (q1 r1 s1 t1 z1 w1 q2 r2 s2 t2 z2 w2 : K)
    (hz1 : z1 ≠ 0) (hz2 : z2 ≠ 0) (hw1 : w1 ≠ 0) (hw2 : w2 ≠ 0)
    (hodd : (2 : K) ≠ 0)
    (valid1 : (X^2 + C (q1/z1^2)*X + C (r1/z1^4)) ∣
      f - (C (s1/(z1^3*w1))*X + C (t1/(z1^5*w1)))^2)
    (valid2 : (X^2 + C (q2/z2^2)*X + C (r2/z2^4)) ∣
      f - (C (s2/(z2^3*w2))*X + C (t2/(z2^5*w2)))^2)
    (hQ : q1*z2^2 = q2*z1^2) (hR : r1*z2^4 = r2*z1^4) :
    ProjectiveLawRuntime.classifySameU22
      (s1*z2^3*w2) (t1*z2^5*w2) (s2*z1^3*w1) (t2*z1^5*w1) ≠
      ProjectiveLawRuntime.Branch22Prelude.sameUMixedZeroSlope := by
  by_cases hs : s1*z2^3*w2 = s2*z1^3*w1
  · have hq : q1/z1^2 = q2/z2^2 :=
      (div_eq_div_iff (pow_ne_zero 2 hz1) (pow_ne_zero 2 hz2)).mpr hQ
    have hr : r1/z1^4 = r2/z2^4 :=
      (div_eq_div_iff (pow_ne_zero 4 hz1) (pow_ne_zero 4 hz2)).mpr hR
    have hsn : s1/(z1^3*w1) = s2/(z2^3*w2) := by
      apply (div_eq_div_iff (mul_ne_zero (pow_ne_zero 3 hz1) hw1)
        (mul_ne_zero (pow_ne_zero 3 hz2) hw2)).mpr
      simpa only [mul_assoc] using hs
    rw [← hq, ← hr] at valid2
    obtain ⟨hx, hc⟩ := validity_coefficients f (q1/z1^2) (r1/z1^4)
      (s1/(z1^3*w1)) (t1/(z1^5*w1)) (s2/(z2^3*w2)) (t2/(z2^5*w2))
      valid1 valid2 hsn
    have es1 := cross_scale s1 z1 z2 w1 w2 3 hz1 hw1
    have et1 := cross_scale t1 z1 z2 w1 w2 5 hz1 hw1
    have es2 : s2*z1^3*w1 = ((z1*z2)^3*(w1*w2))*(s2/(z2^3*w2)) := by
      simpa only [mul_comm z2 z1, mul_comm w2 w1] using cross_scale s2 z2 z1 w2 w1 3 hz2 hw2
    have et2 : t2*z1^5*w1 = ((z1*z2)^5*(w1*w2))*(t2/(z2^5*w2)) := by
      simpa only [mul_comm z2 z1, mul_comm w2 w1] using cross_scale t2 z2 z1 w2 w1 5 hz2 hw2
    apply ProjectiveLawRuntime.classifySameU22_ne_zeroSlope _ _ _ _ hodd
    · rw [es1, es2, et1, et2]
      calc
        _ = (((z1*z2)^5*(w1*w2))*((z1*z2)^3*(w1*w2))) *
          ((t2/(z2^5*w2)-t1/(z1^5*w1))*(s1/(z1^3*w1)+s2/(z2^3*w2))) := by ring
        _ = 0 := by rw [hx, mul_zero]
    · rw [et1, et2]
      calc
        _ = (((z1*z2)^5*(w1*w2))^2) *
          ((t2/(z2^5*w2)-t1/(z1^5*w1))*(t2/(z2^5*w2)+t1/(z1^5*w1))) := by ring
        _ = 0 := by rw [hc, mul_zero]
  · unfold ProjectiveLawRuntime.classifySameU22
    have hz : s1*z2^3*w2 - s2*z1^3*w1 ≠ 0 := sub_ne_zero.mpr hs
    simp only [hs, false_and, hz, ite_false]
    split_ifs <;> intro h <;> cases h
end SameSupportCandidate
