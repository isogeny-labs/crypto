import Mathlib.Data.ZMod.Basic
import Mathlib.Tactic

namespace XedniTwoGeneratorRelations

instance : Fact (Nat.Prime 43) := ⟨by norm_num⟩

/-- The exact first independent collision has coefficient difference
`(-2,5)`. In the 43-element subgroup it fixes the second generator's
coordinate relative to the first. -/
theorem first_collision_fixes_ratio (G R : ZMod 43)
    (h : -2 * G + 5 * R = 0) : R = 9 * G := by
  have h5 : (5 : ZMod 43) ≠ 0 := by decide
  have h45 : (5 : ZMod 43) * 9 = 2 := by decide
  have hzero : (5 : ZMod 43) * (R - 9 * G) = 0 := by
    calc
      5 * (R - 9 * G) = -2 * G + 5 * R := by rw [← h45]; ring
      _ = 0 := h
  exact sub_eq_zero.mp ((mul_eq_zero.mp hzero).resolve_left h5)

#print axioms first_collision_fixes_ratio

end XedniTwoGeneratorRelations
