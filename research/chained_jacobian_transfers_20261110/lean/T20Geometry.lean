import Init.Grind.Ring.Basic

/-! Common leading scale and normalized Richelot brackets.
Actual quadratic splitting, nonzero determinant, Galois kernel, raw target,
discriminant twist and Picard quotient are written/native inputs. -/
set_option autoImplicit false
namespace T20Geometry
open Lean.Grind

theorem common_leading_determinant {R : Type} [CommRing R]
    (q a b c d e f g h i : R) :
    (q*a)*(q*e)*(q*i)+(q*b)*(q*f)*(q*g)+(q*c)*(q*d)*(q*h)-
      (q*a)*(q*f)*(q*h)-(q*b)*(q*d)*(q*i)-(q*c)*(q*e)*(q*g) =
    q^3*(a*e*i+b*f*g+c*d*h-a*f*h-b*d*i-c*e*g) := by grind

theorem normalization_factorization {R : Type} [CommRing R]
    (q s d t a b c : R) :
    q^3*d^3*((s*t*a)*(s*t*b)*(s*t*c)) =
      (q*s)^3*(d*t)^3*(a*b*c) := by grind

theorem normalization_from_units {R : Type} [CommRing R]
    (q s d t a b c : R) (hq : q*s=1) (hd : d*t=1) :
    q^3*d^3*((s*t*a)*(s*t*b)*(s*t*c))=a*b*c := by
  rw [normalization_factorization, hq, hd]
  grind

theorem actual_normalization_constants :
    (11 : Fin 101)^3=18 ∧ (18 : Fin 101)*67=95 ∧
    (18 : Fin 101)*34=6 ∧ (50 : Fin 101)^2=76 := by decide

end T20Geometry
#print axioms T20Geometry.common_leading_determinant
#print axioms T20Geometry.normalization_factorization
#print axioms T20Geometry.normalization_from_units
#print axioms T20Geometry.actual_normalization_constants
