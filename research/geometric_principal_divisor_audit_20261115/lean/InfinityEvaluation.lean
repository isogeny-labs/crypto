module
public import OrderBounds

@[expose] public section
namespace GeometricPrincipalDivisorAudit
noncomputable section
open MillerNormValuationAudit MillerInfinityChartAudit
variable {K : Type*} [Field K]

/-- Actual infinity residue: every actual function regular at infinity differs from a
coefficient-field constant by an actual function of positive infinity order (or zero). -/
theorem infinity_residue_constant (W : WeierstrassCurve.Affine K) (f : W.FunctionField)
    (hf : functionValuation W f ≤ 1) :
    ∃ c : K, functionValuation W (f - constantHom W c) < 1 := by
  let r : valuationRing W := ⟨f, hf⟩
  let a : ChartLocalRing W := (chartLocalEquiv W).symm r
  have ha : localToField W a = f := by
    have he : localToValRing W a = r := (chartLocalEquiv W).apply_symm_apply r
    exact congrArg (fun z : valuationRing W => (z : W.FunctionField)) he
  let c : K := localOriginEval W a
  let b : ChartLocalRing W := a - algebraMap (ChartRing W) (ChartLocalRing W)
    (algebraMap K (ChartRing W) c)
  have hb0 : localOriginEval W b = 0 := by
    change localOriginEval W (a - _) = 0
    rw [map_sub, local_origin_constant]
    exact sub_self c
  have hb : b ∈ IsLocalRing.maximalIdeal (ChartLocalRing W) := by
    rw [← local_origin_kernel]
    exact hb0
  have hbv : chartLocalEquiv W b ∈ IsLocalRing.maximalIdeal (valuationRing W) := by
    rw [← IsLocalRing.map_ringEquiv_maximalIdeal
      (R := ChartLocalRing W) (S := valuationRing W) (chartLocalEquiv W)]
    exact Ideal.mem_map_of_mem (chartLocalEquiv W).toRingHom hb
  have hv := (Valuation.mem_maximalIdeal_iff (v := functionValuation W)).mp hbv
  have he : localToField W b = f - constantHom W c := by
    change localToField W (a - _) = _
    rw [RingHom.map_sub (α := ChartLocalRing W) (β := W.FunctionField) (localToField W) a
      (algebraMap (ChartRing W) (ChartLocalRing W) (algebraMap K (ChartRing W) c)),
      ha, local_to_field_map, chart_to_field_constant]
  refine ⟨c, ?_⟩
  change functionValuation W (localToField W b) < 1 at hv
  rwa [he] at hv

#print axioms infinity_residue_constant
end
end GeometricPrincipalDivisorAudit
