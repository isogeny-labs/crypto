import Mathlib.AlgebraicGeometry.EllipticCurve.Affine.Point
import Mathlib.Tactic

namespace XedniRankOneAnchor

open WeierstrassCurve

def E : WeierstrassCurve ℚ := ⟨0, -47, 0, 751, -3997⟩

instance : E.IsElliptic := ⟨by
  apply isUnit_iff_ne_zero.mpr
  norm_num [E, WeierstrassCurve.Δ, WeierstrassCurve.b₂,
    WeierstrassCurve.b₄, WeierstrassCurve.b₆, WeierstrassCurve.b₈]⟩

private theorem p_on : Affine.Equation E 14 7 := by
  rw [Affine.equation_iff]
  norm_num [E]

private theorem q_on : Affine.Equation E 17 10 := by
  rw [Affine.equation_iff]
  norm_num [E]

def P : Affine.Point E := Affine.Point.mk p_on
def Q : Affine.Point E := Affine.Point.mk q_on

theorem rational_relation : P + 2 • Q = 0 := by
  have hdouble : Q + Q = -P := by
    have hy : (10 : ℚ) ≠ Affine.negY E 17 10 := by
      norm_num [Affine.negY, E]
    simp only [Q, P, Affine.Point.mk]
    rw [Affine.Point.add_self_of_Y_ne hy]
    change _ = Affine.Point.some 14 (Affine.negY E 14 7) _
    norm_num [Affine.addX, Affine.addY, Affine.slope, Affine.negY,
      Affine.negAddY, E]
  simpa [two_nsmul] using congrArg (fun T : Affine.Point E => P + T) hdouble

#print axioms rational_relation

end XedniRankOneAnchor
