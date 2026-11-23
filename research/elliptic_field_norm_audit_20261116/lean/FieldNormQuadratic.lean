import FieldNormLocalization

namespace EllipticFieldNormAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine WeierstrassCurve.Affine.CoordinateRing
open MillerNormValuationAudit MillerInfinityChartAudit MillerFinitePointAudit MillerRecursionAudit
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit NegationPullbackAudit
open scoped EllipticFieldNormAudit nonZeroDivisors
variable {K : Type*} [Field K]

/-- The actual field basis is obtained by localizing the actual coordinate-ring basis. -/
def functionBasis (W : WeierstrassCurve.Affine K) : Module.Basis (Fin 2) (RatFunc K) W.FunctionField :=
  (CoordinateRing.basis W).localizationLocalization (RatFunc K) (nonZeroDivisors K[X]) W.FunctionField

scoped instance functionFinite (W : WeierstrassCurve.Affine K) :
    Module.Finite (RatFunc K) W.FunctionField := Module.Finite.of_basis (functionBasis W)

theorem function_basis_zero (W : WeierstrassCurve.Affine K) : functionBasis W 0=1 := by
  rw [functionBasis,Module.Basis.localizationLocalization_apply,basis_zero,map_one]

theorem function_basis_one (W : WeierstrassCurve.Affine K) : functionBasis W 1=yFunction W := by
  rw [functionBasis,Module.Basis.localizationLocalization_apply,basis_one]
  simp only [yFunction,YClass,map_zero,sub_zero]

/-- Actual degree2, with no algebraic-closure or coprimality hypothesis. -/
theorem function_field_degree_two (W : WeierstrassCurve.Affine K) :
    Module.finrank (RatFunc K) W.FunctionField=2 := by
  rw [Module.finrank_eq_card_basis (functionBasis W)]
  rfl

/-- Exact official field norm agrees with the existing actual coordinate norm on the affine ring. -/
theorem norm_coordinate (W : WeierstrassCurve.Affine K) (a : W.CoordinateRing) :
    Algebra.norm (RatFunc K) (algebraMap W.CoordinateRing W.FunctionField a)=
      algebraMap K[X] (RatFunc K) (Algebra.norm K[X] a) :=
  Algebra.norm_localization K[X] (nonZeroDivisors K[X]) a

theorem norm_fraction (W : WeierstrassCurve.Affine K) (a b : W.FunctionField) :
    Algebra.norm (RatFunc K) (a/b)=Algebra.norm (RatFunc K) a/Algebra.norm (RatFunc K) b := by
  rw [div_eq_mul_inv,map_mul,Algebra.norm_inv,div_eq_mul_inv]

/-- Actual field norm on the actual full elliptic function field, not just a formula on symbols. -/
theorem actual_field_norm_product (W : WeierstrassCurve.Affine K) (f : W.FunctionField) :
    algebraMap (RatFunc K) W.FunctionField (Algebra.norm (RatFunc K) f)=f*fieldNeg W f := by
  obtain ⟨a,b,_hb,hab⟩ := IsFractionRing.div_surjective W.CoordinateRing f
  rw [←hab,norm_fraction,norm_coordinate,norm_coordinate,map_div₀,
    rational_base_polynomial,rational_base_polynomial]
  exact (field_fraction_norm W a b).symm

/-- Scalars have degree2 norm, rather than degree1. -/
theorem norm_base_scalar (W : WeierstrassCurve.Affine K) (c : RatFunc K) :
    Algebra.norm (RatFunc K) (algebraMap (RatFunc K) W.FunctionField c)=c^2 := by
  rw [Algebra.norm_algebraMap,function_field_degree_two]

#print axioms function_field_degree_two
#print axioms norm_coordinate
#print axioms actual_field_norm_product
end
end EllipticFieldNormAudit
