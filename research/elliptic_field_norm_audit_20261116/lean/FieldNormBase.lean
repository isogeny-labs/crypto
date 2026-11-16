import TameCertificate
import NormImports

namespace EllipticFieldNormAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine WeierstrassCurve.Affine.CoordinateRing
open MillerNormValuationAudit MillerInfinityChartAudit MillerFinitePointAudit MillerRecursionAudit
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit NegationPullbackAudit
variable {K : Type*} [Field K]

/-- Actual K(x) embedding, induced by the proved actual polynomial-function injection. -/
def rationalBaseMap (W : WeierstrassCurve.Affine K) : RatFunc K →+* W.FunctionField :=
  IsFractionRing.lift (polynomial_function_injective W)

scoped instance rationalBaseAlgebra (W : WeierstrassCurve.Affine K) :
    Algebra (RatFunc K) W.FunctionField := (rationalBaseMap W).toAlgebra

open scoped EllipticFieldNormAudit

theorem rational_base_polynomial (W : WeierstrassCurve.Affine K) (p : K[X]) :
    algebraMap (RatFunc K) W.FunctionField (algebraMap K[X] (RatFunc K) p)=polynomialFunction W p :=
  IsFractionRing.lift_algebraMap (polynomial_function_injective W) p

/-- The scalar tower uses the actual existing coordinate-polynomial map. -/
scoped instance polynomialBaseTower (W : WeierstrassCurve.Affine K) :
    IsScalarTower K[X] (RatFunc K) W.FunctionField :=
  IsScalarTower.of_algebraMap_eq fun p => (rational_base_polynomial W p).symm

scoped instance coefficientBaseTower (W : WeierstrassCurve.Affine K) :
    IsScalarTower K (RatFunc K) W.FunctionField := by
  apply IsScalarTower.of_algebraMap_eq
  intro c
  rw [IsScalarTower.algebraMap_apply K K[X] (RatFunc K),rational_base_polynomial]
  exact (EllipticTranslationAudit.function_coefficient_map W c).trans
    (polynomial_function_constant W c).symm

theorem rational_base_injective (W : WeierstrassCurve.Affine K) :
    Function.Injective (rationalBaseMap W) := (rationalBaseMap W).injective

theorem actual_polynomial_base_injective (W : WeierstrassCurve.Affine K) :
    Function.Injective (algebraMap K[X] W.FunctionField) := polynomial_function_injective W

#print axioms rational_base_polynomial
#print axioms polynomialBaseTower
#print axioms coefficientBaseTower
end
end EllipticFieldNormAudit
