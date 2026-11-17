module
public import TranslationExample

@[expose] public section
namespace EllipticTranslationAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine
open MillerNormValuationAudit MillerInfinityChartAudit MillerFinitePointAudit MillerRecursionAudit
open EllipticPicardBridgeAudit
variable {K : Type*} [Field K]

/-- Actual reciprocal-x infinity order, with no genus or polynomial-membership premise. -/
theorem reciprocal_x_infinity_order (W : WeierstrassCurve.Affine K) :
    pointOrder W (xFunction W)⁻¹ 0=2 := by
  have hx : pointOrder W (xFunction W) 0= -2 := by
    change -WithZero.log (chartAdicValuation W (xFunction W))= -2
    rw [chart_adic_eq_norm,function_valuation_x,WithZero.log_exp]
  have he := point_order_div W one_ne_zero (x_function_nonzero W) (0 : W.Point)
  simpa only [one_div,point_order_one,hx,sub_neg_eq_add,zero_add] using he

/-- Actual reciprocal x is never in the actual affine coordinate-ring image. -/
theorem reciprocal_x_not_coordinate_image (W : WeierstrassCurve.Affine K) :
    ¬ ∃ a : W.CoordinateRing, (xFunction W)⁻¹=algebraMap W.CoordinateRing W.FunctionField a := by
  rintro ⟨a,ha⟩
  have hn : a ≠ 0 := by
    intro hz
    apply inv_ne_zero (x_function_nonzero W)
    exact ha.trans (by rw [hz,map_zero])
  have ho := reciprocal_x_infinity_order W
  rw [ha,coordinate_infinity_order W hn] at ho
  omega

#print axioms reciprocal_x_not_coordinate_image
end
end EllipticTranslationAudit
