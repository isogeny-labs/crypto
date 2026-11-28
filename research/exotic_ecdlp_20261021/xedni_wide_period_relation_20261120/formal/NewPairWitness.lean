import Mathlib.AlgebraicGeometry.EllipticCurve.Affine.Point
import Mathlib.Tactic

namespace XedniWidePeriodWitness

open WeierstrassCurve

def E : WeierstrassCurve ℚ := ⟨94, 0, 188, 1738, 3805⟩

instance : E.IsElliptic := ⟨by
  apply isUnit_iff_ne_zero.mpr
  norm_num [E, WeierstrassCurve.Δ, WeierstrassCurve.b₂,
    WeierstrassCurve.b₄, WeierstrassCurve.b₆, WeierstrassCurve.b₈]⟩

private theorem p_on : Affine.Equation E 8 19 := by
  rw [Affine.equation_iff]
  norm_num [E]

private theorem q_on : Affine.Equation E (-4) 19 := by
  rw [Affine.equation_iff]
  norm_num [E]

def P : Affine.Point E := Affine.Point.mk p_on
def Q : Affine.Point E := Affine.Point.mk q_on

theorem rational_relation : P + 2 • Q = 0 := by
  have hdouble : Q + Q = -P := by
    have hy : (19 : ℚ) ≠ Affine.negY E (-4) 19 := by
      norm_num [Affine.negY, E]
    simp only [Q, P, Affine.Point.mk]
    rw [Affine.Point.add_self_of_Y_ne hy]
    change _ = Affine.Point.some 8 (Affine.negY E 8 19) _
    norm_num [Affine.addX, Affine.addY, Affine.slope, Affine.negY,
      Affine.negAddY, E]
  simpa [two_nsmul] using congrArg (fun T : Affine.Point E => P + T) hdouble

#print axioms p_on
#print axioms q_on
#print axioms rational_relation

end XedniWidePeriodWitness
