module
public import GaloisCharacters
public import CocycleClasses

@[expose] public section
namespace GaloisCharacterPairingAudit
noncomputable section
open ProcyclicGaloisCocycleAudit

variable (K : Type*) [Field K] [Fintype K]
variable (M : Type*) [CommGroup M] [Finite M]
variable [TopologicalSpace M] [DiscreteTopology M]
variable [TopologicalSpace (MulAut M)] [DiscreteTopology (MulAut M)]

def classCoinvariantEquiv (ρ : Gamma K →ₜ* MulAut M) :
    CocycleClasses K M ρ ≃* FiniteTateDualityAudit.Coinvariants (ρ (frobenius K)) :=
  classEvaluationEquiv K M ρ

theorem classCoinvariant_mk (ρ : Gamma K →ₜ* MulAut M) (c : Cocycles K M ρ) :
    classCoinvariantEquiv K M ρ (QuotientGroup.mk c) =
      QuotientGroup.mk (c.val (frobenius K)) := rfl

/-- The complete left duality map, not an assumed perfectness premise. -/
def leftDuality (ρ : Gamma K →ₜ* MulAut M)
    (hM : ∀ m : M, m ^ (Fintype.card K - 1) = 1) :
    galoisCharacters K M ρ ≃* (CocycleClasses K M ρ →* Kˣ) :=
  ((galoisCharacterEquiv K M ρ hM).trans
    (FiniteTateDualityAudit.characterDescent (ρ (frobenius K)) K)).trans
      (classCoinvariantEquiv K M ρ).monoidHomCongrLeft.symm

/-- The complete right duality map, derived from actual finite-field roots
and the credited finite Abelian biduality theorem. -/
def rightDuality (ρ : Gamma K →ₜ* MulAut M)
    (hM : ∀ m : M, m ^ (Fintype.card K - 1) = 1) :
    CocycleClasses K M ρ ≃* (galoisCharacters K M ρ →* Kˣ) :=
  ((classCoinvariantEquiv K M ρ).trans
    (FiniteTateDualityAudit.rightEvaluationIso (ρ (frobenius K)) K hM)).trans
      (galoisCharacterEquiv K M ρ hM).monoidHomCongrLeft.symm

def pairing (ρ : Gamma K →ₜ* MulAut M)
    (hM : ∀ m : M, m ^ (Fintype.card K - 1) = 1)
    (χ : galoisCharacters K M ρ) (c : CocycleClasses K M ρ) : Kˣ :=
  leftDuality K M ρ hM χ c

theorem rightDuality_value (ρ : Gamma K →ₜ* MulAut M)
    (hM : ∀ m : M, m ^ (Fintype.card K - 1) = 1)
    (c : CocycleClasses K M ρ) (χ : galoisCharacters K M ρ) :
    rightDuality K M ρ hM c χ = pairing K M ρ hM χ c := by
  exact FiniteTateDualityAudit.rightEvaluationIso_apply (ρ (frobenius K)) K hM
    (classCoinvariantEquiv K M ρ c) (galoisCharacterEquiv K M ρ hM χ)

theorem pairing_representative (ρ : Gamma K →ₜ* MulAut M)
    (hM : ∀ m : M, m ^ (Fintype.card K - 1) = 1)
    (χ : galoisCharacters K M ρ) (c : Cocycles K M ρ) :
    pairing K M ρ hM χ (QuotientGroup.mk c) =
      (galoisCharacterEquiv K M ρ hM χ).val (c.val (frobenius K)) := rfl

/-- The exact displayed source formula, with its value proved to lie in K*. -/
theorem source_pairing_formula (ρ : Gamma K →ₜ* MulAut M)
    (hM : ∀ m : M, m ^ (Fintype.card K - 1) = 1)
    (χ : galoisCharacters K M ρ) (c : Cocycles K M ρ) :
    baseUnitMap K (pairing K M ρ hM χ (QuotientGroup.mk c)) = χ.val (c.val (frobenius K)) := by
  rw [pairing_representative]
  exact galoisCharacterEquiv_value K M ρ hM χ _

theorem pairing_left (ρ : Gamma K →ₜ* MulAut M)
    (hM : ∀ m : M, m ^ (Fintype.card K - 1) = 1)
    (χ ψ : galoisCharacters K M ρ) (c : CocycleClasses K M ρ) :
    pairing K M ρ hM (χ * ψ) c = pairing K M ρ hM χ c * pairing K M ρ hM ψ c :=
  congrArg (fun f : CocycleClasses K M ρ →* Kˣ => f c) ((leftDuality K M ρ hM).map_mul χ ψ)

theorem pairing_right (ρ : Gamma K →ₜ* MulAut M)
    (hM : ∀ m : M, m ^ (Fintype.card K - 1) = 1)
    (χ : galoisCharacters K M ρ) (c d : CocycleClasses K M ρ) :
    pairing K M ρ hM χ (c * d) = pairing K M ρ hM χ c * pairing K M ρ hM χ d :=
  (leftDuality K M ρ hM χ).map_mul c d

theorem pairing_left_nondegenerate (ρ : Gamma K →ₜ* MulAut M)
    (hM : ∀ m : M, m ^ (Fintype.card K - 1) = 1)
    (χ : galoisCharacters K M ρ) (h : ∀ c, pairing K M ρ hM χ c = 1) : χ = 1 := by
  apply (leftDuality K M ρ hM).injective
  rw [map_one]
  apply MonoidHom.ext
  exact h

theorem pairing_right_nondegenerate (ρ : Gamma K →ₜ* MulAut M)
    (hM : ∀ m : M, m ^ (Fintype.card K - 1) = 1)
    (c : CocycleClasses K M ρ) (h : ∀ χ, pairing K M ρ hM χ c = 1) : c = 1 := by
  apply (rightDuality K M ρ hM).injective
  rw [map_one]
  apply MonoidHom.ext
  intro χ
  exact (rightDuality_value K M ρ hM c χ).trans (h χ)

theorem duality_cardinality (ρ : Gamma K →ₜ* MulAut M)
    (hM : ∀ m : M, m ^ (Fintype.card K - 1) = 1) :
    Nat.card (galoisCharacters K M ρ) = Nat.card (CocycleClasses K M ρ) :=
  (Nat.card_congr (galoisCharacterEquiv K M ρ hM).toEquiv).trans
    ((FiniteTateDualityAudit.character_coinvariant_card (ρ (frobenius K)) K hM).trans
      (Nat.card_congr (classCoinvariantEquiv K M ρ).toEquiv).symm)

#print axioms leftDuality
#print axioms rightDuality
#print axioms source_pairing_formula
#print axioms pairing_right_nondegenerate
end
end GaloisCharacterPairingAudit
