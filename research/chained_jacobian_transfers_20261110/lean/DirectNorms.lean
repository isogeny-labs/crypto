import Init.Grind.Ring.Basic

/-! Cleared algebra behind the direct balanced quotient norms. Actual finite
fields,valid Mumford jets,elliptic group law and Picard norms remain written
bridges. Ring inverses are explicit hypotheses,not unsafe division by zero. -/
set_option autoImplicit false
namespace DirectNorms
open Lean.Grind

theorem quadratic_sum_squares {K : Type} [CommRing K]
    (x y a b : K) (hs : x+y = -a) (hp : x*y=b) :
    x^2+y^2 = a^2-2*b := by grind

theorem quotient_slope_constraint {K : Type} [CommRing K]
    (lambda a inverse v1 : K) (hi : (lambda*a)*inverse=1) :
    (-v1*inverse)*lambda*a+v1=0 := by grind

theorem quotient_ordinate_support {K : Type} [CommRing K]
    (x a b v0 v1 lambda s r m : K)
    (hu : x^2+a*x+b=0) (hm : m*lambda*a+v1=0) :
    m*(lambda*x^2+s-r)-(v0+v1*x) = m*(s-lambda*b-r)-v0 := by grind

theorem reciprocal_support {K : Type} [CommRing K]
    (x t a b inverse : K) (hu : x^2+a*x+b=0)
    (ht : x*t=1) (hb : b*inverse=1) :
    t^2+(a*inverse)*t+inverse=0 := by
  have identity : b*t^2+a*t+1 =
      t^2*(x^2+a*x+b)+(1-x*t)*(1+x*t+a*t) := by grind
  have cleared : b*t^2+a*t+1=0 := by rw [identity,hu,ht]; grind
  have monic : t^2+(a*inverse)*t+inverse =
      inverse*(b*t^2+a*t+1)+(1-b*inverse)*t^2 := by grind
  rw [monic,cleared,hb]
  grind

theorem reciprocal_ordinate_remainder {K : Type} [CommRing K]
    (t a b v0 v1 scale : K) :
    scale*(v0*t^3+v1*t^2) -
      (scale*(v0*a*b-v1*b)+scale*(v0*(a^2-b)-v1*a)*t) =
      scale*(v0*t+v1-a*v0)*(t^2+a*t+b) := by grind

theorem even_support_remainder {K : Type} [CommRing K]
    (x b v0 v1 : K) :
    (v0+v1*x)^2-((v0^2-b*v1^2)+(2*v0*v1)*x) =
      v1^2*(x^2+b) := by grind

theorem even_support_zero_factor {K : Type} [CommRing K]
    (v0 v1 half : K) (h2 : (2 : K)*half=1)
    (hc : (2 : K)*v0*v1=0)
    (domain : ∀ a b : K, a*b=0 → a=0 ∨ b=0) : v0=0 ∨ v1=0 := by
  have hz : v0*v1=0 := by
    calc
      v0*v1 = (2*half)*(v0*v1) := by rw [h2]; grind
      _ = half*(2*v0*v1) := by grind
      _ = 0 := by rw [hc]; grind
  exact domain v0 v1 hz

end DirectNorms
#print axioms DirectNorms.quadratic_sum_squares
#print axioms DirectNorms.quotient_slope_constraint
#print axioms DirectNorms.quotient_ordinate_support
#print axioms DirectNorms.reciprocal_support
#print axioms DirectNorms.reciprocal_ordinate_remainder
#print axioms DirectNorms.even_support_remainder
#print axioms DirectNorms.even_support_zero_factor
