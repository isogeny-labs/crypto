module
public import RootField

@[expose] public section
namespace FiniteCyclotomicDegreeAudit
noncomputable section
open IntermediateField
variable (K : Type*) [Field K] [Fintype K] (r : ℕ) [NeZero (r : K)]

theorem frobenius_pow_eq_one_iff (n : ℕ) :
    frobenius K r ^ n = 1 ↔ r ∣ Fintype.card K ^ n - 1 := by
  have hq : 1 ≤ Fintype.card K ^ n := Nat.one_le_pow _ _ Fintype.card_pos
  constructor
  · intro h
    obtain ⟨ζ, hζ, _⟩ := roots_field_simple K r
    let z : rootsField K r := ⟨ζ, IntermediateField.subset_adjoin _ _ hζ.pow_eq_one⟩
    have hz := congrArg (fun a : (rootsField K r) ≃ₐ[K] (rootsField K r) =>
      (a z : Closure K)) h
    have hp := congrArg (fun x : rootsField K r => (x : Closure K))
      (frobenius_pow_apply K r n z)
    have he : ζ ^ (Fintype.card K ^ n) = ζ := hp.symm.trans hz
    exact (primitive_power_fixed_iff K r ζ hζ _ hq).mp he
  · intro h
    apply AlgEquiv.coe_toAlgHom_injective
    apply IntermediateField.adjoin_algHom_ext K
    intro x hx
    apply Subtype.ext
    let z : rootsField K r := ⟨x, IntermediateField.subset_adjoin _ _ hx⟩
    have hp := congrArg (fun y : rootsField K r => (y : Closure K))
      (frobenius_pow_apply K r n z)
    exact hp.trans (root_power_fixed K r _ hq x hx h)

/-- Actual extension degree, derived from the actual finite-field Frobenius. -/
theorem roots_field_degree : Module.finrank K (rootsField K r) = orderOf (Fintype.card K : ZMod r) := by
  have he : orderOf (frobenius K r) = orderOf (Fintype.card K : ZMod r) := by
    apply orderOf_eq_orderOf_iff.mpr
    intro n
    exact (frobenius_pow_eq_one_iff K r n).trans (cardinal_cast_pow_iff K r n).symm
  exact (FiniteField.orderOf_frobeniusAlgEquivOfAlgebraic K (rootsField K r)).symm.trans he

theorem actual_degree_dvd_iff (n : ℕ) :
    Module.finrank K (rootsField K r) ∣ n ↔ r ∣ Fintype.card K ^ n - 1 := by
  rw [roots_field_degree, orderOf_dvd_iff_pow_eq_one, cardinal_cast_pow_iff]

#print axioms frobenius_pow_eq_one_iff
#print axioms roots_field_degree
#print axioms actual_degree_dvd_iff
end
end FiniteCyclotomicDegreeAudit
