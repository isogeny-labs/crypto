import InfinityProjection

namespace EllipticInfinityNormProjectionAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerNormValuationAudit MillerInfinityChartAudit MillerFinitePointAudit MillerRecursionAudit
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit NegationPullbackAudit
open FinitePointEvaluationAudit GeneralTranslationAudit WeilAuxiliaryRatioAudit
open EllipticTameSymbolAudit EllipticFieldNormAudit EllipticNormTransportAudit
open scoped EllipticFieldNormAudit WithZero
variable {K : Type*} [Field K]
attribute [local instance] normFunctionCommRing

/-- Actual base residue identification fixes K and has the actual maximal ideal as kernel;
therefore the construction is independent of the elliptic model used to realize it. -/
theorem base_evaluation_model_independent (W₁ W₂ : WeierstrassCurve.Affine K)
    (r : BaseInfinityRing K) : baseEvaluation W₁ r=baseEvaluation W₂ r := by
  have hz : r-baseConstant (baseEvaluation W₁ r)∈RingHom.ker (baseEvaluation W₁) := by
    change baseEvaluation W₁ (r-baseConstant (baseEvaluation W₁ r))=0
    rw [map_sub,base_eval_constant,sub_self]
  have hz₂ : r-baseConstant (baseEvaluation W₁ r)∈RingHom.ker (baseEvaluation W₂) := by
    rw [base_evaluation_kernel] at hz ⊢
    exact hz
  change baseEvaluation W₂ (r-baseConstant (baseEvaluation W₁ r))=0 at hz₂
  rw [map_sub,base_eval_constant,sub_eq_zero] at hz₂
  exact hz₂.symm

theorem base_unit_evaluation_model_independent (W₁ W₂ : WeierstrassCurve.Affine K)
    (u : (BaseInfinityRing K).unitGroup) : baseUnitEvaluation W₁ u=baseUnitEvaluation W₂ u := by
  apply Units.ext
  change baseEvaluation W₁ ((BaseInfinityRing K).unitGroupMulEquiv u : BaseInfinityRing K)=
    baseEvaluation W₂ ((BaseInfinityRing K).unitGroupMulEquiv u : BaseInfinityRing K)
  exact base_evaluation_model_independent W₁ W₂ _

theorem base_tame_model_independent (W₁ W₂ : WeierstrassCurve.Affine K)
    (a b : (RatFunc K)ˣ) : baseTameSymbol W₁ a b=baseTameSymbol W₂ a b := by
  rw [baseTameSymbol,baseTameSymbol,base_unit_evaluation_model_independent W₁ W₂]

/-- Actual official norm of the actual infinity parameter. -/
def normParameter (W : WeierstrassCurve.Affine K) : (RatFunc K)ˣ :=
  normUnit W (infinityParameterUnit W)

theorem norm_parameter_order (W : WeierstrassCurve.Affine K) :
    baseInfinityOrder (normParameter W : RatFunc K)=1 := by
  rw [normParameter,norm_unit_base_order,infinity_parameter_order]

/-- Exact two-positive-order case, including its actual leading sign. -/
theorem actual_norm_parameter_values (W : WeierstrassCurve.Affine K) :
    baseTameSymbol W (normParameter W) (normParameter W)= -1 ∧
    tameSymbol W 0 (infinityParameterUnit W) (embeddedNorm W (infinityParameterUnit W))= -1 ∧
    unsignedBaseTame W (normParameter W) (normParameter W)=1 := by
  have hb:=actual_base_parameter_diagonal W (normParameter W) (norm_parameter_order W)
  have hp:=actual_infinity_norm_projection W (infinityParameterUnit W) (normParameter W)
  change baseTameSymbol W (normParameter W) (normParameter W)=
    tameSymbol W 0 (infinityParameterUnit W) (embeddedNorm W (infinityParameterUnit W)) at hp
  exact ⟨hb.1,hp.symm.trans hb.1,hb.2⟩

private instance : Fact (Nat.Prime 5) := ⟨by decide⟩
private instance : Fact (Nat.Prime 2) := ⟨by decide⟩
private instance : Fact (Nat.Prime 3) := ⟨by decide⟩
attribute [local instance] controlBaseCommRing generalF2CommRing generalF3CommRing

/-- Concrete actual norm/parameter pair disproves deleting the base sign from projection. -/
theorem actual_f5_missing_base_sign_counter :
    tameCurve.Δ≠0 ∧ baseInfinityOrder (normParameter tameCurve : RatFunc ControlBase)=1 ∧
    baseTameSymbol tameCurve (normParameter tameCurve) (normParameter tameCurve)= -1 ∧
    tameSymbol tameCurve 0 (infinityParameterUnit tameCurve)
      (embeddedNorm tameCurve (infinityParameterUnit tameCurve))= -1 ∧
    unsignedBaseTame tameCurve (normParameter tameCurve) (normParameter tameCurve)=1 ∧
    unsignedBaseTame tameCurve (normParameter tameCurve) (normParameter tameCurve)≠
      tameSymbol tameCurve 0 (infinityParameterUnit tameCurve)
        (embeddedNorm tameCurve (infinityParameterUnit tameCurve)) := by
  have hv:=actual_norm_parameter_values tameCurve
  refine ⟨MillerPrincipalIdealAudit.control_delta,norm_parameter_order tameCurve,hv.1,hv.2.1,hv.2.2,?_⟩
  rw [hv.2.2,hv.2.1]
  intro he
  have hc:=congrArg (fun z : ControlBaseˣ=>(z : ControlBase)) he
  exact (by decide +kernel : (1 : ControlBase)≠ -1) hc

/-- Actual nonsingular binary and ternary models give the correctly signed values. -/
theorem actual_small_characteristic_projection_values :
    GeneralTranslationAudit.binaryCurve.Δ≠0 ∧
    (baseTameSymbol GeneralTranslationAudit.binaryCurve
      (normParameter GeneralTranslationAudit.binaryCurve) (normParameter GeneralTranslationAudit.binaryCurve) : GeneralF2)=1 ∧
    (tameSymbol GeneralTranslationAudit.binaryCurve 0 (infinityParameterUnit GeneralTranslationAudit.binaryCurve)
      (embeddedNorm GeneralTranslationAudit.binaryCurve (infinityParameterUnit GeneralTranslationAudit.binaryCurve)) : GeneralF2)=1 ∧
    GeneralTranslationAudit.ternaryCurve.Δ≠0 ∧
    (baseTameSymbol GeneralTranslationAudit.ternaryCurve
      (normParameter GeneralTranslationAudit.ternaryCurve) (normParameter GeneralTranslationAudit.ternaryCurve) : GeneralF3)=2 ∧
    (tameSymbol GeneralTranslationAudit.ternaryCurve 0 (infinityParameterUnit GeneralTranslationAudit.ternaryCurve)
      (embeddedNorm GeneralTranslationAudit.ternaryCurve (infinityParameterUnit GeneralTranslationAudit.ternaryCurve)) : GeneralF3)=2 := by
  have hb:=actual_norm_parameter_values GeneralTranslationAudit.binaryCurve
  have ht:=actual_norm_parameter_values GeneralTranslationAudit.ternaryCurve
  refine ⟨binary_delta,?_,?_,ternary_delta,?_,?_⟩
  · rw [hb.1]
    change (-1 : GeneralF2)=1
    decide +kernel
  · rw [hb.2.1]
    change (-1 : GeneralF2)=1
    decide +kernel
  · rw [ht.1]
    change (-1 : GeneralF3)=2
    decide +kernel
  · rw [ht.2.1]
    change (-1 : GeneralF3)=2
    decide +kernel

#print axioms base_evaluation_model_independent
#print axioms actual_norm_parameter_values
#print axioms actual_f5_missing_base_sign_counter
#print axioms actual_small_characteristic_projection_values
end
end EllipticInfinityNormProjectionAudit
