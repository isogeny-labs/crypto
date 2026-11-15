module
public import TranslationExpansion

@[expose] public section
namespace LocalTranslationAudit
noncomputable section
open WeierstrassCurve.Affine
open EllipticTranslationAudit MillerNormValuationAudit MillerFinitePointAudit
open scoped WithZero
variable {K : Type*} [Field K]

theorem shift_x_value (W : WeierstrassCurve.Affine K) (b : K) :
    functionValuation W (shiftX W b)=WithZero.exp (2 : ℤ) := by
  simpa only [shiftX,vertical_formula] using function_valuation_vertical W b

theorem shift_y_value (W : WeierstrassCurve.Affine K) (c : K) :
    functionValuation W (shiftY W c)=WithZero.exp (3 : ℤ) := by
  have he := function_valuation_line W 0 c 0
  simpa only [line_formula,map_zero,zero_mul,sub_zero,shiftY] using he

theorem shifted_monomial_bound (W : WeierstrassCurve.Affine K) (b c d : K) (a e n : ℕ) :
    functionValuation W (constantHom W d*shiftX W b^a*shiftY W c^e/shiftX W b^n) ≤
      WithZero.exp (2*(a : ℤ)+3*(e : ℤ)-2*(n : ℤ)) := by
  rw [Valuation.map_div,Valuation.map_mul,Valuation.map_mul,Valuation.map_pow,
    Valuation.map_pow,Valuation.map_pow,shift_x_value,shift_y_value]
  calc
    _ ≤ 1*WithZero.exp (2 : ℤ)^a*WithZero.exp (3 : ℤ)^e/WithZero.exp (2 : ℤ)^n := by
      gcongr <;> first | exact function_valuation_constant_le W d | exact zero_le
    _ = _ := by
      rw [one_mul,← WithZero.exp_nsmul,← WithZero.exp_nsmul,← WithZero.exp_nsmul,
        ← WithZero.exp_add,← WithZero.exp_sub]
      congr 1
      simp only [nsmul_eq_mul]
      ring

theorem shifted_monomial_small (W : WeierstrassCurve.Affine K) (b c d : K) (a e n : ℕ)
    (h : 2*(a : ℤ)+3*(e : ℤ)-2*(n : ℤ)<0) :
    functionValuation W (constantHom W d*shiftX W b^a*shiftY W c^e/shiftX W b^n)<1 :=
  (shifted_monomial_bound W b c d a e n).trans_lt (WithZero.exp_lt_one_iff.mpr h)

variable [DecidableEq K] [IsAlgClosed K]

/-- Cancellation uses the actual shifted curve equation, not a formal asymptotic premise. -/
theorem delta_y_expanded (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    {b c : K} (h : W.Nonsingular b c) :
    deltaY W hΔ h=
      constantHom W (-negativeDerivativeX W b c)*shiftX W b*shiftY W c/shiftX W b^3+
      constantHom W (derivativeY W b c*(3*b+W.a₂)-W.a₁*negativeDerivativeX W b c)*
        shiftX W b^2/shiftX W b^3+
      constantHom W (derivativeY W b c*negativeDerivativeX W b c)*shiftX W b/shiftX W b^3-
      constantHom W (derivativeY W b c^2)*shiftY W c/shiftX W b^3 := by
  rw [delta_y_formula,delta_x_formula]
  simp only [map_neg,map_sub,map_mul,map_pow]
  field_simp [shift_x_nonzero W b]
  linear_combination constantHom W (derivativeY W b c)*shifted_field_relation W h.1

theorem delta_x_small (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    {b c : K} (h : W.Nonsingular b c) : functionValuation W (deltaX W hΔ h)<1 := by
  rw [delta_x_formula,sub_div]
  apply (functionValuation W).map_sub_lt
  · simpa only [pow_one,pow_zero,mul_one] using
      shifted_monomial_small W b c (negativeDerivativeX W b c) 1 0 2 (by norm_num)
  · simpa only [pow_one,pow_zero,mul_one,one_mul] using
      shifted_monomial_small W b c (derivativeY W b c) 0 1 2 (by norm_num)

theorem delta_y_small (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    {b c : K} (h : W.Nonsingular b c) : functionValuation W (deltaY W hΔ h)<1 := by
  rw [delta_y_expanded]
  apply (functionValuation W).map_sub_lt
  · apply (functionValuation W).map_add_lt
    · apply (functionValuation W).map_add_lt
      · simpa only [pow_one] using
          shifted_monomial_small W b c (-negativeDerivativeX W b c) 1 1 3 (by norm_num)
      · simpa only [pow_zero,mul_one] using shifted_monomial_small W b c
          (derivativeY W b c*(3*b+W.a₂)-W.a₁*negativeDerivativeX W b c) 2 0 3 (by norm_num)
    · simpa only [pow_one,pow_zero,mul_one] using shifted_monomial_small W b c
        (derivativeY W b c*negativeDerivativeX W b c) 1 0 3 (by norm_num)
  · simpa only [pow_one,pow_zero,mul_one,one_mul] using
      shifted_monomial_small W b c (derivativeY W b c^2) 0 1 3 (by norm_num)

theorem delta_x_parameter_value (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    {b c : K} (h : W.Nonsingular b c) (ht : derivativeY W b c≠0) :
    functionValuation W (deltaX W hΔ h)=WithZero.exp (-1 : ℤ) := by
  have hlow : functionValuation W
      (constantHom W (negativeDerivativeX W b c)*shiftX W b) ≤ WithZero.exp (2 : ℤ) := by
    rw [Valuation.map_mul,shift_x_value]
    simpa only [one_mul] using mul_le_mul_of_nonneg_right
      (function_valuation_constant_le W (negativeDerivativeX W b c)) zero_le
  have hhigh : functionValuation W (constantHom W (derivativeY W b c)*shiftY W c)=
      WithZero.exp (3 : ℤ) := by
    rw [Valuation.map_mul,function_valuation_constant W ht,shift_y_value,one_mul]
  rw [delta_x_formula,Valuation.map_div,
    (functionValuation W).map_sub_eq_of_lt_right (hlow.trans_lt (by rw [hhigh];norm_num)),
    hhigh,Valuation.map_pow,shift_x_value,← WithZero.exp_nsmul,← WithZero.exp_sub]
  norm_num

theorem delta_x_zero_derivative (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    {b c : K} (h : W.Nonsingular b c) (ht : derivativeY W b c=0) :
    deltaX W hΔ h=constantHom W (negativeDerivativeX W b c)/shiftX W b := by
  rw [delta_x_formula,ht,map_zero,zero_mul,sub_zero]
  field_simp [shift_x_nonzero W b]

theorem delta_y_parameter_value (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    {b c : K} (h : W.Nonsingular b c) (ht : derivativeY W b c=0) :
    functionValuation W (deltaY W hΔ h)=WithZero.exp (-1 : ℤ) := by
  have hA : negativeDerivativeX W b c≠0 :=
    (nonsingular_derivatives W h).resolve_left (not_ne_iff.mpr ht)
  have hSlope : functionValuation W (shiftY W c/shiftX W b)=WithZero.exp (1 : ℤ) := by
    rw [Valuation.map_div,shift_y_value,shift_x_value,← WithZero.exp_sub]
    norm_num
  have hs : functionValuation W (shiftY W c/shiftX W b+constantHom W W.a₁)=
      WithZero.exp (1 : ℤ) := by
    rw [(functionValuation W).map_add_eq_of_lt_left
      ((function_valuation_constant_le W W.a₁).trans_lt (by rw [hSlope];norm_num)),hSlope]
  rw [delta_y_formula,ht,map_zero,sub_zero,delta_x_zero_derivative W hΔ h ht,
    Valuation.map_mul,Valuation.map_neg,hs,Valuation.map_div,
    function_valuation_constant W hA,shift_x_value,← WithZero.exp_zero,
    ← WithZero.exp_sub,← WithZero.exp_add]
  norm_num

#print axioms delta_x_small
#print axioms delta_y_small
#print axioms delta_x_parameter_value
#print axioms delta_y_parameter_value
end
end LocalTranslationAudit
