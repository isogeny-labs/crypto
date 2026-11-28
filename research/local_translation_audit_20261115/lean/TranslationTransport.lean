module
public import TranslationValuation

@[expose] public section
namespace LocalTranslationAudit
noncomputable section
open WeierstrassCurve.Affine
open EllipticTranslationAudit MillerFinitePointAudit FinitePointEvaluationAudit
open FunctionDivisorFinitenessAudit
variable {K : Type*} [Field K] [DecidableEq K] [IsAlgClosed K]

theorem regular_translation_injective (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (P Q : W.Point) : Function.Injective (regularTranslation W hΔ P Q) := by
  intro a b he
  apply Subtype.ext
  exact (fieldTranslation W hΔ Q).injective (congrArg Subtype.val he)

theorem regular_translation_surjective (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (P Q : W.Point) : Function.Surjective (regularTranslation W hΔ P Q) := by
  intro r
  have hv : localFunctionValuation W (P+Q) (fieldTranslation W hΔ (-Q) r)≤1 := by
    rw [translation_valuation,add_assoc,add_neg_cancel,add_zero]
    exact r.property
  exact ⟨⟨fieldTranslation W hΔ (-Q) r,hv⟩,
    Subtype.ext (field_translation_right_inverse W hΔ Q r)⟩

/-- Actual isomorphism of actual regular rings, with bijectivity derived. -/
def regularTranslationEquiv (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (P Q : W.Point) :
    RegularRing W (P+Q) ≃+* RegularRing W P :=
  RingEquiv.ofBijective (regularTranslation W hΔ P Q)
    ⟨regular_translation_injective W hΔ P Q,regular_translation_surjective W hΔ P Q⟩

theorem regular_translation_equiv_symm (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (P Q : W.Point) (r : RegularRing W P) :
    ((regularTranslationEquiv W hΔ P Q).symm r : W.FunctionField)=
      fieldTranslation W hΔ (-Q) r := by
  apply (fieldTranslation W hΔ Q).injective
  have he := congrArg Subtype.val ((regularTranslationEquiv W hΔ P Q).apply_symm_apply r)
  change fieldTranslation W hΔ Q ((regularTranslationEquiv W hΔ P Q).symm r : W.FunctionField)=
    (r : W.FunctionField) at he
  rw [he,field_translation_right_inverse]

theorem translation_nonzero (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (Q : W.Point) {f : W.FunctionField} (hf : f≠0) : fieldTranslation W hΔ Q f≠0 := by
  intro hz
  apply hf
  apply (fieldTranslation W hΔ Q).injective
  simpa only [map_zero] using hz

/-- Point pushforward by negative Q is the actual principal divisor of field pullback by Q. -/
theorem translation_function_divisor (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (Q : W.Point) {f : W.FunctionField} (hf : f≠0) :
    functionDivisor W (fieldTranslation W hΔ Q f) (translation_nonzero W hΔ Q hf)=
      divisorTranslation W (-Q) (functionDivisor W f hf) := by
  ext P
  rw [function_divisor_apply,divisor_translation_apply,sub_neg_eq_add,
    function_divisor_apply,translation_point_order]

def translatedUnit (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (Q : W.Point) :
    W.FunctionFieldˣ →* W.FunctionFieldˣ := Units.map (fieldTranslation W hΔ Q).toMonoidHom

theorem translated_unit_coe (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (Q : W.Point) (f : W.FunctionFieldˣ) :
    (translatedUnit W hΔ Q f : W.FunctionField)=fieldTranslation W hΔ Q f := rfl

theorem translation_local_unit_iff (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (P Q : W.Point) (f : W.FunctionFieldˣ) :
    translatedUnit W hΔ Q f ∈ (RegularRing W P).unitGroup ↔
      f ∈ (RegularRing W (P+Q)).unitGroup := by
  rw [mem_point_units_iff,mem_point_units_iff,translated_unit_coe,translation_point_order]

def pointUnitTranslation (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (P Q : W.Point) :
    (RegularRing W (P+Q)).unitGroup →* (RegularRing W P).unitGroup where
  toFun f := ⟨translatedUnit W hΔ Q f.val,(translation_local_unit_iff W hΔ P Q f.val).mpr f.property⟩
  map_one' := Subtype.ext (map_one _)
  map_mul' a b := Subtype.ext (map_mul (translatedUnit W hΔ Q)
    (a : W.FunctionFieldˣ) (b : W.FunctionFieldˣ))

theorem translation_point_unit_evaluation (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (P Q : W.Point) (f : (RegularRing W (P+Q)).unitGroup) :
    pointUnitEvaluation W P (pointUnitTranslation W hΔ P Q f)=pointUnitEvaluation W (P+Q) f := by
  apply Units.ext
  have he : ((RegularRing W P).unitGroupMulEquiv (pointUnitTranslation W hΔ P Q f) :
      RegularRing W P)=regularTranslation W hΔ P Q
        ((RegularRing W (P+Q)).unitGroupMulEquiv f : RegularRing W (P+Q)) := Subtype.ext rfl
  change regularEvaluation W P
    ((RegularRing W P).unitGroupMulEquiv (pointUnitTranslation W hΔ P Q f) : RegularRing W P)=
    regularEvaluation W (P+Q) ((RegularRing W (P+Q)).unitGroupMulEquiv f : RegularRing W (P+Q))
  rw [he,translation_residue]

#print axioms regularTranslationEquiv
#print axioms translation_function_divisor
#print axioms translation_point_unit_evaluation
end
end LocalTranslationAudit
