import Mathlib.AlgebraicGeometry.EllipticCurve.Affine.Point
import Mathlib.Tactic

/-! A tangent-and-divisibility obstruction for the P + 2Q relation. -/

namespace XedniCollisionTangent

private def equation (a1 a2 a3 a4 a6 x y : ℚ) : Prop :=
  y*y+a1*x*y+a3*y = x*x*x+a2*x*x+a4*x+a6

private def obstruction (a1 a2 x1 x2 y1 y2 : ℚ) : ℚ :=
  (x1-x2)^2*(a2+x1+2*x2)-a1*(x1-x2)*(y1-y2)-(y1-y2)^2

theorem obstruction_of_tangent (a1 a2 a3 a4 a6 x1 x2 y1 y2 slope : ℚ)
    (hP : equation a1 a2 a3 a4 a6 x1 y1)
    (hQ : equation a1 a2 a3 a4 a6 x2 y2)
    (hline : slope*(x1-x2) = y1-y2)
    (htangent : slope*(2*y2+a1*x2+a3) = 3*x2*x2+2*a2*x2+a4-a1*y2) :
    obstruction a1 a2 x1 x2 y1 y2 = 0 := by
  dsimp [equation] at hP hQ
  dsimp [obstruction]
  linear_combination
    (x1-x2)*htangent - (hP-hQ) - (2*y2+a1*x2+a3)*hline

theorem integer_divisibility (a1 a2 x1 x2 y1 y2 : ℤ)
    (h : obstruction (a1 : ℚ) (a2 : ℚ) (x1 : ℚ) (x2 : ℚ) (y1 : ℚ) (y2 : ℚ) = 0) :
    (x1-x2) ∣ (y1-y2)^2 := by
  have hz : (x1-x2)^2*(a2+x1+2*x2)-a1*(x1-x2)*(y1-y2)-(y1-y2)^2 = 0 := by
    have hq : (((x1-x2)^2*(a2+x1+2*x2)-a1*(x1-x2)*(y1-y2)-(y1-y2)^2 : ℤ) : ℚ) = 0 := by
      simpa [obstruction] using h
    exact_mod_cast hq
  refine ⟨(x1-x2)*(a2+x1+2*x2)-a1*(y1-y2), ?_⟩
  nlinarith [hz]

theorem obstruction_of_point_relation (W : WeierstrassCurve.Affine ℚ) [W.IsElliptic]
    (x1 x2 y1 y2 : ℚ) (hP : W.Nonsingular x1 y1) (hQ : W.Nonsingular x2 y2)
    (hrel : (WeierstrassCurve.Affine.Point.some x1 y1 hP) +
      (WeierstrassCurve.Affine.Point.some x2 y2 hQ) +
      (WeierstrassCurve.Affine.Point.some x2 y2 hQ) = 0) :
    obstruction W.a₁ W.a₂ x1 x2 y1 y2 = 0 := by
  classical
  let P : W.Point := .some x1 y1 hP
  let Q : W.Point := .some x2 y2 hQ
  have hrel' : P + Q + Q = 0 := hrel
  have hsum : Q + Q = -P := by
    have h := congrArg (fun Z : W.Point => -P + Z) hrel'
    simpa [add_assoc, add_comm, add_left_comm] using h
  have hpnz : P ≠ 0 := WeierstrassCurve.Affine.Point.some_ne_zero hP
  have hqnz : Q + Q ≠ 0 := by
    rw [hsum]
    exact neg_ne_zero.mpr hpnz
  have hy : y2 ≠ W.negY x2 y2 := by
    intro heq
    apply hqnz
    simpa [Q] using (WeierstrassCurve.Affine.Point.add_self_of_Y_eq (h₁ := hQ) heq)
  change (WeierstrassCurve.Affine.Point.some x2 y2 hQ) +
    (WeierstrassCurve.Affine.Point.some x2 y2 hQ) =
    -(WeierstrassCurve.Affine.Point.some x1 y1 hP) at hsum
  rw [WeierstrassCurve.Affine.Point.add_self_of_Y_ne (h₁ := hQ) hy,
      WeierstrassCurve.Affine.Point.neg_some] at hsum
  rcases WeierstrassCurve.Affine.Point.some.inj hsum with ⟨hxdouble, hydouble⟩
  have hline : (W.slope x2 x2 y2 y2)*(x1-x2) = y1-y2 := by
    change W.negY (W.addX x2 x2 (W.slope x2 x2 y2 y2))
      ((W.slope x2 x2 y2 y2) *
        (W.addX x2 x2 (W.slope x2 x2 y2 y2) - x2) + y2) =
        W.negY x1 y1 at hydouble
    rw [hxdouble] at hydouble
    simp only [WeierstrassCurve.Affine.negY] at hydouble
    linear_combination -hydouble
  have hden : y2 - W.negY x2 y2 ≠ 0 := sub_ne_zero.mpr hy
  have hd : y2 - W.negY x2 y2 = 2*y2+W.a₁*x2+W.a₃ := by
    simp only [WeierstrassCurve.Affine.negY]
    ring
  have hslope : (W.slope x2 x2 y2 y2)*(2*y2+W.a₁*x2+W.a₃) =
      3*x2*x2+2*W.a₂*x2+W.a₄-W.a₁*y2 := by
    have hd' : (2*y2+W.a₁*x2+W.a₃) ≠ 0 := by rw [← hd]; exact hden
    calc
      _ = ((3*x2^2+2*W.a₂*x2+W.a₄-W.a₁*y2) /
          (2*y2+W.a₁*x2+W.a₃))*(2*y2+W.a₁*x2+W.a₃) := by
            rw [W.slope_of_Y_ne rfl hy, hd]
      _ = 3*x2^2+2*W.a₂*x2+W.a₄-W.a₁*y2 := div_mul_cancel₀ _ hd'
      _ = _ := by ring
  have hPE : equation W.a₁ W.a₂ W.a₃ W.a₄ W.a₆ x1 y1 := by
    have heq := (W.equation_iff x1 y1).mp hP.1
    dsimp [equation]
    nlinarith [heq]
  have hQE : equation W.a₁ W.a₂ W.a₃ W.a₄ W.a₆ x2 y2 := by
    have heq := (W.equation_iff x2 y2).mp hQ.1
    dsimp [equation]
    nlinarith [heq]
  exact obstruction_of_tangent W.a₁ W.a₂ W.a₃ W.a₄ W.a₆ x1 x2 y1 y2
    (W.slope x2 x2 y2 y2) hPE hQE hline hslope

theorem no_relation_of_nondvd (W : WeierstrassCurve.Affine ℚ) [W.IsElliptic]
    (a1 a2 x1 x2 y1 y2 : ℤ)
    (ha1 : W.a₁ = (a1 : ℚ)) (ha2 : W.a₂ = (a2 : ℚ))
    (hP : W.Nonsingular (x1 : ℚ) (y1 : ℚ))
    (hQ : W.Nonsingular (x2 : ℚ) (y2 : ℚ))
    (hndiv : ¬ (x1-x2) ∣ (y1-y2)^2) :
    (WeierstrassCurve.Affine.Point.some (x1 : ℚ) (y1 : ℚ) hP) +
      (WeierstrassCurve.Affine.Point.some (x2 : ℚ) (y2 : ℚ) hQ) +
      (WeierstrassCurve.Affine.Point.some (x2 : ℚ) (y2 : ℚ) hQ) ≠ 0 := by
  intro hrel
  have hob := obstruction_of_point_relation W _ _ _ _ hP hQ hrel
  rw [ha1, ha2] at hob
  exact hndiv (integer_divisibility a1 a2 x1 x2 y1 y2 hob)

structure Coords where
  x1 : ℤ
  x2 : ℤ
  y1 : ℤ
  y2 : ℤ

def sourcePair : Fin 8 → Coords
  | 0 => ⟨10, -18, 1, 23⟩
  | 1 => ⟨10, -18, -1, -23⟩
  | 2 => ⟨17, 10, 10, -1⟩
  | 3 => ⟨17, 10, -10, 1⟩
  | 4 => ⟨-18, 15, 23, 16⟩
  | 5 => ⟨-18, 15, -23, -16⟩
  | 6 => ⟨-10, -21, 18, 8⟩
  | 7 => ⟨-10, -21, -18, -8⟩

theorem sourcePair_nondvd (i : Fin 8) :
    ¬ ((sourcePair i).x1-(sourcePair i).x2) ∣
      ((sourcePair i).y1-(sourcePair i).y2)^2 := by
  fin_cases i <;> norm_num [sourcePair]

theorem no_relation_for_eight_source_pairs (i : Fin 8)
    (W : WeierstrassCurve.Affine ℚ) [W.IsElliptic] (a1 a2 : ℤ)
    (ha1 : W.a₁ = (a1 : ℚ)) (ha2 : W.a₂ = (a2 : ℚ))
    (hP : W.Nonsingular ((sourcePair i).x1 : ℚ) ((sourcePair i).y1 : ℚ))
    (hQ : W.Nonsingular ((sourcePair i).x2 : ℚ) ((sourcePair i).y2 : ℚ)) :
    (WeierstrassCurve.Affine.Point.some ((sourcePair i).x1 : ℚ)
      ((sourcePair i).y1 : ℚ) hP) +
    (WeierstrassCurve.Affine.Point.some ((sourcePair i).x2 : ℚ)
      ((sourcePair i).y2 : ℚ) hQ) +
    (WeierstrassCurve.Affine.Point.some ((sourcePair i).x2 : ℚ)
      ((sourcePair i).y2 : ℚ) hQ) ≠ 0 := by
  exact no_relation_of_nondvd W a1 a2 (sourcePair i).x1 (sourcePair i).x2
    (sourcePair i).y1 (sourcePair i).y2 ha1 ha2 hP hQ (sourcePair_nondvd i)

#print axioms obstruction_of_tangent
#print axioms integer_divisibility
#print axioms obstruction_of_point_relation
#print axioms no_relation_of_nondvd
#print axioms sourcePair_nondvd
#print axioms no_relation_for_eight_source_pairs

end XedniCollisionTangent
