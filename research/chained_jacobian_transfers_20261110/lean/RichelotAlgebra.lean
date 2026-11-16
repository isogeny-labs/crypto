import Init.Grind.Module.Basic
import Init.Grind.Ring.Basic

/-! Algebraic certificates for the operational non-split prototype. No
finite field, projective correspondence or Picard functor is constructed. -/
set_option autoImplicit false
namespace RichelotAlgebra
open Lean.Grind
def quad {K : Type} [CommRing K] (a b c x : K) := a+b*x+c*x^2
def bracket {K : Type} [CommRing K] (a b c d e f z : K) :=
  (b+2*c*z)*quad d e f z - quad a b c z*(e+2*f*z)
def delta {K : Type} [CommRing K] (a b c d e f g h i : K) :=
  a*e*i+b*f*g+c*d*h-a*f*h-b*d*i-c*e*g

theorem fundamental_identity {K : Type} [CommRing K]
    (a b c d e f g h i x z : K) :
    quad a b c x * bracket d e f g h i z +
    quad d e f x * bracket g h i a b c z +
    quad g h i x * bracket a b c d e f z =
      -(delta a b c d e f g h i)*(x-z)^2 := by
  simp only [quad, bracket, delta]
  grind

theorem correspondence_target_cleared {K : Type} [CommRing K]
    (a b c u v w t y Y : K)
    (hsum : a*u+b*v+c*w = -t^2) (hcorr : a*u+b*v=0)
    (hsource : y^2=a*b*c) (hordinate : y*Y=a*u*t) :
    (a*b*c)*(Y^2-u*v*w)=0 := by
  have hb : b*v=-(a*u) := by grind
  have hc : c*w=-(t^2) := by grind
  have htarget : (a*b*c)*(u*v*w)=(a*u*t)^2 := by
    calc
      _ = (a*u)*(b*v)*(c*w) := by grind
      _ = (a*u)*(-(a*u))*(-(t^2)) := by rw [hb,hc]
      _ = (a*u*t)^2 := by grind
  have himage : (a*b*c)*Y^2=(a*u*t)^2 := by
    calc
      _ = y^2*Y^2 := by rw [hsource]
      _ = (y*Y)^2 := by grind
      _ = (a*u*t)^2 := by rw [hordinate]
  grind

theorem balanced_infinity_sum {A : Type} [IntModule A]
    (d p : A) (m n : Int) :
    d + n • p + (2-m-n) • (-p) = d + (2*n+m-2) • p := by
  rw [IntModule.zsmul_neg, ← IntModule.neg_zsmul,
    AddCommMonoid.add_assoc, ← IntModule.add_zsmul]
  have h : n + -(2-m-n) = 2*n+m-2 := by grind
  rw [h]

theorem recovery_four_mod19 {A : Type} [IntModule A]
    (p : A) (h : (19 : Int) • p = 0) :
    (5 : Int) • ((4 : Int) • p) = p := by grind

end RichelotAlgebra
#print axioms RichelotAlgebra.fundamental_identity
#print axioms RichelotAlgebra.correspondence_target_cleared
#print axioms RichelotAlgebra.balanced_infinity_sum
#print axioms RichelotAlgebra.recovery_four_mod19
