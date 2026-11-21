module
public import TranslationReciprocal

@[expose] public section
namespace EllipticTranslationAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerNormValuationAudit MillerBasechangeAudit
variable {K : Type*} [Field K]

/-- Explicit actual-model adapter for the official addition theorem, retaining its actual domains. -/
theorem generic_add_some (W : WeierstrassCurve.Affine K) {x₁ y₁ x₂ y₂ : W.FunctionField}
    (h₁ : (genericCurve W).Nonsingular x₁ y₁) (h₂ : (genericCurve W).Nonsingular x₂ y₂)
    (hxy : ¬ (x₁=x₂ ∧ y₁=(genericCurve W).negY x₂ y₂)) :
    Point.some x₁ y₁ h₁+Point.some x₂ y₂ h₂ =
      Point.some ((genericCurve W).addX x₁ x₂ ((genericCurve W).slope x₁ x₂ y₁ y₂))
        ((genericCurve W).addY x₁ x₂ y₁ ((genericCurve W).slope x₁ x₂ y₁ y₂))
        ((genericCurve W).nonsingular_add h₁ h₂ hxy) := Point.add_some hxy

theorem example_generic_coefficients (K : Type*) [Field K] :
    (genericCurve (translationExampleCurve K)).a₁=0 ∧
    (genericCurve (translationExampleCurve K)).a₂=0 ∧
    (genericCurve (translationExampleCurve K)).a₃=0 ∧
    (genericCurve (translationExampleCurve K)).a₄=1 ∧
    (genericCurve (translationExampleCurve K)).a₆=0 := by
  refine ⟨?_,?_,?_,?_,?_⟩
  all_goals first
    | change algebraMap K (translationExampleCurve K).FunctionField (0 : K)=0; exact map_zero _
    | change algebraMap K (translationExampleCurve K).FunctionField (1 : K)=1; exact map_one _

theorem example_generic_nonsingular_zero (K : Type*) [Field K] :
    (genericCurve (translationExampleCurve K)).Nonsingular 0 0 := by
  obtain ⟨h₁,h₂,h₃,h₄,h₆⟩ := example_generic_coefficients K
  rw [WeierstrassCurve.Affine.nonsingular_iff',WeierstrassCurve.Affine.equation_iff]
  rw [h₁,h₂,h₃,h₄,h₆]
  norm_num

theorem example_add_x_formula (K : Type*) [Field K] :
    (genericCurve (translationExampleCurve K)).addX (xFunction (translationExampleCurve K)) 0
      ((genericCurve (translationExampleCurve K)).slope (xFunction (translationExampleCurve K)) 0
        (yFunction (translationExampleCurve K)) 0) = (xFunction (translationExampleCurve K))⁻¹ := by
  obtain ⟨h₁,h₂,_,_,_⟩ := example_generic_coefficients K
  have hx := x_function_nonzero (translationExampleCurve K)
  rw [WeierstrassCurve.Affine.slope_of_X_ne hx]
  simp only [sub_zero,WeierstrassCurve.Affine.addX,h₁,h₂,zero_mul,add_zero,sub_zero]
  field_simp [hx]
  linear_combination translation_example_relation K

theorem example_add_y_formula (K : Type*) [Field K] :
    (genericCurve (translationExampleCurve K)).addY (xFunction (translationExampleCurve K)) 0
      (yFunction (translationExampleCurve K))
      ((genericCurve (translationExampleCurve K)).slope (xFunction (translationExampleCurve K)) 0
        (yFunction (translationExampleCurve K)) 0) =
      -yFunction (translationExampleCurve K)/(xFunction (translationExampleCurve K))^2 := by
  obtain ⟨h₁,_,h₃,_,_⟩ := example_generic_coefficients K
  have hx := x_function_nonzero (translationExampleCurve K)
  rw [WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negY,WeierstrassCurve.Affine.negAddY]
  simp only [example_add_x_formula,h₁,h₃,zero_mul,sub_zero]
  rw [WeierstrassCurve.Affine.slope_of_X_ne hx]
  simp only [sub_zero]
  field_simp [hx]
  ring

variable [DecidableEq K]

theorem example_constant_point :
    constantPoint (translationExampleCurve K) (translationExampleQ K)=
      Point.some 0 0 (example_generic_nonsingular_zero K) := by
  change Point.some (algebraMap K (translationExampleCurve K).FunctionField 0)
    (algebraMap K (translationExampleCurve K).FunctionField 0) _=Point.some 0 0 _
  simp only [map_zero]

variable [IsAlgClosed K]

/-- The actual constructed field automorphism agrees with the credited source rational formula. -/
theorem translation_example_field_formula (hΔ : (translationExampleCurve K).Δ ≠ 0) :
    fieldTranslation (translationExampleCurve K) hΔ (translationExampleQ K)
      (xFunction (translationExampleCurve K))=(xFunction (translationExampleCurve K))⁻¹ ∧
    fieldTranslation (translationExampleCurve K) hΔ (translationExampleQ K)
      (yFunction (translationExampleCurve K))=
      -yFunction (translationExampleCurve K)/(xFunction (translationExampleCurve K))^2 := by
  have hxy : ¬ (xFunction (translationExampleCurve K)=0 ∧
      yFunction (translationExampleCurve K)=(genericCurve (translationExampleCurve K)).negY 0 0) :=
    fun h => x_function_nonzero (translationExampleCurve K) h.1
  constructor
  · rw [field_translation_x,translatedX,translatedGeneric,genericPoint,example_constant_point,
      generic_add_some (translationExampleCurve K) (generic_nonsingular _ hΔ)
        (example_generic_nonsingular_zero K) hxy]
    exact example_add_x_formula K
  · rw [field_translation_y,translatedY,translatedGeneric,genericPoint,example_constant_point,
      generic_add_some (translationExampleCurve K) (generic_nonsingular _ hΔ)
        (example_generic_nonsingular_zero K) hxy]
    exact example_add_y_formula K

local instance : Fact (Nat.Prime 5) := ⟨by decide⟩
local instance : CommRing TranslationBase5 := (inferInstance : Field TranslationBase5).toCommRing
local instance : CommRing TranslationClosure5 := (inferInstance : Field TranslationClosure5).toCommRing
local instance : DecidableEq TranslationClosure5 := Classical.decEq _

theorem translation_example_actual_char5 : CharP TranslationClosure5 5 := inferInstance

/-- Concrete actual ACF5 translation fails to preserve the actual affine coordinate ring. -/
theorem translation_example_not_affine_coordinate :
    ¬ ∃ a : (translationExampleCurve TranslationClosure5).CoordinateRing,
      fieldTranslation (translationExampleCurve TranslationClosure5) translation_example_delta_closure
        (translationExampleQ TranslationClosure5) (xFunction (translationExampleCurve TranslationClosure5))=
      algebraMap (translationExampleCurve TranslationClosure5).CoordinateRing
        (translationExampleCurve TranslationClosure5).FunctionField a := by
  have hx := (translation_example_field_formula (K := TranslationClosure5)
    translation_example_delta_closure).1
  rw [hx]
  exact reciprocal_x_not_coordinate_image (translationExampleCurve TranslationClosure5)

#print axioms translation_example_field_formula
#print axioms translation_example_not_affine_coordinate
end
end EllipticTranslationAudit
