import GeometricCardinality
import Mathlib.FieldTheory.IsAlgClosed.AlgebraicClosure

namespace ThreeTorsionRankAudit
noncomputable section
variable {K : Type*} [Field K] [DecidableEq K] [NeZero (2 : K)] [CharP K 3]

omit [DecidableEq K] [NeZero (2 : K)] in
theorem char_three_psi_value (x : K) : (psi (-1 : K) 0).eval x = -1 := by
  rw [psi_eval]
  have h3 : (3 : K)=0 := CharP.cast_eq_zero K 3
  have h6 : (6 : K)=2*3 := by norm_num
  have h12 : (12 : K)=4*3 := by norm_num
  rw [h6,h12,h3]
  ring

omit [DecidableEq K] [NeZero (2 : K)] in
theorem char_three_discriminant : (short (-1 : K) 0).Δ ≠ 0 := by
  have hd : (short (-1 : K) 0).Δ = (64 : K) := by
    norm_num [short,WeierstrassCurve.Δ,WeierstrassCurve.b₂,WeierstrassCurve.b₄,
      WeierstrassCurve.b₆,WeierstrassCurve.b₈]
  have h3 : (3 : K)=0 := CharP.cast_eq_zero K 3
  have h63 : (63 : K)=0 := by
    calc (63 : K)=21*3 := by norm_num
         _ = 0 := by rw [h3]; ring
  have h64 : (64 : K)=1 := by
    calc (64 : K)=63+1 := by norm_num
         _ = 1 := by rw [h63]; ring
  rw [hd,h64]
  exact one_ne_zero

theorem char_three_all_zero (P : (short (-1 : K) 0).Point) (hP : (3 : ℕ) • P=0) : P=0 := by
  cases P with
  | zero => rfl
  | some x y hN =>
    have hx := (three_point_iff (-1) 0 x y char_three_discriminant hN).mp hP
    have he : (psi (-1 : K) 0).eval x=0 := hx
    rw [char_three_psi_value] at he
    exact False.elim ((neg_ne_zero.mpr one_ne_zero) he)

def charThreeEquivUnit : ThreeTorsion (-1 : K) 0 ≃ PUnit.{1} where
  toFun _ := PUnit.unit
  invFun _ := ⟨0,by simp [torsion]⟩
  left_inv P := by apply Subtype.ext; exact (char_three_all_zero P.val P.property).symm
  right_inv P := by cases P; rfl

theorem char_three_torsion_card : Nat.card (ThreeTorsion (-1 : K) 0)=1 := by
  rw [Nat.card_congr charThreeEquivUnit]
  simp

end

noncomputable section
private instance : Fact (Nat.Prime 3) := ⟨by decide⟩
abbrev CharThreeClosure := AlgebraicClosure (ZMod 3)
private instance : DecidableEq CharThreeClosure := Classical.decEq _
private instance : CharP CharThreeClosure 3 := charP_of_injective_algebraMap' (ZMod 3) 3
private instance : NeZero (2 : CharThreeClosure) := by
  constructor
  intro h
  have hd : 3 ∣ 2 := (CharP.cast_eq_zero_iff CharThreeClosure 3 2).mp h
  norm_num at hd

theorem actual_char_three_geometric_control :
    (short (-1 : CharThreeClosure) 0).Δ ≠ 0 ∧
    Nat.card (ThreeTorsion (-1 : CharThreeClosure) 0)=1 ∧
    Nat.card (ThreeTorsion (-1 : CharThreeClosure) 0)≠9 := by
  refine ⟨char_three_discriminant,char_three_torsion_card,?_⟩
  rw [char_three_torsion_card]
  decide

end
end ThreeTorsionRankAudit
