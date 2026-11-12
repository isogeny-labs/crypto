import Init.Grind.Ring.Basic

/-! Finite-algebra identities for the branch Möbius chart in the second
Cantor replay. Explicit inverses/curve-chart equations are hypotheses.
Smooth projective completion and the Picard divisor K~2W remain written. -/
set_option autoImplicit false
namespace KernelChart
open Lean.Grind

theorem quadratic_mobius_identity {K : Type} [CommRing K]
    (x a l w b c : K) :
    w^2*(x^2+b*x+c)-
      ((a^2+b*a+c)*w^2+l*(2*a+b)*w+l^2) =
      ((x-a)*w-l)*((x+a+b)*w+l) := by grind

theorem quadratic_support_cleared {K : Type} [CommRing K]
    (x a l w b c : K) (hu : x^2+b*x+c=0)
    (hchart : (x-a)*w=l) :
    (a^2+b*a+c)*w^2+l*(2*a+b)*w+l^2=0 := by
  have h := quadratic_mobius_identity x a l w b c
  rw [hu,hchart] at h
  grind

theorem monic_quadratic_support {K : Type} [CommRing K]
    (x a l w b c inverse : K) (hu : x^2+b*x+c=0)
    (hchart : (x-a)*w=l) (hi : (a^2+b*a+c)*inverse=1) :
    w^2+(l*(2*a+b)*inverse)*w+l^2*inverse=0 := by
  have h := quadratic_support_cleared x a l w b c hu hchart
  have scaled := congrArg (fun z : K => inverse*z) h
  grind

theorem ordinate_support_transform {K : Type} [CommRing K]
    (x a l w v0 v1 inverse : K) (hchart : (x-a)*w=l)
    (hi : l*inverse=1) :
    inverse*(v0+v1*x)*w^3=v1*w^2+inverse*(v0+v1*a)*w^3 := by
  grind

end KernelChart
#print axioms KernelChart.quadratic_mobius_identity
#print axioms KernelChart.quadratic_support_cleared
#print axioms KernelChart.monic_quadratic_support
#print axioms KernelChart.ordinate_support_transform
