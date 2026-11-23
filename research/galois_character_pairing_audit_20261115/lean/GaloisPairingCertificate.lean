module
public import CharTwoGaloisControl

@[expose] public section
namespace GaloisCharacterPairingAudit
noncomputable section
open ProcyclicGaloisCocycleAudit

local instance : TopologicalSpace TwoTarget := ⊥
local instance : DiscreteTopology TwoTarget := ⟨rfl⟩
local instance : TopologicalSpace (MulAut TwoTarget) := ⊥
local instance : DiscreteTopology (MulAut TwoTarget) := ⟨rfl⟩

/-- Finite-module Galois character / all-continuous-cocycle-class perfect pairing,
with the displayed source formula and actual characteristic-2 guard deletion
counterexample. Separate homogeneous-complex H1 and isogeny bridges are not assumed. -/
theorem galois_character_pairing_certificate :
    (∀ (K : Type) [Field K] [Fintype K]
      (M : Type) [CommGroup M] [Finite M] [TopologicalSpace M] [DiscreteTopology M]
      [TopologicalSpace (MulAut M)] [DiscreteTopology (MulAut M)]
      (ρ : Gamma K →ₜ* MulAut M)
      (hM : ∀ m : M, m ^ (Fintype.card K - 1) = 1),
      Nonempty (galoisCharacters K M ρ ≃* (CocycleClasses K M ρ →* Kˣ)) ∧
      Nonempty (CocycleClasses K M ρ ≃* (galoisCharacters K M ρ →* Kˣ)) ∧
      (∀ (χ : galoisCharacters K M ρ) (c : Cocycles K M ρ),
        baseUnitMap K (pairing K M ρ hM χ (QuotientGroup.mk c)) =
          χ.val (c.val (frobenius K))) ∧
      (∀ χ ψ c, pairing K M ρ hM (χ * ψ) c =
        pairing K M ρ hM χ c * pairing K M ρ hM ψ c) ∧
      (∀ χ c d, pairing K M ρ hM χ (c * d) =
        pairing K M ρ hM χ c * pairing K M ρ hM χ d) ∧
      (∀ χ, (∀ c, pairing K M ρ hM χ c = 1) → χ = 1) ∧
      (∀ c, (∀ χ, pairing K M ρ hM χ c = 1) → c = 1) ∧
      Nat.card (galoisCharacters K M ρ) = Nat.card (CocycleClasses K M ρ)) ∧
    (∀ (K : Type) [Field K] [Fintype K] [CharP K 2],
      (∀ χ : TwoCharacters K, χ = 1) ∧
      missingTwoClass K ≠ 1 ∧
      (¬ ∃ e : TwoCharacters K →* (TwoClasses K →* Kˣ),
        ∀ c : TwoClasses K, (∀ χ : TwoCharacters K, e χ c = 1) → c = 1) ∧
      ¬ ∀ m : TwoTarget, m ^ (Fintype.card K - 1) = 1) := by
  constructor
  · intro K _ _ M _ _ _ _ _ _ ρ hM
    exact ⟨⟨leftDuality K M ρ hM⟩, ⟨rightDuality K M ρ hM⟩,
      source_pairing_formula K M ρ hM, pairing_left K M ρ hM,
      pairing_right K M ρ hM, pairing_left_nondegenerate K M ρ hM,
      pairing_right_nondegenerate K M ρ hM, duality_cardinality K M ρ hM⟩
  · intro K _ _ _
    exact ⟨two_characters_trivial K, missing_two_class_ne_one K,
      annihilation_guard_needed K, two_annihilation_false K⟩

#print axioms galois_character_pairing_certificate
end
end GaloisCharacterPairingAudit
