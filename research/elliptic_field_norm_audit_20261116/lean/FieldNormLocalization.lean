import FieldNormBase

namespace EllipticFieldNormAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine WeierstrassCurve.Affine.CoordinateRing
open MillerNormValuationAudit MillerInfinityChartAudit MillerFinitePointAudit MillerRecursionAudit
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit NegationPullbackAudit
open scoped EllipticFieldNormAudit nonZeroDivisors
variable {K : Type*} [Field K]

/-- Actual nonzero base-polynomial images in the actual affine coordinate ring. -/
def polynomialDenominators (W : WeierstrassCurve.Affine K) : Submonoid W.CoordinateRing :=
  Algebra.algebraMapSubmonoid W.CoordinateRing (nonZeroDivisors K[X])

theorem norm_denominator_member (W : WeierstrassCurve.Affine K) (b : W.CoordinateRing)
    (hb : b≠0) : mk W (C (Algebra.norm K[X] b))∈polynomialDenominators W := by
  refine ⟨Algebra.norm K[X] b,?_,rfl⟩
  exact mem_nonZeroDivisors_iff_ne_zero.mpr (norm_polynomial_nonzero W hb)

/-- Every actual full function has a denominator which is an actual base polynomial.
The witness is obtained from the actual coordinate norm of an arbitrary denominator. -/
theorem base_denominator_surjective (W : WeierstrassCurve.Affine K) (f : W.FunctionField) :
    ∃ x : W.CoordinateRing × polynomialDenominators W,
      f*algebraMap W.CoordinateRing W.FunctionField x.2=
        algebraMap W.CoordinateRing W.FunctionField x.1 := by
  obtain ⟨a,b,hb,hab⟩ := IsFractionRing.div_surjective W.CoordinateRing f
  have hb0 : b≠0 := nonZeroDivisors.ne_zero hb
  have hbf : algebraMap W.CoordinateRing W.FunctionField b≠0 :=
    by simpa only [map_zero] using (IsFractionRing.injective W.CoordinateRing W.FunctionField).ne hb0
  refine ⟨⟨a*coordinateNeg W b,⟨mk W (C (Algebra.norm K[X] b)),norm_denominator_member W b hb0⟩⟩,?_⟩
  change f*algebraMap W.CoordinateRing W.FunctionField (mk W (C (Algebra.norm K[X] b)))=
    algebraMap W.CoordinateRing W.FunctionField (a*coordinateNeg W b)
  rw [←hab,coordinate_norm_product,map_mul,map_mul,←mul_assoc,div_mul_cancel₀ _ hbf]

/-- The actual fraction field is the required base-polynomial localization, not assumed. -/
scoped instance actualBaseLocalization (W : WeierstrassCurve.Affine K) :
    IsLocalization (polynomialDenominators W) W.FunctionField := by
  apply (isLocalization_iff (polynomialDenominators W) W.FunctionField).mpr
  refine ⟨?_,base_denominator_surjective W,?_⟩
  · rintro ⟨y,hy⟩
    obtain ⟨q,hq,rfl⟩ := hy
    apply isUnit_iff_ne_zero.mpr
    change polynomialFunction W q≠0
    exact polynomial_function_nonzero W (nonZeroDivisors.ne_zero hq)
  · intro x y he
    refine ⟨1,?_⟩
    simpa only [OneMemClass.coe_one,one_mul] using
      (IsFractionRing.injective W.CoordinateRing W.FunctionField he)

#print axioms base_denominator_surjective
#print axioms actualBaseLocalization
end
end EllipticFieldNormAudit
