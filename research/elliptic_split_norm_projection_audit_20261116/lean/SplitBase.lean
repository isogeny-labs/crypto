import InfinityCertificate

namespace EllipticSplitNormProjectionAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerNormValuationAudit MillerInfinityChartAudit MillerFinitePointAudit MillerRecursionAudit
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit NegationPullbackAudit
open FinitePointEvaluationAudit GeneralTranslationAudit WeilAuxiliaryRatioAudit
open EllipticTameSymbolAudit EllipticFieldNormAudit EllipticNormTransportAudit EllipticInfinityNormProjectionAudit
open scoped EllipticFieldNormAudit WithZero
variable {K : Type*} [Field K]
attribute [local instance] normFunctionCommRing

/-- Actual restriction to K(x). Normalization at a split point will be derived separately. -/
def pointBaseValuation (W : WeierstrassCurve.Affine K) (P : W.Point) : Valuation (RatFunc K) ℤᵐ⁰ :=
  (localFunctionValuation W P).comap (algebraMap (RatFunc K) W.FunctionField)

def pointBaseOrder (W : WeierstrassCurve.Affine K) (P : W.Point) (c : RatFunc K) : ℤ :=
  -WithZero.log (pointBaseValuation W P c)

theorem point_base_order_embedding (W : WeierstrassCurve.Affine K) (P : W.Point) (c : RatFunc K) :
    pointBaseOrder W P c=pointOrder W (algebraMap (RatFunc K) W.FunctionField c) P := by
  rw [point_order_formula]
  rfl

/-- The two actual conjugate points restrict to exactly the same actual base valuation. -/
theorem point_base_valuation_neg (W : WeierstrassCurve.Affine K) (P : W.Point) :
    pointBaseValuation W (-P)=pointBaseValuation W P := by
  ext c
  change localFunctionValuation W (-P) (algebraMap (RatFunc K) W.FunctionField c)=
    localFunctionValuation W P (algebraMap (RatFunc K) W.FunctionField c)
  have he:=local_negation_value W P (algebraMap (RatFunc K) W.FunctionField c)
  rw [conjugation_fixes_base] at he
  exact he.symm

abbrev PointBaseRing (W : WeierstrassCurve.Affine K) (P : W.Point) :=
  (pointBaseValuation W P).valuationSubring

def pointBaseEmbedding (W : WeierstrassCurve.Affine K) (P : W.Point) :
    PointBaseRing W P →+* RegularRing W P where
  toFun r := ⟨algebraMap (RatFunc K) W.FunctionField r,by
    change pointBaseValuation W P (r : RatFunc K)≤1
    exact r.property⟩
  map_zero' := Subtype.ext (map_zero _)
  map_one' := Subtype.ext (map_one _)
  map_add' a b := Subtype.ext (map_add _ (a : RatFunc K) (b : RatFunc K))
  map_mul' a b := Subtype.ext (map_mul _ (a : RatFunc K) (b : RatFunc K))

def pointBaseEvaluation (W : WeierstrassCurve.Affine K) (P : W.Point) : PointBaseRing W P →+* K :=
  (regularEvaluation W P).comp (pointBaseEmbedding W P)

def pointBaseConstant (W : WeierstrassCurve.Affine K) (P : W.Point) (c : K) : PointBaseRing W P :=
  ⟨algebraMap K (RatFunc K) c,by
    change localFunctionValuation W P (algebraMap (RatFunc K) W.FunctionField
      (algebraMap K (RatFunc K) c))≤1
    rw [base_coefficient_map]
    exact local_constant_integral W P c⟩

theorem point_base_eval_constant (W : WeierstrassCurve.Affine K) (P : W.Point) (c : K) :
    pointBaseEvaluation W P (pointBaseConstant W P c)=c := by
  have he : pointBaseEmbedding W P (pointBaseConstant W P c)=regularConstant W P c :=
    Subtype.ext (base_coefficient_map W c)
  change regularEvaluation W P (pointBaseEmbedding W P (pointBaseConstant W P c))=c
  rw [he,regular_eval_constant]

theorem point_base_eval_surjective (W : WeierstrassCurve.Affine K) (P : W.Point) :
    Function.Surjective (pointBaseEvaluation W P) :=
  fun c=>⟨pointBaseConstant W P c,point_base_eval_constant W P c⟩

theorem point_base_eval_kernel (W : WeierstrassCurve.Affine K) (P : W.Point) :
    RingHom.ker (pointBaseEvaluation W P)=IsLocalRing.maximalIdeal (PointBaseRing W P) :=
  IsLocalRing.eq_maximalIdeal (RingHom.ker_isMaximal_of_surjective _ (point_base_eval_surjective W P))

/-- Actual proper base-unit residue, on its actual valuation-ring unit group. -/
def pointBaseUnitEvaluation (W : WeierstrassCurve.Affine K) (P : W.Point) :
    (PointBaseRing W P).unitGroup →* Kˣ :=
  (Units.map (pointBaseEvaluation W P).toMonoidHom).comp (PointBaseRing W P).unitGroupMulEquiv.toMonoidHom

def pointBaseUnitEmbedding (W : WeierstrassCurve.Affine K) (P : W.Point) :
    (PointBaseRing W P).unitGroup →* (RegularRing W P).unitGroup where
  toFun f := ⟨baseFieldUnit W f.val,by
    apply (mem_point_units_iff W P _).mpr
    rw [point_order_formula]
    change -WithZero.log (pointBaseValuation W P (f.val : RatFunc K))=0
    have hv : pointBaseValuation W P (f.val : RatFunc K)=1 :=
      (Valuation.mem_unitGroup_iff (RatFunc K) (pointBaseValuation W P) f.val).mp f.property
    rw [hv,WithZero.log_one,neg_zero]⟩
  map_one' := Subtype.ext (map_one _)
  map_mul' a b := Subtype.ext (map_mul _ a.val b.val)

theorem point_base_unit_eval_compatibility (W : WeierstrassCurve.Affine K) (P : W.Point)
    (u : (PointBaseRing W P).unitGroup) :
    pointBaseUnitEvaluation W P u=pointUnitEvaluation W P (pointBaseUnitEmbedding W P u) := by
  apply Units.ext
  rfl

#print axioms point_base_order_embedding
#print axioms point_base_valuation_neg
#print axioms point_base_eval_kernel
#print axioms point_base_unit_eval_compatibility
end
end EllipticSplitNormProjectionAudit
