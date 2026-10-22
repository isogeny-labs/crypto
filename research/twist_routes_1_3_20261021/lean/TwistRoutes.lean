import Mathlib.AlgebraicGeometry.EllipticCurve.Affine.Point
import Mathlib.Tactic.Ring
import Mathlib.Tactic.Abel

/-!
Faithful polynomial preconditions and eigenspace consequences on Mathlib's
actual nonsingular Weierstrass point group. The endomorphism and its eigenvalue
hypotheses are explicit. Constructing the finite-field Frobenius, the quadratic
twist embedding, the odd subgroup and the implemented Jacobian/chart maps is
still an open formal bridge. None of those claims is assumed as an axiom here.
-/
namespace TwistRoutes

section PolynomialPreconditions
variable {R : Type*} [CommRing R]

def cubic (A B x : R) : R := x^3 + A*x + B
def complementA3 (A B c : R) : R := 3*A*c^2 + 9*B*c - A^2
def complementB27 (A B c : R) : R :=
  -27*B*c^3 + 18*A^2*c^2 + 27*A*B*c + 27*B^2 + 2*A^3

theorem trace_quartic_discriminant (A X Y m : R) :
    m^4 - 6*X*m^2 - 8*Y*m - 3*X^2 - 4*A =
      (m^2-X)^2 - 4*(X^2 + X*m^2 + 2*m*Y + A) := by ring

theorem trace_chord_polynomial_division (A B X Y m t : R) :
    cubic A B t - (m*(t-X)-Y)^2 =
      (t-X)*(t^2 + (X-m^2)*t + X^2 + X*m^2 + 2*m*Y + A)
        + (cubic A B X - Y^2) := by simp only [cubic]; ring

theorem cover_complement_A_cleared (A B c : R) :
    complementA3 A B c = 9*c*cubic A B c - (3*c^2+A)^2 := by
  simp only [complementA3, cubic]; ring

theorem cover_complement_B_cleared (A B c : R) :
    complementB27 A B c =
      2*(3*c^2+A)^3 - 27*(3*c^2+A)*c*cubic A B c + 27*(cubic A B c)^2 := by
  simp only [complementB27, cubic]; ring

theorem cover_complement_change_of_variables (A B c t : R) :
    27*(cubic A B c)^2*cubic A B (t+c) =
      (3*cubic A B c + (3*c^2+A)*t)^3
        + 3*t^2*complementA3 A B c*(3*cubic A B c + (3*c^2+A)*t)
        + t^3*complementB27 A B c := by
  simp only [cubic, complementA3, complementB27]; ring
end PolynomialPreconditions

section PointGroup
variable {K : Type*} [Field K] [DecidableEq K]
variable (E : WeierstrassCurve.Affine K)

def tracePoint (pi : E.Point →+ E.Point) (P : E.Point) : E.Point := P + pi P

theorem anti_invariant_trace_zero (pi : E.Point →+ E.Point) (P : E.Point)
    (hP : pi P = -P) : tracePoint E pi P = 0 := by
  simp only [tracePoint, hP, add_neg_cancel]

theorem rational_trace_double (pi : E.Point →+ E.Point) (P : E.Point)
    (hP : pi P = P) : tracePoint E pi P = P + P := by
  simp only [tracePoint, hP]

theorem kernel_translate_same_trace (pi : E.Point →+ E.Point) (S U : E.Point)
    (hU : tracePoint E pi U = 0) : tracePoint E pi (S+U) = tracePoint E pi S := by
  simp only [tracePoint, map_add] at *
  calc
    S + U + (pi S + pi U) = (S + pi S) + (U + pi U) := by abel
    _ = S + pi S := by rw [hU, add_zero]

theorem mixed_relation_twist_double_zero (pi : E.Point →+ E.Point)
    (R A B : E.Point) (h : R = A+B) (hR : pi R = R)
    (hA : pi A = A) (hB : pi B = -B) : B+B = 0 := by
  have h' : R = A + (-B) := by
    calc
      R = pi R := hR.symm
      _ = pi (A+B) := congrArg pi h
      _ = A + (-B) := by rw [map_add, hA, hB]
  have hneg : B = -B := add_left_cancel (h.symm.trans h')
  calc
    B+B = (-B)+B := congrArg (fun P => P+B) hneg
    _ = 0 := neg_add_cancel B

end PointGroup
end TwistRoutes

#print axioms TwistRoutes.trace_quartic_discriminant
#print axioms TwistRoutes.trace_chord_polynomial_division
#print axioms TwistRoutes.cover_complement_A_cleared
#print axioms TwistRoutes.cover_complement_B_cleared
#print axioms TwistRoutes.cover_complement_change_of_variables
#print axioms TwistRoutes.anti_invariant_trace_zero
#print axioms TwistRoutes.rational_trace_double
#print axioms TwistRoutes.kernel_translate_same_trace
#print axioms TwistRoutes.mixed_relation_twist_double_zero
