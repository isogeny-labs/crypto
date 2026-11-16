import Mathlib.Tactic
-- Deliberately false: proportional coefficients can be zero.
example : (0:ℚ)*0-0*1=0 → (1:ℚ)=0 := by norm_num
