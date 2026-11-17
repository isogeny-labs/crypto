import FiveConverse
import FiveSeparable

namespace FiveTorsionRankAudit
noncomputable section
open WeierstrassCurve.Affine ThreeTorsionRankAudit.General
variable {K : Type*} [Field K] [DecidableEq K]

def fiveSubgroup (W : WeierstrassCurve.Affine K) : AddSubgroup W.Point where
  carrier := {P | (5:ℕ) • P=0}
  zero_mem' := by simp
  add_mem' := by
    intro P Q hP hQ
    simp only [Set.mem_ofPred_eq] at *
    rw [nsmul_add,hP,hQ,zero_add]
  neg_mem' := by
    intro P hP
    simp only [Set.mem_ofPred_eq] at *
    rw [smul_neg,hP,neg_zero]

abbrev FiveTorsion (W : WeierstrassCurve.Affine K) := ↥(fiveSubgroup W)
abbrev FiveFibers (W : WeierstrassCurve.Affine K) := Σ x : FiveRoot W, Ordinate W x.val
def AffineFive (W : WeierstrassCurve.Affine K) := {xy : K × K //
  ∃ h : W.Nonsingular xy.1 xy.2, (5:ℕ) • Point.some _ _ h=0}

def affineFiveFiberEquiv (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) :
    AffineFive W ≃ FiveFibers W where
  toFun A := ⟨⟨A.val.1,(five_point_iff W A.property.choose).mp A.property.choose_spec⟩,
    ⟨A.val.2,A.property.choose.1⟩⟩
  invFun A := by
    have hn : W.Nonsingular A.1.val A.2.val :=
      (W.equation_iff_nonsingular_of_Δ_ne_zero hΔ).mp A.2.property
    exact ⟨(A.1.val,A.2.val),hn,(five_point_iff W hn).mpr A.1.property⟩
  left_inv A := by apply Subtype.ext; rfl
  right_inv := by rintro ⟨⟨x,hx⟩,⟨y,hy⟩⟩; rfl

def pointFiveFiberEquiv (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) :
    FiveTorsion W ≃ Option (FiveFibers W) := by
  have hz : (5:ℕ) • (Point.zero : W.Point)=0 := by
    change (5:ℕ) • (0:W.Point)=0
    exact smul_zero _
  let e : FiveTorsion W ≃ Option (AffineFive W) :=
    W.nonsingularPointEquivSubtype (p:=fun P => (5:ℕ) • P=0) hz
  exact e.trans (affineFiveFiberEquiv W hΔ).optionCongr

theorem five_torsion_finite [NeZero (5:K)] (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) :
    Finite (FiveTorsion W) := Finite.of_equiv (Option (FiveFibers W)) (pointFiveFiberEquiv W hΔ).symm

theorem five_fibers_card [IsAlgClosed K] [NeZero (5:K)]
    (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) : Nat.card (FiveFibers W)=24 := by
  rw [Nat.card_eq_fintype_card,Fintype.card_sigma]
  have hc (x : FiveRoot W) : Fintype.card (Ordinate W x.val)=2 := by
    rw [←Nat.card_eq_fintype_card]
    exact five_ordinate_card W hΔ x.property
  simp_rw [hc]
  simp only [Finset.sum_const,Finset.card_univ,smul_eq_mul]
  rw [←Nat.card_eq_fintype_card,five_root_card W hΔ]

theorem geometric_five_card [IsAlgClosed K] [NeZero (5:K)]
    (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) : Nat.card (FiveTorsion W)=25 := by
  rw [Nat.card_congr (pointFiveFiberEquiv W hΔ),Finite.card_option,five_fibers_card W hΔ]

#print axioms pointFiveFiberEquiv
#print axioms geometric_five_card
end
end FiveTorsionRankAudit
