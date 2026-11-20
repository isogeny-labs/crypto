module
public import CanonicalPairing

@[expose] public section
namespace ContinuousH1ComparisonAudit
noncomputable section
open ProcyclicGaloisCocycleAudit GaloisCharacterPairingAudit

local instance : TopologicalSpace TwoTarget := ⊥
local instance : DiscreteTopology TwoTarget := ⟨rfl⟩
local instance : TopologicalSpace (MulAut TwoTarget) := ⊥
local instance : DiscreteTopology (MulAut TwoTarget) := ⟨rfl⟩

/-- Actual categorical continuous H1 comparison and finite-field perfect pairing,
with exact actual homogeneous representative formula and guard-deletion control. -/
theorem continuous_h1_comparison_certificate :
    (∀ (k G : Type) [Ring k] [TopologicalSpace k]
      [Group G] [TopologicalSpace G] [IsTopologicalGroup G]
      (X : TopRep.{0} k G) (hρ : Continuous (fun a : G × X => X.ρ a.1 a.2)),
      Nonempty (continuousCohomology 1 X ≃ₗ[k] CrossedClasses X hρ)) ∧
    (∀ (K : Type) [Field K] [Fintype K]
      (M : Type) [CommGroup M] [Finite M] [TopologicalSpace M] [DiscreteTopology M]
      [TopologicalSpace (MulAut M)] [DiscreteTopology (MulAut M)]
      (ρ : Gamma K →ₜ* MulAut M),
      Nonempty (continuousCohomology 1 (coefficientObject K M ρ) ≃ₗ[ℤ]
        Additive (CocycleClasses K M ρ)) ∧
      Nonempty (continuousCohomology 1 (coefficientObject K M ρ) ≃ₗ[ℤ]
        Additive (Coinvariants K M ρ)) ∧
      (∀ hM : ∀ m : M, m ^ (Fintype.card K - 1) = 1,
        Nonempty (galoisCharacters K M ρ ≃* (ActualH1 K M ρ →* Kˣ)) ∧
        Nonempty (ActualH1 K M ρ ≃* (galoisCharacters K M ρ →* Kˣ)) ∧
        (∀ (χ : galoisCharacters K M ρ) (f : KernelOne (coefficientObject K M ρ)),
          baseUnitMap K (canonicalPairing K M ρ hM χ
            (Multiplicative.ofAdd (homogeneousKernelClass (coefficientObject K M ρ) f))) =
              χ.val (f.val.val 1 (frobenius K)).toMul) ∧
        (∀ χ, (∀ c, canonicalPairing K M ρ hM χ c = 1) → χ = 1) ∧
        (∀ c, (∀ χ, canonicalPairing K M ρ hM χ c = 1) → c = 1))) ∧
    (∀ (F : Type) [Field F] [Fintype F] [CharP F 2],
      missingActualTwoClass F ≠ 1 ∧
      ¬ ∃ e : TwoCharacters F →* (ActualTwoH1 F →* Fˣ),
        ∀ c : ActualTwoH1 F, (∀ χ : TwoCharacters F, e χ c = 1) → c = 1) := by
  refine ⟨?_, ?_, ?_⟩
  · intro k G _ _ _ _ _ X hρ
    exact ⟨actualContinuousH1CrossedClasses X hρ⟩
  · intro K _ _ M _ _ _ _ _ _ ρ
    refine ⟨⟨actualGaloisH1Equiv K M ρ⟩, ⟨actualGaloisH1CoinvariantEquiv K M ρ⟩, ?_⟩
    intro hM
    exact ⟨⟨canonicalLeftDuality K M ρ hM⟩, ⟨canonicalRightDuality K M ρ hM⟩,
      canonical_homogeneous_formula K M ρ hM, canonical_left_nondegenerate K M ρ hM,
      canonical_right_nondegenerate K M ρ hM⟩
  · intro F _ _ _
    exact ⟨missing_actual_two_class_ne_one F, canonical_annihilation_guard_needed F⟩

#print axioms continuous_h1_comparison_certificate
end
end ContinuousH1ComparisonAudit
