import Init.Grind.Ring.Basic
import Init.Grind.Module.Basic

/-! Algebra and group consequences of the G3 adapter.
The actual curves, Cartier ideals, Picard functors, Galois descent and the
identification of pullbacks/norms are written/native bridges, not defined here.
No global inverse isogeny or computational advantage is asserted. -/
set_option autoImplicit false
namespace G3PrimeDescent
open Lean.Grind

theorem completed_square_quartic {K : Type} [CommRing K]
    (t B s alpha beta : K) :
    (t^2+B*t+s)^2+alpha*t+beta =
      t^4+2*B*t^3+(B^2+2*s)*t^2+(2*B*s+alpha)*t+s^2+beta := by grind

theorem quartic_to_weierstrass {K : Type} [CommRing K]
    (t y B s alpha beta x Y : K)
    (curve : y^2=(t^2+B*t+s)^2+alpha*t+beta)
    (abscissa : x=2*(y+t^2+B*t+s))
    (ordinate : Y=2*x*t+B*x+alpha) :
    Y^2=x^3+(B^2-4*s)*x^2+(2*B*alpha-4*beta)*x+alpha^2 := by grind

theorem inverse_finite_branch {K : Type} [CommRing K]
    (t B alpha x Y inverse : K)
    (ordinate : Y=2*x*t+B*x+alpha)
    (unit : inverse*(2*x)=1) :
    inverse*(Y-B*x-alpha)=t := by grind

theorem zero_abscissa_ordinate_factor {K : Type} [CommRing K]
    (Y alpha : K) (curve_at_zero : Y^2=alpha^2) :
    (Y-alpha)*(Y+alpha)=0 := by grind

theorem actual_finite_zero_branch {K : Type} [CommRing K]
    (t B s alpha beta y x Y : K)
    (finite_root : alpha*t+beta=0)
    (ordinate_y : y= -(t^2+B*t+s))
    (abscissa : x=2*(y+t^2+B*t+s))
    (ordinate_Y : Y=2*x*t+B*x+alpha) :
    y^2=(t^2+B*t+s)^2+alpha*t+beta ∧ x=0 ∧ Y=alpha := by grind

theorem unique_finite_zero_parameter {K : Type} [CommRing K]
    (t alpha beta inverse : K) (unit : inverse*alpha=1)
    (finite_root : alpha*t+beta=0) :
    t= -inverse*beta := by grind

theorem conditional_trace_pullback_composition
    {E0 E1 E2 J : Type} [IntModule E0] [IntModule E1]
    [IntModule E2] [IntModule J]
    (p0 : E0 → J) (p1 : E1 → J) (p2 : E2 → J) (norm : J → E0)
    (norm_add : ∀ a b, norm (a+b)=norm a+norm b)
    (diagonal : ∀ x, norm (p0 x)=(2 : Int) • x)
    (cross1 : ∀ x, norm (p1 x)=0)
    (cross2 : ∀ x, norm (p2 x)=0)
    (x0 : E0) (x1 : E1) (x2 : E2) :
    norm (p0 x0+p1 x1+p2 x2)=(2 : Int) • x0 := by
  rw [norm_add, norm_add, diagonal, cross1, cross2]
  grind

theorem conditional_odd_subgroup_injectivity
    {E J : Type} [IntModule E]
    (phi : E → J) (norm : J → E)
    (composite : ∀ x, norm (phi x)=(2 : Int) • x)
    (ell u v : Int) (bezout : 2*u+ell*v=1)
    (x y : E) (order_x : ell • x=0) (order_y : ell • y=0)
    (images_equal : phi x=phi y) : x=y := by
  have h := congrArg norm images_equal
  rw [composite, composite] at h
  have coefficient : u*2+v*ell=1 := by grind
  have expand (z : E) : z=u • ((2 : Int) • z)+v • (ell • z) := by
    calc
      z=(1 : Int) • z := (IntModule.one_zsmul z).symm
      _=(u*2+v*ell) • z := by rw [coefficient]
      _=u • ((2 : Int) • z)+v • (ell • z) := by
        rw [IntModule.add_zsmul, IntModule.mul_zsmul, IntModule.mul_zsmul]
  calc
    x=u • ((2 : Int) • x)+v • (ell • x) := expand x
    _=u • ((2 : Int) • y)+v • (ell • y) := by rw [h, order_x, order_y]
    _=y := (expand y).symm

def repeatAdd {E : Type} [IntModule E] (x : E) : Nat → E
  | 0 => 0
  | n+1 => repeatAdd x n+x

theorem conditional_dlp_scalar_transfer
    {E J : Type} [IntModule E] [IntModule J]
    (phi : E → J) (zero : phi 0=0)
    (addition : ∀ x y, phi (x+y)=phi x+phi y)
    (x : E) (k : Nat) :
    phi (repeatAdd x k)=repeatAdd (phi x) k := by
  induction k with
  | zero => simp [repeatAdd, zero]
  | succ n ih => simp [repeatAdd, addition, ih]

#print axioms completed_square_quartic
#print axioms quartic_to_weierstrass
#print axioms inverse_finite_branch
#print axioms zero_abscissa_ordinate_factor
#print axioms actual_finite_zero_branch
#print axioms unique_finite_zero_parameter
#print axioms conditional_trace_pullback_composition
#print axioms conditional_odd_subgroup_injectivity
#print axioms conditional_dlp_scalar_transfer
end G3PrimeDescent
