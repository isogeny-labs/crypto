import Mathlib.Tactic.Ring
import Mathlib.Algebra.CharP.Basic

/-!
# Polynomial identities used by the guarded three-abscissa fall-degree proof

These declarations formalize polynomial cancellation identities only. They do
not formalize bounded-degree closure, scalar-grid normal forms, invertibility,
Semaev point interpretation, or the resulting universal last-fall theorem.
-/

namespace FallDegreeIdentities

section General

variable {R : Type*} [CommRing R]

/-- The scalar expression for the resultant of two monic quadratics. -/
def quadraticResultant (A B C D : R) : R :=
  (B - D)^2 - A * (A - C) * (B - D) + B * (A - C)^2

/-- An ordinary Bezout identity, used here only to formalize the
common-root implication. It makes no bounded-degree claim. -/
theorem quadratic_resultant_bezout (A B C D u : R) :
    quadraticResultant A B C D =
      (A-C)^2 * (u^2+A*u+B) -
        ((A-C)*u+(B-D)) * ((A-C)*u+A*(A-C)-(B-D)) := by
  unfold quadraticResultant
  ring

/-- Two actual roots with the same parameter make the scalar resultant
zero. This is a structural implication, not just a closed numerical check. -/
theorem common_root_implies_resultant_zero (A B C D u : R)
    (h1 : u^2+A*u+B = 0) (h2 : u^2+C*u+D = 0) :
    quadraticResultant A B C D = 0 := by
  have hL : (A-C)*u+(B-D) = 0 := by
    calc
      _ = (u^2+A*u+B) - (u^2+C*u+D) := by ring
      _ = 0 := by rw [h1, h2]; ring
  rw [quadratic_resultant_bezout A B C D u, h1, hL]
  ring

/-- The degree-five construction's first cancellation: its result is linear
in `u`, with a separable coefficient `2(B-D)-A²+C²`. -/
theorem odd_linear_remainder (A B C D u : R) :
    2 * u * ((A - C) * u + (B - D)) -
        (A - C) * ((u^2 + A*u + B) + (u^2 + C*u + D)) =
      (2 * (B - D) - A^2 + C^2) * u - (A - C) * (B + D) := by
  ring

/-- The scalar cancellation at the center of the odd-characteristic proof.
No inverse of two is assumed by this identity; the application needs one. -/
theorem odd_resultant_cancellation (A B C D u : R) :
    (A - C) * ((2 * (B - D) - A^2 + C^2) * u - (A - C) * (B + D)) -
        (2 * (B - D) - A^2 + C^2) * ((A - C) * u + (B - D)) =
      -2 * quadraticResultant A B C D := by
  unfold quadraticResultant
  ring

/-- The full odd construction, before applying scalar normal forms. -/
theorem odd_full_construction (A B C D u : R) :
    (A - C) *
        (2 * u * ((u^2 + A*u + B) - (u^2 + C*u + D)) -
          (A - C) * ((u^2 + A*u + B) + (u^2 + C*u + D))) -
      (2 * (B - D) - A^2 + C^2) *
        ((u^2 + A*u + B) - (u^2 + C*u + D)) =
      -2 * quadraticResultant A B C D := by
  unfold quadraticResultant
  ring

/-- The divided-difference guard identity, abstracting only the already
known relation `gx - gy = d*q`. -/
theorem inverse_guard_divided_difference (v d q gx gy : R)
    (h : gx - gy = d*q) :
    v * (gx - gy) - q * (v*d - 1) = q := by
  rw [h]
  ring

/-- Missing-root transport for a monic cubic, as an exact polynomial identity. -/
theorem cubic_missing_root (s1 s2 s3 x y : R) :
    let g := fun t : R => t^3 - s1*t^2 + s2*t - s3
    let q := x^2 + x*y + y^2 - s1*(x+y) + s2
    g (s1-x-y) = g y + (s1-x-2*y)*q := by
  dsimp
  ring

/-- The characteristic-two construction's integer lift. This identity is
unconditional over every commutative ring; characteristic two removes the
explicit multiple of two in its application. -/
theorem char_two_resultant_integer_lift (A B C D u : R) :
    let E := A+C
    let J := B+D
    E^2 * (u^2+A*u+B) +
      u * (E^2*u+E*J) + J*(E*u+J) + A*(E^2*u+E*J) =
      J^2+A*E*J+B*E^2 +
        2*(E^2*u^2+A*E^2*u+E*J*u) := by
  dsimp
  ring

end General

section CharacteristicTwo

variable {R : Type*} [CommRing R] [CharP R 2]

/-- The characteristic-two Frobenius cancellation which removes the mixed
term. This is the algebraic reason the staged multiplier remains separable. -/
theorem char_two_square_sum (A C : R) : (A+C)^2 = A^2+C^2 := by
  have h2 : (2 : R) = 0 := CharP.cast_eq_zero R 2
  calc
    (A+C)^2 = A^2+C^2 + 2*(A*C) := by ring
    _ = A^2+C^2 := by rw [h2]; ring

/-- The ordinary binary summation polynomial, explicitly collected as a
quadratic in the intermediate coordinate. No curve hypothesis is hidden. -/
theorem binary_s3_as_quadratic (x y u b : R) :
    (x*y+x*u+y*u)^2 + x*y*u + b =
      (x+y)^2*u^2 + (x*y)*u + (x*y)^2 + b := by
  have h2 : (2 : R) = 0 := CharP.cast_eq_zero R 2
  calc
    _ = (x+y)^2*u^2 + (x*y)*u + (x*y)^2 + b +
        (2:R)*(x*y*u*(x+y)) := by ring
    _ = (x+y)^2*u^2 + (x*y)*u + (x*y)^2 + b := by rw [h2]; ring

/-- Characteristic-two resultant expression, written with plus signs. -/
def quadraticResultantTwo (A B C D : R) : R :=
  (B+D)^2 + A*(A+C)*(B+D) + B*(A+C)^2

/-- The characteristic-two replacement for division by minus two. Its proof
is a polynomial identity using the explicit characteristic-two hypothesis. -/
theorem char_two_resultant_cancellation (A B C D u : R) :
    (A+C)^2 * (u^2+A*u+B) +
      u * ((A+C)^2*u + (A+C)*(B+D)) +
      (B+D) * ((A+C)*u+(B+D)) +
      A * ((A+C)^2*u+(A+C)*(B+D)) =
      quadraticResultantTwo A B C D := by
  have h2 : (2 : R) = 0 := CharP.cast_eq_zero R 2
  calc
    _ = quadraticResultantTwo A B C D +
        (2:R) * ((A+C)^2*u^2 + (A+C)^2*A*u + (A+C)*(B+D)*u) := by
      unfold quadraticResultantTwo
      ring
    _ = quadraticResultantTwo A B C D := by rw [h2]; ring

/-- Both scalar formulas agree in characteristic two. -/
theorem char_two_resultant_agrees (A B C D : R) :
    quadraticResultant A B C D = quadraticResultantTwo A B C D := by
  have hneg (x : R) : -x = x := by
    have hx : x+x = 0 := by
      calc
        x+x = (2:R)*x := by ring
        _ = 0 := by rw [show (2:R)=0 from CharP.cast_eq_zero R 2]; ring
    exact neg_eq_iff_add_eq_zero.mpr hx
  unfold quadraticResultant quadraticResultantTwo
  simp only [sub_eq_add_neg, hneg]

end CharacteristicTwo

#print axioms quadraticResultant
#print axioms quadratic_resultant_bezout
#print axioms common_root_implies_resultant_zero
#print axioms odd_linear_remainder
#print axioms odd_resultant_cancellation
#print axioms odd_full_construction
#print axioms inverse_guard_divided_difference
#print axioms cubic_missing_root
#print axioms char_two_resultant_integer_lift
#print axioms char_two_square_sum
#print axioms binary_s3_as_quadratic
#print axioms char_two_resultant_cancellation
#print axioms char_two_resultant_agrees

end FallDegreeIdentities
