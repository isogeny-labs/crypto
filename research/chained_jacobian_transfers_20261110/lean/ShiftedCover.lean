import Init.Grind.Module.Basic
import Init.Grind.Ring.Basic

/-! Polynomial/structural certificates for the frozen shifted109 fixture.
No finite field, elliptic group or Jacobian construction is encoded. -/
set_option autoImplicit false
namespace ShiftedCover
open Lean.Grind

theorem general_cover_shift {K : Type} [CommRing K] (x s a b : K) :
    (x^2+s)^3+a*(x^2+s)+b =
      x^6+3*s*x^4+(3*s^2+a)*x^2+(s^3+a*s+b) := by grind

theorem source109_translation {K : Type} [CommRing K] (u : K) :
    (u+2)^3+(u+2)+46 = u^3+6*u^2+13*u+56 := by grind

theorem shifted_root_product {K : Type} [CommRing K] (r₁ r₂ r₃ s : K) :
    (r₁-s)*(r₂-s)*(r₃-s) = -((s-r₁)*(s-r₂)*(s-r₃)) := by grind

theorem recovery_four_mod109 {A : Type} [IntModule A]
    (p : A) (h : (109 : Int) • p = 0) :
    (82 : Int) • ((4 : Int) • p) = p := by grind

end ShiftedCover
#print axioms ShiftedCover.general_cover_shift
#print axioms ShiftedCover.source109_translation
#print axioms ShiftedCover.shifted_root_product
#print axioms ShiftedCover.recovery_four_mod109
