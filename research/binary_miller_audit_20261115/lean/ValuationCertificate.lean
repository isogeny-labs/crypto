module
public import DiscreteValuation

@[expose] public section
namespace MillerNormValuationAudit
noncomputable section
open scoped WithZero
variable {K : Type*} [Field K]

/-- Actual norm-degree valuation, exact coordinate/line weights and residue normalization.
The canonical projective local-ring valuation identification is not asserted. -/
theorem miller_norm_valuation_certificate :
    ∀ (K : Type) [Field K] (W : WeierstrassCurve.Affine K),
    IsDiscreteValuationRing (valuationRing W) ∧
    Valuation.IsUniformizer (functionValuation W) (parameter W) ∧
    IsLocalRing.maximalIdeal (valuationRing W) = Ideal.span {integralParameter W} ∧
    Function.Surjective (functionValuation W) ∧
    functionValuation W (xFunction W) = WithZero.exp (2 : ℤ) ∧
    functionValuation W (yFunction W) = WithZero.exp (3 : ℤ) ∧
    functionValuation W (parameter W) = WithZero.exp (-1 : ℤ) ∧
    (∀ x : K, functionValuation W (verticalFunction W x) = WithZero.exp (2 : ℤ) ∧
      functionValuation W (parameter W ^ 2 * verticalFunction W x - 1) < 1 ∧
      IsLocalRing.residue (valuationRing W) (normalizedVertical W x) = 1) ∧
    (∀ x y slope : K,
      functionValuation W (lineFunction W x y slope) = WithZero.exp (3 : ℤ) ∧
      functionValuation W (parameter W ^ 3 * lineFunction W x y slope - 1) < 1 ∧
      IsLocalRing.residue (valuationRing W) (normalizedLine W x y slope) = 1 ∧
      functionValuation W ((xFunction W)⁻¹ ^ 3 * lineFunction W x y slope - 1) = 1) ∧
    ¬ Valuation.IsUniformizer (functionValuation W) (xFunction W)⁻¹ := by
  intro K _ W
  refine ⟨inferInstance, parameter_is_uniformizer W, parameter_generates_maximal W,
    function_valuation_surjective W, function_valuation_x W,
    function_valuation_y W, function_valuation_parameter W, ?_, ?_,
    reciprocal_x_not_uniformizer W⟩
  · intro x
    exact ⟨function_valuation_vertical W x, vertical_normalized W x, vertical_residue_one W x⟩
  · intro x y slope
    exact ⟨function_valuation_line W x y slope, line_normalized W x y slope,
      line_residue_one W x y slope, wrong_parameter_normalization W x y slope⟩

#print axioms miller_norm_valuation_certificate
end
end MillerNormValuationAudit
