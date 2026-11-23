import RuntimeSemantics
import Mathlib.Algebra.Polynomial.Degree.SmallDegree
import Mathlib.Algebra.Polynomial.Degree.Domain
import Mathlib.Tactic.FieldSimp

set_option autoImplicit false
namespace SameSupportCandidate
open Polynomial
variable {K : Type*} [Field K]

theorem validity_coefficients
    (f : Polynomial K) (q r s1 t1 s2 t2 : K)
    (valid1 : (X ^ 2 + C q * X + C r) ∣ f - (C s1 * X + C t1) ^ 2)
    (valid2 : (X ^ 2 + C q * X + C r) ∣ f - (C s2 * X + C t2) ^ 2)
    (hs : s1 = s2) :
    (t2 - t1) * (s1 + s2) = 0 ∧ (t2 - t1) * (t2 + t1) = 0 := by
  subst s2
  have hd := dvd_sub valid1 valid2
  have he : (f - (C s1 * X + C t1) ^ 2) -
      (f - (C s1 * X + C t2) ^ 2) =
      C ((t2-t1)*(s1+s1)) * X + C ((t2-t1)*(t2+t1)) := by
    simp only [map_mul, map_sub, map_add]
    ring
  rw [he] at hd
  have hu : (X ^ 2 + C q * X + C r).natDegree = 2 := by
    simpa using (natDegree_quadratic (R := K) (a := (1:K)) (b := q) (c := r) one_ne_zero)
  have hl : (C ((t2-t1)*(s1+s1)) * X + C ((t2-t1)*(t2+t1))).natDegree ≤ 1 :=
    natDegree_linear_le
  have hz := eq_zero_of_dvd_of_natDegree_lt hd (by omega :
    (C ((t2-t1)*(s1+s1)) * X + C ((t2-t1)*(t2+t1))).natDegree <
      (X ^ 2 + C q * X + C r).natDegree)
  have coeff1_linear (a b : K) : (C a * X + C b : K[X]).coeff 1 = a := by
    simp
  have coeff0_linear (a b : K) : (C a * X + C b : K[X]).coeff 0 = b := by
    simp
  constructor
  · have hh := congrArg (fun p : K[X] => p.coeff 1) hz
    rw [coeff1_linear, coeff_zero] at hh
    exact hh
  · have hh := congrArg (fun p : K[X] => p.coeff 0) hz
    rw [coeff0_linear, coeff_zero] at hh
    exact hh

#print axioms validity_coefficients
end SameSupportCandidate
