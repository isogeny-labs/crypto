module
public import ResidueNormalization
public import Mathlib.RingTheory.Valuation.Discrete.Basic

@[expose] public section
namespace MillerNormValuationAudit
noncomputable section
open scoped WithZero
variable {K : Type*} [Field K]

/-- The actual parameter generates every nonzero valuation value. -/
theorem function_valuation_surjective (W : WeierstrassCurve.Affine K) :
    Function.Surjective (functionValuation W) := by
  intro a
  refine WithZero.expRecOn a ?_ (fun n => ?_)
  · exact ⟨0, map_zero _⟩
  · refine ⟨parameter W ^ (-n), ?_⟩
    rw [map_zpow₀, function_valuation_parameter, ← WithZero.exp_zsmul]
    congr 1
    simp

instance valueGroupNontrivial (W : WeierstrassCurve.Affine K) :
    Nontrivial (functionValuation W).valueGroup := by
  rw [Subgroup.nontrivial_iff_exists_ne_one]
  refine ⟨Units.mk0 (WithZero.exp (-1 : ℤ)) WithZero.exp_ne_zero, ?_, ?_⟩
  · apply MonoidWithZeroHom.mem_valueGroup
    exact ⟨parameter W, function_valuation_parameter W⟩
  · intro h
    have he := congrArg (fun u : ℤᵐ⁰ˣ => (u : ℤᵐ⁰)) h
    norm_num at he

instance valuationRingDVR (W : WeierstrassCurve.Affine K) :
    IsDiscreteValuationRing (valuationRing W) :=
  inferInstanceAs (IsDiscreteValuationRing (functionValuation W).valuationSubring)

theorem parameter_is_uniformizer (W : WeierstrassCurve.Affine K) :
    Valuation.IsUniformizer (functionValuation W) (parameter W) := by
  rw [Valuation.IsUniformizer.iff,
    Valuation.IsRankOneDiscrete.generator_eq_exp_neg_one_of_surjective
      (function_valuation_surjective W)]
  exact function_valuation_parameter W

def integralParameter (W : WeierstrassCurve.Affine K) : valuationRing W :=
  ⟨parameter W, by
    change functionValuation W (parameter W) ≤ 1
    rw [function_valuation_parameter]
    norm_num⟩

theorem parameter_generates_maximal (W : WeierstrassCurve.Affine K) :
    IsLocalRing.maximalIdeal (valuationRing W) = Ideal.span {integralParameter W} :=
  Valuation.IsUniformizer.is_generator (parameter_is_uniformizer W)

theorem reciprocal_x_not_uniformizer (W : WeierstrassCurve.Affine K) :
    ¬ Valuation.IsUniformizer (functionValuation W) (xFunction W)⁻¹ := by
  intro h
  exact (reciprocal_x_control W).2 (h.val.trans (parameter_is_uniformizer W).val.symm)

#print axioms parameter_is_uniformizer
#print axioms parameter_generates_maximal
#print axioms reciprocal_x_not_uniformizer
end
end MillerNormValuationAudit
