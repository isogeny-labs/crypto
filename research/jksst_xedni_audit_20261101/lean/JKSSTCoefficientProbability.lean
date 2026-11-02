import JKSSTCoefficientBox
import JKSSTProbability

/-!
# Exact bounded-coefficient success probability

This module specializes the finite-field probability theorem to the exact
nonzero symmetric integer coefficient box.  It replaces the paper's
“approximately `(2C)^r/p`” by a kernel-checked, cross-multiplied inequality.
-/

namespace JKSSTCoefficientProbability

open JKSSTCoefficientBox JKSSTProbability

variable {F ι : Type*} [Field F] [Fintype F] [Fintype ι]

/-- Reduce every nonzero vector in `[-C,C]^ι` into the finite field. -/
noncomputable def reducedCandidates (C : ℕ) : Finset (ι → F) := by
  classical
  exact (nonzeroCoefficientBox C).image (reduceIntVector (F := F))

omit [Fintype F] in
/-- If `C` is below the field characteristic, none of the reduced candidate
vectors is zero. -/
theorem reducedCandidates_ne_zero
    (C : ℕ) (hC : C < ringChar F) :
    ∀ v ∈ reducedCandidates (F := F) (ι := ι) C, v ≠ 0 := by
  classical
  intro v hv
  rw [reducedCandidates, Finset.mem_image] at hv
  obtain ⟨m, hm, rfl⟩ := hv
  rw [mem_nonzeroCoefficientBox_iff] at hm
  exact reduceIntVector_ne_zero_of_abs_le m C hm.1 hm.2 hC

/-- Reduction and projective/event deduplication can only decrease the exact
raw candidate count `(2C+1)^r-1`. -/
theorem card_distinct_reduced_useful_events_le
    (x : F) (C : ℕ) :
    (distinctUsefulEvents x
      (reducedCandidates (F := F) (ι := ι) C)).card ≤
        (2 * C + 1) ^ Fintype.card ι - 1 := by
  classical
  calc
    (distinctUsefulEvents x
        (reducedCandidates (F := F) (ι := ι) C)).card ≤
        (reducedCandidates (F := F) (ι := ι) C).card :=
      card_distinctUsefulEvents_le x _
    _ ≤ (nonzeroCoefficientBox (ι := ι) C).card := by
      unfold reducedCandidates
      exact Finset.card_image_le
    _ = (2 * C + 1) ^ Fintype.card ι - 1 :=
      card_nonzeroCoefficientBox C

/-- Exact finite replacement for the paper's approximate success bound.

Let `q=|F|`, `r=|ι|`, and let the uniform sample space be two independent
vectors in `F^r`, of size `(q^r)^2`.  If algorithmic success implies a useful
relation with some nonzero integral vector in `[-C,C]^r`, and `C<char(F)`,
then

`#success * q^2 ≤ ((2C+1)^r-1) * (q-1) * (q^r)^2`.

Thus `Pr[success] ≤ ((2C+1)^r-1)(q-1)/q^2`.  Scalar multiples are actually
deduplicated internally, so this raw-box corollary may be conservative.
-/
theorem bounded_coefficient_success_probability
    (x : F) (C : ℕ) (hC : C < ringChar F)
    (success : Finset ((ι → F) × (ι → F)))
    (hsuccess : success ⊆ badUsefulSamples x
      (reducedCandidates (F := F) (ι := ι) C)) :
    success.card * Fintype.card F ^ 2 ≤
      ((2 * C + 1) ^ Fintype.card ι - 1) *
        (Fintype.card F - 1) *
          (Fintype.card F ^ Fintype.card ι) ^ 2 := by
  have exact_event_bound :=
    card_algorithmSuccess_mul_card_field_sq_le x
      (reducedCandidates (F := F) (ι := ι) C) success
      (reducedCandidates_ne_zero C hC) hsuccess
  have event_count :=
    card_distinct_reduced_useful_events_le (F := F) (ι := ι) x C
  calc
    success.card * Fintype.card F ^ 2 ≤
        (distinctUsefulEvents x
          (reducedCandidates (F := F) (ι := ι) C)).card *
            (Fintype.card F - 1) *
              (Fintype.card F ^ Fintype.card ι) ^ 2 := exact_event_bound
    _ ≤ ((2 * C + 1) ^ Fintype.card ι - 1) *
          (Fintype.card F - 1) *
            (Fintype.card F ^ Fintype.card ι) ^ 2 := by
      gcongr

/-- The simpler `/q` consequence of the exact useful-event bound. -/
theorem bounded_coefficient_success_over_field_card
    (x : F) (C : ℕ) (hC : C < ringChar F)
    (success : Finset ((ι → F) × (ι → F)))
    (hsuccess : success ⊆ badUsefulSamples x
      (reducedCandidates (F := F) (ι := ι) C)) :
    success.card * Fintype.card F ≤
      ((2 * C + 1) ^ Fintype.card ι - 1) *
        (Fintype.card F ^ Fintype.card ι) ^ 2 := by
  let q := Fintype.card F
  let N := (2 * C + 1) ^ Fintype.card ι - 1
  let total := (Fintype.card F ^ Fintype.card ι) ^ 2
  have exact_bound :=
    bounded_coefficient_success_probability x C hC success hsuccess
  have q_pos : 0 < q := Fintype.card_pos_iff.mpr ⟨0⟩
  have q_minus_one_le : q - 1 ≤ q := Nat.sub_le q 1
  have scaled : success.card * q ^ 2 ≤ N * q * total := by
    exact exact_bound.trans (by
      dsimp [q, N, total]
      gcongr)
  have cancel_form : (success.card * q) * q ≤ (N * total) * q := by
    calc
      (success.card * q) * q = success.card * q ^ 2 := by ring
      _ ≤ N * q * total := scaled
      _ = (N * total) * q := by ring
  have cancelled : success.card * q ≤ N * total :=
    Nat.le_of_mul_le_mul_right cancel_form q_pos
  simpa [q, N, total] using cancelled

/-- Explicit size-comparison `/p` form.  If the base parameter satisfies
`p ≤ h*|F|`, the constant in the `/p` numerator grows by the explicit
factor `h`.  No group-theoretic cofactor claim is hidden in this premise. -/
theorem bounded_coefficient_success_over_base_with_size_factor
    (x : F) (C : ℕ) (hC : C < ringChar F)
    (success : Finset ((ι → F) × (ι → F)))
    (hsuccess : success ⊆ badUsefulSamples x
      (reducedCandidates (F := F) (ι := ι) C))
    (p h : ℕ) (_p_pos : 0 < p)
    (size_comparison : p ≤ h * Fintype.card F) :
    success.card * p ≤
      (h * ((2 * C + 1) ^ Fintype.card ι - 1)) *
        (Fintype.card F ^ Fintype.card ι) ^ 2 := by
  apply div_card_field_to_base_with_size_factor
    (q := Fintype.card F)
    (bounded_coefficient_success_over_field_card x C hC success hsuccess)
    size_comparison

end JKSSTCoefficientProbability
