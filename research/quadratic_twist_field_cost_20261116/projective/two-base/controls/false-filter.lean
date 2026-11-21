import Mathlib.Tactic
-- False concrete instance of replacing the required first leading coefficient
-- hypothesis by nonzero second leading coefficient: a=0,d=1,c=0,f=1,x=0.
namespace TwistTwoBaseFalse
 theorem wrong_leading_hypothesis :
 ((0:ℚ)=0 ∧ (1:ℚ)=0) ↔ ((0:ℚ)=0 ∧ (0:ℚ)=0) := by
  norm_num
end TwistTwoBaseFalse
