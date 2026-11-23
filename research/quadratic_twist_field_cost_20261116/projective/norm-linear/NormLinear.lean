import Mathlib.Tactic
namespace TwistNormLinear
variable {K : Type*} [Field K]

theorem conjugate_reciprocal_vanish (sigma : K ≃+* K) (a b c x : K)
    (hnorm : sigma x*x=1) (hr : a*x^2+b*x+c=0) :
    sigma c*x^2+sigma b*x+sigma a=0 := by
  have hs := congrArg sigma hr
  simp only [map_add,map_mul,map_pow,map_zero] at hs
  linear_combination x^2*hs-(sigma a*(sigma x*x+1)+sigma b*x)*hnorm

theorem linear_combination_identity (a b c aa bb cc x : K) :
    cc*(a*x^2+b*x+c)-a*(cc*x^2+bb*x+aa) =
      (cc*b-a*bb)*x+(cc*c-a*aa) := by ring

theorem norm_root_annihilates_linear (sigma : K ≃+* K) (a b c x : K)
    (hnorm : sigma x*x=1) (hr : a*x^2+b*x+c=0) :
    (sigma c*b-a*sigma b)*x+(sigma c*c-a*sigma a)=0 := by
  have hs := conjugate_reciprocal_vanish sigma a b c x hnorm hr
  linear_combination (sigma c)*hr-a*hs

theorem nonzero_linear_complete (d e x : K) (hd : d ≠ 0)
    (hx : d*x+e=0) : x = -e/d := by
  apply (eq_div_iff hd).2
  linear_combination hx

theorem nonzero_constant_empty (d e x : K) (hd : d=0) (he : e ≠ 0) :
    d*x+e ≠ 0 := by simpa [hd] using he
end TwistNormLinear
