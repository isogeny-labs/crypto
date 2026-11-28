import CharTwoCardinality
import SourceThreeCriterion

namespace BKSmallPrimeSourceAudit
noncomputable section
open WeierstrassCurve.Affine TwoTorsionRankAudit
variable {K : Type*} [Field K] [DecidableEq K] [CharP K 2]

theorem char_two_card_one_or_two (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) :
    Nat.card (TwoTorsion W)=1 ∨ Nat.card (TwoTorsion W)=2 := by
  by_cases ha : W.a₁=0
  · exact Or.inl (char_two_torsion_card_one W hΔ ha)
  · by_cases hex : ∃ P : W.Point,P≠0 ∧ (2:ℕ) • P=0
    · right
      obtain ⟨P,hP,ht⟩ := hex
      apply torsion_card_two_of_classification W P hP ht
      intro Q hQ
      by_cases hz : Q=0
      · exact Or.inl hz
      · exact Or.inr (nonzero_killed_unique W ha Q P hz hP hQ ht)
    · left
      apply Nat.card_eq_one_iff_exists.mpr
      refine ⟨0,fun P => ?_⟩
      apply Subtype.ext
      by_contra hP
      exact hex ⟨P.val,hP,P.property⟩

theorem char_two_finite (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) :
    Finite (TwoTorsion W) := by
  apply Nat.finite_of_card_ne_zero
  rcases char_two_card_one_or_two W hΔ with h | h <;> rw [h] <;> decide

theorem char_two_ne_four (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) :
    Nat.card (TwoTorsion W)≠4 := by
  rcases char_two_card_one_or_two W hΔ with h | h <;> rw [h] <;> decide

#print axioms char_two_card_one_or_two
end
end BKSmallPrimeSourceAudit
