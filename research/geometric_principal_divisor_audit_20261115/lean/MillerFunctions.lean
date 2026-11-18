module
public import FieldValuation

@[expose] public section
namespace MillerNormValuationAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine WeierstrassCurve.Affine.CoordinateRing
open scoped WithZero
variable {K : Type*} [Field K]

def verticalFunction (W : WeierstrassCurve.Affine K) (x : K) : W.FunctionField :=
  algebraMap W.CoordinateRing W.FunctionField (XClass W x)

def lineFunction (W : WeierstrassCurve.Affine K) (x y slope : K) : W.FunctionField :=
  algebraMap W.CoordinateRing W.FunctionField (YClass W (linePolynomial x y slope))

def parameter (W : WeierstrassCurve.Affine K) : W.FunctionField :=
  xFunction W / yFunction W

theorem vertical_formula (W : WeierstrassCurve.Affine K) (x : K) :
    verticalFunction W x = xFunction W - constantHom W x := by
  simp only [verticalFunction, xFunction, XClass, constantHom, RingHom.comp_apply,
    map_sub, map_zero, sub_zero]

theorem line_formula (W : WeierstrassCurve.Affine K) (x y slope : K) :
    lineFunction W x y slope = yFunction W - constantHom W y -
      constantHom W slope * (xFunction W - constantHom W x) := by
  simp only [lineFunction, linePolynomial, yFunction, xFunction, XClass, YClass,
    constantHom, RingHom.comp_apply, map_sub, map_add, map_mul, map_zero, sub_zero]
  ring

theorem x_function_nonzero (W : WeierstrassCurve.Affine K) : xFunction W ≠ 0 := by
  apply (Valuation.ne_zero_iff (functionValuation W)).mp
  rw [function_valuation_x]
  exact WithZero.exp_ne_zero

theorem y_function_nonzero (W : WeierstrassCurve.Affine K) : yFunction W ≠ 0 := by
  apply (Valuation.ne_zero_iff (functionValuation W)).mp
  rw [function_valuation_y]
  exact WithZero.exp_ne_zero

theorem function_curve_relation (W : WeierstrassCurve.Affine K) :
    yFunction W ^ 2 + (constantHom W W.a₁ * xFunction W + constantHom W W.a₃) * yFunction W =
      xFunction W ^ 3 + constantHom W W.a₂ * xFunction W ^ 2 +
        constantHom W W.a₄ * xFunction W + constantHom W W.a₆ := by
  have h₀ : mk W W.polynomial = 0 := AdjoinRoot.mk_self
  have h := congrArg (algebraMap W.CoordinateRing W.FunctionField) h₀
  simp only [WeierstrassCurve.Affine.polynomial, map_sub, map_add, map_mul, map_pow, map_zero] at h
  simp only [yFunction, xFunction, YClass, XClass, constantHom, RingHom.comp_apply,
    map_zero, sub_zero]
  exact sub_eq_zero.mp h

theorem function_valuation_vertical (W : WeierstrassCurve.Affine K) (x : K) :
    functionValuation W (verticalFunction W x) = WithZero.exp (2 : ℤ) := by
  rw [vertical_formula]
  have h : functionValuation W (constantHom W x) < functionValuation W (xFunction W) := by
    rw [function_valuation_x]
    exact (function_valuation_constant_le W x).trans_lt (by norm_num)
  rw [(functionValuation W).map_sub_eq_of_lt_left h, function_valuation_x]

theorem function_valuation_line (W : WeierstrassCurve.Affine K) (x y slope : K) :
    functionValuation W (lineFunction W x y slope) = WithZero.exp (3 : ℤ) := by
  rw [line_formula]
  have hy : functionValuation W (constantHom W y) < functionValuation W (yFunction W) := by
    rw [function_valuation_y]
    exact (function_valuation_constant_le W y).trans_lt (by norm_num)
  have hmain : functionValuation W (yFunction W - constantHom W y) = WithZero.exp (3 : ℤ) := by
    rw [(functionValuation W).map_sub_eq_of_lt_left hy, function_valuation_y]
  have hlower : functionValuation W
      (constantHom W slope * (xFunction W - constantHom W x)) ≤ WithZero.exp (2 : ℤ) := by
    rw [Valuation.map_mul, ← vertical_formula, function_valuation_vertical]
    simpa only [one_mul] using mul_le_mul_of_nonneg_right (function_valuation_constant_le W slope)
      zero_le
  rw [(functionValuation W).map_sub_eq_of_lt_left (hlower.trans_lt (by rw [hmain]; norm_num)), hmain]

theorem function_valuation_parameter (W : WeierstrassCurve.Affine K) :
    functionValuation W (parameter W) = WithZero.exp (-1 : ℤ) := by
  rw [parameter, Valuation.map_div, function_valuation_x, function_valuation_y, ← WithZero.exp_sub]
  norm_num

/-- The reciprocal X has weight minus two, unlike the source parameter X/Y. -/
theorem reciprocal_x_control (W : WeierstrassCurve.Affine K) :
    functionValuation W (xFunction W)⁻¹ = WithZero.exp (-2 : ℤ) ∧
      functionValuation W (xFunction W)⁻¹ ≠ functionValuation W (parameter W) := by
  rw [Valuation.map_inv, function_valuation_x, WithZero.inv_exp, function_valuation_parameter]
  constructor
  · rfl
  · norm_num

#print axioms function_valuation_line
#print axioms function_curve_relation
#print axioms reciprocal_x_control
end
end MillerNormValuationAudit
