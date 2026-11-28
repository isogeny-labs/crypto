import TriplePoint
import QuarticRoots

namespace ThreeTorsionRankAudit
open WeierstrassCurve.Affine
noncomputable section
variable {K : Type*} [Field K] [DecidableEq K] [NeZero (2 : K)]

def torsion (a b : K) : AddSubgroup (short a b).Point where
  carrier := {P | 3 • P=0}
  zero_mem' := by simp
  add_mem' := by intro P Q hP hQ; simp only [Set.mem_ofPred_eq] at *; rw [nsmul_add,hP,hQ,zero_add]
  neg_mem' := by intro P hP; simp only [Set.mem_ofPred_eq] at *; rw [smul_neg,hP,neg_zero]

abbrev ThreeTorsion (a b : K) := ↥(torsion a b)

def AffineThree (a b : K) := {xy : K × K //
  ∃ h : (short a b).Nonsingular xy.1 xy.2, (3 : ℕ) • Point.some _ _ h = 0}

def affineFiberEquiv (a b : K) (hΔ : (short a b).Δ ≠ 0) :
    AffineThree a b ≃ Fibers a b where
  toFun A :=
    ⟨⟨A.val.1,(three_point_iff a b A.val.1 A.val.2 hΔ A.property.choose).mp A.property.choose_spec⟩,
      ⟨A.val.2,(curve_equation a b A.val.1 A.val.2).mp A.property.choose.1⟩⟩
  invFun A := by
    have hn : (short a b).Nonsingular A.1.val A.2.val :=
      (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero hΔ).mp
        ((curve_equation a b A.1.val A.2.val).mpr A.2.property)
    exact ⟨(A.1.val,A.2.val),hn,(three_point_iff a b A.1.val A.2.val hΔ hn).mpr A.1.property⟩
  left_inv := by intro A; apply Subtype.ext; rfl
  right_inv := by rintro ⟨⟨x,hx⟩,⟨y,hy⟩⟩; rfl

def pointFiberEquiv (a b : K) (hΔ : (short a b).Δ ≠ 0) :
    ThreeTorsion a b ≃ Option (Fibers a b) := by
  have hz : (3 : ℕ) • (Point.zero : (short a b).Point)=0 := by
    change 3 • (0 : (short a b).Point)=0
    exact smul_zero _
  let e : ThreeTorsion a b ≃ Option (AffineThree a b) :=
    (short a b).nonsingularPointEquivSubtype (p := fun P => 3 • P=0) hz
  exact e.trans (affineFiberEquiv a b hΔ).optionCongr

theorem torsion_finite [NeZero (3 : K)] (a b : K) (hΔ : (short a b).Δ ≠ 0) :
    Finite (ThreeTorsion a b) :=
  Finite.of_equiv (Option (Fibers a b)) (pointFiberEquiv a b hΔ).symm

theorem geometric_three_torsion_card [IsAlgClosed K] [NeZero (3 : K)]
    (a b : K) (hΔ : (short a b).Δ ≠ 0) : Nat.card (ThreeTorsion a b)=9 := by
  rw [Nat.card_congr (pointFiberEquiv a b hΔ),Finite.card_option,fibers_card_eight a b hΔ]

end
end ThreeTorsionRankAudit
