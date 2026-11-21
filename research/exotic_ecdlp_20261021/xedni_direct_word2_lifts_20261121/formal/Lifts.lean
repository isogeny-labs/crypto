import Mathlib.AlgebraicGeometry.EllipticCurve.Affine.Point
import Mathlib.Tactic

namespace XedniDirectWord2Lifts

open WeierstrassCurve

def E2 : WeierstrassCurve ℚ := ⟨0, 517/9, 0, -1795/9, 1486/9⟩

instance : E2.IsElliptic := ⟨by
  apply isUnit_iff_ne_zero.mpr
  norm_num [E2, WeierstrassCurve.Δ, WeierstrassCurve.b₂,
    WeierstrassCurve.b₄, WeierstrassCurve.b₆, WeierstrassCurve.b₈]⟩

private theorem p2_on : Affine.Equation E2 2 2 := by
  rw [Affine.equation_iff]
  norm_num [E2]

private theorem q2_on : Affine.Equation E2 5 27 := by
  rw [Affine.equation_iff]
  norm_num [E2]

def P2 : Affine.Point E2 := Affine.Point.mk p2_on
def Q2 : Affine.Point E2 := Affine.Point.mk q2_on

theorem rational_relation_2 : P2 + 2 • Q2 = 0 := by
  have hdouble : Q2 + Q2 = -P2 := by
    have hy : (27 : ℚ) ≠ Affine.negY E2 5 27 := by
      norm_num [Affine.negY, E2]
    simp only [Q2, P2, Affine.Point.mk]
    rw [Affine.Point.add_self_of_Y_ne hy]
    change _ = Affine.Point.some 2 (Affine.negY E2 2 2) _
    norm_num [Affine.addX, Affine.addY, Affine.slope, Affine.negY,
      Affine.negAddY, E2]
  simpa [two_nsmul] using congrArg (fun T : Affine.Point E2 => P2 + T) hdouble

#print axioms rational_relation_2

def E85 : WeierstrassCurve ℚ := ⟨0, -188/9, 0, 1166/9, 1345/9⟩

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

def E201 : WeierstrassCurve ℚ := ⟨0, -94, 0, 2537, -14431⟩

instance : E201.IsElliptic := ⟨by
  apply isUnit_iff_ne_zero.mpr
  norm_num [E201, WeierstrassCurve.Δ, WeierstrassCurve.b₂,
    WeierstrassCurve.b₄, WeierstrassCurve.b₆, WeierstrassCurve.b₈]⟩

private theorem p201_on : Affine.Equation E201 8 19 := by
  rw [Affine.equation_iff]
  norm_num [E201]

private theorem q201_on : Affine.Equation E201 43 19 := by
  rw [Affine.equation_iff]
  norm_num [E201]

def P201 : Affine.Point E201 := Affine.Point.mk p201_on
def Q201 : Affine.Point E201 := Affine.Point.mk q201_on

theorem rational_relation_201 : P201 + 2 • Q201 = 0 := by
  have hdouble : Q201 + Q201 = -P201 := by
    have hy : (19 : ℚ) ≠ Affine.negY E201 43 19 := by
      norm_num [Affine.negY, E201]
    simp only [Q201, P201, Affine.Point.mk]
    rw [Affine.Point.add_self_of_Y_ne hy]
    change _ = Affine.Point.some 8 (Affine.negY E201 8 19) _
    norm_num [Affine.addX, Affine.addY, Affine.slope, Affine.negY,
      Affine.negAddY, E201]
  simpa [two_nsmul] using congrArg (fun T : Affine.Point E201 => P201 + T) hdouble

#print axioms rational_relation_201

def E353 : WeierstrassCurve ℚ := ⟨0, -16591/256, 0, 169589/128, -2051639/256⟩

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

def E511 : WeierstrassCurve ℚ := ⟨0, -235/4, 0, 2207/2, -25811/4⟩

instance : E511.IsElliptic := ⟨by
  apply isUnit_iff_ne_zero.mpr
  norm_num [E511, WeierstrassCurve.Δ, WeierstrassCurve.b₂,
    WeierstrassCurve.b₄, WeierstrassCurve.b₆, WeierstrassCurve.b₈]⟩

private theorem p511_on : Affine.Equation E511 15 16 := by
  rw [Affine.equation_iff]
  norm_num [E511]

private theorem q511_on : Affine.Equation E511 23 4 := by
  rw [Affine.equation_iff]
  norm_num [E511]

def P511 : Affine.Point E511 := Affine.Point.mk p511_on
def Q511 : Affine.Point E511 := Affine.Point.mk q511_on

theorem rational_relation_511 : P511 + 2 • Q511 = 0 := by
  have hdouble : Q511 + Q511 = -P511 := by
    have hy : (4 : ℚ) ≠ Affine.negY E511 23 4 := by
      norm_num [Affine.negY, E511]
    simp only [Q511, P511, Affine.Point.mk]
    rw [Affine.Point.add_self_of_Y_ne hy]
    change _ = Affine.Point.some 15 (Affine.negY E511 15 16) _
    norm_num [Affine.addX, Affine.addY, Affine.slope, Affine.negY,
      Affine.negAddY, E511]
  simpa [two_nsmul] using congrArg (fun T : Affine.Point E511 => P511 + T) hdouble

#print axioms rational_relation_511

end XedniDirectWord2Lifts
