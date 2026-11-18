import Scaling

namespace BKBasefieldScalingAudit
noncomputable section
open ThreeTorsionRankAudit
variable {K : Type*} [Field K] [DecidableEq K]

theorem scalePoint_add (a b u : K) (hu : u≠0) (P Q : (shortCurve a b).Point) :
    scalePoint a b u hu (P+Q)=scalePoint a b u hu P+scalePoint a b u hu Q := by
  cases P with
  | zero => simp only [← WeierstrassCurve.Affine.Point.zero_def,zero_add,scalePoint]
  | some x₁ y₁ h₁ =>
    cases Q with
    | zero => simp only [← WeierstrassCurve.Affine.Point.zero_def,add_zero,scalePoint]
    | some x₂ y₂ h₂ =>
      have hx : u^2*x₁=u^2*x₂ ↔ x₁=x₂ := mul_right_inj' (pow_ne_zero 2 hu)
      have hy : u^3*y₁= -(u^3*y₂) ↔ y₁= -y₂ := by
        rw [← mul_neg];exact mul_right_inj' (pow_ne_zero 3 hu)
      have hv : (u^2*x₁=u^2*x₂ ∧
          u^3*y₁=(shortCurve (u^4*a) (u^6*b)).negY (u^2*x₂) (u^3*y₂)) ↔
          (x₁=x₂ ∧ y₁=(shortCurve a b).negY x₂ y₂) := by
        simp only [WeierstrassCurve.Affine.negY,shortCurve,zero_mul,sub_zero,hx,hy]
      by_cases hxy : x₁=x₂ ∧ y₁=(shortCurve a b).negY x₂ y₂
      · have hs:=hv.mpr hxy
        rw [WeierstrassCurve.Affine.Point.add_of_Y_eq hxy.1 hxy.2]
        change (0:(shortCurve (u^4*a) (u^6*b)).Point)=
          WeierstrassCurve.Affine.Point.some _ _ _+WeierstrassCurve.Affine.Point.some _ _ _
        rw [WeierstrassCurve.Affine.Point.add_of_Y_eq hs.1 hs.2]
      · have hs : ¬(u^2*x₁=u^2*x₂ ∧
            u^3*y₁=(shortCurve (u^4*a) (u^6*b)).negY (u^2*x₂) (u^3*y₂)) :=
          fun hh => hxy (hv.mp hh)
        rw [WeierstrassCurve.Affine.Point.add_some hxy]
        simp only [scalePoint]
        rw [WeierstrassCurve.Affine.Point.add_some hs]
        simp only [WeierstrassCurve.Affine.Point.some.injEq]
        rw [scaled_slope a b u hu,scaled_addX,scaled_addY]
        exact ⟨rfl,rfl⟩

def scalingHom (a b u : K) (hu : u≠0) :
    (shortCurve a b).Point →+ (shortCurve (u^4*a) (u^6*b)).Point where
  toFun:=scalePoint a b u hu
  map_zero':=rfl
  map_add':=scalePoint_add a b u hu

theorem scalePoint_injective (a b u : K) (hu : u≠0) :
    Function.Injective (scalePoint a b u hu) := by
  intro P Q h
  cases P <;> cases Q
  · rfl
  · cases h
  · cases h
  · simp only [scalePoint,WeierstrassCurve.Affine.Point.some.injEq] at h ⊢
    exact ⟨(mul_right_inj' (pow_ne_zero 2 hu)).mp h.1,
      (mul_right_inj' (pow_ne_zero 3 hu)).mp h.2⟩

theorem unscaled_nonsingular (a b u : K) (hu : u≠0) {x y : K}
    (h : (shortCurve (u^4*a) (u^6*b)).Nonsingular x y) :
    (shortCurve a b).Nonsingular ((u⁻¹)^2*x) ((u⁻¹)^3*y) := by
  have hh:=scaled_nonsingular (u^4*a) (u^6*b) u⁻¹ (inv_ne_zero hu) h
  simpa only [← mul_assoc,← mul_pow,inv_mul_cancel₀ hu,one_pow,one_mul] using hh

theorem scalePoint_surjective (a b u : K) (hu : u≠0) :
    Function.Surjective (scalePoint a b u hu) := by
  intro Q
  cases Q with
  | zero => exact ⟨.zero,rfl⟩
  | some x y h =>
    refine ⟨.some ((u⁻¹)^2*x) ((u⁻¹)^3*y) (unscaled_nonsingular a b u hu h),?_⟩
    simp only [scalePoint,WeierstrassCurve.Affine.Point.some.injEq]
    simp only [← mul_assoc,← mul_pow,mul_inv_cancel₀ hu,one_pow,one_mul,and_self]

def scalingEquiv (a b u : K) (hu : u≠0) :
    (shortCurve a b).Point ≃+ (shortCurve (u^4*a) (u^6*b)).Point :=
  AddEquiv.ofBijective (scalingHom a b u hu)
    ⟨scalePoint_injective a b u hu,scalePoint_surjective a b u hu⟩

#print axioms scalingEquiv
end
end BKBasefieldScalingAudit
