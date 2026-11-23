import Mathlib.Tactic.Ring

/-! Polynomial identities only. These declarations do not construct smooth
curves, Picard groups, norm maps or isogenies. Local Lean checking is pending. -/
namespace P256Genus
variable {R : Type*} [CommRing R]

theorem genus2_identity (t b : R) :
    (t ^ 2) ^ 3 - 3 * t ^ 2 + b = t ^ 6 - 3 * t ^ 2 + b := by ring

theorem genus3_cleared_identity (t b : R) :
    t * (t ^ 2 + 1) ^ 3 - 3 * t ^ 3 * (t ^ 2 + 1) + b * t ^ 4 =
      t ^ 7 + b * t ^ 4 + t := by ring

theorem genus4_identity (t b : R) :
    (t ^ 3) ^ 3 - 3 * t ^ 3 + b = t ^ 9 - 3 * t ^ 3 + b := by ring

theorem genus5_identity (t b : R) :
    (t ^ 4) ^ 3 - 3 * t ^ 4 + b = t ^ 12 - 3 * t ^ 4 + b := by ring
end P256Genus
