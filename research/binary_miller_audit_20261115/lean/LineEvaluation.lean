module
public import LineIdeals

@[expose] public section
namespace MillerPrincipalIdealAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine WeierstrassCurve.Affine.CoordinateRing
variable {K : Type*} [Field K]

/-- Evaluation of actual coordinate-ring classes at an actual affine curve point. -/
def pointEval (W : WeierstrassCurve.Affine K) (xr yr : K) (hR : W.Equation xr yr) :
    W.CoordinateRing →+* K := AdjoinRoot.evalEval hR

theorem point_eval_XClass (W : WeierstrassCurve.Affine K) {xr yr : K}
    (hR : W.Equation xr yr) (x : K) : pointEval W xr yr hR (XClass W x) = xr - x := by
  change AdjoinRoot.evalEval (show W.polynomial.evalEval xr yr = 0 from hR)
    (AdjoinRoot.mk W.polynomial (C (X - C x))) = xr - x
  rw [AdjoinRoot.evalEval_mk]
  simp only [evalEval_C, eval_sub, eval_X, eval_C]

theorem point_eval_YClass (W : WeierstrassCurve.Affine K) {xr yr : K}
    (hR : W.Equation xr yr) (f : K[X]) : pointEval W xr yr hR (YClass W f) = yr - f.eval xr := by
  change AdjoinRoot.evalEval (show W.polynomial.evalEval xr yr = 0 from hR)
    (AdjoinRoot.mk W.polynomial (X - C f)) = yr - f.eval xr
  rw [AdjoinRoot.evalEval_mk]
  simp only [evalEval_sub, evalEval_X, evalEval_C]

theorem line_evaluation (W : WeierstrassCurve.Affine K) {xr yr : K}
    (hR : W.Equation xr yr) (x y slope : K) :
    pointEval W xr yr hR (lineClass W x y slope) = yr - y - slope * (xr - x) := by
  rw [lineClass, point_eval_YClass]
  simp only [linePolynomial, eval_add, eval_mul, eval_C, eval_sub, eval_X]
  ring

variable [DecidableEq K]

theorem short_tangent_slope (W : WeierstrassCurve.Affine K)
    (ha1 : W.a₁ = 0) (ha2 : W.a₂ = 0) (ha3 : W.a₃ = 0)
    (x y : K) (hy : y ≠ W.negY x y) :
    W.slope x x y y = (3 * x ^ 2 + W.a₄) / (2 * y) := by
  rw [WeierstrassCurve.Affine.slope_of_Y_ne rfl hy]
  simp only [WeierstrassCurve.Affine.negY, ha1, ha2, ha3,
    zero_mul, mul_zero, add_zero, sub_zero]
  congr 1
  ring

/-- Actual regular-function evaluation agrees with the short-model Miller tangent expression. -/
theorem short_three_miller_evaluation (W : WeierstrassCurve.Affine K)
    (ha1 : W.a₁ = 0) (ha2 : W.a₂ = 0) (ha3 : W.a₃ = 0)
    {x y xr yr : K} (hN : W.Nonsingular x y)
    (h3 : (3 : ℕ) • Point.some x y hN = 0) (hR : W.Equation xr yr) :
    pointEval W xr yr hR (lineClass W x y (W.slope x x y y)) =
      yr - y - ((3 * x ^ 2 + W.a₄) / (2 * y)) * (xr - x) := by
  rw [line_evaluation, short_tangent_slope W ha1 ha2 ha3 x y
    (three_tangent_nonvertical W hN h3)]

#print axioms line_evaluation
#print axioms short_three_miller_evaluation
end
end MillerPrincipalIdealAudit
