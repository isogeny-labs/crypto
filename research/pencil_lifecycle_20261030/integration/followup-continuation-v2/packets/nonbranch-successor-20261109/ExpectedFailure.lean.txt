import IndependentControls
open IndependentNonbranch
example : n.eval 1 = b*(-1:ℚ) := by
 norm_num [numerator_value,b,coefficients]
