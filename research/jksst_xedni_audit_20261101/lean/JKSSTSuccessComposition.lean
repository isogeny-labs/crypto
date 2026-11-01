import JKSSTCoefficientProbability
import JKSSTConditionalHeightCore

/-!
# Composing the conditional relation bound with the exact success probability

The deterministic height argument produces a bounded nonzero relation in a
group.  The probability argument needs more: after reduction, that relation
must be a scalar relation for the sampled ECDLP points and its denominator
must be nonzero.  This file exposes that missing bridge as a proposition and
then composes the two sides without adding arithmetic assumptions.
-/

open scoped BigOperators

namespace JKSSTSuccessComposition

open JKSSTCoefficientBox JKSSTCoefficientProbability JKSSTProbability

variable {F ι : Type*} [Field F] [Fintype F] [Fintype ι]

/-- The uniform finite sample used by the probability theorem: two vectors of
scalars, conventionally the sampled `s` and `t` coefficients. -/
abbrev Sample (F ι : Type*) := (ι → F) × (ι → F)

/-- The exact deterministic output promised by the repaired height core. -/
def BoundedGroupRelationAt
    {G : Type*} [AddCommGroup G]
    (P : ι → G) (C : ℕ) (n : ι → ℤ) : Prop :=
  n ≠ 0 ∧ (∀ i, |n i| ≤ (C : ℤ)) ∧ ∑ i, n i • P i = 0

/-- Reduction of a group relation to the scalar equation over the finite
field.  This ordinary relation is enough for the paper's probability upper
bound; it need not reveal the discrete logarithm. -/
def ScalarReductionAt
    (x : F) (z : Sample F ι) (n : ι → ℤ) : Prop :=
  dotLinear (reduceIntVector (F := F) n) z.1 =
    x * dotLinear (reduceIntVector (F := F) n) z.2

/-- The additional usefulness property needed for ECDLP recovery: besides
the scalar relation, the division denominator must be nonzero. -/
def UsefulReductionAt
    (x : F) (z : Sample F ι) (n : ι → ℤ) : Prop :=
  ScalarReductionAt x z n ∧
    dotLinear (reduceIntVector (F := F) n) z.2 ≠ 0

omit [Fintype F] in
/-- The usefulness bridge is genuinely additional: at the all-zero sample,
every possible denominator is zero. -/
theorem no_useful_reduction_at_zero_sample
    (x : F) (n : ι → ℤ) :
    ¬ UsefulReductionAt x (0 : Sample F ι) n := by
  intro h
  exact h.2 (by simp [dotLinear_apply])

/-! ## Ordinary paired relation event (`1/q`) -/

/-- All output pairs `(A,B)` satisfying the scalar relation `A=xB`. -/
noncomputable def scalarRelationOutputs (x : F) : Finset (F × F) := by
  classical
  exact Finset.univ.filter fun y ↦ y.1 = x * y.2

theorem mem_scalarRelationOutputs_iff (x : F) (y : F × F) :
    y ∈ scalarRelationOutputs x ↔ y.1 = x * y.2 := by
  classical
  simp [scalarRelationOutputs]

theorem card_scalarRelationOutputs (x : F) :
    (scalarRelationOutputs x).card = Fintype.card F := by
  classical
  let e : ↑(scalarRelationOutputs x) ≃ F :=
    { toFun := fun y ↦ y.1.2
      invFun := fun b ↦ ⟨(x * b, b),
        (mem_scalarRelationOutputs_iff x _).mpr rfl⟩
      left_inv := by
        rintro ⟨⟨A, B⟩, hy⟩
        apply Subtype.ext
        have hAB := (mem_scalarRelationOutputs_iff x (A, B)).mp hy
        exact Prod.ext hAB.symm rfl
      right_inv := fun _ ↦ rfl }
  calc
    (scalarRelationOutputs x).card = Nat.card ↑(scalarRelationOutputs x) := by simp
    _ = Nat.card F := Nat.card_congr e
    _ = Fintype.card F := Nat.card_eq_fintype_card

/-- Paired samples satisfying the ordinary relation for coefficient vector
`m`, without imposing denominator usefulness. -/
noncomputable def scalarRelationEvent (x : F) (m : ι → F) :
    Finset (Sample F ι) := by
  classical
  exact Finset.univ.filter fun z ↦ pairDot m z ∈ scalarRelationOutputs x

theorem mem_scalarRelationEvent_iff (x : F) (m : ι → F)
    (z : Sample F ι) :
    z ∈ scalarRelationEvent x m ↔
      dotLinear m z.1 = x * dotLinear m z.2 := by
  classical
  simp [scalarRelationEvent, pairDot, mem_scalarRelationOutputs_iff]

/-- For fixed nonzero `m`, the ordinary paired relation has exact probability
`1/q`. -/
theorem card_scalarRelationEvent {m : ι → F} (hm : m ≠ 0) (x : F) :
    (scalarRelationEvent x m).card =
      Fintype.card F *
        (Fintype.card F ^ (Fintype.card ι - 1)) ^ 2 := by
  classical
  unfold scalarRelationEvent
  rw [card_filter_preimage_of_constant_fiber
    (pairDot m) (scalarRelationOutputs x)
    ((Fintype.card F ^ (Fintype.card ι - 1)) ^ 2)]
  · rw [card_scalarRelationOutputs]
  · intro y _hy
    exact card_pairDotFiber hm y

noncomputable def distinctScalarRelationEvents
    (x : F) (R : Finset (ι → F)) : Finset (Finset (Sample F ι)) := by
  classical
  exact R.image (scalarRelationEvent x)

noncomputable def badScalarRelationSamples
    (x : F) (R : Finset (ι → F)) : Finset (Sample F ι) := by
  classical
  exact (distinctScalarRelationEvents x R).biUnion id

theorem card_distinctScalarRelationEvents_le
    (x : F) (R : Finset (ι → F)) :
    (distinctScalarRelationEvents x R).card ≤ R.card := by
  classical
  unfold distinctScalarRelationEvents
  exact Finset.card_image_le

theorem card_badScalarRelationSamples_le
    (x : F) (R : Finset (ι → F)) (hR : ∀ m ∈ R, m ≠ 0) :
    (badScalarRelationSamples x R).card ≤
      (distinctScalarRelationEvents x R).card *
        (Fintype.card F *
          (Fintype.card F ^ (Fintype.card ι - 1)) ^ 2) := by
  classical
  apply Finset.card_biUnion_le_card_mul
  intro E hE
  rw [distinctScalarRelationEvents, Finset.mem_image] at hE
  obtain ⟨m, hmR, rfl⟩ := hE
  exact (card_scalarRelationEvent (hR m hmR) x).le

/-- Cross-multiplied exact union bound `Pr[ordinary relation] ≤ N/q`. -/
theorem card_badScalarRelationSamples_mul_card_field_le
    (x : F) (R : Finset (ι → F)) (hR : ∀ m ∈ R, m ≠ 0) :
    (badScalarRelationSamples x R).card * Fintype.card F ≤
      (distinctScalarRelationEvents x R).card *
        (Fintype.card F ^ Fintype.card ι) ^ 2 := by
  classical
  by_cases hEmpty : R = ∅
  · subst R
    simp [badScalarRelationSamples, distinctScalarRelationEvents]
  · obtain ⟨m, hmR⟩ := Finset.nonempty_iff_ne_empty.mpr hEmpty
    obtain ⟨i, _hi⟩ : ∃ i, m i ≠ 0 := by
      simpa [funext_iff] using hR m hmR
    have hι : 1 ≤ Fintype.card ι := Fintype.card_pos_iff.mpr ⟨i⟩
    have hk :
        Fintype.card F ^ (Fintype.card ι - 1) * Fintype.card F =
          Fintype.card F ^ Fintype.card ι := by
      rw [← pow_succ]
      congr 1
      omega
    calc
      (badScalarRelationSamples x R).card * Fintype.card F ≤
          ((distinctScalarRelationEvents x R).card *
            (Fintype.card F *
              (Fintype.card F ^ (Fintype.card ι - 1)) ^ 2)) *
                Fintype.card F :=
        Nat.mul_le_mul_right _ (card_badScalarRelationSamples_le x R hR)
      _ = (distinctScalarRelationEvents x R).card *
            (Fintype.card F ^ Fintype.card ι) ^ 2 := by
        rw [pow_two, pow_two, ← hk]
        ring

/-- The existing conditional height theorem, repackaged in the exact relation
interface used below.  Every Mordell--Weil, radius, separation, and torsion
hypothesis remains visible. -/
theorem boundedGroupRelation_of_certified_height_ratio
    {G E : Type*} [AddCommGroup G]
    [NormedAddCommGroup E] [NormedSpace ℝ E] [FiniteDimensional ℝ E]
    (P : ι → G) (T : ℕ) (T_pos : 0 < T)
    (rank_gap : Module.finrank ℝ E + 1 ≤ Fintype.card ι)
    (M m K R : ℝ) (M_nonneg : 0 ≤ M) (m_pos : 0 < m)
    (ratio_bound : M / m ≤ K) (hR : 0 ≤ R)
    (radius_bound : R ≤
      (Fintype.card ι : ℝ) *
        (JKSSTConditionalHeightCore.coefficientCutoff
          T (Module.finrank ℝ E) (Fintype.card ι) K : ℕ) *
        Real.sqrt M)
    (S : Finset G) (s : Finset E) (π : G →+ E)
    (range_mem : ∀ c : ι → Fin
        (JKSSTConditionalHeightCore.coefficientCutoff
          T (Module.finrank ℝ E) (Fintype.card ι) K + 1),
      (∑ i, (c i : ℤ) • P i) ∈ S)
    (mapsTo : ∀ g ∈ S, π g ∈ s)
    (fiberBound : ∀ y ∈ s, {g : G | g ∈ S ∧ π g = y}.ncard ≤ T)
    (inside : ∀ y ∈ s, ‖y‖ ≤ R)
    (separated : ∀ y ∈ s, ∀ z ∈ s, y ≠ z →
      Real.sqrt m ≤ ‖y - z‖) :
    ∃ n : ι → ℤ,
      BoundedGroupRelationAt P
        (JKSSTConditionalHeightCore.coefficientCutoff
          T (Module.finrank ℝ E) (Fintype.card ι) K) n := by
  simpa [BoundedGroupRelationAt] using
    (JKSSTConditionalHeightCore.bounded_relation_of_certified_height_ratio
      P T T_pos rank_gap M m K R M_nonneg m_pos ratio_bound hR
      radius_bound S s π range_mem mapsTo fiberBound inside separated)

/-- A reduction homomorphism/cyclic-subgroup certificate turns the bounded
group relation supplied by the height core into an ordinary scalar relation.
No nonzero-denominator claim is needed for this containment. -/
theorem success_subset_badScalarRelationSamples_of_certificates
    {G : Type*} [AddCommGroup G]
    (x : F) (C : ℕ) (success : Finset (Sample F ι))
    (P : Sample F ι → ι → G)
    (relationCertificate : ∀ z ∈ success,
      ∃ n : ι → ℤ, BoundedGroupRelationAt (P z) C n)
    (scalarReductionBridge : ∀ z ∈ success, ∀ n : ι → ℤ,
      BoundedGroupRelationAt (P z) C n → ScalarReductionAt x z n) :
    success ⊆ badScalarRelationSamples x
      (reducedCandidates (F := F) (ι := ι) C) := by
  classical
  intro z hz
  obtain ⟨n, hn⟩ := relationCertificate z hz
  have hnBox : n ∈ nonzeroCoefficientBox (ι := ι) C := by
    rw [mem_nonzeroCoefficientBox_iff]
    exact ⟨hn.1, hn.2.1⟩
  have hmCandidate : reduceIntVector (F := F) n ∈
      reducedCandidates (F := F) (ι := ι) C := by
    rw [reducedCandidates, Finset.mem_image]
    exact ⟨n, hnBox, rfl⟩
  have hScalar : z ∈
      scalarRelationEvent x (reduceIntVector (F := F) n) := by
    rw [mem_scalarRelationEvent_iff]
    exact scalarReductionBridge z hz n hn
  rw [badScalarRelationSamples, Finset.mem_biUnion]
  refine ⟨scalarRelationEvent x (reduceIntVector (F := F) n), ?_, hScalar⟩
  rw [distinctScalarRelationEvents, Finset.mem_image]
  exact ⟨reduceIntVector (F := F) n, hmCandidate, rfl⟩

/-- What the repaired height core can support without a usefulness theorem:
an exact ordinary-relation bound `((2C+1)^r-1)/q`. -/
theorem ordinary_success_bound_of_group_relation_certificates
    {G : Type*} [AddCommGroup G]
    (x : F) (C : ℕ) (hC : C < ringChar F)
    (success : Finset (Sample F ι))
    (P : Sample F ι → ι → G)
    (relationCertificate : ∀ z ∈ success,
      ∃ n : ι → ℤ, BoundedGroupRelationAt (P z) C n)
    (scalarReductionBridge : ∀ z ∈ success, ∀ n : ι → ℤ,
      BoundedGroupRelationAt (P z) C n → ScalarReductionAt x z n) :
    success.card * Fintype.card F ≤
      ((2 * C + 1) ^ Fintype.card ι - 1) *
        (Fintype.card F ^ Fintype.card ι) ^ 2 := by
  classical
  let candidates := reducedCandidates (F := F) (ι := ι) C
  have hSubset : success ⊆ badScalarRelationSamples x candidates := by
    exact success_subset_badScalarRelationSamples_of_certificates
      x C success P relationCertificate scalarReductionBridge
  have hNonzero : ∀ v ∈ candidates, v ≠ 0 := by
    exact reducedCandidates_ne_zero C hC
  have hEventCount :
      (distinctScalarRelationEvents x candidates).card ≤
        (2 * C + 1) ^ Fintype.card ι - 1 := by
    calc
      (distinctScalarRelationEvents x candidates).card ≤ candidates.card :=
        card_distinctScalarRelationEvents_le x candidates
      _ ≤ (nonzeroCoefficientBox (ι := ι) C).card := by
        dsimp [candidates, reducedCandidates]
        exact Finset.card_image_le
      _ = (2 * C + 1) ^ Fintype.card ι - 1 :=
        card_nonzeroCoefficientBox C
  calc
    success.card * Fintype.card F ≤
        (badScalarRelationSamples x candidates).card * Fintype.card F :=
      Nat.mul_le_mul_right _ (Finset.card_le_card hSubset)
    _ ≤ (distinctScalarRelationEvents x candidates).card *
          (Fintype.card F ^ Fintype.card ι) ^ 2 :=
      card_badScalarRelationSamples_mul_card_field_le x candidates hNonzero
    _ ≤ ((2 * C + 1) ^ Fintype.card ι - 1) *
          (Fintype.card F ^ Fintype.card ι) ^ 2 :=
      Nat.mul_le_mul_right _ hEventCount

/-- Explicit size-comparison `/p` consequence of the ordinary relation bound. -/
theorem ordinary_success_bound_over_base_of_group_relation_certificates
    {G : Type*} [AddCommGroup G]
    (x : F) (C : ℕ) (hC : C < ringChar F)
    (success : Finset (Sample F ι))
    (P : Sample F ι → ι → G)
    (relationCertificate : ∀ z ∈ success,
      ∃ n : ι → ℤ, BoundedGroupRelationAt (P z) C n)
    (scalarReductionBridge : ∀ z ∈ success, ∀ n : ι → ℤ,
      BoundedGroupRelationAt (P z) C n → ScalarReductionAt x z n)
    (p h : ℕ) (_p_pos : 0 < p)
    (sizeComparison : p ≤ h * Fintype.card F) :
    success.card * p ≤
      (h * ((2 * C + 1) ^ Fintype.card ι - 1)) *
        (Fintype.card F ^ Fintype.card ι) ^ 2 := by
  apply div_card_field_to_base_with_size_factor
    (q := Fintype.card F)
    (ordinary_success_bound_of_group_relation_certificates
      x C hC success P relationCertificate scalarReductionBridge)
    sizeComparison

/-- If every successful sample produces a bounded group relation and a
certified useful reduction, then successful samples lie in the precise union
of useful finite-field relation events. -/
theorem success_subset_badUsefulSamples_of_certificates
    {G : Type*} [AddCommGroup G]
    (x : F) (C : ℕ) (success : Finset (Sample F ι))
    (P : Sample F ι → ι → G)
    (relationCertificate : ∀ z ∈ success,
      ∃ n : ι → ℤ, BoundedGroupRelationAt (P z) C n)
    (reductionBridge : ∀ z ∈ success,
      (∃ n : ι → ℤ, BoundedGroupRelationAt (P z) C n) →
      ∃ n : ι → ℤ,
        BoundedGroupRelationAt (P z) C n ∧ UsefulReductionAt x z n) :
    success ⊆ badUsefulSamples x
      (reducedCandidates (F := F) (ι := ι) C) := by
  classical
  intro z hz
  have hRelation := relationCertificate z hz
  obtain ⟨n, hn, hReduction⟩ := reductionBridge z hz hRelation
  have hnBox : n ∈ nonzeroCoefficientBox (ι := ι) C := by
    rw [mem_nonzeroCoefficientBox_iff]
    exact ⟨hn.1, hn.2.1⟩
  have hmCandidate : reduceIntVector (F := F) n ∈
      reducedCandidates (F := F) (ι := ι) C := by
    rw [reducedCandidates, Finset.mem_image]
    exact ⟨n, hnBox, rfl⟩
  have hUseful : z ∈ usefulEvent x (reduceIntVector (F := F) n) := by
    rw [mem_usefulEvent_iff]
    exact hReduction
  rw [badUsefulSamples, Finset.mem_biUnion]
  refine ⟨usefulEvent x (reduceIntVector (F := F) n), ?_, hUseful⟩
  rw [distinctUsefulEvents, Finset.mem_image]
  exact ⟨reduceIntVector (F := F) n, hmCandidate, rfl⟩

/-- Exact end-to-end finite bound from abstract deterministic relation and
reduction/usefulness certificates. -/
theorem exact_success_bound_of_group_relation_certificates
    {G : Type*} [AddCommGroup G]
    (x : F) (C : ℕ) (hC : C < ringChar F)
    (success : Finset (Sample F ι))
    (P : Sample F ι → ι → G)
    (relationCertificate : ∀ z ∈ success,
      ∃ n : ι → ℤ, BoundedGroupRelationAt (P z) C n)
    (reductionBridge : ∀ z ∈ success,
      (∃ n : ι → ℤ, BoundedGroupRelationAt (P z) C n) →
      ∃ n : ι → ℤ,
        BoundedGroupRelationAt (P z) C n ∧ UsefulReductionAt x z n) :
    success.card * Fintype.card F ^ 2 ≤
      ((2 * C + 1) ^ Fintype.card ι - 1) *
        (Fintype.card F - 1) *
          (Fintype.card F ^ Fintype.card ι) ^ 2 := by
  apply bounded_coefficient_success_probability x C hC success
  exact success_subset_badUsefulSamples_of_certificates
    x C success P relationCertificate reductionBridge

/-- Size-comparison `/p` consequence of the composed certificate theorem.
The numerical premise `p ≤ h*|F|` is explicit and is not called a cofactor
theorem. -/
theorem success_bound_over_base_of_group_relation_certificates
    {G : Type*} [AddCommGroup G]
    (x : F) (C : ℕ) (hC : C < ringChar F)
    (success : Finset (Sample F ι))
    (P : Sample F ι → ι → G)
    (relationCertificate : ∀ z ∈ success,
      ∃ n : ι → ℤ, BoundedGroupRelationAt (P z) C n)
    (reductionBridge : ∀ z ∈ success,
      (∃ n : ι → ℤ, BoundedGroupRelationAt (P z) C n) →
      ∃ n : ι → ℤ,
        BoundedGroupRelationAt (P z) C n ∧ UsefulReductionAt x z n)
    (p h : ℕ) (p_pos : 0 < p)
    (sizeComparison : p ≤ h * Fintype.card F) :
    success.card * p ≤
      (h * ((2 * C + 1) ^ Fintype.card ι - 1)) *
        (Fintype.card F ^ Fintype.card ι) ^ 2 := by
  apply bounded_coefficient_success_over_base_with_size_factor
    x C hC success _ p h p_pos sizeComparison
  exact success_subset_badUsefulSamples_of_certificates
    x C success P relationCertificate reductionBridge

end JKSSTSuccessComposition
