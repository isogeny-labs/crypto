import Init.Grind.Ring.Basic

/-! Exact algebra for the c=4 successor family. Denominators are cleared.
The projective extension, divisor correspondence and Picard functor are
not constructed by these ring lemmas. Characteristic restrictions enter
when interpreting the rational formulas as smooth curves and morphisms. -/
set_option autoImplicit false
namespace SuccessorCollapse
open Lean.Grind
variable {K : Type} [CommRing K]

theorem source_splitting (x : K) :
    (x^2-1)*(x^2+x+4)*(x^2-x+4) = x^6+6*x^4+9*x^2-16 := by
  grind

theorem target_numerator (z : K) :
    (z^2-4)*(z^2-10*z+1)*(z^2+10*z+1) =
      z^6-102*z^4+393*z^2-4 := by
  grind

theorem fiber_pair_x_cleared (x : K) :
    let a := x^2-1
    let b := x^2+x+4
    let c := x^2-x+4
    let A := x^2-x-6
    let T := 9*x^2+x-4
    let M := 2*x^3+19*x^2+7*x+74
    5*a*M^2+b*c*(98*A^2-100*b^2-2*T*A)+75*a*A^2=0 := by
  dsimp
  grind

theorem fiber_pair_y_cleared (x : K) :
    (x^2-1)*(2*x^3+19*x^2+7*x+74)-(x^2+x+4)^2*(x^2-x+4) =
      -(x^2-x-6)*((x^2-1)^2-24) := by
  grind

theorem fiber_line_constant (x : K) :
    (8*x^3+x^2-47*x-4)-4*(2*x^3+19*x^2+7*x+74) =
      -75*(x^2+x+4) := by
  grind

theorem endpoint_isomorphism_cleared (u v : K) :
    125*v^2-((5*u+4)^3-102*(5*u+4)^2+393*(5*u+4)-4) =
      125*(v^2-(u^3-18*u^2-15*u)) := by
  grind

/-- Standard quotient by (0,0) on y²=x³+A*x²+B*x. -/
theorem two_isogeny_cleared (x y A B : K) :
    let N := x^2+A*x+B
    y^2*(x^2-B)^2-(x*N^3-2*A*x^2*N^2+(A^2-4*B)*x^3*N) =
      (x^2-B)^2*(y^2-(x^3+A*x^2+B*x)) := by
  dsimp
  grind

end SuccessorCollapse

#print axioms SuccessorCollapse.source_splitting
#print axioms SuccessorCollapse.target_numerator
#print axioms SuccessorCollapse.fiber_pair_x_cleared
#print axioms SuccessorCollapse.fiber_pair_y_cleared
#print axioms SuccessorCollapse.fiber_line_constant
#print axioms SuccessorCollapse.endpoint_isomorphism_cleared
#print axioms SuccessorCollapse.two_isogeny_cleared
