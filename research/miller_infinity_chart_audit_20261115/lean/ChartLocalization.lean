module
public import OriginEvaluation
public import Mathlib.RingTheory.Localization.Submodule

@[expose] public section
namespace MillerInfinityChartAudit
noncomputable section
open MillerNormValuationAudit
variable {K : Type*} [Field K]

abbrev ChartLocalRing (W : WeierstrassCurve.Affine K) : Type _ :=
  Localization.AtPrime (originIdeal W)

instance chartLocalDomain (W : WeierstrassCurve.Affine K) : IsDomain (ChartLocalRing W) :=
  IsLocalization.isDomain_of_local_atPrime (inferInstanceAs (originIdeal W).IsPrime)

def localT (W : WeierstrassCurve.Affine K) : ChartLocalRing W :=
  algebraMap (ChartRing W) (ChartLocalRing W) (tCoord W)

def localZ (W : WeierstrassCurve.Affine K) : ChartLocalRing W :=
  algebraMap (ChartRing W) (ChartLocalRing W) (zCoord W)

def bracket (W : WeierstrassCurve.Affine K) : ChartRing W :=
  1 + algebraMap K (ChartRing W) W.a₁ * tCoord W +
    algebraMap K (ChartRing W) W.a₃ * zCoord W -
    algebraMap K (ChartRing W) W.a₂ * tCoord W ^ 2 -
    algebraMap K (ChartRing W) W.a₄ * tCoord W * zCoord W -
    algebraMap K (ChartRing W) W.a₆ * zCoord W ^ 2

def localBracket (W : WeierstrassCurve.Affine K) : ChartLocalRing W :=
  algebraMap (ChartRing W) (ChartLocalRing W) (bracket W)

theorem origin_eval_bracket (W : WeierstrassCurve.Affine K) : originEval W (bracket W) = 1 := by
  simp [bracket, origin_eval_t, origin_eval_z, origin_eval_constant]

theorem bracket_not_origin (W : WeierstrassCurve.Affine K) : bracket W ∉ originIdeal W := by
  change originEval W (bracket W) ≠ 0
  rw [origin_eval_bracket]
  exact one_ne_zero

/-- The source bracket is a unit because its actual origin value is one. -/
theorem local_bracket_unit (W : WeierstrassCurve.Affine K) : IsUnit (localBracket W) :=
  (IsLocalization.AtPrime.isUnit_to_map_iff (ChartLocalRing W) (originIdeal W) (bracket W)).mpr
    (bracket_not_origin W)

theorem local_chart_relation (W : WeierstrassCurve.Affine K) :
    localZ W * localBracket W = localT W ^ 3 := by
  have h : zCoord W * bracket W = tCoord W ^ 3 := chart_relation W
  have hm := congrArg (algebraMap (ChartRing W) (ChartLocalRing W)) h
  simpa only [localZ, localBracket, localT, map_mul, map_pow] using hm

theorem local_z_in_t (W : WeierstrassCurve.Affine K) :
    localZ W ∈ Ideal.span {localT W} := by
  have hp : localT W ^ 3 ∈ Ideal.span {localT W} :=
    Ideal.mem_span_singleton.mpr (dvd_pow_self _ (by decide : (3 : ℕ) ≠ 0))
  have h : localZ W * localBracket W ∈ Ideal.span {localT W} := by
    rw [local_chart_relation]
    exact hp
  exact (Ideal.mul_unit_mem_iff_mem _ (local_bracket_unit W)).mp h

theorem local_maximal_generated (W : WeierstrassCurve.Affine K) :
    IsLocalRing.maximalIdeal (ChartLocalRing W) = Ideal.span {localT W} := by
  rw [← Localization.AtPrime.map_eq_maximalIdeal]
  have he := congrArg (Ideal.map (algebraMap (ChartRing W) (ChartLocalRing W)))
    (origin_ideal_coordinates W)
  rw [he, Ideal.map_span, Set.image_pair]
  change Ideal.span {localT W, localZ W} = Ideal.span {localT W}
  apply le_antisymm
  · apply Ideal.span_le.mpr
    intro a ha
    simp only [Set.mem_insert_iff, Set.mem_singleton_iff] at ha
    rcases ha with rfl | rfl
    · exact Ideal.subset_span (by simp)
    · exact local_z_in_t W
  · apply Ideal.span_mono
    intro a ha
    rcases Set.mem_singleton_iff.mp ha with rfl
    simp

theorem t_coord_nonzero (W : WeierstrassCurve.Affine K) : tCoord W ≠ 0 := by
  intro h
  have hf := congrArg (fun b : ChartRing W => ((b : valuationRing W) : W.FunctionField)) h
  exact (parameter_is_uniformizer W).ne_zero hf

theorem local_t_nonzero (W : WeierstrassCurve.Affine K) : localT W ≠ 0 := by
  have hi := IsLocalization.injective (S := ChartLocalRing W)
    (originIdeal W).primeCompl_le_nonZeroDivisors
  intro h
  apply t_coord_nonzero W
  apply hi
  simpa only [localT, map_zero] using h

theorem local_maximal_nonzero (W : WeierstrassCurve.Affine K) :
    IsLocalRing.maximalIdeal (ChartLocalRing W) ≠ ⊥ := by
  intro h
  have ht : localT W ∈ IsLocalRing.maximalIdeal (ChartLocalRing W) := by
    rw [local_maximal_generated]
    exact Ideal.subset_span (by simp)
  rw [h] at ht
  exact local_t_nonzero W ht

/-- The actual localization at the chart origin is a DVR, with no DVR premise supplied. -/
instance chartLocalDVR (W : WeierstrassCurve.Affine K) : IsDiscreteValuationRing (ChartLocalRing W) where
  toIsPrincipalIdealRing := by
    have hp : (IsLocalRing.maximalIdeal (ChartLocalRing W)).IsPrincipal := by
      rw [local_maximal_generated]
      infer_instance
    exact ((tfae_of_isNoetherianRing_of_isLocalRing_of_isDomain (ChartLocalRing W)).out 5 1).mp hp
  toIsLocalRing := inferInstance
  not_a_field' := local_maximal_nonzero W

#print axioms local_maximal_generated
#print axioms local_t_nonzero
end
end MillerInfinityChartAudit
