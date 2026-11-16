module
public import MillerFunctions
public import Mathlib.RingTheory.Valuation.ValuationSubring

@[expose] public section
namespace MillerNormValuationAudit
noncomputable section
open scoped WithZero
variable {K : Type*} [Field K]

theorem constant_mul_small (W : WeierstrassCurve.Affine K) (c : K)
    {f : W.FunctionField} (hf : functionValuation W f < 1) :
    functionValuation W (constantHom W c * f) < 1 := by
  rw [Valuation.map_mul]
  exact (mul_le_mul_of_nonneg_right (function_valuation_constant_le W c) zero_le).trans_lt
    (by simpa only [one_mul] using hf)

theorem monomial_small (W : WeierstrassCurve.Affine K) (c : K) (a b : ℕ)
    (h : 2 * (a : ℤ) - 3 * (b : ℤ) < 0) :
    functionValuation W (constantHom W c * xFunction W ^ a / yFunction W ^ b) < 1 := by
  have hv : functionValuation W (xFunction W ^ a / yFunction W ^ b) =
      WithZero.exp (2 * (a : ℤ) - 3 * (b : ℤ)) := by
    rw [Valuation.map_div, Valuation.map_pow, Valuation.map_pow,
      function_valuation_x, function_valuation_y, ← WithZero.exp_nsmul,
      ← WithZero.exp_nsmul, ← WithZero.exp_sub]
    congr 1
    simp only [nsmul_eq_mul]
    ring
  rw [mul_div_assoc]
  apply constant_mul_small
  rw [hv]
  exact WithZero.exp_lt_one_iff.mpr h

/-- Actual curve equation makes X^3/Y^2 have residue one for the constructed valuation. -/
theorem cubic_quotient_normalized (W : WeierstrassCurve.Affine K) :
    functionValuation W (xFunction W ^ 3 / yFunction W ^ 2 - 1) < 1 := by
  have he : xFunction W ^ 3 / yFunction W ^ 2 - 1 =
      constantHom W W.a₁ * xFunction W / yFunction W + constantHom W W.a₃ / yFunction W -
      constantHom W W.a₂ * xFunction W ^ 2 / yFunction W ^ 2 -
      constantHom W W.a₄ * xFunction W / yFunction W ^ 2 -
      constantHom W W.a₆ / yFunction W ^ 2 := by
    field_simp [y_function_nonzero W]
    linear_combination - function_curve_relation W
  have h1 : functionValuation W (constantHom W W.a₁ * xFunction W / yFunction W) < 1 := by
    simpa only [pow_one] using monomial_small W W.a₁ 1 1 (by norm_num)
  have h3 : functionValuation W (constantHom W W.a₃ / yFunction W) < 1 := by
    simpa only [pow_zero, mul_one, pow_one] using monomial_small W W.a₃ 0 1 (by norm_num)
  have h2 := monomial_small W W.a₂ 2 2 (by norm_num)
  have h4 : functionValuation W (constantHom W W.a₄ * xFunction W / yFunction W ^ 2) < 1 := by
    simpa only [pow_one] using monomial_small W W.a₄ 1 2 (by norm_num)
  have h6 : functionValuation W (constantHom W W.a₆ / yFunction W ^ 2) < 1 := by
    simpa only [pow_zero, mul_one] using monomial_small W W.a₆ 0 2 (by norm_num)
  rw [he]
  exact (functionValuation W).map_sub_lt
    ((functionValuation W).map_sub_lt
      ((functionValuation W).map_sub_lt ((functionValuation W).map_add_lt h1 h3) h2) h4) h6

/-- Nonvertical line normalized by the actual source parameter X/Y has residue one. -/
theorem line_normalized (W : WeierstrassCurve.Affine K) (x y slope : K) :
    functionValuation W (parameter W ^ 3 * lineFunction W x y slope - 1) < 1 := by
  have he : parameter W ^ 3 * lineFunction W x y slope - 1 =
      (xFunction W ^ 3 / yFunction W ^ 2 - 1) - constantHom W y * parameter W ^ 3 -
        constantHom W slope * (parameter W ^ 3 * verticalFunction W x) := by
    rw [line_formula, vertical_formula, parameter]
    field_simp [y_function_nonzero W]
    ring
  have h0 : functionValuation W (parameter W ^ 3) < 1 := by
    rw [Valuation.map_pow, function_valuation_parameter, ← WithZero.exp_nsmul]
    norm_num
  have hv : functionValuation W (parameter W ^ 3 * verticalFunction W x) < 1 := by
    rw [Valuation.map_mul, Valuation.map_pow, function_valuation_parameter,
      function_valuation_vertical, ← WithZero.exp_nsmul, ← WithZero.exp_add]
    norm_num
  rw [he]
  exact (functionValuation W).map_sub_lt
    ((functionValuation W).map_sub_lt (cubic_quotient_normalized W) (constant_mul_small W y h0))
    (constant_mul_small W slope hv)

theorem vertical_normalized (W : WeierstrassCurve.Affine K) (x : K) :
    functionValuation W (parameter W ^ 2 * verticalFunction W x - 1) < 1 := by
  have he : parameter W ^ 2 * verticalFunction W x - 1 =
      (xFunction W ^ 3 / yFunction W ^ 2 - 1) - constantHom W x * parameter W ^ 2 := by
    rw [vertical_formula, parameter]
    field_simp
    ring
  have h0 : functionValuation W (parameter W ^ 2) < 1 := by
    rw [Valuation.map_pow, function_valuation_parameter, ← WithZero.exp_nsmul]
    norm_num
  rw [he]
  exact (functionValuation W).map_sub_lt (cubic_quotient_normalized W) (constant_mul_small W x h0)

/-- Replacing X/Y with 1/X destroys the residue-one line normalization. -/
theorem wrong_parameter_normalization (W : WeierstrassCurve.Affine K) (x y slope : K) :
    functionValuation W ((xFunction W)⁻¹ ^ 3 * lineFunction W x y slope - 1) = 1 := by
  have h : functionValuation W ((xFunction W)⁻¹ ^ 3 * lineFunction W x y slope) < 1 := by
    rw [Valuation.map_mul, Valuation.map_pow, Valuation.map_inv, function_valuation_x,
      function_valuation_line, WithZero.inv_exp, ← WithZero.exp_nsmul, ← WithZero.exp_add]
    norm_num
  have he := (functionValuation W).map_sub_eq_of_lt_right
    (x := (xFunction W)⁻¹ ^ 3 * lineFunction W x y slope) (y := 1)
    (by simpa only [map_one] using h)
  simpa only [map_one] using he

#print axioms line_normalized
#print axioms vertical_normalized
#print axioms wrong_parameter_normalization
end
end MillerNormValuationAudit
