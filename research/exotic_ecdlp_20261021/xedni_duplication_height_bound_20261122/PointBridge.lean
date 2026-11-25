import Global
import Mathlib.NumberTheory.Height.EllipticCurve
import Mathlib.NumberTheory.Height.NumberField

namespace XedniDuplicationPointBridge

open Height

def FixedCurve : WeierstrassCurve.Affine ℚ := ⟨0, -47, 0, 751, -3997⟩

private theorem xRep_double_ne {x y : ℚ} (h : FixedCurve.Nonsingular x y)
    (hn : y ≠ FixedCurve.negY x y) :
    (WeierstrassCurve.Affine.Point.some x y h + WeierstrassCurve.Affine.Point.some x y h).xRep 0 =
      (x ^ 4 - 1502 * x ^ 2 + 31976 * x - 187435) /
        (4 * x ^ 3 - 188 * x ^ 2 + 3004 * x - 15988) := by
  have hrep := WeierstrassCurve.Affine.Point.xRep_add_self_of_Y_ne h hn
  have hzero := congrArg (fun v : Fin 2 → ℚ => v 0) hrep
  norm_num [FixedCurve, WeierstrassCurve.b₂, WeierstrassCurve.b₄,
    WeierstrassCurve.b₆, WeierstrassCurve.b₈] at hzero
  convert hzero using 1; ring

theorem naiveHeight_double_ne {x y : ℚ} (h : FixedCurve.Nonsingular x y)
    (hn : y ≠ FixedCurve.negY x y) :
    (WeierstrassCurve.Affine.Point.some x y h + WeierstrassCurve.Affine.Point.some x y h).naiveHeight =
      logHeight₁ ((x ^ 4 - 1502 * x ^ 2 + 31976 * x - 187435) /
        (4 * x ^ 3 - 188 * x ^ 2 + 3004 * x - 15988)) := by
  rw [WeierstrassCurve.Affine.Point.naiveHeight_eq_logHeight₁]
  rw [xRep_double_ne h hn]

theorem naiveHeight_double_eq {x y : ℚ} (h : FixedCurve.Nonsingular x y)
    (hn : y = FixedCurve.negY x y) :
    (WeierstrassCurve.Affine.Point.some x y h + WeierstrassCurve.Affine.Point.some x y h).naiveHeight = 0 := by
  have hrep := WeierstrassCurve.Affine.Point.xRep_add_self_of_Y_eq h hn
  have hzero := congrArg (fun v : Fin 2 → ℚ => v 0) hrep
  rw [WeierstrassCurve.Affine.Point.naiveHeight_eq_logHeight₁, hzero]
  norm_num [Rat.logHeight₁_eq_log_max]

#print axioms naiveHeight_double_ne
#print axioms naiveHeight_double_eq

end XedniDuplicationPointBridge
