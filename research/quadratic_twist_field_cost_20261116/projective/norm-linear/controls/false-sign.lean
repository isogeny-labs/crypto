import Mathlib.Tactic
namespace TwistNormFalse
theorem wrong_sign (a b c aa bb cc x : ℚ) : cc*(a*x^2+b*x+c)-a*(cc*x^2+bb*x+aa)=(cc*b+a*bb)*x+(cc*c-a*aa) := by ring
end TwistNormFalse
