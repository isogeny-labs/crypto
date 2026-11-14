import CubicTraceCriterion

namespace FiniteCharThreeTraceAudit
noncomputable section
open BKThreeNormalizationAudit QuadraticTwistCountAudit SmallPrimeTraceGeometryAudit
open SupersingularTraceClassesAudit ThreeTorsionRankAudit.General
variable {K : Type*} [Field K]

def squareCompleted (W : WeierstrassCurve.Affine K) : WeierstrassCurve.Affine K :=
  shifted W 0 (-W.a₁/2) (-W.a₃/2)

theorem completed_delta (W : WeierstrassCurve.Affine K) : (squareCompleted W).Δ=W.Δ :=
  shifted_discriminant W _ _ _

theorem completed_b2 (W : WeierstrassCurve.Affine K) : (squareCompleted W).b₂=W.b₂ := by
  simp only [squareCompleted,shifted,WeierstrassCurve.b₂]
  ring

def completedEquiv [DecidableEq K] (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) :
    (squareCompleted W).Point ≃+ W.Point := shiftEquiv W hΔ _ _ _

theorem completed_card (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) :
    Nat.card (squareCompleted W).Point=Nat.card W.Point := by
  classical
  exact Nat.card_congr (completedEquiv W hΔ).toEquiv

theorem completed_source [Fintype K] (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) :
    sourceSupersingular (squareCompleted W) ↔ sourceSupersingular W := by
  simp only [sourceSupersingular,fieldTrace,completed_card W hΔ]

variable [CharP K 3]

theorem completed_shape (W : WeierstrassCurve.Affine K) :
    squareCompleted W=cubicModel (squareCompleted W).a₂ (squareCompleted W).a₄ (squareCompleted W).a₆ := by
  ext <;> dsimp [squareCompleted,shifted,cubicModel]
  all_goals field_simp [char_three_two_ne_zero (K:=K)]
  all_goals ring

theorem completed_a2_b2 (W : WeierstrassCurve.Affine K) : (squareCompleted W).a₂=W.b₂ := by
  have h := completed_b2 W
  rw [completed_shape W] at h
  simpa [cubicModel,WeierstrassCurve.b₂,char_three_four_eq_one] using h

variable [Fintype K] [DecidableEq K]

theorem source_iff_b2_zero (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) :
    sourceSupersingular W ↔ W.b₂=0 := by
  have hc : (squareCompleted W).Δ≠0 := by rwa [completed_delta]
  have ht := cubic_source_iff_a_zero (squareCompleted W).a₂ (squareCompleted W).a₄
    (squareCompleted W).a₆ (by rwa [←completed_shape W])
  rw [←completed_shape W] at ht
  calc
    sourceSupersingular W ↔ sourceSupersingular (squareCompleted W) := (completed_source W hΔ).symm
    _ ↔ (squareCompleted W).a₂=0 := ht
    _ ↔ W.b₂=0 := by rw [completed_a2_b2]

theorem source_iff_j_zero (W : WeierstrassCurve.Affine K) [W.IsElliptic] :
    sourceSupersingular W ↔ W.j=0 := by
  rw [W.j_eq_zero_iff_of_char_three]
  exact source_iff_b2_zero W (SmallCharacteristicJAudit.delta_nonzero W)

theorem source_implies_rational_three_card_one (W : WeierstrassCurve.Affine K)
    (hΔ : W.Δ≠0) (hs : sourceSupersingular W) : Nat.card (ThreeTorsion W)=1 :=
  SmallCharacteristicJAudit.card_one_when_b2_zero W hΔ ((source_iff_b2_zero W hΔ).mp hs)

section Geometric
variable {L : Type*} [Field L] [Algebra K L] [DecidableEq L] [IsAlgClosed L]

theorem source_iff_geometric_three_card_one (W : WeierstrassCurve.Affine K) [W.IsElliptic] :
    sourceSupersingular W ↔ Nat.card (ThreeTorsion (W.baseChange L))=1 := by
  rw [source_iff_j_zero W,SmallCharacteristicJAudit.three_geometric_j W]
  by_cases hj : W.j=0 <;> simp [hj]

omit [Fintype K] in
theorem geometric_three_rank (W : WeierstrassCurve.Affine K) [W.IsElliptic] :
    Module.finrank (ZMod 3) (ThreeTorsion (W.baseChange L))=if W.j=0 then 0 else 1 := by
  have : CharP L 3 := charP_of_injective_algebraMap' K 3
  have : (W.baseChange L).IsElliptic := SmallCharacteristicJAudit.ellipticOfNonzero _
    (base_discriminant_nonzero W (SmallCharacteristicJAudit.delta_nonzero W))
  have hj : (W.baseChange L).j=0 ↔ W.j=0 := by
    change (W.map (algebraMap K L)).j=0 ↔ W.j=0
    rw [WeierstrassCurve.map_j]
    exact map_eq_zero (algebraMap K L)
  simpa only [hj] using SmallCharacteristicJAudit.three_j_finrank (W.baseChange L)

theorem source_iff_geometric_three_rank_zero (W : WeierstrassCurve.Affine K) [W.IsElliptic] :
    sourceSupersingular W ↔ Module.finrank (ZMod 3) (ThreeTorsion (W.baseChange L))=0 := by
  rw [source_iff_j_zero W,geometric_three_rank W]
  by_cases hj : W.j=0 <;> simp [hj]
end Geometric

section FiniteExtension
variable {L : Type*} [Field L] [Fintype L] [Algebra K L] [DecidableEq L]

theorem finite_extension_source_iff (W : WeierstrassCurve.Affine K) [W.IsElliptic] :
    sourceSupersingular (W.baseChange L) ↔ sourceSupersingular W := by
  have : CharP L 3 := charP_of_injective_algebraMap' K 3
  have : (W.baseChange L).IsElliptic := SmallCharacteristicJAudit.ellipticOfNonzero _
    (base_discriminant_nonzero W (SmallCharacteristicJAudit.delta_nonzero W))
  rw [source_iff_j_zero,source_iff_j_zero]
  change (W.map (algebraMap K L)).j=0 ↔ W.j=0
  rw [WeierstrassCurve.map_j]
  exact map_eq_zero (algebraMap K L)
end FiniteExtension

#print axioms source_iff_b2_zero
#print axioms source_iff_j_zero
#print axioms source_iff_geometric_three_card_one
#print axioms source_iff_geometric_three_rank_zero
#print axioms finite_extension_source_iff
end
end FiniteCharThreeTraceAudit
