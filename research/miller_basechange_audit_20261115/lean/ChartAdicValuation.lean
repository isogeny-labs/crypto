module
public import DVRIdentification
public import Mathlib.RingTheory.Valuation.Discrete.IsDiscreteValuationRing

@[expose] public section
namespace MillerInfinityChartAudit
noncomputable section
open MillerNormValuationAudit
open scoped WithZero
variable {K : Type*} [Field K]

local instance localAdicMonoid (W : WeierstrassCurve.Affine K) : Monoid (ChartLocalRing W) :=
  (inferInstance : CommRing (ChartLocalRing W)).toMonoid

local instance localAdicFractionRing (W : WeierstrassCurve.Affine K) :
    IsFractionRing (ChartLocalRing W) W.FunctionField := localFunctionFractionRing W

/-- Official normalized maximal-ideal adic valuation of the actual chart-origin localization. -/
def chartAdicValuation (W : WeierstrassCurve.Affine K) : Valuation W.FunctionField ℤᵐ⁰ :=
  (IsDiscreteValuationRing.maximalIdeal (ChartLocalRing W)).valuation W.FunctionField

theorem local_algebraMap_field (W : WeierstrassCurve.Affine K) (a : ChartLocalRing W) :
    algebraMap (ChartLocalRing W) W.FunctionField a = localToField W a := rfl

theorem chart_adic_parameter (W : WeierstrassCurve.Affine K) :
    chartAdicValuation W (parameter W) = WithZero.exp (-1 : ℤ) := by
  rw [← local_to_field_t, ← local_algebraMap_field]
  unfold chartAdicValuation
  rw [IsDedekindDomain.HeightOneSpectrum.valuation_of_algebraMap]
  exact (IsDiscreteValuationRing.maximalIdeal (ChartLocalRing W)).intValuation_singleton
    (local_t_nonzero W) (local_maximal_generated W)

theorem chart_adic_unit (W : WeierstrassCurve.Affine K) (u : (ChartLocalRing W)ˣ) :
    chartAdicValuation W (localToField W u) = 1 := by
  rw [← local_algebraMap_field]
  unfold chartAdicValuation
  apply (IsDedekindDomain.HeightOneSpectrum.valuation_eq_one_iff_notMem
    (v := IsDiscreteValuationRing.maximalIdeal (ChartLocalRing W))
    (K := W.FunctionField)).mpr
  change (u : ChartLocalRing W) ∉ IsLocalRing.maximalIdeal (ChartLocalRing W)
  simpa only [IsLocalRing.mem_maximalIdeal, mem_nonunits_iff, not_not] using u.isUnit

/-- The official normalized adic valuation equals the constructed norm-degree valuation everywhere. -/
theorem chart_adic_eq_norm (W : WeierstrassCurve.Affine K) :
    chartAdicValuation W = functionValuation W := by
  ext f
  by_cases hf : f = 0
  · rw [hf, map_zero, map_zero]
  have hπ : Irreducible (localT W) :=
    (IsDiscreteValuationRing.irreducible_iff_uniformizer _).mpr (local_maximal_generated W)
  obtain ⟨n, u, hu⟩ := IsDiscreteValuationRing.exists_units_eq_smul_zpow_of_irreducible hπ hf
  have he : f = localToField W u * parameter W ^ n := by
    simpa only [Units.smul_def, Algebra.smul_def, local_algebraMap_field, local_to_field_t] using hu
  rw [he]
  simp only [map_mul, map_zpow₀, chart_adic_unit, local_unit_value,
    chart_adic_parameter, function_valuation_parameter]

theorem chart_adic_line (W : WeierstrassCurve.Affine K) (x y slope : K) :
    chartAdicValuation W (lineFunction W x y slope) = WithZero.exp (3 : ℤ) := by
  rw [chart_adic_eq_norm, function_valuation_line]

theorem chart_adic_vertical (W : WeierstrassCurve.Affine K) (x : K) :
    chartAdicValuation W (verticalFunction W x) = WithZero.exp (2 : ℤ) := by
  rw [chart_adic_eq_norm, function_valuation_vertical]

/-- Exact source-sign pole orders, obtained from the official normalized local adic valuation. -/
theorem chart_pole_orders (W : WeierstrassCurve.Affine K) (x y slope : K) :
    -WithZero.log (chartAdicValuation W (lineFunction W x y slope)) = (-3 : ℤ) ∧
      -WithZero.log (chartAdicValuation W (verticalFunction W x)) = (-2 : ℤ) := by
  rw [chart_adic_line, chart_adic_vertical, WithZero.log_exp, WithZero.log_exp]
  exact ⟨rfl, rfl⟩

#print axioms chart_adic_eq_norm
#print axioms chart_pole_orders
end
end MillerInfinityChartAudit
