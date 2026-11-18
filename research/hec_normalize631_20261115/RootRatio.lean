import Mathlib
namespace Normalization
/-- Compatible square/cube values determine the root without extraction. -/
theorem root_ratio {K : Type*} [Field K] (a2 a3 : K) (ha2 : a2 ≠ 0)
    (h : a3 ^ 2 = a2 ^ 3) :
    (a3 / a2) ^ 2 = a2 ∧ (a3 / a2) ^ 3 = a3 := by
  constructor
  · rw [div_pow, h]
    field_simp <;> ring
  · have hh : a3 ^ 3 = a3 * a2 ^ 3 := by
      calc
        a3 ^ 3 = a3 * a3 ^ 2 := by ring
        _ = a3 * a2 ^ 3 := by rw [h]
    rw [div_pow, hh]
    field_simp
end Normalization
#print axioms Normalization.root_ratio
