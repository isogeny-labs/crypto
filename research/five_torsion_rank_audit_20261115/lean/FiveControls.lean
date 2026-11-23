import FiveBasis

namespace FiveTorsionRankAudit
noncomputable section
open WeierstrassCurve.Affine ThreeTorsionRankAudit ThreeTorsionRankAudit.General Module
private instance : Fact (Nat.Prime 5) := ⟨by decide⟩
variable {K : Type*} [Field K]

def charFiveCurve (K : Type*) [Field K] : WeierstrassCurve.Affine K := ⟨0,0,0,0,1⟩

theorem charFive_eval [CharP K 5] (x : K) : (psiFive (charFiveCurve K)).eval x= -1 := by
  have h5 : (5:K)=0 := CharP.cast_eq_zero K 5
  calc (psiFive (charFiveCurve K)).eval x=
      5*(x^12+76*x^9-48*x^6-320*x^3-51)-1 := by
        rw [five_eval]
        simp [charFiveCurve,WeierstrassCurve.Ψ₂Sq,WeierstrassCurve.preΨ₄,WeierstrassCurve.Ψ₃,
          WeierstrassCurve.b₂,WeierstrassCurve.b₄,WeierstrassCurve.b₆,WeierstrassCurve.b₈]
        ring
    _= -1 := by rw [h5,zero_mul,zero_sub]

theorem charFive_discriminant [CharP K 5] : (charFiveCurve K).Δ≠0 := by
  have h5 : (5:K)=0 := CharP.cast_eq_zero K 5
  have hd : (charFiveCurve K).Δ=3 := by
    have he : (charFiveCurve K).Δ= -432 := by
      norm_num [charFiveCurve,WeierstrassCurve.Δ,WeierstrassCurve.b₂,WeierstrassCurve.b₄,
        WeierstrassCurve.b₆,WeierstrassCurve.b₈]
    rw [he]
    linear_combination -87*h5
  rw [hd]
  intro hz
  have hb : 5 ∣ 3 := (CharP.cast_eq_zero_iff K 5 3).mp hz
  norm_num at hb

variable [DecidableEq K]

theorem charFive_only_zero [CharP K 5] (P : FiveTorsion (charFiveCurve K)) : P=0 := by
  rcases P with ⟨P,hP⟩
  cases P with
  | zero => rfl
  | some x y hN =>
    have hx : (psiFive (charFiveCurve K)).eval x=0 := (five_point_iff (charFiveCurve K) hN).mp hP
    rw [charFive_eval] at hx
    exact (neg_ne_zero.mpr (one_ne_zero : (1:K)≠0) hx).elim

theorem charFive_card [CharP K 5] : Nat.card (FiveTorsion (charFiveCurve K))=1 := by
  let e : FiveTorsion (charFiveCurve K) ≃ Unit := {
    toFun := fun _ => ()
    invFun := fun _ => 0
    left_inv := fun P => (charFive_only_zero P).symm
    right_inv := by intro P; cases P; rfl }
  rw [Nat.card_congr e]
  simp

private instance : Fact (Nat.Prime 2) := ⟨by decide⟩
private instance : Fact (Nat.Prime 3) := ⟨by decide⟩
private instance : DecidableEq BinaryClosure := Classical.decEq _
private instance : CharP BinaryClosure 2 := charP_of_injective_algebraMap' (ZMod 2) 2
private instance : NeZero (5:BinaryClosure) := by
  constructor
  intro h
  have hb : 2 ∣ 5 := (CharP.cast_eq_zero_iff BinaryClosure 2 5).mp h
  norm_num at hb
private instance : DecidableEq CharThreeClosure := Classical.decEq _
private instance : CharP CharThreeClosure 3 := charP_of_injective_algebraMap' (ZMod 3) 3
private instance : NeZero (5:CharThreeClosure) := by
  constructor
  intro h
  have hb : 3 ∣ 5 := (CharP.cast_eq_zero_iff CharThreeClosure 3 5).mp h
  norm_num at hb

theorem binary_five_positive :
    CharP BinaryClosure 2 ∧ binaryCurve.Δ≠0 ∧ Nat.card (FiveTorsion binaryCurve)=25 ∧
      finrank (ZMod 5) (FiveTorsion binaryCurve)=2 :=
  ⟨inferInstance,binary_discriminant,geometric_five_card binaryCurve binary_discriminant,
    geometric_five_finrank binaryCurve binary_discriminant⟩

theorem ternary_five_positive :
    CharP CharThreeClosure 3 ∧ (short (-1:CharThreeClosure) 0).Δ≠0 ∧
      Nat.card (FiveTorsion (short (-1:CharThreeClosure) 0))=25 ∧
      finrank (ZMod 5) (FiveTorsion (short (-1:CharThreeClosure) 0))=2 :=
  ⟨inferInstance,char_three_discriminant,geometric_five_card _ char_three_discriminant,
    geometric_five_finrank _ char_three_discriminant⟩

#print axioms charFive_card
#print axioms binary_five_positive
#print axioms ternary_five_positive
end
end FiveTorsionRankAudit
