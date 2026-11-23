module
public import GeneralAction

@[expose] public section
namespace GeneralTranslationAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine WeierstrassCurve.Affine.CoordinateRing
open EllipticTranslationAudit LocalTranslationAudit
open MillerNormValuationAudit MillerInfinityChartAudit MillerBasechangeAudit
variable {K : Type*} [Field K]

/-- Actual coefficient/x/y/basis/fraction generation with an arbitrary target field. -/
theorem general_function_hom_ext {S : Type*} [Field S] (W : WeierstrassCurve.Affine K)
    {φ ψ : W.FunctionField →+* S}
    (hC : ∀ c : K, φ (constantHom W c)=ψ (constantHom W c))
    (hX : φ (xFunction W)=ψ (xFunction W))
    (hY : φ (yFunction W)=ψ (yFunction W)) : φ=ψ := by
  have hp (p : K[X]) : φ (p.eval₂ (constantHom W) (xFunction W))=
      ψ (p.eval₂ (constantHom W) (xFunction W)) := by
    induction p using Polynomial.induction_on' with
    | add p q ihp ihq => rw [eval₂_add,map_add,map_add,ihp,ihq]
    | monomial n c => rw [eval₂_monomial,map_mul,map_mul,map_pow,map_pow,hC,hX]
  have hy : algebraMap W.CoordinateRing W.FunctionField (mk W X)=yFunction W := by
    simp only [yFunction,YClass,map_zero,sub_zero]
  have hc (a : W.CoordinateRing) :
      φ (algebraMap W.CoordinateRing W.FunctionField a)=
      ψ (algebraMap W.CoordinateRing W.FunctionField a) := by
    obtain ⟨p,q,rfl⟩ := exists_smul_basis_eq a
    simp only [smul,map_add,map_mul,mul_one,polynomial_function_image,hy]
    rw [hp p,hp q,hY]
  apply RingHom.ext
  intro f
  obtain ⟨a,b,_hb,hab⟩ := IsFractionRing.div_surjective W.CoordinateRing f
  rw [← hab,map_div₀,map_div₀,hc a,hc b]

def genericSlope (W : WeierstrassCurve.Affine K) (b c : K) : W.FunctionField :=
  shiftY W c/shiftX W b

variable [DecidableEq K]

theorem generic_translated_x_formula (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    {b c : K} (h : W.Nonsingular b c) :
    translatedX W hΔ (.some b c h)=genericSlope W b c^2+
      constantHom W W.a₁*genericSlope W b c-constantHom W W.a₂-xFunction W-constantHom W b := by
  rw [(translated_generic_coordinates W hΔ h).1,
    WeierstrassCurve.Affine.slope_of_X_ne (x_function_ne_constant W b),WeierstrassCurve.Affine.addX]
  obtain ⟨h₁,h₂,_⟩ := generic_coefficients W
  rw [h₁,h₂]
  rfl

theorem generic_translated_y_formula (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    {b c : K} (h : W.Nonsingular b c) :
    translatedY W hΔ (.some b c h)=
      -(genericSlope W b c*(translatedX W hΔ (.some b c h)-xFunction W)+yFunction W)-
      constantHom W W.a₁*translatedX W hΔ (.some b c h)-constantHom W W.a₃ := by
  obtain ⟨hx,hy⟩ := translated_generic_coordinates W hΔ h
  rw [hy,WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negY,
    WeierstrassCurve.Affine.negAddY,← hx,
    WeierstrassCurve.Affine.slope_of_X_ne (x_function_ne_constant W b)]
  obtain ⟨h₁,_,h₃⟩ := generic_coefficients W
  rw [h₁,h₃]
  rfl

variable {L : Type*} [Field L] [Algebra K L] [DecidableEq L]

omit [DecidableEq K] [DecidableEq L] in
theorem generic_slope_basechange (W : WeierstrassCurve.Affine K) (b c : K) :
    fieldBaseChange (L := L) W (genericSlope W b c)=
      genericSlope (extendedCurve (L := L) W) (algebraMap K L b) (algebraMap K L c) := by
  simp only [genericSlope,shiftX,shiftY,map_div₀,map_sub,field_base_x,field_base_y,field_base_constant]

theorem generic_translated_x_basechange (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    {b c : K} (h : W.Nonsingular b c) :
    fieldBaseChange (L := L) W (translatedX W hΔ (.some b c h))=
      translatedX (extendedCurve (L := L) W) (extended_delta_nonzero W hΔ)
        (.some (algebraMap K L b) (algebraMap K L c) (base_nonsingular W h)) := by
  rw [generic_translated_x_formula,generic_translated_x_formula]
  simp only [map_sub,map_add,map_mul,map_pow,field_base_x,field_base_constant,generic_slope_basechange]
  rfl

theorem generic_translated_y_basechange (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    {b c : K} (h : W.Nonsingular b c) :
    fieldBaseChange (L := L) W (translatedY W hΔ (.some b c h))=
      translatedY (extendedCurve (L := L) W) (extended_delta_nonzero W hΔ)
        (.some (algebraMap K L b) (algebraMap K L c) (base_nonsingular W h)) := by
  rw [generic_translated_y_formula,generic_translated_y_formula]
  simp only [map_sub,map_neg,map_add,map_mul,field_base_x,field_base_y,field_base_constant,
    generic_slope_basechange,generic_translated_x_basechange]
  rfl

/-- Actual naturality square under arbitrary coefficient-field extension. -/
theorem general_translation_basechange (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (Q : W.Point) :
    (fieldBaseChange (L := L) W).comp (generalTranslation W hΔ Q)=
      (generalTranslation (extendedCurve (L := L) W) (extended_delta_nonzero W hΔ)
        (pointBaseChange (L := L) W Q)).comp (fieldBaseChange (L := L) W) := by
  cases Q with
  | zero =>
    have hq : pointBaseChange (L := L) W Point.zero=
        (Point.zero : (extendedCurve (L := L) W).Point) := rfl
    have hs := general_translation_zero W hΔ
    have ht := general_translation_zero (extendedCurve (L := L) W) (extended_delta_nonzero W hΔ)
    rw [Point.zero_def] at hs ht
    rw [hq,hs,ht]
    rfl
  | some b c h =>
    rw [point_base_some]
    apply general_function_hom_ext W
    · intro d
      simp only [RingHom.comp_apply,general_translation_constant,field_base_constant]
    · simp only [RingHom.comp_apply,general_translation_x,field_base_x]
      exact generic_translated_x_basechange W hΔ h
    · simp only [RingHom.comp_apply,general_translation_y,field_base_y]
      exact generic_translated_y_basechange W hΔ h

#print axioms generic_translated_x_basechange
#print axioms general_translation_basechange
end
end GeneralTranslationAudit
