import Mathlib.Data.ZMod.Basic
import Mathlib.Tactic

namespace XedniHeightOrder

instance : Fact (Nat.Prime 43) := ⟨by norm_num⟩

/-- The first collision in the fixed height order has coefficient difference
`(-7,-4)` and determines the second reduced generator direction. -/
theorem height_collision_fixes_ratio (G R : ZMod 43)
    (h : -7 * G - 4 * R = 0) : R = 9 * G := by
  have h4 : (4 : ZMod 43) ≠ 0 := by decide
  have h36 : (4 : ZMod 43) * 9 = -7 := by decide
  have hzero : (4 : ZMod 43) * (R - 9 * G) = 0 := by
    calc
      4 * (R - 9 * G) = -(-7 * G - 4 * R) := by rw [← h36]; ring
      _ = 0 := by rw [h]; ring
  exact sub_eq_zero.mp ((mul_eq_zero.mp hzero).resolve_left h4)

#print axioms height_collision_fixes_ratio

end XedniHeightOrder
