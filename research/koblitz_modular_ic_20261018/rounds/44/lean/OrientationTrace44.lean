import Mathlib.Tactic.Ring

namespace OrientationTrace44
/-- Cleared orientation identity, conditional on the actual normalization
and quadratic coefficient identities; no native point data imported. -/
theorem cleared_orientation_twist {R : Type*} [CommRing R]
    (a b r c2 C : R) (hc : a^2*C=b+a^2*(r+c2))
    (hr : a^3*r=b) : a^3*C=a^3*c2+a*b+b := by
  calc
    a^3*C=a*(a^2*C) := by ring
    _=a*(b+a^2*(r+c2)) := by rw [hc]
    _=a^3*c2+a*b+a^3*r := by ring
    _=a^3*c2+a*b+b := by rw [hr]
end OrientationTrace44
