import Init.Grind.Ring.Basic

/-! Universal denominator-cleared identities for the two elliptic quotients
of an even sextic,including a nonmonic leading coefficient. Smoothness,
invertibility of 3,checked F101 scales,actual gluing and Picard maps remain
written/independently executed inputs,not axioms of these ring statements. -/
set_option autoImplicit false
namespace UnbalancedCovers
open Lean.Grind

theorem direct_short_quotient {R : Type} [CommRing R] (a b c d x : R) :
    27*a^2*(a*x^6+b*x^4+c*x^2+d) =
    (3*a*x^2+b)^3+(9*a*c-3*b^2)*(3*a*x^2+b)+
      (2*b^3-9*a*b*c+27*a^2*d) := by grind

theorem reciprocal_short_quotient {R : Type} [CommRing R] (a b c d x : R) :
    27*a^2*(a+b*x^2+c*x^4+d*x^6) =
    (3*a+b*x^2)^3+(9*a*c-3*b^2)*(3*a+b*x^2)*x^4+
      (2*b^3-9*a*b*c+27*a^2*d)*x^6 := by grind

end UnbalancedCovers
#print axioms UnbalancedCovers.direct_short_quotient
#print axioms UnbalancedCovers.reciprocal_short_quotient
