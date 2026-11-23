import GeneralRoots
import GeometricCardinality

namespace ThreeTorsionRankAudit.General
open WeierstrassCurve.Affine
noncomputable section
variable {K : Type*} [Field K] [DecidableEq K]

def torsion (W : WeierstrassCurve.Affine K) : AddSubgroup W.Point where
  carrier := {P | 3 • P=0}
  zero_mem' := by simp
  add_mem' := by intro P Q hP hQ; simp only [Set.mem_ofPred_eq] at *; rw [nsmul_add,hP,hQ,zero_add]
  neg_mem' := by intro P hP; simp only [Set.mem_ofPred_eq] at *; rw [smul_neg,hP,neg_zero]

abbrev ThreeTorsion (W : WeierstrassCurve.Affine K) := ↥(torsion W)
def AffineThree (W : WeierstrassCurve.Affine K) := {xy : K × K //
  ∃ h : W.Nonsingular xy.1 xy.2, (3 : ℕ) • Point.some _ _ h=0}

def affineFiberEquiv (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0) :
    AffineThree W ≃ Fibers W where
  toFun A := ⟨⟨A.val.1,(three_point_iff W A.property.choose).mp A.property.choose_spec⟩,
    ⟨A.val.2,A.property.choose.1⟩⟩
  invFun A := by
    have hn : W.Nonsingular A.1.val A.2.val :=
      (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero hΔ).mp A.2.property
    exact ⟨(A.1.val,A.2.val),hn,(three_point_iff W hn).mpr A.1.property⟩
  left_inv A := by apply Subtype.ext; rfl
  right_inv := by rintro ⟨⟨x,hx⟩,⟨y,hy⟩⟩; rfl

def pointFiberEquiv (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0) :
    ThreeTorsion W ≃ Option (Fibers W) := by
  have hz : (3 : ℕ) • (Point.zero : W.Point)=0 := by
    change 3 • (0 : W.Point)=0
    exact smul_zero _
  let e : ThreeTorsion W ≃ Option (AffineThree W) :=
    W.nonsingularPointEquivSubtype (p := fun P => 3 • P=0) hz
  exact e.trans (affineFiberEquiv W hΔ).optionCongr

theorem torsion_finite [NeZero (3 : K)] (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0) :
    Finite (ThreeTorsion W) :=
  Finite.of_equiv (Option (Fibers W)) (pointFiberEquiv W hΔ).symm

theorem geometric_three_torsion_card [IsAlgClosed K] [NeZero (3 : K)]
    (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0) : Nat.card (ThreeTorsion W)=9 := by
  rw [Nat.card_congr (pointFiberEquiv W hΔ),Finite.card_option,fibers_card_eight W hΔ]

def shortTorsionEquiv (a b : K) : ThreeTorsion (short a b) ≃ ThreeTorsionRankAudit.ThreeTorsion a b where
  toFun P := ⟨P.val,P.property⟩
  invFun P := ⟨P.val,P.property⟩
  left_inv _ := rfl
  right_inv _ := rfl

end
end ThreeTorsionRankAudit.General
