import Init.Grind.Ring.Basic

/-! Cleared identities for the closed quadratic-source Richelot chart.
Coordinate rings, scheme multiplicities, the correspondence and balanced
Picard reduction remain explicit written bridges. Inverses are hypotheses. -/
set_option autoImplicit false
namespace ClosedFibres
open Lean.Grind

theorem quadratic_remainder {K : Type} [CommRing K]
    (x a b g0 g1 g2 : K) (hu : x^2+a*x+b=0) :
    g2*x^2+g1*x+g0 = (g1-a*g2)*x+(g0-b*g2) := by grind

theorem quadratic_resultant_identity {K : Type} [CommRing K]
    (x a b A B : K) :
    (A*x+B)*(B-a*A-A*x)+A^2*(x^2+a*x+b) =
      B^2-a*A*B+b*A^2 := by grind

theorem recovered_linear_coordinate {K : Type} [CommRing K]
    (A B inverse : K) (hi : A*inverse=1) :
    A*(-B*inverse)+B=0 := by grind

theorem recovered_quadratic_coordinate {K : Type} [CommRing K]
    (a b A B inverse : K) (hi : A*inverse=1)
    (hr : B^2-a*A*B+b*A^2=0) :
    (-B*inverse)^2+a*(-B*inverse)+b=0 := by
  have identity : (-B*inverse)^2+a*(-B*inverse)+b =
      inverse^2*(B^2-a*A*B+b*A^2) := by grind
  rw [identity,hr]
  grind

theorem repeated_support_resultant {K : Type} [CommRing K]
    (x A B : K) :
    B^2-(-2*x)*A*B+x^2*A^2 = (A*x+B)^2 := by grind

theorem cleared_ordinate_recovery {K : Type} [CommRing K]
    (numerator denominator inverse : K) (hi : denominator*inverse=1) :
    denominator*(numerator*inverse)=numerator := by grind

end ClosedFibres
#print axioms ClosedFibres.quadratic_remainder
#print axioms ClosedFibres.quadratic_resultant_identity
#print axioms ClosedFibres.recovered_linear_coordinate
#print axioms ClosedFibres.recovered_quadratic_coordinate
#print axioms ClosedFibres.repeated_support_resultant
#print axioms ClosedFibres.cleared_ordinate_recovery
