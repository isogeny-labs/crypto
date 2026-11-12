import Init.Grind.Ring.Basic
import Init.Grind.Module.Basic

/-! Denominator-cleared chart identities for support of degree at most4,
and the conditional7-torsion quarter control. No actual fields,divisors,
projective curves,Cantor correctness or Picard functor is constructed. -/
set_option autoImplicit false
namespace IndependentMapChart
open Lean.Grind

theorem support_chart_homogenization {K : Type} [CommRing K]
    (x a l w u0 u1 u2 u3 u4 : K) (hc : (x-a)*w=l) :
    w^4*(u4*x^4+u3*x^3+u2*x^2+u1*x+u0) =
      u4*(a*w+l)^4+u3*w*(a*w+l)^3+u2*w^2*(a*w+l)^2+
      u1*w^3*(a*w+l)+u0*w^4 := by
  have h : x*w=a*w+l := by grind
  have h2 := congrArg (fun z : K => z^2) h
  have h3 := congrArg (fun z : K => z^3) h
  have h4 := congrArg (fun z : K => z^4) h
  grind

theorem ordinate_chart_homogenization {K : Type} [CommRing K]
    (x a l w y z v0 v1 v2 v3 : K)
    (hc : (x-a)*w=l) (hy : y=v3*x^3+v2*x^2+v1*x+v0)
    (hz : l*z=y*w^3) :
    l*z=v3*(a*w+l)^3+v2*w*(a*w+l)^2+v1*w^2*(a*w+l)+v0*w^3 := by
  have h : x*w=a*w+l := by grind
  have h2 := congrArg (fun t : K => t^2) h
  have h3 := congrArg (fun t : K => t^3) h
  grind

theorem seven_torsion_omitted_map_control {A B : Type}
    [IntModule A] [IntModule B]
    (norm : A → B) (pull : B → A)
    (hl : ∀ (n : Int) b, pull (n • b)=n • pull b)
    (hn : ∀ a, pull (norm a)=(4 : Int) • a)
    (a : A) (ha : (7 : Int) • a=0) :
    pull ((2 : Int) • norm a)=a := by
  rw [hl, hn]
  grind

end IndependentMapChart
#print axioms IndependentMapChart.support_chart_homogenization
#print axioms IndependentMapChart.ordinate_chart_homogenization
#print axioms IndependentMapChart.seven_torsion_omitted_map_control
