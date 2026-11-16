import SevenPoint
import SevenSeparable

namespace SevenTorsionRankAudit
noncomputable section
open WeierstrassCurve.Affine ThreeTorsionRankAudit ThreeTorsionRankAudit.General
variable {K : Type*} [Field K] [DecidableEq K]

def sevenSubgroup (W : WeierstrassCurve.Affine K) : AddSubgroup W.Point where
  carrier := {P | (7:ℕ) • P=0}
  zero_mem' := by simp
  add_mem' := by
    intro P Q hP hQ
    simp only [Set.mem_ofPred_eq] at *
    rw [nsmul_add,hP,hQ,zero_add]
  neg_mem' := by
    intro P hP
    simp only [Set.mem_ofPred_eq] at *
    rw [smul_neg,hP,neg_zero]

abbrev SevenTorsion (W : WeierstrassCurve.Affine K) := ↥(sevenSubgroup W)
abbrev SevenFibers (W : WeierstrassCurve.Affine K) := Σ x : SevenRoot W, Ordinate W x.val
def AffineSeven (W : WeierstrassCurve.Affine K) := {xy : K × K //
  ∃ h : W.Nonsingular xy.1 xy.2, (7:ℕ) • Point.some _ _ h=0}

def affineSevenFiberEquiv (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) :
    AffineSeven W ≃ SevenFibers W where
  toFun A := ⟨⟨A.val.1,(seven_point_iff W A.property.choose).mp A.property.choose_spec⟩,
    ⟨A.val.2,A.property.choose.1⟩⟩
  invFun A := by
    have hn : W.Nonsingular A.1.val A.2.val :=
      (W.equation_iff_nonsingular_of_Δ_ne_zero hΔ).mp A.2.property
    exact ⟨(A.1.val,A.2.val),hn,(seven_point_iff W hn).mpr A.1.property⟩
  left_inv A := by apply Subtype.ext; rfl
  right_inv := by rintro ⟨⟨x,hx⟩,⟨y,hy⟩⟩; rfl

def pointSevenFiberEquiv (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) :
    SevenTorsion W ≃ Option (SevenFibers W) := by
  have hz : (7:ℕ) • (Point.zero : W.Point)=0 := by
    change (7:ℕ) • (0:W.Point)=0
    exact smul_zero _
  let e : SevenTorsion W ≃ Option (AffineSeven W) :=
    W.nonsingularPointEquivSubtype (p:=fun P => (7:ℕ) • P=0) hz
  exact e.trans (affineSevenFiberEquiv W hΔ).optionCongr

theorem seven_torsion_finite_of_nonzero (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (hp : psiSeven W≠0) : Finite (SevenTorsion W) := by
  let : Fintype (SevenRoot W) := polynomialRootFintype (psiSeven W) hp
  exact Finite.of_equiv (Option (SevenFibers W)) (pointSevenFiberEquiv W hΔ).symm

theorem seven_torsion_finite [NeZero (7:K)] (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) :
    Finite (SevenTorsion W) := seven_torsion_finite_of_nonzero W hΔ (seven_nonzero W)

theorem seven_root_count_le (W : WeierstrassCurve K) (hp : psiSeven W≠0) :
    Nat.card (SevenRoot W)≤(psiSeven W).natDegree := by
  rw [polynomialRoot_card (psiSeven W) hp]
  exact (Multiset.toFinset_card_le _).trans (psiSeven W).card_roots'

theorem seven_fibers_card_twice [IsAlgClosed K] (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (hp : psiSeven W≠0) : Nat.card (SevenFibers W)=Nat.card (SevenRoot W)*2 := by
  let : Fintype (SevenRoot W) := polynomialRootFintype (psiSeven W) hp
  rw [Nat.card_eq_fintype_card,Fintype.card_sigma]
  have hc (x : SevenRoot W) : Fintype.card (Ordinate W x.val)=2 := by
    rw [←Nat.card_eq_fintype_card]
    exact seven_ordinate_card W hΔ x.property
  simp_rw [hc]
  simp only [Finset.sum_const,Finset.card_univ,smul_eq_mul]
  rw [←Nat.card_eq_fintype_card]

theorem seven_card_fiber_formula [IsAlgClosed K] (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (hp : psiSeven W≠0) : Nat.card (SevenTorsion W)=Nat.card (SevenRoot W)*2+1 := by
  let : Fintype (SevenRoot W) := polynomialRootFintype (psiSeven W) hp
  rw [Nat.card_congr (pointSevenFiberEquiv W hΔ),Finite.card_option,seven_fibers_card_twice W hΔ hp]

theorem geometric_seven_card [IsAlgClosed K] [NeZero (7:K)]
    (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) : Nat.card (SevenTorsion W)=49 := by
  rw [seven_card_fiber_formula W hΔ (seven_nonzero W),seven_root_card W hΔ]

#print axioms pointSevenFiberEquiv
#print axioms geometric_seven_card
end
end SevenTorsionRankAudit
