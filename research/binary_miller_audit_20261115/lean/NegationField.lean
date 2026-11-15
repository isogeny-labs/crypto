module
public import NegationCoordinate

@[expose] public section
namespace NegationPullbackAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine WeierstrassCurve.Affine.CoordinateRing
open MillerNormValuationAudit MillerInfinityChartAudit FunctionDivisorFinitenessAudit
variable {K : Type*} [Field K]

/-- The actual function-field automorphism induced by the actual coordinate involution. -/
def fieldNeg (W : WeierstrassCurve.Affine K) : W.FunctionField ≃+* W.FunctionField :=
  IsFractionRing.ringEquivOfRingEquiv (coordinateNegEquiv W)

theorem field_neg_map (W : WeierstrassCurve.Affine K) (a : W.CoordinateRing) :
    fieldNeg W (algebraMap W.CoordinateRing W.FunctionField a) =
      algebraMap W.CoordinateRing W.FunctionField (coordinateNeg W a) :=
  IsFractionRing.ringEquivOfRingEquiv_algebraMap (coordinateNegEquiv W) a

theorem field_neg_square (W : WeierstrassCurve.Affine K) (f : W.FunctionField) :
    fieldNeg W (fieldNeg W f) = f := by
  obtain ⟨a, b, _hb, rfl⟩ := IsFractionRing.div_surjective W.CoordinateRing f
  simp only [map_div₀, field_neg_map]
  rw [coordinate_neg_involutive W a, coordinate_neg_involutive W b]

theorem field_neg_polynomial (W : WeierstrassCurve.Affine K) (p : K[X]) :
    fieldNeg W (polynomialFunction W p) = polynomialFunction W p := by
  change fieldNeg W (algebraMap W.CoordinateRing W.FunctionField (mk W (C p))) = _
  rw [field_neg_map, coordinate_neg_coefficient]
  rfl

theorem field_neg_x (W : WeierstrassCurve.Affine K) :
    fieldNeg W (xFunction W) = xFunction W := by
  simpa only [polynomialFunction, xFunction, XClass, map_zero, sub_zero,
    RingHom.comp_apply] using field_neg_polynomial W X

theorem field_neg_y (W : WeierstrassCurve.Affine K) :
    fieldNeg W (yFunction W) = -yFunction W - polynomialFunction W (linearH W) := by
  have he := field_neg_map W (mk W X)
  rw [coordinate_neg_root, map_sub, map_neg] at he
  simpa only [yFunction, YClass, map_zero, sub_zero, polynomialFunction,
    RingHom.comp_apply] using he

theorem field_neg_norm_value (W : WeierstrassCurve.Affine K) (f : W.FunctionField) :
    functionValuation W (fieldNeg W f) = functionValuation W f := by
  obtain ⟨a, b, _hb, rfl⟩ := IsFractionRing.div_surjective W.CoordinateRing f
  simp only [map_div₀, field_neg_map, function_valuation_map]
  change degreeValue (normDegree W (coordinateNeg W a)) /
      degreeValue (normDegree W (coordinateNeg W b)) =
    degreeValue (normDegree W a) / degreeValue (normDegree W b)
  simp only [normDegree, coordinate_neg_norm]

theorem field_neg_infinity (W : WeierstrassCurve.Affine K) (f : W.FunctionField) :
    chartAdicValuation W (fieldNeg W f) = chartAdicValuation W f := by
  rw [chart_adic_eq_norm, field_neg_norm_value]

#print axioms field_neg_square
#print axioms field_neg_infinity
end
end NegationPullbackAudit
