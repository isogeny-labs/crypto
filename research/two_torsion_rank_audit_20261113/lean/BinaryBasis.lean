import BaseControl

namespace TwoTorsionRankAudit
noncomputable section

/-- Concrete two-generator basis: four-point generation and independence
of actual F2 coefficient values under the group's actual scalar action. -/
def BinaryBasis {G : Type*} [AddCommGroup G] (A B : G) : Prop :=
  A≠0 ∧ B≠0 ∧ A≠B ∧ (∀ P : G,P=0 ∨ P=A ∨ P=B ∨ P=A+B) ∧
    (∀ a b : ZMod 2,a.val • A+b.val • B=0 ↔ a=0 ∧ b=0)

theorem four_group_binary_basis {G : Type*} [AddCommGroup G] [Finite G]
    (hc : Nat.card G=4) (hk : ∀ P : G,2 • P=0) : ∃ A B : G,BinaryBasis A B := by
  classical
  letI := Fintype.ofFinite G
  have hG : Fintype.card G=4 := by simpa only [Nat.card_eq_fintype_card] using hc
  have hA : ∃ A : G,A≠0 := by
    by_contra h
    have hz : ∀ A : G,A=0 := by
      intro A
      by_contra hn
      exact h ⟨A,hn⟩
    have hs : Subsingleton G := ⟨fun x y => (hz x).trans (hz y).symm⟩
    have hh := (Finite.card_le_one_iff_subsingleton (α:=G)).mpr hs
    omega
  obtain ⟨A,hA⟩ := hA
  have hB : ∃ B : G,B≠0 ∧ B≠A := by
    by_contra h
    have hcover : (Finset.univ : Finset G)⊆{0,A} := by
      intro B _
      by_cases hz : B=0
      · simp [hz]
      · have he : B=A := by
          by_contra hn
          exact h ⟨B,hz,hn⟩
        simp [he]
    have hh := Finset.card_le_card hcover
    have hsize : ({0,A}:Finset G).card=2 := Finset.card_pair (Ne.symm hA)
    rw [Finset.card_univ,hG,hsize] at hh
    omega
  obtain ⟨B,hB,hBA⟩ := hB
  have hneg (P : G) : -P=P := by
    have hp : P+P=0 := by simpa only [two_nsmul] using hk P
    exact (add_eq_zero_iff_eq_neg.mp hp).symm
  have hab0 : A+B≠0 := by
    intro h
    have he : A=B := (add_eq_zero_iff_eq_neg.mp h).trans (hneg B)
    exact hBA he.symm
  have habA : A+B≠A := by
    intro h
    have he : A+B=A+0 := by simpa only [add_zero] using h
    exact hB (add_left_cancel he)
  have habB : A+B≠B := by
    intro h
    have he : A+B=0+B := by simpa only [zero_add] using h
    exact hA (add_right_cancel he)
  have hsize : ({0,A,B,A+B}:Finset G).card=4 := by
    have hn0 : (0:G)∉({A,B,A+B}:Finset G) := by
      simp only [Finset.mem_insert,Finset.mem_singleton,not_or]
      exact ⟨Ne.symm hA,Ne.symm hB,Ne.symm hab0⟩
    have hnA : A∉({B,A+B}:Finset G) := by
      simp only [Finset.mem_insert,Finset.mem_singleton,not_or]
      exact ⟨Ne.symm hBA,Ne.symm habA⟩
    have hnB : B∉({A+B}:Finset G) := by
      simp only [Finset.mem_singleton]
      exact Ne.symm habB
    rw [Finset.card_insert_of_notMem hn0,Finset.card_insert_of_notMem hnA,
      Finset.card_insert_of_notMem hnB,Finset.card_singleton]
  have he : ({0,A,B,A+B}:Finset G)=Finset.univ := by
    apply Finset.eq_univ_of_card
    rw [hsize,hG]
  have hall (P : G) : P=0 ∨ P=A ∨ P=B ∨ P=A+B := by
    have hm : P∈({0,A,B,A+B}:Finset G) := by rw [he]; exact Finset.mem_univ _
    simpa only [Finset.mem_insert,Finset.mem_singleton] using hm
  refine ⟨A,B,hA,hB,Ne.symm hBA,hall,?_⟩
  have hz : (0 : ZMod 2).val=0 := rfl
  have ho : (1 : ZMod 2).val=1 := rfl
  have hcases : ∀ a : ZMod 2,a=0 ∨ a=1 := by decide +kernel
  intro a b
  rcases hcases a with rfl | rfl <;> rcases hcases b with rfl | rfl <;>
    simp [hz,ho,hA,hB,hab0]

variable {K : Type*} [Field K] [DecidableEq K]

theorem actual_torsion_two_killed (W : WeierstrassCurve.Affine K) (P : TwoTorsion W) :
    2 • P=0 := by
  apply Subtype.ext
  change 2 • P.val=0
  exact P.property

theorem actual_geometric_binary_basis [NeZero (2 : K)] [IsAlgClosed K]
    (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) :
    ∃ A B : TwoTorsion W,BinaryBasis A B := by
  letI := torsion_finite W hΔ
  exact four_group_binary_basis (geometric_two_torsion_card W hΔ) (actual_torsion_two_killed W)

#print axioms actual_geometric_binary_basis
end
end TwoTorsionRankAudit
