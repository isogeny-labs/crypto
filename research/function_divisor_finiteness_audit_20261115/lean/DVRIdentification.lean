module
public import ChartFractionField

@[expose] public section
namespace MillerInfinityChartAudit
noncomputable section
open MillerNormValuationAudit
open scoped WithZero
variable {K : Type*} [Field K]

local instance localMonoid (W : WeierstrassCurve.Affine K) : Monoid (ChartLocalRing W) :=
  (inferInstance : CommRing (ChartLocalRing W)).toMonoid

theorem local_unit_value (W : WeierstrassCurve.Affine K) (u : (ChartLocalRing W)ˣ) :
    functionValuation W (localToField W u) = 1 := by
  have hu : IsUnit (localToValRing W (u : ChartLocalRing W)) :=
    u.isUnit.map (localToValRing W)
  have h := Valuation.Integers.one_of_isUnit
    (Valuation.valuationSubring.integers (functionValuation W)) hu
  exact h

/-- Surjectivity is derived from the actual full fraction field and local DVR factorization. -/
theorem local_to_val_surjective (W : WeierstrassCurve.Affine K) :
    Function.Surjective (localToValRing W) := by
  intro r
  by_cases hr : r = 0
  · refine ⟨0, ?_⟩
    rw [map_zero, hr]
  have hr0 : (r : W.FunctionField) ≠ 0 := fun h => hr (Subtype.ext h)
  obtain ⟨a, b, hb, hab⟩ := @IsFractionRing.div_surjective (ChartLocalRing W) inferInstance
    W.FunctionField inferInstance (localFieldAlgebra W) (localFunctionFractionRing W)
    (r : W.FunctionField)
  change localToField W a / localToField W b = (r : W.FunctionField) at hab
  have hb0 : b ≠ 0 := nonZeroDivisors.ne_zero hb
  have ha0 : a ≠ 0 := by
    intro ha
    apply hr0
    rw [← hab, ha, map_zero, zero_div]
  have hπ : Irreducible (localT W) :=
    (IsDiscreteValuationRing.irreducible_iff_uniformizer _).mpr (local_maximal_generated W)
  obtain ⟨n, u, rfl⟩ := IsDiscreteValuationRing.eq_unit_mul_pow_irreducible ha0 hπ
  obtain ⟨m, w, rfl⟩ := IsDiscreteValuationRing.eq_unit_mul_pow_irreducible hb0 hπ
  have hv : functionValuation W (r : W.FunctionField) = WithZero.exp ((m : ℤ) - n) := by
    rw [← hab, Valuation.map_div]
    simp only [map_mul, map_pow, local_unit_value, local_to_field_t,
      function_valuation_parameter, one_mul, ← WithZero.exp_nsmul, ← WithZero.exp_sub]
    congr 1
    simp only [nsmul_eq_mul]
    ring
  have hmn : m ≤ n := by
    have h : WithZero.exp ((m : ℤ) - n) ≤ 1 := by
      rw [← hv]
      exact r.property
    have hi := WithZero.exp_le_one_iff.mp h
    omega
  refine ⟨(u : ChartLocalRing W) * localT W ^ (n - m) * ((w⁻¹ : (ChartLocalRing W)ˣ) : ChartLocalRing W), ?_⟩
  apply Subtype.ext
  change localToField W _ = (r : W.FunctionField)
  rw [← hab]
  simp only [map_mul, map_pow, map_units_inv, local_to_field_t]
  rw [pow_sub₀ _ (parameter_is_uniformizer W).ne_zero hmn]
  simp only [div_eq_mul_inv, mul_inv]
  ring

/-- Actual origin-localized chart-function ring is isomorphic to the actual norm-degree valuation ring. -/
def chartLocalEquiv (W : WeierstrassCurve.Affine K) : ChartLocalRing W ≃+* valuationRing W :=
  RingEquiv.ofBijective (localToValRing W)
    ⟨local_to_val_injective W, local_to_val_surjective W⟩

theorem chart_local_equiv_parameter (W : WeierstrassCurve.Affine K) :
    chartLocalEquiv W (localT W) = integralParameter W := by
  change localToValRing W (algebraMap (ChartRing W) (ChartLocalRing W) (tCoord W)) = _
  rw [local_to_val_map]
  rfl

#print axioms local_to_val_surjective
#print axioms chart_local_equiv_parameter
end
end MillerInfinityChartAudit
