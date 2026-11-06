import Mathlib.Algebra.Field.Basic
import Mathlib.Tactic.Ring

/-!
High-degree coefficients of the common 2+2 residual-support identity.
The x^6 and x^4 coefficients vanish, and x^5 is B². This is an exact
polynomial statement over a commutative ring, not yet the complete curve
equation or divisor-class theorem.
-/

set_option autoImplicit false

namespace Common22SupportHead

variable {R : Type*} [CommRing R]

/-- The source common-branch coefficients, specialized to unit input weights. -/
def coefficientData (q1 r1 s1 t1 q2 r2 s2 t2 : R) : R × R × R × R × R :=
  let dq := q1 - q2
  let dr := r1 - r2
  let ds := s1 - s2
  let dt := t1 - t2
  let B := dq * (q2 * r1 - q1 * r2) - dr ^ 2
  let A := dq * (dt * q2 - ds * r2) - dr * dt
  let C := dq * dt - dr * ds
  let Q := dq * C ^ 2 + 2 * A * C - B ^ 2
  let Rbar := A * (dq * C + A) - (s1 + s2) * B * C + (q1 + q2) * B ^ 2
  (B, A, C, Q, Rbar)

/-- The x^6, x^5, and x^4 coefficients of `N²-u1*u2*U`, with
`N=B*v1-(C*x+A)*u1` and `U=C²*x²+Q*x+Rbar`. -/
def headCoefficients (q1 r1 s1 t1 q2 r2 s2 t2 : R) : R × R × R :=
  let (B, A, C, Q, Rbar) := coefficientData q1 r1 s1 t1 q2 r2 s2 t2
  let n3 := -C
  let n2 := -(C * q1 + A)
  let n1 := B * s1 - C * r1 - A * q1
  let e6 := n3 ^ 2 - C ^ 2
  let e5 := 2 * n3 * n2 - (Q + C ^ 2 * (q1 + q2))
  let e4 := n2 ^ 2 + 2 * n3 * n1 -
    (Rbar + Q * (q1 + q2) + C ^ 2 * (r1 + r2 + q1 * q2))
  (e6, e5, e4)

/-- Exact high-degree residual-support certificate. -/
theorem headCoefficients_eq
    (q1 r1 s1 t1 q2 r2 s2 t2 : R) :
    headCoefficients q1 r1 s1 t1 q2 r2 s2 t2 =
      (0, (coefficientData q1 r1 s1 t1 q2 r2 s2 t2).1 ^ 2, 0) := by
  simp only [headCoefficients, coefficientData]
  ring_nf

end Common22SupportHead

#print axioms Common22SupportHead.headCoefficients_eq
