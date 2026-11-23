import BinaryUnbounded
import PostCollisionControl

namespace RhoTwoChoiceCollisionAudit
noncomputable section
open scoped BigOperators
open WalkWeightedGeneralAudit StaticHashCollisionAudit RhoFirstCollisionAudit

theorem fair_controls : (∀ i : Fin 2, 0 < fair i) ∧
    (∑ i : Fin 2, fair i) = 1 ∧ (∑ i : Fin 2, (fair i)^2) = 1 / 2 := by
  refine ⟨fun i => by norm_num [fair], fair_sum, ?_⟩
  norm_num [Fin.sum_univ_two, fair]

theorem twochoice_collision_certificate :
    (∀ (p : ℕ) [Fact p.Prime], ensembleMean p =
      ∑ f : Fin 2 ↪ ZMod p, (uniformLaw (Fin 2 ↪ ZMod p) f).toReal *
        ∑ h : Hash (ZMod p) (Fin 2), (hashLaw fair fair_nonneg fair_sum h).toReal *
          (firstCollision p (fun x => x + f (h x)) 0 : ℝ)) ∧
    (∀ (p : ℕ) [Fact p.Prime] (m : ℕ), 0 < m →
      4 * m^2 * (4 * m + 1) ≤ p → (m : ℝ)^2 / 2 ≤ ensembleMean p) ∧
    (∀ C : ℕ, ∃ p : ℕ, ∃ hp : p.Prime,
      (C : ℝ) * p < (@ensembleMean p ⟨hp⟩)^2) ∧
    (¬∃ K : ℝ, 0 ≤ K ∧ ∀ p : ℕ, ∀ hp : p.Prime,
      @ensembleMean p ⟨hp⟩ ≤ K * Real.sqrt p) ∧
    (∀ W n : ℕ, ∀ s : ℤ, (W : ℝ)^2 * hitMass W n s ≤ (s : ℝ)^2 + n) ∧
    (hitMass 1 1 0 = 1 ∧ hitMass 2 1 0 = 0) ∧
    ((∀ i : Fin 2, 0 < fair i) ∧ (∑ i : Fin 2, fair i) = 1 ∧
      (∑ i : Fin 2, (fair i)^2) = 1 / 2) ∧
    (∀ (p : ℕ) [Fact p.Prime] (d : ℕ), 0 < d → d < p → ∀ k : ZMod p,
      finiteMean (fun z : TwoSupport p =>
        if (d : ZMod p) * z.1 + k * z.2.val = 0 then (1 : ℝ) else 0) = 1 / p) ∧
    (¬∀ w : Fin 3 → Fin 2,
      expectation controlWeights (fun h : Hash (ZMod 3) (Fin 2) =>
        if observations controlUpdate h 0 3 = w then (1 : ℝ) else 0) =
          weight controlWeights w) := by
  refine ⟨fun p _ => ensembleMean_actual_law, fun p _ m hm hmp =>
    ensemble_mean_lower m hm hmp, ensemble_ratio_unbounded, no_uniform_sqrt_bound,
    squared_maximal, one_step_controls, fair_controls,
    fun p _ d hd hdp k => relation_mass d hd hdp k, unrestricted_word_mass_false⟩

#print axioms twochoice_collision_certificate
end
end RhoTwoChoiceCollisionAudit
