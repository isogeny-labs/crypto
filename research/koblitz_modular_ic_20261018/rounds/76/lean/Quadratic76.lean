import Mathlib.Tactic.Ring

namespace Quadratic76

variable {R : Type*} [CommRing R]

/-- Polynomial norm of R=(1+z)V+z^3 in the quadratic source equation. -/
theorem norm_R (z : R) (hchar : (2 : R) = 0) :
    (1 + z)^2 * (z + z^3) + (1 + z) * z^3 * z^2 + z^6 = z := by
  calc
    _ = z + 2 * (z^2 + z^3 + z^4 + z^5 + z^6) := by ring
    _ = z := by rw [hchar]; ring

/-- Polynomial norm of R+1; its only finite base zero is z+1. -/
theorem norm_R_add_one (z : R) (hchar : (2 : R) = 0) :
    (1 + z)^2 * (z + z^3) + (1 + z) * (z^3 + 1) * z^2 + (z^3 + 1)^2 = (z + 1)^3 := by
  calc
    _ = (z + 1)^3 + 2 * (-z + 2 * z^3 + z^4 + z^5 + z^6) := by ring
    _ = (z + 1)^3 := by rw [hchar]; ring

/-- With the R72 residue degrees, the six rational branch valuations
give finite zero degree 96 for b and 80 for c. -/
theorem rational_zero_degrees :
    2 * 21 + 2 * 3 + 2 * 2 + 2 * 14 + 2 * 1 + 2 * 7 = 96 ∧
    2 * 14 + 2 * 2 + 2 * 2 + 2 * 14 + 2 * 1 + 2 * 7 = 80 := by
  constructor <;> decide

end Quadratic76
