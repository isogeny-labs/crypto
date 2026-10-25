import Mathlib.Algebra.Polynomial.Degree.Domain

/-!
The high-coefficient degree step for the common 2+2 residual equation.
The source-specific coefficient equations and support-product degree are
separate obligations.
-/

set_option autoImplicit false

namespace Common22Degree

open Polynomial

variable {K : Type*} [Field K]

/-- Two degree-at-most-five polynomials with matching scaled coefficients
in degrees five and four differ by a polynomial of degree below four. -/
theorem cleared_difference_degree_lt_four
    (e f : K[X]) (b : K)
    (e_degree : e.natDegree ≤ 5)
    (f_degree : f.natDegree ≤ 5)
    (e_five : e.coeff 5 = b ^ 2)
    (f_five : f.coeff 5 = 1)
    (e_four : e.coeff 4 = 0)
    (f_four : f.coeff 4 = 0) :
    degree (e - C (b ^ 2) * f) < 4 := by
  apply (degree_lt_iff_coeff_zero (e - C (b ^ 2) * f) 4).2
  intro m hm
  by_cases hm4 : m = 4
  · subst m
    simp only [coeff_sub, coeff_C_mul, e_four, f_four, mul_zero, sub_zero]
  by_cases hm5 : m = 5
  · subst m
    simp only [coeff_sub, coeff_C_mul, e_five, f_five, mul_one, sub_self]
  have hm6 : 5 < m := by omega
  have ez : e.coeff m = 0 := coeff_eq_zero_of_natDegree_lt (lt_of_le_of_lt e_degree hm6)
  have fz : f.coeff m = 0 := coeff_eq_zero_of_natDegree_lt (lt_of_le_of_lt f_degree hm6)
  simp only [coeff_sub, coeff_C_mul, ez, fz, mul_zero, sub_zero]

/-- Convert the strict degree-four bound to the product of two quadratic
supports. Their quadratic degrees and nonzeroness are explicit premises. -/
theorem cleared_difference_degree_lt_supports
    (u1 u2 e f : K[X]) (b : K)
    (u1_nonzero : u1 ≠ 0) (u2_nonzero : u2 ≠ 0)
    (u1_degree : u1.natDegree = 2) (u2_degree : u2.natDegree = 2)
    (e_degree : e.natDegree ≤ 5) (f_degree : f.natDegree ≤ 5)
    (e_five : e.coeff 5 = b ^ 2) (f_five : f.coeff 5 = 1)
    (e_four : e.coeff 4 = 0) (f_four : f.coeff 4 = 0) :
    degree (e - C (b ^ 2) * f) < degree (u1 * u2) := by
  have hproduct : degree (u1 * u2) = 4 := by
    rw [degree_eq_natDegree (mul_ne_zero u1_nonzero u2_nonzero),
      natDegree_mul u1_nonzero u2_nonzero, u1_degree, u2_degree]
    norm_num
  rw [hproduct]
  exact cleared_difference_degree_lt_four e f b e_degree f_degree
    e_five f_five e_four f_four

/-- A sextic upper bound drops to a quintic upper bound when its sixth
coefficient vanishes. This handles the zero polynomial explicitly. -/
theorem natDegree_le_five_of_sextic_head_zero
    (e : K[X]) (e_degree : e.natDegree ≤ 6) (e_six : e.coeff 6 = 0) :
    e.natDegree ≤ 5 := by
  by_cases he : e = 0
  · simp [he]
  have hlt : degree e < 6 :=
    (degree_lt_iff_coeff_zero e 6).2 (by
      intro m hm
      by_cases hm6 : m = 6
      · simpa [hm6] using e_six
      have hm7 : 6 < m := by omega
      exact coeff_eq_zero_of_natDegree_lt (lt_of_le_of_lt e_degree hm7))
  have hnat : e.natDegree < 6 := (natDegree_lt_iff_degree_lt he).2 hlt
  omega

/-- The sextic head certificate supplies the same strict support-product
degree bound, provided the two supports are nonzero quadratics. -/
theorem cleared_difference_degree_lt_supports_of_sextic_head
    (u1 u2 e f : K[X]) (b : K)
    (u1_nonzero : u1 ≠ 0) (u2_nonzero : u2 ≠ 0)
    (u1_degree : u1.natDegree = 2) (u2_degree : u2.natDegree = 2)
    (e_degree : e.natDegree ≤ 6) (e_six : e.coeff 6 = 0)
    (f_degree : f.natDegree ≤ 5)
    (e_five : e.coeff 5 = b ^ 2) (f_five : f.coeff 5 = 1)
    (e_four : e.coeff 4 = 0) (f_four : f.coeff 4 = 0) :
    degree (e - C (b ^ 2) * f) < degree (u1 * u2) := by
  exact cleared_difference_degree_lt_supports u1 u2 e f b
    u1_nonzero u2_nonzero u1_degree u2_degree
    (natDegree_le_five_of_sextic_head_zero e e_degree e_six)
    f_degree e_five f_five e_four f_four

end Common22Degree

#print axioms Common22Degree.cleared_difference_degree_lt_four
#print axioms Common22Degree.cleared_difference_degree_lt_supports
#print axioms Common22Degree.natDegree_le_five_of_sextic_head_zero
#print axioms Common22Degree.cleared_difference_degree_lt_supports_of_sextic_head
