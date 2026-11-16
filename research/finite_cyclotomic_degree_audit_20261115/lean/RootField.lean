module
public import Mathlib.RingTheory.RootsOfUnity.AlgebraicallyClosed
public import Mathlib.FieldTheory.IntermediateField.Adjoin.Basic
public import Mathlib.FieldTheory.Finite.Basic
public import Mathlib.Tactic

@[expose] public section
namespace FiniteCyclotomicDegreeAudit
noncomputable section
open IntermediateField

variable (K : Type*) [Field K] (r : ℕ)

abbrev Closure := AlgebraicClosure K
def rootsField : IntermediateField K (Closure K) :=
  IntermediateField.adjoin K {z : Closure K | z ^ r = 1}

theorem roots_one_field_degree : Module.finrank K (rootsField K 1) = 1 := by
  have he : rootsField K 1 = ⊥ := by
    apply le_antisymm
    · apply IntermediateField.adjoin_le_iff.mpr
      intro x hx
      simp only [pow_one, Set.mem_ofPred_eq] at hx
      rw [hx]
      exact IntermediateField.one_mem _
    · exact bot_le
  rw [he]
  exact IntermediateField.finrank_bot

/-- Credited small-order control: every square root of unity is in the base field. -/
theorem roots_two_field_degree : Module.finrank K (rootsField K 2) = 1 := by
  have he : rootsField K 2 = ⊥ := by
    apply le_antisymm
    · apply IntermediateField.adjoin_le_iff.mpr
      intro z hz
      apply IntermediateField.mem_bot.mpr
      rcases sq_eq_one_iff.mp hz with h | h
      · exact ⟨1, by simpa using h.symm⟩
      · exact ⟨-1, by simpa using h.symm⟩
    · exact bot_le
  rw [he]
  exact IntermediateField.finrank_bot

theorem root_power_fixed (N : ℕ) (hN : 1 ≤ N) (x : Closure K) (hx : x ^ r = 1)
    (hd : r ∣ N - 1) : x ^ N = x := by
  obtain ⟨k, hk⟩ := hd
  have he : x ^ (N - 1) = 1 := by rw [hk, pow_mul, hx, one_pow]
  calc
    x ^ N = x ^ (N - 1 + 1) := by rw [Nat.sub_add_cancel hN]
    _ = x := by rw [pow_succ, he, one_mul]

theorem cardinal_cast_pow_iff [Fintype K] (n : ℕ) :
    (Fintype.card K : ZMod r) ^ n = 1 ↔ r ∣ Fintype.card K ^ n - 1 := by
  have hq : 1 ≤ Fintype.card K ^ n := Nat.one_le_pow _ _ Fintype.card_pos
  constructor
  · intro h
    apply (ZMod.natCast_eq_zero_iff _ _).mp
    rw [Nat.cast_sub hq, Nat.cast_one, Nat.cast_pow, h, sub_self]
  · intro h
    have he := (ZMod.natCast_eq_zero_iff _ _).mpr h
    rw [Nat.cast_sub hq, Nat.cast_one, Nat.cast_pow] at he
    exact sub_eq_zero.mp he

section GoodCharacteristic
variable [NeZero (r : K)]

/-- Actual primitive root and actual simple-extension presentation, derived. -/
theorem roots_field_simple : ∃ ζ : Closure K,
    IsPrimitiveRoot ζ r ∧ rootsField K r = K⟮ζ⟯ := by
  let : NeZero r := .of_neZero_natCast K
  obtain ⟨ζ, hζ⟩ := HasEnoughRootsOfUnity.exists_isPrimitiveRoot (Closure K) r
  refine ⟨ζ, hζ, le_antisymm ?_ ?_⟩
  · apply IntermediateField.adjoin_le_iff.mpr
    intro x hx
    obtain ⟨i, _, hi⟩ := hζ.eq_pow_of_pow_eq_one hx
    rw [← hi]
    exact K⟮ζ⟯.toSubfield.pow_mem (IntermediateField.mem_adjoin_simple_self K ζ) i
  · apply IntermediateField.adjoin_simple_le_iff.mpr
    exact IntermediateField.subset_adjoin _ _ hζ.pow_eq_one

instance rootFieldFiniteDimensional : FiniteDimensional K (rootsField K r) := by
  obtain ⟨ζ, _, he⟩ := roots_field_simple K r
  rw [he]
  exact IntermediateField.adjoin.finiteDimensional
    ((Algebra.IsAlgebraic.isAlgebraic ζ).isIntegral)

theorem primitive_power_fixed_iff (ζ : Closure K) (hζ : IsPrimitiveRoot ζ r)
    (N : ℕ) (hN : 1 ≤ N) : ζ ^ N = ζ ↔ r ∣ N - 1 := by
  let : NeZero r := .of_neZero_natCast K
  constructor
  · intro h
    apply hζ.dvd_of_pow_eq_one
    apply mul_right_cancel₀ (hζ.isUnit (NeZero.ne r)).ne_zero
    rw [one_mul, ← pow_succ, Nat.sub_add_cancel hN]
    exact h
  · exact root_power_fixed K r N hN ζ hζ.pow_eq_one

variable [Fintype K]

instance rootFieldFinite : Finite (rootsField K r) :=
  Finite.of_injective (Module.finBasis K (rootsField K r)).equivFun
    (Module.finBasis K (rootsField K r)).equivFun.injective

def frobenius : (rootsField K r) ≃ₐ[K] (rootsField K r) :=
  FiniteField.frobeniusAlgEquivOfAlgebraic K (rootsField K r)

omit [NeZero (r : K)] in
theorem frobenius_pow_apply (n : ℕ) (x : rootsField K r) :
    (frobenius K r ^ n) x = x ^ (Fintype.card K ^ n) := by
  rw [AlgEquiv.coe_pow]
  exact congrFun (FiniteField.coe_frobeniusAlgEquivOfAlgebraic_iterate K (rootsField K r) n) x

#print axioms roots_field_simple
#print axioms primitive_power_fixed_iff
#print axioms frobenius_pow_apply
end GoodCharacteristic
end
end FiniteCyclotomicDegreeAudit
