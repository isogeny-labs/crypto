import Mathlib.Tactic.FieldSimp
import Mathlib.Tactic.Ring
namespace Rational50
/-- The quadratic norm relation exactly clears the rational denominator. -/
theorem quadratic_norm_zero_iff {K : Type*} [Field K]
    (a b e : K) (hb : b ≠ 0) :
    (a / b)^2 + a / b + e = 0 ↔ a^2 + a*b + e*b^2 = 0 := by
  have hid : (a / b)^2 + a / b + e =
      (a^2 + a*b + e*b^2) / b^2 := by
    field_simp [hb] <;> ring
  rw [hid, div_eq_iff (pow_ne_zero 2 hb), zero_mul]

/-- Recover the linear second coordinate with the same rational denominator. -/
theorem linear_coordinate_numerator {K : Type*} [Field K]
    (a b c d : K) (hb : b ≠ 0) :
    c + d * (a / b) = (c*b + d*a) / b := by
  field_simp [hb] <;> ring

/-- A zero denominator breaks the norm equivalence even for a=1,e=0. -/
theorem zero_denominator_counterexample (K : Type*) [Field K] :
    ((1 / (0 : K))^2 + 1 / (0 : K) + 0 = 0) ∧
    ((1 : K)^2 + 1*0 + 0*0^2 ≠ 0) := by
  simp
end Rational50
