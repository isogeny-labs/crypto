import Mathlib.Algebra.Polynomial.Degree.Domain
import Mathlib.Tactic.Ring

/-!
Exact polynomial expansion and coefficient readout for a generic cubic
numerator and quadratic supports. The link from these generic coefficients
to the source common-branch scalar formulas remains a separate obligation.
-/

open Polynomial
variable {K : Type*} [Field K]

namespace Common22PolynomialHead

theorem generic_expansion
    (q1 r1 q2 r2 n3 n2 n1 n0 ucoef qcoef rcoef : K) :
    let u1 : K[X] := X ^ 2 + C q1 * X + C r1
    let u2 : K[X] := X ^ 2 + C q2 * X + C r2
    let n : K[X] := C n3 * X ^ 3 + C n2 * X ^ 2 + C n1 * X + C n0
    let support : K[X] := C ucoef * X ^ 2 + C qcoef * X + C rcoef
    n ^ 2 - u1 * u2 * support =
      C (n3 ^ 2 - ucoef) * X ^ 6 +
      C (2 * n3 * n2 - (qcoef + ucoef * (q1 + q2))) * X ^ 5 +
      C (n2 ^ 2 + 2 * n3 * n1 -
        (rcoef + qcoef * (q1 + q2) + ucoef * (r1 + r2 + q1 * q2))) * X ^ 4 +
      C (2 * n3 * n0 + 2 * n2 * n1 -
        (rcoef * (q1 + q2) + qcoef * (r1 + r2 + q1 * q2) +
         ucoef * (q1 * r2 + q2 * r1))) * X ^ 3 +
      C (n1 ^ 2 + 2 * n2 * n0 -
        (rcoef * (r1 + r2 + q1 * q2) + qcoef * (q1 * r2 + q2 * r1) +
         ucoef * r1 * r2)) * X ^ 2 +
      C (2 * n1 * n0 - (rcoef * (q1 * r2 + q2 * r1) + qcoef * r1 * r2)) * X +
      C (n0 ^ 2 - rcoef * r1 * r2) := by
  dsimp
  simp only [map_add, map_sub, map_mul, map_pow, map_ofNat]
  ring


theorem generic_coeff_six
    (q1 r1 q2 r2 n3 n2 n1 n0 ucoef qcoef rcoef : K) :
    let u1 : K[X] := X ^ 2 + C q1 * X + C r1
    let u2 : K[X] := X ^ 2 + C q2 * X + C r2
    let n : K[X] := C n3 * X ^ 3 + C n2 * X ^ 2 + C n1 * X + C n0
    let support : K[X] := C ucoef * X ^ 2 + C qcoef * X + C rcoef
    (n ^ 2 - u1 * u2 * support).coeff 6 = n3 ^ 2 - ucoef := by
  dsimp
  rw [generic_expansion q1 r1 q2 r2 n3 n2 n1 n0 ucoef qcoef rcoef]
  simp only [coeff_add, coeff_C_mul_X_pow, coeff_C_mul_X, coeff_C]
  simp


theorem generic_coeff_five
    (q1 r1 q2 r2 n3 n2 n1 n0 ucoef qcoef rcoef : K) :
    let u1 : K[X] := X ^ 2 + C q1 * X + C r1
    let u2 : K[X] := X ^ 2 + C q2 * X + C r2
    let n : K[X] := C n3 * X ^ 3 + C n2 * X ^ 2 + C n1 * X + C n0
    let support : K[X] := C ucoef * X ^ 2 + C qcoef * X + C rcoef
    (n ^ 2 - u1 * u2 * support).coeff 5 =
      2 * n3 * n2 - (qcoef + ucoef * (q1 + q2)) := by
  dsimp
  rw [generic_expansion q1 r1 q2 r2 n3 n2 n1 n0 ucoef qcoef rcoef]
  simp only [coeff_add, coeff_C_mul_X_pow, coeff_C_mul_X, coeff_C]
  simp

theorem generic_coeff_four
    (q1 r1 q2 r2 n3 n2 n1 n0 ucoef qcoef rcoef : K) :
    let u1 : K[X] := X ^ 2 + C q1 * X + C r1
    let u2 : K[X] := X ^ 2 + C q2 * X + C r2
    let n : K[X] := C n3 * X ^ 3 + C n2 * X ^ 2 + C n1 * X + C n0
    let support : K[X] := C ucoef * X ^ 2 + C qcoef * X + C rcoef
    (n ^ 2 - u1 * u2 * support).coeff 4 =
      n2 ^ 2 + 2 * n3 * n1 -
        (rcoef + qcoef * (q1 + q2) + ucoef * (r1 + r2 + q1 * q2)) := by
  dsimp
  rw [generic_expansion q1 r1 q2 r2 n3 n2 n1 n0 ucoef qcoef rcoef]
  simp only [coeff_add, coeff_C_mul_X_pow, coeff_C_mul_X, coeff_C]
  simp

end Common22PolynomialHead

#print axioms Common22PolynomialHead.generic_expansion
#print axioms Common22PolynomialHead.generic_coeff_six
#print axioms Common22PolynomialHead.generic_coeff_five
#print axioms Common22PolynomialHead.generic_coeff_four
