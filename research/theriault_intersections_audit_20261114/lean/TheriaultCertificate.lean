import SourceProbability

namespace TheriaultIntersectionsAudit
noncomputable section
open Finset WalkBoundaryAudit

/-- Actual paired iid model, source expectation, literal counterfamily and repair. -/
theorem theriault_intersections_certificate :
    (∀ M : ℕ, Fintype.card (PairedPoint M) = 2 * M ∧
      Function.Involutive (@negatePoint M) ∧
      (∀ a : PairedPoint M, negatePoint a ≠ a) ∧
      (∀ a b : PairedPoint M, a.1 = b.1 ↔ b = a ∨ b = negatePoint a)) ∧
    (∀ (M s : ℕ) [NeZero M] (w : PairedWord M s),
      uniformPMF (PairedWord M s) w =
        ∏ i : Fin s, uniformPMF (PairedPoint M) (w i)) ∧
    (∀ (M s : ℕ) [NeZero M], 0 < s →
      (∀ k : ℕ, 0 ≤ intersectionProbability M s k) ∧
      (∑ k ∈ range s, intersectionProbability M s k) = 1 ∧
      sourceMean M s = pairedMean M s ∧
      sourceMean M s = (s : ℝ) - M + M * (1 - (M : ℝ)⁻¹) ^ s) ∧
    (∀ (M s : ℕ) [NeZero M],
      pairedMean M (s + 1) = (1 - (M : ℝ)⁻¹) * pairedMean M s + (s : ℝ) / M) ∧
    (∀ (M : ℕ) [NeZero M], 4 ≤ M →
      3 ≤ (3 : ℕ) ∧ 3 < (2 * M) / 2 ∧
      sourceMean M 3 = 3 / (M : ℝ) - 1 / (M : ℝ) ^ 2 ∧
      sourceMean M 3 < 2 * (3 : ℝ) ^ 2 / (3 * (2 * M))) ∧
    (¬∀ (M s : ℕ) [NeZero M], 3 ≤ s → s < (2 * M) / 2 →
      2 * (s : ℝ) ^ 2 / (3 * (2 * M)) ≤ sourceMean M s) ∧
    (Fintype.card (PairedPoint 4) = 8 ∧ Fintype.card (PairedWord 4 3) = 512 ∧
      sourceMean 4 3 = 11 / 16 ∧ 2 * (3 : ℝ) ^ 2 / (3 * 8) = 3 / 4 ∧
      sourceMean 4 3 < 3 / 4) ∧
    (∀ (M s : ℕ) [NeZero M], 3 ≤ s → s < (2 * M) / 2 →
      4 * (s : ℝ) ^ 2 / (9 * (2 * M)) ≤ sourceMean M s ∧
      sourceMean M s ≤ (s : ℝ) ^ 2 / (2 * M)) := by
  refine ⟨?_, ?_, ?_, ?_, ?_, literal_theorem_one_lower_false,
    concrete_eight_point_fixture, ?_⟩
  · intro M
    refine ⟨?_, negatePoint_involutive M, negatePoint_no_fixed, same_orbit⟩
    simp [PairedPoint, mul_comm]
  · intro M s _ w
    exact word_mass_product M s w
  · intro M s _ hs
    exact ⟨(fun k => intersectionProbability_nonnegative M s k), intersectionProbability_sum M s hs,
      source_mean_eq M s hs, source_mean_closed M s hs⟩
  · intro M s _
    exact paired_mean_recurrence M s
  · intro M _ hM
    obtain ⟨h3, hhi, hbad⟩ := source_lower_bound_counterfamily M hM
    exact ⟨h3, hhi, source_mean_three M, hbad⟩
  · intro M s _ hs hhi
    exact repaired_source_window M s hs hhi

#print axioms theriault_intersections_certificate
end
end TheriaultIntersectionsAudit
