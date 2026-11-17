import NormTransportCertificate

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

/-- Actual valuation on the base pulls back with ramification factor2, including0. -/
theorem base_embedding_infinity_valuation (W : WeierstrassCurve.Affine K) (c : RatFunc K) :
    functionValuation W (algebraMap (RatFunc K) W.FunctionField c)=(baseInfinityValuation K c)^2 := by
  have he:=actual_norm_infinity_valuation W (algebraMap (RatFunc K) W.FunctionField c)
  rw [norm_base_scalar,map_pow] at he
  exact he.symm

abbrev BaseInfinityRing (K : Type*) [Field K] := (baseInfinityValuation K).valuationSubring

/-- Actual base valuation ring embeds into the actual elliptic infinity regular ring. -/
def baseRegularEmbedding (W : WeierstrassCurve.Affine K) : BaseInfinityRing K →+* RegularRing W 0 where
  toFun r := ⟨algebraMap (RatFunc K) W.FunctionField r,by
    change functionValuation W (algebraMap (RatFunc K) W.FunctionField r)≤1
    rw [base_embedding_infinity_valuation]
    have hr : baseInfinityValuation K (r : RatFunc K)≤1 := r.property
    simpa only [one_pow] using pow_le_pow_left₀ zero_le hr 2⟩
  map_zero' := Subtype.ext (map_zero _)
  map_one' := Subtype.ext (map_one _)
  map_add' a b := Subtype.ext (map_add _ (a : RatFunc K) (b : RatFunc K))
  map_mul' a b := Subtype.ext (map_mul _ (a : RatFunc K) (b : RatFunc K))

/-- Actual base residue evaluation, induced by the actual valuation-ring embedding. -/
def baseEvaluation (W : WeierstrassCurve.Affine K) : BaseInfinityRing K →+* K :=
  (regularEvaluation W 0).comp (baseRegularEmbedding W)

theorem base_constant_integral (c : K) :
    baseInfinityValuation K (algebraMap K (RatFunc K) c)≤1 := by
  classical
  change RatFunc.inftyValuation K (RatFunc.C c)≤1
  by_cases hc : c=0
  · rw [hc,map_zero,map_zero]
    exact zero_le_one
  · rw [RatFunc.inftyValuation.C K hc]

def baseConstant (c : K) : BaseInfinityRing K :=
  ⟨algebraMap K (RatFunc K) c,base_constant_integral c⟩

theorem base_coefficient_map (W : WeierstrassCurve.Affine K) (c : K) :
    algebraMap (RatFunc K) W.FunctionField (algebraMap K (RatFunc K) c)=constantHom W c := by
  rw [←IsScalarTower.algebraMap_apply K (RatFunc K) W.FunctionField]
  exact EllipticTranslationAudit.function_coefficient_map W c

theorem base_eval_constant (W : WeierstrassCurve.Affine K) (c : K) :
    baseEvaluation W (baseConstant c)=c := by
  have he : baseRegularEmbedding W (baseConstant c)=regularConstant W 0 c :=
    Subtype.ext (base_coefficient_map W c)
  change regularEvaluation W 0 (baseRegularEmbedding W (baseConstant c))=c
  rw [he,regular_eval_constant]

theorem base_evaluation_surjective (W : WeierstrassCurve.Affine K) :
    Function.Surjective (baseEvaluation W) := fun c=>⟨baseConstant c,base_eval_constant W c⟩

theorem base_evaluation_kernel (W : WeierstrassCurve.Affine K) :
    RingHom.ker (baseEvaluation W)=IsLocalRing.maximalIdeal (BaseInfinityRing K) :=
  IsLocalRing.eq_maximalIdeal (RingHom.ker_isMaximal_of_surjective _ (base_evaluation_surjective W))

/-- Proper base unit evaluation on its actual valuation-ring unit subgroup. -/
def baseUnitEvaluation (W : WeierstrassCurve.Affine K) : (BaseInfinityRing K).unitGroup →* Kˣ :=
  (Units.map (baseEvaluation W).toMonoidHom).comp (BaseInfinityRing K).unitGroupMulEquiv.toMonoidHom

def basePointUnit (W : WeierstrassCurve.Affine K) :
    (BaseInfinityRing K).unitGroup →* (RegularRing W 0).unitGroup where
  toFun f := ⟨Units.map (algebraMap (RatFunc K) W.FunctionField).toMonoidHom f.val,by
    apply (mem_point_units_iff W 0 _).mpr
    rw [point_order_formula]
    change -WithZero.log (functionValuation W (algebraMap (RatFunc K) W.FunctionField (f.val : RatFunc K)))=0
    have hv : baseInfinityValuation K (f.val : RatFunc K)=1 :=
      (Valuation.mem_unitGroup_iff (RatFunc K) (baseInfinityValuation K) f.val).mp f.property
    rw [base_embedding_infinity_valuation,hv,one_pow,WithZero.log_one,neg_zero]⟩
  map_one' := Subtype.ext (map_one _)
  map_mul' a b := Subtype.ext (map_mul _ a.val b.val)

theorem base_unit_evaluation_compatibility (W : WeierstrassCurve.Affine K)
    (f : (BaseInfinityRing K).unitGroup) :
    baseUnitEvaluation W f=pointUnitEvaluation W 0 (basePointUnit W f) := by
  apply Units.ext
  rfl

#print axioms base_embedding_infinity_valuation
#print axioms base_evaluation_kernel
#print axioms base_unit_evaluation_compatibility
end
end EllipticInfinityNormProjectionAudit
