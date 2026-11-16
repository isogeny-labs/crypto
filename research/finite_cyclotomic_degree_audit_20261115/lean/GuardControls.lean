module
public import RootRationality

@[expose] public section
namespace FiniteCyclotomicDegreeAudit
noncomputable section
open IntermediateField
variable (K : Type*) [Field K] (p : ℕ) [Fact p.Prime] [CharP K p]

/-- In characteristic p, every actual p-th root of unity is 1. -/
theorem characteristic_root_one (x : Closure K) (hx : x ^ p = 1) : x = 1 := by
  apply iterateFrobenius_inj (Closure K) p 1
  change x ^ (p ^ 1) = (1 : Closure K) ^ (p ^ 1)
  simpa only [pow_one, one_pow] using hx

theorem characteristic_root_field_bot : rootsField K p = ⊥ := by
  apply le_antisymm
  · apply IntermediateField.adjoin_le_iff.mpr
    intro x hx
    rw [characteristic_root_one K p x hx]
    exact IntermediateField.one_mem _
  · exact bot_le

theorem characteristic_root_field_degree : Module.finrank K (rootsField K p) = 1 := by
  rw [characteristic_root_field_bot]
  exact IntermediateField.finrank_bot

variable [Fintype K]

theorem characteristic_modular_order_zero : orderOf (Fintype.card K : ZMod p) = 0 := by
  have hd : p ∣ Fintype.card K :=
    (CharP.cast_eq_zero_iff K p _).mp (FiniteField.cast_card_eq_zero K)
  rw [(ZMod.natCast_eq_zero_iff _ _).mpr hd, orderOf_zero]

omit [Fact p.Prime] in
theorem characteristic_card_minus_one_not_dvd : ¬ p ∣ Fintype.card K - 1 := by
  intro hd
  have he := (CharP.cast_eq_zero_iff K p _).mpr hd
  rw [Nat.cast_sub (Nat.one_le_iff_ne_zero.mpr Fintype.card_ne_zero), Nat.cast_one,
    FiniteField.cast_card_eq_zero, zero_sub] at he
  exact neg_ne_zero.mpr one_ne_zero he

/-- Actual fields refute both identities if the nonzero-characteristic guard is deleted. -/
theorem characteristic_guard_counterexample :
    Module.finrank K (rootsField K p) ≠ orderOf (Fintype.card K : ZMod p) ∧
    rootsField K p ≤ (⊥ : IntermediateField K (Closure K)) ∧
    ¬ p ∣ Fintype.card K - 1 := by
  refine ⟨?_, ?_, characteristic_card_minus_one_not_dvd K p⟩
  · rw [characteristic_root_field_degree, characteristic_modular_order_zero]
    decide
  · rw [characteristic_root_field_bot]

#print axioms characteristic_root_one
#print axioms characteristic_root_field_degree
#print axioms characteristic_modular_order_zero
#print axioms characteristic_guard_counterexample
end
end FiniteCyclotomicDegreeAudit
