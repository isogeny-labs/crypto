module
public import LocalTranslationCertificate
public import Mathlib.RingTheory.Valuation.IsTrivialOn
public import Mathlib.RingTheory.Algebraic.Integral

@[expose] public section
namespace GeneralTranslationAudit
noncomputable section
open WeierstrassCurve.Affine
open EllipticTranslationAudit LocalTranslationAudit
open MillerNormValuationAudit MillerFinitePointAudit MillerBasechangeAudit
open scoped WithZero
variable {K : Type*} [Field K]

local instance coefficientValuationTrivial (W : WeierstrassCurve.Affine K) :
    (functionValuation W).IsTrivialOn K where
  eq_one c hc := by
    rw [function_coefficient_map]
    exact function_valuation_constant W hc

variable [DecidableEq K]

/-- Actual generic translated-coordinate difference, defined without an algebraic-closure instance. -/
def genericDifference (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    {b c : K} (h : W.Nonsingular b c) : W.FunctionField :=
  translatedX W hΔ (.some b c h)-constantHom W b

theorem generic_difference_formula (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    {b c : K} (h : W.Nonsingular b c) :
    genericDifference W hΔ h=(constantHom W (negativeDerivativeX W b c)*shiftX W b-
      constantHom W (derivativeY W b c)*shiftY W c)/shiftX W b^2 := by
  rw [genericDifference,(translated_generic_coordinates W hΔ h).1,
    WeierstrassCurve.Affine.slope_of_X_ne (x_function_ne_constant W b),
    WeierstrassCurve.Affine.addX]
  obtain ⟨h₁,h₂,_⟩ := generic_coefficients W
  rw [h₁,h₂]
  have hq := congrArg (constantHom W) ((W.equation_iff b c).mp h.1)
  simp only [map_add,map_mul,map_pow] at hq
  simp only [shiftX,shiftY,derivativeY,negativeDerivativeX,map_add,map_sub,map_mul,
    map_pow,map_ofNat]
  field_simp [sub_ne_zero.mpr (x_function_ne_constant W b)]
  linear_combination function_curve_relation W-hq

/-- The actual difference has nontrivial normalized value over every coefficient field. -/
theorem generic_difference_value (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    {b c : K} (h : W.Nonsingular b c) :
    functionValuation W (genericDifference W hΔ h)=
      if derivativeY W b c≠0 then WithZero.exp (-1 : ℤ) else WithZero.exp (-2 : ℤ) := by
  classical
  by_cases ht : derivativeY W b c≠0
  · rw [ite_eq_left ht]
    have hlow : functionValuation W
        (constantHom W (negativeDerivativeX W b c)*shiftX W b)≤WithZero.exp (2 : ℤ) := by
      rw [Valuation.map_mul,shift_x_value]
      simpa only [one_mul] using mul_le_mul_of_nonneg_right
        (function_valuation_constant_le W (negativeDerivativeX W b c)) zero_le
    have hhigh : functionValuation W (constantHom W (derivativeY W b c)*shiftY W c)=
        WithZero.exp (3 : ℤ) := by
      rw [Valuation.map_mul,function_valuation_constant W ht,shift_y_value,one_mul]
    rw [generic_difference_formula,Valuation.map_div,
      (functionValuation W).map_sub_eq_of_lt_right (hlow.trans_lt (by rw [hhigh];norm_num)),
      hhigh,Valuation.map_pow,shift_x_value,← WithZero.exp_nsmul,← WithZero.exp_sub]
    norm_num
  · rw [ite_eq_right ht]
    have hz : derivativeY W b c=0 := not_ne_iff.mp ht
    have hA : negativeDerivativeX W b c≠0 := (nonsingular_derivatives W h).resolve_left ht
    have he : genericDifference W hΔ h=constantHom W (negativeDerivativeX W b c)/shiftX W b := by
      rw [generic_difference_formula,hz,map_zero,zero_mul,sub_zero]
      field_simp [shift_x_nonzero W b]
    rw [he,Valuation.map_div,function_valuation_constant W hA,shift_x_value,
      ← WithZero.exp_zero,← WithZero.exp_sub]
    norm_num

theorem generic_difference_transcendental (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    {b c : K} (h : W.Nonsingular b c) : Transcendental K (genericDifference W hΔ h) := by
  have hv : functionValuation W (genericDifference W hΔ h)≠0 := by
    rw [generic_difference_value]
    split_ifs <;> exact WithZero.exp_ne_zero
  have hf : genericDifference W hΔ h≠0 := by
    intro he
    exact hv (he ▸ map_zero _)
  have hn : functionValuation W (genericDifference W hΔ h)≠1 := by
    rw [generic_difference_value]
    split_ifs <;> norm_num
  exact Valuation.transcendental_of_ne_one K (v := functionValuation W) _ hf hn

/-- Derived transcendence for every actual translated generic abscissa over every field. -/
theorem general_translated_x_transcendental (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (Q : W.Point) : Transcendental K (translatedX W hΔ Q) := by
  cases Q with
  | zero =>
    have h0 : constantPoint W Point.zero=(Point.zero : (genericCurve W).Point) := rfl
    rw [translatedX,translatedGeneric,h0,← Point.zero_def,add_zero]
    change Transcendental K (xFunction W)
    apply Valuation.transcendental_of_ne_one K (v := functionValuation W) _ (x_function_nonzero W)
    rw [function_valuation_x]
    norm_num
  | some b c h =>
    intro hx
    have hc : IsAlgebraic K (constantHom W b) := by
      rw [← function_coefficient_map]
      exact isAlgebraic_algebraMap b
    exact generic_difference_transcendental W hΔ h (hx.sub hc)

#print axioms generic_difference_value
#print axioms general_translated_x_transcendental
end
end GeneralTranslationAudit
