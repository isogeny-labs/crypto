import Init.Grind.Ring.Basic

/-! Exact C2 specializations for the two quotient curves used by CT25.
The cleared polynomial identities quantify over arbitrary commutative rings;
the curve implications explicitly assume characteristic101. No division,
finite-field construction,balanced norm or Picard functor is encoded here. -/
set_option autoImplicit false
namespace SuccessorNorms
open Lean.Grind

theorem first_quotient_identity_cleared {K : Type} [CommRing K] (x : K) :
    ((x^2+41)^3+17*(x^2+41)+91) - (x^6+22*x^4+10*x^2+19) =
      101*(x^4+50*x^2+690) := by grind

theorem first_quotient_curve {K : Type} [CommRing K]
    (x y : K) (h101 : (101 : K)=0)
    (hy : y^2=x^6+22*x^4+10*x^2+19) :
    y^2=(x^2+41)^3+17*(x^2+41)+91 := by
  have h := first_quotient_identity_cleared x
  rw [h101] at h
  grind

theorem reciprocal_quotient_identity_cleared {K : Type} [CommRing K] (x : K) :
    ((19+37*x^2)^3+48*(19+37*x^2)*x^4+48*x^6) -
      19^2*(x^6+22*x^4+10*x^2+19) =
      101*(361*x^2+703*x^4+516*x^6) := by grind

theorem reciprocal_quotient_curve_cleared {K : Type} [CommRing K]
    (x y : K) (h101 : (101 : K)=0)
    (hy : y^2=x^6+22*x^4+10*x^2+19) :
    (19*y)^2=(19+37*x^2)^3+48*(19+37*x^2)*x^4+48*x^6 := by
  have h := reciprocal_quotient_identity_cleared x
  rw [h101] at h
  grind

theorem reciprocal_quotient_curve_from_inverse {K : Type} [CommRing K]
    (x y t : K) (h101 : (101 : K)=0)
    (hy : y^2=x^6+22*x^4+10*x^2+19) (ht : x*t=1) :
    (19*y*t^3)^2=(19*t^2+37)^3+48*(19*t^2+37)+48 := by
  have h := reciprocal_quotient_curve_cleared x y h101 hy
  have hs := congrArg (fun z : K => z*t^6) h
  grind

theorem actual_scale18_coefficients :
    (18^4*17 : Nat)%101=23 ∧ (18^6*91 : Nat)%101=7 ∧
    (18^4*48 : Nat)%101=59 ∧ (18^6*48 : Nat)%101=27 := by decide

theorem exact_shift_and_constant_coefficients :
    (3*41 : Nat)%101=22 ∧ (3*37 : Nat)%101=10 ∧
    (25^2 : Nat)%101=19 := by decide

end SuccessorNorms
#print axioms SuccessorNorms.first_quotient_identity_cleared
#print axioms SuccessorNorms.first_quotient_curve
#print axioms SuccessorNorms.reciprocal_quotient_identity_cleared
#print axioms SuccessorNorms.reciprocal_quotient_curve_cleared
#print axioms SuccessorNorms.reciprocal_quotient_curve_from_inverse
#print axioms SuccessorNorms.actual_scale18_coefficients
#print axioms SuccessorNorms.exact_shift_and_constant_coefficients
