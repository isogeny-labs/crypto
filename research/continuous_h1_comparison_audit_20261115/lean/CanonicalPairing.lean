module
public import GaloisCoefficients
public import CharTwoGaloisControl

@[expose] public section
namespace ContinuousH1ComparisonAudit
noncomputable section
open ProcyclicGaloisCocycleAudit GaloisCharacterPairingAudit

universe u
variable (K : Type u) [Field K] [Fintype K]
variable (M : Type u) [CommGroup M] [TopologicalSpace M] [DiscreteTopology M]
variable [TopologicalSpace (MulAut M)] [DiscreteTopology (MulAut M)]

abbrev ActualH1 (ρ : Gamma K →ₜ* MulAut M) :=
  Multiplicative (continuousCohomology 1 (coefficientObject K M ρ))

def classComparison (ρ : Gamma K →ₜ* MulAut M) :
    ActualH1 K M ρ ≃* CocycleClasses K M ρ :=
  (actualGaloisH1Equiv K M ρ).toAddEquiv.toMultiplicativeLeft

variable [Finite M]

def canonicalLeftDuality (ρ : Gamma K →ₜ* MulAut M)
    (hM : ∀ m : M, m ^ (Fintype.card K - 1) = 1) :
    galoisCharacters K M ρ ≃* (ActualH1 K M ρ →* Kˣ) :=
  (leftDuality K M ρ hM).trans (classComparison K M ρ).monoidHomCongrLeft.symm

def canonicalRightDuality (ρ : Gamma K →ₜ* MulAut M)
    (hM : ∀ m : M, m ^ (Fintype.card K - 1) = 1) :
    ActualH1 K M ρ ≃* (galoisCharacters K M ρ →* Kˣ) :=
  (classComparison K M ρ).trans (rightDuality K M ρ hM)

def canonicalPairing (ρ : Gamma K →ₜ* MulAut M)
    (hM : ∀ m : M, m ^ (Fintype.card K - 1) = 1)
    (χ : galoisCharacters K M ρ) (c : ActualH1 K M ρ) : Kˣ :=
  canonicalLeftDuality K M ρ hM χ c

theorem canonical_right_value (ρ : Gamma K →ₜ* MulAut M)
    (hM : ∀ m : M, m ^ (Fintype.card K - 1) = 1)
    (c : ActualH1 K M ρ) (χ : galoisCharacters K M ρ) :
    canonicalRightDuality K M ρ hM c χ = canonicalPairing K M ρ hM χ c :=
  rightDuality_value K M ρ hM (classComparison K M ρ c) χ

theorem canonical_left_nondegenerate (ρ : Gamma K →ₜ* MulAut M)
    (hM : ∀ m : M, m ^ (Fintype.card K - 1) = 1)
    (χ : galoisCharacters K M ρ) (h : ∀ c, canonicalPairing K M ρ hM χ c = 1) : χ = 1 := by
  apply (canonicalLeftDuality K M ρ hM).injective
  rw [map_one]
  exact MonoidHom.ext h

theorem canonical_right_nondegenerate (ρ : Gamma K →ₜ* MulAut M)
    (hM : ∀ m : M, m ^ (Fintype.card K - 1) = 1)
    (c : ActualH1 K M ρ) (h : ∀ χ, canonicalPairing K M ρ hM χ c = 1) : c = 1 := by
  apply (canonicalRightDuality K M ρ hM).injective
  rw [map_one]
  apply MonoidHom.ext
  intro χ
  exact (canonical_right_value K M ρ hM c χ).trans (h χ)

theorem canonical_homogeneous_formula (ρ : Gamma K →ₜ* MulAut M)
    (hM : ∀ m : M, m ^ (Fintype.card K - 1) = 1)
    (χ : galoisCharacters K M ρ) (f : KernelOne (coefficientObject K M ρ)) :
    baseUnitMap K (canonicalPairing K M ρ hM χ
      (Multiplicative.ofAdd (homogeneousKernelClass (coefficientObject K M ρ) f))) =
        χ.val (f.val.val 1 (frobenius K)).toMul := by
  let c := kernelCrossedEquiv (coefficientObject K M ρ) (coefficient_joint_continuous K M ρ) f
  have he : classComparison K M ρ
      (Multiplicative.ofAdd (homogeneousKernelClass (coefficientObject K M ρ) f)) =
        QuotientGroup.mk (toOldCocycle K M ρ c) :=
    congrArg Additive.toMul (actualGaloisH1_representative K M ρ f)
  change baseUnitMap K (pairing K M ρ hM χ
    (classComparison K M ρ (Multiplicative.ofAdd
      (homogeneousKernelClass (coefficientObject K M ρ) f)))) = _
  rw [he]
  exact source_pairing_formula K M ρ hM χ (toOldCocycle K M ρ c)

section CharTwo
local instance : TopologicalSpace TwoTarget := ⊥
local instance : DiscreteTopology TwoTarget := ⟨rfl⟩
local instance : TopologicalSpace (MulAut TwoTarget) := ⊥
local instance : DiscreteTopology (MulAut TwoTarget) := ⟨rfl⟩

variable (F : Type) [Field F] [Fintype F]

abbrev ActualTwoH1 := ActualH1 F TwoTarget (twoAction F)

def missingActualTwoClass : ActualTwoH1 F :=
  (classComparison F TwoTarget (twoAction F)).symm (missingTwoClass F)

theorem missing_actual_two_class_ne_one : missingActualTwoClass F ≠ 1 := by
  intro h
  apply missing_two_class_ne_one F
  have he := congrArg (classComparison F TwoTarget (twoAction F)) h
  simpa only [missingActualTwoClass, MulEquiv.apply_symm_apply, map_one] using he

variable [CharP F 2]

/-- The guard-deletion obstruction now holds on actual continuousCohomology1. -/
theorem canonical_annihilation_guard_needed :
    ¬ ∃ e : TwoCharacters F →* (ActualTwoH1 F →* Fˣ),
      ∀ c : ActualTwoH1 F, (∀ χ : TwoCharacters F, e χ c = 1) → c = 1 := by
  rintro ⟨e, he⟩
  apply missing_actual_two_class_ne_one F
  apply he
  intro χ
  rw [two_characters_trivial F χ, map_one]
  rfl
end CharTwo

#print axioms classComparison
#print axioms canonicalLeftDuality
#print axioms canonicalRightDuality
#print axioms canonical_homogeneous_formula
#print axioms canonical_annihilation_guard_needed
end
end ContinuousH1ComparisonAudit
