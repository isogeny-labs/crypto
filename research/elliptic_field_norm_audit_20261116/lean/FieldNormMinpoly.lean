import FieldNormQuadratic

namespace EllipticFieldNormAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine WeierstrassCurve.Affine.CoordinateRing
open MillerNormValuationAudit MillerInfinityChartAudit MillerFinitePointAudit MillerRecursionAudit
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit NegationPullbackAudit
open scoped EllipticFieldNormAudit
variable {K : Type*} [Field K]
noncomputable abbrev normFunctionCommRing (W : WeierstrassCurve.Affine K) :
    CommRing W.FunctionField := (inferInstance : Field W.FunctionField).toCommRing
attribute [local instance] normFunctionCommRing

def functionPowerBasis (W : WeierstrassCurve.Affine K) : PowerBasis (RatFunc K) W.FunctionField where
  gen := yFunction W
  dim := 2
  basis := functionBasis W
  basis_eq_pow i := by
    fin_cases i
    · change functionBasis W 0=1
      exact function_basis_zero W
    · change functionBasis W 1=(yFunction W)^1
      rw [function_basis_one,pow_one]

/-- Actual y-monic equation over the actual rational-function base. -/
def functionQuadratic (W : WeierstrassCurve.Affine K) : (RatFunc K)[X] :=
  X^2+C (algebraMap K[X] (RatFunc K) (linearH W))*X-
    C (algebraMap K[X] (RatFunc K) (X^3+C W.a₂*X^2+C W.a₄*X+C W.a₆))

theorem function_quadratic_monic (W : WeierstrassCurve.Affine K) : (functionQuadratic W).Monic := by
  unfold functionQuadratic
  monicity <;> norm_num

theorem function_quadratic_degree (W : WeierstrassCurve.Affine K) : (functionQuadratic W).degree=2 := by
  unfold functionQuadratic
  compute_degree <;> norm_num

theorem actual_polynomial_x (W : WeierstrassCurve.Affine K) : polynomialFunction W X=xFunction W := by
  simp only [polynomialFunction,xFunction,XClass,RingHom.comp_apply,map_zero,sub_zero]

theorem function_quadratic_root (W : WeierstrassCurve.Affine K) :
    aeval (yFunction W) (functionQuadratic W)=0 := by
  simp only [functionQuadratic,map_sub,map_add,map_mul,map_pow,aeval_X,aeval_C,
    rational_base_polynomial,linearH,actual_polynomial_x,polynomial_function_constant]
  linear_combination function_curve_relation W

/-- Actual minimal polynomial, derived from the actual basis, not supplied irreducibility. -/
theorem actual_y_minpoly (W : WeierstrassCurve.Affine K) :
    minpoly (RatFunc K) (yFunction W)=functionQuadratic W := by
  apply minpoly.eq_of_linearIndependent (RatFunc K) (yFunction W) (function_quadratic_monic W) (function_quadratic_root W)
    2 (function_quadratic_degree W)
  have he : (fun i : Fin 2 => (yFunction W)^i.val)=functionBasis W := by
    funext i
    exact ((functionPowerBasis W).basis_eq_pow i).symm
  rw [he]
  exact (functionBasis W).linearIndependent

theorem function_quadratic_irreducible (W : WeierstrassCurve.Affine K) :
    Irreducible (functionQuadratic W) := by
  rw [←actual_y_minpoly]
  exact minpoly.irreducible (functionPowerBasis W).isIntegral_gen

/-- Every actual function has its actual two rational-base coefficients. -/
theorem rational_linear_representation (W : WeierstrassCurve.Affine K) (f : W.FunctionField) :
    ∃ a b : RatFunc K,algebraMap (RatFunc K) W.FunctionField a+
      algebraMap (RatFunc K) W.FunctionField b*yFunction W=f := by
  have he := (functionBasis W).sum_repr f
  rw [Fin.sum_univ_succ,Fin.sum_univ_one,Fin.succ_zero_eq_one,function_basis_zero,function_basis_one] at he
  simp only [Algebra.smul_def,mul_one] at he
  exact ⟨_,_,he⟩

#print axioms function_quadratic_root
#print axioms actual_y_minpoly
#print axioms function_quadratic_irreducible
#print axioms rational_linear_representation
end
end EllipticFieldNormAudit
