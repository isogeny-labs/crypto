module
public import Mathlib.Algebra.Polynomial.Coeff
public import Mathlib.Tactic
public section
namespace NormQuotientInteger
open Polynomial

/-- Universal integer lift of the planted F17 norm identity. -/
theorem normLift {R : Type*} [CommRing R] :
    (C (5:R)*X^2+C 11*X+C 6)^2-(X^3+C 2*X+C 2)-
      C 8*(X*(X-C 5)*(X-C 6)*(X-C 3))=
    C 17*(X^4+C 13*X^3-C 19*X^2+C 50*X+C 2) := by
  simp only [map_ofNat]
  ring

/-- Universal integer lift of the modulo-A inverse identity. -/
theorem inverseLift {R : Type*} [CommRing R] :
    (X^3+C (2:R)*X+C 2)*(C 7*X+C 6)-1-
      ((X-C 16)*(X-C 9))*(C 7*X^2+C 11*X+C 12)=
    C 17*(C 10*X^3-C 43*X^2-C 74*X-C 101) := by
  simp only [map_ofNat]
  ring
end NormQuotientInteger
#print axioms NormQuotientInteger.normLift
#print axioms NormQuotientInteger.inverseLift
