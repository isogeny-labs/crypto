import PerfectEvaluation
import CharTwoDomains

namespace FiniteTateDualityAudit
noncomputable section
open CharPPairingDomainAudit TwoTorsionRankAudit
private instance : Fact (Nat.Prime 2) := ⟨by decide⟩

/-- Identity action has the whole actual group as its coinvariant quotient. -/
def identityCoinvariantEquiv (G : Type*) [CommGroup G] :
    Coinvariants (MulEquiv.refl G) ≃* G := by
  have hr : (difference (MulEquiv.refl G)).range=⊥ := by
    ext x
    constructor
    · rintro ⟨y,rfl⟩
      change y/y=1
      exact div_self' y
    · intro hx
      change x=1 at hx
      subst x
      exact ⟨1,by simp [difference]⟩
  change (G ⧸ (difference (MulEquiv.refl G)).range) ≃* G
  rw [hr]
  exact QuotientGroup.quotientBot

variable {K : Type*} [Field K] [DecidableEq K]
abbrev ActualTwoGroup (W : WeierstrassCurve.Affine K) := Multiplicative (actualLeft W 2)

instance actualTwoFinite [Finite K] (W : WeierstrassCurve.Affine K) : Finite (ActualTwoGroup W) := by
  have : Finite W.Point := BKLinearAudit.actual_rational_point_finite W
  infer_instance

theorem actual_two_square (W : WeierstrassCurve.Affine K) (g : ActualTwoGroup W) : g^2=1 := by
  change (2:ℕ) • g.toAdd=0
  exact actual_left_killed W 2 g.toAdd

theorem odd_actual_annihilation [Fintype K] (hK : ringChar K≠2)
    (W : WeierstrassCurve.Affine K) :
    ∀ g : ActualTwoGroup W,g^(Fintype.card K-1)=1 := by
  have hodd := FiniteField.odd_card_of_char_ne_two hK
  have hd : 2∣Fintype.card K-1 := by omega
  obtain ⟨m,hm⟩ := hd
  intro g
  rw [hm,pow_mul,actual_two_square W g,one_pow]

theorem odd_actual_two_duality [Fintype K] (hK : ringChar K≠2)
    (W : WeierstrassCurve.Affine K) (σ : ActualTwoGroup W ≃* ActualTwoGroup W) :
    Nat.card (invariantCharacters σ K)=Nat.card (Coinvariants σ) ∧
    ∀ q : Coinvariants σ,(∀ χ : invariantCharacters σ K,evaluate σ K χ q=1) → q=1 :=
  ⟨character_coinvariant_card σ K (odd_actual_annihilation hK W),
    evaluation_right_nondegenerate σ K (odd_actual_annihilation hK W)⟩

section CharTwo
variable [CharP K 2]

theorem char_two_characters_only_one (W : WeierstrassCurve.Affine K)
    (σ : ActualTwoGroup W ≃* ActualTwoGroup W) (χ : invariantCharacters σ K) : χ=1 := by
  apply Subtype.ext
  apply MonoidHom.ext
  intro g
  have hp : (χ.val g)^2=1 := by rw [←map_pow,actual_two_square,map_one]
  exact unit_prime_power_root 2 1 (χ.val g) (by simpa only [pow_one] using hp)

theorem char_two_invariant_card (W : WeierstrassCurve.Affine K)
    (σ : ActualTwoGroup W ≃* ActualTwoGroup W) : Nat.card (invariantCharacters σ K)=1 :=
  Nat.card_eq_one_iff_exists.mpr ⟨1,char_two_characters_only_one W σ⟩

theorem char_two_ordinary_sizes [Finite K] :
    Nat.card (invariantCharacters (MulEquiv.refl (ActualTwoGroup (ordinaryModel K))) K)=1 ∧
      Nat.card (Coinvariants (MulEquiv.refl (ActualTwoGroup (ordinaryModel K))))=2 := by
  refine ⟨char_two_invariant_card _ _,?_⟩
  have hc := Nat.card_congr (identityCoinvariantEquiv (ActualTwoGroup (ordinaryModel K))).toEquiv
  have hg : Nat.card (ActualTwoGroup (ordinaryModel K))=2 :=
    (ordinary_model_domain_cards (K:=K)).1
  exact hc.trans hg

theorem char_two_actual_right_degeneracy :
    ∃ q : Coinvariants (MulEquiv.refl (ActualTwoGroup (ordinaryModel K))),q≠1 ∧
      ∀ χ : invariantCharacters (MulEquiv.refl (ActualTwoGroup (ordinaryModel K))) K,
        evaluate _ K χ q=1 := by
  let P : actualLeft (ordinaryModel K) 2 := ⟨ordinaryPoint K,ordinaryPoint_killed⟩
  let g : ActualTwoGroup (ordinaryModel K) := Multiplicative.ofAdd P
  have hg : g≠1 := nonzero_actual_left _ 2 _ ordinaryPoint_nonzero ordinaryPoint_killed
  let e := identityCoinvariantEquiv (ActualTwoGroup (ordinaryModel K))
  refine ⟨e.symm g,?_,?_⟩
  · intro hz
    apply hg
    have h := congrArg e hz
    simpa only [e.apply_symm_apply,map_one] using h
  · intro χ
    rw [char_two_characters_only_one (ordinaryModel K) _ χ]
    change (characterDescent _ K 1) (e.symm g)=1
    rw [map_one]
    rfl
end CharTwo

/-- All finiteness/field/automorphism hypotheses remain; only the source
annihilation assumption has been dropped. The witness is actual curve torsion. -/
theorem deleting_annihilation_false :
    ¬∀ (F : Type) [Field F] [Fintype F] (G : Type) [CommGroup G] [Finite G]
      (σ : G ≃* G) (q : Coinvariants σ),
      (∀ χ : invariantCharacters σ F,evaluate σ F χ q=1) → q=1 := by
  intro h
  obtain ⟨q,hq,he⟩ := char_two_actual_right_degeneracy (K:=ZMod 2)
  exact hq (h (ZMod 2) (ActualTwoGroup (ordinaryModel (ZMod 2))) (.refl _) q he)

section Positive
private instance : Fact (Nat.Prime 3) := ⟨by decide⟩
abbrev PositiveField := ZMod 3
def positiveCurve : WeierstrassCurve.Affine PositiveField := ⟨0,0,0,2,0⟩

theorem positive_delta : positiveCurve.Δ≠0 := by decide +kernel

def positivePoint : positiveCurve.Point := .some 0 0
  ((positiveCurve.equation_iff_nonsingular_of_Δ_ne_zero positive_delta).mp (by
    rw [WeierstrassCurve.Affine.equation_iff]
    decide +kernel))

theorem positive_point_killed : (2:ℕ) • positivePoint=0 := by decide +kernel

theorem positive_point_order : addOrderOf positivePoint=2 :=
  addOrderOf_eq_prime positive_point_killed (WeierstrassCurve.Affine.Point.some_ne_zero _)

theorem every_positive_root :
    ∀ x : PositiveField,positiveCurve.twoTorsionPolynomial.toPoly.IsRoot x := by
  simp only [Polynomial.IsRoot.def,WeierstrassCurve.eval_toPoly_twoTorsionPolynomial,
    positiveCurve,WeierstrassCurve.b₂,WeierstrassCurve.b₄,WeierstrassCurve.b₆]
  decide +kernel

def positiveRootEquiv : Root positiveCurve ≃ PositiveField where
  toFun x := x.val
  invFun x := ⟨x,every_positive_root x⟩
  left_inv x := by apply Subtype.ext; rfl
  right_inv _ := rfl

theorem positive_two_card : Nat.card (ActualTwoGroup positiveCurve)=4 := by
  have : NeZero (2:PositiveField) := ⟨by decide⟩
  have hr : Nat.card (Root positiveCurve)=3 :=
    (Nat.card_congr positiveRootEquiv).trans (by simp [PositiveField])
  change Nat.card (TwoTorsion positiveCurve)=4
  rw [Nat.card_congr (pointRootEquiv positiveCurve positive_delta),Finite.card_option,hr]

theorem positive_identity_sizes :
    Nat.card (invariantCharacters (MulEquiv.refl (ActualTwoGroup positiveCurve)) PositiveField)=4 ∧
      Nat.card (Coinvariants (MulEquiv.refl (ActualTwoGroup positiveCurve)))=4 := by
  have hK : ringChar PositiveField≠2 := by
    rw [ZMod.ringChar_zmod_n]
    decide
  have hq := (Nat.card_congr (identityCoinvariantEquiv (ActualTwoGroup positiveCurve)).toEquiv).trans
    positive_two_card
  exact ⟨(odd_actual_two_duality hK positiveCurve (.refl _)).1.trans hq,hq⟩
end Positive

#print axioms positive_delta
#print axioms positive_point_order
#print axioms positive_two_card
#print axioms positive_identity_sizes
#print axioms odd_actual_two_duality
#print axioms char_two_ordinary_sizes
#print axioms char_two_actual_right_degeneracy
#print axioms deleting_annihilation_false
end
end FiniteTateDualityAudit
