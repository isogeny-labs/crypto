import FinitePlaceFiber

namespace EllipticFinitePlaceProjectionAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerNormValuationAudit MillerInfinityChartAudit MillerFinitePointAudit MillerRecursionAudit
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit NegationPullbackAudit
open FinitePointEvaluationAudit GeneralTranslationAudit WeilAuxiliaryRatioAudit
open EllipticTameSymbolAudit EllipticFieldNormAudit EllipticNormTransportAudit EllipticInfinityNormProjectionAudit
open EllipticSplitNormProjectionAudit EllipticRamifiedNormProjectionAudit
open scoped EllipticFieldNormAudit WithZero
variable {K : Type*} [Field K]
attribute [local instance] normFunctionCommRing

/-- The same official normalized base ring embeds at both actual finite point types. -/
def finitePlaceEmbedding (W : WeierstrassCurve.Affine K) {x y : K} (h : W.Nonsingular x y) :
    FiniteBaseRing x →+* RegularRing W (Point.some x y h) where
  toFun r:=⟨algebraMap (RatFunc K) W.FunctionField r,by
    change localFunctionValuation W (Point.some x y h) (algebraMap (RatFunc K) W.FunctionField (r : RatFunc K))≤1
    rw [actual_finite_place_valuation W h]
    exact pow_le_one₀ zero_le r.property⟩
  map_zero':=Subtype.ext (map_zero _)
  map_one':=Subtype.ext (map_one _)
  map_add' a b:=Subtype.ext (map_add _ (a : RatFunc K) (b : RatFunc K))
  map_mul' a b:=Subtype.ext (map_mul _ (a : RatFunc K) (b : RatFunc K))

def finitePlaceEvaluation (W : WeierstrassCurve.Affine K) {x y : K} (h : W.Nonsingular x y) :
    FiniteBaseRing x →+* K := (regularEvaluation W (Point.some x y h)).comp (finitePlaceEmbedding W h)

theorem finite_place_eval_constant (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) (c : K) : finitePlaceEvaluation W h (finiteBaseConstant x c)=c := by
  have he : finitePlaceEmbedding W h (finiteBaseConstant x c)=regularConstant W (Point.some x y h) c :=
    Subtype.ext (base_coefficient_map W c)
  change regularEvaluation W (Point.some x y h) (finitePlaceEmbedding W h (finiteBaseConstant x c))=c
  rw [he,regular_eval_constant]

theorem finite_place_eval_surjective (W : WeierstrassCurve.Affine K) {x y : K} (h : W.Nonsingular x y) :
    Function.Surjective (finitePlaceEvaluation W h) := fun c=>⟨finiteBaseConstant x c,finite_place_eval_constant W h c⟩

theorem finite_place_eval_kernel (W : WeierstrassCurve.Affine K) {x y : K} (h : W.Nonsingular x y) :
    RingHom.ker (finitePlaceEvaluation W h)=IsLocalRing.maximalIdeal (FiniteBaseRing x) :=
  IsLocalRing.eq_maximalIdeal (RingHom.ker_isMaximal_of_surjective _ (finite_place_eval_surjective W h))

def finitePlaceUnitEvaluation (W : WeierstrassCurve.Affine K) {x y : K} (h : W.Nonsingular x y) :
    (FiniteBaseRing x).unitGroup →* Kˣ :=
  (Units.map (finitePlaceEvaluation W h).toMonoidHom).comp (FiniteBaseRing x).unitGroupMulEquiv.toMonoidHom

def finitePlaceUnitEmbedding (W : WeierstrassCurve.Affine K) {x y : K} (h : W.Nonsingular x y) :
    (FiniteBaseRing x).unitGroup →* (RegularRing W (Point.some x y h)).unitGroup where
  toFun f:=⟨baseFieldUnit W f.val,by
    apply (mem_point_units_iff W (Point.some x y h) _).mpr
    rw [point_order_formula]
    change -WithZero.log (localFunctionValuation W (Point.some x y h)
      (algebraMap (RatFunc K) W.FunctionField (f.val : RatFunc K)))=0
    have hv : finiteBaseValuation x (f.val : RatFunc K)=1 :=
      (Valuation.mem_unitGroup_iff (RatFunc K) (finiteBaseValuation x) f.val).mp f.property
    rw [actual_finite_place_valuation W h,hv,one_pow,WithZero.log_one,neg_zero]⟩
  map_one':=Subtype.ext (map_one _)
  map_mul' a b:=Subtype.ext (map_mul _ a.val b.val)

theorem finite_place_unit_eval_compatibility (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) (u : (FiniteBaseRing x).unitGroup) :
    finitePlaceUnitEvaluation W h u=pointUnitEvaluation W (Point.some x y h) (finitePlaceUnitEmbedding W h u) := by
  apply Units.ext
  rfl

/-- Same normalized base place, arbitrary realizing points/models, even across split and fixed. -/
theorem finite_place_eval_model_independent (W₁ W₂ : WeierstrassCurve.Affine K) {x y₁ y₂ : K}
    (h₁ : W₁.Nonsingular x y₁) (h₂ : W₂.Nonsingular x y₂) (r : FiniteBaseRing x) :
    finitePlaceEvaluation W₁ h₁ r=finitePlaceEvaluation W₂ h₂ r := by
  have hz : r-finiteBaseConstant x (finitePlaceEvaluation W₁ h₁ r)∈RingHom.ker (finitePlaceEvaluation W₁ h₁) := by
    change finitePlaceEvaluation W₁ h₁ (r-finiteBaseConstant x (finitePlaceEvaluation W₁ h₁ r))=0
    rw [map_sub,finite_place_eval_constant,sub_self]
  have hz₂ : r-finiteBaseConstant x (finitePlaceEvaluation W₁ h₁ r)∈RingHom.ker (finitePlaceEvaluation W₂ h₂) := by
    rw [finite_place_eval_kernel] at hz ⊢
    exact hz
  change finitePlaceEvaluation W₂ h₂ (r-finiteBaseConstant x (finitePlaceEvaluation W₁ h₁ r))=0 at hz₂
  rw [map_sub,finite_place_eval_constant,sub_eq_zero] at hz₂
  exact hz₂.symm

theorem finite_place_unit_model_independent (W₁ W₂ : WeierstrassCurve.Affine K) {x y₁ y₂ : K}
    (h₁ : W₁.Nonsingular x y₁) (h₂ : W₂.Nonsingular x y₂) (u : (FiniteBaseRing x).unitGroup) :
    finitePlaceUnitEvaluation W₁ h₁ u=finitePlaceUnitEvaluation W₂ h₂ u := by
  apply Units.ext
  exact finite_place_eval_model_independent W₁ W₂ h₁ h₂ _

theorem finite_place_eval_fixed (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) (hP : (Point.some x y h : W.Point)= -Point.some x y h) (r : FiniteBaseRing x) :
    finitePlaceEvaluation W h r=finiteBaseEvaluation W h hP r := rfl

theorem finite_place_unit_eval_fixed (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) (hP : (Point.some x y h : W.Point)= -Point.some x y h) (u : (FiniteBaseRing x).unitGroup) :
    finitePlaceUnitEvaluation W h u=finiteBaseUnitEvaluation W h hP u := rfl

#print axioms finite_place_eval_kernel
#print axioms finite_place_unit_eval_compatibility
#print axioms finite_place_eval_model_independent
#print axioms finite_place_unit_eval_fixed
end
end EllipticFinitePlaceProjectionAudit
