module

public import FieldCharacterRoots
public import CoinvariantCharacters

@[expose] public section
namespace FiniteTateDualityAudit
noncomputable section
variable {G : Type*} [CommGroup G] (σ : G ≃* G)
variable (K : Type*) [Field K] [Fintype K]

omit [Field K] in
theorem coinvariant_annihilation (h : ∀ g : G,g^(Fintype.card K-1)=1) :
    ∀ q : Coinvariants σ,q^(Fintype.card K-1)=1 := by
  intro q
  refine QuotientGroup.induction_on q fun x => ?_
  calc
    (QuotientGroup.mk x : Coinvariants σ)^(Fintype.card K-1)=
      QuotientGroup.mk (x^(Fintype.card K-1)) :=
        (map_pow (QuotientGroup.mk' _) x _).symm
    _=1 := by rw [h x]; rfl

theorem coinvariant_roots (h : ∀ g : G,g^(Fintype.card K-1)=1) :
    HasEnoughRootsOfUnity K (Monoid.exponent (Coinvariants σ)) :=
  roots_of_annihilation K (coinvariant_annihilation σ K h)

variable [Finite G]

/-- The right evaluation map is an isomorphism, rather than merely a
nondegenerate map. Only the source (q-1)-annihilation condition is assumed. -/
def rightEvaluationIso (h : ∀ g : G,g^(Fintype.card K-1)=1) :
    Coinvariants σ ≃* (invariantCharacters σ K →* Kˣ) := by
  have : HasEnoughRootsOfUnity K (Monoid.exponent (Coinvariants σ)) :=
    coinvariant_roots σ K h
  exact (CommGroup.monoidHomMonoidHomEquiv (Coinvariants σ) K).symm.trans
    (characterDescent σ K).monoidHomCongrLeft.symm

theorem rightEvaluationIso_apply (h : ∀ g : G,g^(Fintype.card K-1)=1)
    (q : Coinvariants σ) (χ : invariantCharacters σ K) :
    rightEvaluationIso σ K h q χ=evaluate σ K χ q := by
  have : HasEnoughRootsOfUnity K (Monoid.exponent (Coinvariants σ)) :=
    coinvariant_roots σ K h
  change (CommGroup.monoidHomMonoidHomEquiv (Coinvariants σ) K).symm q
    (characterDescent σ K χ)=evaluate σ K χ q
  rw [CommGroup.monoidHomMonoidHomEquiv_symm_apply_apply]
  rfl

theorem character_coinvariant_card (h : ∀ g : G,g^(Fintype.card K-1)=1) :
    Nat.card (invariantCharacters σ K)=Nat.card (Coinvariants σ) := by
  have : HasEnoughRootsOfUnity K (Monoid.exponent (Coinvariants σ)) :=
    coinvariant_roots σ K h
  exact (Nat.card_congr (characterDescent σ K).toEquiv).trans
    (CommGroup.card_monoidHom_of_hasEnoughRootsOfUnity (Coinvariants σ) K)

omit [Finite G] [Fintype K] in
/-- Descent supplies the entire left character group even before imposing
an exponent/root condition. -/
theorem evaluation_left_nondegenerate (χ : invariantCharacters σ K)
    (h : ∀ q : Coinvariants σ,evaluate σ K χ q=1) : χ=1 := by
  apply (characterDescent σ K).injective
  rw [map_one]
  apply MonoidHom.ext
  intro q
  exact h q

theorem evaluation_right_nondegenerate (h : ∀ g : G,g^(Fintype.card K-1)=1)
    (q : Coinvariants σ) (hq : ∀ χ : invariantCharacters σ K,evaluate σ K χ q=1) : q=1 := by
  apply (rightEvaluationIso σ K h).injective
  rw [map_one]
  apply MonoidHom.ext
  intro χ
  exact (rightEvaluationIso_apply σ K h q χ).trans (hq χ)

#print axioms rightEvaluationIso
#print axioms rightEvaluationIso_apply
#print axioms character_coinvariant_card
#print axioms evaluation_left_nondegenerate
#print axioms evaluation_right_nondegenerate
end
end FiniteTateDualityAudit
