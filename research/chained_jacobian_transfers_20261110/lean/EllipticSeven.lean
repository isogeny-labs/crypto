import Init.Grind.Module.Basic
import Init.Grind.Ring.Basic

/-! Coordinate/scalar certificates for prospective fixed degree7 maps.
No finite fields,actual kernels or geometric isogenies are constructed. -/
set_option autoImplicit false
namespace EllipticSeven
open Lean.Grind

theorem short_translation_cleared {K : Type} [CommRing K]
    (x a₂ a₄ a₆ : K) :
    (3*x+a₂)^3+(9*a₄-3*a₂^2)*(3*x+a₂)+(2*a₂^3-9*a₂*a₄+27*a₆) =
      27*(x^3+a₂*x^2+a₄*x+a₆) := by grind

theorem isomorphism_scaling {K : Type} [CommRing K] (x y a b u : K) :
    (u^3*y)^2-((u^2*x)^3+(u^4*a)*(u^2*x)+u^6*b) =
      u^6*(y^2-(x^3+a*x+b)) := by grind

theorem recover_order109 {A : Type} [IntModule A]
    (p : A) (h : (109 : Int) • p = 0) :
    (78 : Int) • ((7 : Int) • p) = p := by grind

theorem recover_order113 {A : Type} [IntModule A]
    (p : A) (h : (113 : Int) • p = 0) :
    (97 : Int) • ((7 : Int) • p) = p := by grind

end EllipticSeven
#print axioms EllipticSeven.short_translation_cleared
#print axioms EllipticSeven.isomorphism_scaling
#print axioms EllipticSeven.recover_order109
#print axioms EllipticSeven.recover_order113
