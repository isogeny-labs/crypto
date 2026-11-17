import Mathlib.Algebra.Field.Basic
import Mathlib.Tactic.Ring

/-!
Cleared ordinate congruence for the common 2+2 residual divisor. This is a
polynomial identity over any commutative ring. Source bindings and the
principal-divisor argument are separate obligations.
-/

set_option autoImplicit false

namespace Common22Residual

variable {R : Type*} [CommRing R]

/-- The cubic numerator interpolating the first input support. -/
def numerator (X q1 r1 s1 t1 B A C : R) : R :=
  B * (s1 * X + t1) - (C * X + A) * (X ^ 2 + q1 * X + r1)

/-- The residual quadratic in the common branch's cleared coordinates. -/
def support (X Q Rbar C : R) : R :=
  C ^ 2 * X ^ 2 + Q * X + Rbar

/-- The literal source-shaped returned ordinate numerators. -/
def outputS (q1 r1 s1 B A C Q Rbar : R) : R :=
  let r := C ^ 2 * Rbar
  let qdelta := q1 * C ^ 2 - Q
  let rdelta := r1 * C ^ 4 - r
  rdelta + (A * C - Q) * qdelta - s1 * B * C ^ 3

def outputT (q1 r1 t1 B A C Q Rbar : R) : R :=
  let r := C ^ 2 * Rbar
  let qdelta := q1 * C ^ 2 - Q
  let rdelta := r1 * C ^ 4 - r
  rdelta * A * C - r * qdelta - t1 * B * C ^ 5

/-- Cleared congruence between the source-shaped ordinate numerators and the
cubic numerator modulo the residual support. The quotient is explicit; no
division, input-validity, or coordinate-normalization claim is hidden here. -/
theorem returned_v_cleared
    (X q1 r1 s1 t1 B A C Q Rbar : R) :
    C ^ 5 * numerator X q1 r1 s1 t1 B A C +
      outputS q1 r1 s1 B A C Q Rbar * C ^ 2 * X +
      outputT q1 r1 t1 B A C Q Rbar =
    support X Q Rbar C *
      (-C ^ 4 * X + C ^ 3 * (-C * q1 - A) + C ^ 2 * Q) := by
  simp only [numerator, support, outputS, outputT]
  ring

end Common22Residual

#print axioms Common22Residual.returned_v_cleared
