module
public import Mathlib.Algebra.GroupWithZero.Units.Basic

public section
namespace SectionSearchAssembly

-- Existing-law assembly only. The actual quotient-algebra unit/product
-- and geometric/counting bindings are explicitly outside this endpoint.
theorem sixth_power_rescaling_zero_iff {R : Type*} [CommMonoidWithZero R]
    (a h : R) (ha : IsUnit a) : (a * h) ^ 6 = 0 ↔ h ^ 6 = 0 := by
  rw [mul_pow]
  exact (ha.pow 6).mul_right_eq_zero

#print axioms sixth_power_rescaling_zero_iff
end SectionSearchAssembly
