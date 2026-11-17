module
public import GeneralField

@[expose] public section
namespace GeneralTranslationAudit
noncomputable section
open WeierstrassCurve.Affine
open EllipticTranslationAudit MillerNormValuationAudit MillerBasechangeAudit
variable {K : Type*} [Field K] [DecidableEq K]

theorem general_point_generic (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (Q : W.Point) :
    coefficientPointMap W (generalTranslationAlgHom W hΔ Q) (EllipticTranslationAudit.genericPoint W hΔ)=
      translatedGeneric W hΔ Q := by
  apply generic_points_eq_of_coordinates W
  · intro hz
    apply Point.some_ne_zero (generic_nonsingular W hΔ)
    change EllipticTranslationAudit.genericPoint W hΔ=0
    apply coefficient_point_map_injective W (generalTranslationAlgHom W hΔ Q)
    simpa only [map_zero] using hz
  · exact translated_generic_nonzero W hΔ Q
  · rw [generic_x_map]
    exact general_translation_x W hΔ Q
  · rw [generic_y_map]
    exact general_translation_y W hΔ Q

theorem general_point_constant (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (Q R : W.Point) :
    coefficientPointMap W (generalTranslationAlgHom W hΔ Q) (constantPoint W R)=constantPoint W R :=
  Point.map_baseChange (W' := W) (generalTranslationAlgHom W hΔ Q) R

theorem general_point_translated (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (Q R : W.Point) :
    coefficientPointMap W (generalTranslationAlgHom W hΔ Q) (translatedGeneric W hΔ R)=
      translatedGeneric W hΔ (Q+R) := by
  rw [translatedGeneric,map_add,general_point_generic,general_point_constant]
  simp only [translatedGeneric,map_add]
  abel

/-- Actual composition of coefficient-fixing function-field maps over every field. -/
theorem general_translation_comp (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (Q R : W.Point) : (generalTranslation W hΔ Q).comp (generalTranslation W hΔ R)=
      generalTranslation W hΔ (Q+R) := by
  apply function_hom_ext W
  · intro c
    simp only [RingHom.comp_apply,general_translation_constant]
  · simp only [RingHom.comp_apply,general_translation_x]
    change generalTranslationAlgHom W hΔ Q (genericX W (translatedGeneric W hΔ R))=
      genericX W (translatedGeneric W hΔ (Q+R))
    rw [← generic_x_map,general_point_translated]
  · simp only [RingHom.comp_apply,general_translation_y]
    change generalTranslationAlgHom W hΔ Q (genericY W (translatedGeneric W hΔ R))=
      genericY W (translatedGeneric W hΔ (Q+R))
    rw [← generic_y_map,general_point_translated]

theorem general_translation_zero (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) :
    generalTranslation W hΔ 0=RingHom.id W.FunctionField := by
  apply function_hom_ext W
  · intro c
    simp only [general_translation_constant,RingHom.id_apply]
  · rw [RingHom.id_apply,general_translation_x]
    change genericX W (EllipticTranslationAudit.genericPoint W hΔ+constantPoint W 0)=xFunction W
    rw [map_zero,add_zero]
    rfl
  · rw [RingHom.id_apply,general_translation_y]
    change genericY W (EllipticTranslationAudit.genericPoint W hΔ+constantPoint W 0)=yFunction W
    rw [map_zero,add_zero]
    rfl

theorem general_translation_left_inverse (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (Q : W.Point) : Function.LeftInverse (generalTranslation W hΔ (-Q)) (generalTranslation W hΔ Q) := by
  intro f
  have he := RingHom.congr_fun (general_translation_comp W hΔ (-Q) Q) f
  rw [neg_add_cancel,general_translation_zero] at he
  simpa only [RingHom.comp_apply,RingHom.id_apply] using he

theorem general_translation_right_inverse (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (Q : W.Point) : Function.RightInverse (generalTranslation W hΔ (-Q)) (generalTranslation W hΔ Q) := by
  intro f
  have he := RingHom.congr_fun (general_translation_comp W hΔ Q (-Q)) f
  rw [add_neg_cancel,general_translation_zero] at he
  simpa only [RingHom.comp_apply,RingHom.id_apply] using he

def generalTranslationEquiv (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (Q : W.Point) :
    W.FunctionField ≃+* W.FunctionField :=
  RingEquiv.ofBijective (generalTranslation W hΔ Q)
    ⟨(generalTranslation W hΔ Q).injective,(general_translation_right_inverse W hΔ Q).surjective⟩

theorem general_translation_equiv_apply (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (Q : W.Point) (f : W.FunctionField) :
    generalTranslationEquiv W hΔ Q f=generalTranslation W hΔ Q f := rfl

theorem general_translation_equiv_symm (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (Q : W.Point) (f : W.FunctionField) :
    (generalTranslationEquiv W hΔ Q).symm f=generalTranslation W hΔ (-Q) f := by
  apply (generalTranslationEquiv W hΔ Q).injective
  rw [RingEquiv.apply_symm_apply]
  exact (general_translation_right_inverse W hΔ Q f).symm

theorem general_translation_faithful (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) :
    Function.Injective (generalTranslation W hΔ) := by
  intro Q R hQR
  have hx : translatedX W hΔ Q=translatedX W hΔ R :=
    (general_translation_x W hΔ Q).symm.trans
      ((RingHom.congr_fun hQR (xFunction W)).trans (general_translation_x W hΔ R))
  have hy : translatedY W hΔ Q=translatedY W hΔ R :=
    (general_translation_y W hΔ Q).symm.trans
      ((RingHom.congr_fun hQR (yFunction W)).trans (general_translation_y W hΔ R))
  have hp := generic_points_eq_of_coordinates W
    (translated_generic_nonzero W hΔ Q) (translated_generic_nonzero W hΔ R) hx hy
  change EllipticTranslationAudit.genericPoint W hΔ+constantPoint W Q=
    EllipticTranslationAudit.genericPoint W hΔ+constantPoint W R at hp
  exact point_base_injective (L := W.FunctionField) W (add_left_cancel hp)

#print axioms generalTranslationEquiv
#print axioms general_translation_comp
#print axioms general_translation_faithful
end
end GeneralTranslationAudit
