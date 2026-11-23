import InfinityControls

namespace EllipticInfinityNormProjectionAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerNormValuationAudit MillerInfinityChartAudit MillerFinitePointAudit MillerRecursionAudit
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit NegationPullbackAudit
open FinitePointEvaluationAudit GeneralTranslationAudit WeilAuxiliaryRatioAudit
open EllipticTameSymbolAudit EllipticFieldNormAudit EllipticNormTransportAudit
open scoped EllipticFieldNormAudit WithZero
attribute [local instance] normFunctionCommRing

/-- Exact actual infinity norm/tame projection, actual base residue identification and
actual inertia character, with canonical-model independence and concrete sign controls. -/
theorem actual_elliptic_infinity_norm_projection_certificate
    (K : Type*) [Field K] (W : WeierstrassCurve.Affine K) :
    (∀ c : RatFunc K,functionValuation W (algebraMap (RatFunc K) W.FunctionField c)=
      (baseInfinityValuation K c)^2) ∧
    (RingHom.ker (baseEvaluation W)=IsLocalRing.maximalIdeal (BaseInfinityRing K) ∧
      Function.Surjective (baseEvaluation W) ∧ ∀ c : K,baseEvaluation W (baseConstant c)=c) ∧
    (∀ W' : WeierstrassCurve.Affine K,∀ r : BaseInfinityRing K,baseEvaluation W r=baseEvaluation W' r) ∧
    (∀ u : (BaseInfinityRing K).unitGroup,
      baseUnitEvaluation W u=pointUnitEvaluation W 0 (basePointUnit W u)) ∧
    (∀ W' : WeierstrassCurve.Affine K,∀ a b : (RatFunc K)ˣ,baseTameSymbol W a b=baseTameSymbol W' a b) ∧
    (inertiaCharacter W (infinityParameterUnit W)= -1 ∧
      ∀ f : W.FunctionFieldˣ,inertiaCharacter W f=(-1 : Kˣ)^pointOrder W f 0) ∧
    (∀ f : W.FunctionFieldˣ,∀ c : (RatFunc K)ˣ,
      baseTameSymbol W (normUnit W f) c=tameSymbol W 0 f (baseFieldUnit W c) ∧
      (baseTameSymbol W (normUnit W f) c)⁻¹=milnorSymbol W 0 f (baseFieldUnit W c)) ∧
    (baseInfinityOrder (normParameter W : RatFunc K)=1 ∧
      baseTameSymbol W (normParameter W) (normParameter W)= -1 ∧
      tameSymbol W 0 (infinityParameterUnit W) (embeddedNorm W (infinityParameterUnit W))= -1 ∧
      unsignedBaseTame W (normParameter W) (normParameter W)=1) ∧
    (tameCurve.Δ≠0 ∧ baseInfinityOrder (normParameter tameCurve : RatFunc ControlBase)=1 ∧
    baseTameSymbol tameCurve (normParameter tameCurve) (normParameter tameCurve)= -1 ∧
    tameSymbol tameCurve 0 (infinityParameterUnit tameCurve)
      (embeddedNorm tameCurve (infinityParameterUnit tameCurve))= -1 ∧
    unsignedBaseTame tameCurve (normParameter tameCurve) (normParameter tameCurve)=1 ∧
    unsignedBaseTame tameCurve (normParameter tameCurve) (normParameter tameCurve)≠
      tameSymbol tameCurve 0 (infinityParameterUnit tameCurve)
        (embeddedNorm tameCurve (infinityParameterUnit tameCurve))) ∧
    (GeneralTranslationAudit.binaryCurve.Δ≠0 ∧
    (baseTameSymbol GeneralTranslationAudit.binaryCurve
      (normParameter GeneralTranslationAudit.binaryCurve) (normParameter GeneralTranslationAudit.binaryCurve) : GeneralF2)=1 ∧
    (tameSymbol GeneralTranslationAudit.binaryCurve 0 (infinityParameterUnit GeneralTranslationAudit.binaryCurve)
      (embeddedNorm GeneralTranslationAudit.binaryCurve (infinityParameterUnit GeneralTranslationAudit.binaryCurve)) : GeneralF2)=1 ∧
    GeneralTranslationAudit.ternaryCurve.Δ≠0 ∧
    (baseTameSymbol GeneralTranslationAudit.ternaryCurve
      (normParameter GeneralTranslationAudit.ternaryCurve) (normParameter GeneralTranslationAudit.ternaryCurve) : GeneralF3)=2 ∧
    (tameSymbol GeneralTranslationAudit.ternaryCurve 0 (infinityParameterUnit GeneralTranslationAudit.ternaryCurve)
      (embeddedNorm GeneralTranslationAudit.ternaryCurve (infinityParameterUnit GeneralTranslationAudit.ternaryCurve)) : GeneralF3)=2) := by
  refine ⟨base_embedding_infinity_valuation W,
    ⟨base_evaluation_kernel W,base_evaluation_surjective W,base_eval_constant W⟩,
    base_evaluation_model_independent W,base_unit_evaluation_compatibility W,base_tame_model_independent W,
    ⟨inertia_on_parameter W,actual_inertia_character W⟩,?_,
    ⟨norm_parameter_order W,actual_norm_parameter_values W⟩,
    actual_f5_missing_base_sign_counter,actual_small_characteristic_projection_values⟩
  intro f c
  exact ⟨actual_infinity_norm_projection W f c,actual_infinity_inverse_projection W f c⟩

#print axioms actual_elliptic_infinity_norm_projection_certificate
end
end EllipticInfinityNormProjectionAudit
