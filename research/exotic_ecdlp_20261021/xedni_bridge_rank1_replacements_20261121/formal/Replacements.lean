import Mathlib.AlgebraicGeometry.EllipticCurve.Affine.Point
import Mathlib.Tactic

namespace XedniRankOneReplacements

open WeierstrassCurve

def E85 : WeierstrassCurve ℚ := ⟨-47, -47/9, 0, -7999/9, 1345/9⟩

instance : E85.IsElliptic := ⟨by
  apply isUnit_iff_ne_zero.mpr
  norm_num [E85, WeierstrassCurve.Δ, WeierstrassCurve.b₂,
    WeierstrassCurve.b₄, WeierstrassCurve.b₆, WeierstrassCurve.b₈]⟩

private theorem p85_on : Affine.Equation E85 5 20 := by
  rw [Affine.equation_iff]
  norm_num [E85]

private theorem q85_on : Affine.Equation E85 8 19 := by
  rw [Affine.equation_iff]
  norm_num [E85]

def P85 : Affine.Point E85 := Affine.Point.mk p85_on
def Q85 : Affine.Point E85 := Affine.Point.mk q85_on

theorem rational_relation_85 : P85 + 2 • Q85 = 0 := by
  have hdouble : Q85 + Q85 = -P85 := by
    have hy : (19 : ℚ) ≠ Affine.negY E85 8 19 := by
      norm_num [Affine.negY, E85]
    simp only [Q85, P85, Affine.Point.mk]
    rw [Affine.Point.add_self_of_Y_ne hy]
    change _ = Affine.Point.some 5 (Affine.negY E85 5 20) _
    norm_num [Affine.addX, Affine.addY, Affine.slope, Affine.negY,
      Affine.negAddY, E85]
  simpa [two_nsmul] using congrArg (fun T : Affine.Point E85 => P85 + T) hdouble

#print axioms rational_relation_85

def E353 : WeierstrassCurve ℚ := ⟨0, -16591/256, 47, 172221/128, -2025319/256⟩

instance : E353.IsElliptic := ⟨by
  apply isUnit_iff_ne_zero.mpr
  norm_num [E353, WeierstrassCurve.Δ, WeierstrassCurve.b₂,
    WeierstrassCurve.b₄, WeierstrassCurve.b₆, WeierstrassCurve.b₈]⟩

private theorem p353_on : Affine.Equation E353 11 7 := by
  rw [Affine.equation_iff]
  norm_num [E353]

private theorem q353_on : Affine.Equation E353 27 14 := by
  rw [Affine.equation_iff]
  norm_num [E353]

def P353 : Affine.Point E353 := Affine.Point.mk p353_on
def Q353 : Affine.Point E353 := Affine.Point.mk q353_on

theorem rational_relation_353 : P353 + 2 • Q353 = 0 := by
  have hdouble : Q353 + Q353 = -P353 := by
    have hy : (14 : ℚ) ≠ Affine.negY E353 27 14 := by
      norm_num [Affine.negY, E353]
    simp only [Q353, P353, Affine.Point.mk]
    rw [Affine.Point.add_self_of_Y_ne hy]
    change _ = Affine.Point.some 11 (Affine.negY E353 11 7) _
    norm_num [Affine.addX, Affine.addY, Affine.slope, Affine.negY,
      Affine.negAddY, E353]
  simpa [two_nsmul] using congrArg (fun T : Affine.Point E353 => P353 + T) hdouble

#print axioms rational_relation_353

end XedniRankOneReplacements
