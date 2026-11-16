import ActualDualityControls

namespace FiniteTateDualityAudit
noncomputable section
open CharPPairingDomainAudit TwoTorsionRankAudit

def EvaluationPerfect {G : Type*} [CommGroup G] (σ : G ≃* G)
    (K : Type*) [CommMonoid K] : Prop :=
  (∀ χ : invariantCharacters σ K,(∀ q : Coinvariants σ,evaluate σ K χ q=1) → χ=1) ∧
  (∀ q : Coinvariants σ,(∀ χ : invariantCharacters σ K,evaluate σ K χ q=1) → q=1)

def ordinaryAction (K : Type*) [Field K] [DecidableEq K] [CharP K 2] :=
  MulEquiv.refl (ActualTwoGroup (ordinaryModel K))

def positiveAction := MulEquiv.refl (ActualTwoGroup positiveCurve)

theorem positive_target_card : Nat.card PositiveFieldˣ=2 := by
  classical
  rw [Nat.card_eq_fintype_card,Fintype.card_units,ZMod.card]

theorem positive_perfect : EvaluationPerfect positiveAction PositiveField := by
  have hK : ringChar PositiveField≠2 := by rw [ZMod.ringChar_zmod_n]; decide
  exact ⟨evaluation_left_nondegenerate _ _,
    evaluation_right_nondegenerate _ _ (odd_actual_annihilation hK positiveCurve)⟩

theorem ordinary_source_annihilation_false (K : Type*) [Field K] [DecidableEq K]
    [Fintype K] [CharP K 2] :
    ¬∀ g : ActualTwoGroup (ordinaryModel K),g^(Fintype.card K-1)=1 := by
  intro h
  obtain ⟨q,hq,he⟩ := char_two_actual_right_degeneracy (K:=K)
  exact hq (evaluation_right_nondegenerate (ordinaryAction K) K h q he)

/-- Algebraic finite character/coinvariant core of Bruin2011 Lemma2.1,
with actual point-group controls. No H1/Lang/Cartier/divisor bridge is assumed. -/
theorem finite_character_duality_certificate :
    (∀ (K : Type) [Field K] [Fintype K],HasEnoughRootsOfUnity K (Fintype.card K-1)) ∧
    (∀ (K : Type) [Field K] [Fintype K] (G : Type) [CommGroup G] [Finite G]
      (σ : G ≃* G),(∀ g : G,g^(Fintype.card K-1)=1) →
      (∃ l : invariantCharacters σ K ≃* (Coinvariants σ →* Kˣ),
        ∀ χ q,l χ q=evaluate σ K χ q) ∧
      (∃ r : Coinvariants σ ≃* (invariantCharacters σ K →* Kˣ),
        ∀ q χ,r q χ=evaluate σ K χ q) ∧
      Nat.card (invariantCharacters σ K)=Nat.card (Coinvariants σ) ∧ EvaluationPerfect σ K) ∧
    (∀ (K : Type) [Field K] [Fintype K] [DecidableEq K],ringChar K≠2 →
      ∀ (W : WeierstrassCurve.Affine K) (σ : ActualTwoGroup W ≃* ActualTwoGroup W),
      (∀ g : ActualTwoGroup W,g^(Fintype.card K-1)=1) ∧
      Nat.card (invariantCharacters σ K)=Nat.card (Coinvariants σ) ∧ EvaluationPerfect σ K) ∧
    (positiveCurve.Δ≠0 ∧ addOrderOf positivePoint=2 ∧
      Nat.card (ActualTwoGroup positiveCurve)=4 ∧
      Nat.card (invariantCharacters positiveAction PositiveField)=4 ∧
      Nat.card (Coinvariants positiveAction)=4 ∧ Nat.card PositiveFieldˣ=2 ∧
      EvaluationPerfect positiveAction PositiveField) ∧
    (∀ (K : Type) [Field K] [Fintype K] [DecidableEq K] [CharP K 2],
      Nat.card (invariantCharacters (ordinaryAction K) K)=1 ∧
      Nat.card (Coinvariants (ordinaryAction K))=2 ∧
      (¬∀ g : ActualTwoGroup (ordinaryModel K),g^(Fintype.card K-1)=1) ∧
      ∃ q : Coinvariants (ordinaryAction K),q≠1 ∧
        ∀ χ : invariantCharacters (ordinaryAction K) K,evaluate _ K χ q=1) ∧
    (¬∀ (F : Type) [Field F] [Fintype F] (G : Type) [CommGroup G] [Finite G]
      (σ : G ≃* G) (q : Coinvariants σ),
      (∀ χ : invariantCharacters σ F,evaluate σ F χ q=1) → q=1) := by
  refine ⟨?_,?_,?_,?_,?_,?_⟩
  · intro K _ _
    exact field_card_roots K
  · intro K _ _ G _ _ σ h
    exact ⟨⟨characterDescent σ K,fun _ _ => rfl⟩,
      ⟨rightEvaluationIso σ K h,rightEvaluationIso_apply σ K h⟩,
      character_coinvariant_card σ K h,
      evaluation_left_nondegenerate σ K,evaluation_right_nondegenerate σ K h⟩
  · intro K _ _ _ hK W σ
    have h := odd_actual_annihilation hK W
    exact ⟨h,character_coinvariant_card σ K h,
      evaluation_left_nondegenerate σ K,evaluation_right_nondegenerate σ K h⟩
  · exact ⟨positive_delta,positive_point_order,positive_two_card,
      positive_identity_sizes.1,positive_identity_sizes.2,positive_target_card,positive_perfect⟩
  · intro K _ _ _ _
    exact ⟨(char_two_ordinary_sizes (K:=K)).1,(char_two_ordinary_sizes (K:=K)).2,
      ordinary_source_annihilation_false K,char_two_actual_right_degeneracy⟩
  · exact deleting_annihilation_false

#print axioms positive_perfect
#print axioms ordinary_source_annihilation_false
#print axioms finite_character_duality_certificate
end
end FiniteTateDualityAudit
