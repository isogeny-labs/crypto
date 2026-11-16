module
public import FrobeniusDegree

@[expose] public section
namespace FiniteCyclotomicDegreeAudit
noncomputable section
open Polynomial IntermediateField
variable (K : Type*) [Field K] [Fintype K]

omit [Fintype K] in
/-- In an actual finite intermediate field, all elements fixed by its cardinality power descend. -/
theorem finite_field_power_fixed_iff (L : IntermediateField K (Closure K)) [Finite L]
    (x : Closure K) : x ^ Nat.card L = x ↔ x ∈ L := by
  let : Fintype L := Fintype.ofFinite L
  rw [Nat.card_eq_fintype_card]
  constructor
  · intro h
    have hs : (X ^ Fintype.card L - X : L[X]).Splits := by
      rw [Polynomial.splits_iff_card_roots, FiniteField.roots_X_pow_card_sub_X,
        ← Finset.card_def, Finset.card_univ,
        FiniteField.X_pow_card_sub_X_natDegree_eq L Fintype.one_lt_card]
    have hr : ((X ^ Fintype.card L - X : L[X]).map (algebraMap L (Closure K))).IsRoot x := by
      simpa [Polynomial.IsRoot, sub_eq_zero] using h
    obtain ⟨y, hy⟩ := hs.mem_range_of_isRoot
      (FiniteField.X_pow_card_sub_X_ne_zero L Fintype.one_lt_card)
      hr
    exact hy ▸ y.property
  · intro hx
    have h := congrArg (fun y : L => (y : Closure K)) (FiniteField.pow_card (⟨x, hx⟩ : L))
    simpa using h

variable (r : ℕ) [NeZero (r : K)]

omit [Fintype K] in
/-- Containment of the actual all-roots field in an actual finite extension. -/
theorem roots_field_le_iff (L : IntermediateField K (Closure K)) [Finite L] :
    rootsField K r ≤ L ↔ r ∣ Nat.card L - 1 := by
  have hL : 1 ≤ Nat.card L := Nat.card_pos
  constructor
  · intro h
    obtain ⟨ζ, hζ, _⟩ := roots_field_simple K r
    have hz : ζ ∈ L := h (IntermediateField.subset_adjoin _ _ hζ.pow_eq_one)
    exact (primitive_power_fixed_iff K r ζ hζ _ hL).mp
      ((finite_field_power_fixed_iff K L ζ).mpr hz)
  · intro h
    apply IntermediateField.adjoin_le_iff.mpr
    intro x hx
    exact (finite_field_power_fixed_iff K L x).mp (root_power_fixed K r _ hL x hx h)

theorem roots_field_le_degree_iff (L : IntermediateField K (Closure K)) [Finite L] :
    rootsField K r ≤ L ↔ r ∣ Fintype.card K ^ Module.finrank K L - 1 := by
  rw [roots_field_le_iff K r L, Module.natCard_eq_pow_finrank (K := K),
    Nat.card_eq_fintype_card]

theorem all_roots_rational_iff (L : IntermediateField K (Closure K)) [Finite L] :
    (∀ x : Closure K, x ^ r = 1 → x ∈ L) ↔
      r ∣ Fintype.card K ^ Module.finrank K L - 1 := by
  rw [← roots_field_le_degree_iff K r L]
  exact IntermediateField.adjoin_le_iff.symm

#print axioms finite_field_power_fixed_iff
#print axioms roots_field_le_iff
#print axioms roots_field_le_degree_iff
#print axioms all_roots_rational_iff
end
end FiniteCyclotomicDegreeAudit
