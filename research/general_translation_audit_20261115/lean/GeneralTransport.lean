module
public import GeneralRegularBasechange

@[expose] public section
namespace GeneralTranslationAudit
noncomputable section
open WeierstrassCurve.Affine
open EllipticTranslationAudit MillerFinitePointAudit FinitePointEvaluationAudit
open FunctionDivisorFinitenessAudit
variable {K : Type*} [Field K] [DecidableEq K]

theorem general_regular_injective (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (P Q : W.Point) : Function.Injective (generalRegularTranslation W hΔ P Q) := by
  intro a b he
  exact Subtype.ext ((generalTranslation W hΔ Q).injective (congrArg Subtype.val he))

theorem general_regular_surjective (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (P Q : W.Point) : Function.Surjective (generalRegularTranslation W hΔ P Q) := by
  intro r
  have hv : localFunctionValuation W (P+Q) (generalTranslation W hΔ (-Q) r)≤1 := by
    rw [general_translation_valuation,add_assoc,add_neg_cancel,add_zero]
    exact r.property
  exact ⟨⟨generalTranslation W hΔ (-Q) r,hv⟩,
    Subtype.ext (general_translation_right_inverse W hΔ Q r)⟩

def generalRegularEquiv (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (P Q : W.Point) :
    RegularRing W (P+Q) ≃+* RegularRing W P :=
  RingEquiv.ofBijective (generalRegularTranslation W hΔ P Q)
    ⟨general_regular_injective W hΔ P Q,general_regular_surjective W hΔ P Q⟩

theorem general_regular_equiv_symm (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (P Q : W.Point) (r : RegularRing W P) :
    ((generalRegularEquiv W hΔ P Q).symm r : W.FunctionField)=generalTranslation W hΔ (-Q) r := by
  apply (generalTranslation W hΔ Q).injective
  have he := congrArg Subtype.val ((generalRegularEquiv W hΔ P Q).apply_symm_apply r)
  change generalTranslation W hΔ Q ((generalRegularEquiv W hΔ P Q).symm r : W.FunctionField)=
    (r : W.FunctionField) at he
  rw [he,general_translation_right_inverse]

theorem general_translation_nonzero (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (Q : W.Point) {f : W.FunctionField} (hf : f≠0) : generalTranslation W hΔ Q f≠0 := by
  intro hz
  apply hf
  apply (generalTranslation W hΔ Q).injective
  simpa only [map_zero] using hz

/-- Actual K-rational point-divisor transport. No geometric degree-zero assertion is made
when K is not algebraically closed. -/
theorem general_translation_function_divisor (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (Q : W.Point) {f : W.FunctionField} (hf : f≠0) :
    functionDivisor W (generalTranslation W hΔ Q f) (general_translation_nonzero W hΔ Q hf)=
      divisorTranslation W (-Q) (functionDivisor W f hf) := by
  ext P
  rw [function_divisor_apply,divisor_translation_apply,sub_neg_eq_add,
    function_divisor_apply,general_translation_order]

def generalTranslatedUnit (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (Q : W.Point) :
    W.FunctionFieldˣ →* W.FunctionFieldˣ := Units.map (generalTranslation W hΔ Q).toMonoidHom

theorem general_translated_unit_coe (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (Q : W.Point) (f : W.FunctionFieldˣ) :
    (generalTranslatedUnit W hΔ Q f : W.FunctionField)=generalTranslation W hΔ Q f := rfl

theorem general_translation_local_unit_iff (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (P Q : W.Point) (f : W.FunctionFieldˣ) :
    generalTranslatedUnit W hΔ Q f ∈ (RegularRing W P).unitGroup ↔
      f ∈ (RegularRing W (P+Q)).unitGroup := by
  rw [mem_point_units_iff,mem_point_units_iff,general_translated_unit_coe,general_translation_order]

def generalPointUnitTranslation (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (P Q : W.Point) :
    (RegularRing W (P+Q)).unitGroup →* (RegularRing W P).unitGroup where
  toFun f := ⟨generalTranslatedUnit W hΔ Q f.val,
    (general_translation_local_unit_iff W hΔ P Q f.val).mpr f.property⟩
  map_one' := Subtype.ext (map_one _)
  map_mul' a b := Subtype.ext (map_mul (generalTranslatedUnit W hΔ Q)
    (a : W.FunctionFieldˣ) (b : W.FunctionFieldˣ))

theorem general_translation_point_unit_evaluation (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (P Q : W.Point) (f : (RegularRing W (P+Q)).unitGroup) :
    pointUnitEvaluation W P (generalPointUnitTranslation W hΔ P Q f)=pointUnitEvaluation W (P+Q) f := by
  apply Units.ext
  have he : ((RegularRing W P).unitGroupMulEquiv (generalPointUnitTranslation W hΔ P Q f) :
      RegularRing W P)=generalRegularTranslation W hΔ P Q
        ((RegularRing W (P+Q)).unitGroupMulEquiv f : RegularRing W (P+Q)) := Subtype.ext rfl
  change regularEvaluation W P
    ((RegularRing W P).unitGroupMulEquiv (generalPointUnitTranslation W hΔ P Q f) : RegularRing W P)=
    regularEvaluation W (P+Q) ((RegularRing W (P+Q)).unitGroupMulEquiv f : RegularRing W (P+Q))
  rw [he,general_translation_residue]

#print axioms generalRegularEquiv
#print axioms general_translation_function_divisor
#print axioms general_translation_point_unit_evaluation
end
end GeneralTranslationAudit
