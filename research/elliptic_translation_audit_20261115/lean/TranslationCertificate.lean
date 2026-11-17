module
public import TranslationExampleField

@[expose] public section
namespace EllipticTranslationAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerNormValuationAudit FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit
open EllipticPicardBridgeAudit
local instance : Fact (Nat.Prime 5) := ⟨by decide⟩
local instance : CommRing TranslationBase5 := (inferInstance : Field TranslationBase5).toCommRing
local instance : CommRing TranslationClosure5 := (inferInstance : Field TranslationClosure5).toCommRing
local instance : DecidableEq TranslationClosure5 := Classical.decEq _

/-- Actual field and point-divisor translations, with a concrete affine-ring and domain counter.
All-local-order compatibility between the two actions is deliberately not a premise or conclusion. -/
theorem actual_elliptic_translation_certificate :
    (∀ (K : Type) [Field K] [DecidableEq K] [IsAlgClosed K]
      (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0),
      (∀ Q : W.Point, ∀ c : K,
        fieldTranslationEquiv W hΔ Q (constantHom W c)=constantHom W c) ∧
      (∀ (Q : W.Point) (f : W.FunctionField),
        fieldTranslationEquiv W hΔ Q f=fieldTranslation W hΔ Q f) ∧
      (∀ Q R : W.Point, (fieldTranslation W hΔ Q).comp (fieldTranslation W hΔ R)=
        fieldTranslation W hΔ (Q+R)) ∧
      fieldTranslation W hΔ 0=RingHom.id W.FunctionField ∧
      (∀ (Q : W.Point) (f : W.FunctionField),
        (fieldTranslationEquiv W hΔ Q).symm f=fieldTranslation W hΔ (-Q) f) ∧
      Function.Injective (fieldTranslation W hΔ) ∧
      (∀ Q : W.Point, coefficientPointMap W (fieldTranslationAlgHom W hΔ Q) (genericPoint W hΔ)=
        translatedGeneric W hΔ Q) ∧
      (∀ (Q : W.Point) (D : W.Point →₀ ℤ),
        divisorDegree W (divisorTranslation W Q D)=divisorDegree W D ∧
        divisorPointSum W (divisorTranslation W Q D)=divisorPointSum W D+divisorDegree W D • Q ∧
        (divisorTranslation W Q D ∈ principalPointDivisors W ↔ D ∈ principalPointDivisors W)) ∧
      (∀ Q : W.Point, picardTranslation W hΔ Q=AddMonoidHom.id (PicardZero W hΔ))) ∧
    CharP TranslationClosure5 5 ∧
    (translationExampleCurve TranslationClosure5).Δ ≠ 0 ∧
    (¬ ∃ a : (translationExampleCurve TranslationClosure5).CoordinateRing,
      fieldTranslation (translationExampleCurve TranslationClosure5) translation_example_delta_closure
        (translationExampleQ TranslationClosure5) (xFunction (translationExampleCurve TranslationClosure5))=
      algebraMap (translationExampleCurve TranslationClosure5).CoordinateRing
        (translationExampleCurve TranslationClosure5).FunctionField a) ∧
    rawExampleAtQ ≠ translationExampleQ TranslationBase5+translationExampleQ TranslationBase5 ∧
    (2 : TranslationBase5) ≠ 0 ∧
    translationExampleP5+translationExampleQ TranslationBase5=translationExampleS5 ∧
    (((2 : TranslationBase5)⁻¹),-(0 : TranslationBase5)/(2 : TranslationBase5)^2)=(3,0) := by
  refine ⟨?_,translation_example_actual_char5,translation_example_delta_closure,
    translation_example_not_affine_coordinate,raw_example_at_q_wrong,
    translation_example_guarded_positive⟩
  intro K _ _ _ W hΔ
  refine ⟨?_,field_translation_equiv_apply W hΔ,field_translation_comp W hΔ,
    field_translation_zero W hΔ,field_translation_equiv_symm_apply W hΔ,
    field_translation_faithful W hΔ,field_point_generic W hΔ,?_,picard_translation_identity W hΔ⟩
  · intro Q c
    rw [field_translation_equiv_apply,field_translation_constant]
  · intro Q D
    exact ⟨divisor_translation_degree W Q D,divisor_translation_point_sum W Q D,
      divisor_translation_principal_iff W hΔ Q D⟩

#print axioms actual_elliptic_translation_certificate
end
end EllipticTranslationAudit
