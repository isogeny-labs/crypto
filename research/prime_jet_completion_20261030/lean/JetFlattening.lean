module
public import Mathlib.Algebra.Polynomial.Derivative
public import Mathlib.Tactic.Ring
public section
open Polynomial
namespace JetFlattening
noncomputable def flatten (p : ℚ[X]) (t : ℚ) : ℚ[X] :=
  p ^ 2 * (1 - C (2 * p.derivative.eval t) * (X - C t))
theorem value (p : ℚ[X]) (t : ℚ) (h : p.eval t = 1) :
    (flatten p t).eval t = 1 := by
  simp [flatten, h]
theorem first_derivative (p : ℚ[X]) (t : ℚ) (h : p.eval t = 1) :
    (flatten p t).derivative.eval t = 0 := by
  simp [flatten, derivative_mul, derivative_pow, h]
#print axioms value
#print axioms first_derivative
end JetFlattening
