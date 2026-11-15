import CubeScaling

namespace VerheulDistortionAudit
noncomputable section
open WeierstrassCurve.Affine WeierstrassCurve.Affine.Point

variable {K : Type*} [Field K] [DecidableEq K]

theorem scaled_slope (a w : K) (hw : w^3=1) (x₁ x₂ y₁ y₂ : K) :
    (model a).slope (w*x₁) (w*x₂) y₁ y₂ = w^2*(model a).slope x₁ x₂ y₁ y₂ := by
  have hw0 := cube_root_ne_zero hw
  by_cases hx : x₁=x₂
  · subst x₂
    by_cases hy : y₁= -y₂
    · simp [slope,negY,model,hy]
    · simp [slope,negY,model,hy,mul_pow]
      ring
  · have hs : w*x₁≠w*x₂ := fun h => hx (mul_left_cancel₀ hw0 h)
    rw [slope_of_X_ne hs,slope_of_X_ne hx]
    field_simp [sub_ne_zero.mpr hx,sub_ne_zero.mpr hs]
    linear_combination -(y₁-y₂)*hw

omit [DecidableEq K] in
theorem scaled_addX (a w : K) (hw : w^3=1) (x₁ x₂ ℓ : K) :
    (model a).addX (w*x₁) (w*x₂) (w^2*ℓ) = w*(model a).addX x₁ x₂ ℓ := by
  simp only [addX,model,zero_mul,add_zero,sub_zero]
  linear_combination w*ℓ^2*hw

omit [DecidableEq K] in
theorem scaled_addY (a w : K) (hw : w^3=1) (x₁ x₂ y₁ ℓ : K) :
    (model a).addY (w*x₁) (w*x₂) y₁ (w^2*ℓ) = (model a).addY x₁ x₂ y₁ ℓ := by
  unfold addY negAddY
  rw [scaled_addX a w hw]
  simp only [negY,model,zero_mul,sub_zero,addX,add_zero]
  linear_combination -ℓ*(ℓ^2-x₁-x₂-x₁)*hw

variable (a w : K) (hΔ : (model a).Δ≠0) (hw : w^3=1)

def correctedPoint : (model a).Point → (model a).Point
  | 0 => 0
  | .some x y h => .some (w*x) y (corrected_nonsingular a w hΔ hw h)

omit [DecidableEq K] in
@[simp] theorem correctedPoint_zero : correctedPoint a w hΔ hw 0 = 0 := rfl

omit [DecidableEq K] in
@[simp] theorem correctedPoint_some {x y : K} (h : (model a).Nonsingular x y) :
    correctedPoint a w hΔ hw (.some x y h) =
      .some (w*x) y (corrected_nonsingular a w hΔ hw h) := rfl

theorem correctedPoint_add (P Q : (model a).Point) :
    correctedPoint a w hΔ hw (P+Q) =
      correctedPoint a w hΔ hw P+correctedPoint a w hΔ hw Q := by
  cases P with
  | zero => rfl
  | some x₁ y₁ h₁ =>
    cases Q with
    | zero => rfl
    | some x₂ y₂ h₂ =>
      have hw0 := cube_root_ne_zero hw
      by_cases hxy : x₁=x₂ ∧ y₁=(model a).negY x₂ y₂
      · rw [add_of_Y_eq hxy.1 hxy.2,correctedPoint_zero,correctedPoint_some,correctedPoint_some]
        symm
        apply add_of_Y_eq (congrArg (w*·) hxy.1)
        simpa [negY,model] using hxy.2
      · have hs : ¬ (w*x₁=w*x₂ ∧ y₁=(model a).negY (w*x₂) y₂) := by
          rintro ⟨hx,hy⟩
          exact hxy ⟨mul_left_cancel₀ hw0 hx,by simpa [negY,model] using hy⟩
        rw [add_some hxy,correctedPoint_some,correctedPoint_some,correctedPoint_some,add_some hs]
        simp only [some.injEq]
        rw [scaled_slope a w hw,scaled_addX a w hw,scaled_addY a w hw]
        exact ⟨rfl,rfl⟩

def correctedHom : (model a).Point →+ (model a).Point where
  toFun := correctedPoint a w hΔ hw
  map_zero' := rfl
  map_add' := correctedPoint_add a w hΔ hw

omit [DecidableEq K] in
theorem correctedPoint_cube (P : (model a).Point) :
    correctedPoint a w hΔ hw (correctedPoint a w hΔ hw (correctedPoint a w hΔ hw P)) = P := by
  cases P with
  | zero => rfl
  | some x y h =>
    simp only [correctedPoint_some,some.injEq]
    constructor
    · calc
        w*(w*(w*x)) = w^3*x := by ring
        _ = x := by rw [hw,one_mul]
    · trivial

def correctedEquiv : (model a).Point ≃+ (model a).Point where
  toFun := correctedPoint a w hΔ hw
  invFun P := correctedPoint a w hΔ hw (correctedPoint a w hΔ hw P)
  left_inv := correctedPoint_cube a w hΔ hw
  right_inv := correctedPoint_cube a w hΔ hw
  map_add' := correctedPoint_add a w hΔ hw

end
end VerheulDistortionAudit
