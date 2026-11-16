module
public import PolynomialValues

@[expose] public section
namespace FunctionDivisorFinitenessAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine WeierstrassCurve.Affine.CoordinateRing
open MillerPrincipalIdealAudit MillerFinitePointAudit MillerNormValuationAudit
open MillerInfinityChartAudit MillerRecursionAudit
open scoped WithZero
local instance : Fact (Nat.Prime 3) := ⟨by decide⟩
abbrev F3 := ZMod 3
local instance : CommRing F3 := (inferInstance : Field F3).toCommRing

def rationalCurve : WeierstrassCurve.Affine F3 := ⟨0, 0, 0, 1, 1⟩
theorem rational_delta : rationalCurve.Δ ≠ 0 := by decide +kernel

def numeratorPolynomial : F3[X] := X ^ 2 + 1
def denominatorPolynomial : F3[X] := X ^ 2 + X + C 2

theorem numerator_degree : numeratorPolynomial.degree = 2 := by
  have h : (1 : F3[X]).degree < (X ^ 2 : F3[X]).degree := by simp
  rw [numeratorPolynomial, degree_add_eq_left_of_degree_lt h]
  simp

theorem denominator_degree : denominatorPolynomial.degree = 2 := by
  have hx : (X : F3[X]).degree < (X ^ 2 : F3[X]).degree := by simp
  have hs : (X ^ 2 + X : F3[X]).degree = 2 := by
    rw [degree_add_eq_left_of_degree_lt hx]
    simp
  have hc : (C (2 : F3)).degree < (X ^ 2 + X : F3[X]).degree := by
    rw [hs]
    exact degree_C_le.trans_lt (by norm_num)
  rw [denominatorPolynomial, degree_add_eq_left_of_degree_lt hc, hs]

theorem numerator_polynomial_nonzero : numeratorPolynomial ≠ 0 := by
  intro h
  have hd := congrArg Polynomial.degree h
  rw [numerator_degree, degree_zero] at hd
  exact (by decide : (2 : WithBot ℕ) ≠ ⊥) hd

theorem denominator_polynomial_nonzero : denominatorPolynomial ≠ 0 := by
  intro h
  have hd := congrArg Polynomial.degree h
  rw [denominator_degree, degree_zero] at hd
  exact (by decide : (2 : WithBot ℕ) ≠ ⊥) hd

theorem numerator_evaluation_nonzero : ∀ x : F3, numeratorPolynomial.eval x ≠ 0 := by
  have h : ∀ x : F3, x ^ 2 + 1 ≠ 0 := by decide +kernel
  simpa only [numeratorPolynomial, eval_add, eval_pow, eval_X, eval_one] using h

theorem denominator_evaluation_nonzero : ∀ x : F3, denominatorPolynomial.eval x ≠ 0 := by
  have h : ∀ x : F3, x ^ 2 + x + 2 ≠ 0 := by decide +kernel
  simpa only [denominatorPolynomial, eval_add, eval_pow, eval_X, eval_C] using h

def numeratorFunction : rationalCurve.FunctionField := polynomialFunction rationalCurve numeratorPolynomial
def denominatorFunction : rationalCurve.FunctionField := polynomialFunction rationalCurve denominatorPolynomial
def rationalFunction : rationalCurve.FunctionField := numeratorFunction / denominatorFunction

theorem numerator_function_nonzero : numeratorFunction ≠ 0 :=
  polynomial_function_nonzero rationalCurve numerator_polynomial_nonzero
theorem denominator_function_nonzero : denominatorFunction ≠ 0 :=
  polynomial_function_nonzero rationalCurve denominator_polynomial_nonzero
theorem rational_function_nonzero : rationalFunction ≠ 0 :=
  div_ne_zero numerator_function_nonzero denominator_function_nonzero

/-- Actual nonconstant rational function; polynomial embedding and coefficients prove this. -/
theorem rational_function_nonconstant :
    ∀ c : F3, rationalFunction ≠ constantHom rationalCurve c := by
  intro c hc
  have hm : numeratorFunction = constantHom rationalCurve c * denominatorFunction :=
    (div_eq_iff denominator_function_nonzero).mp hc
  have hp0 : numeratorPolynomial - C c * denominatorPolynomial = 0 := by
    apply polynomial_function_injective rationalCurve
    rw [map_sub, map_mul, polynomial_function_constant, map_zero]
    exact sub_eq_zero.mpr hm
  have hcoeff := congrArg (fun p : F3[X] => p.coeff 1) hp0
  have hc0 : c = 0 := by
    simpa [numeratorPolynomial, denominatorPolynomial, coeff_C_mul, coeff_one] using hcoeff
  rw [hc0] at hp0
  apply numerator_polynomial_nonzero
  simpa using hp0

theorem numerator_value : functionValuation rationalCurve numeratorFunction = WithZero.exp (4 : ℤ) := by
  rw [numeratorFunction, polynomial_function_value, numerator_degree]
  norm_num [degreeValue]

theorem denominator_value : functionValuation rationalCurve denominatorFunction = WithZero.exp (4 : ℤ) := by
  rw [denominatorFunction, polynomial_function_value, denominator_degree]
  norm_num [degreeValue]

theorem rational_infinity_order : pointOrder rationalCurve rationalFunction 0 = 0 := by
  change -WithZero.log (chartAdicValuation rationalCurve rationalFunction) = 0
  rw [chart_adic_eq_norm, rationalFunction, map_div₀, numerator_value, denominator_value]
  simp

/-- Every actual F3-rational local order is zero, despite actual nonconstancy. -/
theorem rational_point_order_zero (P : rationalCurve.Point) : pointOrder rationalCurve rationalFunction P = 0 := by
  cases P with
  | zero => exact rational_infinity_order
  | some x y h =>
    have hn : pointEval rationalCurve x y h.1 (mk rationalCurve (C numeratorPolynomial)) ≠ 0 := by
      rw [point_eval_polynomial]
      exact numerator_evaluation_nonzero x
    have hd : pointEval rationalCurve x y h.1 (mk rationalCurve (C denominatorPolynomial)) ≠ 0 := by
      rw [point_eval_polynomial]
      exact denominator_evaluation_nonzero x
    have vn := coordinate_nonvanishing_value rationalCurve h _ hn
    have vd := coordinate_nonvanishing_value rationalCurve h _ hd
    change -WithZero.log (finiteAdic rationalCurve h (numeratorFunction / denominatorFunction)) = 0
    rw [map_div₀]
    change -WithZero.log (finiteAdic rationalCurve h
      (algebraMap rationalCurve.CoordinateRing rationalCurve.FunctionField (mk rationalCurve (C numeratorPolynomial))) /
      finiteAdic rationalCurve h
      (algebraMap rationalCurve.CoordinateRing rationalCurve.FunctionField (mk rationalCurve (C denominatorPolynomial)))) = 0
    rw [vn, vd]
    simp

/-- Dropping geometric/closed-point scope makes zero-divisor-implies-constant false. -/
theorem rational_point_divisor_zero : functionDivisor rationalCurve rationalFunction rational_function_nonzero = 0 := by
  ext P
  exact rational_point_order_zero P

theorem rational_function_monic : InfinityMonic rationalCurve rationalFunction := by
  unfold InfinityMonic infinityScaled
  rw [rational_infinity_order]
  simp only [neg_zero, zpow_zero, one_mul]
  have he : rationalFunction - 1 =
      polynomialFunction rationalCurve (numeratorPolynomial - denominatorPolynomial) / denominatorFunction := by
    rw [map_sub, sub_div]
    change numeratorFunction / denominatorFunction - 1 =
      numeratorFunction / denominatorFunction - denominatorFunction / denominatorFunction
    rw [div_self denominator_function_nonzero]
  have hp : numeratorPolynomial - denominatorPolynomial = -X - 1 := by
    simp only [numeratorPolynomial, denominatorPolynomial, map_ofNat]
    ring
  have hd : (numeratorPolynomial - denominatorPolynomial).degree = 1 := by
    rw [hp, degree_sub_eq_left_of_degree_lt (by simp)]
    simp
  rw [he, map_div₀, polynomial_function_value, hd, denominator_value]
  norm_num [degreeValue, ← WithZero.exp_sub]

#print axioms rational_function_nonconstant
#print axioms rational_point_divisor_zero
#print axioms rational_function_monic
end
end FunctionDivisorFinitenessAudit
