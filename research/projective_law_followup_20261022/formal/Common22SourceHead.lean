import Common22PolynomialHead
import Common22SupportHead
import Common22Coprime
import Common22Degree

/-!
Actual polynomial head for the proof-side unit-weight common 2+2 residual.
The source-derived scalar coefficient formulas are substituted into the
generic polynomial expansion. This does not parse Rust or prove the curve
and divisor-class bridge.
-/

open Polynomial
variable {K : Type*} [Field K]

namespace Common22SourceHead

theorem canonical_n_eq_interpolant (q1 r1 s1 t1 b a c : K) :
    (C (-c) * X ^ 3 + C (-(c * q1 + a)) * X ^ 2 +
      C (b * s1 - c * r1 - a * q1) * X + C (b * t1 - a * r1) : K[X]) =
    C b * (C s1 * X + C t1) -
      (C c * X + C a) * (X ^ 2 + C q1 * X + C r1) := by
  simp only [map_add, map_sub, map_mul, map_neg]
  ring

/-- The source-style cubic also agrees with the second input ordinate
modulo the second quadratic support. -/
theorem canonical_n_interpolates_second
    (q1 r1 s1 t1 q2 r2 s2 t2 : K) :
    let (b, a, c, _, _) :=
      Common22SupportHead.coefficientData q1 r1 s1 t1 q2 r2 s2 t2
    let n : K[X] := C (-c) * X ^ 3 + C (-(c * q1 + a)) * X ^ 2 +
      C (b * s1 - c * r1 - a * q1) * X + C (b * t1 - a * r1)
    n - C b * (C s2 * X + C t2) =
      (C (-c) * X + C (-c * (q1 - q2) - a)) *
        (X ^ 2 + C q2 * X + C r2) := by
  dsimp [Common22SupportHead.coefficientData]
  simp only [map_add, map_sub, map_mul, map_pow, map_neg]
  ring

noncomputable def sourceE (q1 r1 s1 t1 q2 r2 s2 t2 : K) : K[X] :=
  let (b, a, c, q, r) := Common22SupportHead.coefficientData q1 r1 s1 t1 q2 r2 s2 t2
  let u1 : K[X] := X ^ 2 + C q1 * X + C r1
  let u2 : K[X] := X ^ 2 + C q2 * X + C r2
  let n : K[X] := C (-c) * X ^ 3 + C (-(c * q1 + a)) * X ^ 2 +
    C (b * s1 - c * r1 - a * q1) * X + C (b * t1 - a * r1)
  let support : K[X] := C (c ^ 2) * X ^ 2 + C q * X + C r
  n ^ 2 - u1 * u2 * support

theorem sourceE_coeff_six (q1 r1 s1 t1 q2 r2 s2 t2 : K) :
    (sourceE q1 r1 s1 t1 q2 r2 s2 t2).coeff 6 = 0 := by
  unfold sourceE
  simp only [Common22SupportHead.coefficientData]
  rw [Common22PolynomialHead.generic_coeff_six]
  ring


theorem sourceE_coeff_five (q1 r1 s1 t1 q2 r2 s2 t2 : K) :
    (sourceE q1 r1 s1 t1 q2 r2 s2 t2).coeff 5 =
      (Common22SupportHead.coefficientData q1 r1 s1 t1 q2 r2 s2 t2).1 ^ 2 := by
  unfold sourceE
  simp only [Common22SupportHead.coefficientData]
  rw [Common22PolynomialHead.generic_coeff_five]
  ring_nf

theorem sourceE_coeff_four (q1 r1 s1 t1 q2 r2 s2 t2 : K) :
    (sourceE q1 r1 s1 t1 q2 r2 s2 t2).coeff 4 = 0 := by
  unfold sourceE
  simp only [Common22SupportHead.coefficientData]
  rw [Common22PolynomialHead.generic_coeff_four]
  ring_nf

/-- The proof-side source-style residual is at most sextic. -/
theorem sourceE_natDegree_le_six (q1 r1 s1 t1 q2 r2 s2 t2 : K) :
    (sourceE q1 r1 s1 t1 q2 r2 s2 t2).natDegree ≤ 6 := by
  unfold sourceE
  simp only [Common22SupportHead.coefficientData]
  apply Common22PolynomialHead.generic_natDegree_le_six

/-- For a monic quintic with zero fourth coefficient, the source-style
residual difference has degree below the two-support product. -/
theorem sourceE_strict_degree
    (q1 r1 s1 t1 q2 r2 s2 t2 : K) (f : K[X])
    (f_degree : f.natDegree ≤ 5)
    (f_five : f.coeff 5 = 1)
    (f_four : f.coeff 4 = 0) :
    degree (sourceE q1 r1 s1 t1 q2 r2 s2 t2 -
      C ((Common22SupportHead.coefficientData q1 r1 s1 t1 q2 r2 s2 t2).1 ^ 2) * f) <
    degree (Common22Coprime.firstSupport q1 r1 *
      Common22Coprime.secondSupport q2 r2) := by
  exact Common22Degree.cleared_difference_degree_lt_supports_of_sextic_head
    (Common22Coprime.firstSupport q1 r1)
    (Common22Coprime.secondSupport q2 r2)
    (sourceE q1 r1 s1 t1 q2 r2 s2 t2) f
    (Common22SupportHead.coefficientData q1 r1 s1 t1 q2 r2 s2 t2).1
    (Common22Coprime.firstSupport_nonzero q1 r1)
    (Common22Coprime.secondSupport_nonzero q2 r2)
    (Common22Coprime.firstSupport_natDegree q1 r1)
    (Common22Coprime.secondSupport_natDegree q2 r2)
    (sourceE_natDegree_le_six q1 r1 s1 t1 q2 r2 s2 t2)
    (sourceE_coeff_six q1 r1 s1 t1 q2 r2 s2 t2)
    f_degree
    (sourceE_coeff_five q1 r1 s1 t1 q2 r2 s2 t2)
    f_five
    (sourceE_coeff_four q1 r1 s1 t1 q2 r2 s2 t2)
    f_four

end Common22SourceHead

#print axioms Common22SourceHead.sourceE_coeff_six
#print axioms Common22SourceHead.sourceE_coeff_five
#print axioms Common22SourceHead.sourceE_coeff_four
#print axioms Common22SourceHead.canonical_n_eq_interpolant
#print axioms Common22SourceHead.canonical_n_interpolates_second
#print axioms Common22SourceHead.sourceE_natDegree_le_six
#print axioms Common22SourceHead.sourceE_strict_degree
