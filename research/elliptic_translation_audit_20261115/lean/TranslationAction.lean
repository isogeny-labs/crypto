module
public import TranslationNaturality

@[expose] public section
namespace EllipticTranslationAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerNormValuationAudit MillerBasechangeAudit
variable {K : Type*} [Field K] [DecidableEq K] [IsAlgClosed K]

/-- Actual function-field composition, derived from actual generic-point addition and generation. -/
theorem field_translation_comp (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0)
    (Q R : W.Point) : (fieldTranslation W hΔ Q).comp (fieldTranslation W hΔ R)=
      fieldTranslation W hΔ (Q+R) := by
  apply function_hom_ext W
  · intro c
    simp only [RingHom.comp_apply,field_translation_constant]
  · simp only [RingHom.comp_apply,field_translation_x]
    change fieldTranslationAlgHom W hΔ Q (genericX W (translatedGeneric W hΔ R))=
      genericX W (translatedGeneric W hΔ (Q+R))
    rw [← generic_x_map,field_point_translated]
  · simp only [RingHom.comp_apply,field_translation_y]
    change fieldTranslationAlgHom W hΔ Q (genericY W (translatedGeneric W hΔ R))=
      genericY W (translatedGeneric W hΔ (Q+R))
    rw [← generic_y_map,field_point_translated]

theorem field_translation_zero (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0) :
    fieldTranslation W hΔ 0=RingHom.id W.FunctionField := by
  apply function_hom_ext W
  · intro c
    simp only [field_translation_constant,RingHom.id_apply]
  · rw [RingHom.id_apply,field_translation_x]
    change genericX W (genericPoint W hΔ+constantPoint W 0)=xFunction W
    rw [map_zero,add_zero]
    rfl
  · rw [RingHom.id_apply,field_translation_y]
    change genericY W (genericPoint W hΔ+constantPoint W 0)=yFunction W
    rw [map_zero,add_zero]
    rfl

theorem field_translation_left_inverse (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0)
    (Q : W.Point) : Function.LeftInverse (fieldTranslation W hΔ (-Q)) (fieldTranslation W hΔ Q) := by
  intro f
  have he := RingHom.congr_fun (field_translation_comp W hΔ (-Q) Q) f
  rw [neg_add_cancel,field_translation_zero] at he
  simpa only [RingHom.comp_apply,RingHom.id_apply] using he

theorem field_translation_right_inverse (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0)
    (Q : W.Point) : Function.RightInverse (fieldTranslation W hΔ (-Q)) (fieldTranslation W hΔ Q) := by
  intro f
  have he := RingHom.congr_fun (field_translation_comp W hΔ Q (-Q)) f
  rw [add_neg_cancel,field_translation_zero] at he
  simpa only [RingHom.comp_apply,RingHom.id_apply] using he

/-- Actual coefficient-fixing function-field automorphism; actual inverse is translation by -Q. -/
def fieldTranslationEquiv (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0) (Q : W.Point) :
    W.FunctionField ≃+* W.FunctionField :=
  RingEquiv.ofBijective (fieldTranslation W hΔ Q)
    ⟨(fieldTranslation W hΔ Q).injective,(field_translation_right_inverse W hΔ Q).surjective⟩

theorem field_translation_equiv_apply (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0)
    (Q : W.Point) (f : W.FunctionField) :
    fieldTranslationEquiv W hΔ Q f=fieldTranslation W hΔ Q f := rfl

theorem field_translation_equiv_symm_apply (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0)
    (Q : W.Point) (f : W.FunctionField) :
    (fieldTranslationEquiv W hΔ Q).symm f=fieldTranslation W hΔ (-Q) f := by
  apply (fieldTranslationEquiv W hΔ Q).injective
  rw [RingEquiv.apply_symm_apply]
  exact (field_translation_right_inverse W hΔ Q f).symm

/-- Distinct actual constant translations induce distinct actual function-field maps. -/
theorem field_translation_faithful (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0) :
    Function.Injective (fieldTranslation W hΔ) := by
  intro Q R hQR
  have hx : translatedX W hΔ Q=translatedX W hΔ R :=
    (field_translation_x W hΔ Q).symm.trans
      ((RingHom.congr_fun hQR (xFunction W)).trans (field_translation_x W hΔ R))
  have hy : translatedY W hΔ Q=translatedY W hΔ R :=
    (field_translation_y W hΔ Q).symm.trans
      ((RingHom.congr_fun hQR (yFunction W)).trans (field_translation_y W hΔ R))
  have hp := generic_points_eq_of_coordinates W
    (translated_generic_nonzero W hΔ Q) (translated_generic_nonzero W hΔ R) hx hy
  change genericPoint W hΔ+constantPoint W Q=genericPoint W hΔ+constantPoint W R at hp
  have hc : constantPoint W Q=constantPoint W R := add_left_cancel hp
  exact point_base_injective (L := W.FunctionField) W hc

#print axioms field_translation_comp
#print axioms fieldTranslationEquiv
#print axioms field_translation_faithful
end
end EllipticTranslationAudit
