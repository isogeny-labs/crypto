import FieldNormCertificate
import TransportImports

namespace EllipticNormTransportAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerNormValuationAudit MillerInfinityChartAudit MillerFinitePointAudit MillerRecursionAudit
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit NegationPullbackAudit
open FinitePointEvaluationAudit GeneralTranslationAudit WeilAuxiliaryRatioAudit
open EllipticTameSymbolAudit EllipticFieldNormAudit
open scoped EllipticFieldNormAudit
variable {K : Type*} [Field K]
attribute [local instance] normFunctionCommRing

/-- The official finite-base norm of actual units. -/
def normUnit (W : WeierstrassCurve.Affine K) : W.FunctionFieldˣ →* (RatFunc K)ˣ :=
  Units.map (Algebra.norm (RatFunc K))

/-- Embed the official norm into the actual elliptic field; no product formula is assumed. -/
def embeddedNorm (W : WeierstrassCurve.Affine K) : W.FunctionFieldˣ →* W.FunctionFieldˣ :=
  (Units.map (algebraMap (RatFunc K) W.FunctionField).toMonoidHom).comp (normUnit W)

theorem embedded_norm_coe (W : WeierstrassCurve.Affine K) (f : W.FunctionFieldˣ) :
    (embeddedNorm W f : W.FunctionField)=
      algebraMap (RatFunc K) W.FunctionField (Algebra.norm (RatFunc K) (f : W.FunctionField)) := rfl

theorem embedded_norm_product (W : WeierstrassCurve.Affine K) (f : W.FunctionFieldˣ) :
    embeddedNorm W f=f*negatedUnit W f := by
  apply Units.ext
  exact actual_field_norm_product W f

theorem actual_norm_point_order (W : WeierstrassCurve.Affine K) (f : W.FunctionFieldˣ)
    (P : W.Point) : pointOrder W (embeddedNorm W f) P=
      pointOrder W f P+pointOrder W f (-P) := by
  rw [embedded_norm_product,Units.val_mul,point_order_mul W f.ne_zero (negatedUnit W f).ne_zero]
  change pointOrder W f P+pointOrder W (fieldNeg W (f : W.FunctionField)) P=_
  rw [point_order_pullback]

theorem actual_norm_fixed_point_order (W : WeierstrassCurve.Affine K) (f : W.FunctionFieldˣ)
    (P : W.Point) (hP : -P=P) :
    pointOrder W (embeddedNorm W f) P=2*pointOrder W f P := by
  rw [actual_norm_point_order,hP]
  ring

/-- Actual finite-support divisor identity for the embedded official field norm. -/
theorem actual_norm_divisor (W : WeierstrassCurve.Affine K) (f : W.FunctionFieldˣ) :
    functionDivisor W (embeddedNorm W f) (embeddedNorm W f).ne_zero=
      functionDivisor W f f.ne_zero+divisorNeg W (functionDivisor W f f.ne_zero) := by
  ext P
  rw [function_divisor_apply,Finsupp.add_apply,divisor_neg_apply,
    function_divisor_apply,function_divisor_apply,actual_norm_point_order]

/-- Norm on actual rational-base scalars is square, at the actual unit level. -/
theorem embedded_norm_base (W : WeierstrassCurve.Affine K) (c : (RatFunc K)ˣ) :
    embeddedNorm W (Units.map (algebraMap (RatFunc K) W.FunctionField).toMonoidHom c)=
      (Units.map (algebraMap (RatFunc K) W.FunctionField).toMonoidHom c)^2 := by
  apply Units.ext
  change algebraMap (RatFunc K) W.FunctionField
    (Algebra.norm (RatFunc K) (algebraMap (RatFunc K) W.FunctionField (c : RatFunc K)))=_
  rw [norm_base_scalar,map_pow]
  rfl

#print axioms actual_norm_point_order
#print axioms actual_norm_divisor
#print axioms embedded_norm_base
end
end EllipticNormTransportAudit
