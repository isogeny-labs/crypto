module
public import ChartAlgebra

@[expose] public section
namespace MillerInfinityChartAudit
noncomputable section
open MillerNormValuationAudit
variable {K : Type*} [Field K]

def coefficientResidueEquiv (W : WeierstrassCurve.Affine K) :
    K ≃+* (coefficientResidue W).range :=
  RingEquiv.ofBijective (coefficientResidue W).rangeRestrict
    ⟨fun _ _ h => (coefficientResidue W).injective (congrArg Subtype.val h),
      (coefficientResidue W).rangeRestrict_surjective⟩

def originResidue (W : WeierstrassCurve.Affine K) :
    ChartRing W →+* IsLocalRing.ResidueField (valuationRing W) :=
  (IsLocalRing.residue (valuationRing W)).comp (chartAlgebra W).val.toRingHom

def reducedOrigin (W : WeierstrassCurve.Affine K) : ChartRing W →+* (coefficientResidue W).range :=
  (originResidue W).codRestrict _ (fun b => chart_residue_constant W b.property)

/-- Actual evaluation at the chart origin, derived from the actual residue map. -/
def originEval (W : WeierstrassCurve.Affine K) : ChartRing W →+* K :=
  (coefficientResidueEquiv W).symm.toRingHom.comp (reducedOrigin W)

theorem residue_originEval (W : WeierstrassCurve.Affine K) (b : ChartRing W) :
    coefficientResidue W (originEval W b) = originResidue W b := by
  change ((coefficientResidueEquiv W)
    ((coefficientResidueEquiv W).symm (reducedOrigin W b)) :
      IsLocalRing.ResidueField (valuationRing W)) = _
  rw [RingEquiv.apply_symm_apply]
  rfl

theorem origin_eval_constant (W : WeierstrassCurve.Affine K) (c : K) :
    originEval W (algebraMap K (ChartRing W) c) = c := by
  apply (coefficientResidue W).injective
  rw [residue_originEval]
  rfl

theorem origin_eval_t (W : WeierstrassCurve.Affine K) : originEval W (tCoord W) = 0 := by
  apply (coefficientResidue W).injective
  rw [residue_originEval, map_zero]
  exact residue_chartT W

theorem origin_eval_z (W : WeierstrassCurve.Affine K) : originEval W (zCoord W) = 0 := by
  apply (coefficientResidue W).injective
  rw [residue_originEval, map_zero]
  exact residue_chartZ W

theorem origin_eval_surjective (W : WeierstrassCurve.Affine K) :
    Function.Surjective (originEval W) :=
  fun c => ⟨algebraMap K (ChartRing W) c, origin_eval_constant W c⟩

def originIdeal (W : WeierstrassCurve.Affine K) : Ideal (ChartRing W) :=
  RingHom.ker (originEval W)

instance originMaximal (W : WeierstrassCurve.Affine K) : (originIdeal W).IsMaximal :=
  RingHom.ker_isMaximal_of_surjective _ (origin_eval_surjective W)

def coordinateIdeal (W : WeierstrassCurve.Affine K) : Ideal (ChartRing W) :=
  Ideal.span {tCoord W, zCoord W}

theorem coordinate_ideal_le_origin (W : WeierstrassCurve.Affine K) :
    coordinateIdeal W ≤ originIdeal W := by
  apply Ideal.span_le.mpr
  intro a ha
  simp only [Set.mem_insert_iff, Set.mem_singleton_iff] at ha
  rcases ha with rfl | rfl
  · exact origin_eval_t W
  · exact origin_eval_z W

/-- Every actual chart function is congruent to its actual origin value modulo (t,z). -/
theorem chart_congr_origin (W : WeierstrassCurve.Affine K) (b : ChartRing W) :
    b - algebraMap K (ChartRing W) (originEval W b) ∈ coordinateIdeal W := by
  have hb : b ∈ Algebra.adjoin K {tCoord W, zCoord W} := by
    rw [chart_internal_generation]
    trivial
  induction hb using Algebra.adjoin_induction with
  | mem a ha =>
    simp only [Set.mem_insert_iff, Set.mem_singleton_iff] at ha
    rcases ha with rfl | rfl
    · rw [origin_eval_t, map_zero, sub_zero]
      exact Ideal.subset_span (by simp)
    · rw [origin_eval_z, map_zero, sub_zero]
      exact Ideal.subset_span (by simp)
  | algebraMap c =>
    rw [origin_eval_constant, sub_self]
    exact Ideal.zero_mem _
  | add a b ha hb ih₁ ih₂ =>
    have he : a + b - algebraMap K (ChartRing W) (originEval W (a + b)) =
        (a - algebraMap K (ChartRing W) (originEval W a)) +
        (b - algebraMap K (ChartRing W) (originEval W b)) := by
      rw [map_add, map_add]
      ring
    rw [he]
    exact Ideal.add_mem _ ih₁ ih₂
  | mul a b ha hb ih₁ ih₂ =>
    have he : a * b - algebraMap K (ChartRing W) (originEval W (a * b)) =
        (a - algebraMap K (ChartRing W) (originEval W a)) * b +
        algebraMap K (ChartRing W) (originEval W a) *
          (b - algebraMap K (ChartRing W) (originEval W b)) := by
      rw [map_mul, map_mul]
      ring
    rw [he]
    exact Ideal.add_mem _ (Ideal.mul_mem_right _ _ ih₁) (Ideal.mul_mem_left _ _ ih₂)

theorem origin_ideal_coordinates (W : WeierstrassCurve.Affine K) :
    originIdeal W = Ideal.span {tCoord W, zCoord W} := by
  apply le_antisymm
  · intro b hb
    have h := chart_congr_origin W b
    have h0 : originEval W b = 0 := hb
    simpa only [coordinateIdeal, h0, map_zero, sub_zero] using h
  · exact coordinate_ideal_le_origin W

/-- The derived origin map is the unique homomorphism with these coordinate/constant values. -/
theorem origin_eval_unique (W : WeierstrassCurve.Affine K) (f : ChartRing W →+* K)
    (hc : ∀ c : K, f (algebraMap K (ChartRing W) c) = c)
    (ht : f (tCoord W) = 0) (hz : f (zCoord W) = 0) : f = originEval W := by
  ext b
  have hb : b ∈ Algebra.adjoin K {tCoord W, zCoord W} := by
    rw [chart_internal_generation]
    trivial
  induction hb using Algebra.adjoin_induction with
  | mem a ha =>
    simp only [Set.mem_insert_iff, Set.mem_singleton_iff] at ha
    rcases ha with rfl | rfl
    · rw [ht, origin_eval_t]
    · rw [hz, origin_eval_z]
  | algebraMap c => rw [hc, origin_eval_constant]
  | add a b ha hb ih₁ ih₂ => rw [map_add, map_add, ih₁, ih₂]
  | mul a b ha hb ih₁ ih₂ => rw [map_mul, map_mul, ih₁, ih₂]

#print axioms origin_ideal_coordinates
#print axioms origin_eval_unique
end
end MillerInfinityChartAudit
