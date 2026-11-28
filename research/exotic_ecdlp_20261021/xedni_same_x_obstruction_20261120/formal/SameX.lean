import Mathlib.AlgebraicGeometry.EllipticCurve.Affine.Point

/-! An exact obstruction to a three-term relation for distinct affine points with one x-coordinate. -/

namespace XedniSameX

open WeierstrassCurve

theorem no_relation_of_equal_xRep {F : Type*} [Field F] [DecidableEq F]
    {W : WeierstrassCurve.Affine F} [W.IsElliptic] (P Q : W.Point)
    (hx : P.xRep = Q.xRep) (hne : P ≠ Q) (hq : Q ≠ 0) :
    P + Q + Q ≠ 0 := by
  intro h
  rcases WeierstrassCurve.Affine.Point.eq_or_eq_neg_of_xRep_eq_xRep hx with heq | heq
  · exact hne heq
  · rw [heq] at h
    exact hq (by simpa using h)

theorem no_relation_of_same_x_distinct_y {F : Type*} [Field F] [DecidableEq F]
    {W : WeierstrassCurve.Affine F} [W.IsElliptic] {x y₁ y₂ : F}
    (h₁ : W.Nonsingular x y₁) (h₂ : W.Nonsingular x y₂)
    (hy : y₁ ≠ y₂) :
    (WeierstrassCurve.Affine.Point.some x y₁ h₁) +
      (WeierstrassCurve.Affine.Point.some x y₂ h₂) +
      (WeierstrassCurve.Affine.Point.some x y₂ h₂) ≠ 0 := by
  let P : W.Point := .some x y₁ h₁
  let Q : W.Point := .some x y₂ h₂
  change P + Q + Q ≠ 0
  have hx : P.xRep = Q.xRep := by simp [P, Q]
  have hne : P ≠ Q := by
    intro h
    exact hy (WeierstrassCurve.Affine.Point.some.inj h).right
  have hq : Q ≠ 0 := WeierstrassCurve.Affine.Point.some_ne_zero h₂
  exact no_relation_of_equal_xRep P Q hx hne hq

#print axioms no_relation_of_equal_xRep
#print axioms no_relation_of_same_x_distinct_y

end XedniSameX
