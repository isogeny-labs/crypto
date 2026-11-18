module
public import GuardControls

@[expose] public section
namespace FiniteCyclotomicDegreeAudit
noncomputable section

/-- Actual finite-field root-adjoin degree, all-root extension rationality,
small-order controls and characteristic-guard counterexample family. -/
theorem finite_cyclotomic_degree_certificate :
    (∀ (K : Type) [Field K] [Fintype K] (r : ℕ) [NeZero (r : K)],
      Module.finrank K (rootsField K r) = orderOf (Fintype.card K : ZMod r) ∧
      (∀ (n : ℕ), Module.finrank K (rootsField K r) ∣ n ↔
        r ∣ Fintype.card K ^ n - 1) ∧
      (∀ (L : IntermediateField K (Closure K)) [Finite L],
        (rootsField K r ≤ L ↔ r ∣ Nat.card L - 1) ∧
        ((∀ x : Closure K, x ^ r = 1 → x ∈ L) ↔
          r ∣ Fintype.card K ^ Module.finrank K L - 1))) ∧
    (∀ (K : Type) [Field K],
      Module.finrank K (rootsField K 1) = 1 ∧ Module.finrank K (rootsField K 2) = 1) ∧
    (∀ (K : Type) [Field K] [Fintype K] (p : ℕ) [Fact p.Prime] [CharP K p],
      Module.finrank K (rootsField K p) = 1 ∧
      orderOf (Fintype.card K : ZMod p) = 0 ∧
      Module.finrank K (rootsField K p) ≠ orderOf (Fintype.card K : ZMod p) ∧
      rootsField K p ≤ (⊥ : IntermediateField K (Closure K)) ∧
      ¬ p ∣ Fintype.card K - 1) := by
  refine ⟨?_, ?_, ?_⟩
  · intro K _ _ r _
    refine ⟨roots_field_degree K r, actual_degree_dvd_iff K r, ?_⟩
    intro L _
    exact ⟨roots_field_le_iff K r L, all_roots_rational_iff K r L⟩
  · intro K _
    exact ⟨roots_one_field_degree K, roots_two_field_degree K⟩
  · intro K _ _ p _ _
    exact ⟨characteristic_root_field_degree K p, characteristic_modular_order_zero K p,
      characteristic_guard_counterexample K p⟩

#print axioms finite_cyclotomic_degree_certificate
end
end FiniteCyclotomicDegreeAudit
