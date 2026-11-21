import ReciprocitySingular

namespace EllipticGlobalReciprocityAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerNormValuationAudit MillerInfinityChartAudit MillerFinitePointAudit MillerRecursionAudit
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit NegationPullbackAudit
open FinitePointEvaluationAudit GeneralTranslationAudit WeilAuxiliaryRatioAudit
open EllipticTameSymbolAudit EllipticFieldNormAudit EllipticNormTransportAudit EllipticInfinityNormProjectionAudit
open scoped EllipticFieldNormAudit WithZero
variable {K : Type*} [Field K]
attribute [local instance] normFunctionCommRing

theorem actual_y_not_base (W : WeierstrassCurve.Affine K) (c : RatFunc K) :
    yFunction W≠algebraMap (RatFunc K) W.FunctionField c := by
  intro he
  have hh:=congrArg (fun z : W.FunctionField=>(functionBasis W).repr z (1 : Fin 2)) he
  have hy : (functionBasis W).repr (yFunction W) (1 : Fin 2)=1 := by
    rw [←function_basis_one W]
    simp
  have hc : (functionBasis W).repr (algebraMap (RatFunc K) W.FunctionField c) (1 : Fin 2)=0 := by
    have hs : algebraMap (RatFunc K) W.FunctionField c=c • functionBasis W 0 := by
      rw [function_basis_zero,Algebra.smul_def,mul_one]
    rw [hs]
    simp
  rw [hy,hc] at hh
  exact one_ne_zero hh

theorem actual_y_minus_one_not_base (W : WeierstrassCurve.Affine K) (c : RatFunc K) :
    yFunction W-1≠algebraMap (RatFunc K) W.FunctionField c := by
  intro he
  apply actual_y_not_base W (c+1)
  rw [map_add,map_one]
  linear_combination he

theorem actual_y_minus_one_nonzero (W : WeierstrassCurve.Affine K) : yFunction W-1≠0 := by
  simpa only [map_zero] using actual_y_minus_one_not_base W (0 : RatFunc K)

def actualYUnit (W : WeierstrassCurve.Affine K) : W.FunctionFieldˣ :=
  Units.mk0 (yFunction W) (y_function_nonzero W)

def actualYMinusOneUnit (W : WeierstrassCurve.Affine K) : W.FunctionFieldˣ :=
  Units.mk0 (yFunction W-1) (actual_y_minus_one_nonzero W)

#print axioms actual_y_not_base
#print axioms actual_y_minus_one_not_base
#print axioms actual_y_minus_one_nonzero
end
end EllipticGlobalReciprocityAudit
