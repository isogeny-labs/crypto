import Mathlib.Tactic
-- Deliberately false: omit the actual cube-residue hypothesis.
example : ((2:ℚ)^7)^3 = 2 := by norm_num
