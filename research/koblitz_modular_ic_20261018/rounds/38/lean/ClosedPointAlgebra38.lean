import Mathlib.Algebra.CharP.Lemmas
import Mathlib.Tactic.NormNum

/-! Conditional coefficient extraction at precision615. Native arrays,
normalization and complete RR spaces are not imported as formal evidence. -/
namespace ClosedPointAlgebra38
variable {R : Type*} [CommRing R] [CharP R 2]

theorem coefficient_lift_precision615 (c z : R) (hc : c ^ 32 = c)
    (hz : z ^ 615 = 0) : (c + z) ^ 1024 = c := by
  have hz1024 : z ^ 1024 = 0 := by
    have h : z ^ (615 + 409) = z ^ 615 * z ^ 409 := pow_add z 615 409
    simpa only [hz, zero_mul] using h
  have hc1024 : c ^ 1024 = c := by
    calc
      c ^ 1024 = (c ^ 32) ^ 32 := by rw [← pow_mul]
      _ = c := by rw [hc, hc]
  have h := add_pow_char_pow c z 2 10
  norm_num only at h
  rw [h, hz1024, hc1024, add_zero]

end ClosedPointAlgebra38
