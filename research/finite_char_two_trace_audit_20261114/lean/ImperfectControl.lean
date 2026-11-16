import TraceParity
import Mathlib.FieldTheory.RatFunc.Degree

namespace FiniteCharTwoTraceAudit
noncomputable section
open WeierstrassCurve.Affine TwoTorsionRankAudit
private instance : Fact (Nat.Prime 2) := ⟨by decide⟩
abbrev ImperfectField := RatFunc (ZMod 2)
private instance : DecidableEq ImperfectField := Classical.decEq _

theorem variable_not_square : ¬∃ y : ImperfectField,y^2=RatFunc.X := by
  rintro ⟨y,hy⟩
  have hy0 : y≠0 := by
    intro hz
    rw [hz,zero_pow (by decide : (2:ℕ)≠0)] at hy
    exact RatFunc.X_ne_zero hy.symm
  have hd := congrArg RatFunc.intDegree hy
  rw [pow_two,RatFunc.intDegree_mul hy0 hy0,RatFunc.intDegree_X] at hd
  omega

def imperfectControl : WeierstrassCurve.Affine ImperfectField := ⟨1,0,0,0,RatFunc.X⟩

theorem imperfect_delta : imperfectControl.Δ=RatFunc.X := by
  rw [WeierstrassCurve.Δ_of_char_two]
  simp [imperfectControl,WeierstrassCurve.b₈]

theorem imperfect_nonsingular : imperfectControl.Δ≠0 := by
  rw [imperfect_delta]
  exact RatFunc.X_ne_zero

instance imperfectElliptic : imperfectControl.IsElliptic :=
  SmallCharacteristicJAudit.ellipticOfNonzero imperfectControl imperfect_nonsingular

theorem imperfect_j_nonzero : imperfectControl.j≠0 := by
  intro hj
  exact one_ne_zero (imperfectControl.j_eq_zero_iff_of_char_two.mp hj)

theorem imperfect_only_zero (P : imperfectControl.Point) (hk : 2 • P=0) : P=0 := by
  cases P with
  | zero => rfl
  | some x y h =>
    have hx := (char_two_killed_iff imperfectControl x y h).mp hk
    have hx0 : x=0 := by simpa [imperfectControl] using hx
    have he := h.1
    rw [WeierstrassCurve.Affine.equation_iff,hx0] at he
    have hy : y^2=RatFunc.X := by simpa [imperfectControl] using he
    exact False.elim (variable_not_square ⟨y,hy⟩)

theorem imperfect_rational_two_card : Nat.card (TwoTorsion imperfectControl)=1 :=
  (SmallCharacteristicJAudit.killed_card_one_iff 2).mpr imperfect_only_zero

abbrev ImperfectClosure := AlgebraicClosure ImperfectField
private instance : DecidableEq ImperfectClosure := Classical.decEq _

theorem imperfect_geometric_two_card :
    Nat.card (TwoTorsion (imperfectControl.baseChange ImperfectClosure))=2 := by
  have h := SmallCharacteristicJAudit.two_geometric_j (L:=ImperfectClosure) imperfectControl
  simpa only [imperfect_j_nonzero,ite_false] using h

/-- Ordinary geometric two-torsion need not be rational over an imperfect field. -/
theorem deleting_finiteness_false :
    ¬∀ (K : Type) [Field K] [DecidableEq K] [CharP K 2]
      (W : WeierstrassCurve.Affine K),W.Δ≠0 → W.a₁≠0 →
      ∃ P : W.Point,P≠0 ∧ 2 • P=0 := by
  intro h
  obtain ⟨P,hP,hk⟩ := h ImperfectField imperfectControl imperfect_nonsingular one_ne_zero
  exact hP (imperfect_only_zero P hk)

#print axioms variable_not_square
#print axioms imperfect_rational_two_card
#print axioms imperfect_geometric_two_card
#print axioms deleting_finiteness_false
end
end FiniteCharTwoTraceAudit
