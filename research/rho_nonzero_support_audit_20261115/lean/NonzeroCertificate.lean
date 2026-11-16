import NonzeroBounds

namespace RhoNonzeroSupportAudit
noncomputable section
open scoped BigOperators
open RhoTwoChoiceCollisionAudit WalkWeightedGeneralAudit RhoFirstCollisionAudit

theorem nonzero_support_certificate :
    (∀ (p : ℕ) [Fact p.Prime], ensembleMean p ≤ nonzeroMean p + 2) ∧
    (∀ (p : ℕ) [Fact p.Prime] [Fact (2 < p)], nonzeroMean p =
      ∑ f : NonzeroEmbedding p, (uniformLaw (NonzeroEmbedding p) f).toReal *
        ∑ h : Hash (ZMod p) (Fin 2), (hashLaw fair fair_nonneg fair_sum h).toReal *
          (firstCollision p (fun x => x + (f (h x)).val) 0 : ℝ)) ∧
    (∀ (p : ℕ) [Fact p.Prime] (m : ℕ), 3 ≤ m →
      4 * m^2 * (4 * m + 1) ≤ p → (m : ℝ)^2 / 4 ≤ nonzeroMean p) ∧
    (∀ C : ℕ, ∃ p : ℕ, ∃ hp : p.Prime, 2 < p ∧
      (C : ℝ) * p < (@nonzeroMean p ⟨hp⟩)^2) ∧
    (¬∃ K : ℝ, 0 ≤ K ∧ ∀ p : ℕ, ∀ hp : p.Prime, 2 < p →
      @nonzeroMean p ⟨hp⟩ ≤ K * Real.sqrt p) ∧
    (∀ (p : ℕ) [Fact p.Prime], finiteMean (fun z : TwoSupport p =>
      if ¬Good z then (1 : ℝ) else 0) = 2 / p) ∧
    (Fintype.card (NonzeroSupport 2) = 0 ∧ Fintype.card (NonzeroSupport 3) = 2) := by
  exact ⟨fun p _ => all_mean_le_nonzero_add_two, fun p _ _ => nonzeroMean_actual_law,
    fun p _ m hm hmp => nonzero_mean_quarter m hm hmp, nonzero_ratio_unbounded,
    nonzero_no_uniform_sqrt_bound, fun p _ => discarded_mass, small_population_controls⟩

#print axioms nonzero_support_certificate
end
end RhoNonzeroSupportAudit
