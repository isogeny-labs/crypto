module
public import ChartAdicValuation
public import LocalOriginEvaluation

@[expose] public section
namespace MillerInfinityChartAudit
noncomputable section
open MillerNormValuationAudit
open scoped WithZero

/-- Actual chart origin, localization, full fraction field, normalized adic valuation and controls.
This does not assert a separate Mathlib projective Scheme.localRing comparison or a full divisor. -/
theorem miller_infinity_chart_certificate :
    ∀ (K : Type) [Field K] (W : WeierstrassCurve.Affine K),
    Function.Surjective (originEval W) ∧
    originIdeal W = Ideal.span {tCoord W, zCoord W} ∧
    IsDiscreteValuationRing (ChartLocalRing W) ∧
    IsLocalRing.maximalIdeal (ChartLocalRing W) = Ideal.span {localT W} ∧
    IsFractionRing (ChartLocalRing W) W.FunctionField ∧
    Function.Bijective (localToValRing W) ∧
    (∀ a : ChartLocalRing W, (chartLocalEquiv W a : W.FunctionField) = localToField W a) ∧
    chartAdicValuation W = functionValuation W ∧
    (∀ a : ChartLocalRing W,
      chartResidueEquiv W (IsLocalRing.residue (ChartLocalRing W) a) = localOriginEval W a) ∧
    (∀ x y slope : K,
      -WithZero.log (chartAdicValuation W (lineFunction W x y slope)) = (-3 : ℤ) ∧
      -WithZero.log (chartAdicValuation W (verticalFunction W x)) = (-2 : ℤ) ∧
      chartAdicValuation W (parameter W ^ 3 * lineFunction W x y slope - 1) < 1 ∧
      chartAdicValuation W (parameter W ^ 2 * verticalFunction W x - 1) < 1 ∧
      chartAdicValuation W ((xFunction W)⁻¹ ^ 3 * lineFunction W x y slope - 1) = 1) := by
  intro K _ W
  refine ⟨origin_eval_surjective W, origin_ideal_coordinates W, chartLocalDVR W,
    local_maximal_generated W, localFunctionFractionRing W,
    ⟨local_to_val_injective W, local_to_val_surjective W⟩, ?_, chart_adic_eq_norm W,
    chart_residue_eval W, ?_⟩
  · intro a
    rfl
  · intro x y slope
    refine ⟨(chart_pole_orders W x y slope).1, (chart_pole_orders W x y slope).2, ?_, ?_, ?_⟩
    · rw [chart_adic_eq_norm]
      exact line_normalized W x y slope
    · rw [chart_adic_eq_norm]
      exact vertical_normalized W x
    · rw [chart_adic_eq_norm]
      exact wrong_parameter_normalization W x y slope

#print axioms miller_infinity_chart_certificate
end
end MillerInfinityChartAudit
