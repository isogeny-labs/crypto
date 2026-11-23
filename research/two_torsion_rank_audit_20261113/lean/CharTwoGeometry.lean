import OddCardinality

namespace TwoTorsionRankAudit
open Polynomial WeierstrassCurve.Affine
noncomputable section
variable {K : Type*} [Field K] [DecidableEq K] [CharP K 2]

theorem some_two_killed_iff (W : WeierstrassCurve.Affine K) (x y : K)
    (h : W.Nonsingular x y) :
    2 • (Point.some x y h)=0 ↔ y=W.negY x y := by
  rw [two_nsmul,add_eq_zero_iff_eq_neg,Point.neg_some]
  simp only [Point.some.injEq,true_and]

theorem char_two_killed_iff (W : WeierstrassCurve.Affine K) (x y : K)
    (h : W.Nonsingular x y) :
    2 • (Point.some x y h)=0 ↔ W.a₁*x+W.a₃=0 := by
  rw [some_two_killed_iff]
  unfold WeierstrassCurve.Affine.negY
  have h2 : (2 : K)=0 := CharP.cast_eq_zero K 2
  constructor
  · intro hy
    linear_combination hy-y*h2
  · intro hx
    linear_combination hx+y*h2

theorem a3_nonzero_of_a1_zero (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (ha : W.a₁=0) : W.a₃≠0 := by
  have hd : W.Δ=W.a₃^4 := by simpa [ha] using W.Δ_of_char_two
  intro hz
  apply hΔ
  rw [hd,hz]
  simp

theorem only_zero_when_a1_zero (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (ha : W.a₁=0) (P : W.Point) (hP : 2 • P=0) : P=0 := by
  cases P with
  | zero => rfl
  | some x y h =>
    have hx := (char_two_killed_iff W x y h).mp hP
    rw [ha,zero_mul,zero_add] at hx
    exact False.elim (a3_nonzero_of_a1_zero W hΔ ha hx)

theorem char_two_torsion_card_one (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (ha : W.a₁=0) : Nat.card (TwoTorsion W)=1 := by
  let e : TwoTorsion W ≃ PUnit.{1} := {
    toFun := fun _ => PUnit.unit
    invFun := fun _ => 0
    left_inv := by
      intro P
      apply Subtype.ext
      exact (only_zero_when_a1_zero W hΔ ha P.val P.property).symm
    right_inv := by intro x; cases x; rfl }
  rw [Nat.card_congr e,Nat.card_eq_fintype_card,Fintype.card_punit]

theorem killed_x_coordinate (W : WeierstrassCurve.Affine K) (ha : W.a₁≠0)
    (x y : K) (h : W.Nonsingular x y) (hP : 2 • (Point.some x y h)=0) :
    x= -W.a₃/W.a₁ := by
  apply (eq_div_iff ha).mpr
  have hx := (char_two_killed_iff W x y h).mp hP
  linear_combination hx

theorem nonzero_killed_unique (W : WeierstrassCurve.Affine K) (ha : W.a₁≠0)
    (P Q : W.Point) (hP : P≠0) (hQ : Q≠0) (hkP : 2 • P=0) (hkQ : 2 • Q=0) : P=Q := by
  cases P with
  | zero => exact False.elim (hP rfl)
  | some x y h =>
    cases Q with
    | zero => exact False.elim (hQ rfl)
    | some u v j =>
      have hx : x=u := (killed_x_coordinate W ha x y h hkP).trans
        (killed_x_coordinate W ha u v j hkQ).symm
      rcases (Point.X_eq_iff (h₁:=h) (h₂:=j)).mp hx with hh | hh
      · exact hh
      · have hneg : Point.some u v j= -(Point.some u v j) :=
          add_eq_zero_iff_eq_neg.mp (by simpa only [two_nsmul] using hkQ)
        exact hh.trans hneg.symm

theorem ordinary_nonzero_killed [IsAlgClosed K] (W : WeierstrassCurve.Affine K)
    (hΔ : W.Δ≠0) (ha : W.a₁≠0) : ∃ P : W.Point,P≠0 ∧ 2 • P=0 := by
  let x : K := -W.a₃/W.a₁
  have hx : W.a₁*x+W.a₃=0 := by dsimp [x]; field_simp; ring
  obtain ⟨y,hy⟩ := IsAlgClosed.exists_pow_nat_eq
    (x^3+W.a₂*x^2+W.a₄*x+W.a₆) (by decide : 0<(2:ℕ))
  have he : W.Equation x y := by
    rw [WeierstrassCurve.Affine.equation_iff]
    linear_combination hy+y*hx
  have hn := (W.equation_iff_nonsingular_of_Δ_ne_zero hΔ).mp he
  exact ⟨Point.some x y hn,Point.some_ne_zero hn,(char_two_killed_iff W x y hn).mpr hx⟩

#print axioms char_two_torsion_card_one
#print axioms ordinary_nonzero_killed
end
end TwoTorsionRankAudit
