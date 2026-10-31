import Mathlib.Tactic.Ring

/-!
# Local polynomial dependencies of the prime-field bit-restriction proof

These are symbolic identities and their explicitly conditional consequences.
They do not formalize Boolean assignment localization, filtered polynomial
closure, lookup-map point lifts, auxiliary-bit surjectivity, or the complete
last-fall bound. The older FallDegreeIdentities module is unchanged.
-/

namespace PrimeBitLocal

section RingIdentities

variable {R : Type*} [CommRing R]

/-- A monic quadratic with its coefficient names kept explicit. -/
def monicQuadratic (A B u : R) : R := u^2 + A*u + B

/-- The short-Weierstrass third summation polynomial. -/
def oddS3 (a b x y u : R) : R :=
  (x-y)^2*u^2 - 2*((x+y)*(x*y+a)+2*b)*u +
    (x*y-a)^2 - 4*b*(x+y)

/-- The only variable multiplication in the local quadratic/linear
refutation. No filtered-degree or ideal-membership claim is made here. -/
theorem quadratic_linear_cancellation (A B alpha u : R) :
    monicQuadratic A B u - u*(u-alpha) - (A+alpha)*(u-alpha) =
      alpha^2 + A*alpha + B := by
  unfold monicQuadratic
  ring

/-- At repeated abscissas the summation polynomial becomes linear in u. -/
theorem odd_s3_diagonal (a b t u : R) :
    oddS3 a b t t u =
      -4*(t^3+a*t+b)*u + (t^2-a)^2 - 8*b*t := by
  unfold oddS3
  ring

/-- The repeated-abscissa constant term differs from the squared derivative
by the explicit multiple of the cubic curve polynomial. -/
theorem diagonal_constant_relation (a b t : R) :
    (t^2-a)^2 - 8*b*t =
      (3*t^2+a)^2 - 8*t*(t^3+a*t+b) := by
  ring

/-- Specialization at a root of the cubic. This does not establish that
the derivative is nonzero; that hypothesis is kept separate below. -/
theorem odd_s3_at_cubic_root (a b t u : R)
    (hroot : t^3+a*t+b = 0) :
    oddS3 a b t t u = (3*t^2+a)^2 := by
  calc
    oddS3 a b t t u = (t^2-a)^2 - 8*b*t := by
      rw [odd_s3_diagonal, hroot]
      ring
    _ = (3*t^2+a)^2 := by
      rw [diagonal_constant_relation, hroot]
      ring

/-- A nonzero short-Weierstrass discriminant expression prevents the cubic
and its derivative from vanishing at the same point. This proves the needed
implication directly, without assuming the derivative conclusion. -/
theorem derivative_ne_zero_at_cubic_root (a b t : R)
    (hroot : t^3+a*t+b = 0) (hdisc : 4*a^3+27*b^2 ≠ 0) :
    3*t^2+a ≠ 0 := by
  intro hderivative
  have ha : a = -(3*t^2) := by
    calc
      a = (3*t^2+a) - 3*t^2 := by ring
      _ = -(3*t^2) := by rw [hderivative]; ring
  have hb : b = 2*t^3 := by
    calc
      b = (t^3+a*t+b) - (t^3+a*t) := by ring
      _ = 2*t^3 := by rw [hroot, ha]; ring
  apply hdisc
  rw [ha, hb]
  ring

end RingIdentities

section FieldConsequences

variable {K : Type*} [Field K]

/-- A root of the curve cubic with explicitly nonzero derivative produces
a nonzero constant summation polynomial on the diagonal. -/
theorem odd_s3_diagonal_ne_zero (a b t u : K)
    (hroot : t^3+a*t+b = 0) (hderivative : 3*t^2+a ≠ 0) :
    oddS3 a b t t u ≠ 0 := by
  rw [odd_s3_at_cubic_root a b t u hroot]
  exact pow_ne_zero 2 hderivative

/-- The same-abscissa/two-torsion branch is a nonzero constant when the
explicit discriminant expression is nonzero. -/
theorem odd_s3_diagonal_ne_zero_of_discriminant (a b t u : K)
    (hroot : t^3+a*t+b = 0) (hdisc : 4*a^3+27*b^2 ≠ 0) :
    oddS3 a b t t u ≠ 0 := by
  exact odd_s3_diagonal_ne_zero a b t u hroot
    (derivative_ne_zero_at_cubic_root a b t hroot hdisc)

/-- The local cancellation becomes one after multiplication by the inverse
of its explicitly nonzero evaluation. This is an identity, not a proof that
the two terms are available in a particular bounded-degree closure. -/
theorem normalized_quadratic_linear_refutation (A B alpha u : K)
    (hne : alpha^2 + A*alpha + B ≠ 0) :
    (alpha^2 + A*alpha + B)⁻¹ *
      (monicQuadratic A B u - u*(u-alpha) - (A+alpha)*(u-alpha)) = 1 := by
  rw [quadratic_linear_cancellation]
  exact inv_mul_cancel₀ hne

end FieldConsequences

end PrimeBitLocal

#print axioms PrimeBitLocal.quadratic_linear_cancellation
#print axioms PrimeBitLocal.odd_s3_diagonal
#print axioms PrimeBitLocal.diagonal_constant_relation
#print axioms PrimeBitLocal.odd_s3_at_cubic_root
#print axioms PrimeBitLocal.derivative_ne_zero_at_cubic_root
#print axioms PrimeBitLocal.odd_s3_diagonal_ne_zero
#print axioms PrimeBitLocal.odd_s3_diagonal_ne_zero_of_discriminant
#print axioms PrimeBitLocal.normalized_quadratic_linear_refutation
