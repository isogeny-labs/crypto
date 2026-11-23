import Mathlib.Tactic
namespace TwistNormFalse
theorem missing_norm (a b c x : ℚ) (h : a*x^2+b*x+c=0) : c*x^2+b*x+a=0 := by
  linear_combination h
end TwistNormFalse
