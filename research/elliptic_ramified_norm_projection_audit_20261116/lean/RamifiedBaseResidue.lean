import RamifiedParameter

namespace EllipticRamifiedNormProjectionAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine
open MillerNormValuationAudit MillerInfinityChartAudit MillerFinitePointAudit MillerRecursionAudit
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit NegationPullbackAudit
open FinitePointEvaluationAudit GeneralTranslationAudit WeilAuxiliaryRatioAudit
open EllipticTameSymbolAudit EllipticFieldNormAudit EllipticNormTransportAudit EllipticInfinityNormProjectionAudit
open EllipticSplitNormProjectionAudit
open scoped EllipticFieldNormAudit WithZero
variable {K : Type*} [Field K]
attribute [local instance] normFunctionCommRing

abbrev FiniteBaseRing (x : K) := (finiteBaseValuation x).valuationSubring

def finiteBaseEmbedding (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) (hP : (Point.some x y h : W.Point)= -Point.some x y h) :
    FiniteBaseRing x →+* RegularRing W (Point.some x y h) where
  toFun r:=⟨algebraMap (RatFunc K) W.FunctionField r,by
    change localFunctionValuation W (Point.some x y h) (algebraMap (RatFunc K) W.FunctionField (r : RatFunc K))≤1
    rw [fixed_base_valuation W h hP]
    exact pow_le_one₀ zero_le r.property⟩
  map_zero':=Subtype.ext (map_zero _)
  map_one':=Subtype.ext (map_one _)
  map_add' a b:=Subtype.ext (map_add _ (a : RatFunc K) (b : RatFunc K))
  map_mul' a b:=Subtype.ext (map_mul _ (a : RatFunc K) (b : RatFunc K))

def finiteBaseEvaluation (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) (hP : (Point.some x y h : W.Point)= -Point.some x y h) :
    FiniteBaseRing x →+* K := (regularEvaluation W (Point.some x y h)).comp (finiteBaseEmbedding W h hP)

def finiteBaseConstant (x : K) (c : K) : FiniteBaseRing x :=
  ⟨algebraMap K (RatFunc K) c,by
    change finiteBaseValuation x (algebraMap K[X] (RatFunc K) (C c))≤1
    rw [finiteBaseValuation,IsDedekindDomain.HeightOneSpectrum.valuation_of_algebraMap]
    exact (finiteBasePlace x).intValuation_le_one (C c)⟩

theorem finite_base_eval_constant (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) (hP : (Point.some x y h : W.Point)= -Point.some x y h) (c : K) :
    finiteBaseEvaluation W h hP (finiteBaseConstant x c)=c := by
  have he : finiteBaseEmbedding W h hP (finiteBaseConstant x c)=regularConstant W (Point.some x y h) c :=
    Subtype.ext (base_coefficient_map W c)
  change regularEvaluation W (Point.some x y h) (finiteBaseEmbedding W h hP (finiteBaseConstant x c))=c
  rw [he,regular_eval_constant]

theorem finite_base_eval_surjective (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) (hP : (Point.some x y h : W.Point)= -Point.some x y h) :
    Function.Surjective (finiteBaseEvaluation W h hP) := fun c=>⟨finiteBaseConstant x c,finite_base_eval_constant W h hP c⟩

theorem finite_base_eval_kernel (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) (hP : (Point.some x y h : W.Point)= -Point.some x y h) :
    RingHom.ker (finiteBaseEvaluation W h hP)=IsLocalRing.maximalIdeal (FiniteBaseRing x) :=
  IsLocalRing.eq_maximalIdeal (RingHom.ker_isMaximal_of_surjective _ (finite_base_eval_surjective W h hP))

def finiteBaseUnitEvaluation (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) (hP : (Point.some x y h : W.Point)= -Point.some x y h) :
    (FiniteBaseRing x).unitGroup →* Kˣ :=
  (Units.map (finiteBaseEvaluation W h hP).toMonoidHom).comp (FiniteBaseRing x).unitGroupMulEquiv.toMonoidHom

def finiteBaseUnitEmbedding (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) (hP : (Point.some x y h : W.Point)= -Point.some x y h) :
    (FiniteBaseRing x).unitGroup →* (RegularRing W (Point.some x y h)).unitGroup where
  toFun f:=⟨baseFieldUnit W f.val,by
    apply (mem_point_units_iff W (Point.some x y h) _).mpr
    rw [point_order_formula]
    change -WithZero.log (localFunctionValuation W (Point.some x y h)
      (algebraMap (RatFunc K) W.FunctionField (f.val : RatFunc K)))=0
    have hv : finiteBaseValuation x (f.val : RatFunc K)=1 :=
      (Valuation.mem_unitGroup_iff (RatFunc K) (finiteBaseValuation x) f.val).mp f.property
    rw [fixed_base_valuation W h hP,hv,one_pow,WithZero.log_one,neg_zero]⟩
  map_one':=Subtype.ext (map_one _)
  map_mul' a b:=Subtype.ext (map_mul _ a.val b.val)

theorem finite_base_unit_eval_compatibility (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) (hP : (Point.some x y h : W.Point)= -Point.some x y h)
    (u : (FiniteBaseRing x).unitGroup) :
    finiteBaseUnitEvaluation W h hP u=pointUnitEvaluation W (Point.some x y h) (finiteBaseUnitEmbedding W h hP u) := by
  apply Units.ext
  rfl

/-- Constant fixing and the actual maximal-ideal kernel make the proper base residue canonical. -/
theorem finite_base_eval_model_independent (W₁ W₂ : WeierstrassCurve.Affine K) {x y₁ y₂ : K}
    (h₁ : W₁.Nonsingular x y₁) (hP₁ : (Point.some x y₁ h₁ : W₁.Point)= -Point.some x y₁ h₁)
    (h₂ : W₂.Nonsingular x y₂) (hP₂ : (Point.some x y₂ h₂ : W₂.Point)= -Point.some x y₂ h₂)
    (r : FiniteBaseRing x) : finiteBaseEvaluation W₁ h₁ hP₁ r=finiteBaseEvaluation W₂ h₂ hP₂ r := by
  have hz : r-finiteBaseConstant x (finiteBaseEvaluation W₁ h₁ hP₁ r)∈RingHom.ker (finiteBaseEvaluation W₁ h₁ hP₁) := by
    change finiteBaseEvaluation W₁ h₁ hP₁ (r-finiteBaseConstant x (finiteBaseEvaluation W₁ h₁ hP₁ r))=0
    rw [map_sub,finite_base_eval_constant,sub_self]
  have hz₂ : r-finiteBaseConstant x (finiteBaseEvaluation W₁ h₁ hP₁ r)∈RingHom.ker (finiteBaseEvaluation W₂ h₂ hP₂) := by
    rw [finite_base_eval_kernel] at hz ⊢
    exact hz
  change finiteBaseEvaluation W₂ h₂ hP₂ (r-finiteBaseConstant x (finiteBaseEvaluation W₁ h₁ hP₁ r))=0 at hz₂
  rw [map_sub,finite_base_eval_constant,sub_eq_zero] at hz₂
  exact hz₂.symm

#print axioms finite_base_eval_kernel
#print axioms finite_base_unit_eval_compatibility
#print axioms finite_base_eval_model_independent
end
end EllipticRamifiedNormProjectionAudit
