import GeometricCardinality

namespace BKBasefieldScalingAudit
noncomputable section
open ThreeTorsionRankAudit
variable {K : Type*} [Field K]

theorem short_nonsingular_iff (a b x y : K) :
    (shortCurve a b).Nonsingular x y ↔
    y^2=x^3+a*x+b ∧ (3*x^2+a≠0 ∨ 2*y≠0) := by
  rw [(shortCurve a b).nonsingular_iff']
  simp only [shortCurve,WeierstrassCurve.Affine.equation_iff,zero_mul,mul_zero,
    zero_add,add_zero,zero_sub,sub_zero,neg_ne_zero]

theorem scaled_equation (a b u x y : K) (he : y^2=x^3+a*x+b) :
    (u^3*y)^2=(u^2*x)^3+(u^4*a)*(u^2*x)+u^6*b := by
  linear_combination u^6*he

theorem scaled_nonsingular (a b u : K) (hu : u≠0) {x y : K}
    (h : (shortCurve a b).Nonsingular x y) :
    (shortCurve (u^4*a) (u^6*b)).Nonsingular (u^2*x) (u^3*y) := by
  rw [short_nonsingular_iff] at h ⊢
  refine ⟨scaled_equation a b u x y h.1,?_⟩
  rcases h.2 with hx | hy
  · left
    convert mul_ne_zero (pow_ne_zero 4 hu) hx using 1 <;> ring
  · right
    convert mul_ne_zero (pow_ne_zero 3 hu) hy using 1 <;> ring

def scalePoint (a b u : K) (hu : u≠0) :
    (shortCurve a b).Point → (shortCurve (u^4*a) (u^6*b)).Point
  | .zero => .zero
  | .some x y h => .some (u^2*x) (u^3*y) (scaled_nonsingular a b u hu h)

theorem scaled_discriminant (a b u : K) :
    4*(u^4*a)^3+27*(u^6*b)^2=u^12*(4*a^3+27*b^2) := by ring

variable [DecidableEq K]

theorem scaled_slope (a b u : K) (hu : u≠0) (x₁ x₂ y₁ y₂ : K) :
    (shortCurve (u^4*a) (u^6*b)).slope (u^2*x₁) (u^2*x₂) (u^3*y₁) (u^3*y₂)=
      u*(shortCurve a b).slope x₁ x₂ y₁ y₂ := by
  have hx : u^2*x₁=u^2*x₂ ↔ x₁=x₂ := mul_right_inj' (pow_ne_zero 2 hu)
  have hy : u^3*y₁= -(u^3*y₂) ↔ y₁= -y₂ := by
    rw [← mul_neg]
    exact mul_right_inj' (pow_ne_zero 3 hu)
  simp only [WeierstrassCurve.Affine.slope,WeierstrassCurve.Affine.negY,shortCurve,
    zero_mul,mul_zero,zero_add,add_zero,sub_zero,hx,hy]
  split_ifs
  · ring
  · rw [show 3*(u^2*x₁)^2+u^4*a=u^4*(3*x₁^2+a) by ring]
    rw [show u^3*y₁- -(u^3*y₁)=u^3*(y₁- -y₁) by ring,mul_div_mul_comm]
    have hratio : u^4/u^3=u := by field_simp
    rw [hratio]
  · rw [show u^3*y₁-u^3*y₂=u^3*(y₁-y₂) by ring,
      show u^2*x₁-u^2*x₂=u^2*(x₁-x₂) by ring,mul_div_mul_comm]
    have hratio : u^3/u^2=u := by field_simp
    rw [hratio]

theorem scaled_addX (a b u x₁ x₂ s : K) :
    (shortCurve (u^4*a) (u^6*b)).addX (u^2*x₁) (u^2*x₂) (u*s)=
      u^2*(shortCurve a b).addX x₁ x₂ s := by
  simp only [WeierstrassCurve.Affine.addX,shortCurve];ring

theorem scaled_addY (a b u x₁ x₂ y₁ s : K) :
    (shortCurve (u^4*a) (u^6*b)).addY (u^2*x₁) (u^2*x₂) (u^3*y₁) (u*s)=
      u^3*(shortCurve a b).addY x₁ x₂ y₁ s := by
  simp only [WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY,WeierstrassCurve.Affine.addX,shortCurve]
  ring

#print axioms scaled_slope
#print axioms scaled_nonsingular
end
end BKBasefieldScalingAudit
