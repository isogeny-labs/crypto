import CharTwoGeometry

namespace TwoTorsionRankAudit
open Polynomial WeierstrassCurve.Affine
noncomputable section
variable {K : Type*} [Field K] [DecidableEq K]

theorem torsion_card_two_of_classification (W : WeierstrassCurve.Affine K)
    (P : W.Point) (hP : P≠0) (hk : 2 • P=0)
    (hall : ∀ Q : W.Point,2 • Q=0 → Q=0 ∨ Q=P) : Nat.card (TwoTorsion W)=2 := by
  have he : (torsion W : Set W.Point)=↑({0,P}:Finset W.Point) := by
    ext Q
    change (2 • Q=0) ↔ Q∈({0,P}:Finset W.Point)
    simp only [Finset.mem_insert,Finset.mem_singleton]
    constructor
    · exact hall Q
    · rintro (rfl | rfl)
      · exact smul_zero _
      · exact hk
  calc
    Nat.card (TwoTorsion W)=Nat.card ↥({0,P}:Finset W.Point) :=
      Nat.card_congr (Set.equivOfEq he)
    _ = 2 := by
      rw [Nat.card_eq_fintype_card,Fintype.card_coe]
      exact Finset.card_pair (Ne.symm hP)

theorem char_two_torsion_card_two [CharP K 2] [IsAlgClosed K]
    (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (ha : W.a₁≠0) :
    Nat.card (TwoTorsion W)=2 := by
  obtain ⟨P,hP,hkP⟩ := ordinary_nonzero_killed W hΔ ha
  apply torsion_card_two_of_classification W P hP hkP
  intro Q hkQ
  by_cases hQ : Q=0
  · exact Or.inl hQ
  · exact Or.inr (nonzero_killed_unique W ha Q P hQ hP hkQ hkP)

theorem char_two_geometric_classification [CharP K 2] [IsAlgClosed K]
    (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) :
    Nat.card (TwoTorsion W)=if W.a₁=0 then 1 else 2 := by
  split_ifs with ha
  · exact char_two_torsion_card_one W hΔ ha
  · exact char_two_torsion_card_two W hΔ ha

#print axioms char_two_geometric_classification
end
end TwoTorsionRankAudit
