import Mathlib.Tactic
-- Deliberately false: omega=1 does not satisfy the required root-unity relation.
example : ((0:ℚ)-(1+2))^3 = 0^3-3*1*2*0-(1^3+2^3) := by norm_num
