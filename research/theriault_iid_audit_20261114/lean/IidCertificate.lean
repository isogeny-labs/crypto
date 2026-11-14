import PairedOccupancy

namespace TheriaultIidAudit

/-- Actual finite uniform sampling, with no assumed recurrence or closed form. -/
theorem actual_iid_collision_certificate :
    (∀ (α : Type) [Fintype α] [DecidableEq α] [Nonempty α],
      Function.Involutive (pairedNeg : α × Bool → α × Bool) ∧
      (∀ p : α × Bool, pairedNeg p ≠ p) ∧
      (∀ p q : α × Bool, p.1 = q.1 ↔ p = q ∨ p = pairedNeg q) ∧
      (∀ s : ℕ, expectedRepeats α s = (s : ℚ) - Fintype.card α +
        Fintype.card α * (1-1/(Fintype.card α : ℚ))^s) ∧
      (∀ s : ℕ, projectedExpectation α Bool s =
        TheriaultCollisionAudit.sourceClosedForm (Fintype.card (α × Bool) : ℚ) s) ∧
      (∀ s : ℕ, projectedExpectation α Bool (s+1) =
        (1-2/(Fintype.card (α × Bool) : ℚ))*projectedExpectation α Bool s +
        2*s/(Fintype.card (α × Bool) : ℚ)) ∧
      (∀ s : ℕ, 3 ≤ s → (s : ℚ) ≤ (Fintype.card (α × Bool) : ℚ)/2 →
        4*(s : ℚ)^2/(9*(Fintype.card (α × Bool) : ℚ)) ≤ projectedExpectation α Bool s ∧
        projectedExpectation α Bool s ≤ (s : ℚ)^2/(Fintype.card (α × Bool) : ℚ))) ∧
    (∀ t : ℕ, (3 : ℚ) < (Fintype.card (Fin (t+4) × Bool) : ℚ)/2 ∧
      ¬ (2*(3 : ℚ)^2/(3*(Fintype.card (Fin (t+4) × Bool) : ℚ)) ≤
        projectedExpectation (Fin (t+4)) Bool 3)) ∧
    (∀ b : ℕ, ∃ t : ℕ, b < Fintype.card (Fin (t+4) × Bool)) ∧
    projectedExpectation (Fin 4) Bool 3 = 11/16 ∧ expectedRepeats (Fin 8) 3 = 23/64 := by
  refine ⟨?_, actual_unbounded_counterfamily, pool_sizes_unbounded,
    eight_element_control, unpaired_negative_control⟩
  intro α _ _ _
  exact ⟨pairedNeg_involutive, pairedNeg_free, paired_same_orbit,
    expected_repeats_formula, actual_source_closed_form, actual_paired_recurrence,
    actual_paired_repaired_bounds⟩

#print axioms actual_iid_collision_certificate

end TheriaultIidAudit
