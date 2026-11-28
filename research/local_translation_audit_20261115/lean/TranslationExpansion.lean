module
public import TranslationCertificate

@[expose] public section
namespace LocalTranslationAudit
noncomputable section
open WeierstrassCurve.Affine
open EllipticTranslationAudit MillerNormValuationAudit MillerFinitePointAudit
open MillerBasechangeAudit
variable {K : Type*} [Field K]

def shiftX (W : WeierstrassCurve.Affine K) (b : K) : W.FunctionField :=
  xFunction W-constantHom W b
def shiftY (W : WeierstrassCurve.Affine K) (c : K) : W.FunctionField :=
  yFunction W-constantHom W c

theorem shift_x_nonzero (W : WeierstrassCurve.Affine K) (b : K) : shiftX W b≠0 :=
  sub_ne_zero.mpr (x_function_ne_constant W b)

/-- Actual shifted curve equation, with no restrictions on characteristic. -/
theorem shifted_field_relation (W : WeierstrassCurve.Affine K) {b c : K}
    (h : W.Equation b c) :
    shiftY W c^2+constantHom W (derivativeY W b c)*shiftY W c+
      constantHom W W.a₁*shiftX W b*shiftY W c =
    shiftX W b^3+constantHom W (3*b+W.a₂)*shiftX W b^2+
      constantHom W (negativeDerivativeX W b c)*shiftX W b := by
  have hq := congrArg (constantHom W) ((W.equation_iff b c).mp h)
  simp only [map_add,map_mul,map_pow] at hq
  simp only [shiftX,shiftY,derivativeY,negativeDerivativeX,map_add,map_sub,map_mul,
    map_pow,map_ofNat]
  linear_combination function_curve_relation W-hq

theorem generic_coefficients (W : WeierstrassCurve.Affine K) :
    (genericCurve W).a₁=constantHom W W.a₁ ∧
    (genericCurve W).a₂=constantHom W W.a₂ ∧
    (genericCurve W).a₃=constantHom W W.a₃ := ⟨rfl,rfl,rfl⟩

variable [DecidableEq K] [IsAlgClosed K]

def deltaX (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) {b c : K}
    (h : W.Nonsingular b c) : W.FunctionField :=
  fieldTranslation W hΔ (.some b c h) (xFunction W)-constantHom W b
def deltaY (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) {b c : K}
    (h : W.Nonsingular b c) : W.FunctionField :=
  fieldTranslation W hΔ (.some b c h) (yFunction W)-constantHom W c

omit [IsAlgClosed K] in
theorem translated_generic_coordinates (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    {b c : K} (h : W.Nonsingular b c) :
    translatedX W hΔ (.some b c h)=
      (genericCurve W).addX (xFunction W) (constantHom W b)
        ((genericCurve W).slope (xFunction W) (constantHom W b) (yFunction W) (constantHom W c)) ∧
    translatedY W hΔ (.some b c h)=
      (genericCurve W).addY (xFunction W) (constantHom W b) (yFunction W)
        ((genericCurve W).slope (xFunction W) (constantHom W b) (yFunction W) (constantHom W c)) := by
  have hq : (genericCurve W).Nonsingular (constantHom W b) (constantHom W c) := by
    apply (equation_iff_nonsingular_of_Δ_ne_zero (generic_curve_delta W hΔ)).mp
    rw [(genericCurve W).equation_iff]
    have he := congrArg (constantHom W) ((W.equation_iff b c).mp h.1)
    simpa only [genericCurve,extendedCurve,WeierstrassCurve.map,function_coefficient_map,
      map_add,map_mul,map_pow] using he
  have hxy : ¬ (xFunction W=constantHom W b ∧
      yFunction W=(genericCurve W).negY (constantHom W b) (constantHom W c)) :=
    fun he => x_function_ne_constant W b he.1
  have hc : constantPoint W (.some b c h)=
      Point.some (constantHom W b) (constantHom W c) hq := rfl
  constructor
  · rw [translatedX,translatedGeneric,EllipticTranslationAudit.genericPoint,hc,generic_add_some W (generic_nonsingular W hΔ) hq hxy]
    rfl
  · rw [translatedY,translatedGeneric,EllipticTranslationAudit.genericPoint,hc,generic_add_some W (generic_nonsingular W hΔ) hq hxy]
    rfl

/-- Actual translated abscissa, rearranged to expose its infinity behavior. -/
theorem delta_x_formula (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    {b c : K} (h : W.Nonsingular b c) :
    deltaX W hΔ h=(constantHom W (negativeDerivativeX W b c)*shiftX W b-
      constantHom W (derivativeY W b c)*shiftY W c)/shiftX W b^2 := by
  rw [deltaX,field_translation_x,(translated_generic_coordinates W hΔ h).1,
    WeierstrassCurve.Affine.slope_of_X_ne (x_function_ne_constant W b),
    WeierstrassCurve.Affine.addX]
  obtain ⟨h₁,h₂,_⟩ := generic_coefficients W
  rw [h₁,h₂]
  have hq := congrArg (constantHom W) ((W.equation_iff b c).mp h.1)
  simp only [map_add,map_mul,map_pow] at hq
  simp only [shiftX,shiftY,derivativeY,negativeDerivativeX,map_add,map_sub,map_mul,
    map_pow,map_ofNat]
  field_simp [sub_ne_zero.mpr (x_function_ne_constant W b)]
  linear_combination function_curve_relation W-hq

theorem delta_y_formula (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    {b c : K} (h : W.Nonsingular b c) :
    deltaY W hΔ h=-(shiftY W c/shiftX W b+constantHom W W.a₁)*deltaX W hΔ h-
      constantHom W (derivativeY W b c) := by
  rw [deltaY,deltaX,field_translation_x,field_translation_y,
    (translated_generic_coordinates W hΔ h).1,(translated_generic_coordinates W hΔ h).2,
    WeierstrassCurve.Affine.slope_of_X_ne (x_function_ne_constant W b),
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negY,
    WeierstrassCurve.Affine.negAddY]
  obtain ⟨h₁,_,h₃⟩ := generic_coefficients W
  rw [h₁,h₃]
  simp only [shiftX,shiftY,derivativeY,map_add,map_mul,map_ofNat]
  field_simp [sub_ne_zero.mpr (x_function_ne_constant W b)]
  ring

#print axioms shifted_field_relation
#print axioms delta_x_formula
#print axioms delta_y_formula
end
end LocalTranslationAudit
