module
public import NegationControls

@[expose] public section
namespace NegationPullbackAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine WeierstrassCurve.Affine.CoordinateRing
open FunctionDivisorFinitenessAudit
variable {K : Type*} [Field K]

/-- The actual rank-two algebra norm is the product with actual coordinate conjugation. -/
theorem coordinate_norm_product (W : WeierstrassCurve.Affine K) (a : W.CoordinateRing) :
    mk W (C (Algebra.norm K[X] a)) = a * coordinateNeg W a := by
  obtain ⟨p, q, rfl⟩ := exists_smul_basis_eq a
  have he := coe_norm_smul_basis (W' := W) p q
  change mk W (C (Algebra.norm K[X] (p • (1 : W.CoordinateRing) + q • mk W X))) =
    mk W ((C p + C q * X) * (C p + C q * (-X - C (linearH W)))) at he
  rw [he, coordinate_neg_basis]
  simp only [smul, mul_one, map_mul, map_add, map_sub, map_neg]
  ring

theorem field_norm_product (W : WeierstrassCurve.Affine K) (a : W.CoordinateRing) :
    polynomialFunction W (Algebra.norm K[X] a) =
      algebraMap W.CoordinateRing W.FunctionField a *
        fieldNeg W (algebraMap W.CoordinateRing W.FunctionField a) := by
  change algebraMap W.CoordinateRing W.FunctionField (mk W (C (Algebra.norm K[X] a))) = _
  rw [coordinate_norm_product, map_mul, field_neg_map]

/-- The norm-product identity on actual full function-field fractions. -/
theorem field_fraction_norm (W : WeierstrassCurve.Affine K) (a b : W.CoordinateRing) :
    (algebraMap W.CoordinateRing W.FunctionField a /
        algebraMap W.CoordinateRing W.FunctionField b) *
      fieldNeg W (algebraMap W.CoordinateRing W.FunctionField a /
        algebraMap W.CoordinateRing W.FunctionField b) =
      polynomialFunction W (Algebra.norm K[X] a) /
        polynomialFunction W (Algebra.norm K[X] b) := by
  rw [map_div₀, div_mul_div_comm, ← field_norm_product, ← field_norm_product]

#print axioms coordinate_norm_product
#print axioms field_fraction_norm
end
end NegationPullbackAudit
